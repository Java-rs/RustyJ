use super::*;

#[test]
fn int_fields_class() {
    let class = Class {
        name: "IntFields".to_string(),
        fields: vec![
            FieldDecl {
                field_type: Type::Int,
                name: "x".to_string(),
                val: None,
            },
            FieldDecl {
                field_type: Type::Int,
                name: "y".to_string(),
                val: None,
            },
        ],
        methods: vec![],
    };
    create_test_file(&tast_to_ast(&class), Some(&class), "IntFields");
}
