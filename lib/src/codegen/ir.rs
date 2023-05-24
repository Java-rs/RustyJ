use crate::typechecker::*;
use crate::types::*;
/// The DIR(Duck Intermediate Representation) is our IR for generating Java Bytecode
/// from our TAST
pub struct DIR {
    pub(crate) constant_pool: Vec<Constant>,
    pub(crate) classes: Vec<IRClass>,
}
pub(crate) struct IRClass {
    pub(crate) name: String,
    pub(crate) super_name: String,
    pub(crate) fields: Vec<FieldDecl>,
    pub(crate) methods: Vec<CompiledMethod>,
}

impl IRClass {
    pub(crate) fn new(
        name: String,
        fields: Vec<FieldDecl>,
        methods: Vec<CompiledMethod>,
    ) -> IRClass {
        IRClass {
            name,
            super_name: String::from("java/lang/Object"),
            fields,
            methods,
        }
    }
}

pub(crate) struct CompiledMethod {
    pub(crate) name: String,
    pub(crate) max_stack: u16,
    pub(crate) code: Vec<Instruction>,
}
pub(crate) enum Constant {
    Class(String),
    FieldRef(String),
    MethodRef(String),
    NameAndType(String),
    Utf8(String),
}
pub(crate) enum Instruction {
    aload(u8),
    iload(u8),
    ifeq(u16),
    Return,
}

pub fn generate_dir(ast: &Prg) -> DIR {
    let mut dir = DIR {
        constant_pool: vec![],
        classes: vec![],
    };
    for class in ast {
        dir.classes.push(generate_class(class, &dir));
    }
    dir
}

fn generate_class(class: &Class, dir: &DIR) -> IRClass {
    let mut ir_class = IRClass::new(class.name.clone(), vec![], vec![]);
    for field in &class.fields {
        ir_class.fields.push(field.clone());
    }
    for method in &class.methods {
        ir_class.methods.push(generate_method(method, dir));
    }
    ir_class
}

fn generate_field(field: &FieldDecl, constant_pool: &mut Vec<Constant>) -> FieldDecl {
    constant_pool.push(Constant::Utf8(field.name.clone()));
    let name_index = constant_pool.len();
    constant_pool.push(Constant::FieldRef(field.types.clone()));
    let type_index = constant_pool.len();
    let mut compiled_field = FieldDecl::new(field.types.clone(), field.name.clone());
    compiled_field
}

// TODO: Parallelize this, since methods are not dependent on each other(hopefully)
fn generate_method(method: &MethodDecl, dir: &DIR) -> CompiledMethod {
    let mut compiled_method = CompiledMethod {
        name: method.name.clone(),
        max_stack: 0,
        code: vec![],
    };

    compiled_method
        .code
        .append(&mut generate_code_stmt(method.body.clone(), dir));

    compiled_method
}

fn generate_code_stmt(stmt: Stmt, dir: &DIR) -> Vec<Instruction> {
    match stmt {
        Stmt::Block(stmts) => {}
        Stmt::Return(expr) => {
            // Generate bytecode for return
        }
        Stmt::While(expr, stmt) => {
            // Generate bytecode for while
        }
        Stmt::LocalVarDecl(types, name) => {
            // Generate bytecode for local var decl
        }
        Stmt::If(expr, stmt1, stmt2) => {
            // Generate bytecode for if
        }
        Stmt::StmtExprStmt(stmt_expr) => {
            // Generate bytecode for stmt expr
        }
        Stmt::TypedStmt(stmt, types) => {
            // Generate bytecode for typed stmt
        }
    }
    vec![]
}

fn generate_code_stmt_expr(stmt_expr: &StmtExpr, code: &mut Vec<u8>) {
    match stmt_expr {
        StmtExpr::Assign(name, expr) => {
            // Generate bytecode for assignment
        }
        StmtExpr::New(types, exprs) => {
            // Generate bytecode for new
        }
        StmtExpr::MethodCall(expr, name, exprs) => {
            // Generate bytecode for method call
        }
        StmtExpr::TypedStmtExpr(stmt_expr, types) => {
            // Generate bytecode for typed stmt expr
        }
    }
}

fn generate_code_expr(expr: &Expr, code: &mut Vec<u8>) {
    match expr {
        Expr::This => {
            // Generate bytecode for this
        }
        Expr::Super => {
            // Generate bytecode for super
        }
        Expr::LocalOrFieldVar(name) => {
            // Generate bytecode for local or field var
        }
        Expr::InstVar(expr, name) => {
            // Generate bytecode for inst var
        }
        Expr::Unary(op, expr) => {
            // Generate bytecode for unary
        }
        Expr::Binary(op, expr1, expr2) => {
            // Generate bytecode for binary
        }

        Expr::Integer(value) => {
            // Generate bytecode for int literal
        }
        Expr::Bool(value) => {
            // Generate bytecode for bool literal
        }
        Expr::Char(value) => {
            // Generate bytecode for char literal
        }
        Expr::String(value) => {
            // Generate bytecode for string literal
        }
        Expr::Jnull => {
            // Generate bytecode for null literal
        }
        Expr::StmtExprExpr(stmt_expr) => {
            // Generate bytecode for stmt expr
        }
        Expr::TypedExpr(expr, types) => {
            // Generate bytecode for typed expr
        }
    }
}
fn generate_code_type(types: &Type, code: &mut Vec<u8>) {
    match types {
        Type::Int => {
            // Generate bytecode for int
        }
        Type::Bool => {
            // Generate bytecode for bool
        }
        Type::Char => {
            // Generate bytecode for char
        }
        Type::String => {
            // Generate bytecode for string
        }
    }
}
