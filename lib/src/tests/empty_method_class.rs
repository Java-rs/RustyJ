use super::*;

#[test]
fn empty_method_class() {
    let class = Class {
        name: "emptyMethod".to_string(),
        fields: vec![],
        methods: vec![MethodDecl {
            ret_type: Type::Void,
            name: "f".to_string(),
            params: vec![],
            body: TypedStmt(Box::new(Block(vec![])), Type::Void),
        }],
    };
    single_class_test(&tast_to_ast(&class), Some(&class), "emptyMethod");
}
