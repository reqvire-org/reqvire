use std::path::PathBuf;
use log::error;

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
pub mod init;
pub mod html;

use crate::model::ModelManager;
use crate::cli::handle_command;
use crate::cli::Args;
use crate::config::Config;

/*
#[path = "tests/general_tests.rs"]
#[cfg(test)]
mod general_tests;
#[path = "tests/validation_tests.rs"]
#[cfg(test)]
mod validation_tests;
#[path = "tests/config_tests.rs"]
#[cfg(test)]
mod config_tests;
#[path = "tests/linting_tests.rs"]
#[cfg(test)]
mod linting_tests;
*/

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let args = Args::parse_args();
    let config = Config::load_from_args(&args);

    let mut model_manager = ModelManager::new(config.clone());

  
    // Run `handle_command` and get exit code
    let exit_code = handle_command(
        args, 
        &mut model_manager, 
        &config.get_specification_folder(),
        &config.get_external_folders(), 
        &config.get_output_folder(), 
        &config.get_excluded_filename_patterns_glob_set()
     )
        .unwrap_or_else(|e| {
            error!("Execution failed: {}", e);
            1 // Return exit code 1 in case of an error
        });

    std::process::exit(exit_code); 
}

