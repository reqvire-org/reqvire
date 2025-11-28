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
:root {
    /* Primary Colors */
    --color-primary: #3F51B5;           /* Indigo - core branding */
    --color-primary-hover: #7986CB;     /* Much lighter Indigo for hover - more visible */
    --color-primary-active: #303F9F;    /* Darker Indigo for active */

    /* Element Type Colors */
    --color-requirement: #673AB7;       /* Deep Purple - core requirements */
    --color-verification: #4CAF50;      /* Emerald Green - validated/completed */
    --color-other: #9E9E9E;             /* Cool Gray - other element types */

    /* Status Colors */
    --color-verified: #4CAF50;          /* Forest Green - verified/passing */
    --color-pending: #FFB74D;           /* Amber - pending/warning */
    --color-error: #F44336;             /* Red - error/failed */

    /* Interactive Colors */
    --color-highlight: #FFAB91;         /* Peach - hover highlight */
    --color-link: #212121;              /* Black - link color */
    --color-link-hover: #3F51B5;        /* Indigo (same as nav bar) - link hover */

    /* Text Colors */
    --color-text-primary: #212121;      /* Primary text */
    --color-text-secondary: #424242;    /* Body text */
    --color-text-muted: #757575;        /* De-emphasized text */
    --color-background: #FAFAFA;        /* Off-white background */
    --color-border: #EEEEEE;            /* Very light gray borders */
}
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
    line-height: 1.6;
    margin: 0;
    padding: 0;
    background-color: #FAFAFA;
    color: var(--color-text-secondary);
}
.reqvire-nav {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: 50px;
    background-color: var(--color-primary);
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    z-index: 1000;
    display: flex;
    align-items: center;
    padding: 0 20px;
}
.reqvire-nav a {
    color: #ffffff;
    text-decoration: none;
    padding: 10px 20px;
    margin-right: 5px;
    border-radius: 3px;
    transition: background-color 0.2s, text-decoration 0.2s;
}
.reqvire-nav a.nav-logo {
    display: flex;
    align-items: center;
    padding: 5px 15px 5px 0;
}
.reqvire-nav a.nav-logo img {
    vertical-align: middle;
}
.reqvire-nav a:hover {
    background-color: var(--color-primary-hover);
    text-decoration: underline;
    color: #ffffff;
}
.reqvire-nav a:active,
.reqvire-nav a.active {
    background-color: var(--color-primary-active);
    text-decoration: underline;
    color: #ffffff;
}
.reqvire-nav-spacer {
    height: 50px;
}
.container {
    max-width: 95%;
    margin: 0 auto;
    padding: 20px;
}
.content {
    background-color: #fff;
    padding: 30px;
    border-radius: 4px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.08);
    border: 1px solid #E0E0E0;
}
h1 {
    color: var(--color-text-primary);
    border-bottom: 2px solid #EEEEEE;
    padding-bottom: 10px;
    margin-top: 0;
}
h2 {
    color: var(--color-text-primary);
    border-bottom: 1px solid #EEEEEE;
    padding-bottom: 5px;
}
h3 {
    color: var(--color-text-primary);
    margin-top: 25px;
}
h4 {
    color: var(--color-text-secondary);
    font-weight: 600;
}
a {
    color: var(--color-primary);
    text-decoration: none;
    font-weight: 500;
    transition: text-decoration 0.15s ease;
}
a:hover {
    text-decoration: underline;
}
a:focus {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
}
table {
    border-collapse: collapse;
    width: 100%;
    margin: 20px 0;
}
table, th, td {
    border: 1px solid #EEEEEE;
}
th, td {
    padding: 8px 12px;
    text-align: left;
}
th {
    background-color: #F5F5F5;
    color: #212121;
    font-weight: 600;
}
tr:hover td {
    background-color: #FFF8E1;
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
    color: var(--color-text-muted);
    border-left: 4px solid var(--color-primary);
}
.mermaid {
    margin: 20px 0;
    text-align: center;
    height: calc(100vh - 150px);
    width: 100%;
    overflow: hidden;
    position: relative; /* Required for absolutely-positioned navigation buttons */
    border: 1px solid #EEEEEE;
    border-radius: 3px;
    background-color: #FAFAFA;
    display: flex;
    align-items: flex-start;
    justify-content: center;
}
.mermaid svg {
    width: 100%;
    height: 100%;
    max-width: 100%;
    max-height: 100%;
    position: relative;
    z-index: 1;
}
.diagram-nav-buttons {
    position: absolute;
    top: 5px;
    left: 5px;
    z-index: 999; /* higher than SVG (1) but lower than header nav (1000) */
    display: flex;
    flex-direction: column;
    gap: 2px;
    background-color: rgba(255, 255, 255, 0.85);
    padding: 3px;
    border-radius: 3px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.15);
}
.diagram-nav-row {
    display: flex;
    gap: 2px;
    justify-content: center;
}
.diagram-nav-btn {
    width: 22px;
    height: 22px;
    background-color: var(--color-primary);
    color: white;
    border: none;
    border-radius: 2px;
    cursor: pointer;
    font-size: 11px;
    display: flex;
    align-items: center;
    justify-content: center;
}
.diagram-nav-btn:hover {
    background-color: var(--color-primary-hover);
}
.diagram-nav-btn:active {
    background-color: var(--color-primary-active);
}
</style>
"#;

/// HTML template for model.html page with full-size diagram support
/// Loaded at compile time from templates/model.html
pub const HTML_TEMPLATE_MODEL: &str = include_str!("../templates/model.html");

/// HTML template for whole-model.html page with tighter edge detection for dense diagrams
/// Loaded at compile time from templates/whole-model.html
/// NOTE: Currently unused as whole-model generation is disabled, but preserved for future use
#[allow(dead_code)]
pub const HTML_TEMPLATE_WHOLE_MODEL: &str = include_str!("../templates/whole-model.html");

/// HTML template for generated pages
/// Loaded at compile time from templates/base.html
pub const HTML_TEMPLATE: &str = include_str!("../templates/base.html");

