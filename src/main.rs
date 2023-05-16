mod types;

use crate::types::{Class, check_class};
use tracing::info;
use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;
use serde::{Serialize, Deserialize};
use serde_json;


fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("tests/If-AST.txt")?;
    let mut ast_string = String::new();

    file.read_to_string(&mut ast_string)?;
    let ast: Class = ast_string.parse()?;
    println!("{:#?}", ast);
    //check_class(&ast);

    //let typed_ast_string = serde_json::to_string_pretty(&ast)?;

    //let mut file = File::create("typed_if-ast.txt")?;
    //file.write_all(typed_ast_string.as_bytes())?;

    Ok(())
}
