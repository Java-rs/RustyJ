use super::*;

#[test]
fn test_class() {
    let class = empty_class();
    class_test(&tast_to_ast(&class), Some(&class), "Empty");
}

#[test]
fn test_parser() {
    let class = empty_class();
    parser_test(&tast_to_ast(&class), "Empty");
}

#[test]
fn test_typechecker() {
    let class = empty_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = empty_class();
    codegen_test(&class, "Empty");
}

fn empty_class() -> Class {
    Class {
        name: "Empty".to_string(),
        fields: vec![],
        methods: vec![],
    }
}
