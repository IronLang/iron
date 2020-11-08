use super::{
    super::{token::Token, Error},
    into_tokens::IntoTokens,
};
use std::fs;

/// Tokenize the contents of a file located at some `path`.
pub fn tokenize_file(path: &str) -> Result<Vec<Token>, Error> {
    match fs::read_to_string(path) {
        Ok(contents) => contents.into_tokens(),
        Err(err) => Err(Error::IO(err)),
    }
}
