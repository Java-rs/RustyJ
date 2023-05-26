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

fn create_test_file(class: &Class, name: &str) {
    let file_path = format!("tests/{name}.java");
    let gen_file_path = format!("tests/{name}-gen.java");
    let class_file_path = format!("tests/{name}.class");

    // Generate Java Code from AST and write to file
    let class_code = class_to_java(class);
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
    serde_json::to_writer_pretty(&mut file, &class).expect("Couldn't serialize class");

    // let mut file =
    //     File::create(format!("tests/{name}-TAST.json")).expect("File couldn't be created");
    // serde_json::to_writer_pretty(&mut file, &class).expect("Couldn't serialize class");
}

#[test]
fn if_class() {
    let class = Class {
        name: "If".to_string(),
        fields: vec![],
        methods: vec![MethodDecl {
            name: "f".to_string(),
            params: vec![(Type::Char, "c".to_string())],
            ret_type: Type::Bool,
            body: Block(vec![
                If(
                    Binary(
                        "==".to_string(),
                        Box::new(LocalOrFieldVar("c".to_string())),
                        Box::new(Char('a')),
                    ),
                    Box::new(Return(Bool(true))),
                    None,
                ),
                Return(Bool(false)),
            ]),
        }],
    };
    create_test_file(&class, "If");
}

#[test]
fn arithmetic_methods_class() {
    let class = Class {
        name: "ArithmeticMethods".to_string(),
        fields: vec![
            FieldDecl {
                field_type: Type::Int,
                name: "x".to_string(),
                val: Some("69".to_string()),
            },
            FieldDecl {
                field_type: Type::Int,
                name: "y".to_string(),
                val: Some("420".to_string()),
            },
        ],
        methods: vec![
            MethodDecl {
                ret_type: Type::Int,
                name: "addX".to_string(),
                params: vec![(Type::Int, "a".to_string())],
                body: Block(vec![Return(Binary(
                    "+".to_string(),
                    Box::new(LocalOrFieldVar("x".to_string())),
                    Box::new(LocalOrFieldVar("a".to_string())),
                ))]),
            },
            MethodDecl {
                ret_type: Type::Int,
                name: "addY".to_string(),
                params: vec![(Type::Int, "a".to_string())],
                body: Block(vec![Return(Binary(
                    "+".to_string(),
                    Box::new(LocalOrFieldVar("y".to_string())),
                    Box::new(LocalOrFieldVar("a".to_string())),
                ))]),
            },
            MethodDecl {
                ret_type: Type::Int,
                name: "complexMath".to_string(),
                params: vec![(Type::Int, "a".to_string()), (Type::Int, "b".to_string())],
                body: Block(vec![
                    StmtExprStmt(Assign(
                        "a".to_string(),
                        Binary(
                            "*".to_string(),
                            Box::new(LocalOrFieldVar("y".to_string())),
                            Box::new(Binary(
                                "/".to_string(),
                                Box::new(Binary(
                                    "+".to_string(),
                                    Box::new(LocalOrFieldVar("a".to_string())),
                                    Box::new(LocalOrFieldVar("b".to_string())),
                                )),
                                Box::new(LocalOrFieldVar("x".to_string())),
                            )),
                        ),
                    )),
                    StmtExprStmt(Assign(
                        "b".to_string(),
                        Binary(
                            "+".to_string(),
                            Box::new(LocalOrFieldVar("a".to_string())),
                            Box::new(Unary(
                                "-".to_string(),
                                Box::new(LocalOrFieldVar("b".to_string())),
                            )),
                        ),
                    )),
                    StmtExprStmt(Assign(
                        "a".to_string(),
                        Binary(
                            "+".to_string(),
                            Box::new(LocalOrFieldVar("x".to_string())),
                            Box::new(Binary(
                                "*".to_string(),
                                Box::new(LocalOrFieldVar("b".to_string())),
                                Box::new(LocalOrFieldVar("a".to_string())),
                            )),
                        ),
                    )),
                    Return(Binary(
                        "*".to_string(),
                        Box::new(LocalOrFieldVar("x".to_string())),
                        Box::new(LocalOrFieldVar("a".to_string())),
                    )),
                ]),
            },
        ],
    };
    create_test_file(&class, "ArithmeticMethods");
}

