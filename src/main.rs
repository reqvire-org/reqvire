pub mod model;
pub mod cli;
pub mod config;
pub mod element_registry;
pub mod element;
pub mod relation;
pub mod error;
pub mod utils;
pub mod parser;
pub mod html_export;
pub mod linting;
pub mod html;
pub mod filesystem;
pub mod diagrams;
pub mod index_generator;
pub mod reports;
pub mod git_commands;
pub mod change_impact;


use log::error;
use crate::cli::handle_command;
use crate::cli::Args;
use crate::config::Config;


fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    let args = Args::parse_args();
    
    if args.json{
        std::env::set_var("RUST_LOG", "critical");
    }
    
    env_logger::init();
    
    let config = Config::load_from_args(&args);

    // Run `handle_command` and get exit code
    let exit_code = handle_command(
        args, 
        &config.get_specification_folder(),
        &config.get_external_folders(), 
        &config.get_output_folder(), 
        &config.get_excluded_filename_patterns_glob_set(),
        &config.style.diagram_direction
     )
        .unwrap_or_else(|e| {
            error!("Execution failed: {}", e);
            1 // Return exit code 1 in case of an error
        });

    std::process::exit(exit_code); 
}

