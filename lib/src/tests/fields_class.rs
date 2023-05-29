use super::*;

#[test]
fn fields_class() {
    let class = Class {
        name: "Fields".to_string(),
        fields: vec![
            FieldDecl {
                field_type: Type::Int,
                name: "x".to_string(),
                val: None,
            },
            FieldDecl {
                field_type: Type::Bool,
                name: "b".to_string(),
                val: None,
            },
            FieldDecl {
                field_type: Type::Char,
                name: "c".to_string(),
                val: None,
            },
            FieldDecl {
                field_type: Type::String,
                name: "s".to_string(),
                val: None,
            },
        ],
        methods: vec![],
    };
    create_test_file(&tast_to_ast(&class), Some(&class), "Fields");
}