#[test]
fn assigned_fields_class() {
    let class = Class {
        name: "AssignedFields".to_string(),
        fields: vec![
            FieldDecl {
                field_type: Type::Int,
                name: "x".to_string(),
                val: Some("69".to_string()),
            },
            FieldDecl {
                field_type: Type::Char,
                name: "c".to_string(),
                val: Some("x".to_string()),
            },
            FieldDecl {
                field_type: Type::String,
                name: "s".to_string(),
                val: Some("Hello World".to_string()),
            },
            FieldDecl {
                field_type: Type::String,
                name: "stringsCanBeNull".to_string(),
                val: Some("null".to_string()),
            },
            FieldDecl {
                field_type: Type::Bool,
                name: "b".to_string(),
                val: Some("true".to_string()),
            },
        ],
        methods: vec![],
    };
    create_test_file(&class, "AssignedFields");
}

#[test]
fn bool_alg_class() {
    let class = Class {
        name: "BoolAlg".to_string(),
        fields: vec![],
        methods: vec![MethodDecl {
            ret_type: Type::Bool,
            name: "f".to_string(),
            params: vec![
                (Type::Bool, "a".to_string()),
                (Type::Bool, "b".to_string()),
                (Type::Bool, "c".to_string()),
            ],
            body: Block(vec![Return(Binary(
                "||".to_string(),
                Box::new(Binary(
                    "&&".to_string(),
                    Box::new(Binary(
                        "&&".to_string(),
                        Box::new(LocalOrFieldVar("a".to_string())),
                        Box::new(LocalOrFieldVar("b".to_string())),
                    )),
                    Box::new(LocalOrFieldVar("c".to_string())),
                )),
                Box::new(LocalOrFieldVar("c".to_string())),
            ))]),
        }],
    };
    create_test_file(&class, "BoolAlg");
}

#[test]
fn complex_if_class() {
    let class = Class {
        name: "ComplexIf".to_string(),
        fields: vec![],
        methods: vec![MethodDecl {
            ret_type: Type::Bool,
            name: "f".to_string(),
            params: vec![(Type::Char, "c".to_string())],
            body: Block(vec![If(
                Binary(
                    "==".to_string(),
                    Box::new(LocalOrFieldVar("c".to_string())),
                    Box::new(Expr::Char('a')),
                ),
                Box::new(Block(vec![Return(Expr::Bool(true))])),
                Some(Box::new(If(
                    Binary(
                        "==".to_string(),
                        Box::new(LocalOrFieldVar("c".to_string())),
                        Box::new(Expr::Char('b')),
                    ),
                    Box::new(Block(vec![Return(Expr::Bool(false))])),
                    Some(Box::new(If(
                        Binary(
                            "==".to_string(),
                            Box::new(LocalOrFieldVar("c".to_string())),
                            Box::new(Expr::Char('c')),
                        ),
                        Box::new(Return(Expr::Bool(true))),
                        Some(Box::new(Block(vec![If(
                            Binary(
                                "||".to_string(),
                                Box::new(Binary(
                                    "==".to_string(),
                                    Box::new(LocalOrFieldVar("c".to_string())),
                                    Box::new(Expr::Char('d')),
                                )),
                                Box::new(Binary(
                                    "==".to_string(),
                                    Box::new(LocalOrFieldVar("c".to_string())),
                                    Box::new(Expr::Char('e')),
                                )),
                            ),
                            Box::new(Return(Expr::Bool(false))),
                            Some(Box::new(If(
                                Binary(
                                    "||".to_string(),
                                    Box::new(Binary(
                                        "==".to_string(),
                                        Box::new(LocalOrFieldVar("c".to_string())),
                                        Box::new(Expr::Char('d')),
                                    )),
                                    Box::new(Binary(
                                        "==".to_string(),
                                        Box::new(LocalOrFieldVar("c".to_string())),
                                        Box::new(Expr::Char('e')),
                                    )),
                                ),
                                Box::new(Return(Expr::Bool(true))),
                                Some(Box::new(Block(vec![Return(Expr::Bool(false))]))),
                            ))),
                        )]))),
                    ))),
                ))),
            )]),
        }],
    };
    create_test_file(&class, "ComplexIf");
}

#[test]
fn empty_class() {
    let class = Class {
        name: "Empty".to_string(),
        fields: vec![],
        methods: vec![],
    };
    create_test_file(&class, "Empty");
}

#[test]
fn empty_method_class() {
    let class = Class {
        name: "emptyMethod".to_string(),
        fields: vec![],
        methods: vec![MethodDecl {
            ret_type: Type::Void,
            name: "f".to_string(),
            params: vec![],
            body: Block(vec![]),
        }],
    };
    create_test_file(&class, "emptyMethod");
}

