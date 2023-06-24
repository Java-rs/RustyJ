use super::*;

#[test]
fn test_class() {
    let class = fields_class();
    class_test(&tast_to_ast(&class), Some(&class), "Fields");
}

#[test]
fn test_parser() {
    let class = fields_class();
    parser_test(&tast_to_ast(&class), "Fields");
}

#[test]
fn test_typechecker() {
    let class = fields_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = fields_class();
    codegen_test(&class, "Fields");
}

fn fields_class() -> Class {
    Class {
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
    }
}
