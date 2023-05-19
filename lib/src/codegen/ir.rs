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
        Prg::Field(field) => generate_code_field(field, code),
        Prg::Type(types) => generate_code_type(types, code),
        Prg::MethodDecl(methoddecl) => generate_code_methoddecl(methoddecl, code),
        Prg::FieldDecl(fielddecl) => generate_code_fielddecl(fielddecl, code),
    }
}

fn generate_code_expr(expr: &Expr, code: &mut Vec<u8>) {
    match expr {
        Expr::IntLiteral(value) => {
            // Generate bytecode for int literal
        }
        Expr::BoolLiteral(value) => {
            // Generate bytecode for bool literal
        } // ...
    }
}

fn generate_code_stmt(stmt: &Stmt, code: &mut Vec<u8>) {
    match stmt {
        Stmt {} => {
            // Generate bytecode for if statement
        } // ...
    }
}
fn generate_code_stmt_expr(stmt_expr: &StmtExpr, code: &mut Vec<u8>) {
    match stmt {
        Stmt {} => {
            // Generate bytecode for if statement
        } // ...
    }
}
fn generate_code_class(class: &Class, code: &mut Vec<u8>) {
    match class {
        Class::Class {
            name,
            fields,
            methods,
        } => {
            // Generate bytecode for class
        } // ...
    }
}
fn generate_code_type(types: &Type, code: &mut Vec<u8>) {
    match types {
        Method::MethodDecl {
            name,
            ret_type,
            params,
            body,
        } => {
            // Generate bytecode for method declaration
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
fn generate_code_fielddecl(fielddecl: &FieldDecl, code: &mut Vec<u8>) {
    match fielddecl {
        FieldDecl::FieldDecl {
            name,
            field_type,
            value,
        } => {
            // Generate bytecode for field declaration
        } // ...
    }
}
