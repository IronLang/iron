use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    /// `else`
    Else,
    /// `for`
    For,
    /// `function`
    Function,
    /// `if`
    If,
    /// `implement`
    Implement,
    /// `import`
    Import,
    /// `match`
    Match,
    /// `module`
    Module,
    /// `protocol`
    Protocol,
    /// `public`
    Public,
    /// `type`
    Type,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Keyword::Else => "else",
                Keyword::For => "for",
                Keyword::Function => "function",
                Keyword::If => "if",
                Keyword::Implement => "implement",
                Keyword::Import => "import",
                Keyword::Match => "match",
                Keyword::Module => "module",
                Keyword::Protocol => "protocol",
                Keyword::Public => "public",
                Keyword::Type => "type",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Keyword;

    #[test]
    /// Ensures that the `fmt::Display` trait is implemented correctly.
    fn display_impl() {
        assert_eq!(Keyword::Else.to_string(), "else");
        assert_eq!(Keyword::For.to_string(), "for");
        assert_eq!(Keyword::Function.to_string(), "function");
        assert_eq!(Keyword::If.to_string(), "if");
        assert_eq!(Keyword::Implement.to_string(), "implement");
        assert_eq!(Keyword::Import.to_string(), "import");
        assert_eq!(Keyword::Match.to_string(), "match");
        assert_eq!(Keyword::Module.to_string(), "module");
        assert_eq!(Keyword::Protocol.to_string(), "protocol");
        assert_eq!(Keyword::Public.to_string(), "public");
        assert_eq!(Keyword::Type.to_string(), "type");

        assert_eq!(Keyword::Else.to_string().len(), 4);
        assert_eq!(Keyword::For.to_string().len(), 3);
        assert_eq!(Keyword::Function.to_string().len(), 8);
        assert_eq!(Keyword::If.to_string().len(), 2);
        assert_eq!(Keyword::Implement.to_string().len(), 9);
        assert_eq!(Keyword::Import.to_string().len(), 6);
        assert_eq!(Keyword::Match.to_string().len(), 5);
        assert_eq!(Keyword::Module.to_string().len(), 6);
        assert_eq!(Keyword::Protocol.to_string().len(), 8);
        assert_eq!(Keyword::Public.to_string().len(), 6);
        assert_eq!(Keyword::Type.to_string().len(), 4);
    }
}
