use std::fmt;
/// Basically we will have two type of AST.
///
/// 1. Raw
/// 2. Canonical
///
/// Where the raw AST is directly outputted by parser and canonical is for type checking
/// So we will accepting S expression (lisp grammar) like the Pie language.
use std::iter::Peekable;
use std::str::Chars;

pub enum Token {
    Word(Word),
    Number(String),
    OpenParen,
    CloseParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Word(ref w) => write!(f, "{}", w),
            Token::Number(ref n) => f.write_str(n),
            Token::OpenParen => f.write_str("("),
            Token::CloseParen => f.write_str(")"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    // If the keyword is not empty, then it is a keyword
    pub keyword: String,
    pub value: String,
}

pub struct Tokenizer {
    // This stores the whole source unit.
    pub source: String,
    pub line: u64,
    pub col: u64,
}

impl Tokenizer {
    pub fn new(src: &str) -> Self {
        Self {
            source: src.to_string(),
            line: 1,
            col: 1,
        }
    }

    pub tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut peekable = self.source.chars().peekale();
        let mut tokens : Vec<Token> = vec![];
        while let Some(token) = self.next_token(&mut peekable)? {
            match &token {
                _ => unimplemented!(),
            }
        }
    }

    pub fn next_token(&self, chars: &mut Peekable<Chars<'_>>) -> Result<Option<Token>, String> {
        match chars.peek() {
            Some(&ch) => match ch {
                _ => unimplemented!(),
            }
        }
    }
}