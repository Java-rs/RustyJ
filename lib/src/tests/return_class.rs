use super::*;

#[test]
fn test_class() {
    let class = return_class();
    class_test(&tast_to_ast(&class), Some(&class), "Return");
}

#[test]
fn test_parser() {
    let class = return_class();
    parser_test(&tast_to_ast(&class), "Return");
}

#[test]
fn test_typechecker() {
    let class = return_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = return_class();
    codegen_test(&class, "Return");
}

fn return_class() -> Class {
    Class {
        name: "Return".to_string(),
        fields: vec![],
        methods: vec![
            MethodDecl {
                ret_type: Type::Char,
                name: "id".to_string(),
                params: vec![(Type::Char, "x".to_string())],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(Expr::LocalVar("x".to_string())),
                            Type::Char,
                        ))),
                        Type::Char,
                    )])),
                    Type::Char,
                ),
            },
            MethodDecl {
                ret_type: Type::Bool,
                name: "id".to_string(),
                params: vec![(Type::Bool, "b".to_string())],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(Expr::LocalVar("b".to_string())),
                            Type::Bool,
                        ))),
                        Type::Bool,
                    )])),
                    Type::Bool,
                ),
            },
        ],
    }
}
