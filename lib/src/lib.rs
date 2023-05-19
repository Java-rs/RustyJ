use tracing::info;
pub mod codegen;
pub fn hi() {
    info!("Hello from our library!");
}
// We can re-export our stuff here(using `pub use`) and put the parsing, type-checking and codegen in seperate mods, so we don't get merge conflicts
// Also common definitions between our parser, type-checker and codegen can be put here