/// Convert markdown content to styled HTML with additional processing
pub fn convert_to_html(
    file_path: &PathBuf,
    markdown_content: &str,
    title: &str,
    base_folder: &PathBuf
) -> Result<String, ReqvireError> {
    // 1. Extract Mermaid, D3 tree, D3 Sankey, D3 Sunburst, and D3 Icicle blocks before link conversion
    let (markdown_without_mermaid, mermaid_blocks) = extract_mermaid_blocks(markdown_content);
    let (markdown_without_d3, d3_tree_blocks) = extract_d3_tree_blocks(&markdown_without_mermaid);
    let (markdown_without_sankey, d3_sankey_blocks) = extract_d3_sankey_blocks(&markdown_without_d3);
    let (markdown_without_sunburst, d3_sunburst_blocks) = extract_d3_sunburst_blocks(&markdown_without_sankey);
    let (markdown_without_icicle, d3_icicle_blocks) = extract_d3_icicle_blocks(&markdown_without_sunburst);

    // 2. Convert .md links to .html — safely
    let markdown_html_ready = convert_markdown_links_to_html(file_path, &markdown_without_icicle, base_folder);

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

    // 5. Process anchor IDs, Mermaid blocks, D3 tree blocks, Sankey blocks, Sunburst blocks, and Icicle blocks
    let html_with_anchors = add_anchor_ids(&html_output);
    let html_with_mermaid = process_mermaid_diagrams(file_path, &html_with_anchors);
    let html_with_d3 = restore_d3_tree_blocks(&html_with_mermaid, &d3_tree_blocks);
    let html_with_sankey = restore_d3_sankey_blocks(&html_with_d3, &d3_sankey_blocks);
    let html_with_sunburst = restore_d3_sunburst_blocks(&html_with_sankey, &d3_sunburst_blocks);
    let html_with_icicle = restore_d3_icicle_blocks(&html_with_sunburst, &d3_icicle_blocks);

    // 6. Calculate relative path prefix for navigation links
    let nav_prefix = calculate_nav_prefix(file_path, base_folder);

    // 7. Determine which template to use based on filename
    let filename = file_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    let template = if filename == "whole-model.md" {
        HTML_TEMPLATE_WHOLE_MODEL
    } else if filename == "model.md" {
        HTML_TEMPLATE_MODEL
    } else {
        HTML_TEMPLATE
    };

    // 8. Final output with relative navigation links
    let html_document = template
        .replace("{title}", title)
        .replace("{styles}", EMBEDDED_STYLES)
        .replace("{content}", &html_with_icicle)
        .replace("{nav_prefix}", &nav_prefix);

    Ok(html_document)
}

