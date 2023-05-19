use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Class {
    pub(crate) name: String,
    pub(crate) fields: Vec<FieldDecl>,
    pub(crate) methods: Vec<MethodDecl>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldDecl {
    pub(crate) field_type: Type,
    pub(crate) name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MethodDecl {
    pub(crate) retType: Type,
    pub(crate) name: String,
    pub(crate) params: Vec<(Type, String)>,
    pub(crate) body: Stmt,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Return(Expr),
    While(Expr, Box<Stmt>),
    LocalVarDecl(Type, String),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    StmtExprStmt(StmtExpr),
    TypedStmt(Box<Stmt>, Type),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum StmtExpr {
    Assign(String, Expr),
    New(Type, Vec<Expr>),
    MethodCall(Expr, String, Vec<Expr>),
    TypedStmtExpr(Box<StmtExpr>, Type),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Expr {
    This,
    Super,
    LocalOrFieldVar(String),
    InstVar(Box<Expr>, String),
    Unary(String, Box<Expr>),
    Binary(String, Box<Expr>, Box<Expr>),
    Integer(i32),
    Bool(bool),
    Char(char),
    String(String),
    Jnull,
    StmtExprExpr(Box<StmtExpr>),
    TypedExpr(Box<Expr>, Type),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Hash, Eq)]
pub enum Type {
    Int,
    Bool,
    Char,
    String,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
            Type::Char => write!(f, "char"),
            Type::String => write!(f, "String"),
        }
    }
}

pub type Prg = Vec<Class>;
