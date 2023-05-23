use crate::typechecker::*;
use crate::types::*;
/// The DIR(Duck Intermediate Representation) is our IR for generating Java Bytecode
/// from our TAST
pub(crate) struct DIR {
    pub(crate) constant_pool: Vec<Constant>,
    pub(crate) classes: Vec<IRClass>,
}
pub(crate) struct IRClass {
    pub(crate) name: String,
    pub(crate) super_name: String,
    pub(crate) fields: Vec<FieldDecl>,
    pub(crate) methods: Vec<CompiledMethod>,
}

pub(crate) struct CompiledMethod {
    pub(crate) name: String,
    pub(crate) max_stack: u16,
    pub(crate) code: Vec<Instruction>,
}
pub(crate) struct Constant {
    pub(crate) tag: u8,
    pub(crate) data: Vec<u8>,
}
pub(crate) enum Instruction {
    aload(u8),
    iload(u8),
    Return,
}
pub fn generate_dir(ast: &Prg) -> DIR {
    let mut dir = DIR {
        constant_pool: vec![],
        classes: vec![],
    };
    for class in ast {
        dir.classes.push(generate_class(class, &mut dir));
    }
    dir
}

fn generate_class(class: &Class, dir: &mut DIR) -> IRClass {
    let mut ir_class = IRClass {
        name: class.name.clone(),
        super_name: class.superclass.clone(),
        fields: vec![],
        methods: vec![],
    };
    for field in &class.fields {
        ir_class.fields.push(field.clone());
    }
    for method in &class.methods {
        ir_class.methods.push(generate_method(method, dir));
    }
    ir_class
}
// TODO: Parallelize this, since methods are not dependent on each other(hopefully)
fn generate_method(method: &MethodDecl, dir: &mut DIR) -> CompiledMethod {
    let mut compiled_method = CompiledMethod {
        name: method.name.clone(),
        max_stack: 0,
        code: vec![],
    };
    for stmt in &method.body {
        compiled_method.code.append(&mut generate_stmt(stmt, dir));
    }
    compiled_method
}

fn generate_stmt(stmt: Stmt, dir: &mut DIR) -> Vec<Instruction> {
    todo!()
}

fn generate_code_stmt(stmt: &Stmt, code: &mut Vec<u8>) {
    match stmt {
        Stmt::Block(stmts) => {
            // Generate bytecode for block
        }
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