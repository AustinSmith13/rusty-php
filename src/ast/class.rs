use super::{interfaces::Interface, node::AstNode};

#[derive(Debug, PartialEq)]
pub enum ClassMember {
    ConstantDeclaration(Constant),
    PropertyDeclaration(Property),
    MethodDeclaration(Method),
}

#[derive(Debug, PartialEq)]
pub struct Property {
    pub name: String,
    pub default_value: Option<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Method {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
    pub body: Vec<AstNode>,
}
