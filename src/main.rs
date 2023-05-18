mod parser;

use parser::Example;
use tracing::info;
use crate::parser::parse_Example;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    info!("Hello RustyJ!");


    // let unparesed_file =
    let test:  Example = parse_Example("Sander,Stella\nTori,Gönnheimer").expect("u suck");
    println!("{}", serde_json::to_string(&test).unwrap());


    lib::hi();
    Ok(())
}