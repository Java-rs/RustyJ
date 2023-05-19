use crate::typechecker::*;
use crate::types::*;

pub fn generate_code(ast: &Prg) -> Vec<u8> {
    let mut code = Vec::new();
    generate_code_recursive(&ast, &mut code);
    code
}

fn generate_code_recursive(node: &Prg, code: &mut Vec<u8>) {
    match node {
        Prg::Expr(expr) => generate_code_expr(expr, code),
        Prg::Stmt(stmt) => generate_code_stmt(stmt, code),
        Prg::Field(field) => generate_code_fielddecl(field, code),
        Prg::Type(types) => generate_code_type(types, code),
        Prg::MethodDecl(methoddecl) => generate_code_methoddecl(methoddecl, code),
        Prg::FieldDecl(fielddecl) => generate_code_fielddecl(fielddecl, code),
    }
}
fn generate_code_class(class: &Class, code: &mut Vec<u8>) {
    match class {
        Class::Class {
            name,
            super_name,
            fields,
            methods,
        } => {
            // Generate bytecode for class
        } // ...
    }
}
fn generate_code_fielddecl(fielddecl: &FieldDecl, code: &mut Vec<u8>) {
    match fielddecl {
        FieldDecl::FieldDecl { name, types, expr } => {
            // Generate bytecode for field declaration
        } // ...
    }
}
fn generate_code_methoddecl(methoddecl: &MethodDecl, code: &mut Vec<u8>) {
    match methoddecl {
        MethodDecl::MethodDecl {
            name,
            ret_type,
            params,
            body,
        } => {
            // Generate bytecode for method declaration
        } // ...
    }
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
