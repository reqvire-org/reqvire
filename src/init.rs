use std::fs;
use std::path::Path;
use log::{info, warn};

use crate::error::ReqFlowError;

/// Initialize a new ReqFlow project with basic structure
pub fn initialize_project(target_dir: &Path) -> Result<(), ReqFlowError> {
    info!("Initializing a new ReqFlow project in {:?}", target_dir);
    
    // Create the target directory if it doesn't exist
    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?;
        info!("Created target directory {:?}", target_dir);
    }
    
    // Create the basic directory structure
    let specs_dir = target_dir.join("specifications");
    let system_reqs_dir = specs_dir.join("SystemRequirements");
    let design_specs_dir = specs_dir.join("DesignSpecifications");
    
    if !specs_dir.exists() {
        fs::create_dir_all(&specs_dir)?;
        info!("Created specifications directory");
    }
    
    if !system_reqs_dir.exists() {
        fs::create_dir_all(&system_reqs_dir)?;
        info!("Created SystemRequirements directory");
    }
    
    if !design_specs_dir.exists() {
        fs::create_dir_all(&design_specs_dir)?;
        info!("Created DesignSpecifications directory");
    }
    
    // Create example configuration file
    let config_path = target_dir.join("reqflow.yaml");
    if !config_path.exists() {
        let config_content = r#"# ReqFlow Configuration

general:
  # Enable verbose output
  verbose: false
  # Generate diagrams automatically
  generate_diagrams: true

paths:
  # Folder name for system requirements 
  system_requirements_folder: "SystemRequirements"
  
  # Folder name for design specifications 
  design_specifications_folder: "DesignSpecifications"
  
  # String to match in filename for identifying Requirements files
  requirements_filename_match: "Requirements"

style:
  # Diagram direction (TD for top-down, LR for left-to-right)
  diagram_direction: "TD"

validation:
  # Validate markdown structure
  validate_markdown: true
  
  # Validate relations between elements
  validate_relations: true
  
  # Validate cross-component dependencies
  validate_cross_components: true
  
linting:
  # Enable linting
  lint: true
  
  # Show suggestions without applying (dry run mode)
  dry_run: false
"#;
        fs::write(&config_path, config_content)?;
        info!("Created configuration file at {:?}", config_path);
    } else {
        warn!("Configuration file already exists, skipping");
    }
    
    info!("Project initialization complete!");
    print_next_steps();
    
    Ok(())
}

/// Print next steps for the user after initialization
fn print_next_steps() {
    println!("\n======= ReqFlow Project Initialized =======");
    println!("A basic ReqFlow project structure has been created.");
    println!("\nDirectory structure:");
    println!("- specifications/");
    println!("  - SystemRequirements/");
    println!("  - DesignSpecifications/");
    println!("- reqflow.yaml (configuration file)");
    println!("\nNext steps:");
    println!("1. Create your requirements files in the appropriate directories");
    println!("2. Follow the format in the documentation for creating requirements");
    println!("3. Use ReqFlow commands to validate, lint, and generate documentation");
    println!("\nExample commands:");
    println!("- reqflow --validate-all");
    println!("- reqflow --lint");
    println!("- reqflow --generate-diagrams");
    println!("- reqflow --html output/");
    println!("\nFor more information, run `reqflow --help` or `reqflow --llm-context`");
    println!("===========================================");
}