// use smallvec::SmallVec;
use std::fmt;

// pub enum Atom {
//     Str(String),
//     Int(i64),
//     Float(f64),
// }

// pub enum Sexp {
//     Atom(Atom),
//     List(SmallVec<[Sexp; 32]>),
// }

pub enum Token {
    Word(String),
    QuotedWord(String),
    Number(String),
    OpenParen,
    CloseParen,
    Whitespace(Whitespace),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Word(ref w) => write!(f, "{}", w),
            Token::QuotedWord(ref w) => write!(f, "{}", w),
            Token::Number(ref n) => f.write_str(n),
            Token::OpenParen => f.write_str("("),
            Token::CloseParen => f.write_str(")"),
            Token::Whitespace(ref w) => write!(f, "{}", w),
        }
    }
}

pub enum Whitespace {
    Space,
    Newline,
    Tab,
}

impl fmt::Display for Whitespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Whitespace::Space => f.write_str(" "),
            Whitespace::Newline => f.write_str("\n"),
            Whitespace::Tab => f.write_str("\t"),
        }
    }
}

pub struct TokenError {
    pub message: &'static str,
    pub line: usize,
    pub column: usize,
    pub line_offset: usize,
}

pub struct Tokenizer {
    // This stores the whole source unit.
    pub source: String,
    pub line: usize,
    pub column: usize,
}

impl Tokenizer {
    pub fn new(src: &str) -> Self {
        Self {
            source: src.to_string(),
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenError> {
        let mut peekable = self.source.chars().peekable();
        let mut tokens: Vec<Token> = vec![];

        while let Some(token) = self.next_token(&mut peekable)? {
            match &token {
                Token::Whitespace(Whitespace::Newline) => {
                    self.line += 1;
                    self.col = 1;
                }
                Token::Whitespace(Whitespace::Tab) => self.column += 4,
                Token::Word(w) => self.column += w.len(),
                Token::Number(s) => self.column += s.len(),
                _ => self.column += 1,
            }
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn consume_and_return(
        &self,
        chars: &mut Peekable<Chars<'_>>,
        t: Token,
    ) -> Result<Option<Token>, TokenError> {
        chars.next();
        Ok(Some(t))
    }

    /// Get the next token or return None
    fn next_token(&self, chars: &mut Peekable<Chars<'_>>) -> Result<Option<Token>, TokenError> {
        //println!("next_token: {:?}", chars.peek());
        match chars.peek() {
            Some(&ch) => match ch {
                '(' => self.comsume_and_return(chars, Token::OpenParen),
                ')' => self.consume_and_return(chars, Token::CloseParen),
                ' ' => self.consume_and_return(chars, Token::Whitespace(Whitespace::Space)),
                '\t' => self.consume_and_return(chars, Token::Whitespace(Whitespace::Tab)),
                '\n' => self.consume_and_return(chars, Token::Whitespace(Whitespace::Newline)),
                '\r' => {
                    // Emit a single Whitespace::Newline token for \r and \r\n
                    chars.next();
                    if let Some('\n') = chars.peek() {
                        chars.next();
                    }
                    Ok(Some(Token::Whitespace(Whitespace::Newline)))
                }
                '0'..='9' => {
                    let s = peeking_take_while(chars, |ch| match ch {
                        '0'..='9' | '.' => true,
                        _ => false,
                    });
                    Ok(Some(Token::Number(s)))
                }
                _ => Token::Word("aaa".to_owned()),
            },
            None => Ok(None),
        }
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

#[cfg(test)]
mod sexpr_lexer {

    #[test]
    fn lexer_test() {
        let blob = "(1 23 )";
        let tokens = Tokenizer::new(blob).tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::OpenParen,
                Token::Number("1"),
                Token::Number("2"),
                Token::CloseParen
            ]
        )
    }
}
