use std::path::{Path, PathBuf};
use crate::config::Config;
use regex::Regex;

/// Normalize an identifier based on the specifications folder and external folders.
/// If an identifier is a **fragment only**, it is relative to the current document.
pub fn normalize_identifier(config: &Config, current_file: &str, raw_identifier: &str) -> String {
    let fragment_re = Regex::new(r"^#(.+)$").unwrap();
    let markdown_link_re = Regex::new(r"^\[.*\]\((.+)\)$").unwrap();

    let mut identifier = raw_identifier.trim().to_string();

    // Case 1: If it's a Markdown link, extract the identifier
    if let Some(caps) = markdown_link_re.captures(&identifier) {
        identifier = caps.get(1).unwrap().as_str().to_string();
    }

    // Case 2: If the identifier is a fragment only (e.g., `#element-name`), prepend current file
    if let Some(caps) = fragment_re.captures(&identifier) {
        let element_name = caps.get(1).unwrap().as_str();
        return format!("{}#{}", normalize_path(config, current_file), normalize_element_name(element_name));
    }

    // Case 3: If the identifier contains `#`, split file and fragment
    let (file_part, fragment_part) = match identifier.split_once('#') {
        Some((file, fragment)) => (file.trim(), Some(fragment.trim())),
        None => (identifier.as_str(), None),
    };

    let normalized_path = normalize_path(config, file_part);
    if let Some(fragment) = fragment_part {
        format!("{}#{}", normalized_path, normalize_element_name(fragment))
    } else {
        normalized_path
    }
}

/// Normalize a file path according to the `specifications_folder` and `external_folders`
pub fn normalize_path(config: &Config, path: &str) -> String {
    let path = Path::new(path);
    
    // If already absolute, return as is
    if path.is_absolute() {
        return path.to_string_lossy().to_string();
    }

    // Try to resolve within the specifications folder
    let spec_path = Path::new(&config.paths.specifications_folder).join(path);
    if spec_path.exists() {
        return spec_path.to_string_lossy().to_string();
    }

    // Try to resolve within external folders
    for external_folder in &config.paths.external_folders {
        let external_path = Path::new(external_folder).join(path);
        if external_path.exists() {
            return external_path.to_string_lossy().to_string();
        }
    }

    // Default: Treat as a relative path
    let relative_path = Path::new(&config.paths.base_path).join(path);
    relative_path.to_string_lossy().to_string()
}

/// Normalize an element name to GitHub-style anchor links
pub fn normalize_element_name(element_name: &str) -> String {
    element_name
        .trim()
        .to_lowercase()
        .replace(' ', "-")
        .replace(&['(', ')', '.', ',', ':', ';', '\''][..], "")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, PathsConfig};

    fn mock_config() -> Config {
        Config {
            paths: PathsConfig {
                specifications_folder: "/specifications".to_string(),
                output_folder: "/output".to_string(),
                external_folders: vec!["/external1".to_string(), "/external2".to_string()],
                excluded_filename_patterns: vec![],
                base_path: PathBuf::from("/current"),
            },
            ..Default::default()
        }
    }

    #[test]
    fn test_normalize_identifier() {
        let config = mock_config();
        assert_eq!(
            normalize_identifier(&config, "docs/doc1.md", "#My Element"),
            "/current/docs/doc1.md#my-element"
        );
        assert_eq!(
            normalize_identifier(&config, "docs/doc1.md", "docs/doc2.md#Another Element"),
            "/current/docs/doc2.md#another-element"
        );
    }

    #[test]
    fn test_normalize_path() {
        let config = mock_config();
        assert_eq!(normalize_path(&config, "docs/spec.md"), "/current/docs/spec.md");
        assert_eq!(normalize_path(&config, "/absolute/path.md"), "/absolute/path.md");
    }
}

