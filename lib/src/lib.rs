mod parser;
#[cfg(test)]
mod tests;
mod typechecker;
mod types;

use tracing::info;

pub fn hi() {
    info!("Hello from our library!");
}
// We can re-export our stuff here(using `pub use`) and put the parsing, type-checking and codegen in seperate mods, so we don't get merge conflicts
