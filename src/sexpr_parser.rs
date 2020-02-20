use crate::sexpr_tokenizer::*;

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
///
/// TODO: I don't know whether I need to add those seven Lisp primitives to
/// this enum or not.
///
/// [Seven lisp primitives, or ten?](https://stackoverflow.com/questions/3482389/how-many-primitives-does-it-take-to-build-a-lisp-machine-ten-seven-or-five)
#[derive(PartialEq, Clone, PartialOrd)]
pub enum Sexp {
    Atom(Atom),
    List(Vec<Sexp>),
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

#[derive(Debug)]
pub enum ParserError {
    TokenizerError(String),
    ParserError(String),
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, index: 0 }
    }

    /// This function will produce canonical AST
    pub fn parse(sexpr: String) -> Result<Vec<Sexp>, ParserError> {
        let mut tokenizer = Tokenizer::new(&sexpr);

        // TODO: FromError to turn this error to ParserError::TokenizerError
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);

        let mut expressions = vec![];

        loop {
            let expression = parser.parse_expression()?;
            expressions.push(expression);
        }

        OK(expressions)
    }

    pub fn parse_expression(tokens: Vec<Token>) -> Result<Option<Sexp>, ParserError> {}
}
