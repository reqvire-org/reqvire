use std::fs;
use std::path::Path;
use anyhow::Result;
use log::{debug, warn};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::env;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::process;


/// Configuration settings for the Reqvire application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub paths: PathsConfig,
    
    #[serde(default)]
    pub style: StyleConfig,
    
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathsConfig {
    #[serde(default)]
    pub user_requirements_root_folder: String,
    #[serde(skip)]
    pub base_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StyleConfig {
    pub theme: String,
    pub max_width: usize,
    pub custom_css: Option<String>,
    #[serde(default = "default_diagram_direction")]
    pub diagram_direction: String,
    #[serde(default = "default_diagrams_with_blobs")]    
    pub diagrams_with_blobs: bool,    
}




impl Default for PathsConfig {
    fn default() -> Self {

        Self {
            user_requirements_root_folder: "".to_string(),
            base_path: env::current_dir().expect("Failed to get current directory"),
        }
    }
}


// Default value for diagram direction
fn default_diagram_direction() -> String {
    "TD".to_string()  // Default to top-down layout
}

// Default diagrams with blobs
fn default_diagrams_with_blobs() -> bool {
   false
}


impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            max_width: 1200,
            custom_css: None,
            diagram_direction: default_diagram_direction(),
            diagrams_with_blobs: false
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paths: PathsConfig::default(),
            style: StyleConfig::default()
        }
    }
}


impl Config {    

    pub fn get_user_requirements_root_folder(&self) -> Option<std::path::PathBuf> {
        if self.paths.user_requirements_root_folder.is_empty() {
            return None;
        }
        
        let path = self.paths.base_path.join(self.paths.user_requirements_root_folder.clone());
        match path.canonicalize() {
            Ok(p) => Some(p),
            Err(e) => {
                eprintln!("ERROR: Failed to resolve user requirements root folder at {:?}: {}", path, e);
                process::exit(1);
            }
        }
    }

   
    
    /// Finds the root of the git repository
    fn find_git_root() -> Option<PathBuf> {
        let current_dir = env::current_dir().ok()?;
        let mut dir = current_dir.as_path();

        loop {
            if dir.join(".git").exists() {
                return Some(dir.to_path_buf());
            }

            dir = dir.parent()?;
        }
    }

    /// Reads gitignore patterns from the repository root .gitignore file
    fn read_gitignore_patterns() -> Vec<String> {
        let git_root = match Self::find_git_root() {
            Some(root) => root,
            None => {
                debug!("No git repository found, skipping .gitignore");
                return vec![];
            }
        };

        let gitignore_path = git_root.join(".gitignore");

        if !gitignore_path.exists() {
            debug!("No .gitignore file found at repository root");
            return vec![];
        }

        // Read the gitignore file and extract patterns
        // Note: We're just parsing the file content directly rather than using
        // the gitignore builder since we need to convert patterns to globs
        match fs::read_to_string(&gitignore_path) {
            Ok(content) => {
                content
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
                    .map(|line| {
                        let pattern = line.trim();
                        // Convert gitignore patterns to glob patterns

                        // If pattern already starts with globstar, use as-is
                        if pattern.starts_with("**/") || pattern.starts_with("**") {
                            pattern.to_string()
                        } else if pattern.ends_with('/') {
                            // Directory pattern - match everything under directory anywhere in tree
                            let dir_name = pattern.trim_end_matches('/');
                            format!("**/{}/**", dir_name)
                        } else if pattern.contains('/') {
                            // Path pattern - use as-is with globstar prefix if needed
                            if pattern.starts_with('/') {
                                format!("**{}", pattern)
                            } else {
                                format!("**/{}", pattern)
                            }
                        } else {
                            // Filename pattern - match anywhere in tree
                            format!("**/{}", pattern)
                        }
                    })
                    .collect()
            }
            Err(e) => {
                warn!("Failed to read .gitignore content: {}", e);
                vec![]
            }
        }
    }

    /// Returns reserved filenames that are always excluded from structured markdown processing
    fn reserved_filenames() -> Vec<String> {
        vec![
            "**/README.md".to_string(),
            "**/CHANGELOG.md".to_string(),
            "**/CHANGES.md".to_string(),
            "**/CONTRIBUTING.md".to_string(),
            "**/LICENSE.md".to_string(),
            "**/CODE_OF_CONDUCT.md".to_string(),
            "**/SECURITY.md".to_string(),
            "**/AUTHORS.md".to_string(),
            "**/ROADMAP.md".to_string(),
            "**/CLAUDE.md".to_string(),
            "**/AGENT.md".to_string(),
            "**/AI.md".to_string(),
            "**/PROMPT.md".to_string(),
            "**/INSTRUCTIONS.md".to_string(),
            "**/CONTEXT.md".to_string(),
            "**/CURSOR.md".to_string(),
            "**/COPILOT.md".to_string(),
        ]
    }

