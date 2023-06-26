use super::*;

#[test]
fn test_class() {
    let class = full_test_class();
    class_test(&tast_to_ast(&class), Some(&class), "FullTest");
}

#[test]
fn test_parser() {
    let class = full_test_class();
    parser_test(&tast_to_ast(&class), "FullTest");
}

#[test]
fn test_typechecker() {
    let class = full_test_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = full_test_class();
    codegen_test(&class, "FullTest");
}

fn full_test_class() -> Class {
    Class {
        name: "FullTest".to_string(),
        fields: vec![],
        methods: vec![],
    }
}
