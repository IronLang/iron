extern crate iron_lexer;

use crate::parse_tokens;
use iron_lexer::Error as LexerError;
use std::str::FromStr;

/// An Iron module.
///
/// Modules are the building blocks of Iron systems. They are defined as files containing Iron
/// source code, and should be denoted with the `.fe` file extension.
///
/// Modules can define and expose different types of functionality for consumption by other,
/// external modules.
///
/// - Imports:
/// - Types:
/// - Functions:
/// - Protocols:
#[derive(Debug)]
pub struct Module {
    path: String,
}

impl FromStr for Module {
    type Err = LexerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = String::from(s);

        match iron_lexer::tokenize_file(s) {
            Ok(tokens) => {
                match parse_tokens(tokens) {
                    Ok(_) => println!("parsed okay!"),
                    Err(_) => return Err(LexerError::UnsupportedCharacterEncoding),
                }

                Ok(Module { path })
            }
            Err(err) => Err(err),
        }
    }
}
