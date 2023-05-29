use super::*;

#[test]
fn local_var_decl_class() {
    let class = Class {
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
                            Box::new(LocalOrFieldVar("x".to_string())),
                            Type::Int,
                        ))),
                        Type::Int,
                    ),
                ])),
                Type::Int,
            ),
        }],
    };
    create_test_file(&tast_to_ast(&class), Some(&class), "LocalVarDecl");
}
