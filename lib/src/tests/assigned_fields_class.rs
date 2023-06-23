use super::*;

#[test]
fn assigned_fields_class() {
    let class = Class {
        name: "AssignedFields".to_string(),
        fields: vec![
            FieldDecl {
                field_type: Type::Int,
                name: "x".to_string(),
                val: Some("69".to_string()),
            },
            FieldDecl {
                field_type: Type::Char,
                name: "c".to_string(),
                val: Some("x".to_string()),
            },
            FieldDecl {
                field_type: Type::String,
                name: "s".to_string(),
                val: Some("Hello World".to_string()),
            },
            FieldDecl {
                field_type: Type::String,
                name: "stringsCanBeNull".to_string(),
                val: Some("null".to_string()),
            },
            FieldDecl {
                field_type: Type::Bool,
                name: "b".to_string(),
                val: Some("true".to_string()),
            },
        ],
        methods: vec![],
    };
    single_class_test(&tast_to_ast(&class), Some(&class), "AssignedFields");
}
