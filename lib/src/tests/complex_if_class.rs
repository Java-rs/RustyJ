use super::*;

#[test]
fn complex_if_class() {
    let class = Class {
        name: "ComplexIf".to_string(),
        fields: vec![],
        methods: vec![MethodDecl {
            ret_type: Type::Bool,
            name: "f".to_string(),
            params: vec![(Type::Char, "c".to_string())],
            body: Block(vec![If(
                Binary(
                    "==".to_string(),
                    Box::new(LocalOrFieldVar("c".to_string())),
                    Box::new(Expr::Char('a')),
                ),
                Box::new(Block(vec![Return(Expr::Bool(true))])),
                Some(Box::new(If(
                    Binary(
                        "==".to_string(),
                        Box::new(LocalOrFieldVar("c".to_string())),
                        Box::new(Expr::Char('b')),
                    ),
                    Box::new(Block(vec![Return(Expr::Bool(false))])),
                    Some(Box::new(If(
                        Binary(
                            "==".to_string(),
                            Box::new(LocalOrFieldVar("c".to_string())),
                            Box::new(Expr::Char('c')),
                        ),
                        Box::new(Return(Expr::Bool(true))),
                        Some(Box::new(Block(vec![If(
                            Binary(
                                "||".to_string(),
                                Box::new(Binary(
                                    "==".to_string(),
                                    Box::new(LocalOrFieldVar("c".to_string())),
                                    Box::new(Expr::Char('d')),
                                )),
                                Box::new(Binary(
                                    "==".to_string(),
                                    Box::new(LocalOrFieldVar("c".to_string())),
                                    Box::new(Expr::Char('e')),
                                )),
                            ),
                            Box::new(Return(Expr::Bool(false))),
                            Some(Box::new(If(
                                Binary(
                                    "||".to_string(),
                                    Box::new(Binary(
                                        "==".to_string(),
                                        Box::new(LocalOrFieldVar("c".to_string())),
                                        Box::new(Expr::Char('d')),
                                    )),
                                    Box::new(Binary(
                                        "==".to_string(),
                                        Box::new(LocalOrFieldVar("c".to_string())),
                                        Box::new(Expr::Char('e')),
                                    )),
                                ),
                                Box::new(Return(Expr::Bool(true))),
                                Some(Box::new(Block(vec![Return(Expr::Bool(false))]))),
                            ))),
                        )]))),
                    ))),
                ))),
            )]),
        }],
    };
    create_test_file(&tast_to_ast(&class), Some(&class), "ComplexIf");
}
