use self::interfaces::Interface;

pub mod array;
pub mod class;
pub mod expression;
pub mod interfaces;
pub mod literal;
pub mod node;
pub mod operators;
pub mod statement;

#[derive(Debug, PartialEq)]
pub enum PhpAST {
    Program(Vec<statement::TopLevelStatement>),
}

#[derive(Debug, PartialEq)]
pub struct Namespace {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Use {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Constant {
    pub name: String,
    pub value: Expr,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_hint: Option<String>,
    pub default_value: Option<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Trait {
    pub name: String,
    pub members: Vec<TraitMember>,
}

#[derive(Debug, PartialEq)]
pub enum TraitMember {
    // Add different types of trait members here, like
    // Method, Property, etc.
}

#[derive(Debug, PartialEq)]
pub struct GlobalVariable {
    pub name: String,
    pub value: Option<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    // Add different types of statements here, like
    // If, While, For, Foreach, Switch, etc.
}
