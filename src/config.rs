use std::fs;
use std::path::Path;
use anyhow::Result;
use log::{debug, warn};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::env;
use globset::{Glob, GlobSet, GlobSetBuilder};
 
/// Configuration settings for the ReqFlow application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub paths: PathsConfig,
    
    #[serde(default)]
    pub style: StyleConfig,
    
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathsConfig {
    pub specifications_folder: String,
    pub output_folder: String,
    #[serde(default)]
    pub external_folders: Vec<String>,
    #[serde(default)]
    pub excluded_filename_patterns: Vec<String>,
    /// Base path where the tool is running (default: current working directory)
    #[serde(skip)]  // Skip serialization (hidden in config files)
    pub base_path: PathBuf,             
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StyleConfig {
    pub theme: String,
    pub max_width: usize,
    pub custom_css: Option<String>,
    #[serde(default = "default_diagram_direction")]
    pub diagram_direction: String,
}



impl Default for PathsConfig {     
    fn default() -> Self {
    
        Self {
            specifications_folder: "specifications".to_string(),
            output_folder: "output".to_string(),
            external_folders: Vec::new(),
            excluded_filename_patterns: vec![
                "**/README*.md".to_string(), 
                "**/index.md".to_string(),
                // Exclude design specifications folder by pattern instead of dedicated parameter
                "**/DesignSpecifications/**/*.md".to_string()
            ],
            base_path: env::current_dir().expect("Failed to get current directory"),            
        }
    }
}


// Default value for diagram direction
fn default_diagram_direction() -> String {
    "TD".to_string()  // Default to top-down layout
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            max_width: 1200,
            custom_css: None,
            diagram_direction: default_diagram_direction(),
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

    pub fn get_external_folders(&self) -> Vec<PathBuf> {
        self.paths.external_folders.iter()
        .map(|folder| {
            let path = self.paths.base_path.join(folder.clone());
            path.canonicalize().expect(&format!("FATAL: Failed to resolve external folder {:?}", path)).clone()
        })
        .collect::<Vec<PathBuf>>()
    }   
    pub fn get_specification_folder(&self) -> std::path::PathBuf {
        let path = self.paths.base_path.join(self.paths.specifications_folder.clone());
        path.canonicalize().expect(&format!("FATAL: Failed to resolve external folder {:?}", path)).clone()
    }   

    pub fn get_output_folder(&self) -> std::path::PathBuf {
        let path = self.paths.base_path.join(self.paths.output_folder.clone());
        path.canonicalize().expect(&format!("FATAL: Failed to resolve external folder {:?}", path)).clone()
    }   
    
    /// Builds a GlobSet from the excluded filename patterns
    pub fn get_excluded_filename_patterns_glob_set(&self) -> GlobSet {
        let mut builder = GlobSetBuilder::new();
        for pattern in &self.paths.excluded_filename_patterns {
            if let Ok(glob) = Glob::new(pattern.as_str()) {
                builder.add(glob);
            } else {
                warn!("Invalid glob pattern: {}", pattern);
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
            "reqflow.yml",
            "reqflow.yaml",
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
        let mut final_config = config.unwrap_or_else(|| {
            warn!("No configuration file found, using defaults");
            Self::default()
        });
        
        // Get the list of files generated by ReqFlow that should be excluded
        let always_excluded = Self::get_reqflow_generated_files();
        
        // Add the patterns if they're not already in the list
        for pattern in &always_excluded {
            if !final_config.paths.excluded_filename_patterns.contains(&pattern.to_string()) {
                final_config.paths.excluded_filename_patterns.push(pattern.to_string());
                debug!("Added auto-generated file pattern {} to excluded_filename_patterns", pattern);
            }
        }
        
        // No path normalization needed - we handle exact folder names in the validation logic
        
        final_config
    }
    
    /// Get the list of files generated by ReqFlow that should never be processed as requirements
    /// This list should be updated whenever a new feature is added that generates files
    pub fn get_reqflow_generated_files() -> Vec<String> {
        vec![
            "**/TraceabilityMatrix.md".to_string(),   // Matrix generated by ReqFlow
            "**/index.md".to_string(),   // Index generated by Reqflow           
            // Add more patterns here as new features are added that generate files
        ]
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
    <!-- Enhanced mermaid configuration for ReqFlow diagrams -->
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
