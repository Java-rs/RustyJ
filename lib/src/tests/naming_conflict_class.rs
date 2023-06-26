use super::*;

#[test]
fn test_class() {
    let class = naming_conflict();
    class_test(&tast_to_ast(&class), Some(&class), "NamingConflict");
}

#[test]
fn test_parser() {
    let class = naming_conflict();
    parser_test(&tast_to_ast(&class), "NamingConflict");
}

#[test]
fn test_typechecker() {
    let class = naming_conflict();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = naming_conflict();
    codegen_test(&class, "NamingConflict");
}

fn naming_conflict() -> Class {
    Class {
        name: "NamingConflict".to_string(),
        fields: vec![FieldDecl {
            field_type: Type::Int,
            name: "x".to_string(),
            val: Some(Expr::Integer(69)),
        }],
        methods: vec![MethodDecl {
            ret_type: Type::Int,
            name: "f".to_string(),
            params: vec![(Type::Int, "x".to_string())],
            body: TypedStmt(
                Box::new(Block(vec![TypedStmt(
                    Box::new(Return(TypedExpr(
                        Box::new(Binary(
                            "+".to_string(),
                            Box::new(TypedExpr(
                                Box::new(InstVar(
                                    Box::new(TypedExpr(
                                        Box::new(Expr::This),
                                        Type::Class("NamingConflict".to_string()),
                                    )),
                                    "x".to_string(),
                                )),
                                Type::Int,
                            )),
                            Box::new(TypedExpr(Box::new(LocalVar("x".to_string())), Type::Int)),
                        )),
                        Type::Int,
                    ))),
                    Type::Int,
                )])),
                Type::Int,
            ),
        }],
    }
}
