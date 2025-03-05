#[cfg(test)]
mod config_tests {
    use std::fs;
    use tempfile::tempdir;
    use crate::config::Config;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        
        assert_eq!(config.general.html_output, false);
        assert_eq!(config.general.verbose, false);
        
        assert_eq!(config.paths.specifications_folder, "specifications");
        assert_eq!(config.paths.design_specifications_folder, "DesignSpecifications");
        assert_eq!(config.paths.output_folder, "output");
        
        assert_eq!(config.validation.validate_markdown, false);
        assert_eq!(config.validation.validate_relations, false);
        assert_eq!(config.validation.validate_all, false);
        assert_eq!(config.validation.json_output, false);
        
        // Verify linting defaults
        assert_eq!(config.linting.lint, false);
        assert_eq!(config.linting.dry_run, false);
        
        // Verify style defaults
        assert_eq!(config.style.theme, "default");
        assert_eq!(config.style.max_width, 1200);
        assert_eq!(config.style.custom_css, None);
    }

    #[test]
    fn test_load_from_yaml() {
        // Create a temporary directory for our test config
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("reqflow.yml");
        
        // Create a test YAML configuration
        let yaml_content = r#"
general:
  verbose: true

paths:
  specifications_folder: "docs"
  design_specifications_folder_name: "Design"
  output_folder: "generated"

style:
  theme: "dark"
  max_width: 1000
  custom_css: "custom.css"
"#;
        
        // Write the test config to the temporary file
        fs::write(&config_path, yaml_content).unwrap();
        
        // Load the config
        let config = Config::from_file(&config_path).unwrap();
        
        // Verify the config values were loaded correctly
        assert_eq!(config.general.verbose, true);
        
        assert_eq!(config.paths.specifications_folder, "docs");
        assert_eq!(config.paths.design_specifications_folder, "Design");
        assert_eq!(config.paths.output_folder, "generated");
        
        assert_eq!(config.style.theme, "dark");
        assert_eq!(config.style.max_width, 1000);
        assert_eq!(config.style.custom_css, Some("custom.css".to_string()));
        
        // Validation should still be set to default values
        assert_eq!(config.validation.validate_markdown, false);
    }
    
    
    #[test]
    fn test_element_type_detection() {
        // This test will be updated to test the new element type detection through metadata
        // when the corresponding test infrastructure is available
    }
}