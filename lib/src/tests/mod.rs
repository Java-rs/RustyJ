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
mod while_class;

use self::to_java::class_to_java;
use crate::types::Expr::*;
use crate::types::Stmt::*;
use crate::types::StmtExpr::*;
use crate::types::*;
use std::fs::read;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::read_to_string;
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

pub fn single_class_test(ast: &Class, tast: Option<&Class>, name: &str) {
    // Write AST & TAST to files
    let mut file =
        File::create(format!("tests/{name}-AST.json")).expect("File failed to be created");
    serde_json::to_writer_pretty(&mut file, &ast).expect("failed to serialize class");

    if let Some(tast) = tast {
        let mut file =
            File::create(format!("tests/{name}-TAST.json")).expect("File failed to be created");
        serde_json::to_writer_pretty(&mut file, tast).expect("failed to serialize class");
    };

    // Load orignal java code
    let file = File::open(format!("tests/{name}.java")).expect("failed to open original java file");
    let og_java_code = read_to_string(file).expect("failed to read original java file");

    let res = test_helper(ast, tast, name, &og_java_code);

    if let Err(msg) = res {
        let mut file = File::create(format!("tests/{name}.java"))
            .expect("failed to open original java file for writing");
        file.write(og_java_code.as_bytes())
            .expect("failed to write the original java code back into its file");
        panic!("{msg}");
    }
}

fn test_helper(
    ast: &Class,
    tast: Option<&Class>,
    name: &str,
    og_java_code: &str,
) -> Result<(), std::string::String> {
    // Generate Java Code from AST and write to file
    let class_code = class_to_java(ast);
    let mut file = File::create(format!("tests/{name}.java"))
        .expect("failed to open original java file for writing generated code");
    println!("Generated code: {class_code}");
    file.write(class_code.as_bytes())
        .map_err(|x| "failed to write generated java code".to_string())?;

    // Compile generated java code
    let mut child = Command::new("javac")
        .arg(format!("tests/{name}.java"))
        .arg("-g:none")
        .spawn()
        .map_err(|x| "failed to compile generated java-code".to_string())?;
    let ecode = child
        .wait()
        .map_err(|x| "failed to wait on child compiling generated java code".to_string())?;
    assert!(ecode.success());
    let gen_clz_file = read(format!("tests/{name}.class"))
        .map_err(|x| "failed to read generated java class file".to_string())?;
    let mut file = File::create(format!("tests/{name}-gen.txt")).unwrap();
    let mut child = Command::new("javap")
        .arg("-v")
        .arg("-c")
        .arg(format!("tests/{name}.class"))
        .stdout(Stdio::from(file))
        .spawn()
        .map_err(|x| "failed to disassemble generated java class file".to_string())?;
    let ecode = child
        .wait()
        .map_err(|x| "failed to wait on child decompiling generated java code".to_string())?;
    assert!(ecode.success());

    // Compile original java code
    let mut file = File::create(format!("tests/{name}.java"))
        .expect("failed to open original java file for writing");
    file.write(og_java_code.as_bytes())
        .map_err(|x| "failed to write original java code back".to_string())?;
    let mut child = Command::new("javac")
        .arg(format!("tests/{name}.java"))
        .arg("-g:none")
        .spawn()
        .map_err(|x| "failed to compile original java-code".to_string())?;
    let ecode = child
        .wait()
        .map_err(|x| "failed to wait on child compiling original java code".to_string())?;
    assert!(ecode.success());
    let og_clz_file = read(format!("tests/{name}.class"))
        .map_err(|x| "failed to read original java class file".to_string())?;
    let mut file = File::create(format!("tests/{name}.txt")).unwrap();
    let mut child = Command::new("javap")
        .arg("-v")
        .arg("-c")
        .arg(format!("tests/{name}.class"))
        .stdout(Stdio::from(file))
        .spawn()
        .map_err(|x| "failed to disassemble original java class file".to_string())?;
    let ecode = child
        .wait()
        .map_err(|x| "failed to wait on child compiling original java code".to_string())?;
    assert!(ecode.success());

    assert_eq!(og_clz_file, gen_clz_file);
    Ok(())
}

// use super::*\n#[test]
// fn Fields_class() {
//     let class = Class {};
//     single_class_test(&tast_to_ast(&class), Some(&class), "Fields");
// }

// use super::*\n#[test]
// fn IntFields_class() {
//     let class = Class {};
//     single_class_test(&tast_to_ast(&class), Some(&class), "IntFields");
// }

// use super::*\n#[test]
// fn LocalVarDecl_class() {
//     let class = Class {};
//     single_class_test(&tast_to_ast(&class), Some(&class), "LocalVarDecl");
// }

// use super::*\n#[test]
// fn MethodCall_class() {
//     let class = Class {};
//     single_class_test(&tast_to_ast(&class), Some(&class), "MethodCall");
// }

// use super::*\n#[test]
// fn NamingConflict_class() {
//     let class = Class {};
//     single_class_test(&tast_to_ast(&class), Some(&class), "NamingConflict");
// }

// use super::*\n#[test]
// fn Negator_class() {
//     let class = Class {};
//     single_class_test(&tast_to_ast(&class), Some(&class), "Negator");
// }

// use super::*\n#[test]
// fn Return_class() {
//     let class = Class {};
//     single_class_test(&tast_to_ast(&class), Some(&class), "Return");
// }

// use super::*\n#[test]
// fn SetterGetter_class() {
//     let class = Class {};
//     single_class_test(&tast_to_ast(&class), Some(&class), "SetterGetter");
// }

// use super::*\n#[test]
// fn StrAdd_class() {
//     let class = Class {};
//     single_class_test(&tast_to_ast(&class), Some(&class), "StrAdd");
// }

// use super::*\n#[test]
// fn While_class() {
//     let class = Class {};
//     single_class_test(&tast_to_ast(&class), Some(&class), "While");
// }
