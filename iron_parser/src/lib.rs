extern crate iron_lexer;

use iron_lexer::{Error, Token};
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
    tokens: Vec<Token>,
}

pub trait ModuleDefinition {
    fn is_public(&self) -> bool;
}

impl FromStr for Module {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = String::from(s);

        match iron_lexer::tokenize_file(s) {
            Ok(tokens) => Ok(Module { path, tokens }),
            Err(err) => Err(err),
        }
    }
}
