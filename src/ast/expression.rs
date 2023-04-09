use super::literal::Literal;

pub enum Expr {
    Variable(String),
    Literal(Literal),
    Array(PhpArray),
    UnaryOp(UnaryOp, Box<Expr>),
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
    TernaryOp(Box<Expr>, Box<Expr>, Box<Expr>),
    FunctionCall(String, Vec<Expr>),
    MethodCall(Box<Expr>, String, Vec<Expr>),
    StaticMethodCall(String, String, Vec<Expr>),
    PropertyAccess(Box<Expr>, String),
    StaticPropertyAccess(String, String),
    ArrayAccess(Box<Expr>, Box<Expr>),
    NewInstance(String, Vec<Expr>),
    AnonymousFunction(AnonymousFunction),
    ArrowFunction(ArrowFunction),
    Closure(Closure),
    ClassConstant(String, String),
}
