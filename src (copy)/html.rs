use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};

use crate::config::Config;
use crate::error::ReqFlowError;

/// Convert markdown content to styled HTML with additional processing
pub fn convert_to_html(markdown_content: &str, title: &str) -> Result<String, ReqFlowError> {
    // First, convert all Markdown links to use .html extension
    let markdown_content_with_html_links = convert_markdown_links_to_html(markdown_content);
    
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
    let html_with_mermaid = process_mermaid_diagrams(&html_with_anchors);
    
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

/// Process mermaid diagrams to ensure they render correctly in HTML
/// Also handles conversion of .md links to .html links within the diagrams
pub fn process_mermaid_diagrams(html_content: &str) -> String {
    use regex::Regex;
    
    lazy_static::lazy_static! {
        static ref MERMAID_REGEX: Regex = Regex::new(r#"<pre><code class="language-mermaid">([\s\S]*?)</code></pre>"#).unwrap();
        // Add regex to fix mermaid click links - convert .md to .html in click statements
        // This pattern captures parent directory references (../../) correctly
        static ref MERMAID_CLICK_REGEX: Regex = Regex::new(r#"(click\s+\S+\s+&quot;)([^&\#"]*)\.md((?:#[^&"]*)?&quot;)"#).unwrap();
    }
    
    // First extract the mermaid content
    let with_div = MERMAID_REGEX
        .replace_all(html_content, |caps: &regex::Captures| {
            let diagram_content = &caps[1];
            
            // Convert any .md links in the diagram to .html
            let fixed_content = MERMAID_CLICK_REGEX.replace_all(diagram_content, |caps: &regex::Captures| {
                let prefix = &caps[1];   // "click ID "
                let path = &caps[2];     // path part before .md
                let suffix = &caps[3];   // #anchor" part
                
                format!("{}{}.html{}", prefix, path, suffix)
            });
            
            format!(
                r#"<div class="mermaid">{}</div>"#,
                fixed_content
            )
        })
        .to_string();
    
    with_div
}

/// Convert all markdown links from .md to .html for HTML output
/// Pre-processes markdown content to convert all markdown links with .md extension to .html 
/// This is used to ensure all links in the generated HTML point to HTML files
fn convert_markdown_links_to_html(markdown_content: &str) -> String {
    use regex::Regex;
    
    lazy_static::lazy_static! {
        // Match markdown links with hash fragments: [text](url.md#fragment)
        static ref MD_LINK_WITH_HASH_REGEX: Regex = Regex::new(r"(\]\()([^#)]+)\.md(#[^)]+)(\))").unwrap();
        
        // Match markdown links: [text](url.md) or [text](path/to/url.md) - including with parent directory references (../../)
        // This pattern also handles links in relation sections like: * satisfiedBy: [link](path.md)
        static ref MD_LINK_REGEX: Regex = Regex::new(r"(\]\()([^#)]+)\.md(\))").unwrap();
        
        // Match markdown links to directories without extension: [text](path/to/dir)
        // Only if 'dir' doesn't contain a dot (not an extension)
        static ref DIR_LINK_REGEX: Regex = Regex::new(r"(\]\()([^)]+/)([^)/\.]+)(\))").unwrap();
        
        // Match markdown links with md/element format: [text](path/to/url.md/element)
        // Including paths with parent directory references (../../)
        static ref MD_ELEMENT_LINK_REGEX: Regex = Regex::new(r"(\]\()([^)]+)\.md/([^)]+)(\))").unwrap();
        
        // Match link text that refers to .md files: [path/to/file.md]
        static ref MD_LINK_TEXT_REGEX: Regex = Regex::new(r"\[([^]]+)\.md\]").unwrap();
    }
    
    // First convert markdown links with hash fragments: [text](url.md#fragment)
    let content = MD_LINK_WITH_HASH_REGEX.replace_all(markdown_content, |caps: &regex::Captures| {
        let prefix = &caps[1]; // ](
        let path = &caps[2];   // path/to/file part
        let fragment = &caps[3]; // #fragment
        let suffix = &caps[4]; // )
        
        // Replace .md extension with .html, preserving the hash fragment
        format!("{}{}.html{}{}", prefix, path, fragment, suffix)
    });
    
    // Then convert regular .md files to .html
    let content = MD_LINK_REGEX.replace_all(&content, |caps: &regex::Captures| {
        let prefix = &caps[1]; // ](
        let path = &caps[2];   // path/to/file part
        let suffix = &caps[3]; // )
        
        // Replace .md extension with .html
        format!("{}{}.html{}", prefix, path, suffix)
    });
    
    // Then convert .md/element to .html#element
    let content = MD_ELEMENT_LINK_REGEX.replace_all(&content, |caps: &regex::Captures| {
        let prefix = &caps[1]; // ](
        let path = &caps[2]; // path/to/url
        let element = &caps[3]; // element
        let suffix = &caps[4]; // )
        
        let element_anchor = element.replace(' ', "-").to_lowercase();
        format!("{}{}.html#{}{}", prefix, path, element_anchor, suffix)
    });
    
    // Finally add .html to directory links without extension
    let content = DIR_LINK_REGEX.replace_all(&content, |caps: &regex::Captures| {
        let prefix = &caps[1]; // ](
        let path = &caps[2]; // path/to/
        let dir = &caps[3]; // dir
        let suffix = &caps[4]; // )
        
        // Add .html to all directory links for consistency
        format!("{}{}{}.html{}", prefix, path, dir, suffix)
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
    
    #[test]
    fn test_convert_markdown_links_to_html_with_parent_paths() {
        // Test with relative paths containing parent directory references
        let markdown = r#"
- [Normal Link](file.md)
- [Parent Link](../parent.md)
- [Multiple Parents](../../grandparent.md)
- [Element in Parent](../other.md/Element)
- [Element with Hash](../something.md#header)
- * satisfiedBy: [DesignSpecifications/DirectMessages.md](DesignSpecifications/DirectMessages.md)
"#;
        
        let html = convert_markdown_links_to_html(markdown);
        println!("Converted HTML: {}", html);
        
        // Check that all links are converted properly
        assert!(!html.contains("file.md"));
        assert!(!html.contains("../parent.md"));
        assert!(!html.contains("../../grandparent.md"));
        assert!(!html.contains("../other.md/Element"));
        assert!(!html.contains("../something.md#header"));
        assert!(!html.contains("DesignSpecifications/DirectMessages.md"));
        
        // Check that they're converted to HTML
        assert!(html.contains("file.html"));
        assert!(html.contains("../parent.html"));
        assert!(html.contains("../../grandparent.html"));
        assert!(html.contains("../other.html#element"));
        assert!(html.contains("../something.html#header"));
        assert!(html.contains("DesignSpecifications/DirectMessages.html"));
    }
    
    #[test]
    fn test_process_mermaid_diagrams_with_parent_paths() {
        // Test with mermaid diagrams containing parent directory references
        let html_with_mermaid = r#"<pre><code class="language-mermaid">
graph TD;
    A[Start] --> B[End];
    click A &quot;../../path/to/file.md#section&quot;;
    click B &quot;../another/file.md&quot;;
</code></pre>"#;
        
        let processed = process_mermaid_diagrams(html_with_mermaid);
        
        // Check that all links are converted properly
        assert!(!processed.contains("../../path/to/file.md#section"));
        assert!(!processed.contains("../another/file.md"));
        
        // Check that they're converted to HTML
        assert!(processed.contains("../../path/to/file.html#section"));
        assert!(processed.contains("../another/file.html"));
    }
}