//! CLI entry point for React Intl Extract
//!
//! This is a thin wrapper that parses CLI arguments and delegates
//! to the core logic in the `core` module.

pub mod core;
pub mod extractor;
mod visitors;

use anyhow::Result;
use clap::Parser;

use crate::core::{run_cli, Args};

fn main() -> Result<()> {
    let args = Args::parse();
    let exit_code = run_cli(args)?;
    std::process::exit(exit_code);
}
