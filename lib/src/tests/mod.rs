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
mod tast_to_ast;
mod to_java;
mod while_class;

use self::to_java::class_to_java;
use crate::codegen;
use crate::codegen::*;
use crate::parser;
use crate::typechecker::typechecker::TypeChecker;
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
use tast_to_ast::*;

fn normalize_str(s: std::string::String) -> std::string::String {
    s.split('\n')
        .fold("".to_string(), |acc, s| acc + s)
        .split("\t")
        .fold("".to_string(), |acc, s| acc + s)
}

pub fn parser_test(ast: &Class, name: &str) {
    // Call parser with java code
    // TODO: Can only be done, once we have a parsing method that returns a Class
    let parse_res = parser::parse_Programm(
        &read_to_string(File::open(format!("testcases/{name}.java")).unwrap()).unwrap(),
    )
    .unwrap();
    let parse_res = parse_res.get(0).unwrap();
    assert_eq!(parse_res, ast);
}

pub fn typechecker_test(ast: &Class, tast: &Class) {
    // TODO: Errors are just ignored for now, oops
    let mut tc = TypeChecker::new(vec![ast.clone()]).unwrap();
    tc.check_and_type_program().unwrap();
    let v: Vec<&Class> = tc.typed_classes.values().collect();
    let typed = v[0];
    println!("{}", typed);
    assert_eq!(*typed, *ast);
}

pub fn codegen_test(tast: &Class, name: &str) {
    // TODO: I have not decided to how to test the codegen yet
    // ir = generate_dir(&vec![tast]);
}

pub fn class_test(ast: &Class, tast: Option<&Class>, name: &str) {
    // Write AST & TAST to files
    let mut file =
        File::create(format!("testcases/{name}-AST.json")).expect("File failed to be created");
    serde_json::to_writer_pretty(&mut file, &ast).expect("failed to serialize class");

    if let Some(tast) = tast {
        let mut file =
            File::create(format!("testcases/{name}-TAST.json")).expect("File failed to be created");
        serde_json::to_writer_pretty(&mut file, tast).expect("failed to serialize class");
    };

    // Load orignal java code
    let file =
        File::open(format!("testcases/{name}.java")).expect("failed to open original java file");
    let og_java_code = read_to_string(file).expect("failed to read original java file");

    let res = test_helper(ast, tast, name, &og_java_code);

    if let Err(msg) = res {
        let mut file = File::create(format!("testcases/{name}.java"))
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
    let mut file = File::create(format!("testcases/{name}.java"))
        .expect("failed to open original java file for writing generated code");
    file.write(class_code.as_bytes())
        .map_err(|x| "failed to write generated java code in original java file".to_string())?;
    let mut file = File::create(format!("testcases/{name}-gen.java")) // Only for debugging tests
        .expect("failed to open generated java file for writing generated code");
    file.write(class_code.as_bytes())
        .map_err(|x| "failed to write generated java code in generated java file".to_string())?;

    // Compile generated java code
    let mut child = Command::new("javac")
        .arg(format!("testcases/{name}.java"))
        .arg("-g:none")
        .spawn()
        .map_err(|x| "failed to compile generated java-code".to_string())?;
    let ecode = child
        .wait()
        .map_err(|x| "failed to wait on child compiling generated java code".to_string())?;
    assert!(ecode.success());
    let gen_clz_file = read(format!("testcases/{name}.class"))
        .map_err(|x| "failed to read generated java class file".to_string())?;
    let mut file = File::create(format!("testcases/{name}-gen.txt")).unwrap();
    let mut child = Command::new("javap")
        .arg("-v")
        .arg("-c")
        .arg(format!("testcases/{name}.class"))
        .stdout(Stdio::from(file))
        .spawn()
        .map_err(|x| "failed to disassemble generated java class file".to_string())?;
    let ecode = child
        .wait()
        .map_err(|x| "failed to wait on child decompiling generated java code".to_string())?;
    assert!(ecode.success());

    // Compile original java code
    let mut file = File::create(format!("testcases/{name}.java"))
        .expect("failed to open original java file for writing");
    file.write(og_java_code.as_bytes())
        .map_err(|x| "failed to write original java code back".to_string())?;
    let mut child = Command::new("javac")
        .arg(format!("testcases/{name}.java"))
        .arg("-g:none")
        .spawn()
        .map_err(|x| "failed to compile original java-code".to_string())?;
    let ecode = child
        .wait()
        .map_err(|x| "failed to wait on child compiling original java code".to_string())?;
    assert!(ecode.success());
    let og_clz_file = read(format!("testcases/{name}.class"))
        .map_err(|x| "failed to read original java class file".to_string())?;
    let mut file = File::create(format!("testcases/{name}.txt")).unwrap();
    let mut child = Command::new("javap")
        .arg("-v")
        .arg("-c")
        .arg(format!("testcases/{name}.class"))
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

// use super::*
// #[test]
// fn Fields_class() {
//     let class = Class {};
//     class_test(&tast_to_ast(&class), Some(&class), "Fields");
// }

// use super::*
// #[test]
// fn IntFields_class() {
//     let class = Class {};
//     class_test(&tast_to_ast(&class), Some(&class), "IntFields");
// }

// use super::*
// #[test]
// fn LocalVarDecl_class() {
//     let class = Class {};
//     class_test(&tast_to_ast(&class), Some(&class), "LocalVarDecl");
// }

// use super::*
// #[test]
// fn MethodCall_class() {
//     let class = Class {};
//     class_test(&tast_to_ast(&class), Some(&class), "MethodCall");
// }

// use super::*
// #[test]
// fn NamingConflict_class() {
//     let class = Class {};
//     class_test(&tast_to_ast(&class), Some(&class), "NamingConflict");
// }

// use super::*
// #[test]
// fn Negator_class() {
//     let class = Class {};
//     class_test(&tast_to_ast(&class), Some(&class), "Negator");
// }

// use super::*
// #[test]
// fn Return_class() {
//     let class = Class {};
//     class_test(&tast_to_ast(&class), Some(&class), "Return");
// }

// use super::*
// #[test]
// fn SetterGetter_class() {
//     let class = Class {};
//     class_test(&tast_to_ast(&class), Some(&class), "SetterGetter");
// }

// use super::*
// #[test]
// fn StrAdd_class() {
//     let class = Class {};
//     class_test(&tast_to_ast(&class), Some(&class), "StrAdd");
// }

// use super::*
// #[test]
// fn While_class() {
//     let class = Class {};
//     class_test(&tast_to_ast(&class), Some(&class), "While");
// }
