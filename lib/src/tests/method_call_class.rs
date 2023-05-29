use super::*;

#[test]
fn method_call_class() {
    let class = Class {
        name: "MethodCall".to_string(),
        fields: vec![],
        methods: vec![
            MethodDecl {
                ret_type: Type::String,
                name: "world".to_string(),
                params: vec![],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(Expr::String("World".to_string())),
                            Type::String,
                        ))),
                        Type::String,
                    )])),
                    Type::String,
                ),
            },
            MethodDecl {
                ret_type: Type::String,
                name: "hello".to_string(),
                params: vec![],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(Expr::String("Hello".to_string())),
                            Type::String,
                        ))),
                        Type::String,
                    )])),
                    Type::String,
                ),
            },
            MethodDecl {
                ret_type: Type::String,
                name: "f".to_string(),
                params: vec![],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(Binary(
                                "+".to_string(),
                                Box::new(TypedExpr(
                                    Box::new(StmtExprExpr(Box::new(TypedStmtExpr(
                                        Box::new(MethodCall(
                                            TypedExpr(Box::new(This), Type::String),
                                            "hello".to_string(),
                                            vec![],
                                        )),
                                        Type::String,
                                    )))),
                                    Type::String,
                                )),
                                Box::new(TypedExpr(
                                    Box::new(Binary(
                                        "+".to_string(),
                                        Box::new(TypedExpr(
                                            Box::new(Expr::String(" ".to_string())),
                                            Type::String,
                                        )),
                                        Box::new(TypedExpr(
                                            Box::new(StmtExprExpr(Box::new(TypedStmtExpr(
                                                Box::new(MethodCall(
                                                    TypedExpr(Box::new(This), Type::String),
                                                    "world".to_string(),
                                                    vec![],
                                                )),
                                                Type::String,
                                            )))),
                                            Type::String,
                                        )),
                                    )),
                                    Type::String,
                                )),
                            )),
                            Type::String,
                        ))),
                        Type::String,
                    )])),
                    Type::String,
                ),
            },
        ],
    };
    create_test_file(&tast_to_ast(&class), Some(&class), "MethodCall");
}
