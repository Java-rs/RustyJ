mod arithmetic_methods_class;
mod assigned_fields_class;
mod bool_alg_class;
mod complex_if_class;
mod empty_class;
mod empty_method_class;
mod fib_class;
mod fields_class;
mod if_class;
mod int_fields_class;
mod local_var_decl_class;
mod method_call_class;
mod to_java;

use self::to_java::class_to_java;
use crate::types::Expr::*;
use crate::types::Stmt::*;
use crate::types::StmtExpr::*;
use crate::types::*;
use std::fs::read;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

fn normalize_str(s: std::string::String) -> std::string::String {
    s.split('\n')
        .fold("".to_string(), |acc, s| acc + s)
        .split("\t")
        .fold("".to_string(), |acc, s| acc + s)
}

fn stmt_tast_to_ast(stmt: &Stmt) -> Stmt {
    match stmt {
        Stmt::TypedStmt(x, typ) => stmt_tast_to_ast(x),
        Stmt::Block(stmts) => Block(stmts.iter().map(|x| stmt_tast_to_ast(x)).collect()),
        Stmt::Return(expr) => Return(expr_tast_to_ast(expr)),
        Stmt::While(cond, body) => While(expr_tast_to_ast(cond), Box::new(stmt_tast_to_ast(body))),
        Stmt::If(cond, body, elze) => If(
            expr_tast_to_ast(cond),
            Box::new(stmt_tast_to_ast(body)),
            match elze {
                Some(x) => Some(Box::new(stmt_tast_to_ast(x))),
                None => None,
            },
        ),
        Stmt::StmtExprStmt(stmt_expr) => StmtExprStmt(stmt_expr_tast_to_ast(stmt_expr)),
        default => stmt.clone(),
    }
}

fn stmt_expr_tast_to_ast(stmt_expr: &StmtExpr) -> StmtExpr {
    match stmt_expr {
        StmtExpr::Assign(var, val) => Assign(var.clone(), expr_tast_to_ast(val)),
        StmtExpr::New(typ, params) => New(
            typ.clone(),
            params.iter().map(|x| expr_tast_to_ast(x)).collect(),
        ),
        StmtExpr::MethodCall(obj, method, params) => MethodCall(
            expr_tast_to_ast(obj),
            method.clone(),
            params.iter().map(|x| expr_tast_to_ast(x)).collect(),
        ),
        StmtExpr::TypedStmtExpr(x, typ) => stmt_expr_tast_to_ast(x),
    }
}

fn expr_tast_to_ast(expr: &Expr) -> Expr {
    match expr {
        Expr::InstVar(x, s) => InstVar(Box::new(expr_tast_to_ast(x)), s.clone()),
        Expr::Unary(s, x) => Unary(s.clone(), Box::new(expr_tast_to_ast(x))),
        Expr::Binary(op, l, r) => Binary(
            op.clone(),
            Box::new(expr_tast_to_ast(l)),
            Box::new(expr_tast_to_ast(r)),
        ),
        Expr::StmtExprExpr(x) => StmtExprExpr(Box::new(stmt_expr_tast_to_ast(x))),
        Expr::TypedExpr(x, t) => expr_tast_to_ast(x),
        default => expr.clone(),
    }
}

fn tast_to_ast(class: &Class) -> Class {
    Class {
        name: class.name.clone(),
        fields: class.fields.clone(),
        methods: class
            .methods
            .clone()
            .into_iter()
            .map(|method| MethodDecl {
                ret_type: method.ret_type.clone(),
                name: method.name.clone(),
                params: method.params.clone(),
                body: stmt_tast_to_ast(&method.body),
            })
            .collect(),
    }
}

