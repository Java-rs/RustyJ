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
    // let parse_res = parser::parse(&read_to_string(File::open(format!("lib/testcases/{name}.java"))));
    // assert_eq!(parse_res, ast);
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

const TEST_VALS_AMOUNT: usize = 5;
static BOOL_TEST_VALS: [&str; 2] = ["true", "false"];
static CHAR_TEST_VALS: [&str; TEST_VALS_AMOUNT] = ["'c'", "'x'", "'!'", "'a'", "'f'"];
static INT_TEST_VALS: [&str; TEST_VALS_AMOUNT] = ["5", "8", "257", "0", "69"];
static STR_TEST_VALS: [&str; TEST_VALS_AMOUNT] = [
    "\"a\"",
    "\"test\"",
    "\"Hello World!?!\"",
    "\"A bit of escaping going on here... \\\"\"",
    "Just another string test",
];

pub fn get_test_val(t: Type, i: usize) -> std::string::String {
    match t {
        Type::Bool => BOOL_TEST_VALS[i % BOOL_TEST_VALS.len()].to_string(),
        Type::Char => CHAR_TEST_VALS[i % CHAR_TEST_VALS.len()].to_string(),
        Type::Int => INT_TEST_VALS[i % INT_TEST_VALS.len()].to_string(),
        Type::Null => "null".to_string(),
        Type::Void => panic!("can't create a test value for parameters of type 'void'"),
        Type::String => STR_TEST_VALS[i % STR_TEST_VALS.len()].to_string(),
        Type::Class(name) => format!("new {name}()"),
    }
}

pub fn codegen_test(tast: &Class, name: &str) {
    // Create code to run tests on generated class file
    let mut java_code = format!(
        "class Test {{\npublic static void main(String[] args) {{\n{} m = new {}();\n",
        tast.name, tast.name
    );
    for method in tast.methods.iter() {
        let n = if method.params.is_empty() {
            1
        } else {
            TEST_VALS_AMOUNT
        };
        for i in 0..n {
            let test_inputs: Vec<std::string::String> = method
                .params
                .iter()
                .enumerate()
                .map(|(i, p)| get_test_val(p.0.clone(), i))
                .collect();
            let test_inputs = test_inputs.join(",");
            let method_call = format!("m.{}({})", method.name, test_inputs);
            if method.ret_type == Type::Void {
                java_code.push_str(&method_call);
            } else {
                java_code.push_str(&format!("System.out.println({method_call})"));
            }
            java_code.push_str(";\n");
        }
    }
    java_code.push_str("}}");

    File::create("lib/testcases/Test.java")
        .expect("failed to create Test.java")
        .write(java_code.as_bytes())
        .expect("failed to write to Test.java");
    // Compile & run tests on generated DIR
    let mut dir = generate_dir(&vec![tast.clone()]);
    File::create(format!("lib/testcases/{name}.class"))
        .expect(&format!("failed to create {name}.class"))
        .write(&dir.as_bytes())
        .expect(&format!("failed to write generated DIR into {name}.class"));
    disassemble_java(name, &format!("{name}-codegen")); // Probably useful for debugging
    compile_java("Test");
    let codegen_out = run_java("Test");
    // Compile original java code for expected result
    compile_java(name);
    compile_java("Test");
    let expected_out = run_java("Test");

    assert_eq!(codegen_out.status, expected_out.status);
    assert_eq!(codegen_out.stderr, expected_out.stderr);
    assert_eq!(codegen_out.stdout, expected_out.stdout);
}

pub fn class_test(ast: &Class, tast: Option<&Class>, name: &str) {
    // Write AST & TAST to files
    let mut file =
        File::create(format!("lib/testcases/{name}-AST.json")).expect("File failed to be created");
    serde_json::to_writer_pretty(&mut file, &ast).expect("failed to serialize class");

    if let Some(tast) = tast {
        let mut file = File::create(format!("lib/testcases/{name}-TAST.json"))
            .expect("File failed to be created");
        serde_json::to_writer_pretty(&mut file, tast).expect("failed to serialize class");
    };

    // Load orignal java code
    let file = File::open(format!("lib/testcases/{name}.java"))
        .expect("failed to open original java file");
    let og_java_code = read_to_string(file).expect("failed to read original java file");

    let res = test_helper(ast, tast, name, &og_java_code);
}

fn test_helper(ast: &Class, tast: Option<&Class>, name: &str, og_java_code: &str) {
    // Generate Java Code from AST and write to file
    let class_code = class_to_java(ast);
    let mut file = File::create(format!("lib/testcases/{name}.java"))
        .expect("failed to open original java file for writing generated code");
    file.write(class_code.as_bytes())
        .expect("failed to write generated java code in original java file");
    let mut file = File::create(format!("lib/testcases/{name}-expected.java")) // Only for debugging tests
        .expect("failed to open generated java file for writing generated code");
    file.write(class_code.as_bytes())
        .expect("failed to write generated java code in generated java file");

    // Compile generated java code
    compile_java(name);
    let gen_clz = disassemble_java(name, &format!("{name}-expected"));

    // Compile original java code
    let mut file = File::create(format!("lib/testcases/{name}.java"))
        .expect("failed to open original java file for writing");
    file.write(og_java_code.as_bytes())
        .expect("failed to write original java code back");
    compile_java(name);
    let og_clz = disassemble_java(name, &format!("{name}"));

    assert_eq!(og_clz, gen_clz);
}

fn run_java(name: &str) -> std::process::Output {
    Command::new("java")
        .arg(name)
        .output()
        .expect(&format!("failed to run 'java {name}'"))
}

fn compile_java(name: &str) {
    let mut child = Command::new("javac")
        .arg(format!("lib/testcases/{name}.java"))
        .arg("-g:none")
        .spawn()
        .expect(&format!("failed to compile {name}.java"));
    let ecode = child
        .wait()
        .expect(&format!("failed to wait on child compiling {name}.java"));
    assert!(ecode.success());
}

// Decompiles the class file, writing the result in lib/testcases/out.txt and returning the bytes that were read from the class file
fn disassemble_java(name: &str, out: &str) -> Vec<u8> {
    let clz_file_path = format!("lib/testcases/{name}.class");
    let clz_file = read(clz_file_path.clone()).expect("failed to read generated java class file");
    let mut file = File::create(format!("lib/testcases/{out}.txt"))
        .expect(&format!("failed to create {out}.txt"));
    let mut child = Command::new("javap")
        .arg("-v")
        .arg("-c")
        .arg(clz_file_path)
        .stdout(Stdio::from(file))
        .spawn()
        .expect(&format!("failed to disassemble {name}.java"));
    let ecode = child.wait().expect(&format!(
        "failed to wait on child disassembling {name}.java"
    ));
    assert!(ecode.success());
    clz_file
}
