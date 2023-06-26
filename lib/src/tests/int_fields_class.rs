use super::*;

#[test]
fn test_class() {
    let class = int_fields_class();
    class_test(&tast_to_ast(&class), Some(&class), "IntFields");
}

#[test]
fn test_parser() {
    let class = int_fields_class();
    parser_test(&tast_to_ast(&class), "IntFields");
}

#[test]
fn test_typechecker() {
    let class = int_fields_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = int_fields_class();
    codegen_test(&class, "IntFields");
}

fn int_fields_class() -> Class {
    Class {
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
            FieldDecl {
                field_type: Type::Int,
                name: "z".to_string(),
                val: None,
            },
            FieldDecl {
                field_type: Type::Int,
                name: "another_int".to_string(),
                val: None,
            },
        ],
        methods: vec![],
    }
}
