#[derive(Debug, PartialEq)]
pub struct Interface {
    pub name: String,
    pub extends: Interface,
    pub members: Vec<InterfaceMember>,
}

#[derive(Debug, PartialEq)]
pub enum InterfaceMember {
    // Add different types of interface members here, like
    // Method, etc.
    MethodSignature(MethodSignature),
}

#[derive(Debug, PartialEq)]
pub struct MethodSignature {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option,
}
