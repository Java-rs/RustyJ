use lib::codegen::generate_dir;
use lib::parser::parse_programm;
use lib::typechecker::typechecker::TypeChecker;
use std::fs::{read_to_string, File};
use std::io::Write;
use tracing::info;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    let args = std::env::args().collect::<Vec<_>>();
    let input_file = args.get(1).unwrap_or_else(|| {
        panic!("No input file provided. Usage: {} <input_file>", args[0]);
    });
    info!("Parsing the file {}", input_file);
    let file = read_to_string(input_file)?;
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
    // The output file, which is the input file with the extension replaced by .class and the folder cut off
    let alt_outfile = input_file
        .split('/')
        .last()
        .unwrap()
        .replace(".java", ".class");
    let out_file = args.get(2).unwrap_or(&alt_outfile);
    info!("Writing code to {}", out_file);
    let mut file = File::create(out_file)?;
    file.write_all(bytes.as_slice())?;
    Ok(())
}
