use super::*;

#[test]
fn test_class() {
    let class = empty_method_class();
    class_test(&tast_to_ast(&class), Some(&class), "EmptyMethod");
}

#[test]
fn test_parser() {
    let class = empty_method_class();
    parser_test(&tast_to_ast(&class), "EmptyMethod");
}

#[test]
fn test_typechecker() {
    let class = empty_method_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = empty_method_class();
    codegen_test(&class, "EmptyMethod");
}

fn empty_method_class() -> Class {
    Class {
        name: "emptyMethod".to_string(),
        fields: vec![],
        methods: vec![MethodDecl {
            ret_type: Type::Void,
            name: "f".to_string(),
            params: vec![],
            body: TypedStmt(Box::new(Block(vec![])), Type::Void),
        }],
    }
}
