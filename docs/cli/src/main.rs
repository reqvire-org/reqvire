pub mod cli;
pub mod config;
mod mcp;
mod serve;

use crate::cli::apply_workspace;
use crate::cli::handle_command;
use crate::cli::Args;
use crate::config::get_excluded_filename_patterns_glob_set;
use log::error;
use reqvire::error::ReqvireError;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error");
    }

    let args = Args::parse_args();

    // Check if any command uses JSON output to suppress logs
    let uses_json = match &args.command {
        Some(cli::Commands::ChangeImpact { json, .. }) => *json,
        Some(cli::Commands::Format { json, .. }) => *json,
        Some(cli::Commands::Search { json, .. }) => *json,
        Some(cli::Commands::Traces { json, .. }) => *json,
        Some(cli::Commands::Coverage { json, .. }) => *json,
        _ => false,
    };

    if uses_json {
        std::env::set_var("RUST_LOG", "error");
    }

    env_logger::init();

    if let Err(err) = apply_workspace(args.workspace.as_ref()) {
        error!("{}", err);
        std::process::exit(1);
    }

    // Run `handle_command` and get exit code
    let exit_code = handle_command(args, &get_excluded_filename_patterns_glob_set())
        .await
        .unwrap_or_else(|e| {
            match e {
                ReqvireError::ValidationError(errors) => {
                    let mut messages: Vec<String> =
                        errors.iter().map(|err| err.to_string()).collect();
                    messages.sort();
                    eprintln!("Validation failed with {} error(s):", messages.len());
                    for (idx, msg) in messages.iter().enumerate() {
                        eprintln!("{}. {}", idx + 1, msg);
                    }
                }
                other => error!("{}", other),
            }
            1 // Return exit code 1 in case of an error
        });

    std::process::exit(exit_code);
}
