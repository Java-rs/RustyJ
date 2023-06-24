use super::*;

#[test]
fn test_class() {
    let class = arithmetic_methods_class();
    class_test(&tast_to_ast(&class), Some(&class), "ArithmeticMethods");
}

#[test]
fn test_parser() {
    let class = arithmetic_methods_class();
    parser_test(&tast_to_ast(&class), "ArithmeticMethods");
}

#[test]
fn test_typechecker() {
    let class = arithmetic_methods_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = arithmetic_methods_class();
    codegen_test(&class, "ArithmeticMethods");
}

fn arithmetic_methods_class() -> Class {
    Class {
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
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(Binary(
                                "+".to_string(),
                                Box::new(TypedExpr(
                                    Box::new(LocalOrFieldVar("x".to_string())),
                                    Type::Int,
                                )),
                                Box::new(TypedExpr(
                                    Box::new(LocalOrFieldVar("a".to_string())),
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
                name: "addY".to_string(),
                params: vec![(Type::Int, "a".to_string())],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(Binary(
                                "+".to_string(),
                                Box::new(TypedExpr(
                                    Box::new(LocalOrFieldVar("y".to_string())),
                                    Type::Int,
                                )),
                                Box::new(TypedExpr(
                                    Box::new(LocalOrFieldVar("a".to_string())),
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
                name: "complexMath".to_string(),
                params: vec![(Type::Int, "a".to_string()), (Type::Int, "b".to_string())],
                body: TypedStmt(
                    Box::new(Block(vec![
                        TypedStmt(
                            Box::new(StmtExprStmt(TypedStmtExpr(
                                Box::new(Assign(
                                    "a".to_string(),
                                    TypedExpr(
                                        Box::new(Binary(
                                            "/".to_string(),
                                            Box::new(TypedExpr(
                                                Box::new(Binary(
                                                    "*".to_string(),
                                                    Box::new(TypedExpr(
                                                        Box::new(LocalOrFieldVar("y".to_string())),
                                                        Type::Int,
                                                    )),
                                                    Box::new(TypedExpr(
                                                        Box::new(Binary(
                                                            "+".to_string(),
                                                            Box::new(TypedExpr(
                                                                Box::new(LocalOrFieldVar(
                                                                    "a".to_string(),
                                                                )),
                                                                Type::Int,
                                                            )),
                                                            Box::new(TypedExpr(
                                                                Box::new(LocalOrFieldVar(
                                                                    "b".to_string(),
                                                                )),
                                                                Type::Int,
                                                            )),
                                                        )),
                                                        Type::Int,
                                                    )),
                                                )),
                                                Type::Int,
                                            )),
                                            Box::new(TypedExpr(
                                                Box::new(LocalOrFieldVar("x".to_string())),
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
                                    "b".to_string(),
                                    TypedExpr(
                                        Box::new(Binary(
                                            "+".to_string(),
                                            Box::new(TypedExpr(
                                                Box::new(LocalOrFieldVar("a".to_string())),
                                                Type::Int,
                                            )),
                                            Box::new(TypedExpr(
                                                Box::new(Unary(
                                                    "-".to_string(),
                                                    Box::new(TypedExpr(
                                                        Box::new(LocalOrFieldVar("b".to_string())),
                                                        Type::Int,
                                                    )),
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
                                    "a".to_string(),
                                    TypedExpr(
                                        Box::new(Binary(
                                            "+".to_string(),
                                            Box::new(TypedExpr(
                                                Box::new(LocalOrFieldVar("x".to_string())),
                                                Type::Int,
                                            )),
                                            Box::new(TypedExpr(
                                                Box::new(Binary(
                                                    "*".to_string(),
                                                    Box::new(TypedExpr(
                                                        Box::new(LocalOrFieldVar("b".to_string())),
                                                        Type::Int,
                                                    )),
                                                    Box::new(TypedExpr(
                                                        Box::new(LocalOrFieldVar("a".to_string())),
                                                        Type::Int,
                                                    )),
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
                            Box::new(Return(TypedExpr(
                                Box::new(Binary(
                                    "*".to_string(),
                                    Box::new(TypedExpr(
                                        Box::new(LocalOrFieldVar("x".to_string())),
                                        Type::Int,
                                    )),
                                    Box::new(TypedExpr(
                                        Box::new(LocalOrFieldVar("a".to_string())),
                                        Type::Int,
                                    )),
                                )),
                                Type::Int,
                            ))),
                            Type::Int,
                        ),
                    ])),
                    Type::Int,
                ),
            },
        ],
    }
}
