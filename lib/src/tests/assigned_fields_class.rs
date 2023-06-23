use super::*;

#[test]
fn test_class() {
    let class = assigned_fields_class();
    class_test(&tast_to_ast(&class), Some(&class), "AssignedFields");
}

#[test]
fn test_parser() {
    let class = assigned_fields_class();
    parser_test(&tast_to_ast(&class), "AssignedFields");
}

#[test]
fn test_typechecker() {
    let class = assigned_fields_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = assigned_fields_class();
    codegen_test(&class, "AssignedFields");
}

fn assigned_fields_class() -> Class {
    Class {
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
    }
}
