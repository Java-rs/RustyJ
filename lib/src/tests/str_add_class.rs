use super::*;

#[test]
fn test_class() {
    let class = str_add_class();
    class_test(&tast_to_ast(&class), Some(&class), "StrAdd");
}

#[test]
fn test_parser() {
    let class = str_add_class();
    parser_test(&tast_to_ast(&class), "StrAdd");
}

#[test]
fn test_typechecker() {
    let class = str_add_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = str_add_class();
    codegen_test(&class, "StrAdd");
}

fn str_add_class() -> Class {
    Class {
        name: "StrAdd".to_string(),
        fields: vec![FieldDecl {
            field_type: Type::String,
            name: "hello".to_string(),
            val: Some(Expr::String("Hi".to_string())),
        }],
        methods: vec![MethodDecl {
            ret_type: Type::String,
            name: "helloWorld".to_string(),
            params: vec![],
            body: TypedStmt(
                Box::new(Return(TypedExpr(
                    Box::new(Binary(
                        "+".to_string(),
                        Box::new(TypedExpr(
                            Box::new(FieldVar("hello".to_string())),
                            Type::String,
                        )),
                        Box::new(TypedExpr(
                            Box::new(Expr::String(" World".to_string())),
                            Type::String,
                        )),
                    )),
                    Type::String,
                ))),
                Type::String,
            ),
        }],
    }
}
