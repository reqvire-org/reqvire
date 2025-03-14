use std::env;
use std::path::Path;
use serde::Serialize;
use crate::config::Config;
use crate::model::ModelManager;
use crate::error::ReqFlowError;

/// Structure for JSON output of validation results
#[derive(Serialize)]
struct ValidationResult {
    validation_type: String,
    errors: Vec<String>,
    fixed: bool, // Kept for API compatibility but always false now
}

/// Helper function to print validation results
fn print_validation_results(validation_type: &str, errors: &[ReqFlowError], json_output: bool) {
    if json_output {
        let json_result = ValidationResult {
            validation_type: validation_type.to_string(),
            errors: errors.iter().map(|e| e.to_string()).collect(),
            fixed: false,
        };
        println!("{}", serde_json::to_string_pretty(&json_result).unwrap());
    } else {
        println!("❌ {} validation failed with {} errors.", validation_type, errors.len());
        for error in errors {
            println!("  - {}", error);
        }
    }
}

fn main() {
    let config = Config {
        paths: config::PathsConfig {
            specifications_folder: "/specifications".to_string(),
            output_folder: "/output".to_string(),
            external_folders: vec!["/external1".to_string(), "/external2".to_string()],
            excluded_filename_patterns: vec![],
            base_path: std::env::current_dir().unwrap(),
        },
        ..Default::default()
    };

    let input_folder = Path::new("/path/to/markdown");

    // Check if the user provided `--json` flag
    let json_output = env::args().any(|arg| arg == "--json");

    let mut model_manager = ModelManager::new(config);

    match model_manager.parse_and_validate(input_folder) {
        Ok(errors) => {
            if errors.is_empty() {
                println!("✅ Validation completed successfully with no errors.");
            } else {
                print_validation_results("Validation Summary", &errors, json_output);
            }
        }
        Err(e) => eprintln!("❌ Validation failed: {}", e),
    }
}

