mod coordinate;
mod kind;

pub use coordinate::Coordinate;
pub use kind::TokenKind;

/// Represents a semantic value in a file containing Iron source code.
///
/// Tokens store _starting_ and _ending_ coordinates to help the compiler produce meaningful
/// warning and error messages. Additional information required to interpret the token is stored
/// in its `kind` attribute.
#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    start: Coordinate,
    end: Coordinate,
}

impl Token {
    /// Creates a new `Token`.
    pub fn new(kind: TokenKind, start: Coordinate, end: Coordinate) -> Token {
        return Token { kind, start, end };
    }
}
