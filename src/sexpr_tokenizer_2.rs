use core::iter::*;
use crate::sexpr_tokenizer::Token;
use crate::sexpr_tokenizer::Whitespace;

pub fn tokenize(input: String) -> Vec<Token> {
  let lines = input.split('\n').collect::<Vec<&str>>().iter().enumerate();
  return lines.flat_map(|(lineNumber, line)| {
    return line.chars().enumerate().fold(vec![], |tokens, (columnNumber, character)| {
        let token = to_token(character);
        match tokens.last() {
            Some(lastToken) => match (lastToken, token) {
                (Token::Whitespace, Token::Whitespace) =>
                    [&tokens[0..lineNumber]].concat(),
            },
            None => vec![token]
        }
    })
  });
}

fn to_token(c: char) -> Token {
    match c {
        ' ' => Token::Whitespace(Whitespace::Space),
        _ => unimplemented!()
    }
}