    /// Reads reqvireignore patterns from the repository root .reqvireignore file
    fn read_reqvireignore_patterns() -> Vec<String> {
        let git_root = match Self::find_git_root() {
            Some(root) => root,
            None => {
                debug!("No git repository found, skipping .reqvireignore");
                return vec![];
            }
        };

        let reqvireignore_path = git_root.join(".reqvireignore");

        if !reqvireignore_path.exists() {
            debug!("No .reqvireignore file found at repository root");
            return vec![];
        }

        // Read the reqvireignore file and extract patterns
        // Same format as gitignore patterns
        match fs::read_to_string(&reqvireignore_path) {
            Ok(content) => {
                content
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
                    .map(|line| {
                        let pattern = line.trim();
                        // Convert reqvireignore patterns to glob patterns

                        // If pattern already starts with globstar, use as-is
                        if pattern.starts_with("**/") || pattern.starts_with("**") {
                            pattern.to_string()
                        } else if pattern.ends_with('/') {
                            // Directory pattern - match everything under directory anywhere in tree
                            let dir_name = pattern.trim_end_matches('/');
                            format!("**/{}/**", dir_name)
                        } else if pattern.contains('/') {
                            // Path pattern - use as-is with globstar prefix if needed
                            if pattern.starts_with('/') {
                                format!("**{}", pattern)
                            } else {
                                format!("**/{}", pattern)
                            }
                        } else {
                            // Filename pattern - match anywhere in tree
                            format!("**/{}", pattern)
                        }
                    })
                    .collect()
            }
            Err(e) => {
                warn!("Failed to read .reqvireignore content: {}", e);
                vec![]
            }
        }
    }

    /// Builds a GlobSet from reserved filenames, gitignore, and reqvireignore
    pub fn get_excluded_filename_patterns_glob_set(&self) -> GlobSet {
        let mut builder = GlobSetBuilder::new();

        // Add reserved filenames (always excluded)
        for pattern in Self::reserved_filenames() {
            if let Ok(glob) = Glob::new(&pattern) {
                builder.add(glob);
                debug!("Added reserved filename pattern: {}", pattern);
            } else {
                warn!("Invalid reserved filename pattern: {}", pattern);
            }
        }

        // Add patterns from root .gitignore
        for pattern in Self::read_gitignore_patterns() {
            if let Ok(glob) = Glob::new(&pattern) {
                builder.add(glob);
                debug!("Added gitignore pattern: {}", pattern);
            } else {
                warn!("Invalid gitignore pattern: {}", pattern);
            }
        }

        // Add patterns from root .reqvireignore
        for pattern in Self::read_reqvireignore_patterns() {
            if let Ok(glob) = Glob::new(&pattern) {
                builder.add(glob);
                debug!("Added reqvireignore pattern: {}", pattern);
            } else {
                warn!("Invalid reqvireignore pattern: {}", pattern);
            }
        }

        builder.build().expect("Failed to build glob set")
    }
    
    
    /// Load configuration from CLI arguments and update settings accordingly
    pub fn load_from_args(args: &crate::cli::Args) -> Self {
        // Load configuration from a file if provided, otherwise use default
        let config = match &args.config {
            Some(config_path) => match Self::from_file(config_path) {
                Ok(cfg) => {
                    log::info!("Loaded configuration from {:?}", config_path);
                    cfg
                }
                Err(e) => {
                    log::error!("Error loading configuration from {:?}: {}", config_path, e);
                    log::warn!("Using default configuration");
                    Self::default()
                }
            },
            None => Self::load(), // Load from default locations
        };


        config
    }
    /// Load configuration from a YAML file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_yml::from_str(&content)?;
        Ok(config)
    }
    
    /// Find and load the configuration file
    pub fn load() -> Self {
        let config_paths = [
            "reqvire.yml",
            "reqvire.yaml",
            ".reqvire.yaml",
            ".reqvire.yml",                                    
        ];
        
        let mut config = None;
        
        for path_str in &config_paths {
            let path = Path::new(path_str);
            if path.exists() {
                match Self::from_file(path) {
                    Ok(loaded_config) => {
                        debug!("Loaded configuration from {}", path_str);
                        config = Some(loaded_config);
                        break;
                    },
                    Err(e) => {
                        warn!("Error loading configuration from {}: {}", path_str, e);
                    }
                }
            }
        }
        
        // If no config file was found, use the default config
        let final_config = config.unwrap_or_else(|| {
            warn!("No configuration file found, using defaults");
            Self::default()
        });
        

        
        // No path normalization needed - we handle exact folder names in the validation logic
        
        final_config
    }

    /// Embedded CSS styles for HTML output
    pub fn embedded_styles() -> &'static str {
        r#"
