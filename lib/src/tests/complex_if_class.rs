use super::*;

#[test]
fn test_class() {
    let class = complex_if_class();
    class_test(&tast_to_ast(&class), Some(&class), "ComplexIf");
}

#[test]
fn test_parser() {
    let class = complex_if_class();
    parser_test(&tast_to_ast(&class), "ComplexIf");
}

#[test]
fn test_typechecker() {
    let class = complex_if_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = complex_if_class();
    codegen_test(&class, "ComplexIf");
}

fn complex_if_class() -> Class {
    Class {
        name: "ComplexIf".to_string(),
        fields: vec![],
        methods: vec![MethodDecl {
            ret_type: Type::Bool,
            name: "f".to_string(),
            params: vec![(Type::Char, "c".to_string())],
            body: Block(vec![If(
                Binary(
                    "==".to_string(),
                    Box::new(TypedExpr(
                        Box::new(LocalOrFieldVar("c".to_string())),
                        Type::Char,
                    )),
                    Box::new(TypedExpr(Box::new(Expr::Char('a')), Type::Char)),
                ),
                Box::new(TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(Box::new(Expr::Bool(true)), Type::Bool))),
                        Type::Bool,
                    )])),
                    Type::Bool,
                )),
                Some(Box::new(TypedStmt(
                    Box::new(If(
                        TypedExpr(
                            Box::new(Binary(
                                "==".to_string(),
                                Box::new(TypedExpr(
                                    Box::new(LocalOrFieldVar("c".to_string())),
                                    Type::Char,
                                )),
                                Box::new(TypedExpr(Box::new(Expr::Char('b')), Type::Char)),
                            )),
                            Type::Bool,
                        ),
                        Box::new(TypedStmt(
                            Box::new(Block(vec![TypedStmt(
                                Box::new(Return(TypedExpr(
                                    Box::new(Expr::Bool(false)),
                                    Type::Bool,
                                ))),
                                Type::Bool,
                            )])),
                            Type::Bool,
                        )),
                        Some(Box::new(TypedStmt(
                            Box::new(If(
                                TypedExpr(
                                    Box::new(Binary(
                                        "==".to_string(),
                                        Box::new(TypedExpr(
                                            Box::new(LocalOrFieldVar("c".to_string())),
                                            Type::Char,
                                        )),
                                        Box::new(TypedExpr(Box::new(Expr::Char('c')), Type::Char)),
                                    )),
                                    Type::Bool,
                                ),
                                Box::new(TypedStmt(
                                    Box::new(Return(TypedExpr(
                                        Box::new(Expr::Bool(true)),
                                        Type::Bool,
                                    ))),
                                    Type::Bool,
                                )),
                                Some(Box::new(TypedStmt(
                                    Box::new(Block(vec![TypedStmt(
                                        Box::new(If(
                                            TypedExpr(
                                                Box::new(Binary(
                                                    "||".to_string(),
                                                    Box::new(TypedExpr(
                                                        Box::new(Binary(
                                                            "==".to_string(),
                                                            Box::new(TypedExpr(
                                                                Box::new(LocalOrFieldVar(
                                                                    "c".to_string(),
                                                                )),
                                                                Type::Char,
                                                            )),
                                                            Box::new(TypedExpr(
                                                                Box::new(Expr::Char('d')),
                                                                Type::Char,
                                                            )),
                                                        )),
                                                        Type::Bool,
                                                    )),
                                                    Box::new(TypedExpr(
                                                        Box::new(Binary(
                                                            "==".to_string(),
                                                            Box::new(TypedExpr(
                                                                Box::new(LocalOrFieldVar(
                                                                    "c".to_string(),
                                                                )),
                                                                Type::Char,
                                                            )),
                                                            Box::new(TypedExpr(
                                                                Box::new(Expr::Char('e')),
                                                                Type::Char,
                                                            )),
                                                        )),
                                                        Type::Bool,
                                                    )),
                                                )),
                                                Type::Bool,
                                            ),
                                            Box::new(TypedStmt(
                                                Box::new(Return(TypedExpr(
                                                    Box::new(Expr::Bool(false)),
                                                    Type::Bool,
                                                ))),
                                                Type::Bool,
                                            )),
                                            Some(Box::new(TypedStmt(
                                                Box::new(If(
                                                    TypedExpr(
                                                        Box::new(Binary(
                                                            "&&".to_string(),
                                                            Box::new(TypedExpr(
                                                                Box::new(Binary(
                                                                    "==".to_string(),
                                                                    Box::new(TypedExpr(
                                                                        Box::new(LocalOrFieldVar(
                                                                            "c".to_string(),
                                                                        )),
                                                                        Type::Char,
                                                                    )),
                                                                    Box::new(TypedExpr(
                                                                        Box::new(Expr::Char('f')),
                                                                        Type::Char,
                                                                    )),
                                                                )),
                                                                Type::Bool,
                                                            )),
                                                            Box::new(TypedExpr(
                                                                Box::new(Binary(
                                                                    "==".to_string(),
                                                                    Box::new(TypedExpr(
                                                                        Box::new(LocalOrFieldVar(
                                                                            "c".to_string(),
                                                                        )),
                                                                        Type::Char,
                                                                    )),
                                                                    Box::new(TypedExpr(
                                                                        Box::new(Expr::Char('g')),
                                                                        Type::Char,
                                                                    )),
                                                                )),
                                                                Type::Bool,
                                                            )),
                                                        )),
                                                        Type::Bool,
                                                    ),
                                                    Box::new(TypedStmt(
                                                        Box::new(Return(TypedExpr(
                                                            Box::new(Expr::Bool(true)),
                                                            Type::Bool,
                                                        ))),
                                                        Type::Bool,
                                                    )),
                                                    Some(Box::new(TypedStmt(
                                                        Box::new(Block(vec![TypedStmt(
                                                            Box::new(Return(TypedExpr(
                                                                Box::new(Expr::Bool(false)),
                                                                Type::Bool,
                                                            ))),
                                                            Type::Bool,
                                                        )])),
                                                        Type::Bool,
                                                    ))),
                                                )),
                                                Type::Bool,
                                            ))),
                                        )),
                                        Type::Bool,
                                    )])),
                                    Type::Bool,
                                ))),
                            )),
                            Type::Bool,
                        ))),
                    )),
                    Type::Bool,
                ))),
            )]),
        }],
    }
}
