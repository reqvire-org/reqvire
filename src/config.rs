use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::path::Path;
use anyhow::Result;
use log::{info, warn};
use serde::{Deserialize, Serialize};

/// Configuration settings for the ReqFlow application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    
    #[serde(default)]
    pub paths: PathsConfig,
    
    #[serde(default)]
    pub style: StyleConfig,
    
    // These are needed for validation logic but not exposed in the config file
    #[serde(skip_serializing, skip_deserializing)]
    pub validation: ValidationConfig,
    
    // Linting configuration
    #[serde(skip_serializing, skip_deserializing)]
    pub linting: LintingConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    #[serde(default)]
    pub html_output: bool,
    pub verbose: bool,
    #[serde(default)]
    pub generate_diagrams: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathsConfig {
    pub specifications_folder: String,
    #[serde(alias = "design_specifications_folder_name")]
    pub design_specifications_folder: String,
    pub output_folder: String,
    #[serde(default)]
    pub external_folders: Vec<String>,
    #[serde(default)]
    pub excluded_filename_patterns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StyleConfig {
    pub theme: String,
    pub max_width: usize,
    pub custom_css: Option<String>,
    #[serde(default = "default_diagram_direction")]
    pub diagram_direction: String,
}

// The following structs are kept for backwards compatibility
// but are not exposed in the config file

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationConfig {
    pub validate_markdown: bool,
    pub validate_relations: bool,
    pub validate_all: bool,
    pub json_output: bool,
    pub generate_matrix: bool,  // Generate traceability matrix
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LintingConfig {
    pub lint: bool,                // Enable linting
    pub dry_run: bool,             // Preview changes without applying them
}

impl Default for LintingConfig {
    fn default() -> Self {
        Self {
            lint: false,
            dry_run: false,
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            html_output: false,
            verbose: false,
            generate_diagrams: false,
        }
    }
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self {
            specifications_folder: "specifications".to_string(),
            design_specifications_folder: "DesignSpecifications".to_string(),
            output_folder: "output".to_string(),
            external_folders: Vec::new(),
            excluded_filename_patterns: vec!["**/README*.md".to_string(), "**/index.md".to_string()],
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            validate_markdown: false,
            validate_relations: false,
            validate_all: false,
            json_output: false,
            generate_matrix: false,
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
            general: GeneralConfig::default(),
            paths: PathsConfig::default(),
            style: StyleConfig::default(),
            validation: ValidationConfig::default(),
            linting: LintingConfig::default(),
        }
    }
}

impl Config {
    
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
        
        for path_str in &config_paths {
            let path = Path::new(path_str);
            if path.exists() {
                match Self::from_file(path) {
                    Ok(config) => {
                        info!("Loaded configuration from {}", path_str);
                        return config;
                    },
                    Err(e) => {
                        warn!("Error loading configuration from {}: {}", path_str, e);
                    }
                }
            }
        }
        
        info!("No configuration file found, using defaults");
        Self::default()
    }
    
    /// Regular expression to match element headers (level 3)
    pub fn element_regex() -> &'static Regex {
        lazy_static! {
            static ref ELEMENT_REGEX: Regex = Regex::new(r"^###\s+(.+)").unwrap();
        }
        &ELEMENT_REGEX
    }
    
    /// Regular expression to match subsection headers (level 4)
    pub fn subsection_regex() -> &'static Regex {
        lazy_static! {
            static ref SUBSECTION_REGEX: Regex = Regex::new(r"^####\s+(.+)").unwrap();
        }
        &SUBSECTION_REGEX
    }

    /// Regular expression to match relation entries
    pub fn relation_regex() -> &'static Regex {
        lazy_static! {
            // Updated to handle ReqFlow relation format with various indentation styles and bullet formats
            // This handles both * and - bullets with either 0, 2 or 4 spaces of indentation
            static ref RELATION_REGEX: Regex = Regex::new(r"^\s*(?:\*|-)\s+(\w+):\s*(.+)").unwrap();
        }
        &RELATION_REGEX
    }

    /// Regular expression to check if a line is a valid markdown link
    #[allow(dead_code)]
    pub fn markdown_link_regex() -> &'static Regex {
        lazy_static! {
            static ref MARKDOWN_LINK_REGEX: Regex = Regex::new(r"\[.+?\]\(.+?\)").unwrap();
        }
        &MARKDOWN_LINK_REGEX
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