/// Calculate the relative path prefix needed for navigation links
/// based on the depth of the current file relative to base_folder
fn calculate_nav_prefix(file_path: &PathBuf, base_folder: &PathBuf) -> String {
    // Get relative path from base_folder
    let relative_path = match file_path.strip_prefix(base_folder) {
        Ok(rel) => rel,
        Err(_) => {
            // If strip_prefix fails, assume file is at root
            return String::new();
        }
    };

    // Count the number of directory components (excluding the filename)
    let depth = relative_path.components().count().saturating_sub(1);

    if depth == 0 {
        // File is at root level
        String::new()
    } else {
        // Need to go up 'depth' levels
        "../".repeat(depth)
    }
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

            // Decode HTML entities that pulldown_cmark added
            let decoded = inner
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&amp;", "&");

            // Handle .md links, but preserve GitHub blob links
            let fixed = MD_LINK.replace_all(&decoded, |c: &regex::Captures| {
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

/// Extracts D3 tree blocks and replaces them with placeholders
fn extract_d3_tree_blocks(markdown: &str) -> (String, HashMap<String, String>) {
    lazy_static! {
        static ref D3_TREE_BLOCK: Regex = Regex::new(
            r"(?s)```d3-tree\s*\n(?P<json>.*?)```"
        ).unwrap();
    }

    let mut map = HashMap::new();
    let mut counter = 0;
    let result = D3_TREE_BLOCK.replace_all(markdown, |caps: &Captures| {
        let json_data = caps["json"].trim();
        let placeholder = format!("{{{{D3_TREE_BLOCK_{}}}}}", counter);
        map.insert(placeholder.clone(), json_data.to_string());
        counter += 1;
        placeholder
    });

    (result.into_owned(), map)
}

/// Restores D3 tree placeholders with rendered HTML
fn restore_d3_tree_blocks(content: &str, blocks: &HashMap<String, String>) -> String {
    let mut result = content.to_string();
    for (placeholder, json_data) in blocks {
        let d3_html = generate_d3_tree_html(json_data);
        result = result.replace(placeholder, &d3_html);
    }
    result
}

/// Generate HTML for D3.js collapsible tree visualization
fn generate_d3_tree_html(json_data: &str) -> String {
    let unique_id = format!("d3tree_{:x}", json_data.as_ptr() as usize);

    format!(r##"
<div class="d3-tree-container" id="{id}">
    <div class="d3-tree-controls">
        <button onclick="expandAll_{id}()">Expand All</button>
        <button onclick="collapseAll_{id}()">Collapse All</button>
    </div>
    <svg class="d3-tree-svg"></svg>
</div>
<script src="https://d3js.org/d3.v7.min.js"></script>
<script>
(function() {{
    const data = {json};
    const container = document.getElementById("{id}");
    const svg = container.querySelector("svg");

    // Configuration
    const nodeWidth = 200;
    const nodeHeight = 28;
    const margin = {{top: 20, right: 120, bottom: 20, left: 60}};

    // Colors matching Reqvire theme
    const colors = {{
        "folder": "#9E9E9E",
        "file": "#B8860B",
        "user-requirement": "#7E57C2",
        "system-requirement": "#673AB7",
        "requirement": "#673AB7",
        "verification": "#4CAF50",
        "refinement": "#FF9800",
        "design-document": "#8D6E63",
        "element": "#424242",
        "attachment-element": "#FF9800",
        "attachment-file": "#607D8B"
    }};

    const icons = {{
        "folder": "📁",
        "file": "📄",
        "user-requirement": "👤",
        "system-requirement": "📐",
        "requirement": "📐",
        "verification": "✅",
        "refinement": "🔧",
        "design-document": "📝",
        "element": "◽",
        "attachment-element": "🔧",
        "attachment-file": "📎"
    }};

    // Get color for node type
    function getColor(type) {{
        if (colors[type]) return colors[type];
        return "#424242";
    }}

    // Get icon for node type
    function getIcon(type) {{
        if (icons[type]) return icons[type];
        return "◽";
    }}

    // Create hierarchy
    const root = d3.hierarchy(data);
    root.x0 = 0;
    root.y0 = 0;

    // Initially collapse all but first two levels
    root.descendants().forEach((d, i) => {{
        if (d.depth > 1) {{
            d._children = d.children;
            d.children = null;
        }}
    }});

    // Tree layout
    const tree = d3.tree().nodeSize([nodeHeight + 4, nodeWidth]);

    // Create SVG group
    const g = d3.select(svg)
        .attr("width", "100%")
        .attr("height", 600)
        .append("g")
        .attr("transform", `translate(${{margin.left}},${{margin.top}})`);

    // Links group (rendered first, below nodes)
    const linksGroup = g.append("g").attr("class", "links");
    const nodesGroup = g.append("g").attr("class", "nodes");

    function update(source) {{
        const duration = 300;
        const treeData = tree(root);
        const nodes = treeData.descendants();
        const links = treeData.links();

        // Normalize for fixed-depth
        nodes.forEach(d => {{ d.y = d.depth * nodeWidth; }});

        // Update SVG height based on tree
        const minX = d3.min(nodes, d => d.x);
        const maxX = d3.max(nodes, d => d.x);
        const height = maxX - minX + margin.top + margin.bottom + 50;
        d3.select(svg).attr("height", height);
        g.attr("transform", `translate(${{margin.left}},${{margin.top + Math.abs(minX) + 20}})`);

        // --- LINKS ---
        const link = linksGroup.selectAll("path.link")
            .data(links, d => d.target.data.name + d.target.depth);

        const linkEnter = link.enter()
            .append("path")
            .attr("class", "link")
            .attr("fill", "none")
            .attr("stroke", "#ccc")
            .attr("stroke-width", 1.5)
            .attr("d", d => {{
                const o = {{x: source.x0, y: source.y0}};
                return diagonal({{source: o, target: o}});
            }});

        link.merge(linkEnter)
            .transition()
            .duration(duration)
            .attr("d", diagonal);

        link.exit()
            .transition()
            .duration(duration)
            .attr("d", d => {{
                const o = {{x: source.x, y: source.y}};
                return diagonal({{source: o, target: o}});
            }})
            .remove();

        // --- NODES ---
        const node = nodesGroup.selectAll("g.node")
            .data(nodes, d => d.data.name + d.depth);

        const nodeEnter = node.enter()
            .append("g")
            .attr("class", "node")
            .attr("transform", d => `translate(${{source.y0}},${{source.x0}})`);

        // Check if node is an attachment (child metadata)
        function isMetaNode(type) {{
            return type && type.startsWith("attachment-");
        }}

        // Circle for expand/collapse indicator - smaller for attachments
        // Click on circle to toggle expand/collapse
        nodeEnter.append("circle")
            .attr("r", d => isMetaNode(d.data.type) ? 4 : 6)
            .attr("fill", d => getColor(d.data.type))
            .attr("stroke", "#fff")
            .attr("stroke-width", d => isMetaNode(d.data.type) ? 1 : 1.5)
            .style("cursor", d => d.children || d._children ? "pointer" : "default")
            .on("click", (event, d) => {{
                event.stopPropagation();
                if (d.children || d._children) {{
                    d.children = d.children ? null : d._children;
                    d._children = d._children ? null : d.children;
                    update(d);
                }}
            }});

        // Icon + Text - click to navigate or expand/collapse
        nodeEnter.append("text")
            .attr("dy", "0.35em")
            .attr("x", d => isMetaNode(d.data.type) ? 10 : 12)
            .attr("text-anchor", "start")
            .style("font-size", d => isMetaNode(d.data.type) ? "11px" : "13px")
            .style("font-style", d => isMetaNode(d.data.type) ? "italic" : "normal")
            .style("opacity", d => isMetaNode(d.data.type) ? 0.85 : 1)
            .style("font-family", "system-ui, sans-serif")
            .style("cursor", "pointer")
            .text(d => `${{getIcon(d.data.type)}} ${{d.data.name}}`)
            .on("click", (event, d) => {{
                event.stopPropagation();
                if (d.data.link) {{
                    // Navigate to link
                    window.location.href = d.data.link.replace(".md", ".html");
                }} else if (d.children || d._children) {{
                    // No link - toggle expand/collapse
                    d.children = d.children ? null : d._children;
                    d._children = d._children ? null : d.children;
                    update(d);
                }}
            }})
            .clone(true).lower()
            .attr("stroke", "white")
            .attr("stroke-width", 3);

        // Update positions
        const nodeUpdate = nodeEnter.merge(node);
        nodeUpdate.transition()
            .duration(duration)
            .attr("transform", d => `translate(${{d.y}},${{d.x}})`);

        // Update circles to show expand/collapse state
        nodeUpdate.select("circle")
            .attr("fill", d => getColor(d.data.type))
            .attr("stroke", d => d._children ? "#333" : "#fff");

        // Remove exiting nodes
        node.exit()
            .transition()
            .duration(duration)
            .attr("transform", d => `translate(${{source.y}},${{source.x}})`)
            .remove();

        // Store positions for next transition
        nodes.forEach(d => {{
            d.x0 = d.x;
            d.y0 = d.y;
        }});
    }}

    // Diagonal path generator
    function diagonal(d) {{
        return `M${{d.source.y}},${{d.source.x}}
                C${{(d.source.y + d.target.y) / 2}},${{d.source.x}}
                 ${{(d.source.y + d.target.y) / 2}},${{d.target.x}}
                 ${{d.target.y}},${{d.target.x}}`;
    }}

    // Recursive expand function that handles collapsed children
    function expandNode(d) {{
        if (d._children) {{
            d.children = d._children;
            d._children = null;
        }}
        if (d.children) {{
            d.children.forEach(expandNode);
        }}
    }}

    // Recursive collapse function
    function collapseNode(d) {{
        if (d.children) {{
            d.children.forEach(collapseNode);
            d._children = d.children;
            d.children = null;
        }}
    }}

    // Expand/Collapse all functions
    window.expandAll_{id} = function() {{
        expandNode(root);
        update(root);
    }};

    window.collapseAll_{id} = function() {{
        if (root.children) {{
            root.children.forEach(collapseNode);
        }}
        update(root);
    }};

    // Initial render
    update(root);
}})();
</script>
<style>
.d3-tree-container {{
    background: #FAFAFA;
    border: 1px solid #EEEEEE;
    border-radius: 4px;
    padding: 10px;
    overflow-x: auto;
}}
.d3-tree-controls {{
    margin-bottom: 10px;
}}
.d3-tree-controls button {{
    background: #3F51B5;
    color: white;
    border: none;
    padding: 6px 12px;
    margin-right: 8px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 13px;
}}
.d3-tree-controls button:hover {{
    background: #303F9F;
}}
.d3-tree-svg {{
    display: block;
}}
.node text {{
    fill: #424242;
}}
.node:hover text {{
    fill: #3F51B5;
}}
</style>
"##, id = unique_id, json = json_data)
}

/// Extracts D3 Sankey blocks and replaces them with placeholders
fn extract_d3_sankey_blocks(markdown: &str) -> (String, HashMap<String, String>) {
    lazy_static! {
        static ref D3_SANKEY_BLOCK: Regex = Regex::new(
            r"(?s)```d3-sankey\s*\n(?P<json>.*?)```"
        ).unwrap();
    }

    let mut map = HashMap::new();
    let mut counter = 0;
    let result = D3_SANKEY_BLOCK.replace_all(markdown, |caps: &Captures| {
        let json_data = caps["json"].trim();
        let placeholder = format!("{{{{D3_SANKEY_BLOCK_{}}}}}", counter);
        map.insert(placeholder.clone(), json_data.to_string());
        counter += 1;
        placeholder
    });

    (result.into_owned(), map)
}

/// Restores D3 Sankey placeholders with rendered HTML
fn restore_d3_sankey_blocks(content: &str, blocks: &HashMap<String, String>) -> String {
    let mut result = content.to_string();
    for (placeholder, json_data) in blocks {
        let sankey_html = generate_d3_sankey_html(json_data);
        result = result.replace(placeholder, &sankey_html);
    }
    result
}

/// Generate HTML for D3.js Sankey diagram visualization
/// Expected JSON format:
/// {
///   "nodes": [{"name": "Node1", "type": "requirement"}, ...],
///   "links": [{"source": 0, "target": 1, "value": 1}, ...]
/// }
fn generate_d3_sankey_html(json_data: &str) -> String {
    let unique_id = format!("d3sankey_{:x}", json_data.as_ptr() as usize);

    format!(r##"
<div class="d3-sankey-container" id="{id}">
    <svg class="d3-sankey-svg"></svg>
</div>
<script src="https://d3js.org/d3.v7.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/d3-sankey@0.12.3/dist/d3-sankey.min.js"></script>
<script>
(function() {{
    const data = {json};
    const container = document.getElementById("{id}");
    const svg = d3.select(container).select("svg");

    // Configuration
    const margin = {{top: 20, right: 20, bottom: 20, left: 20}};
    const width = container.clientWidth - margin.left - margin.right;
    const height = Math.max(400, data.nodes.length * 25);

    svg.attr("width", width + margin.left + margin.right)
       .attr("height", height + margin.top + margin.bottom);

    const g = svg.append("g")
        .attr("transform", `translate(${{margin.left}},${{margin.top}})`);

    // Colors matching Reqvire theme
    const colors = {{
        "user-requirement": "#7E57C2",
        "system-requirement": "#673AB7",
        "requirement": "#673AB7",
        "verification": "#4CAF50",
        "test-verification": "#4CAF50",
        "refinement": "#FF9800",
        "design-document": "#8D6E63",
        "default": "#9E9E9E"
    }};

    function getColor(type) {{
        return colors[type] || colors["default"];
    }}

    // Create sankey generator
    const sankey = d3.sankey()
        .nodeId(d => d.name)
        .nodeWidth(20)
        .nodePadding(15)
        .nodeAlign(d3.sankeyLeft)
        .extent([[0, 0], [width, height]]);

    // Generate sankey layout
    const {{nodes, links}} = sankey({{
        nodes: data.nodes.map(d => Object.assign({{}}, d)),
        links: data.links.map(d => Object.assign({{}}, d))
    }});

    // Draw links
    const link = g.append("g")
        .attr("fill", "none")
        .attr("stroke-opacity", 0.4)
        .selectAll("path")
        .data(links)
        .join("path")
        .attr("d", d3.sankeyLinkHorizontal())
        .attr("stroke", d => getColor(d.source.type))
        .attr("stroke-width", d => Math.max(1, d.width))
        .on("mouseover", function() {{
            d3.select(this).attr("stroke-opacity", 0.7);
        }})
        .on("mouseout", function() {{
            d3.select(this).attr("stroke-opacity", 0.4);
        }});

    link.append("title")
        .text(d => `${{d.source.name}} → ${{d.target.name}}`);

    // Draw nodes
    const node = g.append("g")
        .selectAll("g")
        .data(nodes)
        .join("g")
        .attr("transform", d => `translate(${{d.x0}},${{d.y0}})`);

    node.append("rect")
        .attr("height", d => d.y1 - d.y0)
        .attr("width", d => d.x1 - d.x0)
        .attr("fill", d => getColor(d.type))
        .attr("stroke", "#fff")
        .attr("stroke-width", 1)
        .style("cursor", d => d.link ? "pointer" : "default")
        .on("click", (event, d) => {{
            if (d.link) {{
                window.location.href = d.link.replace(".md", ".html");
            }}
        }});

    node.append("text")
        .attr("x", d => d.x0 < width / 2 ? (d.x1 - d.x0) + 6 : -6)
        .attr("y", d => (d.y1 - d.y0) / 2)
        .attr("dy", "0.35em")
        .attr("text-anchor", d => d.x0 < width / 2 ? "start" : "end")
        .style("font-size", "12px")
        .style("font-family", "system-ui, sans-serif")
        .style("fill", "#424242")
        .text(d => d.name)
        .style("cursor", d => d.link ? "pointer" : "default")
        .on("click", (event, d) => {{
            if (d.link) {{
                window.location.href = d.link.replace(".md", ".html");
            }}
        }});

    node.append("title")
        .text(d => `${{d.name}}\nType: ${{d.type || "unknown"}}`);
}})();
</script>
<style>
.d3-sankey-container {{
    background: #FAFAFA;
    border: 1px solid #EEEEEE;
    border-radius: 4px;
    padding: 10px;
    overflow-x: auto;
}}
.d3-sankey-svg {{
    display: block;
    width: 100%;
}}
</style>
"##, id = unique_id, json = json_data)
}

/// Extracts D3 Sunburst blocks and replaces them with placeholders
fn extract_d3_sunburst_blocks(markdown: &str) -> (String, HashMap<String, String>) {
    lazy_static! {
        static ref D3_SUNBURST_BLOCK: Regex = Regex::new(
            r"(?s)```d3-sunburst\s*\n(?P<json>.*?)```"
        ).unwrap();
    }

    let mut map = HashMap::new();
    let mut counter = 0;
    let result = D3_SUNBURST_BLOCK.replace_all(markdown, |caps: &Captures| {
        let json_data = caps["json"].trim();
        let placeholder = format!("{{{{D3_SUNBURST_BLOCK_{}}}}}", counter);
        map.insert(placeholder.clone(), json_data.to_string());
        counter += 1;
        placeholder
    });

    (result.into_owned(), map)
}

/// Restores D3 Sunburst placeholders with rendered HTML
fn restore_d3_sunburst_blocks(content: &str, blocks: &HashMap<String, String>) -> String {
    let mut result = content.to_string();
    for (placeholder, json_data) in blocks {
        let sunburst_html = generate_d3_sunburst_html(json_data);
        result = result.replace(placeholder, &sunburst_html);
    }
    result
}

/// Generate HTML for D3.js Sunburst diagram visualization
/// Uses same hierarchical JSON format as d3-tree
fn generate_d3_sunburst_html(json_data: &str) -> String {
    let unique_id = format!("d3sunburst_{:x}", json_data.as_ptr() as usize);

    format!(r##"
<div class="d3-sunburst-container" id="{id}">
    <div class="d3-sunburst-breadcrumb"></div>
    <div class="d3-sunburst-wrapper">
        <svg class="d3-sunburst-svg"></svg>
    </div>
</div>
<script src="https://d3js.org/d3.v7.min.js"></script>
<script>
(function() {{
    const data = {json};
    const container = document.getElementById("{id}");
    const wrapper = container.querySelector(".d3-sunburst-wrapper");
    const svgEl = container.querySelector("svg");
    const svg = d3.select(svgEl);
    const breadcrumb = d3.select(container).select(".d3-sunburst-breadcrumb");

    // Fixed size for consistent rendering
    const size = 650;
    const radius = size / 2;

    // Use viewBox centered on origin - svg fills container width
    svg.attr("width", "100%")
       .attr("height", size)
       .attr("viewBox", `${{-size/2}} ${{-size/2}} ${{size}} ${{size}}`)
       .attr("preserveAspectRatio", "xMidYMid meet")
       .style("font", "12px system-ui, sans-serif")
       .style("display", "block")
       .style("margin", "0 auto");

    // Main group for drawing (no transform needed - viewBox is centered)
    const viewport = svg.append("g");

    // Colors matching Mermaid diagram theme
    const colors = {{
        "folder": "#9E9E9E",
        "file": "#B8860B",
        "design-document": "#607D8B",
        "user-requirement": "#7E57C2",
        "system-requirement": "#673AB7",
        "requirement": "#673AB7",
        "verification": "#4CAF50",
        "test-verification": "#4CAF50",
        "refinement": "#FF9800",
        "element": "#424242",
        "attachment-element": "#8D6E63",
        "attachment-file": "#8D6E63"
    }};

    const icons = {{
        "folder": "📁",
        "file": "📄",
        "user-requirement": "👤",
        "system-requirement": "📐",
        "requirement": "📐",
        "verification": "✅",
        "refinement": "🔧",
        "design-document": "📝",
        "element": "◽",
        "attachment-element": "🔧",
        "attachment-file": "📎"
    }};

    function getColor(type) {{
        return colors[type] || "#9E9E9E";
    }}

    function getIcon(type) {{
        return icons[type] || "◽";
    }}

    // Create hierarchy
    const root = d3.hierarchy(data)
        .sum(d => d.children ? 0 : 1)
        .sort((a, b) => b.value - a.value);

    // Create partition layout
    const partition = d3.partition()
        .size([2 * Math.PI, radius]);

    partition(root);

    // Arc generator
    const arc = d3.arc()
        .startAngle(d => d.x0)
        .endAngle(d => d.x1)
        .padAngle(d => Math.min((d.x1 - d.x0) / 2, 0.005))
        .padRadius(radius / 2)
        .innerRadius(d => d.y0)
        .outerRadius(d => d.y1 - 1);

    // Store ancestor nodes for breadcrumb clicks
    let breadcrumbAncestors = [];

    // Helper function to generate breadcrumb HTML for a node with click handlers
    function updateBreadcrumbDisplay(node, isHover = false) {{
        const ancestors = node.ancestors().reverse();
        breadcrumbAncestors = ancestors;

        const pathText = ancestors.map((a, i) => {{
            let name = a.data.name;
            if (name.endsWith(".html")) {{
                name = name.replace(".html", ".md");
            }}
            // Make clickable if not the last item (current focus) and not in hover mode
            const isLast = i === ancestors.length - 1;
            const canClick = !isHover && !isLast && node === currentFocus;
            const style = isLast
                ? `color: ${{getColor(a.data.type)}}; font-weight: bold;`
                : canClick
                    ? `color: ${{getColor(a.data.type)}}; cursor: pointer; text-decoration: underline;`
                    : `color: ${{getColor(a.data.type)}};`;
            return `<span class="breadcrumb-item" data-index="${{i}}" style="${{style}}">${{getIcon(a.data.type)}} ${{name}}</span>`;
        }}).join(" → ");
        breadcrumb.html(pathText);

        // Add click handlers to breadcrumb items (only when showing current focus)
        if (!isHover) {{
            breadcrumb.selectAll(".breadcrumb-item").on("click", function() {{
                const index = +d3.select(this).attr("data-index");
                const targetNode = breadcrumbAncestors[index];
                if (targetNode && targetNode !== currentFocus) {{
                    clicked(null, targetNode);
                }}
            }});
        }}
    }}

    // Draw arcs - use viewport group for pan/zoom
    const path = viewport.append("g")
        .selectAll("path")
        .data(root.descendants().filter(d => d.depth))
        .join("path")
        .attr("fill", d => getColor(d.data.type))
        .attr("fill-opacity", d => d.children ? 0.8 : 0.6)
        .attr("d", arc)
        .style("cursor", "pointer")
        .on("click", clicked)
        .on("mouseover", function(event, d) {{
            d3.select(this).attr("fill-opacity", 1);
            updateBreadcrumbDisplay(d, true);
        }})
        .on("mouseout", function(event, d) {{
            d3.select(this).attr("fill-opacity", d.children ? 0.8 : 0.6);
            // Show current focus breadcrumb when not hovering (with click handlers)
            updateBreadcrumbDisplay(currentFocus, false);
        }});

    path.append("title")
        .text(d => {{
            const path = d.ancestors().map(d => d.data.name).reverse().join(" / ");
            return d.data.link ? `${{path}}\n${{d.data.link}}` : path;
        }});

    // Add labels for larger segments - use viewport
    const label = viewport.append("g")
        .attr("pointer-events", "none")
        .attr("text-anchor", "middle")
        .selectAll("text")
        .data(root.descendants().filter(d => d.depth && (d.y0 + d.y1) / 2 * (d.x1 - d.x0) > 10))
        .join("text")
        .attr("transform", function(d) {{
            const x = (d.x0 + d.x1) / 2 * 180 / Math.PI;
            const y = (d.y0 + d.y1) / 2;
            return `rotate(${{x - 90}}) translate(${{y}},0) rotate(${{x < 180 ? 0 : 180}})`;
        }})
        .attr("dy", "0.35em")
        .style("font-size", "10px")
        .style("fill", "#fff")
        .text(d => d.data.name.length > 12 ? d.data.name.substring(0, 10) + "..." : d.data.name);

    // Center circle for going back - use viewport
    const parent = viewport.append("circle")
        .datum(root)
        .attr("r", radius / 6)
        .attr("fill", "#FAFAFA")
        .attr("stroke", "#EEEEEE")
        .attr("stroke-width", 2)
        .attr("pointer-events", "all")
        .style("cursor", "pointer")
        .on("click", clicked);

    // Track current focused node for center link
    let currentFocus = root;

    // Center text - clickable link when focused node has a link - use viewport
    const centerText = viewport.append("text")
        .attr("text-anchor", "middle")
        .attr("dy", "0.35em")
        .style("font-size", "14px")
        .style("fill", "#424242")
        .style("cursor", "default")
        .text(data.name);

    // Update center text to be a link if current node has a link
    function updateCenterLink(node) {{
        currentFocus = node;
        const hasLink = node.data.link;
        centerText
            .text(node.data.name)
            .style("cursor", hasLink ? "pointer" : "default")
            .style("fill", hasLink ? "var(--color-link-hover, #3F51B5)" : "#424242")
            .style("text-decoration", hasLink ? "underline" : "none");
        // Update breadcrumb to show current focus with click handlers
        updateBreadcrumbDisplay(node, false);
    }}

    // Show initial breadcrumb for root
    updateBreadcrumbDisplay(root, false);

    // Click on center text navigates to link
    centerText.on("click", function(event) {{
        event.stopPropagation();
        if (currentFocus.data.link) {{
            window.location.href = currentFocus.data.link.replace(".md", ".html");
        }}
    }});

    function clicked(event, p) {{
        // If clicking on a leaf with a link, navigate
        if (!p.children && p.data.link) {{
            window.location.href = p.data.link.replace(".md", ".html");
            return;
        }}

        parent.datum(p.parent || root);
        updateCenterLink(p);

        root.each(d => d.target = {{
            x0: Math.max(0, Math.min(1, (d.x0 - p.x0) / (p.x1 - p.x0))) * 2 * Math.PI,
            x1: Math.max(0, Math.min(1, (d.x1 - p.x0) / (p.x1 - p.x0))) * 2 * Math.PI,
            y0: Math.max(0, d.y0 - p.y0),
            y1: Math.max(0, d.y1 - p.y0)
        }});

        const t = svg.transition().duration(750);

        path.transition(t)
            .tween("data", d => {{
                const i = d3.interpolate(d.current, d.target);
                return t => d.current = i(t);
            }})
            .filter(function(d) {{
                return +this.getAttribute("fill-opacity") || arcVisible(d.target);
            }})
            .attr("fill-opacity", d => arcVisible(d.target) ? (d.children ? 0.8 : 0.6) : 0)
            .attr("pointer-events", d => arcVisible(d.target) ? "auto" : "none")
            .attrTween("d", d => () => arc(d.current));

        label.filter(function(d) {{
                return +this.getAttribute("fill-opacity") || labelVisible(d.target);
            }}).transition(t)
            .attr("fill-opacity", d => +labelVisible(d.target))
            .attrTween("transform", d => () => labelTransform(d.current));
    }}

    function arcVisible(d) {{
        return d.y1 <= radius && d.y0 >= 0 && d.x1 > d.x0;
    }}

    function labelVisible(d) {{
        return d.y1 <= radius && d.y0 >= 0 && (d.y0 + d.y1) / 2 * (d.x1 - d.x0) > 10;
    }}

    function labelTransform(d) {{
        const x = (d.x0 + d.x1) / 2 * 180 / Math.PI;
        const y = (d.y0 + d.y1) / 2;
        return `rotate(${{x - 90}}) translate(${{y}},0) rotate(${{x < 180 ? 0 : 180}})`;
    }}

    // Store current state for transitions
    root.each(d => d.current = d);
}})();
</script>
<style>
.d3-sunburst-container {{
    background: #FAFAFA;
    border: 1px solid #EEEEEE;
    border-radius: 4px;
    padding: 10px;
}}
.d3-sunburst-wrapper {{
    position: relative;
    width: 100%;
}}
.d3-sunburst-svg {{
    display: block;
    margin: 0 auto;
    max-width: 100%;
}}
.d3-sunburst-breadcrumb {{
    padding: 8px;
    min-height: 24px;
    font-size: 13px;
    color: #424242;
    background: #fff;
    border-radius: 4px;
    margin-bottom: 10px;
}}
.d3-sunburst-breadcrumb span {{
    margin: 0 2px;
}}
</style>
"##, id = unique_id, json = json_data)
}

/// Extracts D3 Icicle blocks and replaces them with placeholders
fn extract_d3_icicle_blocks(markdown: &str) -> (String, HashMap<String, String>) {
    lazy_static! {
        static ref D3_ICICLE_BLOCK: Regex = Regex::new(
            r"(?s)```d3-icicle\s*\n(?P<json>.*?)```"
        ).unwrap();
    }

    let mut map = HashMap::new();
    let mut counter = 0;
    let result = D3_ICICLE_BLOCK.replace_all(markdown, |caps: &Captures| {
        let json_data = caps["json"].trim();
        let placeholder = format!("{{{{D3_ICICLE_BLOCK_{}}}}}", counter);
        map.insert(placeholder.clone(), json_data.to_string());
        counter += 1;
        placeholder
    });

    (result.into_owned(), map)
}

/// Restores D3 Icicle placeholders with rendered HTML
fn restore_d3_icicle_blocks(content: &str, blocks: &HashMap<String, String>) -> String {
    let mut result = content.to_string();
    for (placeholder, json_data) in blocks {
        let icicle_html = generate_d3_icicle_html(json_data);
        result = result.replace(placeholder, &icicle_html);
    }
    result
}

/// Generate HTML for D3.js Icicle/Partition diagram visualization
fn generate_d3_icicle_html(json_data: &str) -> String {
    let unique_id = format!("d3icicle_{:x}", json_data.as_ptr() as usize);

    format!(r##"
<div class="d3-icicle-container" id="{id}">
    <div class="d3-icicle-breadcrumb"></div>
    <div class="d3-icicle-wrapper">
        <svg class="d3-icicle-svg"></svg>
    </div>
</div>
<script src="https://d3js.org/d3.v7.min.js"></script>
<script>
(function() {{
    const data = {json};
    const container = document.getElementById("{id}");
    const wrapper = container.querySelector(".d3-icicle-wrapper");
    const svgEl = container.querySelector("svg");
    const svg = d3.select(svgEl);
    const breadcrumb = d3.select(container).select(".d3-icicle-breadcrumb");

    // Fixed dimensions for consistent rendering
    const width = 1200;
    const height = 800;

    svg.attr("width", "100%")
       .attr("height", height)
       .attr("viewBox", `0 0 ${{width}} ${{height}}`)
       .attr("preserveAspectRatio", "xMidYMid meet")
       .style("font", "11px system-ui, sans-serif")
       .style("display", "block");

    // Main group for drawing
    const viewport = svg.append("g");

    // Colors matching Mermaid diagram theme
    const colors = {{
        "folder": "#9E9E9E",
        "file": "#B8860B",
        "design-document": "#607D8B",
        "user-requirement": "#7E57C2",
        "system-requirement": "#673AB7",
        "requirement": "#673AB7",
        "verification": "#4CAF50",
        "test-verification": "#4CAF50",
        "refinement": "#FF9800",
        "element": "#424242",
        "attachment-element": "#8D6E63",
        "attachment-file": "#8D6E63"
    }};

    function getColor(type) {{
        return colors[type] || colors["element"];
    }}

    // Create hierarchy
    const root = d3.hierarchy(data)
        .sum(d => d.children ? 0 : 1)
        .sort((a, b) => b.height - a.height || a.data.name.localeCompare(b.data.name));

    // Create partition layout
    const partition = d3.partition()
        .size([height, width])
        .padding(1);

    partition(root);

    // Track current focus
    let currentFocus = root;

    // Create cells - use viewport for pan/zoom
    const cell = viewport.selectAll("g")
        .data(root.descendants())
        .join("g")
        .attr("transform", d => `translate(${{d.y0}},${{d.x0}})`);

    const rect = cell.append("rect")
        .attr("width", d => d.y1 - d.y0 - 1)
        .attr("height", d => rectHeight(d))
        .attr("fill", d => getColor(d.data.type))
        .attr("fill-opacity", d => d.children ? 0.8 : 0.6)
        .style("cursor", "pointer")
        .on("click", clicked)
        .on("mouseover", function(event, d) {{
            d3.select(this).attr("fill-opacity", 1);
            // Update breadcrumb
            const ancestors = d.ancestors().reverse();
            const pathText = ancestors.map(a => {{
                let name = a.data.name;
                if (name.endsWith(".html")) {{
                    name = name.replace(".html", ".md");
                }}
                return `<span style="color: ${{getColor(a.data.type)}}">${{name}}</span>`;
            }}).join(" → ");
            breadcrumb.html(pathText);
        }})
        .on("mouseout", function(event, d) {{
            d3.select(this).attr("fill-opacity", d.children ? 0.8 : 0.6);
            // Show current focus path
            updateBreadcrumb(currentFocus);
        }});

    rect.append("title")
        .text(d => d.data.link ? `${{d.data.name}}\n${{d.data.link}}` : d.data.name);

    // Add text labels
    const text = cell.append("text")
        .attr("pointer-events", "none")
        .attr("x", 4)
        .attr("y", d => Math.min(rectHeight(d) / 2 + 4, 14))
        .attr("fill", "#fff")
        .attr("fill-opacity", d => labelVisible(d) ? 1 : 0)
        .text(d => d.data.name);

    // Center link text (shown when zoomed into a node with a link) - use viewport
    const centerLink = viewport.append("text")
        .attr("class", "center-link")
        .attr("text-anchor", "start")
        .attr("x", 10)
        .attr("y", 20)
        .style("font-size", "14px")
        .style("font-weight", "bold")
        .style("fill", "var(--color-link-hover, #3F51B5)")
        .style("cursor", "pointer")
        .style("text-decoration", "underline")
        .style("display", "none")
        .on("click", function() {{
            if (currentFocus.data.link) {{
                window.location.href = currentFocus.data.link.replace(".md", ".html");
            }}
        }});

    // Store ancestor nodes for breadcrumb clicks
    let breadcrumbAncestors = [];

    function updateBreadcrumb(node) {{
        const ancestors = node.ancestors().reverse();
        breadcrumbAncestors = ancestors;

        const pathText = ancestors.map((a, i) => {{
            let name = a.data.name;
            if (name.endsWith(".html")) {{
                name = name.replace(".html", ".md");
            }}
            // Make clickable if not the last item (current focus)
            const isLast = i === ancestors.length - 1;
            const style = isLast
                ? `color: ${{getColor(a.data.type)}}; font-weight: bold;`
                : `color: ${{getColor(a.data.type)}}; cursor: pointer; text-decoration: underline;`;
            return `<span class="breadcrumb-item" data-index="${{i}}" style="${{style}}">${{name}}</span>`;
        }}).join(" → ");
        breadcrumb.html(pathText);

        // Add click handlers to breadcrumb items
        breadcrumb.selectAll(".breadcrumb-item").on("click", function() {{
            const index = +d3.select(this).attr("data-index");
            const targetNode = breadcrumbAncestors[index];
            if (targetNode && targetNode !== currentFocus) {{
                zoomTo(targetNode);
            }}
        }});

        // Update center link
        if (node.data.link && node !== root) {{
            let displayName = node.data.name;
            if (displayName.endsWith(".html")) {{
                displayName = displayName.replace(".html", ".md");
            }}
            centerLink
                .text("→ " + displayName)
                .style("display", "block");
        }} else {{
            centerLink.style("display", "none");
        }}
    }}

    function zoomTo(target) {{
        currentFocus = target;
        updateBreadcrumb(currentFocus);

        root.each(d => {{
            d.target = {{
                x0: (d.x0 - currentFocus.x0) / (currentFocus.x1 - currentFocus.x0) * height,
                x1: (d.x1 - currentFocus.x0) / (currentFocus.x1 - currentFocus.x0) * height,
                y0: d.y0 - currentFocus.y0,
                y1: d.y1 - currentFocus.y0
            }};
        }});

        const t = svg.transition().duration(750);

        cell.transition(t)
            .attr("transform", d => `translate(${{d.target.y0}},${{d.target.x0}})`);

        rect.transition(t)
            .attr("height", d => rectHeight(d.target));

        text.transition(t)
            .attr("fill-opacity", d => labelVisible(d.target) ? 1 : 0)
            .attr("y", d => Math.min(rectHeight(d.target) / 2 + 4, 14));
    }}

    function clicked(event, p) {{
        // If clicking on a leaf with a link, navigate
        if (!p.children && p.data.link) {{
            window.location.href = p.data.link.replace(".md", ".html");
            return;
        }}

        // Toggle: if clicking same node, go to parent; otherwise zoom into clicked node
        const target = currentFocus === p ? (p.parent || root) : p;
        zoomTo(target);
    }}

    function rectHeight(d) {{
        return d.x1 - d.x0 - 1;
    }}

    function labelVisible(d) {{
        return d.y1 <= width && d.y0 >= 0 && d.x1 - d.x0 > 16;
    }}

    // Store current state for transitions
    root.each(d => d.target = {{ x0: d.x0, x1: d.x1, y0: d.y0, y1: d.y1 }});

    // Show initial breadcrumb
    updateBreadcrumb(root);
}})();
</script>
<style>
.d3-icicle-container {{
    background: #FAFAFA;
    border: 1px solid #EEEEEE;
    border-radius: 4px;
    padding: 10px;
}}
.d3-icicle-wrapper {{
    position: relative;
    width: 100%;
    overflow: hidden;
}}
.d3-icicle-svg {{
    display: block;
}}
.d3-icicle-breadcrumb {{
    margin-bottom: 10px;
    padding: 8px;
    min-height: 24px;
    font-size: 13px;
    color: #424242;
    background: #fff;
    border-radius: 4px;
}}
.d3-icicle-breadcrumb span {{
    margin: 0 2px;
}}
</style>
"##, id = unique_id, json = json_data)
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
