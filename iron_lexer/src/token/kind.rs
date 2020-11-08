use super::Keyword;
use std::collections::HashMap;

/// An Iron _value_ with semantic meaning.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// Single-character tokens.
    ///
    /// These are generally used as punctuation in different aspects of the language. Because
    /// the context can vary so much, now or in the future, they are named in a non-semantic way.

    /// Denotes the `&` character.
    Ampersand,
    /// Denotes the `>` character.
    AngleClose,
    /// Denotes the `<` character.
    AngleOpen,
    /// Denotes the `*` character.
    Asterisk,
    /// Denotes the `@` character.
    At,
    /// Denotes the `\` character.
    Backslash,
    /// Denotes the ``` character.
    Backtick,
    /// Denotes the `|` character.
    Bar,
    /// Denotes the `}` character.
    BraceClose,
    /// Denotes the `{` character.
    BraceOpen,
    /// Denotes the `]` character.
    BracketClose,
    /// Denotes the `[` character.
    BracketOpen,
    /// Denotes the `^` character.
    Caret,
    /// Denotes the `:` character.
    Colon,
    /// Denotes the `+` character.
    Cross,
    /// Denotes the `-` character.
    Dash,
    /// Denotes the `.` character.
    Dot,
    /// Denotes the `=` character.
    Equal,
    /// Denotes the `!` character.
    Exclamation,
    /// Denotes the `\n` character.
    Newline,
    /// Denotes the `#` character.
    Octothorpe,
    /// Denotes the `)` character.
    ParenClose,
    /// Denotes the `(` character.
    ParenOpen,
    /// Denotes the `%` character.
    Percent,
    /// Denotes the `"` character.
    QuoteDouble,
    /// Denotes the `'` character.
    QuoteSingle,
    /// Denotes the `;` character.
    Semicolon,
    /// Denotes the `/` character.
    Slash,
    /// Denotes the ` ` character.
    Space,
    /// Denotes the `\t` character.
    Tab,
    /// Denotes the `~` character.
    Tilde,
    /// Denotes the `_` character.
    Underscore,

    /// Multi-character tokens.
    ///
    /// You will notice that these tokens have more semantically-informed names. This is because
    /// there is generally less ambiguity in the contexts that they are used.

    /// Denotes the `&=` character sequence.
    AndEqual,
    /// Denotes the `->` character sequence.
    ArrowThin,
    /// Denotes the `=>` character sequence.
    ArrowWide,
    /// Denotes the `^=` character sequence.
    CaretEqual,
    /// Denotes the `/=` character sequence.
    DivEqual,
    /// Denotes the `==` character sequence.
    EqualEqual,
    /// Denotes the `>=` character sequence.
    GreaterThanEqual,
    /// Denotes the `<=` character sequence.
    LessThanEqual,
    /// Denotes the `-=` character sequence.
    MinusEqual,
    /// Denotes the `%=` character sequence.
    ModuloEqual,
    /// Denotes the `!=` character sequence.
    NotEqual,
    /// Denotes the `|=` character sequence.
    OrEqual,
    /// Denotes the `+=` character sequence.
    PlusEqual,
    /// Denotes the `::` character sequence.
    Scope,
    /// Denotes the `*=` character sequence.
    TimesEqual,

    /// Value-containing tokens.

    /// Denotes a comment.
    ///
    /// A comment begins with the character sequence `//` and continues until the next line break.
    /// The characters between those two points are attached to the `Comment` token as a `String`.
    Comment(String),

    /// Represents an identifier.
    ///
    /// An identifier is used to uniquely reference a function, variable, protocol, or some other
    /// entity that has been defined in an Iron program.
    Identifier(String),

    /// Denotes a specific keyword.
    ///
    /// Keywords are protected values in Iron programs that cannot be used as identifiers.
    Keyword(Keyword),
}

impl TokenKind {
    /// Returns the size of the `TokenKind`.
    pub fn size(&self) -> (usize, usize) {
        match self {
            TokenKind::AndEqual
            | TokenKind::ArrowThin
            | TokenKind::ArrowWide
            | TokenKind::CaretEqual
            | TokenKind::DivEqual
            | TokenKind::EqualEqual
            | TokenKind::GreaterThanEqual
            | TokenKind::LessThanEqual
            | TokenKind::MinusEqual
            | TokenKind::ModuloEqual
            | TokenKind::NotEqual
            | TokenKind::OrEqual
            | TokenKind::PlusEqual
            | TokenKind::TimesEqual => (0, 2),
            TokenKind::Newline => (1, 0),
            TokenKind::Comment(value) => (0, value.len()),
            TokenKind::Keyword(value) => (0, value.to_string().len()),
            TokenKind::Identifier(value) => (0, value.len()),
            _ => (0, 1),
        }
    }

    /// Indicates which characters could be used to build multi-character tokens.
    pub fn potential_subsequent(&self) -> Option<HashMap<char, Self>> {
        match self {
            TokenKind::Ampersand => {
                let mut map: HashMap<char, Self> = HashMap::new();
                map.insert('=', TokenKind::AndEqual);
                Some(map)
            }
            TokenKind::AngleClose => {
                let mut map: HashMap<char, Self> = HashMap::new();
                map.insert('=', TokenKind::GreaterThanEqual);
                Some(map)
            }
            TokenKind::AngleOpen => {
                let mut map: HashMap<char, Self> = HashMap::new();
                map.insert('=', TokenKind::LessThanEqual);
                Some(map)
            }
            TokenKind::Asterisk => {
                let mut map: HashMap<char, Self> = HashMap::new();
                map.insert('=', TokenKind::TimesEqual);
                Some(map)
            }
            TokenKind::Bar => {
                let mut map: HashMap<char, Self> = HashMap::new();
                map.insert('=', TokenKind::OrEqual);
                Some(map)
            }
            TokenKind::Caret => {
                let mut map: HashMap<char, Self> = HashMap::new();
                map.insert('=', TokenKind::CaretEqual);
                Some(map)
            }
            TokenKind::Colon => {
                let mut map: HashMap<char, Self> = HashMap::new();
                map.insert(':', TokenKind::Scope);
                Some(map)
            }
            TokenKind::Cross => {
                let mut map: HashMap<char, Self> = HashMap::new();
                map.insert('=', TokenKind::PlusEqual);
                Some(map)
            }
            TokenKind::Dash => {
                let mut map: HashMap<char, Self> = HashMap::new();
                map.insert('>', TokenKind::ArrowThin);
                map.insert('=', TokenKind::MinusEqual);
                Some(map)
            }
            TokenKind::Equal => {
                let mut map: HashMap<char, Self> = HashMap::new();
                map.insert('>', TokenKind::ArrowWide);
                map.insert('=', TokenKind::EqualEqual);
                Some(map)
            }
            TokenKind::Exclamation => {
                let mut map: HashMap<char, Self> = HashMap::new();
                map.insert('=', TokenKind::NotEqual);
                Some(map)
            }
            TokenKind::Percent => {
                let mut map: HashMap<char, Self> = HashMap::new();
                map.insert('=', TokenKind::ModuloEqual);
                Some(map)
            }
            TokenKind::Slash => {
                let mut map: HashMap<char, Self> = HashMap::new();
                map.insert('=', TokenKind::DivEqual);
                Some(map)
            }
            _ => None,
        }
    }
}
