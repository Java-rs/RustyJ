use super::*;

#[test]
fn test_class() {
    let class = setter_getter_class();
    class_test(&tast_to_ast(&class), Some(&class), "SetterGetter");
}

#[test]
fn test_parser() {
    let class = setter_getter_class();
    parser_test(&tast_to_ast(&class), "SetterGetter");
}

#[test]
fn test_typechecker() {
    let class = setter_getter_class();
    typechecker_test(&tast_to_ast(&class), &class);
}

#[test]
fn test_codegen() {
    let class = setter_getter_class();
    codegen_test(&class, "SetterGetter");
}

fn setter_getter_class() -> Class {
    Class {
        name: "SetterGetter".to_string(),
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
        methods: vec![
            MethodDecl {
                ret_type: Type::Int,
                name: "getX".to_string(),
                params: vec![],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(InstVar(
                                Box::new(TypedExpr(
                                    Box::new(Expr::This),
                                    Type::Class("SetterGetter".to_string()),
                                )),
                                "x".to_string(),
                            )),
                            Type::Int,
                        ))),
                        Type::Int,
                    )])),
                    Type::Int,
                ),
            },
            MethodDecl {
                ret_type: Type::Void,
                name: "setX".to_string(),
                params: vec![(Type::Int, "x".to_string())],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(StmtExprStmt(TypedStmtExpr(
                            Box::new(Assign(
                                "this.x".to_string(),
                                TypedExpr(Box::new(LocalVar("x".to_string())), Type::Int),
                            )),
                            Type::Int,
                        ))),
                        Type::Int,
                    )])),
                    Type::Void,
                ),
            },
            MethodDecl {
                ret_type: Type::Bool,
                name: "getB".to_string(),
                params: vec![],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(InstVar(
                                Box::new(TypedExpr(
                                    Box::new(Expr::This),
                                    Type::Class("SetterGetter".to_string()),
                                )),
                                "b".to_string(),
                            )),
                            Type::Bool,
                        ))),
                        Type::Bool,
                    )])),
                    Type::Bool,
                ),
            },
            MethodDecl {
                ret_type: Type::Void,
                name: "setB".to_string(),
                params: vec![(Type::Bool, "b".to_string())],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(StmtExprStmt(TypedStmtExpr(
                            Box::new(Assign(
                                "this.b".to_string(),
                                TypedExpr(Box::new(LocalVar("b".to_string())), Type::Bool),
                            )),
                            Type::Bool,
                        ))),
                        Type::Bool,
                    )])),
                    Type::Void,
                ),
            },
            MethodDecl {
                ret_type: Type::Char,
                name: "getC".to_string(),
                params: vec![],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(InstVar(
                                Box::new(TypedExpr(
                                    Box::new(Expr::This),
                                    Type::Class("SetterGetter".to_string()),
                                )),
                                "c".to_string(),
                            )),
                            Type::Char,
                        ))),
                        Type::Char,
                    )])),
                    Type::Char,
                ),
            },
            MethodDecl {
                ret_type: Type::Void,
                name: "setC".to_string(),
                params: vec![(Type::Char, "c".to_string())],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(StmtExprStmt(TypedStmtExpr(
                            Box::new(Assign(
                                "this.c".to_string(),
                                TypedExpr(Box::new(LocalVar("c".to_string())), Type::Char),
                            )),
                            Type::Char,
                        ))),
                        Type::Char,
                    )])),
                    Type::Void,
                ),
            },
            MethodDecl {
                ret_type: Type::String,
                name: "getS".to_string(),
                params: vec![],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(Return(TypedExpr(
                            Box::new(InstVar(
                                Box::new(TypedExpr(
                                    Box::new(Expr::This),
                                    Type::Class("SetterGetter".to_string()),
                                )),
                                "s".to_string(),
                            )),
                            Type::String,
                        ))),
                        Type::String,
                    )])),
                    Type::String,
                ),
            },
            MethodDecl {
                ret_type: Type::Void,
                name: "setS".to_string(),
                params: vec![(Type::String, "s".to_string())],
                body: TypedStmt(
                    Box::new(Block(vec![TypedStmt(
                        Box::new(StmtExprStmt(TypedStmtExpr(
                            Box::new(Assign(
                                "this.s".to_string(),
                                TypedExpr(Box::new(LocalVar("s".to_string())), Type::String),
                            )),
                            Type::String,
                        ))),
                        Type::String,
                    )])),
                    Type::Void,
                ),
            },
        ],
    }
}
