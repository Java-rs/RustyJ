use super::*;

#[test]
fn test_class() {
    let class = method_call_class();
    class_test(&tast_to_ast(&class), Some(&class), "MethodCall");
}

#[test]
fn test_parser() {
    let class = method_call_class();
    parser_test(&tast_to_ast(&class), "MethodCall");
}

#[test]
fn test_typechecker() {
    let class = method_call_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = method_call_class();
    codegen_test(&class, "MethodCall");
}

fn method_call_class() -> Class {
    Class {
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
                                            TypedExpr(
                                                Box::new(This),
                                                Type::Class("MethodCall".to_string()),
                                            ),
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
                                                    TypedExpr(
                                                        Box::new(This),
                                                        Type::Class("MethodCall".to_string()),
                                                    ),
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
    }
}
