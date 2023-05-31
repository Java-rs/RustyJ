use super::*;

#[test]
fn empty_class() {
    let class = Class {
        name: "Empty".to_string(),
        fields: vec![],
        methods: vec![],
    };
    single_class_test(&tast_to_ast(&class), Some(&class), "Empty");
}
