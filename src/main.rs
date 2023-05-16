mod typechecker;
mod types;

use std::error::Error;
use tracing::info;

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    info!("Hello RustyJ!");
    lib::hi();
    Ok(())
}
