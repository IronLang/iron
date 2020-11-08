use super::super::token::{Coordinate, Token, TokenKind};

/// Adds a new token to `result`, and advances the `position` based on the size of the `TokenKind`.
pub fn advance(kind: TokenKind, result: &mut Vec<Token>, position: &mut Coordinate) {
    let (row_delta, col_delta) = kind.size();
    let (row_end, col_end) = if kind == TokenKind::Newline {
        (position.row + 1, 0)
    } else {
        (position.row + row_delta, position.col + col_delta)
    };

    let start = Coordinate::new(position.row, position.col);
    let end = Coordinate::new(row_end, col_end);

    // Update position
    position.row = row_end;
    position.col = col_end;

    // Push new token
    let token = Token::new(kind, start, end);
    result.push(token);
}
