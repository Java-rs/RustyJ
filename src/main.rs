use lib::typechecker::typechecker;
use lib::types::{Class, Expr, FieldDecl, MethodDecl, Prg, Stmt, Type};
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use tracing::info;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    info!("Hello RustyJ!");
    lib::hi();
    //let mut file = File::open("tests/If-AST.json")?;
    //let mut ast_string = String::new();

    //file.read_to_string(&mut ast_string)?;
    //let ast_value: Value = serde_json::from_str(&ast_string)?;
    //println!("{:#?}", ast_value);
    //let ast: Class = serde_json::from_value(ast_value.clone())?;
    //println!("{:#?}", ast);
    let class: Class = Class {
        name: "test".to_string(),
        fields: vec![],
        methods: vec![MethodDecl {
            name: "f".to_string(),
            params: vec![(Type::Char, "c".to_string())],
            retType: Type::Bool,
            body: Stmt::Block(vec![
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

    let program: Prg = vec![class.clone()];
    let mut typechecker = typechecker::TypeChecker::new(program);
    typechecker
        .unwrap()
        .check_program()
        .expect("TODO: panic message");

    // Create a new json file
    let mut file = File::create("typed_if-test-local.json")?;

    serde_json::to_writer_pretty(&mut file, &class)?;

    //let typed_ast_string = serde_json::to_string_pretty(&ast)?;

    //let mut file = File::create("typed_if-ast.txt")?;
    //file.write_all(typed_ast_string.as_bytes())?;
    Ok(())
}
