use super::*;

#[test]
fn test_class() {
    let class = negator_class();
    class_test(&tast_to_ast(&class), Some(&class), "Negator");
}

#[test]
fn test_parser() {
    let class = negator_class();
    parser_test(&tast_to_ast(&class), "Negator");
}

#[test]
fn test_typechecker() {
    let class = negator_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = negator_class();
    codegen_test(&class, "Negator");
}

fn negator_class() -> Class {
    Class {
        name: "Negator".to_string(),
        fields: vec![],
        methods: vec![
            MethodDecl {
                ret_type: Type::Int,
                name: "neg1".to_string(),
                params: vec![(Type::Int, "x".to_string())],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(Unary(
                                "-".to_string(),
                                Box::new(TypedExpr(
                                    Box::new(Expr::LocalVar("x".to_string())),
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
            MethodDecl {
                ret_type: Type::Int,
                name: "neg2".to_string(),
                params: vec![(Type::Int, "x".to_string())],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(Unary(
                                "-".to_string(),
                                Box::new(TypedExpr(
                                    Box::new(Unary(
                                        "+".to_string(),
                                        Box::new(TypedExpr(
                                            Box::new(Unary(
                                                "-".to_string(),
                                                Box::new(TypedExpr(
                                                    Box::new(Binary(
                                                        "*".to_string(),
                                                        Box::new(TypedExpr(
                                                            Box::new(Expr::Integer(-1)),
                                                            Type::Int,
                                                        )),
                                                        Box::new(TypedExpr(
                                                            Box::new(LocalVar("x".to_string())),
                                                            Type::Int,
                                                        )),
                                                    )),
                                                    Type::Int,
                                                )),
                                            )),
                                            Type::Int,
                                        )),
                                    )),
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
