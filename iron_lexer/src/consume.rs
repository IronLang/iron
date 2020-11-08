use super::token::{Coordinate, Token, TokenKind};
use std::iter::Peekable;
use std::str::Chars;

/// Advances the state of the lexer by consuming a token.
pub fn consume(
    kind: TokenKind,
    chars: &mut Peekable<Chars>,
    result: &mut Vec<Token>,
    position: &mut Coordinate,
) {
    // Handle comments.
    //
    // Comments are a special case where, if we have a line comment starting sequence (`//`),
    // we need to consume everything until the next `TokenKind::Newline`. That entire value should
    // be processed as a single `TokenKind::Comment` with everything up to the newline being the
    // associated string value.
    if kind == TokenKind::Slash && chars.peek() == Some(&'/') {
        chars.next(); // consume the second `/`

        let mut line = String::from("//");
        while chars.peek() != Some(&'\n') && chars.peek() != None {
            line.push(chars.next().expect("failed to catch `chars.next()` value"));
        }
        return consume_inner(TokenKind::Comment(line), result, position);
    }

    // Handle remaining cases.
    match kind.potential_subsequent() {
        Some(options) => match chars.peek() {
            Some(c) => match options.get(c) {
                Some(multichar) => {
                    chars.next(); // Consume character value
                    consume_inner(multichar.clone(), result, position)
                }
                None => consume_inner(kind, result, position),
            },
            None => consume_inner(kind, result, position),
        },
        None => consume_inner(kind, result, position),
    }
}

/// Advances the state of the lexer.
///
/// 1. Adds a new `Token` to `result`
/// 2. Advances the `position` based on the size of the `TokenKind`
fn consume_inner(kind: TokenKind, result: &mut Vec<Token>, position: &mut Coordinate) {
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
