use std::str::FromStr;
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
use regex::Regex;



#[derive(Serialize, Deserialize, Debug,PartialEq)]
enum Type {
    Boolean,
    Char,
}
#[derive(Serialize, Deserialize, Debug)]
enum Types {
    boolean,
    char,
    Block(Vec<Stmt>),
}
#[derive(Serialize, Deserialize, Debug)]
enum Expr {
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LocalOrFieldVar(String),
    Char(char),
    Bool(bool),
}
#[derive(Serialize, Deserialize, Debug)]
enum BinaryOp {
    Equal,
}
#[derive(Serialize, Deserialize, Debug)]
enum Stmt {
    If {
        cond: Expr,
        body: Box<Stmt>,
        else_stmt: Option<Box<Stmt>>,
    },
    Return(Expr),
    Block(Vec<Stmt>),
}
#[derive(Serialize, Deserialize, Debug)]
struct Method {
   r#type: Type,
    name: String,
    params: Vec<VarDecl>,
    body: Stmt,
}

#[derive(Serialize, Deserialize, Debug)]
struct VarDecl {
    r#type: Type,
    name: String,
    value: Option<Types>
}


#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Class {
    name: String,
    fields: Vec<VarDecl>,
    methods: Vec<Method>,
}

fn check_expr(expr: &Expr) -> Type {
    match expr {
        Expr::Binary { op, left, right } => {
            let left_type = check_expr(left);
            let right_type = check_expr(right);
            // Check if types of left and right expressions are the same
            assert_eq!(left_type, right_type);
            // For simplicity, we assume that the type of a binary expression is always boolean
            Type::Boolean
        },
        Expr::LocalOrFieldVar(_) => {
            // For simplicity, we assume that the type of a variable is always char
            Type::Char
        },
        Expr::Char(_) => Type::Char,
        Expr::Bool(_) => Type::Boolean,
    }
}

fn check_stmt(stmt: &Stmt) -> Type {
    match stmt {
        Stmt::If { cond, body, else_stmt } => {
            // Check if the condition is a boolean
            assert_eq!(check_expr(cond), Type::Boolean);
            // Check the body and the else statement
            check_stmt(body);
            if let Some(else_stmt) = else_stmt {
                check_stmt(else_stmt);
            }
            Type::Boolean
        },
        Stmt::Return(expr) => check_expr(expr),
        Stmt::Block(stmts) => {
            for stmt in stmts {
                check_stmt(stmt);
            }
            Type::Boolean
        },
    }
}

fn check_method(method: &Method) {
    // Check if the type of the body is the same as the return type
    assert_eq!(check_stmt(&method.body), method.r#type);
}

pub(crate) fn check_class(class: &Class) {
    for method in &class.methods {
        check_method(method);
    }
}

impl FromStr for Class {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

    }
}