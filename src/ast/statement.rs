use super::{class::Class, interfaces::Interface};

#[derive(Debug, PartialEq)]
pub enum TopLevelStatement {
    NamespaceDeclaration(Namespace),
    UseDeclaration(Use),
    ClassDeclaration(Class),
    TraitDeclaration(Trait),
    InterfaceDeclaration(Interface),
    FunctionDeclaration(Function),
    ConstantDeclaration(Constant),
    GlobalVariableDeclaration(GlobalVariable),
    GlobalStatement(Statement),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expr),
    Block(Vec<Statement>),
    If(Expr, Box<Statement>, Option<Box<Statement>>),
    ElseIf(Expr, Box<Statement>),
    Else(Box<Statement>),
    While(Expr, Box<Statement>),
    DoWhile(Expr, Box<Statement>),
    For(Option<Expr>, Option<Expr>, Option<Expr>, Box<Statement>),
    Foreach(Expr, Expr, Box<Statement>),
    Switch(Expr, Vec<SwitchCase>),
    Break,
    Continue,
    Return(Option<Expr>),
    Global(Vec<String>),
    Static(Vec<(String, Option<Expr>)>),
    Echo(Vec<Expr>),
    Unset(Vec<Expr>),
    Declare(Vec<(String, Expr)>),
    Namespace(String),
    Use(Vec<UseStatement>),
    Function(FunctionDeclaration),
    Class(ClassDeclaration),
    Interface(InterfaceDeclaration),
    Trait(TraitDeclaration),
    Try(TryCatchBlock),
}
