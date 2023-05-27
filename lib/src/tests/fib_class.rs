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
    create_test_file(&tast_to_ast(&class), Some(&class), "Fib");
}
