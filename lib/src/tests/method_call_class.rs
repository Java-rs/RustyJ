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
                ret_type: Type::Int,
                name: "a".to_string(),
                params: vec![],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(Box::new(Expr::Integer(2)), Type::Int))),
                        Type::Int,
                    )])),
                    Type::Int,
                ),
            },
            MethodDecl {
                ret_type: Type::Int,
                name: "b".to_string(),
                params: vec![],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(Box::new(Expr::Integer(5)), Type::Int))),
                        Type::Int,
                    )])),
                    Type::Int,
                ),
            },
            MethodDecl {
                ret_type: Type::Int,
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
                                            "a".to_string(),
                                            vec![],
                                        )),
                                        Type::Int,
                                    )))),
                                    Type::Int,
                                )),
                                Box::new(TypedExpr(
                                    Box::new(StmtExprExpr(Box::new(TypedStmtExpr(
                                        Box::new(MethodCall(
                                            TypedExpr(
                                                Box::new(This),
                                                Type::Class("MethodCall".to_string()),
                                            ),
                                            "b".to_string(),
                                            vec![],
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
                ),
            },
        ],
    }
}
