#[cfg(test)]
mod config_tests {
    use std::fs;
    use std::path::Path;
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
        
        assert_eq!(config.style.theme, "dark");
        assert_eq!(config.style.max_width, 1000);
        assert_eq!(config.style.custom_css, Some("custom.css".to_string()));
        
        // Validation should still be set to default values
        assert_eq!(config.validation.validate_markdown, false);
    }
    
    #[test]
    fn test_system_requirements_path() {
        let mut config = Config::default();
        config.paths.specifications_folder = "docs".to_string();
        config.paths.system_requirements_folder = "SysReqs".to_string();
        
        let base_path = Path::new("/project");
        let sys_reqs_path = config.system_requirements_path(base_path);
        
        assert_eq!(sys_reqs_path, Path::new("/project/docs/SysReqs"));
    }
    
    #[test]
    fn test_design_specifications_path() {
        let mut config = Config::default();
        config.paths.specifications_folder = "docs".to_string();
        config.paths.design_specifications_folder = "Design".to_string();
        
        let base_path = Path::new("/project");
        let design_specs_path = config.design_specifications_path(base_path);
        
        assert_eq!(design_specs_path, Path::new("/project/docs/Design"));
    }
}