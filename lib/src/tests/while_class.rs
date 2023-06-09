use super::*;

#[test]
fn test_class() {
    let class = while_class();
    class_test(&tast_to_ast(&class), Some(&class), "While");
}

#[test]
fn test_parser() {
    let class = while_class();
    parser_test(&tast_to_ast(&class), "While");
}

#[test]
fn test_typechecker() {
    let class = while_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = while_class();
    codegen_test(&class, "While");
}

fn while_class() -> Class {
    Class {
        name: "While".to_string(),
        fields: vec![FieldDecl {
            field_type: Type::Int,
            name: "n".to_string(),
            val: Some(Expr::Integer(2)),
        }],
        methods: vec![MethodDecl {
            name: "f".to_string(),
            params: vec![(Type::Int, "x".to_string())],
            ret_type: Type::Int,
            body: TypedStmt(
                Box::new(Block(vec![
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
                                TypedExpr(Box::new(Integer(0)), Type::Int),
                            )),
                            Type::Int,
                        ))),
                        Type::Int,
                    ),
                    TypedStmt(
                        Box::new(LocalVarDecl(Type::Int, "a".to_string())),
                        Type::Int,
                    ),
                    TypedStmt(
                        Box::new(StmtExprStmt(TypedStmtExpr(
                            Box::new(Assign(
                                Expr::TypedExpr(
                                    Box::new(Expr::LocalVar("a".to_string())),
                                    Type::Int,
                                ),
                                TypedExpr(Box::new(FieldVar("n".to_string())), Type::Int),
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
                                        Box::new(LocalVar("x".to_string())),
                                        Type::Int,
                                    )),
                                )),
                                Type::Bool,
                            ),
                            Box::new(TypedStmt(
                                Box::new(Block(vec![
                                    TypedStmt(
                                        Box::new(StmtExprStmt(TypedStmtExpr(
                                            Box::new(Assign(
                                                Expr::TypedExpr(
                                                    Box::new(Expr::LocalVar("a".to_string())),
                                                    Type::Int,
                                                ),
                                                TypedExpr(
                                                    Box::new(Binary(
                                                        "+".to_string(),
                                                        Box::new(TypedExpr(
                                                            Box::new(LocalVar("a".to_string())),
                                                            Type::Int,
                                                        )),
                                                        Box::new(TypedExpr(
                                                            Box::new(LocalVar("a".to_string())),
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
                                                            Box::new(Integer(1)),
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
                            Box::new(LocalVar("a".to_string())),
                            Type::Int,
                        ))),
                        Type::Int,
                    ),
                ])),
                Type::Int,
            ),
        }],
    }
}
