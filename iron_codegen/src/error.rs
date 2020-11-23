use iron_ast::IronASTNode;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum IronCodegenError {
    Unknown,
    UnexpectedASTNode(IronASTNode),
}

impl fmt::Display for IronCodegenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IronCodegenError::UnexpectedASTNode(_) => "unexpected AST node",
                IronCodegenError::Unknown => "unknown",
            }
        )
    }
}

impl Error for IronCodegenError {}
