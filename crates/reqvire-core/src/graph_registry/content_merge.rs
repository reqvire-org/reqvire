//! Content and ontology-block merge helpers used by `GraphRegistry` merge
//! operations. These are free functions operating on element content strings.

use rustc_hash::FxHashSet;

use crate::crud::replace_prefix_token;
use crate::element::Element;
use crate::error::ReqvireError;
use crate::parser::extract_single_fenced_subsection;

/// Extract main content and details section from element content
///
/// Returns (main_content, details_content) where:
/// - main_content: Everything before the first "#### Details" header
/// - details_content: Everything after "#### Details" header until the next #### section
pub(crate) fn extract_content_parts(content: &str) -> (String, String) {
    let details_marker = "#### Details";
    if let Some(pos) = content.find(details_marker) {
        let main = content[..pos].to_string();
        let after_marker = pos + details_marker.len();
        let rest = &content[after_marker..];

        // Find end of details (next #### or end)
        let details_end = rest.find("\n#### ").unwrap_or(rest.len());

        (main, rest[..details_end].to_string())
    } else {
        (content.to_string(), String::new())
    }
}

pub(crate) fn extract_leading_prose(content: &str) -> String {
    let mut lines = Vec::new();
    for line in content.lines() {
        if line.trim_start().starts_with("#### ") {
            break;
        }
        lines.push(line);
    }
    lines.join("\n")
}

/// Merge additional content into the Details section of target content
pub(crate) fn merge_content_into_details(target_content: &str, additional: &str) -> String {
    if additional.trim().is_empty() {
        return target_content.to_string();
    }

    let details_marker = "#### Details";
    if let Some(pos) = target_content.find(details_marker) {
        // Find end of existing details
        let after_marker = pos + details_marker.len();
        let rest = &target_content[after_marker..];
        let details_end = rest
            .find("\n#### ")
            .map(|p| after_marker + p)
            .unwrap_or(target_content.len());

        // Insert additional content at end of Details
        let mut result = target_content[..details_end].to_string();
        result.push_str(additional);
        result.push_str(&target_content[details_end..]);
        result
    } else {
        // No Details section - create one
        format!(
            "{}\n#### Details\n{}",
            target_content.trim_end(),
            additional
        )
    }
}

pub(crate) fn merge_ontology_blocks_into_target(
    target_content: &str,
    target_element: &Element,
    source_elements: &[Element],
) -> Result<String, ReqvireError> {
    let mut merged_blocks = Vec::new();

    let target_block = extract_single_fenced_subsection(target_content, "Ontology")
        .into_iter()
        .next()
        .ok_or_else(|| {
            ReqvireError::InvalidOperation(format!(
                "Ontology element '{}' must contain exactly one #### Ontology fenced Turtle block.",
                target_element.name
            ))
        })?;
    merged_blocks.push(target_block.content.trim_end().to_string());

    for source_element in source_elements {
        let Some(source_block) =
            extract_single_fenced_subsection(&source_element.content, "Ontology")
                .into_iter()
                .next()
        else {
            continue;
        };
        let rewritten =
            rewrite_ontology_block_for_merge(&source_block.content, source_element, target_element);
        if !rewritten.trim().is_empty() {
            merged_blocks.push(rewritten);
        }
    }

    Ok(dedupe_turtle_block(merged_blocks.join("\n\n").trim_end()))
}

fn rewrite_ontology_block_for_merge(
    block: &str,
    source_element: &Element,
    target_element: &Element,
) -> String {
    let source_base = source_element
        .metadata
        .get("ontology_base")
        .cloned()
        .unwrap_or_default();
    let source_prefix = source_element
        .metadata
        .get("ontology_prefix")
        .cloned()
        .unwrap_or_default();
    let target_base = target_element
        .metadata
        .get("ontology_base")
        .cloned()
        .unwrap_or_default();
    let target_prefix = target_element
        .metadata
        .get("ontology_prefix")
        .cloned()
        .unwrap_or_default();

    let mut rewritten_lines = Vec::new();
    let source_namespace = if source_base.is_empty() {
        String::new()
    } else {
        format!("{}#", source_base)
    };

    for line in block.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&format!("@prefix {}: <", source_prefix))
            && trimmed.contains(&format!("<{}> .", source_namespace))
        {
            continue;
        }
        if trimmed.starts_with("@prefix owl: ") || trimmed.starts_with("prefix owl: ") {
            continue;
        }
        if trimmed.contains("owl:Ontology")
            || trimmed.contains("owl#imports")
            || trimmed.contains("owl:imports")
        {
            continue;
        }
        rewritten_lines.push(line.to_string());
    }

    let mut rewritten = rewritten_lines.join("\n");
    if !source_base.is_empty() && !target_base.is_empty() {
        rewritten = rewritten.replace(&source_base, &target_base);
    }
    if !source_prefix.is_empty() && !target_prefix.is_empty() && source_prefix != target_prefix {
        rewritten = replace_prefix_token(&rewritten, &source_prefix, &target_prefix);
    }

    rewritten.trim().to_string()
}

fn dedupe_turtle_block(block: &str) -> String {
    let mut seen_prefix_lines = FxHashSet::default();
    let mut seen_exact_lines = FxHashSet::default();
    let mut output = Vec::new();

    for line in block.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if output
                .last()
                .map(|prev: &String| prev.trim().is_empty())
                .unwrap_or(false)
            {
                continue;
            }
            output.push(String::new());
            continue;
        }

        if trimmed.starts_with("@prefix ") || trimmed.starts_with("prefix ") {
            if !seen_prefix_lines.insert(trimmed.to_string()) {
                continue;
            }
        } else if !seen_exact_lines.insert(trimmed.to_string()) {
            continue;
        }

        output.push(line.to_string());
    }

    output.join("\n").trim_end().to_string()
}

pub(crate) fn replace_single_fenced_subsection(
    content: &str,
    subsection: &str,
    replacement: &str,
) -> Result<String, ReqvireError> {
    let header = format!("#### {}", subsection);
    let mut output = String::new();
    let mut in_target_section = false;
    let mut saw_fence = false;
    let mut skipping_block = false;
    let mut replaced = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if !replaced && trimmed == header {
            in_target_section = true;
            output.push_str(line);
            output.push('\n');
            continue;
        }

        if in_target_section && !saw_fence {
            output.push_str(line);
            output.push('\n');
            if trimmed.starts_with("```") {
                saw_fence = true;
                if !replacement.trim().is_empty() {
                    output.push_str(replacement.trim_end());
                    output.push('\n');
                }
                skipping_block = true;
            }
            continue;
        }

        if skipping_block {
            if trimmed.starts_with("```") {
                output.push_str(line);
                output.push('\n');
                skipping_block = false;
                in_target_section = false;
                replaced = true;
            }
            continue;
        }

        output.push_str(line);
        output.push('\n');
    }

    if !replaced {
        return Err(ReqvireError::InvalidOperation(format!(
            "Expected to replace #### {} fenced Turtle block but the section was not found.",
            subsection
        )));
    }

    Ok(output.trim_end_matches('\n').to_string())
}
