use super::*;

#[test]
fn if_class() {
    let class = Class {
        name: "If".to_string(),
        fields: vec![],
        methods: vec![MethodDecl {
            name: "f".to_string(),
            params: vec![(Type::Char, "c".to_string())],
            ret_type: Type::Bool,
            body: TypedStmt(
                Box::new(Block(vec![
                    TypedStmt(
                        Box::new(If(
                            TypedExpr(
                                Box::new(Binary(
                                    "==".to_string(),
                                    Box::new(TypedExpr(
                                        Box::new(LocalOrFieldVar("c".to_string())),
                                        Type::Char,
                                    )),
                                    Box::new(Char('a')),
                                )),
                                Type::Bool,
                            ),
                            Box::new(TypedStmt(
                                Box::new(Return(TypedExpr(Box::new(Bool(true)), Type::Bool))),
                                Type::Bool,
                            )),
                            None,
                        )),
                        Type::Bool,
                    ),
                    TypedStmt(
                        Box::new(Return(TypedExpr(Box::new(Bool(false)), Type::Bool))),
                        Type::Bool,
                    ),
                ])),
                Type::Bool,
            ),
        }],
    };
    single_class_test(&tast_to_ast(&class), Some(&class), "If");
}
