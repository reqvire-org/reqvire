use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod config;
mod element;
mod error;
mod html;
mod markdown;
mod model;
mod relation;
mod utils;
mod validation;
#[cfg(test)]
mod tests;

use model::ModelManager;

/// ReqFlow is an agile Model-Based Systems Engineering (MBSE) framework
/// designed to integrate seamlessly with modern Git workflows.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Path to the input folder containing Markdown files
    #[clap(index = 1)]
    input_folder: PathBuf,

    /// Path to the output folder
    #[clap(index = 2)]
    output_folder: PathBuf,

    /// Convert Markdown to HTML with embedded styles
    #[clap(long)]
    html: bool,

    /// Enable verbose output
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    // Initialize logger with a more verbose default level
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let args = Args::parse();

    if args.verbose {
        println!("Processing files from {:?} to {:?}", args.input_folder, args.output_folder);
    }

    let mut model_manager = ModelManager::new();
    model_manager.process_files(&args.input_folder, &args.output_folder, args.html)?;

    println!("Files processed and saved to {:?}", args.output_folder);
    Ok(())
}