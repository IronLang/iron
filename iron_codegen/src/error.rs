use std::{error::Error, fmt};

#[derive(Debug)]
pub enum IronCodegenError {
    Unexpected,
}

impl fmt::Display for IronCodegenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IronCodegenError::Unexpected => "unexpected",
            }
        )
    }
}

impl Error for IronCodegenError {}
