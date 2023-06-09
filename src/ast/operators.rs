#[derive(Debug, PartialEq)]
pub enum UnaryOpKind {
    Negation,
    LogicalNot,
    BitwiseNot,
    PreIncrement,
    PreDecrement,
    PostIncrement,
    PostDecrement,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOpKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Concat,
    Pow,
    Equal,
    NotEqual,
    Identical,
    NotIdentical,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Xor,
    ShiftLeft,
    ShiftRight,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
    Coalesce,
    NullCoalesceEqual,
    Spaceship,
}
