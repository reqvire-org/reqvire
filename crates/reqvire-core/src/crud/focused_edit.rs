//! Selected-content edits share the ordinary candidate validation and persistence path.
use super::*;
use crate::element::{Element, SubSection};
use crate::parser::{subsection_headers, CodeFence};
use serde::Deserialize;
use std::ops::Range;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FocusedEditOperation {
    Append,
    Replace,
    ReplaceText,
    Create,
    Delete,
}

#[derive(Debug, Deserialize)]
pub struct FocusedEdit {
    pub element_name: String,
    pub target: String,
    pub operation: FocusedEditOperation,
    pub content: Option<String>,
    pub old_text: Option<String>,
    pub new_text: Option<String>,
    pub expected_model_revision: Option<String>,
    #[serde(default)]
    pub dry_run: bool,
}

pub struct FocusedEditResult {
    pub result: CrudResult,
    pub base_model_revision: String,
}

fn invalid(message: impl Into<String>) -> ReqvireError {
    ReqvireError::InvalidMarkdownStructure(message.into())
}

impl FocusedEdit {
    fn validate_arguments(&self) -> Result<(), ReqvireError> {
        use FocusedEditOperation::*;
        let valid = match self.operation {
            Append | Replace | Create => {
                self.content.is_some() && self.old_text.is_none() && self.new_text.is_none()
            }
            ReplaceText => {
                self.content.is_none()
                    && self.old_text.as_ref().is_some_and(|s| !s.is_empty())
                    && self.new_text.is_some()
            }
            Delete => self.content.is_none() && self.old_text.is_none() && self.new_text.is_none(),
        };
        if !valid {
            return Err(invalid("Invalid focused edit arguments: append/replace/create require only content; replace_text requires nonempty old_text and new_text; delete accepts none of these fields"));
        }
        if self.target.is_empty()
            || self.target.trim() != self.target
            || self.target.contains(['\n', '\r'])
        {
            return Err(invalid(
                "target must be an exact nonempty subsection name or 'Element Text'",
            ));
        }
        Ok(())
    }

    fn validate_target_policy(&self, element: &Element) -> Result<(), ReqvireError> {
        use FocusedEditOperation::*;
        let allowed = match self.target.as_str() {
            "Element Text" => !matches!(self.operation, Create | Delete),
            "Ontology" => {
                element.element_type.is_ontology()
                    && matches!(self.operation, Replace | ReplaceText)
            }
            "Shapes" => {
                element.element_type.is_semantic_contract()
                    && matches!(self.operation, Replace | ReplaceText)
            }
            "Query" => {
                element.element_type.is_semantic_query()
                    && matches!(self.operation, Replace | ReplaceText)
            }
            "Produces" | "Concepts" | "Definition" | "Top Concepts" => false,
            _ => match SubSection::parse(&self.target) {
                SubSection::Details => !element.element_type.is_concept_family(),
                SubSection::ScopeNote | SubSection::Examples => element.element_type.is_concept(),
                SubSection::Other(_) => {
                    !crate::element::is_legacy_contract_bindings_section(&self.target)
                }
                // New native subsections must acquire an explicit editing policy.
                _ => false,
            },
        };
        if !allowed {
            return Err(invalid(format!("Target '{}' does not support this operation on type '{}'; use its supported declaration tool or full-definition override", self.target, element.element_type.as_str())));
        }
        Ok(())
    }
}

struct TargetRange {
    whole: Range<usize>,
    contents: Range<usize>,
}

fn target_range(content: &str, target: &str) -> Result<Option<TargetRange>, ReqvireError> {
    let headers = subsection_headers(content);
    let mut offset = 0;
    let offsets: Vec<_> = content
        .split_inclusive('\n')
        .map(|line| {
            let start = offset;
            offset += line.len();
            (start, offset)
        })
        .collect();
    if target == "Element Text" {
        let end = headers
            .first()
            .map_or(content.len(), |(line, _)| offsets[*line].0);
        return Ok(Some(TargetRange {
            whole: 0..end,
            contents: 0..end,
        }));
    }
    let matches: Vec<_> = headers
        .iter()
        .enumerate()
        .filter(|(_, (_, name))| *name == target)
        .collect();
    if matches.len() > 1 {
        return Err(invalid(format!("Ambiguous repeated subsection '{target}'")));
    }
    Ok(matches.first().map(|(index, (line, _))| {
        let end = headers
            .get(index + 1)
            .map_or(content.len(), |(next, _)| offsets[*next].0);
        TargetRange {
            whole: offsets[*line].0..end,
            contents: offsets[*line].1..end,
        }
    }))
}

