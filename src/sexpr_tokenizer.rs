use std::iter::Peekable;
use std::str::Chars;
/// A single data element in an s-expression. Floats are excluded to ensure
/// atoms may be used as keys in ordered and hashed data structures.
///
/// All strings must be valid utf-8.
#[derive(PartialEq, Clone, PartialOrd)]
pub enum Atom {
    /// N stands for node
    N(String),
    /// I stands for integer
    I(i64),
    /// F stands for floating integer (number that has a '.')
    F(f64),
}

/// An s-expression is either an atom or a list of s-expressions. This is
/// similar to the data format used by lisp.
#[derive(PartialEq, Clone, PartialOrd)]
pub enum Sexp {
    Atom(Atom),
    List(Vec<Sexp>),
}

/// The representation of an s-expression parse error.
#[derive(Debug)]
pub struct TokenizerError {
    /// The error message.
    pub message: &'static str,
    /// The line number on which the error occurred.
    pub line: usize,
    /// The column number on which the error occurred.
    pub column: usize,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Word(String),
    /// Word that starts with a `'`
    Atom(String),
    Number(String),
    OpenParen,
    CloseParen,
    Whitespace(Whitespace),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Whitespace {
    Tab,
    Newline,
    Space,
}
pub const TAB_SIZE: usize = 4;

pub struct Tokenizer {
    pub source: String,
    pub line: usize,
    pub column: usize,
}

impl Tokenizer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_owned(),
            line: 1,
            column: 1,
        }
    }

    /// This will parse the source into token streams, modifying the tokenizer info
    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut peekable = self.source.chars().peekable();
        let mut tokens: Vec<Token> = vec![];

        while let Some(token) = self.next_token(&mut peekable)? {
            match &token {
                Token::Atom(s) => self.column += s.len() + 1,
                Token::Word(s) => self.column += s.len(),
                Token::Whitespace(Whitespace::Tab) => self.column += TAB_SIZE,
                Token::Whitespace(Whitespace::Newline) => {
                    self.line += 1;
                    self.column = 1;
                }
                _ => self.column += 1,
            }
            tokens.push(token)
        }

        Ok(tokens)
    }

    /// This will consume text stream and produces tokens one by one
    fn next_token(&self, chars: &mut Peekable<Chars<'_>>) -> Result<Option<Token>, TokenizerError> {
        match chars.peek() {
            Some(&ch) => match ch {
                ' ' => self.consume_and_return(chars, Token::Whitespace(Whitespace::Space)),
                '\t' => self.consume_and_return(chars, Token::Whitespace(Whitespace::Tab)),
                '\r' => {
                    // Emit a single Whitespace::Newline token for \r and \r\n
                    chars.next();
                    if let Some('\n') = chars.peek() {
                        chars.next();
                    }
                    Ok(Some(Token::Whitespace(Whitespace::Newline)))
                }
                '(' => self.consume_and_return(chars, Token::OpenParen),
                ')' => self.consume_and_return(chars, Token::CloseParen),
                '0'..='9' => {
                    let s = peeking_take_while(chars, |ch| match ch {
                        '0'..='9' | '.' => true,
                        _ => false,
                    });
                    Ok(Some(Token::Number(s)))
                }
                '\'' => {
                    chars.next(); // consume the `'`
                    let s = peeking_take_while(chars, |ch| match ch {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '/' => true,
                        _ => false,
                    });
                    Ok(Some(Token::Atom(s)))
                }
                // TODO: tokenizer should throw error on the `"` char
                _ => {
                    let mut s = String::new();
                    s.push_str(&peeking_take_while(chars, |ch| match ch {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '/' => true,
                        _ => false,
                    }));
                    Ok(Some(Token::Word(s)))
                }
            },
            None => Ok(None),
        }
    }

    fn consume_and_return(
        &self,
        chars: &mut Peekable<Chars<'_>>,
        t: Token,
    ) -> Result<Option<Token>, TokenizerError> {
        chars.next();
        Ok(Some(t))
    }
}

/// Read from `chars` until `predicate` returns `false` or EOF is hit.
/// Return the characters read as String, and keep the first non-matching
/// char available as `chars.next()`.
fn peeking_take_while(
    chars: &mut Peekable<Chars<'_>>,
    mut predicate: impl FnMut(char) -> bool,
) -> String {
    let mut s = String::new();
    while let Some(&ch) = chars.peek() {
        if predicate(ch) {
            chars.next(); // consume
            s.push(ch);
        } else {
            break;
        }
    }
    s
}

#[test]
fn empty_blob() {
    let blob = "";
    let tokens = Tokenizer::new(blob).tokenize().unwrap();
    assert_eq!(tokens, vec![]);
}

#[test]
fn atoms() {
    use Whitespace::*;
    let blob = "(foo 'bar 123 1.23)";
    let tokens = Tokenizer::new(blob).tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::OpenParen,
            Token::Word("foo".to_owned()),
            Token::Whitespace(Space),
            Token::Atom("bar".to_owned()),
            Token::Whitespace(Space),
            Token::Number("123".to_owned()),
            Token::Whitespace(Space),
            Token::Number("1.23".to_owned()),
            Token::CloseParen,
        ]
    );
}

#[test]
fn surrogate_atom() {
    let blob = "aa'aa";
    let tokens = Tokenizer::new(blob).tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![Token::Word("aa".to_owned()), Token::Atom("aa".to_owned())]
    )
}

#[test]
fn tailing_atom() {
    let blob = "a'a'";
    let tokens = Tokenizer::new(blob).tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Word("a".to_owned()),
            Token::Atom("a".to_owned()),
            Token::Atom("".to_owned())
        ]
    )
}
