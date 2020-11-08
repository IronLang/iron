use super::super::{Error, Token};

pub trait IntoTokens {
    fn into_tokens(&self) -> Result<Vec<Token>, Error>;
}
