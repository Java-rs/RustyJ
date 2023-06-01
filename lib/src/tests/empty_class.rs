use super::*;

#[test]
fn empty_class() {
    let class = Class {
        name: "Empty".to_string(),
        fields: vec![],
        methods: vec![],
    };
    create_test_file(&tast_to_ast(&class), Some(&class), "Empty");
}
