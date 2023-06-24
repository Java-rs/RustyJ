use lib::typechecker::typechecker;
use lib::types::{Class, Expr, FieldDecl, MethodDecl, Prg, Stmt, StmtExpr, Type};
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use tracing::info;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    info!("Hello RustyJ!");

    lib::hi();
    Ok(())
}
