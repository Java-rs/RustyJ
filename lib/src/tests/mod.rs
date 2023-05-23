use crate::types::Expr::*;
use crate::types::Stmt::*;
use crate::types::StmtExpr::*;
use crate::types::*;
use std::fs::File;

fn create_test_file(class: &Class, name: &str) {
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
            retType: Type::Bool,
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
                // TODO: Add assignment of 69
            },
            FieldDecl {
                field_type: Type::Int,
                name: "y".to_string(),
                // TODO: Add assignment of 420
            },
        ],
        methods: vec![
            MethodDecl {
                retType: Type::Int,
                name: "addX".to_string(),
                params: vec![(Type::Int, "a".to_string())],
                body: Block(vec![Return(Binary(
                    "+".to_string(),
                    Box::new(LocalOrFieldVar("x".to_string())),
                    Box::new(LocalOrFieldVar("a".to_string())),
                ))]),
            },
            MethodDecl {
                retType: Type::Int,
                name: "addY".to_string(),
                params: vec![(Type::Int, "a".to_string())],
                body: Block(vec![Return(Binary(
                    "+".to_string(),
                    Box::new(LocalOrFieldVar("y".to_string())),
                    Box::new(LocalOrFieldVar("a".to_string())),
                ))]),
            },
            MethodDecl {
                retType: Type::Int,
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
                // TODO: Add assignment of 69
            },
            FieldDecl {
                field_type: Type::Char,
                name: "c".to_string(),
                // TODO: Add assignment of 'x'
            },
            FieldDecl {
                field_type: Type::String,
                name: "s".to_string(),
                // TODO: Add assignment of "Hello World"
            },
            FieldDecl {
                field_type: Type::String,
                name: "stringsCanBeNull".to_string(),
                // TODO: Add assignment of null
            },
            FieldDecl {
                field_type: Type::Bool,
                name: "b".to_string(),
                // TODO: Add assignment of true
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
            retType: Type::Bool,
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
            retType: Type::Bool,
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
            retType: Type::Int, // TODO: This should be Void actually
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
                retType: Type::Int,
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
                retType: Type::Int,
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
                    LocalVarDecl(Type::Int, "x".to_string()), // TODO: Add assignment to 0
                    LocalVarDecl(Type::Int, "y".to_string()), // TODO: Add assignment to 1
                    LocalVarDecl(Type::Int, "i".to_string()), // TODO: Add assignment to 1
                    While(
                        Binary(
                            "<".to_string(),
                            Box::new(LocalOrFieldVar("i".to_string())),
                            Box::new(LocalOrFieldVar("n".to_string())),
                        ),
                        Box::new(Block(vec![
                            LocalVarDecl(Type::Int, "next".to_string()), // TODO: Add assignment to y + x
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
