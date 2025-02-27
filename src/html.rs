use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};

use crate::config::Config;
use crate::error::ReqFlowError;

/// Convert markdown content to styled HTML with additional processing
pub fn convert_to_html(markdown_content: &str, title: &str) -> Result<String, ReqFlowError> {
    // Parse the markdown content to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    // These are important for ReqFlow diagrams and code blocks
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    
    let parser = Parser::new_ext(markdown_content, options);
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
#[allow(dead_code)]
pub fn process_mermaid_diagrams(html_content: &str) -> String {
    use regex::Regex;
    
    lazy_static::lazy_static! {
        static ref MERMAID_REGEX: Regex = Regex::new(r#"<pre><code class="language-mermaid">([\s\S]*?)</code></pre>"#).unwrap();
    }
    
    MERMAID_REGEX
        .replace_all(html_content, |caps: &regex::Captures| {
            let diagram_content = &caps[1];
            format!(
                r#"<div class="mermaid">{}</div>"#,
                diagram_content
            )
        })
        .to_string()
}