use std::process::Command;
use std::error::Error;
use serde::Serialize;
use serde_json;
use anyhow::Result;

/// Represents a report detailing which files (and elements) were changed.
#[derive(Debug, Serialize)]
pub struct ChangeImpactReport {
    /// List of files that changed according to `git diff`.
    pub changed_files: Vec<String>,
    /// List of changed Markdown files considered to have impacted elements.
    pub impacted_elements: Vec<String>,
}

impl ChangeImpactReport {
    /// Generates the change impact report by invoking the `git diff` command.
    ///
    /// This implementation runs:
    /// 
    ///   git diff --name-only
    /// 
    /// to get a list of changed files and then marks all Markdown files (.md)
    /// as potentially impacting elements. A production implementation might
    /// re‑parse the Markdown diff to determine which elements (identified using
    /// the rules from the specification documents) are changed.
    pub fn generate() -> Result<Self, Box<dyn Error>> {
        // Run the git diff command to get the list of changed files.
        let output = Command::new("git")
            .args(&["diff", "--name-only"])
            .output()?;
            
        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("git diff failed: {}", err),
            )));
        }
        
        // Collect changed file paths from the git diff output.
        let stdout = String::from_utf8_lossy(&output.stdout);
        let changed_files: Vec<String> = stdout
            .lines()
            .map(|s| s.to_string())
            .filter(|s| !s.trim().is_empty())
            .collect();
        
        // For Markdown files (ending with .md), assume that elements may be impacted.
        let impacted_elements: Vec<String> = changed_files
            .iter()
            .filter(|file| file.ends_with(".md"))
            .cloned()
            .collect();
        
        Ok(ChangeImpactReport {
            changed_files,
            impacted_elements,
        })
    }

    /// Prints the change impact report in a human-friendly format.
    pub fn print_report(&self) {
        println!("=== Change Impact Report ===");
        println!("\nChanged Files:");
        for file in &self.changed_files {
            println!("  - {}", file);
        }
        println!("\nPotentially Impacted Elements (Markdown files):");
        for file in &self.impacted_elements {
            println!("  - {}", file);
        }
    }

    /// Prints the report in a pretty manner.
    ///
    /// If `as_json` is true, the report is printed as pretty-printed JSON.
    /// Otherwise, the human-friendly text format is used.
    pub fn print_pretty(&self, as_json: bool) {
        if as_json {
            match serde_json::to_string_pretty(self) {
                Ok(json_str) => println!("{}", json_str),
                Err(e) => eprintln!("Failed to serialize report to JSON: {}", e),
            }
        } else {
            self.print_report();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_report() {
        // Note: In real tests, you may want to mock Command::new("git") to return a controlled output.
        // For now, we only test that the function returns a report (or an error if no git repository is found).
        let report = ChangeImpactReport::generate();
        // Depending on the testing environment, this may fail if there is no git repo,
        // so we simply check that the Result is handled.
        match report {
            Ok(rep) => {
                // Print both formats for visual inspection.
                println!("Human-friendly report:");
                rep.print_pretty(false);
                println!("\nJSON report:");
                rep.print_pretty(true);
            }
            Err(e) => {
                eprintln!("Failed to generate change impact report: {}", e);
            }
        }
    }
}

