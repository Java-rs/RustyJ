use lib::codegen::generate_dir;
use lib::parser::parse_programm;
use lib::typechecker::typechecker;
use lib::typechecker::typechecker::TypeChecker;
use lib::types::{Class, Expr, FieldDecl, MethodDecl, Prg, Stmt, StmtExpr, Type};
use serde_json::Value;
use std::fs::{read_to_string, File};
use std::io::{Read, Write};
use tracing::info;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    let args = std::env::args().collect::<Vec<_>>();
    info!("Parsing the file {}", args[1]);
    let mut file = read_to_string(&args[1])?;
    let prg_parsed = parse_programm(&file)?;
    info!("Typechecking the program...");
    let prg_typechecked = TypeChecker::new(prg_parsed)?.check_and_type_program()?;
    info!("Generating code using ducc...");
    // Generate code using codegen_ducc
    let mut dir = generate_dir(&Prg(prg_typechecked));
    let bytes = dir.as_bytes();
    let out_file = args.get(2).unwrap_or(&"out.class".to_string());
    info!("Writing code to {}", out_file);
    let mut file = File::create(out_file)?;
    file.write_all(bytes.as_slice())?;
    Ok(())
}
