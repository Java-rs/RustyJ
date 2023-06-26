use lib::codegen::generate_dir;
use lib::typechecker::typechecker;
use lib::types::{Class, Expr, FieldDecl, MethodDecl, Prg, Stmt, StmtExpr, Type};
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use tracing::info;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    info!("Hello RustyJ!");

    lib::hi();
    let mut file = File::open("lib/testcases/Fib-AST.json")?;
    let mut ast_string = String::new();

    file.read_to_string(&mut ast_string)?;
    let ast_value: Value = serde_json::from_str(&ast_string)?;
    //println!("{:#?}", ast_value);
    let class2: Class = serde_json::from_value(ast_value.clone())?;
    println!("{:#?}", class2);
    let class: Class = Class {
        name: "test".to_string(),
        fields: vec![
            FieldDecl {
                field_type: Type::Char,
                name: "c".to_string(),
                val: None,
            },
            FieldDecl {
                field_type: Type::Int,
                name: "d".to_string(),
                val: Some(Expr::Integer(42)),
            },
        ],
        methods: vec![MethodDecl {
            name: "f".to_string(),
            params: vec![(Type::Char, "c".to_string())],
            ret_type: Type::Bool,
            body: Stmt::Block(vec![
                Stmt::LocalVarDecl(Type::Int, "x".to_string()),
                Stmt::StmtExprStmt(StmtExpr::Assign(
                    "c".to_string(),
                    Expr::Binary(
                        "+".to_string(),
                        Box::new(Expr::LocalOrFieldVar("x".to_string())),
                        Box::new(Expr::Integer(1)),
                    ),
                )),
                Stmt::LocalVarDecl(Type::Int, "d".to_string()),
                Stmt::If(
                    Expr::Binary(
                        "==".to_string(),
                        Box::new(Expr::LocalOrFieldVar("c".to_string())),
                        Box::new(Expr::Char('a')),
                    ),
                    Box::new(Stmt::Return(Expr::Bool(true))),
                    None,
                ),
                Stmt::Return(Expr::Bool(false)),
            ]),
        }],
    };

    let program: Prg = vec![class2.clone()];
    let mut typechecker = typechecker::TypeChecker::new(program).unwrap();
    typechecker.check_and_type_program().expect("ERROR");
    serde_json::to_writer_pretty(
        &mut File::create("typed_if-test.json")?,
        &typechecker.typed_classes.get(&class2.name.clone()),
    )?;

    let dir = generate_dir(&typechecker.typed_classes.values().cloned().collect());
    println!("{:#?}", dir);

    //let mut file = File::create("typed_if-ast.txt")?;
    //file.write_all(typed_ast_string.as_bytes())?;
    Ok(())
}
