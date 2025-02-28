use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod config;
mod element;
mod error;
mod html;
mod linting;
mod markdown;
mod model;
mod relation;
mod utils;
mod validation;
#[cfg(test)]
mod tests;
#[path = "tests/validation_tests.rs"]
#[cfg(test)]
mod validation_tests;
#[path = "tests/config_tests.rs"]
#[cfg(test)]
mod config_tests;
#[path = "tests/linting_tests.rs"]
#[cfg(test)]
mod linting_tests;

use model::ModelManager;

/// ReqFlow is an agile Model-Based Systems Engineering (MBSE) framework
/// designed to integrate seamlessly with modern Git workflows.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Path to the input folder containing Markdown files
    /// If not provided, this will be read from the configuration file
    #[clap(index = 1)]
    input_folder: Option<PathBuf>,

    /// Path to the output folder (not required for validate commands)
    /// If not provided, this will be read from the configuration file
    #[clap(index = 2)]
    output_folder: Option<PathBuf>,

    /// Convert Markdown to HTML with embedded styles
    #[clap(long)]
    html: bool,

    /// Enable verbose output
    #[clap(short, long)]
    verbose: bool,
    
    /// Validate Markdown structure only
    /// Checks for:
    /// - Unique element names within documents
    /// - Proper heading hierarchy
    /// - Well-formed relation sections
    #[clap(long, conflicts_with = "html")]
    validate_markdown: bool,
    
    /// Validate relations only
    /// Checks for:
    /// - Valid relation types according to spec
    /// - Valid relation targets (elements and files exist)
    /// - Proper relation formatting
    #[clap(long, conflicts_with = "html")]
    validate_relations: bool,
    
    /// Run all validations
    /// Includes markdown structure validation, relation validation,
    /// and cross-component dependency validation
    #[clap(long, conflicts_with = "html")]
    validate_all: bool,
    
    /// Enable linting to find potential improvements (non-blocking)
    /// By default, fixes will be applied automatically
    #[clap(long)]
    lint: bool,
    
    /// When linting, only show suggestions without applying fixes
    #[clap(long, requires = "lint")]
    dry_run: bool,
    
    
    /// Output validation results in JSON format
    /// Useful for CI/CD pipelines and automation
    #[clap(long)]
    json: bool,
    
    /// Path to a custom configuration file (YAML format)
    /// If not provided, the system will look for reqflow.yml, reqflow.yaml, 
    /// .reqflow.yml, or .reqflow.yaml in the current directory
    #[clap(long, short = 'c')]
    config: Option<PathBuf>,
}

