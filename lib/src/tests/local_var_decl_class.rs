use super::*;

#[test]
fn test_class() {
    let class = local_var_decl_class();
    class_test(&tast_to_ast(&class), Some(&class), "LocalVarDecl");
}

#[test]
fn test_parser() {
    let class = local_var_decl_class();
    parser_test(&tast_to_ast(&class), "LocalVarDecl");
}

#[test]
fn test_typechecker() {
    let class = local_var_decl_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = local_var_decl_class();
    codegen_test(&class, "LocalVarDecl");
}

fn local_var_decl_class() -> Class {
    Class {
        name: "LocalVarDecl".to_string(),
        fields: vec![],
        methods: vec![MethodDecl {
            ret_type: Type::Int,
            name: "f".to_string(),
            params: vec![],
            body: TypedStmt(
                Box::new(Block(vec![
                    TypedStmt(
                        Box::new(LocalVarDecl(Type::Int, "x".to_string())),
                        Type::Void,
                    ),
                    TypedStmt(
                        Box::new(StmtExprStmt(TypedStmtExpr(
                            Box::new(Assign(
                                "x".to_string(),
                                TypedExpr(Box::new(Integer(69)), Type::Int),
                            )),
                            Type::Int,
                        ))),
                        Type::Int,
                    ),
                    TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(LocalVar("x".to_string())),
                            Type::Int,
                        ))),
                        Type::Int,
                    ),
                ])),
                Type::Int,
            ),
        }],
    }
}