/// A fragment must close its own literal containers and cannot change the
/// surrounding document grammar. Use the same fence recognizer as model parsing.
fn validate_fragment(text: &str, target: &str, single_element: bool) -> Result<(), ReqvireError> {
    let mut fence = CodeFence::default();
    static TAGS: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"(?i)<!--|-->|</?(details|div|section|article|aside|blockquote|table|thead|tbody|tfoot|tr|td|th|ul|ol|li|dl|dt|dd|pre|script|style|textarea|p|summary|figure|figcaption)(?:\s[^>]*|)>").expect("HTML boundary pattern")
    });
    let mut html: Vec<String> = Vec::new();
    let mut in_details = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if !in_details && fence.consume(trimmed) {
            continue;
        }
        // The native grammar treats only details containers as literal content.
        if !in_details {
            let level = trimmed.bytes().take_while(|c| *c == b'#').count();
            let heading = level > 0
                && trimmed
                    .as_bytes()
                    .get(level)
                    .is_some_and(u8::is_ascii_whitespace);
            if trimmed == "---"
                || (heading && (level == 4 || (!single_element && level <= 3)))
                || (heading && level >= 5 && !single_element && target != "Details")
            {
                return Err(invalid(
                    "Focused edit content crosses a document or subsection boundary",
                ));
            }
        }
        // Mirror native details parsing: opening/closing must begin a line,
        // and an opening line cannot also close the native literal block.
        if in_details {
            if trimmed.starts_with("</details>") {
                in_details = false;
            }
        } else if trimmed.starts_with("<details") {
            in_details = true;
        }
        for capture in TAGS.captures_iter(line) {
            let token = capture.get(0).expect("matched HTML token").as_str();
            if html.last().is_some_and(|tag| tag == "!--") && token != "-->" {
                continue;
            }
            if token == "-->" {
                if html.pop().as_deref() != Some("!--") {
                    return Err(invalid("Unmatched HTML comment boundary in focused edit"));
                }
            } else if token == "<!--" {
                html.push("!--".into());
            } else {
                let name = capture
                    .get(1)
                    .expect("matched HTML tag name")
                    .as_str()
                    .to_ascii_lowercase();
                if token.starts_with("</") {
                    if html.pop().as_deref() != Some(&name) {
                        return Err(invalid("Unmatched HTML container boundary in focused edit"));
                    }
                } else if !token.ends_with("/>") {
                    html.push(name);
                }
            }
        }
    }
    if fence.is_open() || !html.is_empty() || in_details {
        return Err(invalid(
            "Focused edit contains an unclosed code fence or HTML container",
        ));
    }
    Ok(())
}

fn edited_content(element: &Element, edit: &FocusedEdit) -> Result<String, ReqvireError> {
    use FocusedEditOperation::*;
    let range = target_range(&element.content, &edit.target)?;
    if edit.operation == Create {
        if range.is_some() {
            return Err(invalid(format!(
                "Subsection '{}' already exists",
                edit.target
            )));
        }
    } else if range.is_none() {
        return Err(invalid(format!(
            "Subsection '{}' does not exist",
            edit.target
        )));
    }
    let old = range.as_ref().map_or("", |r| {
        element.content[r.contents.clone()].trim_matches('\n')
    });
    let replacement = match edit.operation {
        Append => match (
            old.is_empty(),
            edit.content.as_deref().expect("validated content"),
        ) {
            (_, "") => old.to_owned(),
            (true, addition) => addition.to_owned(),
            (false, addition) => format!("{old}\n\n{addition}"),
        },
        Replace | Create => edit.content.clone().expect("validated content"),
        ReplaceText => {
            let needle = edit.old_text.as_deref().expect("validated old_text");
            // Count overlapping occurrences as ambiguous as well.
            let matches = old
                .char_indices()
                .filter(|(i, _)| old[*i..].starts_with(needle))
                .take(2)
                .count();
            if matches != 1 {
                return Err(invalid(
                    "old_text must match exactly once within the selected target",
                ));
            }
            old.replacen(
                needle,
                edit.new_text.as_deref().expect("validated new_text"),
                1,
            )
        }
        Delete => String::new(),
    };
    let single = element.metadata.contains_key("_single_element_format");
    validate_fragment(&replacement, &edit.target, single)?;
    // Preserve unchanged targets byte for byte, including their trailing separators.
    if !matches!(edit.operation, Create | Delete) && replacement == old {
        return Ok(element.content.clone());
    }
    let (range, replacement) = match (edit.operation, range) {
        (Create, _) => (
            element.content.len()..element.content.len(),
            format!("#### {}\n{}", edit.target, replacement),
        ),
        (Delete, Some(r)) => (r.whole, replacement),
        (_, Some(r)) => (r.contents, replacement),
        _ => unreachable!(),
    };
    let before = element.content[..range.start].trim_end_matches('\n');
    let after = element.content[range.end..].trim_start_matches('\n');
    let mut result = before.to_owned();
    if !replacement.is_empty() {
        if !result.is_empty() {
            // Subsection headings have one newline before their contents.
            result.push_str(
                if edit.operation != Create
                    && edit.operation != Delete
                    && edit.target != "Element Text"
                {
                    "\n"
                } else {
                    "\n\n"
                },
            );
        }
        result.push_str(&replacement);
    }
    if !after.is_empty() {
        if !result.is_empty() {
            result.push_str("\n\n");
        }
        result.push_str(after);
    }
    Ok(result)
}