fn main() -> Result<()> {
    // Initialize logger with a more verbose default level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let args = Args::parse();

    // Load configuration
    let mut config = match &args.config {
        Some(config_path) => {
            match config::Config::from_file(config_path) {
                Ok(cfg) => {
                    println!("Loaded configuration from {:?}", config_path);
                    cfg
                },
                Err(e) => {
                    eprintln!("Error loading configuration from {:?}: {}", config_path, e);
                    eprintln!("Using default configuration");
                    config::Config::default()
                }
            }
        },
        None => {
            config::Config::load()
        }
    };
    
    // Update config with command line arguments
    if args.html {
        config.general.html_output = true;
    }
    
    if args.verbose {
        config.general.verbose = true;
    }
    
    if args.validate_markdown {
        config.validation.validate_markdown = true;
    }
    
    if args.validate_relations {
        config.validation.validate_relations = true;
    }
    
    if args.validate_all {
        config.validation.validate_all = true;
    }
    
    if args.lint {
        config.linting.lint = true;
        config.linting.dry_run = args.dry_run;
        config.linting.requirements_only = true; // Always only lint requirements files
    }
    
    if args.json {
        config.validation.json_output = true;
    }
    
    // Get input and output folders from command line arguments or configuration
    let input_folder_path = args.input_folder
        .unwrap_or_else(|| PathBuf::from(&config.paths.specifications_folder));
    
    let output_folder_path = args.output_folder
        .unwrap_or_else(|| PathBuf::from(&config.paths.output_folder));

    // Create model manager with the configuration
    let mut model_manager = ModelManager::new_with_config(config.clone());
    
    // Determine if we're in validation mode
    let validation_mode = config.validation.validate_markdown || 
                          config.validation.validate_relations || 
                          config.validation.validate_all;
                          
    // Determine if we're in linting mode
    let linting_mode = config.linting.lint;
    
    if validation_mode {
        // Run in validation mode
        if config.general.verbose {
            println!("Validating files in {:?}", input_folder_path);
        }
        
        // Only collect identifiers, don't process files
        model_manager.collect_identifiers_only(&input_folder_path)?;
        
        // Run the appropriate validations
        let mut exit_code = 0;
        
        if config.validation.validate_markdown || config.validation.validate_all {
            let markdown_errors = model_manager.validate_markdown_structure()?;
            if !markdown_errors.is_empty() {
                exit_code = 1;
                
                if config.validation.json_output {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "markdown".to_string(),
                        errors: markdown_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: false,
                    })?);
                } else {
                    // Human-readable output
                    println!("❌ Markdown validation failed with {} errors:", markdown_errors.len());
                    for error in markdown_errors {
                        println!("  - {}", error);
                    }
                }
            } else if config.general.verbose || !config.validation.json_output {
                println!("✅ Markdown structure validation passed");
            }
        }
        
        if config.validation.validate_relations || config.validation.validate_all {
            let relation_errors = model_manager.validate_relations()?;
            if !relation_errors.is_empty() {
                exit_code = 1;
                
                if config.validation.json_output {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "relations".to_string(),
                        errors: relation_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: false,
                    })?);
                } else {
                    // Human-readable output
                    println!("❌ Relation validation failed with {} errors:", relation_errors.len());
                    for error in relation_errors {
                        println!("  - {}", error);
                    }
                }
            } else if config.general.verbose || !config.validation.json_output {
                println!("✅ Relations validation passed");
            }
        }
        
        if config.validation.validate_all {
            // Filesystem structure validation (directory layout, file naming, etc.)
            let filesystem_errors = model_manager.validate_filesystem_structure(&input_folder_path, &config)?;
            if !filesystem_errors.is_empty() {
                exit_code = 1;
                
                if config.validation.json_output {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "filesystem".to_string(),
                        errors: filesystem_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: false,
                    })?);
                } else {
                    // Human-readable output
                    println!("❌ Filesystem structure validation failed with {} errors:", filesystem_errors.len());
                    for error in filesystem_errors {
                        println!("  - {}", error);
                    }
                }
            } else if config.general.verbose || !config.validation.json_output {
                println!("✅ Filesystem structure validation passed");
            }
            
            // Cross-component dependency validation
            let dependency_errors = model_manager.validate_cross_component_dependencies()?;
            if !dependency_errors.is_empty() {
                exit_code = 1;
                
                if config.validation.json_output {
                    // Output JSON for CI/CD integration
                    println!("{}", serde_json::to_string_pretty(&ValidationResult {
                        validation_type: "dependencies".to_string(),
                        errors: dependency_errors.iter().map(|e| e.to_string()).collect(),
                        fixed: false,
                    })?);
                } else {
                    // Human-readable output
                    println!("❌ Cross-component dependency validation failed with {} errors:", dependency_errors.len());
                    for error in dependency_errors {
                        println!("  - {}", error);
                    }
                }
            } else if config.general.verbose || !config.validation.json_output {
                println!("✅ Cross-component dependency validation passed");
            }
        }
        
        // Exit with appropriate code based on validation results
        if exit_code != 0 {
            std::process::exit(exit_code);
        }
        
        println!("All validations passed successfully!");
    } else if linting_mode {
        // Run in linting mode
        if config.general.verbose {
            let dry_run_msg = if config.linting.dry_run { " (dry run)" } else { "" };
            
            println!("Linting requirements files in {:?}{}", 
                input_folder_path, 
                dry_run_msg);
        }
        
        // Run the linting with the configuration
        let lint_suggestions = linting::lint_directory_with_config(
            &input_folder_path, 
            config.linting.dry_run,
            &config
        )?;
        
        if lint_suggestions.is_empty() {
            println!("✅ No linting suggestions found. Your files are clean!");
        } else {
            println!("ℹ️ Found {} linting suggestions:", lint_suggestions.len());
            
            // Group suggestions by file and type for better readability
            let mut grouped_by_file: std::collections::HashMap<String, Vec<&linting::LintSuggestion>> = 
                std::collections::HashMap::new();
            
            for suggestion in &lint_suggestions {
                let file_path = suggestion.file_path.to_string_lossy().to_string();
                grouped_by_file.entry(file_path).or_default().push(suggestion);
            }
            
            // Print a summary first
            println!("Files with formatting issues:");
            for (file_path, suggestions) in &grouped_by_file {
                // Group by issue type within each file
                let mut type_counts: std::collections::HashMap<linting::LintType, usize> = 
                    std::collections::HashMap::new();
                
                for suggestion in suggestions {
                    *type_counts.entry(suggestion.suggestion_type.clone()).or_default() += 1;
                }
                
                // Check if this is a requirements file (more thorough check)
                let path = std::path::Path::new(file_path);
                let is_requirements_file = if let Some(filename) = path.file_name() {
                    let filename_str = filename.to_string_lossy().to_lowercase();
                    let path_str = path.to_string_lossy().to_lowercase();
                    let req_pattern = config.paths.requirements_filename_match.to_lowercase();
                    
                    // Check filename and path
                    filename_str.contains(&req_pattern) || 
                    path_str.contains(&format!("/{}/", config.paths.system_requirements_folder.to_lowercase())) ||
                    path_str.contains(&format!("/{}/", req_pattern.to_lowercase()))
                } else {
                    false
                };
                
                // Add a tag to indicate if this is a requirements file
                let file_type_tag = if is_requirements_file {
                    "[requirement document]"
                } else {
                    "[other document]" 
                };
                
                println!("  {} {} ({})", file_path, file_type_tag, suggestions.len());
                for (lint_type, count) in &type_counts {
                    let type_name = match lint_type {
                        linting::LintType::AbsoluteLink => "Absolute links",
                        linting::LintType::ExcessWhitespace => "Excess whitespace",
                        linting::LintType::InconsistentNewlines => "Inconsistent newlines",
                        linting::LintType::MissingSeparator => "Missing separators",
                        linting::LintType::InconsistentIndentation => "Inconsistent indentation",
                    };
                    println!("    - {}: {}", type_name, count);
                }
            }
            
            // Then print detailed git-style diffs
            println!("\nSuggested changes (git diff style):");
            
            // Sort suggestions by file and line number for easier reading
            let mut sorted_suggestions = lint_suggestions.clone();
            sorted_suggestions.sort_by(|a, b| {
                let file_cmp = a.file_path.to_string_lossy().cmp(&b.file_path.to_string_lossy());
                if file_cmp == std::cmp::Ordering::Equal {
                    a.line_number.unwrap_or(0).cmp(&b.line_number.unwrap_or(0))
                } else {
                    file_cmp
                }
            });
            
            for suggestion in &sorted_suggestions {
                // Print the colorized diff, ignoring any errors
                let _ = suggestion.print_colorized_diff();
            }
            
            if config.linting.dry_run {
                println!("Run without --dry-run to apply these fixes automatically");
            } else {
                println!("✅ All suggestions have been applied");
            }
        }
    } else {
        // Normal processing mode
        if config.general.verbose {
            println!("Processing files from {:?} to {:?}", input_folder_path, output_folder_path);
        }

        // Process files normally
        model_manager.process_files(&input_folder_path, 
                                  &output_folder_path, 
                                  config.general.html_output)?;

        println!("Files processed and saved to {:?}", output_folder_path);
    }
    
    Ok(())
}

/// Structure for JSON output of validation results
#[derive(serde::Serialize)]
struct ValidationResult {
    validation_type: String,
    errors: Vec<String>,
    fixed: bool, // Kept for API compatibility but always false now
}