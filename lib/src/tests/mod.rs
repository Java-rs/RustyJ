#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(non_snake_case)]

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
mod naming_conflict_class;
mod negator_class;
mod return_class;
mod setter_getter_class;
mod tast_to_ast;
mod to_java;
mod while_class;
mod wonky_assignments_class;

use self::to_java::class_to_java;
use crate::codegen::*;
use crate::parser;
use crate::typechecker::typechecker::TypeChecker;
use crate::types::Expr::*;
use crate::types::Stmt::*;
use crate::types::StmtExpr::*;
use crate::types::*;
use std::fs::read;
use std::fs::File;
use std::io::read_to_string;
use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;
use tast_to_ast::*;

static mut RNG_STATE: u32 = 420;

fn xorshift() -> usize {
    unsafe {
        RNG_STATE ^= RNG_STATE << 13;
        RNG_STATE ^= RNG_STATE >> 17;
        RNG_STATE ^= RNG_STATE << 5;
        RNG_STATE as usize
    }
}

fn normalize_str(s: std::string::String) -> std::string::String {
    s.split('\n')
        .fold("".to_string(), |acc, s| acc + s)
        .split('\t')
        .fold("".to_string(), |acc, s| acc + s)
}

pub fn parser_test(ast: &Class, name: &str) {
    // Call parser with java code
    let parse_res = parser::parse_programm(
        &read_to_string(File::open(format!("lib/testcases/{name}.java")).unwrap()).unwrap(),
    )
    .unwrap();
    let parse_res = parse_res.get(0).unwrap();
    assert_eq!(parse_res, ast);
}

pub fn typechecker_test(ast: &Class, tast: &Class) {
    let mut tc = TypeChecker::new(vec![ast.clone()]).unwrap();
    let typed_classes = tc.check_and_type_program().unwrap();
    assert_eq!(typed_classes[0], *tast);
}

const TEST_VALS_AMOUNT: usize = 5;
static BOOL_TEST_VALS: [&str; 4] = ["true", "false", "false", "true"];
static CHAR_TEST_VALS: [&str; TEST_VALS_AMOUNT] = ["'c'", "'x'", "'!'", "'a'", "'f'"];
static STR_TEST_VALS: [&str; TEST_VALS_AMOUNT] = [
    "\"a\"",
    "\"test\"",
    "\"Hello World!?!\"",
    "\"A bit of escaping going on here... \\\"\"",
    "\"Just another string test\"",
];

pub fn get_test_val(t: Type, i: usize) -> std::string::String {
    match t {
        Type::Bool => BOOL_TEST_VALS[xorshift().wrapping_add(i) % BOOL_TEST_VALS.len()].to_string(),
        Type::Char => CHAR_TEST_VALS[i % CHAR_TEST_VALS.len()].to_string(),
        Type::Int => (xorshift().wrapping_add(i) % 30).to_string(),
        Type::Null => "null".to_string(),
        Type::Void => panic!("can't create a test value for parameters of type 'void'"),
        Type::String => STR_TEST_VALS[i % STR_TEST_VALS.len()].to_string(),
        Type::Class(name) => format!("new {name}()"),
    }
}

