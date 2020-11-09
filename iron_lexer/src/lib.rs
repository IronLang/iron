//! Lexing functionality for the Iron programming language.
//!
//! Lexical analysis (or _tokenization_) is the first phase of compiling an Iron module. This phase
//! entails the initial reading of the input file(s) and identification of _tokens_, character
//! sequences that have special value in Iron programs.

mod consume;
mod error;
mod into_tokens;
mod token;

use std::fs;

pub use error::Error;
pub use into_tokens::IntoTokens;
pub use token::{Keyword, Token, TokenKind};

/// Tokenize the contents of a file located at some `path`.
pub fn tokenize_file(path: &str) -> Result<Vec<Token>, Error> {
    match fs::read_to_string(path) {
        Ok(contents) => contents.into_tokens(),
        Err(err) => Err(Error::IO(err)),
    }
}
