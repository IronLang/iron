pub enum Op {
    Plus,       // +
    PlusEqual,  // +=
    Minus,      // -
    MinusEqual, // -=
    Times,      // *
    TimesEqual, // *=
    Div,        // /
    DivEqual,   // /=
    Mod,        // %
    ModEqual,   // %=
    Pow,        // **

    And, // &&
    Or,  // ||
    Not, // !

    BitwiseAnd, // &
    BitwiseOr,  // |
    BitwiseNot, // ~
    BitwiseXor, // ^
    ShiftLeft,  // <<
    ShiftRight, // >>

    EqualTo,    // ==
    NotEqualTo, // !=
}