/// Called under the MCP context's exclusive mutation gate, against its accepted snapshot.
pub fn update_element(
    model_manager: &mut ModelManager,
    edit: &FocusedEdit,
    excluded_patterns: &GlobSet,
    workspace_root: &Path,
) -> Result<FocusedEditResult, ReqvireError> {
    edit.validate_arguments()?;
    let base_model_revision =
        crate::model_fingerprint(&model_manager.graph_registry.get_all_elements())?;
    if edit
        .expected_model_revision
        .as_ref()
        .is_some_and(|revision| revision != &base_model_revision)
        || (!edit.dry_run && edit.expected_model_revision.is_none())
    {
        return Err(invalid("expected_model_revision must match the current accepted model revision; read the model and preview again"));
    }
    let original = model_manager
        .graph_registry
        .select_element(&edit.element_name, "update_element element_name")?
        .clone();
    edit.validate_target_policy(&original)?;
    let content = edited_content(&original, edit)?;
    let mut result = CrudResult {
        operation: CrudOperation::Update,
        element_id: original.identifier.clone(),
        element_name: original.name.clone(),
        diffs: vec![],
        dry_run: edit.dry_run,
    };
    if content != original.content {
        let snapshot = model_manager.graph_registry.clone();
        let mut candidate = snapshot.clone();
        let element = &mut candidate
            .nodes
            .get_mut(&original.identifier)
            .expect("selected element in candidate snapshot")
            .element;
        element.content = content;
        // Recompute diagnostics from the candidate rather than stale source locations.
        element.content_source_lines.clear();
        element.freeze_content();
        if element.content == original.content {
            return Ok(FocusedEditResult {
                result,
                base_model_revision,
            });
        }
        let (errors, _) = candidate.build_relations(excluded_patterns)?;
        if !errors.is_empty() {
            return Err(ReqvireError::validation_diagnostics(errors));
        }
        candidate.modified_files.insert(original.file_path.clone());
        let mut modified_before = snapshot_modified_files(model_manager);
        modified_before.remove(&original.file_path);
        model_manager.graph_registry = candidate;
        let finalized = finalize_crud_operation(
            model_manager,
            &modified_before,
            workspace_root,
            // Session previews must prepare the exact bytes for the ordinary
            // full-file validation/format pass, then the gate discards them.
            edit.dry_run && !crate::mutation_io::active(),
            None,
            None,
        );
        if finalized.is_err() || edit.dry_run {
            model_manager.graph_registry = snapshot;
        }
        result.diffs = finalized?;
    }
    Ok(FocusedEditResult {
        result,
        base_model_revision,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_boundaries_follow_the_document_grammar() {
        for fragment in [
            "```markdown\n#### Metadata\n### Example\n````",
            "<details>\n#### Metadata\n</details>",
        ] {
            assert!(
                validate_fragment(fragment, "Details", false).is_ok(),
                "{fragment}"
            );
        }
        for fragment in [
            "```markdown\n#### Metadata",
            "<details>\nUnclosed",
            "<div>\nUnclosed",
            "<!-- Unclosed",
            "<details>inline</details>",
            "<DETAILS>\n#### Metadata\n</DETAILS>",
        ] {
            assert!(
                validate_fragment(fragment, "Details", false).is_err(),
                "{fragment}"
            );
        }
        assert!(validate_fragment("## Nested heading", "Element Text", true).is_ok());
        assert!(validate_fragment("## Nested heading", "Element Text", false).is_err());
        assert!(validate_fragment("##### Nested heading", "Details", false).is_ok());
        assert!(validate_fragment("##### Nested heading", "Element Text", false).is_err());
    }

    #[test]
    fn reserved_text_selector_and_repeated_subsections_are_unambiguous() {
        let content = "Opening prose.\n\n#### Element Text\nCustom section.\n\n#### Details\nFirst.\n\n#### Details\nSecond.";
        let selected = target_range(content, "Element Text")
            .expect("selection")
            .expect("opening text");
        assert_eq!(content[selected.contents].trim_end(), "Opening prose.");
        assert!(target_range(content, "Details").is_err());
        assert!(target_range(content, "Missing")
            .expect("selection")
            .is_none());
    }

    #[test]
    fn overlapping_literal_matches_cannot_choose_a_replacement() {
        let mut element = Element::new("Example", "Model.md#example", "Model.md", 1, None);
        element.content = "aaa".into();
        let edit: FocusedEdit = serde_json::from_value(serde_json::json!({
            "element_name":"Example", "target":"Element Text", "operation":"replace_text",
            "old_text":"aa", "new_text":"b", "dry_run":true
        }))
        .expect("valid edit fixture");
        assert!(edited_content(&element, &edit).is_err());
    }
}
