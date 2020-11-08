use super::{
    super::{
        token::{Coordinate, Token, TokenKind},
        Error,
    },
    advance::advance,
    into_tokens::IntoTokens,
};

impl IntoTokens for String {
    /// Tokenizes the contents of a `String`.
    fn into_tokens(&self) -> Result<Vec<Token>, Error> {
        if !self.is_ascii() {
            return Err(Error::UnsupportedCharacterEncoding);
        }

        let mut result: Vec<Token> = Vec::new();
        let mut position = Coordinate::new(0, 0);
        let mut chars = self.chars().peekable();

        loop {
            match chars.peek() {
                Some(_) => match chars.next() {
                    Some(c) => match c {
                        '>' => match chars.peek() {
                            Some(c) => match c {
                                '=' => {
                                    chars.next(); // consume additional token
                                    advance(TokenKind::GreaterThanEqual, &mut result, &mut position)
                                }
                                _ => advance(TokenKind::AngleClose, &mut result, &mut position),
                            },
                            None => advance(TokenKind::AngleClose, &mut result, &mut position),
                        },
                        '<' => match chars.peek() {
                            Some(c) => match c {
                                '=' => {
                                    chars.next(); // consume additional token
                                    advance(TokenKind::LessThanEqual, &mut result, &mut position)
                                }
                                _ => advance(TokenKind::AngleOpen, &mut result, &mut position),
                            },
                            None => advance(TokenKind::AngleOpen, &mut result, &mut position),
                        },
                        '|' => match chars.peek() {
                            Some(c) => match c {
                                '=' => {
                                    chars.next(); // consume additional token
                                    advance(TokenKind::OrEqual, &mut result, &mut position)
                                }
                                _ => advance(TokenKind::Bar, &mut result, &mut position),
                            },
                            None => advance(TokenKind::Bar, &mut result, &mut position),
                        },
                        '%' => match chars.peek() {
                            Some(c) => match c {
                                '=' => {
                                    chars.next(); // consume additional token
                                    advance(TokenKind::ModuloEqual, &mut result, &mut position)
                                }
                                _ => advance(TokenKind::Percent, &mut result, &mut position),
                            },
                            None => advance(TokenKind::Percent, &mut result, &mut position),
                        },
                        '}' => advance(TokenKind::BraceClose, &mut result, &mut position),
                        '{' => advance(TokenKind::BraceOpen, &mut result, &mut position),
                        ']' => advance(TokenKind::BracketClose, &mut result, &mut position),
                        '[' => advance(TokenKind::BracketOpen, &mut result, &mut position),
                        ')' => advance(TokenKind::ParenClose, &mut result, &mut position),
                        '(' => advance(TokenKind::ParenOpen, &mut result, &mut position),
                        ':' => advance(TokenKind::Colon, &mut result, &mut position),
                        ';' => advance(TokenKind::Semicolon, &mut result, &mut position),
                        '+' => match chars.peek() {
                            Some(c) => match c {
                                '=' => {
                                    chars.next(); // consume additional token
                                    advance(TokenKind::PlusEqual, &mut result, &mut position)
                                }
                                _ => advance(TokenKind::Cross, &mut result, &mut position),
                            },
                            None => advance(TokenKind::Cross, &mut result, &mut position),
                        },
                        '-' => match chars.peek() {
                            Some(c) => match c {
                                '=' => {
                                    chars.next(); // consume additional token
                                    advance(TokenKind::MinusEqual, &mut result, &mut position)
                                }
                                '>' => {
                                    chars.next(); // consume additional token
                                    advance(TokenKind::ArrowThin, &mut result, &mut position)
                                }
                                _ => advance(TokenKind::Dash, &mut result, &mut position),
                            },
                            None => advance(TokenKind::Dash, &mut result, &mut position),
                        },
                        '^' => match chars.peek() {
                            Some(c) => match c {
                                '=' => {
                                    chars.next(); // consume additional token
                                    advance(TokenKind::CaretEqual, &mut result, &mut position)
                                }
                                _ => advance(TokenKind::Caret, &mut result, &mut position),
                            },
                            None => advance(TokenKind::Caret, &mut result, &mut position),
                        },
                        '&' => match chars.peek() {
                            Some(c) => match c {
                                '=' => {
                                    chars.next(); // consume additional token
                                    advance(TokenKind::AndEqual, &mut result, &mut position)
                                }
                                _ => advance(TokenKind::Ampersand, &mut result, &mut position),
                            },
                            None => advance(TokenKind::Ampersand, &mut result, &mut position),
                        },
                        '*' => match chars.peek() {
                            Some(c) => match c {
                                '=' => {
                                    chars.next(); // consume additional token
                                    advance(TokenKind::TimesEqual, &mut result, &mut position)
                                }
                                _ => advance(TokenKind::Asterisk, &mut result, &mut position),
                            },
                            None => advance(TokenKind::Asterisk, &mut result, &mut position),
                        },
                        '=' => match chars.peek() {
                            Some(c) => match c {
                                '=' => {
                                    chars.next(); // consume additional token
                                    advance(TokenKind::EqualEqual, &mut result, &mut position)
                                }
                                '>' => {
                                    chars.next(); // consume additional token
                                    advance(TokenKind::ArrowWide, &mut result, &mut position)
                                }
                                _ => advance(TokenKind::Equal, &mut result, &mut position),
                            },
                            None => advance(TokenKind::Equal, &mut result, &mut position),
                        },
                        '#' => advance(TokenKind::Octothorpe, &mut result, &mut position),
                        '@' => advance(TokenKind::At, &mut result, &mut position),
                        '!' => advance(TokenKind::Exclamation, &mut result, &mut position),
                        '`' => advance(TokenKind::Backtick, &mut result, &mut position),
                        '~' => advance(TokenKind::Tilde, &mut result, &mut position),
                        '"' => advance(TokenKind::QuoteDouble, &mut result, &mut position),
                        '\'' => advance(TokenKind::QuoteSingle, &mut result, &mut position),
                        '/' => match chars.peek() {
                            Some(c) => match c {
                                '=' => {
                                    chars.next(); // consume additional token
                                    advance(TokenKind::DivEqual, &mut result, &mut position)
                                }
                                '/' => {
                                    // begin processing comment
                                    chars.next(); // consume additional token
                                    advance(
                                        TokenKind::Comment(String::from("Hello world!")),
                                        &mut result,
                                        &mut position,
                                    )
                                }
                                _ => advance(TokenKind::Slash, &mut result, &mut position),
                            },
                            None => advance(TokenKind::Slash, &mut result, &mut position),
                        },
                        '\\' => advance(TokenKind::Backslash, &mut result, &mut position),
                        ' ' => advance(TokenKind::Space, &mut result, &mut position),
                        '_' => advance(TokenKind::Underscore, &mut result, &mut position),
                        '\t' => advance(TokenKind::Tab, &mut result, &mut position),
                        '\n' => advance(TokenKind::Newline, &mut result, &mut position),

                        _ => {
                            print!("{}", c);
                        }
                    },
                    None => panic!("expected Some(char), got None"),
                },
                None => return Ok(result),
            }
        }
    }
}
