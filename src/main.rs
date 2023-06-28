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
    let input_file = args.get(1).unwrap_or_else(|| {
        panic!("No input file provided. Usage: {} <input_file>", args[0]);
    });
    info!("Parsing the file {}", args[1]);
    let mut file = read_to_string(&args[1])?;
    let prg_parsed = parse_programm(&file)?;
    info!("Typechecking the program...");
    let prg_typechecked = TypeChecker::new(prg_parsed)
        .unwrap_or_else(|e| panic!("{}", e))
        .check_and_type_program()
        .unwrap_or_else(|e| panic!("{}", e));
    info!("Generating code using ducc...");
    // Generate code using codegen_ducc
    let mut dir = generate_dir(&prg_typechecked);
    let bytes = dir.as_bytes();
    let alt_outfile = "out.class".to_string();
    let out_file = args.get(2).unwrap_or(&alt_outfile);
    info!("Writing code to {}", out_file);
    let mut file = File::create(out_file)?;
    file.write_all(bytes.as_slice())?;
    Ok(())
}
