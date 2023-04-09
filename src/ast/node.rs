use super::{
    class::Method,
    operators::{BinaryOpKind, UnaryOpKind},
};
use std::collections::HashMap;
pub enum FunctionReturnType {
    Any,
    Guard(String),
}

pub struct ClassFlags {
    is_abstract: bool,
    is_final: bool,
    is_interface: bool,
    is_trait: bool,
    is_readonly: bool,
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    FunctionDecleration {
        name: String,
        params: Vec<AstNode>,
        return_type: Option<Box<AstNode>>,
        body: Box<AstNode>,
    },
    Closure,
    ClassDeclaration {
        name: String,
        flags: ClassFlags,
        extends: Option<Box<AstNode>>,
        implements: Vec<AstNode>,
        members: Vec<Method>,
    },
    ArgList(Vec<AstNode>),
    List(Vec<AstNode>),
    Array(Vec<AstNode>),
    EncapsList(Vec<AstNode>),
    ExprList(Vec<AstNode>),
    StmtList(Vec<AstNode>),
    If(Vec<AstNode>),
    SwitchList(Vec<AstNode>),
    CatchList(Vec<AstNode>),
    ParamList(Vec<AstNode>),
    ClosureUses(Vec<AstNode>),
    PropDecl(Vec<AstNode>),
    ConstDecl(Vec<AstNode>),
    ClassConstDecl(Vec<AstNode>),
    NameList(Vec<AstNode>),
    TraitAdaptations(Vec<AstNode>),
    Use(Vec<AstNode>),

    MagicConst,
    Type,

    UnaryOp {
        kind: UnaryOpKind,
        child: Box<AstNode>,
    },
    BinaryOp {
        kind: BinaryOpKind,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },

    Dim {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },

    Prop(Box<AstNode>),
    StaticProp(Box<AstNode>),
    Call(Box<AstNode>),
    ClassConst(Box<AstNode>),

    Assign {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    AssignRef {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    AssignOp {
        kind: BinaryOpKind,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Greater {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    GreaterEqual {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    And {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Or {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    ArrayElem {
        key: Box<AstNode>,
        value: Box<AstNode>,
    },
    New(Box<AstNode>),
    InstanceOf {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Yield(Box<AstNode>),

    Static(Box<AstNode>),
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    DoWhile {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    IfElem(Box<AstNode>),
    Switch {
        condition: Box<AstNode>,
        cases: Vec<AstNode>,
    },
    SwitchCase {
        condition: Option<Box<AstNode>>,
        body: Box<AstNode>,
    },
    Declare(Vec<(String, AstNode)>),
    PropElem(Box<AstNode>),
    ConstElem(Box<AstNode>),
    UseTrait(Box<AstNode>),
    TraitPrecedence(Box<AstNode>),
    MethodReference(Box<AstNode>),
    Namespace(Box<AstNode>),
    TraitAlias,

    MethodCall {
        object: Box<AstNode>,
        method: Box<AstNode>,
        arguments: Box<AstNode>,
    },
    StaticCall {
        class: Box<AstNode>,
        method: Box<AstNode>,
        arguments: Box<AstNode>,
    },
    Conditional {
        condition: Box<AstNode>,
        true_expr: Box<AstNode>,
        false_expr: Box<AstNode>,
    },

    Try {
        try_block: Box<AstNode>,
        catch_list: Box<AstNode>,
        finally_block: Option<Box<AstNode>>,
    },
    Catch {
        class: Box<AstNode>,
        variable: Box<AstNode>,
        body: Box<AstNode>,
    },
    Param {
        name: Box<AstNode>,
        type_hint: Option<Box<AstNode>>,
        default_value: Option<Box<AstNode>>,
    },
    For {
        init: Box<AstNode>,
        condition: Box<AstNode>,
        update: Box<AstNode>,
        body: Box<AstNode>,
    },
    Foreach {
        array: Box<AstNode>,
        key: Option<Box<AstNode>>,
        value: Box<AstNode>,
        body: Box<AstNode>,
    },
}
