use super::*;

#[test]
fn bool_alg_class() {
    let class = Class {
        name: "BoolAlg".to_string(),
        fields: vec![],
        methods: vec![MethodDecl {
            ret_type: Type::Bool,
            name: "f".to_string(),
            params: vec![
                (Type::Bool, "a".to_string()),
                (Type::Bool, "b".to_string()),
                (Type::Bool, "c".to_string()),
            ],
            body: Block(vec![Return(Binary(
                "||".to_string(),
                Box::new(Binary(
                    "&&".to_string(),
                    Box::new(Binary(
                        "&&".to_string(),
                        Box::new(LocalOrFieldVar("a".to_string())),
                        Box::new(LocalOrFieldVar("b".to_string())),
                    )),
                    Box::new(LocalOrFieldVar("c".to_string())),
                )),
                Box::new(LocalOrFieldVar("c".to_string())),
            ))]),
        }],
    };
    create_test_file(&tast_to_ast(&class), Some(&class), "BoolAlg");
}
