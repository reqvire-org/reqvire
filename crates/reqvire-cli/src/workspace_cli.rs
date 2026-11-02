//! CLI translation only; workspace state and operations belong to its crate.
use crate::cli::WorkspaceCommands;
use reqvire::error::ReqvireError;
use reqvire_workspace::{Operation, Workspace};
pub fn handle(command: &WorkspaceCommands, json: bool) -> Result<i32, ReqvireError> {
    let workspace = Workspace::open(&std::env::current_dir()?)?;
    if matches!(command, WorkspaceCommands::Status) {
        let status = workspace.status()?;
        if json {
            println!("{}", serde_json::to_string_pretty(&status)?);
        } else {
            println!("Workspace: {}", workspace.root.display());
            if let Some(view) = &workspace.composition {
                println!(
                    "Manifest: {}\nRepositories directory: {}",
                    view.manifest.display(),
                    view.repositories_dir.display()
                );
            } else {
                println!("Single repository");
            }
            for project in status["projects"].as_array().expect("project inventory") {
                println!(
                    "{}: pin {} | HEAD {} | ref {}{}",
                    project["path"].as_str().unwrap_or_default(),
                    project["commit"].as_str().unwrap_or("unresolved"),
                    project["head"].as_str().unwrap_or("missing"),
                    project["ref"].as_str().unwrap_or("none"),
                    project["diagnostic"]
                        .as_str()
                        .map(|s| format!(" | {s}"))
                        .unwrap_or_default()
                );
            }
        }
        return Ok(0);
    }
    let (operation, dry_run) = match command {
        WorkspaceCommands::Add {
            url,
            path,
            reference,
        } => (
            Operation::Add {
                url: url.clone(),
                path: path.clone(),
                reference: reference.clone(),
            },
            false,
        ),
        WorkspaceCommands::Remove { path } => (Operation::Remove { path: path.clone() }, false),
        WorkspaceCommands::Sync { latest } => (Operation::Sync { latest: *latest }, false),
        WorkspaceCommands::Freeze { dry_run } => (Operation::Freeze, *dry_run),
        WorkspaceCommands::Migrate {
            source, dry_run, ..
        } => (
            Operation::Migrate {
                source: source.clone(),
            },
            *dry_run,
        ),
        WorkspaceCommands::Status => unreachable!(),
    };
    let report = match workspace.prepare(operation, dry_run, &[]) {
        Ok(prepared) => prepared.accept(),
        Err(error) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &serde_json::json!({"error":error.to_string(),"outcomes":error.outcomes(),"outcome_unknown":error.outcome_unknown()})
                    )?
                );
            } else {
                eprintln!("{error}");
                for outcome in error.outcomes() {
                    eprintln!("{}: {}", outcome.path.display(), outcome.outcome);
                }
            }
            return Ok(1);
        }
    };
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!(
            "Workspace {}: {}{}",
            report.operation,
            if report.changed {
                "changed"
            } else {
                "unchanged"
            },
            if dry_run { " (preview)" } else { "" }
        );
        if report.changed {
            println!(
                "--- {} (before)\n{}+++ {} (after)\n{}",
                report.manifest.display(),
                report.before,
                report.manifest.display(),
                report.after
            );
        }
        for suggestion in &report.ignore_suggestions {
            println!("Suggested root .gitignore entry: {suggestion}");
        }
        for outcome in &report.outcomes {
            println!(
                "{}: {} {}",
                outcome.path.display(),
                outcome.outcome,
                outcome.commit.as_deref().unwrap_or_default()
            );
        }
    }
    Ok(0)
}
