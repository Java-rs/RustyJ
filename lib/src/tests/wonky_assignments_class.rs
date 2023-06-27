use super::*;

#[test]
fn test_class() {
    let class = wonky_assignments_class();
    class_test(&tast_to_ast(&class), Some(&class), "WonkyAssignments");
}

#[test]
fn test_parser() {
    let class = wonky_assignments_class();
    parser_test(&tast_to_ast(&class), "WonkyAssignments");
}

#[test]
fn test_typechecker() {
    let class = wonky_assignments_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = wonky_assignments_class();
    codegen_test(&class, "WonkyAssignments");
}

fn wonky_assignments_class() -> Class {
    Class {
        name: "WonkyAssignments".to_string(),
        fields: vec![
            FieldDecl {
                field_type: Type::Int,
                name: "x".to_string(),
                val: None,
            },
            FieldDecl {
                field_type: Type::Int,
                name: "y".to_string(),
                val: Some(Expr::Integer(3)),
            },
            FieldDecl {
                field_type: Type::Int,
                name: "z".to_string(),
                val: None,
            },
            FieldDecl {
                field_type: Type::Bool,
                name: "a".to_string(),
                val: Some(Expr::Bool(false)),
            },
            FieldDecl {
                field_type: Type::Bool,
                name: "b".to_string(),
                val: Some(Expr::Bool(true)),
            },
            FieldDecl {
                field_type: Type::Bool,
                name: "c".to_string(),
                val: None,
            },
        ],
        methods: vec![MethodDecl {
            ret_type: Type::Int,
            name: "f".to_string(),
            params: vec![(Type::Int, "newX".to_string())],
            body: TypedStmt(
                Box::new(Block(vec![
                    TypedStmt(
                        Box::new(StmtExprStmt(TypedStmtExpr(
                            Box::new(Assign(
                                "c".to_string(),
                                TypedExpr(
                                    Box::new(Binary(
                                        ">".to_string(),
                                        Box::new(TypedExpr(
                                            Box::new(StmtExprExpr(Box::new(TypedStmtExpr(
                                                Box::new(Assign(
                                                    "x".to_string(),
                                                    TypedExpr(
                                                        Box::new(LocalVar("newX".to_string())),
                                                        Type::Int,
                                                    ),
                                                )),
                                                Type::Int,
                                            )))),
                                            Type::Int,
                                        )),
                                        Box::new(TypedExpr(
                                            Box::new(FieldVar("y".to_string())),
                                            Type::Int,
                                        )),
                                    )),
                                    Type::Bool,
                                ),
                            )),
                            Type::Bool,
                        ))),
                        Type::Bool,
                    ),
                    TypedStmt(
                        Box::new(LocalVarDecl(Type::Int, "i".to_string())),
                        Type::Int,
                    ),
                    TypedStmt(
                        Box::new(LocalVarDecl(Type::Int, "j".to_string())),
                        Type::Int,
                    ),
                    TypedStmt(
                        Box::new(StmtExprStmt(TypedStmtExpr(
                            Box::new(Assign(
                                "j".to_string(),
                                TypedExpr(
                                    Box::new(StmtExprExpr(Box::new(TypedStmtExpr(
                                        Box::new(Assign(
                                            "z".to_string(),
                                            TypedExpr(
                                                Box::new(FieldVar("x".to_string())),
                                                Type::Int,
                                            ),
                                        )),
                                        Type::Int,
                                    )))),
                                    Type::Int,
                                ),
                            )),
                            Type::Int,
                        ))),
                        Type::Int,
                    ),
                    TypedStmt(
                        Box::new(LocalVarDecl(Type::Int, "k".to_string())),
                        Type::Int,
                    ),
                    TypedStmt(
                        Box::new(StmtExprStmt(TypedStmtExpr(
                            Box::new(Assign(
                                "k".to_string(),
                                TypedExpr(Box::new(Expr::Integer(-1)), Type::Int),
                            )),
                            Type::Int,
                        ))),
                        Type::Int,
                    ),
                    TypedStmt(
                        Box::new(StmtExprStmt(TypedStmtExpr(
                            Box::new(Assign(
                                "a".to_string(),
                                TypedExpr(
                                    Box::new(Unary(
                                        "!".to_string(),
                                        Box::new(TypedExpr(
                                            Box::new(StmtExprExpr(Box::new(TypedStmtExpr(
                                                Box::new(Assign(
                                                    "b".to_string(),
                                                    TypedExpr(
                                                        Box::new(Binary(
                                                            "==".to_string(),
                                                            Box::new(TypedExpr(
                                                                Box::new(FieldVar("c".to_string())),
                                                                Type::Bool,
                                                            )),
                                                            Box::new(TypedExpr(
                                                                Box::new(Expr::Bool(true)),
                                                                Type::Bool,
                                                            )),
                                                        )),
                                                        Type::Bool,
                                                    ),
                                                )),
                                                Type::Bool,
                                            )))),
                                            Type::Bool,
                                        )),
                                    )),
                                    Type::Bool,
                                ),
                            )),
                            Type::Bool,
                        ))),
                        Type::Bool,
                    ),
                    TypedStmt(
                        Box::new(If(
                            TypedExpr(Box::new(FieldVar("a".to_string())), Type::Bool),
                            Box::new(TypedStmt(
                                Box::new(Return(TypedExpr(
                                    Box::new(FieldVar("z".to_string())),
                                    Type::Int,
                                ))),
                                Type::Int,
                            )),
                            Some(Box::new(TypedStmt(
                                Box::new(Return(TypedExpr(
                                    Box::new(LocalVar("k".to_string())),
                                    Type::Int,
                                ))),
                                Type::Int,
                            ))),
                        )),
                        Type::Int,
                    ),
                ])),
                Type::Int,
            ),
        }],
    }
}
