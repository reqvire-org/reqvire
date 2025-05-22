pub mod cli;
pub mod config;

use log::error;
use crate::cli::handle_command;
use crate::cli::Args;
use crate::config::Config;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error");
    }

    let args = Args::parse_args();
    
    if args.json {
        std::env::set_var("RUST_LOG", "error");
    }
    
    env_logger::init();
    
    let config = Config::load_from_args(&args);

    // Run `handle_command` and get exit code
    let exit_code = handle_command(
        args,
        &config.get_output_folder(), 
        &config.get_excluded_filename_patterns_glob_set(),
        &config.style.diagram_direction,
        config.style.diagrams_with_blobs,
        &config.get_user_requirements_root_folder()
     )
        .unwrap_or_else(|e| {
            error!("{}", e);
            1 // Return exit code 1 in case of an error
        });

    std::process::exit(exit_code); 
}
