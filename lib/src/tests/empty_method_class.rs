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
            body: Block(vec![]),
        }],
    };
    create_test_file(&tast_to_ast(&class), Some(&class), "emptyMethod");
}
