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
pub struct Error {
    /// The error message.
    pub message: &'static str,
    /// The line number on which the error occurred.
    pub line: usize,
    /// The column number on which the error occurred.
    pub column: usize,
}

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

    /// This will parse the source into Sexp
    pub fn parse_sexpr(&mut self) -> () {
        let mut peekable = self.source.chars().peekable();
        let mut statements: Vec<Sexp> = vec![];

        while let Some(token) = self.next_token(&mut peekable)? {
            statements.push(tokens)
        }
    }

    fn next_token(&self, chars: &mut Peekable<Chars<'_>>) -> Result<Option<Sexp>, String> {
        Err("a".to_owned())
    }
}
