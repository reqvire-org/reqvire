use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};
use crate::error::ReqvireError;
use std::path::PathBuf;
use lazy_static::lazy_static;
use regex::{Regex, Captures};
use std::path::Path;



/// Embedded CSS styles for HTML output
pub const EMBEDDED_STYLES: &str = r#"
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
"#;

/// HTML template for generated pages
pub const HTML_TEMPLATE: &str = r#"
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
"#;

/// Convert markdown content to styled HTML with additional processing
pub fn convert_to_html(
    file_path: &PathBuf,
    markdown_content: &str, 
    title: &str,
    base_folder: &PathBuf
) -> Result<String, ReqvireError> {
    // 1. Extract Mermaid blocks before link conversion
    let (markdown_without_mermaid, mermaid_blocks) = extract_mermaid_blocks(markdown_content);

    // 2. Convert .md links to .html — safely
    let markdown_html_ready = convert_markdown_links_to_html(file_path, &markdown_without_mermaid, base_folder);

    // 3. Restore Mermaid blocks (untouched by md → html rewrite)
    let markdown_final = restore_mermaid_blocks(&markdown_html_ready, &mermaid_blocks);

    // 4. Convert Markdown to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(&markdown_final, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // 5. Process anchor IDs, Mermaid blocks
    let html_with_anchors = add_anchor_ids(&html_output);
    let html_with_mermaid = process_mermaid_diagrams(file_path, &html_with_anchors);

    // 6. Final output
    let html_document = HTML_TEMPLATE
        .replace("{title}", title)
        .replace("{styles}", EMBEDDED_STYLES)
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

/// Rewrite every `<pre><code class="language-mermaid">…</code></pre>`
/// into `<div class="mermaid">…</div>` and convert relative links from .md to .html
/// GitHub blob links are preserved as-is (keeping the .md extension)
pub fn process_mermaid_diagrams(
    _file_path: &Path,     // Used to determine if we're in a specifications folder
    html_content: &str,    // the rendered HTML
) -> String {
    lazy_static! {
        /// 1) Find each mermaid code‐block
        static ref MERMAID_BLOCK: Regex = Regex::new(
            r#"<pre><code class="language-mermaid">([\s\S]*?)</code></pre>"#
        ).unwrap();
        
        /// 2) Find all .md links, we'll filter GitHub links in the replacement code
        static ref MD_LINK: Regex = Regex::new(
            // Matches "click X &quot;path/file.md#fragment&quot;"
            r#"(click\s+\S+\s+&quot;)([^&"]*?)\.md(#[^&"]*)?(&quot;)"#
        ).unwrap();
    }
    
    // Process mermaid blocks
    let mermaid_processed = MERMAID_BLOCK
        .replace_all(html_content, |caps: &regex::Captures| {
            let inner = &caps[1];
            
            // Handle .md links, but preserve GitHub blob links
            let fixed = MD_LINK.replace_all(inner, |c: &regex::Captures| {
                let prefix = &c[1];          // click X &quot;
                let path = &c[2];            // path/to/file
                let anchor = c.get(3).map_or("", |m| m.as_str());
                let suffix = &c[4];          // &quot;
                
                // Check if this is a GitHub URL - if so, preserve the .md extension
                if path.starts_with("https://github.com") {
                    format!("{}{}.md{}{}", prefix, path, anchor, suffix)
                } else {
                    // Otherwise convert to .html
                    format!("{}{}.html{}{}", prefix, path, anchor, suffix)
                }
            });

            // swap <pre><code>…</code></pre> → <div class="mermaid">…</div>
            format!(r#"<div class="mermaid">{}</div>"#, fixed)
        })
        .to_string();
    
    mermaid_processed
}



use std::collections::HashMap;

/// Extracts Mermaid blocks and replaces them with placeholders
fn extract_mermaid_blocks(markdown: &str) -> (String, HashMap<String, String>) {
    lazy_static! {
        static ref MERMAID_BLOCK: Regex = Regex::new(
            r"(?s)(?P<full>```mermaid\s+(?P<code>.*?)```)"
        ).unwrap();
    }

    let mut map = HashMap::new();
    let mut counter = 0;
    let result = MERMAID_BLOCK.replace_all(markdown, |caps: &Captures| {
        let full_block = &caps["full"];
        let placeholder = format!("{{{{MERMAID_BLOCK_{}}}}}", counter);
        map.insert(placeholder.clone(), full_block.to_string());
        counter += 1;
        placeholder
    });

    (result.into_owned(), map)
}

/// Replaces placeholders back with the original Mermaid blocks
fn restore_mermaid_blocks(content: &str, blocks: &HashMap<String, String>) -> String {
    let mut result = content.to_string();
    for (key, value) in blocks {
        result = result.replace(key, value);
    }
    result
}


/// Convert all markdown links from .md to .html for HTML output
/// Pre-processes markdown content to convert all markdown links with .md extension to .html 
/// This is used to ensure all links in the generated HTML point to HTML files
fn convert_markdown_links_to_html(
    _file_path: &PathBuf,
    markdown_content: &str,
    _base_folder: &PathBuf
) -> String {
    lazy_static! {
        // 1) [text](../path/to/file.md#fragment)
        static ref MD_LINK_WITH_HASH_REGEX: Regex =
            Regex::new(r"(\]\()((?:\.\./)*)([^#)]+)\.md(#[^)]+)(\))").unwrap();

        // 2) [text](../path/to/file.md)
        static ref MD_LINK_REGEX: Regex =
            Regex::new(r"(\]\()((?:\.\./)*)([^#)]+)\.md(\))").unwrap();

        // 3) bare link text [foo.md]
        static ref MD_LINK_TEXT_REGEX: Regex =
            Regex::new(r"\[([^]]+)\.md\]").unwrap();
    }

    // 1) Links with a fragment
    let content = MD_LINK_WITH_HASH_REGEX.replace_all(markdown_content, |caps: &Captures| {
        let before   = &caps[1]; // "]("
        let parents  = &caps[2]; // e.g. "../../"
        let path     = &caps[3]; // "path/to/file"
        let fragment = &caps[4]; // "#section"
        let close    = &caps[5]; // ")"

        // apply your existing folder‑name rewrites only to the path portion
        format!("{}{}{}.html{}{}", before, parents, path, fragment, close)
    });

    // 2) Links without a fragment
    let content = MD_LINK_REGEX.replace_all(&content, |caps: &Captures| {
        let before  = &caps[1]; // "]("
        let parents = &caps[2]; // "../"*
        let path    = &caps[3]; // "foo/bar"
        let close   = &caps[4]; // ")"

        format!("{}{}{}.html{}", before, parents, path, close)
    });

    // 3) Bare link text (no URL): [foo.md] → [foo.html]
    let content = MD_LINK_TEXT_REGEX.replace_all(&content, |caps: &Captures| {
        let text = &caps[1];
        format!("[{}.html]", text)
    });

    content.into_owned()
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
- [MarkdownFile](../something.md)
- [File](../something.rs)
- * satisfiedBy: [DesignSpecifications/DirectMessages.md](DesignSpecifications/DirectMessages.md)
"#;
        // Dummy file path.
        let file_path = &PathBuf::from("dummy.md");
        // Use "DesignSpecifications" as the specification folder.
        let base_folder = &PathBuf::from("./");

        let html = convert_markdown_links_to_html(file_path, markdown, base_folder);
        println!("Converted HTML: {}", html);
        
        // Check that no link still contains ".md".
        assert!(!html.contains("file.md"));
        assert!(!html.contains("../parent.md"));
        assert!(!html.contains("../../grandparent.md"));
        assert!(!html.contains("../other.md#element"));
        assert!(!html.contains("../something.md#header"));
        assert!(!html.contains("../something.md"));                    
        assert!(!html.contains("DesignSpecifications/DirectMessages.md"));
        
        // Check that links are converted to .html.
        assert!(html.contains("file.html"));
        assert!(html.contains("../parent.html"));
        assert!(html.contains("../../grandparent.html"));
        assert!(html.contains("../other.html#element"));
        assert!(html.contains("../something.html#header"));
        assert!(html.contains("../something.html"));
        assert!(html.contains("../something.rs"));         
        // Specification folder links remain intact.
        assert!(html.contains("DesignSpecifications/DirectMessages.html"));
    }
    
    #[test]    
    fn test_mermaid_click_links_preserve_rs_files() {
        let html_with_mermaid = r#"<pre><code class="language-mermaid">
    graph TD;
        click A &quot;https://github.com/user/repo/blob/main/specs/Reqs.md#id1&quot;;
        click B &quot;https://github.com/user/repo/blob/main/src/main.rs&quot;;
    </code></pre>"#;

        let file_path = PathBuf::from("specs/diagrams/example.md");
        let processed = process_mermaid_diagrams(&file_path, html_with_mermaid);

        // GitHub blob links are preserved with .md extension
        assert!(processed.contains("https://github.com/user/repo/blob/main/specs/Reqs.md#id1"));
        // .rs links remain untouched
        assert!(processed.contains("https://github.com/user/repo/blob/main/src/main.rs"));
    }
    
    #[test]
    fn test_direct_markdown_links_in_mermaid() {
        let html_with_mermaid = r#"<pre><code class="language-mermaid">
    graph TD;
        click A &quot;specs/Reqs.md#id1&quot;;
        click B &quot;../../src/main.rs&quot;;
    </code></pre>"#;

        let file_path = PathBuf::from("specs/diagrams/example.md");
        let processed = process_mermaid_diagrams(&file_path, html_with_mermaid);

        // Regular .md links are converted to .html
        assert!(processed.contains("specs/Reqs.html#id1"));
        // Other files remain untouched
        assert!(processed.contains("../../src/main.rs"));
        // original .md link is gone
        assert!(!processed.contains("specs/Reqs.md#id1"));
    }
    
    #[test]
    fn test_parent_directory_links_in_mermaid() {
        let html_with_mermaid = r#"<pre><code class="language-mermaid">
    graph TD;
        click A &quot;../parent/Reqs.md#id1&quot;;
        click B &quot;../../grandparent/Reqs.md#id1&quot;;
        click B &quot;../../grandparent/Reqs.rs&quot;;        
    </code></pre>"#;

        let file_path = PathBuf::from("specs/diagrams/example.md");
        let processed = process_mermaid_diagrams(&file_path, html_with_mermaid);

        // Parent directories are preserved
        assert!(processed.contains("../parent/Reqs.html#id1"));
        assert!(processed.contains("../../grandparent/Reqs.html#id1"));
        assert!(processed.contains("../../grandparent/Reqs.rs"));        
                
        // Original .md links are gone
        assert!(!processed.contains("../parent/Reqs.md#id1"));
        assert!(!processed.contains("../../grandparent/Reqs.md#id1"));

    }
    
    #[test]
    fn test_mermaid_links_without_fragments() {
        let html_with_mermaid = r#"<pre><code class="language-mermaid">
    graph TD;
        click A &quot;specs/Reqs.md&quot;;
        click B &quot;../parent/Reqs.md&quot;;
        click C &quot;https://github.com/user/repo/blob/main/specs/Reqs.md&quot;;
    </code></pre>"#;

        let file_path = PathBuf::from("specs/diagrams/example.md");
        let processed = process_mermaid_diagrams(&file_path, html_with_mermaid);

        // Regular .md links are converted to .html
        assert!(processed.contains("specs/Reqs.html"));
        assert!(processed.contains("../parent/Reqs.html"));
        
        // GitHub blob links are preserved with the .md extension
        assert!(processed.contains("https://github.com/user/repo/blob/main/specs/Reqs.md"));
        
        // Original regular .md links are gone
        assert!(!processed.contains("click A &quot;specs/Reqs.md"));
        assert!(!processed.contains("click B &quot;../parent/Reqs.md"));
    }
}
