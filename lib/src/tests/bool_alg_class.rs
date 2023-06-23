use super::*;

#[test]
fn test_class() {
    let class = bool_alg_class();
    class_test(&tast_to_ast(&class), Some(&class), "BoolAlg");
}

#[test]
fn test_parser() {
    let class = bool_alg_class();
    parser_test(&tast_to_ast(&class), "BoolAlg");
}

#[test]
fn test_typechecker() {
    let class = bool_alg_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = bool_alg_class();
    codegen_test(&class, "BoolAlg");
}

fn bool_alg_class() -> Class {
    Class {
        name: "BoolAlg".to_string(),
        fields: vec![],
        methods: vec![MethodDecl {
            ret_type: Type::Bool,
            name: "f".to_string(),
            params: vec![
                (Type::Bool, "a".to_string()),
                (Type::Bool, "b".to_string()),
                (Type::Bool, "c".to_string()),
            ],
            body: TypedStmt(
                Box::new(Block(vec![TypedStmt(
                    Box::new(Return(TypedExpr(
                        Box::new(Binary(
                            "||".to_string(),
                            Box::new(TypedExpr(
                                Box::new(Binary(
                                    "&&".to_string(),
                                    Box::new(TypedExpr(
                                        Box::new(LocalOrFieldVar("a".to_string())),
                                        Type::Bool,
                                    )),
                                    Box::new(TypedExpr(
                                        Box::new(LocalOrFieldVar("b".to_string())),
                                        Type::Bool,
                                    )),
                                )),
                                Type::Bool,
                            )),
                            Box::new(TypedExpr(
                                Box::new(LocalOrFieldVar("c".to_string())),
                                Type::Bool,
                            )),
                        )),
                        Type::Bool,
                    ))),
                    Type::Bool,
                )])),
                Type::Bool,
            ),
        }],
    }
}
