use super::*;

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
                    TypedExpr(
                        Box::new(Binary(
                            "<".to_string(),
                            Box::new(TypedExpr(
                                Box::new(LocalOrFieldVar("n".to_string())),
                                Type::Int,
                            )),
                            Box::new(TypedExpr(Box::new(Expr::Integer(2)), Type::Int)),
                        )),
                        Type::Bool,
                    ),
                    Box::new(TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(LocalOrFieldVar("n".to_string())),
                            Type::Int,
                        ))),
                        Type::Int,
                    )),
                    Some(Box::new(TypedStmt(
                        Box::new(Block(vec![TypedStmt(
                            Box::new(Return(TypedExpr(
                                Box::new(Binary(
                                    "+".to_string(),
                                    Box::new(TypedExpr(
                                        Box::new(StmtExprExpr(Box::new(TypedStmtExpr(
                                            Box::new(MethodCall(
                                                TypedExpr(
                                                    Box::new(Expr::This),
                                                    Type::Class("Fib".to_string()),
                                                ),
                                                "rec".to_string(),
                                                vec![TypedExpr(
                                                    Box::new(Binary(
                                                        "-".to_string(),
                                                        Box::new(TypedExpr(
                                                            Box::new(LocalOrFieldVar(
                                                                "n".to_string(),
                                                            )),
                                                            Type::Int,
                                                        )),
                                                        Box::new(TypedExpr(
                                                            Box::new(Expr::Integer(1)),
                                                            Type::Int,
                                                        )),
                                                    )),
                                                    Type::Int,
                                                )],
                                            )),
                                            Type::Int,
                                        )))),
                                        Type::Int,
                                    )),
                                    Box::new(TypedExpr(
                                        Box::new(StmtExprExpr(Box::new(TypedStmtExpr(
                                            Box::new(MethodCall(
                                                TypedExpr(
                                                    Box::new(Expr::This),
                                                    Type::Class("Fib".to_string()),
                                                ),
                                                "rec".to_string(),
                                                vec![TypedExpr(
                                                    Box::new(Binary(
                                                        "-".to_string(),
                                                        Box::new(TypedExpr(
                                                            Box::new(LocalOrFieldVar(
                                                                "n".to_string(),
                                                            )),
                                                            Type::Int,
                                                        )),
                                                        Box::new(TypedExpr(
                                                            Box::new(Expr::Integer(2)),
                                                            Type::Int,
                                                        )),
                                                    )),
                                                    Type::Int,
                                                )],
                                            )),
                                            Type::Int,
                                        )))),
                                        Type::Int,
                                    )),
                                )),
                                Type::Int,
                            ))),
                            Type::Int,
                        )])),
                        Type::Int,
                    ))),
                )]),
            },
            MethodDecl {
                ret_type: Type::Int,
                name: "iter".to_string(),
                params: vec![(Type::Int, "n".to_string())],
                body: TypedStmt(
                    Box::new(Block(vec![
                        TypedStmt(
                            Box::new(If(
                                TypedExpr(
                                    Box::new(Binary(
                                        "<".to_string(),
                                        Box::new(TypedExpr(
                                            Box::new(LocalOrFieldVar("n".to_string())),
                                            Type::Int,
                                        )),
                                        Box::new(TypedExpr(Box::new(Expr::Integer(2)), Type::Int)),
                                    )),
                                    Type::Bool,
                                ),
                                Box::new(TypedStmt(
                                    Box::new(Return(TypedExpr(
                                        Box::new(LocalOrFieldVar("n".to_string())),
                                        Type::Int,
                                    ))),
                                    Type::Int,
                                )),
                                None,
                            )),
                            Type::Int,
                        ),
                        TypedStmt(
                            Box::new(LocalVarDecl(Type::Int, "x".to_string())),
                            Type::Void,
                        ), // TODO: Should the type of a declaration be `void`?
                        TypedStmt(
                            Box::new(StmtExprStmt(TypedStmtExpr(
                                Box::new(Assign(
                                    "x".to_string(),
                                    TypedExpr(Box::new(Expr::Integer(0)), Type::Int),
                                )),
                                Type::Int,
                            ))),
                            Type::Int,
                        ),
                        TypedStmt(
                            Box::new(LocalVarDecl(Type::Int, "y".to_string())),
                            Type::Void,
                        ), // TODO: Should the type of a declaration be `void`?
                        TypedStmt(
                            Box::new(StmtExprStmt(TypedStmtExpr(
                                Box::new(Assign(
                                    "y".to_string(),
                                    TypedExpr(Box::new(Expr::Integer(1)), Type::Int),
                                )),
                                Type::Int,
                            ))),
                            Type::Int,
                        ),
                        TypedStmt(
                            Box::new(LocalVarDecl(Type::Int, "i".to_string())),
                            Type::Void,
                        ), // TODO: Should the type of a declaration be `void`?
                        TypedStmt(
                            Box::new(StmtExprStmt(TypedStmtExpr(
                                Box::new(Assign(
                                    "i".to_string(),
                                    TypedExpr(Box::new(Expr::Integer(1)), Type::Int),
                                )),
                                Type::Int,
                            ))),
                            Type::Int,
                        ),
                        TypedStmt(
                            Box::new(While(
                                TypedExpr(
                                    Box::new(Binary(
                                        "<".to_string(),
                                        Box::new(TypedExpr(
                                            Box::new(LocalOrFieldVar("i".to_string())),
                                            Type::Int,
                                        )),
                                        Box::new(TypedExpr(
                                            Box::new(LocalOrFieldVar("i".to_string())),
                                            Type::Int,
                                        )),
                                    )),
                                    Type::Bool,
                                ),
                                Box::new(TypedStmt(
                                    Box::new(Block(vec![
                                        TypedStmt(
                                            Box::new(LocalVarDecl(Type::Int, "next".to_string())),
                                            Type::Void,
                                        ),
                                        TypedStmt(
                                            Box::new(StmtExprStmt(TypedStmtExpr(
                                                Box::new(Assign(
                                                    "next".to_string(),
                                                    TypedExpr(
                                                        Box::new(Binary(
                                                            "+".to_string(),
                                                            Box::new(TypedExpr(
                                                                Box::new(LocalOrFieldVar(
                                                                    "y".to_string(),
                                                                )),
                                                                Type::Int,
                                                            )),
                                                            Box::new(TypedExpr(
                                                                Box::new(LocalOrFieldVar(
                                                                    "x".to_string(),
                                                                )),
                                                                Type::Int,
                                                            )),
                                                        )),
                                                        Type::Int,
                                                    ),
                                                )),
                                                Type::Int,
                                            ))),
                                            Type::Int,
                                        ),
                                        TypedStmt(
                                            Box::new(StmtExprStmt(TypedStmtExpr(
                                                Box::new(Assign(
                                                    "x".to_string(),
                                                    TypedExpr(
                                                        Box::new(LocalOrFieldVar("y".to_string())),
                                                        Type::Int,
                                                    ),
                                                )),
                                                Type::Int,
                                            ))),
                                            Type::Int,
                                        ),
                                        TypedStmt(
                                            Box::new(StmtExprStmt(TypedStmtExpr(
                                                Box::new(Assign(
                                                    "y".to_string(),
                                                    TypedExpr(
                                                        Box::new(LocalOrFieldVar(
                                                            "next".to_string(),
                                                        )),
                                                        Type::Int,
                                                    ),
                                                )),
                                                Type::Int,
                                            ))),
                                            Type::Int,
                                        ),
                                        TypedStmt(
                                            Box::new(StmtExprStmt(TypedStmtExpr(
                                                Box::new(Assign(
                                                    "i".to_string(),
                                                    TypedExpr(
                                                        Box::new(Binary(
                                                            "+".to_string(),
                                                            Box::new(TypedExpr(
                                                                Box::new(LocalOrFieldVar(
                                                                    "i".to_string(),
                                                                )),
                                                                Type::Int,
                                                            )),
                                                            Box::new(TypedExpr(
                                                                Box::new(Expr::Integer(1)),
                                                                Type::Int,
                                                            )),
                                                        )),
                                                        Type::Int,
                                                    ),
                                                )),
                                                Type::Int,
                                            ))),
                                            Type::Int,
                                        ),
                                    ])),
                                    Type::Int,
                                )),
                            )),
                            Type::Int,
                        ),
                        TypedStmt(
                            Box::new(Return(TypedExpr(
                                Box::new(LocalOrFieldVar("y".to_string())),
                                Type::Int,
                            ))),
                            Type::Int,
                        ),
                    ])),
                    Type::Int,
                ),
            },
        ],
    };
    single_class_test(&tast_to_ast(&class), Some(&class), "Fib");
}