#[test]
fn fib_class() {
    let class = Class {
        name: "Fib".to_string(),
        fields: vec![],
        methods: vec![
            MethodDecl {
                ret_type: Type::Int,
                name: "rec".to_string(),
                params: vec![(Type::Int, "n".to_string())],
                body: Block(vec![If(
                    Binary(
                        "<".to_string(),
                        Box::new(LocalOrFieldVar("n".to_string())),
                        Box::new(Expr::Integer(2)),
                    ),
                    Box::new(Return(LocalOrFieldVar("n".to_string()))),
                    Some(Box::new(Block(vec![Return(Binary(
                        "+".to_string(),
                        Box::new(StmtExprExpr(Box::new(MethodCall(
                            Expr::This,
                            "rec".to_string(),
                            vec![Binary(
                                "-".to_string(),
                                Box::new(LocalOrFieldVar("n".to_string())),
                                Box::new(Expr::Integer(1)),
                            )],
                        )))),
                        Box::new(StmtExprExpr(Box::new(MethodCall(
                            Expr::This,
                            "rec".to_string(),
                            vec![Binary(
                                "-".to_string(),
                                Box::new(LocalOrFieldVar("n".to_string())),
                                Box::new(Expr::Integer(2)),
                            )],
                        )))),
                    ))]))),
                )]),
            },
            MethodDecl {
                ret_type: Type::Int,
                name: "iter".to_string(),
                params: vec![(Type::Int, "n".to_string())],
                body: Block(vec![
                    If(
                        Binary(
                            "<".to_string(),
                            Box::new(LocalOrFieldVar("n".to_string())),
                            Box::new(Expr::Integer(2)),
                        ),
                        Box::new(Return(LocalOrFieldVar("n".to_string()))),
                        None,
                    ),
                    LocalVarDecl(Type::Int, "x".to_string()),
                    StmtExprStmt(Assign("x".to_string(), Expr::Integer(0))),
                    LocalVarDecl(Type::Int, "y".to_string()),
                    StmtExprStmt(Assign("y".to_string(), Expr::Integer(1))),
                    LocalVarDecl(Type::Int, "i".to_string()),
                    StmtExprStmt(Assign("i".to_string(), Expr::Integer(1))),
                    While(
                        Binary(
                            "<".to_string(),
                            Box::new(LocalOrFieldVar("i".to_string())),
                            Box::new(LocalOrFieldVar("n".to_string())),
                        ),
                        Box::new(Block(vec![
                            LocalVarDecl(Type::Int, "next".to_string()),
                            StmtExprStmt(Assign(
                                "next".to_string(),
                                Binary(
                                    "+".to_string(),
                                    Box::new(LocalOrFieldVar("y".to_string())),
                                    Box::new(LocalOrFieldVar("x".to_string())),
                                ),
                            )),
                            StmtExprStmt(Assign("x".to_string(), LocalOrFieldVar("y".to_string()))),
                            StmtExprStmt(Assign(
                                "y".to_string(),
                                LocalOrFieldVar("next".to_string()),
                            )),
                            StmtExprStmt(Assign(
                                "i".to_string(),
                                Binary(
                                    "+".to_string(),
                                    Box::new(LocalOrFieldVar("i".to_string())),
                                    Box::new(Expr::Integer(1)),
                                ),
                            )),
                        ])),
                    ),
                    Return(LocalOrFieldVar("y".to_string())),
                ]),
            },
        ],
    };
    create_test_file(&class, "Fib");
}

// #[test]
// fn Fields_class() {
//     let class = Class {};
//     create_test_file(&class, "Fields");
// }

// #[test]
// fn IntFields_class() {
//     let class = Class {};
//     create_test_file(&class, "IntFields");
// }

// #[test]
// fn LocalVarDecl_class() {
//     let class = Class {};
//     create_test_file(&class, "LocalVarDecl");
// }

// #[test]
// fn MethodCall_class() {
//     let class = Class {};
//     create_test_file(&class, "MethodCall");
// }

// #[test]
// fn NamingConflict_class() {
//     let class = Class {};
//     create_test_file(&class, "NamingConflict");
// }

// #[test]
// fn Negator_class() {
//     let class = Class {};
//     create_test_file(&class, "Negator");
// }

// #[test]
// fn Return_class() {
//     let class = Class {};
//     create_test_file(&class, "Return");
// }

// #[test]
// fn SetterGetter_class() {
//     let class = Class {};
//     create_test_file(&class, "SetterGetter");
// }

// #[test]
// fn StrAdd_class() {
//     let class = Class {};
//     create_test_file(&class, "StrAdd");
// }

// #[test]
// fn While_class() {
//     let class = Class {};
//     create_test_file(&class, "While");
// }
