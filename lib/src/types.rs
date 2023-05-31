use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Class {
    pub name: String,
    pub fields: Vec<FieldDecl>,
    pub methods: Vec<MethodDecl>,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields = String::new();
        for field in &self.fields {
            fields.push_str(&format!("{}: {}, ", field.name, field.field_type));
        }
        let mut methods = String::new();
        for method in &self.methods {
            methods.push_str(&format!("{}: {}, ", method.name, method.ret_type));
        }
        write!(
            f,
            "class {} {{\n\tfields: {}\n\tmethods: {}\n}}",
            self.name, fields, methods
        )
    }
}

impl Default for Class {
    fn default() -> Self {
        Class {
            name: String::new(),
            fields: Vec::new(),
            methods: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldDecl {
    pub field_type: Type,
    pub name: String,
    pub val: Option<String>, // @Decide: Should probably Option<Expr> instead
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MethodDecl {
    pub ret_type: Type,
    pub name: String,
    pub params: Vec<(Type, String)>,
    pub body: Stmt,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Stmt {
    // @Decide: What are the Types of While/If?
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
    Void,
    Null,
    Class(String),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "boolean"),
            Type::Char => write!(f, "char"),
            Type::String => write!(f, "String"),
            Type::Void => write!(f, "void"),
            Type::Null => write!(f, "null"),
            Type::Class(name) => write!(f, "{}", name),
        }
    }
}

pub type Prg = Vec<Class>;
