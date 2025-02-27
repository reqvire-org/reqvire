use lazy_static::lazy_static;
use regex::Regex;

/// Configuration settings for the ReqFlow application
pub struct Config;

impl Config {
    /// Regular expression to match element headers (level 3)
    pub fn element_regex() -> &'static Regex {
        lazy_static! {
            static ref ELEMENT_REGEX: Regex = Regex::new(r"### (.+)").unwrap();
        }
        &ELEMENT_REGEX
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