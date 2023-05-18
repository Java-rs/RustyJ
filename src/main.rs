mod parser;

use parser::Example;
use parser::*;
use tracing::info;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    info!("Hello RustyJ!");

    // let unparesed_file =
    let test: Example = parse_Example(
        "Val,Richter\nBene,Brandmeier\nMario,Hinkel\nMaxi,Floto\nPflipper,Wolf\nTori,Gonnheimer\nSander,Stella\n",
    )
    .expect("u suck");
    println!("{}", serde_json::to_string(&test).unwrap());

    lib::hi();
    Ok(())
}
