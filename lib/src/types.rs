#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(non_snake_case)]

use crate::codegen::ConstantPool;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FieldDecl {
    pub field_type: Type,
    pub name: String,
    pub val: Option<Expr>,
}

impl FieldDecl {
    /// See https://docs.oracle.com/javase/specs/jvms/se15/html/jvms-4.html#jvms-4.5
    pub fn as_bytes(&self, class_name: &str, constant_pool: &mut ConstantPool) -> Vec<u8> {
        use crate::codegen::Constant;
        use crate::codegen::FieldRef;
        use crate::codegen::NameAndType;

        let mut bytes = Vec::new();
        // No access modifier
        bytes.extend_from_slice(&[0x0, 0x0]);
        // Name index
        bytes.extend_from_slice(
            &constant_pool
                .add(Constant::Utf8(self.name.clone()))
                .to_be_bytes(),
        );
        // Descripter index
        bytes.extend_from_slice(
            &constant_pool
                .add(Constant::NameAndType(NameAndType {
                    name: self.name.clone(),
                    r#type: self.field_type.to_ir_string(),
                }))
                .to_be_bytes(),
        );
        // Attributes count
        bytes.extend_from_slice(&[0x0, 0x0]);
        if let Some(val) = &self.val {}
        bytes
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct MethodDecl {
    pub ret_type: Type,
    pub name: String,
    pub params: Vec<(Type, String)>,
    pub body: Stmt,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Return(Expr),
    While(Expr, Box<Stmt>), // first condition, then body of the while-statement
    LocalVarDecl(Type, String), // first type of the local variable, then it's name
    If(Expr, Box<Stmt>, Option<Box<Stmt>>), // first condition, then body ofthe if-statement and lastly the optional body of the else-statement
    StmtExprStmt(StmtExpr),
    TypedStmt(Box<Stmt>, Type),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum StmtExpr {
    Assign(Expr, Expr), // first the name of the variable, then the value it is being assigned to
    New(Type, Vec<Expr>), // first the class type, that should be instantiated, then the list of arguments for the constructor
    // FIXME: This needs to be changed to represent more how the JVM handles method calls. We need a class(at least name) and a method name with the typed arguments inside it, also the return type
    //    #2 = Methodref          #3.#17         // MethodTest.y:(I)I
    //    #3 = Class              #18            // MethodTest
    //    #17 = NameAndType        #19:#20        // y:(I)I
    //    #18 = Utf8               MethodTest
    //    #19 = Utf8               y
    //    #20 = Utf8               (I)I
    MethodCall(Expr, String, Vec<Expr>), // first the object to which the method belongs (e.g. Expr::This), then the name of the method and lastly the list of arguments for the method call
    TypedStmtExpr(Box<StmtExpr>, Type),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Expr {
    This,
    LocalOrFieldVar(String), // name of the variable
    InstVar(Box<Expr>, String),
    LocalVar(String),                     // name of the variable
    FieldVar(String),                     // name of the variable
    Unary(String, Box<Expr>),             // operation first, then operand
    Binary(String, Box<Expr>, Box<Expr>), // operation first, then left and right operands
    Integer(i32),
    Bool(bool),
    Char(char),
    String(String),
    Jnull,
    StmtExprExpr(Box<StmtExpr>),
    TypedExpr(Box<Expr>, Type),
}

impl Expr {
    /// Gets the type if one is present
    pub(crate) fn get_type(&self) -> Option<Type> {
        match self {
            Expr::TypedExpr(_, t) => Some(t.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UnaryOp {
    Pos,
    Neg,
    Not,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Pos => write!(f, "+"),
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}

impl From<&str> for UnaryOp {
    fn from(s: &str) -> Self {
        match s {
            "+" => UnaryOp::Pos,
            "-" => UnaryOp::Neg,
            "!" => UnaryOp::Not,
            _ => panic!("Invalid unary operator: {}", s),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Le,
    Ge,
    Lt,
    Gt,
    Eq,
    Ne,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Mod => write!(f, "%"),
            BinaryOp::And => write!(f, "&&"),
            BinaryOp::Or => write!(f, "||"),
            BinaryOp::Le => write!(f, "<="),
            BinaryOp::Ge => write!(f, ">="),
            BinaryOp::Lt => write!(f, "<"),
            BinaryOp::Gt => write!(f, ">"),
            BinaryOp::Eq => write!(f, "=="),
            BinaryOp::Ne => write!(f, "!="),
        }
    }
}

impl From<&str> for BinaryOp {
    fn from(s: &str) -> Self {
        match s {
            "+" => BinaryOp::Add,
            "-" => BinaryOp::Sub,
            "*" => BinaryOp::Mul,
            "/" => BinaryOp::Div,
            "%" => BinaryOp::Mod,
            "&&" => BinaryOp::And,
            "||" => BinaryOp::Or,
            "<=" => BinaryOp::Le,
            ">=" => BinaryOp::Ge,
            "<" => BinaryOp::Lt,
            ">" => BinaryOp::Gt,
            "==" => BinaryOp::Eq,
            "!=" => BinaryOp::Ne,
            _ => panic!("Invalid binary operator: {}", s),
        }
    }
}

impl BinaryOp {
    pub fn prec(op: &str) -> u8 {
        match op {
            "*" => 0,
            "/" => 0,
            "%" => 0,
            "+" => 1,
            "-" => 1,
            "<=" => 2,
            ">=" => 2,
            "<" => 2,
            ">" => 2,
            "==" => 3,
            "!=" => 3,
            "&&" => 4,
            "||" => 4,
            _ => panic!("Invalid binary operator: {}", op),
        }
    }
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

impl Type {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_ir_string().as_bytes().to_vec()
    }
    pub fn to_ir_string(&self) -> String {
        match self {
            Type::Int => "I",
            Type::Char => "C",
            Type::Bool => "Z",
            Type::String => "Ljava/lang/String;",
            Type::Void => "V",
            // FIXME: Either the class has the formatting `L<class>;' or we have to add it here.
            Type::Class(name) => name,
            _ => panic!("Invalid type: {}", self),
        }
        .to_string()
    }
}

pub type Prg = Vec<Class>;
