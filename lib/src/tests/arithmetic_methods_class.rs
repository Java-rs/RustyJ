use super::*;

#[test]
fn arithmetic_methods_class() {
    let class = Class {
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
                body: Block(vec![Return(Binary(
                    "+".to_string(),
                    Box::new(LocalOrFieldVar("x".to_string())),
                    Box::new(LocalOrFieldVar("a".to_string())),
                ))]),
            },
            MethodDecl {
                ret_type: Type::Int,
                name: "addY".to_string(),
                params: vec![(Type::Int, "a".to_string())],
                body: Block(vec![Return(Binary(
                    "+".to_string(),
                    Box::new(LocalOrFieldVar("y".to_string())),
                    Box::new(LocalOrFieldVar("a".to_string())),
                ))]),
            },
            MethodDecl {
                ret_type: Type::Int,
                name: "complexMath".to_string(),
                params: vec![(Type::Int, "a".to_string()), (Type::Int, "b".to_string())],
                body: Block(vec![
                    StmtExprStmt(Assign(
                        "a".to_string(),
                        Binary(
                            "*".to_string(),
                            Box::new(LocalOrFieldVar("y".to_string())),
                            Box::new(Binary(
                                "/".to_string(),
                                Box::new(Binary(
                                    "+".to_string(),
                                    Box::new(LocalOrFieldVar("a".to_string())),
                                    Box::new(LocalOrFieldVar("b".to_string())),
                                )),
                                Box::new(LocalOrFieldVar("x".to_string())),
                            )),
                        ),
                    )),
                    StmtExprStmt(Assign(
                        "b".to_string(),
                        Binary(
                            "+".to_string(),
                            Box::new(LocalOrFieldVar("a".to_string())),
                            Box::new(Unary(
                                "-".to_string(),
                                Box::new(LocalOrFieldVar("b".to_string())),
                            )),
                        ),
                    )),
                    StmtExprStmt(Assign(
                        "a".to_string(),
                        Binary(
                            "+".to_string(),
                            Box::new(LocalOrFieldVar("x".to_string())),
                            Box::new(Binary(
                                "*".to_string(),
                                Box::new(LocalOrFieldVar("b".to_string())),
                                Box::new(LocalOrFieldVar("a".to_string())),
                            )),
                        ),
                    )),
                    Return(Binary(
                        "*".to_string(),
                        Box::new(LocalOrFieldVar("x".to_string())),
                        Box::new(LocalOrFieldVar("a".to_string())),
                    )),
                ]),
            },
        ],
    };
    create_test_file(&tast_to_ast(&class), Some(&class), "ArithmeticMethods");
}