<style>
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
    line-height: 1.6;
    margin: 0;
    padding: 0;
    background-color: #f8f9fa;
    color: #333;
}
.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
}
.content {
    background-color: #fff;
    padding: 30px;
    border-radius: 5px;
    box-shadow: 0 2px 5px rgba(0,0,0,0.1);
}
h1 {
        
    color: #333;
    border-bottom: 2px solid #eaecef;
    padding-bottom: 10px;
    margin-top: 0;
}
h2 {
    color: #333;
    border-bottom: 1px solid #eaecef;
    padding-bottom: 5px;
}
h3 {
    color: #1a6fb7;
    margin-top: 25px;
}
h4 {
    color: #555;
    font-weight: 600;
}
a {
    color: #0366d6;
    text-decoration: none;
}
a:hover {
    text-decoration: underline;
}
table {
    border-collapse: collapse;
    width: 100%;
    margin: 20px 0;
}
table, th, td {
    border: 1px solid #dfe2e5;
}
th, td {
    padding: 8px 12px;
    text-align: left;
}
th {
    background-color: #f6f8fa;
}
code {
    background: #f6f8fa;
    padding: 2px 4px;
    border-radius: 3px;
    font-family: SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace;
    font-size: 90%;
}
pre {
    background: #f6f8fa;
    border-radius: 3px;
    padding: 16px;
    overflow: auto;
}
pre code {
    background: transparent;
    padding: 0;
}
blockquote {
    margin: 0;
    padding: 0 15px;
    color: #777;
    border-left: 4px solid #dfe2e5;
}
.mermaid {
    margin: 20px 0;
    text-align: center;
}
</style>
"#
    }

    /// HTML template for generated pages
    pub fn html_template() -> &'static str {
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    {styles}
    <!-- Enhanced mermaid configuration for Reqvire diagrams -->
    <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
    <script>
        mermaid.initialize({ 
            startOnLoad: true,
            theme: 'neutral',
            maxTextSize: 90000,
            flowchart: {
                useMaxWidth: true,
                htmlLabels: true,
                curve: 'basis'
            },
            securityLevel: 'loose'
        });
    </script>
</head>
<body>
    <div class="container">
        <div class="content">
            {content}
        </div>
    </div>
</body>
</html>
"#
    }
}


#[cfg(test)]
mod config_tests {
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;
    use crate::config::Config;


    #[test]
    fn test_default_config() {
        let config = Config::default();

        assert_eq!(config.paths.user_requirements_root_folder, "");

        // Build globset to verify it works correctly
        // (The actual patterns will be tested in e2e tests)
        let globset = config.get_excluded_filename_patterns_glob_set();
        // Note: Whether files are excluded depends on reserved filenames, .gitignore, and .reqvireignore
        // This test just verifies the globset builds successfully

        // Verify style defaults
        assert_eq!(config.style.theme, "default");
        assert_eq!(config.style.max_width, 1200);
        assert_eq!(config.style.custom_css, None);
    }

    #[test]
    fn test_load_from_yaml() {
        // Create a temporary directory for our test config
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("reqvire.yml");

        // Create a test YAML configuration
        let yaml_content = r#"
paths:
  user_requirements_root_folder: "specifications"

style:
  theme: "dark"
  max_width: 1000
  custom_css: "custom.css"
"#;

        // Write the test config to the temporary file
        fs::write(&config_path, yaml_content).unwrap();

        // Load the config
        let config = Config::from_file(&config_path).unwrap();

        assert_eq!(config.paths.user_requirements_root_folder, "specifications");

        assert_eq!(config.style.theme, "dark");
        assert_eq!(config.style.max_width, 1000);
        assert_eq!(config.style.custom_css, Some("custom.css".to_string()));

    }
    
    
    #[test]
    fn test_element_type_detection() {
        // This test will be updated to test the new element type detection through metadata
        // when the corresponding test infrastructure is available
    }
}
