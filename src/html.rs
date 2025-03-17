use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};
use crate::utils;
use crate::config::Config;
use crate::error::ReqFlowError;
use std::path::PathBuf;
use lazy_static::lazy_static;
use regex::Regex;

/// Convert markdown content to styled HTML with additional processing
pub fn convert_to_html(
    file_path: &PathBuf,
    markdown_content: &str, 
    title: &str,
    specification_folder: &PathBuf,
    external_folders: &[PathBuf],   
) -> Result<String, ReqFlowError> {
    // First, convert all Markdown links to use .html extension
    let markdown_content_with_html_links = convert_markdown_links_to_html(file_path,markdown_content,specification_folder,external_folders);
    
    // Parse the markdown content to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    // These are important for ReqFlow diagrams and code blocks
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    
    let parser = Parser::new_ext(&markdown_content_with_html_links, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    // Process headers to add IDs for anchors
    let html_with_anchors = add_anchor_ids(&html_output);
    
    // Process mermaid diagrams for proper rendering
    let html_with_mermaid = process_mermaid_diagrams(file_path, &html_with_anchors, specification_folder, external_folders );
    
    // Insert into HTML template
    let html_document = Config::html_template()
        .replace("{title}", title)
        .replace("{styles}", Config::embedded_styles())
        .replace("{content}", &html_with_mermaid);
    
    Ok(html_document)
}

/// Add id attributes to headers for anchor links
fn add_anchor_ids(html_content: &str) -> String {
    use regex::Regex;
    
    lazy_static::lazy_static! {
        static ref HEADER_REGEX: Regex = Regex::new(r"<(h[1-3])>([^<]+)</h[1-3]>").unwrap();
    }
    
    HEADER_REGEX
        .replace_all(html_content, |caps: &regex::Captures| {
            let tag = &caps[1];
            let text = &caps[2];
            let id = text.trim().replace(' ', "-").to_lowercase();
            format!("<{} id=\"{}\">{}</{}>", tag, id, text, tag)
        })
        .to_string()
}

/// Process Mermaid diagrams to ensure links point to the correct `.html` files.
/// Uses `to_relative_identifier` to correctly resolve paths.
pub fn process_mermaid_diagrams(
    file_path: &PathBuf,
    html_content: &str,
    specification_folder: &PathBuf,
    external_folders: &[PathBuf],
) -> String {

    lazy_static! {
        static ref MERMAID_REGEX: Regex = Regex::new(r#"<pre><code class="language-mermaid">([\s\S]*?)</code></pre>"#).unwrap();
        
        static ref MERMAID_CLICK_REGEX: Regex = Regex::new(
            r#"(click\s+\S+\s+&quot;)([^&\#"]+\.[a-zA-Z0-9]+)((?:#[^&"]*)?&quot;)"#
        ).unwrap();
    }

    // Use the folder of `file_path` as the base for relative links.
    let file_dir = file_path.parent().unwrap_or(file_path).to_path_buf();


    MERMAID_REGEX.replace_all(html_content, |caps: &regex::Captures| {
        let diagram_content = &caps[1];        // Convert Markdown links inside Mermaid diagrams to correct HTML paths
        
        let fixed_content = MERMAID_CLICK_REGEX.replace_all(diagram_content, |caps: &regex::Captures| {
        let prefix = &caps[1];   // "click ID "
        let full_filename = caps[2].to_string(); // Extracted full filename with extension
        let fragment = &caps[3]; // Optional hash fragment (e.g., `#section"`)


        let mut modified_filename = full_filename.to_string();

        if let Some(spec_name) = specification_folder.file_name().and_then(|s| s.to_str()) {
                // Construct the needle as "/SPECIFICATION_FOLDER_NAME/"
                let needle = format!("/{}/", spec_name);
                // And the replacement is simply "/"
                let replacement = format!("/");
                // Perform a simple string replacement.
                modified_filename = modified_filename.replace(&needle, &replacement);       
        }     
        for ext in external_folders {
            if let Some(ext_name) = ext.file_name().and_then(|s| s.to_str()) {
                // Construct the needle as "../EXTERNAL_FOLDER_NAME/"
                let needle = format!("../{}/", ext_name);
                // And the replacement is simply "EXTERNAL_FOLDER_NAME/"
                let replacement = format!("{}/", ext_name);
                // Perform a simple string replacement.
                modified_filename = modified_filename.replace(&needle, &replacement);
            }
        }

        modified_filename=modified_filename.replace(".md", ".html");

        format!("{}{}{}", prefix,  modified_filename, fragment)
      });

      format!(r#"<div class="mermaid">{}</div>"#, fixed_content)
      }).to_string()
}

/// Convert all markdown links from .md to .html for HTML output
/// Pre-processes markdown content to convert all markdown links with .md extension to .html 
/// This is used to ensure all links in the generated HTML point to HTML files
fn convert_markdown_links_to_html(
    file_path: &PathBuf,
    markdown_content: &str, 
    specification_folder: &PathBuf,
    external_folders: &[PathBuf],   
) -> String {
    use regex::Regex;
    
    lazy_static::lazy_static! {
        // Match markdown links with hash fragments: [text](url.md#fragment)
        static ref MD_LINK_WITH_HASH_REGEX: Regex = Regex::new(r"(\]\()([^#)]+)\.md(#[^)]+)(\))").unwrap();
        
        // Match markdown links: [text](url.md) or [text](path/to/url.md) - including with parent directory references (../../)
        // This pattern also handles links in relation sections like: * satisfiedBy: [link](path.md)
        static ref MD_LINK_REGEX: Regex = Regex::new(r"(\]\()([^#)]+)\.md(\))").unwrap();
        
        // Match link text that refers to .md files: [path/to/file.md]
        static ref MD_LINK_TEXT_REGEX: Regex = Regex::new(r"\[([^]]+)\.md\]").unwrap();
    }

    // Helper function: If a link contains a pattern like "../SPECIFICATION_FOLDER_NAME/",
    // remove the preceding "SPECIFICATION_FOLDER_NAME" so that it becomes "../".       
    // If a link contains a pattern like "../EXTERNAL_FOLDER_NAME/",
    // remove the preceding "../" so that it becomes "/EXTERNAL_FOLDER_NAME/".
    fn fix_link_path(link: &str, specification_folder: &PathBuf, external_folders: &[std::path::PathBuf]) -> String {
        let mut fixed = link.to_string();
        
        if let Some(spec_name) = specification_folder.file_name().and_then(|s| s.to_str()) {
            let needle = format!("/{}/", spec_name);
            // Define the replacement (i.e. without the preceding "../")
            let replacement = "/";
            // A simple string-level replacement.
            fixed = fixed.replace(&needle, &replacement);
        }else{
                  
            for ext in external_folders {
                if let Some(ext_name) = ext.file_name().and_then(|s| s.to_str()) {
                    // Define the needle we're looking for:
                    let needle = format!("../{}/", ext_name);
                    // Define the replacement (i.e. without the preceding "../")
                    let replacement = format!("/{}/", ext_name);
                    // A simple string-level replacement.
                    fixed = fixed.replace(&needle, &replacement);
                }
            }
        }
        fixed
    }
    
    
    // Process links with hash fragments.
    let content = MD_LINK_WITH_HASH_REGEX.replace_all(markdown_content, |caps: &regex::Captures| {
        let prefix   = &caps[1];  // the literal "](" starting the URL
        let link     = &caps[2];  // the link path (without .md)
        let fragment = &caps[3];  // the hash fragment, e.g. "#section"
        let suffix   = &caps[4];  // the closing ")"
        
        let fixed_link = fix_link_path(link, specification_folder, external_folders);
        format!("{}{}.html{}{}", prefix, fixed_link, fragment, suffix)
    });
    
    // Process links without hash fragments.
    let content = MD_LINK_REGEX.replace_all(&content, |caps: &regex::Captures| {
        let prefix = &caps[1];
        let link   = &caps[2];
        let suffix = &caps[3];
        
        let fixed_link = fix_link_path(link, specification_folder, external_folders);
        format!("{}{}.html{}", prefix, fixed_link, suffix)
    });

    // Convert link text that contains .md to use .html instead
    // This is useful for links in relation sections where the display text should also be updated
    // Example: * satisfiedBy: [DesignSpecifications/DirectMessages.md](DesignSpecifications/DirectMessages.md)
    let content = MD_LINK_TEXT_REGEX.replace_all(&content, |caps: &regex::Captures| {
        let path = &caps[1]; // path/to/file part without the .md
        
        // Replace .md with .html in the link text
        format!("[{}.html]", path)
    });

  
    content.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_convert_markdown_links_to_html_with_parent_paths() {
        // Test with relative paths containing parent directory references.
        let markdown = r#"
- [Normal Link](file.md)
- [Parent Link](../parent.md)
- [Multiple Parents](../../grandparent.md)
- [Element in Parent](../other.md#element)
- [Element with Hash](../something.md#header)
- * satisfiedBy: [DesignSpecifications/DirectMessages.md](DesignSpecifications/DirectMessages.md)
"#;
        // Dummy file path.
        let file_path = &PathBuf::from("dummy.md");
        // Use "DesignSpecifications" as the specification folder.
        let specification_folder = &PathBuf::from("./");
        // Provide an external folder that does not interfere with these links.
        let external_folders: Vec<PathBuf> = vec![PathBuf::from("external")];

        let html = convert_markdown_links_to_html(file_path, markdown, specification_folder, &external_folders);
        println!("Converted HTML: {}", html);
        
        // Check that no link still contains ".md".
        assert!(!html.contains("file.md"));
        assert!(!html.contains("../parent.md"));
        assert!(!html.contains("../../grandparent.md"));
        assert!(!html.contains("../other.md#element"));
        assert!(!html.contains("../something.md#header"));
        assert!(!html.contains("DesignSpecifications/DirectMessages.md"));
        
        // Check that links are converted to .html.
        assert!(html.contains("file.html"));
        assert!(html.contains("../parent.html"));
        assert!(html.contains("../../grandparent.html"));
        assert!(html.contains("../other.html#element"));
        assert!(html.contains("../something.html#header"));
        // Specification folder links remain intact.
        assert!(html.contains("DesignSpecifications/DirectMessages.html"));
    }
    
    #[test]
    fn test_process_mermaid_diagrams_with_parent_paths() {
        // Test with mermaid diagrams containing parent directory references.
        let html_with_mermaid = r#"<pre><code class="language-mermaid">
graph TD;
    A[Start] --> B[End];
    click A &quot;../../path/to/file.md#section&quot;;
    click B &quot;../another/file.md&quot;;
</code></pre>"#;
        
        let file_path = &PathBuf::from("dummy.md");
        // For testing, use a specification folder and external folder that do not affect these links.
        let specification_folder = &PathBuf::from("DesignSpecifications");
        let external_folders: Vec<PathBuf> = vec![PathBuf::from("external")];

        let processed = process_mermaid_diagrams(file_path, html_with_mermaid, specification_folder, &external_folders);
        
        // Check that original .md links are not present.
        assert!(!processed.contains("../../path/to/file.md#section"));
        assert!(!processed.contains("../another/file.md"));
        
        // Check that they're converted to .html.
        assert!(processed.contains("../../path/to/file.html#section"));
        assert!(processed.contains("../another/file.html"));
    }
}
