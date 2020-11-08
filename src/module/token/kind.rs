/// An Iron _value_ with semantic meaning.
#[derive(Debug, PartialEq, Eq)]
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
    /// Denotes the `-=` character sequence.
    MinusEqual,
    /// Denotes the `%=` character sequence.
    ModuloEqual,
    /// Denotes the `|=` character sequence.
    OrEqual,
    /// Denotes the `+=` character sequence.
    PlusEqual,
    /// Denotes the `*=` character sequence.
    TimesEqual,
    /// Denotes the `<=` character sequence.
    LessThanEqual,
    /// Denotes the `>=` character sequence.
    GreaterThanEqual,

    /// Value-containing tokens.

    /// Denotes a comment.
    ///
    /// A comment begins with the character sequence `//` and continues until the next line break.
    /// The characters between those two points are attached to the `Comment` token as a `String`.
    Comment(String),
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
            | TokenKind::OrEqual
            | TokenKind::PlusEqual
            | TokenKind::TimesEqual => (0, 2),
            TokenKind::Newline => (1, 0),
            _ => (0, 1),
        }
    }
}
