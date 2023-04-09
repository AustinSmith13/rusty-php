use crate::ast::expression::Expr;
use std::collections::HashMap;

pub struct PhpArray {
    pub indexed: Vec<Expr>,
    pub associative: HashMap<Expr, Expr>,
}

pub enum ArrayElement {
    Indexed(Expr),
    AssociativeString(String, Expr),
    AssociativeExpr(Expr, Expr),
}
