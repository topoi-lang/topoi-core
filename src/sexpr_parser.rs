use crate::sexpr_tokenizer::*;

/// A single data element in an s-expression. Floats are excluded to ensure
/// atoms may be used as keys in ordered and hashed data structures.
///
/// All strings must be valid utf-8.
#[derive(PartialEq, Clone, PartialOrd, Debug)]
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
#[derive(PartialEq, Clone, PartialOrd, Debug)]
pub enum Sexp {
    /// Bottom type, ()
    Unit,
    Atom(Atom),
    Pair(Box<Sexp>, Box<Sexp>),
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
            if parser.peek_token().is_none() {
                break;
            }
            let expression = parser.parse_expression()?;
            expressions.push(expression);
        }

        Ok(expressions)
    }

    pub fn parse_expression(&mut self) -> Result<Sexp, ParserError> {
        println!("next token: {:?}", self.peek_token());
        match self.next_token() {
            Some(t) => match t {
                Token::OpenParen => Ok(self.build_sexp()?),
                Token::CloseParen => Err(ParserError::ParserError(
                    "Unmatched right parenthesis".to_string(),
                )),
                _ => Err(ParserError::ParserError(
                    "No root value outside of parenthesis".to_string(),
                )),
            },

            None => Err(ParserError::ParserError(
                "No expression is given. Quiting...".to_string(),
            )),
        }
    }

    pub fn build_sexp(&mut self) -> Result<Sexp, ParserError> {
        let mut temp_sexp = vec![];
        loop {
            match self.next_token() { // It consumes the open parenthesis
                Some(t) => match t {
                    Token::Atom(s) => temp_sexp.push(Sexp::Atom(Atom::N(s))),
                    Token::Word(s) => temp_sexp.push(Sexp::Atom(Atom::N(s))),
                    Token::Number(number_literal) => {
                        match number_literal.parse::<i64>() {
                            Ok(n) => temp_sexp.push(Sexp::Atom(Atom::I(n))),
                            Err(_) => match number_literal.parse::<f64>() {
                                Ok(n) => temp_sexp.push(Sexp::Atom(Atom::F(n))),
                                Err(_) => return Err(ParserError::ParserError("Error parsing number literal".to_string()))
                            }
                        }
                    }
                    Token::OpenParen => temp_sexp.push(self.build_sexp()?),
                    Token::CloseParen => break,
                    Token::Whitespace(_) => unreachable!(),
                }
                None => return Err(ParserError::ParserError("Unclosed parenthesis".to_string())),
            }
        }

        match temp_sexp.len() {
            0 => Ok(Sexp::Unit),
            1 => Ok(temp_sexp[0].clone()),
            2 => Ok(Sexp::Pair(Box::new(temp_sexp[0].clone()), Box::new(temp_sexp[1].clone()))),
            _ => Ok(build_pair(temp_sexp)),
        }
    }

    /// Return the first non-whitespace token that has not yet been processed
    /// (or None if reached end-of-file)
    pub fn peek_token(&self) -> Option<Token> {
        self.peek_nth_token(0)
    }

    /// Return nth non-whitespace token that has not yet been processed
    pub fn peek_nth_token(&self, mut n: usize) -> Option<Token> {
        let mut index = self.index;
        loop {
            index += 1;
            match self.tokens.get(index - 1) {
                Some(Token::Whitespace(_)) => continue,
                non_whitespace => {
                    if n == 0 {
                        return non_whitespace.cloned();
                    }
                    n -= 1;
                }
            }
        }
    }

    /// Return the first non-whitespace token that has not yet been processed
    /// (or None if reached end-of-file) and mark it as processed. OK to call
    /// repeatedly after reaching EOF.
    pub fn next_token(&mut self) -> Option<Token> {
        loop {
            self.index += 1;
            match self.tokens.get(self.index - 1) {
                Some(Token::Whitespace(_)) => continue,
                token => return token.cloned(),
            }
        }
    }
}

pub fn build_pair(mut expressions: Vec<Sexp>) -> Sexp {
    expressions.reverse();
    expressions
        .iter()
        .fold(Sexp::Unit, |pair_tree, el| Sexp::Pair(Box::new(el.clone()), Box::new(pair_tree)))
}

#[cfg(test)]
mod test {
    use crate::sexpr_parser::*;
    use crate::sexpr_parser::Sexp::*;
    use crate::sexpr_parser::Atom::*;

    #[test]
    fn parse_unit_expression() {
        let blob = "()";
        let sexp = Parser::parse(blob.to_owned()).unwrap();
        assert_eq!(sexp, vec![Unit])
    }

    #[test]
    fn parse_atom_expression() {
        let blob = "(atom)";
        let sexp = Parser::parse(blob.to_owned()).unwrap();
        assert_eq!(sexp, vec![Atom(N("atom".to_string()))])
    }

    #[test]
    fn parse_pair_atom_expression() {
        let blob = "(Giuseppe Verdi)";
        let sexp = Parser::parse(blob.to_owned()).unwrap();
        assert_eq!(sexp, vec![
            Pair(Box::new(Atom(N("Giuseppe".to_string()))), Box::new(Atom(N("Verdi".to_string()))))
        ])
    }

    #[test]
    fn parse_multiple_atom_expression() {
        let blob = "(Giuseppe Verdi Louis)";
        let sexp = Parser::parse(blob.to_owned()).unwrap();
        assert_eq!(sexp, vec![
            Pair(Box::new(Atom(N("Giuseppe".to_string()))),
                Box::new(Pair(Box::new(Atom(N("Verdi".to_string()))),
                    Box::new(Pair(Box::new(Atom(N("Louis".to_string()))), Box::new(Unit))))))
        ]);
    }
}
