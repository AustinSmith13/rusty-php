use super::{class::Class, expression::Expr};
use std::collections::HashMap;

pub struct PhpObject {
    pub handle: u32,
    pub class_entry: Class,
    pub properties: HashMap<String, Expr>,
}
