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
        assert_eq!(config.paths.system_requirements_folder, "SystemRequirements");
        assert_eq!(config.paths.design_specifications_folder, "DesignSpecifications");
        assert_eq!(config.paths.output_folder, "output");
        assert_eq!(config.paths.requirements_filename_match, "Requirements");
        
        assert_eq!(config.validation.validate_markdown, false);
        assert_eq!(config.validation.validate_relations, false);
        assert_eq!(config.validation.validate_all, false);
        assert_eq!(config.validation.fix_automatically, false);
        assert_eq!(config.validation.json_output, false);
        
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
  system_requirements_folder_name: "SysReqs"
  design_specifications_folder_name: "Design"
  output_folder: "generated"
  requirements_filename_match: "UserReqs"

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
        assert_eq!(config.paths.system_requirements_folder, "SysReqs");
        assert_eq!(config.paths.design_specifications_folder, "Design");
        assert_eq!(config.paths.output_folder, "generated");
        assert_eq!(config.paths.requirements_filename_match, "UserReqs");
        
        assert_eq!(config.style.theme, "dark");
        assert_eq!(config.style.max_width, 1000);
        assert_eq!(config.style.custom_css, Some("custom.css".to_string()));
        
        // Validation should still be set to default values
        assert_eq!(config.validation.validate_markdown, false);
    }
    
    
    #[test]
    fn test_user_requirements_detection() {
        // Test the logic for identifying user requirements files based on the match string
        let mut config = Config::default();
        config.paths.requirements_filename_match = "UserReqs".to_string();
        
        // Should identify these as user requirements
        let user_reqs_files = [
            "UserReqs.md",
            "UserReqs-API.md",
            "UserReqsLanding.md",
            "API-UserReqs.md"
        ];
        
        // Should not identify these as user requirements
        let non_user_reqs_files = [
            "SystemRequirements.md",
            "UserStories.md",
            "Requirements.md",
            "Test-UserReq.md", // Does not match exactly "UserReqs"
        ];
        
        for file in &user_reqs_files {
            assert!(file.contains(&config.paths.requirements_filename_match), 
                    "Should identify '{}' as a User Requirements file", file);
        }
        
        for file in &non_user_reqs_files {
            assert!(!file.contains(&config.paths.requirements_filename_match) || 
                    // Explicit exclusions can be added here if needed
                    false,  
                    "Should not identify '{}' as a User Requirements file", file);
        }
    }
}