pub fn codegen_test(tast: &Class, name: &str) {
    // Create code to run tests on generated class file
    let mut java_code = format!(
        "class {name}Test {{\npublic static void main(String[] args) {{\n{} m = new {}();\n",
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
                .map(|(j, p)| get_test_val(p.0.clone(), i * j))
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

    File::create(format! {"lib/testcases/{name}Test.java"})
        .expect("failed to create Test.java")
        .write_all(java_code.as_bytes())
        .expect("failed to write to Test.java");

    // Compile original java code for expected result
    compile_java(name);
    let mut expected_bytes = vec![];
    File::open(format!("lib/testcases/{name}.class"))
        .unwrap()
        .read_to_end(&mut expected_bytes)
        .unwrap();
    println!("Expected bytes:  {:?}", expected_bytes);
    disassemble_java(name, &format!("{name}")); // Probably useful for debugging
    compile_java(&format!("{name}Test"));
    let expected_out = run_java(&format!("{name}Test"));

    // Compile & run tests on generated DIR
    let mut dir = generate_dir(&vec![tast.clone()]);
    let generated_bytes = dir.as_bytes();
    File::create(format!("lib/testcases/{name}.class"))
        .unwrap_or_else(|_| panic!("failed to create {name}.class"))
        .write_all(&generated_bytes)
        .unwrap_or_else(|_| panic!("failed to write generated DIR into {name}.class"));
    println!("Generated bytes: {:?}", generated_bytes);
    disassemble_java(name, &format!("{name}-codegen")); // Probably useful for debugging
    compile_java(&format!("{name}Test"));
    let codegen_out = run_java(&format!("{name}Test"));

    let (codegen_out, expected_out) = dbg!(codegen_out, expected_out);
    assert_eq!(codegen_out.status.code().unwrap(), 0);
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

    // Generate Java Code from AST and write to file
    let class_code = class_to_java(ast);
    println!("{class_code}");
    let mut file = File::create(format!("lib/testcases/{name}.java"))
        .expect("failed to open original java file for writing generated code");
    file.write_all(class_code.as_bytes())
        .expect("failed to write generated java code in original java file");
    let mut file = File::create(format!("lib/testcases/{name}-gen.java")) // Only for debugging tests
        .expect("failed to open generated java file for writing generated code");
    file.write_all(class_code.as_bytes())
        .expect("failed to write generated java code in generated java file");
    compile_java(name);
    let gen_clz = disassemble_java(name, &format!("{name}-gen"));

    // Compile original java code
    let mut file = File::create(format!("lib/testcases/{name}.java"))
        .expect("failed to open original java file for writing");
    file.write_all(og_java_code.as_bytes())
        .expect("failed to write original java code back");
    compile_java(name);
    let og_clz = disassemble_java(name, &format!("{name}"));

    assert_eq!(og_clz, gen_clz);
}

fn run_java(name: &str) -> std::process::Output {
    Command::new("java")
        // .arg("-noverify")
        .arg(name)
        .current_dir("lib/testcases/")
        .output()
        .unwrap_or_else(|_| panic!("failed to run 'java {name}'"))
}

fn compile_java(name: &str) {
    let mut child = Command::new("javac")
        .current_dir("lib/testcases/")
        .arg(format!("{name}.java"))
        .arg("-g:none")
        .spawn()
        .unwrap_or_else(|_| panic!("failed to compile {name}.java"));
    let ecode = child
        .wait()
        .unwrap_or_else(|_| panic!("failed to wait on child compiling {name}.java"));
    assert!(
        ecode.success(),
        "Failed to compile {name}.java with exit code {ecode}"
    );
    assert_eq!(ecode.code().unwrap(), 0);
}

// Decompiles the class file, writing the result in lib/testcases/out.txt and returning the bytes that were read from the class file
fn disassemble_java(name: &str, out: &str) -> Vec<u8> {
    let clz_file_path = format!("lib/testcases/{name}.class");
    let clz_file = read(clz_file_path.clone()).expect("failed to read generated java class file");
    let file = File::create(format!("lib/testcases/{out}.txt"))
        .unwrap_or_else(|_| panic!("failed to create {out}.txt"));
    let mut child = Command::new("javap")
        .arg("-v")
        .arg("-c")
        .arg(clz_file_path)
        .stdout(Stdio::from(file))
        .spawn()
        .unwrap_or_else(|_| panic!("failed to disassemble {name}.java"));
    let ecode = child
        .wait()
        .unwrap_or_else(|_| panic!("failed to wait on child disassembling {name}.java"));
    assert!(ecode.success());
    assert_eq!(ecode.code().unwrap(), 0);
    clz_file
}