fn create_test_file(ast: &Class, tast: Option<&Class>, name: &str) {
    let file_path = format!("tests/{name}.java");
    let gen_file_path = format!("tests/{name}-gen.java");
    let class_file_path = format!("tests/{name}.class");

    // Generate Java Code from AST and write to file
    let class_code = class_to_java(ast);
    let mut file =
        File::create(gen_file_path.clone()).expect("File for generated code couldn't be created");
    file.write(class_code.as_bytes())
        .expect("Couldn't write generate java code");

    // TODO: Check that generated java code and original java code are equivalent to javac
    let mut child = Command::new("javac")
        .arg(file_path)
        .spawn()
        .expect("failed to compile original java-code");
    let ecode = child
        .wait()
        .expect("failed to wait on child compiling original java code");
    assert!(ecode.success());
    let mut file = File::create(format!("tests/{name}.txt")).unwrap();
    let mut child = Command::new("javap")
        .arg("-v")
        .arg("-c")
        .arg(format!("tests/{}.class", name))
        .stdout(Stdio::from(file))
        .spawn()
        .expect("failed to disassemble original java class file");
    let ecode = child
        .wait()
        .expect("failed to wait on child compiling original java code");
    assert!(ecode.success());
    // let og_clz_file =
    //     read(class_file_path.clone()).expect("failed to read original java class file");
    let mut child = Command::new("javac")
        .arg(gen_file_path)
        .spawn()
        .expect("failed to compile generated java-code");
    let ecode = child
        .wait()
        .expect("failed to wait on child compiling generated java code");
    assert!(ecode.success());
    let mut file = File::create(format!("tests/{name}-gen.txt")).unwrap();
    let mut child = Command::new("javap")
        .arg("-v")
        .arg("-c")
        .arg(format!("tests/{}.class", name))
        .stdout(Stdio::from(file))
        .spawn()
        .expect("failed to disassemble original java class file");
    let ecode = child
        .wait()
        .expect("failed to wait on child compiling original java code");
    assert!(ecode.success());
    // let gen_clz_file = read(class_file_path).expect("failed to read generated java class file");
    // assert_eq!(og_clz_file, gen_clz_file);

    // Write AST & TAST to files
    let mut file =
        File::create(format!("tests/{name}-AST.json")).expect("File couldn't be created");
    serde_json::to_writer_pretty(&mut file, &ast).expect("Couldn't serialize class");

    if let Some(tast) = tast {
        let mut file =
            File::create(format!("tests/{name}-TAST.json")).expect("File couldn't be created");
        serde_json::to_writer_pretty(&mut file, tast).expect("Couldn't serialize class");
    }
}

// use super::*\n#[test]
// fn Fields_class() {
//     let class = Class {};
//     create_test_file(&tast_to_ast(&class), Some(&class), "Fields");
// }

// use super::*\n#[test]
// fn IntFields_class() {
//     let class = Class {};
//     create_test_file(&tast_to_ast(&class), Some(&class), "IntFields");
// }

// use super::*\n#[test]
// fn LocalVarDecl_class() {
//     let class = Class {};
//     create_test_file(&tast_to_ast(&class), Some(&class), "LocalVarDecl");
// }

// use super::*\n#[test]
// fn MethodCall_class() {
//     let class = Class {};
//     create_test_file(&tast_to_ast(&class), Some(&class), "MethodCall");
// }

// use super::*\n#[test]
// fn NamingConflict_class() {
//     let class = Class {};
//     create_test_file(&tast_to_ast(&class), Some(&class), "NamingConflict");
// }

// use super::*\n#[test]
// fn Negator_class() {
//     let class = Class {};
//     create_test_file(&tast_to_ast(&class), Some(&class), "Negator");
// }

// use super::*\n#[test]
// fn Return_class() {
//     let class = Class {};
//     create_test_file(&tast_to_ast(&class), Some(&class), "Return");
// }

// use super::*\n#[test]
// fn SetterGetter_class() {
//     let class = Class {};
//     create_test_file(&tast_to_ast(&class), Some(&class), "SetterGetter");
// }

// use super::*\n#[test]
// fn StrAdd_class() {
//     let class = Class {};
//     create_test_file(&tast_to_ast(&class), Some(&class), "StrAdd");
// }

// use super::*\n#[test]
// fn While_class() {
//     let class = Class {};
//     create_test_file(&tast_to_ast(&class), Some(&class), "While");
// }
