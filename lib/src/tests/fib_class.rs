use super::*;

#[test]
fn test_class() {
    let class = fib_class();
    class_test(&tast_to_ast(&class), Some(&class), "Fib");
}

#[test]
fn test_parser() {
    let class = fib_class();
    parser_test(&tast_to_ast(&class), "Fib");
}

#[test]
fn test_typechecker() {
    let class = fib_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = fib_class();
    codegen_test(&class, "Fib");
}

fn fib_class() -> Class {
    Class {
        name: "Fib".to_string(),
        fields: vec![],
        methods: vec![
            MethodDecl {
                ret_type: Type::Int,
                name: "rec".to_string(),
                params: vec![(Type::Int, "n".to_string())],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(If(
                            TypedExpr(
                                Box::new(Binary(
                                    "<".to_string(),
                                    Box::new(TypedExpr(
                                        Box::new(LocalVar("n".to_string())),
                                        Type::Int,
                                    )),
                                    Box::new(TypedExpr(Box::new(Expr::Integer(2)), Type::Int)),
                                )),
                                Type::Bool,
                            ),
                            Box::new(TypedStmt(
                                Box::new(Block(vec![TypedStmt(
                                    Box::new(Return(TypedExpr(
                                        Box::new(LocalVar("n".to_string())),
                                        Type::Int,
                                    ))),
                                    Type::Int,
                                )])),
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
                                                                    Box::new(LocalVar(
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
                                                                    Box::new(LocalVar(
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
                        )),
                        Type::Int,
                    )])),
                    Type::Int,
                ),
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
                                            Box::new(LocalVar("n".to_string())),
                                            Type::Int,
                                        )),
                                        Box::new(TypedExpr(Box::new(Expr::Integer(2)), Type::Int)),
                                    )),
                                    Type::Bool,
                                ),
                                Box::new(TypedStmt(
                                    Box::new(Block(vec![TypedStmt(
                                        Box::new(Return(TypedExpr(
                                            Box::new(LocalVar("n".to_string())),
                                            Type::Int,
                                        ))),
                                        Type::Int,
                                    )])),
                                    Type::Int,
                                )),
                                None,
                            )),
                            Type::Int,
                        ),
                        TypedStmt(
                            Box::new(LocalVarDecl(Type::Int, "x".to_string())),
                            Type::Int,
                        ),
                        TypedStmt(
                            Box::new(StmtExprStmt(TypedStmtExpr(
                                Box::new(Assign(
                                    Expr::TypedExpr(
                                        Box::new(Expr::LocalVar("x".to_string())),
                                        Type::Int,
                                    ),
                                    TypedExpr(Box::new(Expr::Integer(0)), Type::Int),
                                )),
                                Type::Int,
                            ))),
                            Type::Int,
                        ),
                        TypedStmt(
                            Box::new(LocalVarDecl(Type::Int, "y".to_string())),
                            Type::Int,
                        ),
                        TypedStmt(
                            Box::new(StmtExprStmt(TypedStmtExpr(
                                Box::new(Assign(
                                    Expr::TypedExpr(
                                        Box::new(Expr::LocalVar("y".to_string())),
                                        Type::Int,
                                    ),
                                    TypedExpr(Box::new(Expr::Integer(1)), Type::Int),
                                )),
                                Type::Int,
                            ))),
                            Type::Int,
                        ),
                        TypedStmt(
                            Box::new(LocalVarDecl(Type::Int, "i".to_string())),
                            Type::Int,
                        ),
                        TypedStmt(
                            Box::new(StmtExprStmt(TypedStmtExpr(
                                Box::new(Assign(
                                    Expr::TypedExpr(
                                        Box::new(Expr::LocalVar("i".to_string())),
                                        Type::Int,
                                    ),
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
                                            Box::new(LocalVar("i".to_string())),
                                            Type::Int,
                                        )),
                                        Box::new(TypedExpr(
                                            Box::new(LocalVar("n".to_string())),
                                            Type::Int,
                                        )),
                                    )),
                                    Type::Bool,
                                ),
                                Box::new(TypedStmt(
                                    Box::new(Block(vec![
                                        TypedStmt(
                                            Box::new(LocalVarDecl(Type::Int, "next".to_string())),
                                            Type::Int,
                                        ),
                                        TypedStmt(
                                            Box::new(StmtExprStmt(TypedStmtExpr(
                                                Box::new(Assign(
                                                    Expr::TypedExpr(
                                                        Box::new(Expr::LocalVar(
                                                            "next".to_string(),
                                                        )),
                                                        Type::Int,
                                                    ),
                                                    TypedExpr(
                                                        Box::new(Binary(
                                                            "+".to_string(),
                                                            Box::new(TypedExpr(
                                                                Box::new(LocalVar("y".to_string())),
                                                                Type::Int,
                                                            )),
                                                            Box::new(TypedExpr(
                                                                Box::new(LocalVar("x".to_string())),
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
                                                    Expr::TypedExpr(
                                                        Box::new(Expr::LocalVar("x".to_string())),
                                                        Type::Int,
                                                    ),
                                                    TypedExpr(
                                                        Box::new(LocalVar("y".to_string())),
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
                                                    Expr::TypedExpr(
                                                        Box::new(Expr::LocalVar("y".to_string())),
                                                        Type::Int,
                                                    ),
                                                    TypedExpr(
                                                        Box::new(LocalVar("next".to_string())),
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
                                                    Expr::TypedExpr(
                                                        Box::new(Expr::LocalVar("i".to_string())),
                                                        Type::Int,
                                                    ),
                                                    TypedExpr(
                                                        Box::new(Binary(
                                                            "+".to_string(),
                                                            Box::new(TypedExpr(
                                                                Box::new(LocalVar("i".to_string())),
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
                                    Type::Void,
                                )),
                            )),
                            Type::Void,
                        ),
                        TypedStmt(
                            Box::new(Return(TypedExpr(
                                Box::new(LocalVar("y".to_string())),
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
