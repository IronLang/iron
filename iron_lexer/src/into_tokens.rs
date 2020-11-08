use super::{
    consume::consume,
    error::Error,
    token::{Coordinate, Keyword, Token, TokenKind},
};
use std::iter::Peekable;
use std::str::Chars;

/// Enables some type to be represented as a vector of `Token` values.
pub trait IntoTokens {
    fn into_tokens(&self) -> Result<Vec<Token>, Error>;
}

impl IntoTokens for String {
    /// Tokenizes the contents of a `String`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use iron_lexer::{IntoTokens, TokenKind};
    ///
    /// let result = String::from("!@#").into_tokens();
    /// let mut tokens = result.expect("String should be tokenized successfully.");
    ///
    /// assert_eq!(tokens.pop().unwrap().kind, TokenKind::Octothorpe);
    /// assert_eq!(tokens.pop().unwrap().kind, TokenKind::At);
    /// assert_eq!(tokens.pop().unwrap().kind, TokenKind::Exclamation);
    /// assert_eq!(tokens.pop(), None);
    /// ```
    ///
    fn into_tokens(&self) -> Result<Vec<Token>, Error> {
        if !self.is_ascii() {
            return Err(Error::UnsupportedCharacterEncoding);
        }

        let mut result: Vec<Token> = Vec::new();
        let mut position = Coordinate::new(0, 0);
        consume_chars(&mut self.chars().peekable(), &mut result, &mut position);

        Ok(result)
    }
}

/// Builds `Token` vector by consuming a peekable character iterator.
fn consume_chars(chars: &mut Peekable<Chars>, result: &mut Vec<Token>, position: &mut Coordinate) {
    loop {
        match chars.peek() {
            Some(_) => match chars.next() {
                Some(c) => match c {
                    '}' => consume(TokenKind::BraceClose, chars, result, position),
                    '{' => consume(TokenKind::BraceOpen, chars, result, position),
                    ']' => consume(TokenKind::BracketClose, chars, result, position),
                    '[' => consume(TokenKind::BracketOpen, chars, result, position),
                    ')' => consume(TokenKind::ParenClose, chars, result, position),
                    '(' => consume(TokenKind::ParenOpen, chars, result, position),
                    ';' => consume(TokenKind::Semicolon, chars, result, position),
                    '#' => consume(TokenKind::Octothorpe, chars, result, position),
                    '@' => consume(TokenKind::At, chars, result, position),
                    '.' => consume(TokenKind::Dot, chars, result, position),
                    '`' => consume(TokenKind::Backtick, chars, result, position),
                    '~' => consume(TokenKind::Tilde, chars, result, position),
                    '"' => consume(TokenKind::QuoteDouble, chars, result, position),
                    '\'' => consume(TokenKind::QuoteSingle, chars, result, position),
                    '\\' => consume(TokenKind::Backslash, chars, result, position),
                    ' ' => consume(TokenKind::Space, chars, result, position),
                    '_' => consume(TokenKind::Underscore, chars, result, position),
                    '\t' => consume(TokenKind::Tab, chars, result, position),
                    '\n' => consume(TokenKind::Newline, chars, result, position),
                    '>' => consume(TokenKind::AngleClose, chars, result, position),
                    '<' => consume(TokenKind::AngleOpen, chars, result, position),
                    '|' => consume(TokenKind::Bar, chars, result, position),
                    '%' => consume(TokenKind::Percent, chars, result, position),
                    ':' => consume(TokenKind::Colon, chars, result, position),
                    '+' => consume(TokenKind::Cross, chars, result, position),
                    '-' => consume(TokenKind::Dash, chars, result, position),
                    '^' => consume(TokenKind::Caret, chars, result, position),
                    '&' => consume(TokenKind::Ampersand, chars, result, position),
                    '*' => consume(TokenKind::Asterisk, chars, result, position),
                    '=' => consume(TokenKind::Equal, chars, result, position),
                    '!' => consume(TokenKind::Exclamation, chars, result, position),
                    '/' => consume(TokenKind::Slash, chars, result, position),
                    _ => {
                        // Process an identifier.
                        //
                        // Identifiers must be alphanumeric sequences of characters, with the
                        // starting character being alphabetic.
                        if c.is_ascii_alphabetic() {
                            let mut identifier = String::from(c);
                            while chars.peek() != None
                                && chars.peek().unwrap().is_ascii_alphanumeric()
                            {
                                identifier.push(chars.next().unwrap())
                            }

                            match identifier.as_str() {
                                "else" => consume(
                                    TokenKind::Keyword(Keyword::Else),
                                    chars,
                                    result,
                                    position,
                                ),
                                "for" => consume(
                                    TokenKind::Keyword(Keyword::For),
                                    chars,
                                    result,
                                    position,
                                ),
                                "function" => consume(
                                    TokenKind::Keyword(Keyword::Function),
                                    chars,
                                    result,
                                    position,
                                ),
                                "if" => consume(
                                    TokenKind::Keyword(Keyword::If),
                                    chars,
                                    result,
                                    position,
                                ),
                                "implement" => consume(
                                    TokenKind::Keyword(Keyword::Implement),
                                    chars,
                                    result,
                                    position,
                                ),
                                "import" => consume(
                                    TokenKind::Keyword(Keyword::Import),
                                    chars,
                                    result,
                                    position,
                                ),
                                "match" => consume(
                                    TokenKind::Keyword(Keyword::Match),
                                    chars,
                                    result,
                                    position,
                                ),
                                "module" => consume(
                                    TokenKind::Keyword(Keyword::Module),
                                    chars,
                                    result,
                                    position,
                                ),
                                "protocol" => consume(
                                    TokenKind::Keyword(Keyword::Protocol),
                                    chars,
                                    result,
                                    position,
                                ),
                                "public" => consume(
                                    TokenKind::Keyword(Keyword::Public),
                                    chars,
                                    result,
                                    position,
                                ),
                                _ => consume(
                                    TokenKind::Identifier(identifier),
                                    chars,
                                    result,
                                    position,
                                ),
                            }
                        }
                    }
                },
                None => panic!("expected Some(char), got None"),
            },
            None => return,
        }
    }
}
