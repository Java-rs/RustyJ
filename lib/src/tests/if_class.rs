use super::*;

#[test]
fn test_class() {
    let class = if_class();
    class_test(&tast_to_ast(&class), Some(&class), "If");
}

#[test]
fn test_parser() {
    let class = if_class();
    parser_test(&tast_to_ast(&class), "If");
}

#[test]
fn test_typechecker() {
    let class = if_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = if_class();
    codegen_test(&class, "If");
}

fn if_class() -> Class {
    Class {
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
                                        Box::new(LocalVar("c".to_string())),
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
    }
}
