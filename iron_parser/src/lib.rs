mod error;
mod expression;
mod module;
mod op;

pub use module::Module;

use error::Error;
use iron_ast;
use iron_lexer::{Keyword, Token, TokenKind};
use std::iter::Peekable;
use std::vec::IntoIter;

pub fn parse_tokens(tokens: Vec<Token>) -> Result<(), Error> {
    parse(&mut tokens.into_iter().peekable());
    return Err(Error::Unimplemented);
}

/// Parses top-level definitions.
///
/// Iron allows five different types of values to be defined at the top level:
///
/// 1. Imports
/// 2. Types
/// 3. Functions
/// 4. Protocols
/// 5. Implementations
///
fn parse(tokens: &mut Peekable<IntoIter<Token>>) {
    match tokens.peek() {
        Some(token) => match token.kind {
            TokenKind::Comment(_) => (), // ignore for now
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Public => parse_public_definition(tokens),
                Keyword::Function => parse_function(tokens),
                Keyword::Import => parse_import(tokens),
                Keyword::Protocol => parse_function(tokens),
                Keyword::Type => parse_type(tokens),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        },
        None => unimplemented!(),
    }
}

fn parse_public_definition(tokens: &mut Peekable<IntoIter<Token>>) {
    unimplemented!()
}

fn parse_function(tokens: &mut Peekable<IntoIter<Token>>) {
    unimplemented!()
}

fn parse_import(tokens: &mut Peekable<IntoIter<Token>>) {
    unimplemented!()
}

fn parse_type(tokens: &mut Peekable<IntoIter<Token>>) {
    unimplemented!()
}
