pub mod cli;
pub mod config;
mod live_store;
mod mcp;
mod mcp_github;
mod mcp_http;
mod mcp_process;
mod mcp_session;
mod mcp_worker;
mod mcp_worktrees;
mod serve;

use crate::cli::apply_workspace;
use crate::cli::handle_command;
use crate::cli::Args;
use crate::config::get_excluded_filename_patterns_glob_set;
use log::error;
use reqvire::error::ReqvireError;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();
    if std::env::args().nth(1).as_deref() == Some("__mcp-worktree-worker") {
        if let Err(error) = mcp_worker::run() {
            eprintln!("{error}");
            std::process::exit(1);
        }
        return;
    }
    let args = Args::parse_args();

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
                ReqvireError::ValidationDiagnostics { related_errors, .. } => {
                    let mut messages: Vec<String> =
                        related_errors.iter().map(|err| err.to_string()).collect();
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
