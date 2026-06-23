use crate::element::{
    ConceptReference, Element, ElementType, FencedBlock, RequirementType,
    ContractBindingEntry, ContractBindingTarget, SubSection,
    CONTRACT_BINDINGS_SECTION, is_legacy_contract_bindings_section,
};
use crate::error::ReqvireError;
use crate::relation::{self, Relation};
use crate::utils;
use log::debug;
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

pub const ELEMENTS_HEADER: &str = "# Elements";
pub const SINGLE_ELEMENT_HEADER: &str = "# Element";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedExternalOntologySource {
    pub prefix: String,
    pub namespace: String,
    pub resource: String,
    pub source: String,
    pub format: String,
    pub line_number: usize,
}

pub fn extract_single_fenced_subsection(content: &str, subsection: &str) -> Vec<FencedBlock> {
    let header = format!("#### {}", subsection);
    let mut blocks = Vec::new();
    let mut in_section = false;
    let mut in_fence = false;
    let mut language = String::new();
    let mut block_content = String::new();
    let mut fence_line_number = 0;

    for (line_index, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("#### ") {
            if in_fence {
                blocks.push(FencedBlock {
                    language: language.clone(),
                    content: block_content.trim_end().to_string(),
                    line_number: fence_line_number,
                });
                in_fence = false;
                language.clear();
                block_content.clear();
                fence_line_number = 0;
            }
            in_section = trimmed == header;
            continue;
        }

        if !in_section {
            continue;
        }

        if trimmed.starts_with("```") {
            if in_fence {
                blocks.push(FencedBlock {
                    language: language.clone(),
                    content: block_content.trim_end().to_string(),
                    line_number: fence_line_number,
                });
                in_fence = false;
                language.clear();
                block_content.clear();
                fence_line_number = 0;
            } else {
                in_fence = true;
                language = trimmed.trim_start_matches("```").trim().to_string();
                fence_line_number = line_index + 1;
            }
            continue;
        }

        if in_fence {
            block_content.push_str(line);
            block_content.push('\n');
        }
    }

    if in_fence {
        blocks.push(FencedBlock {
            language,
            content: block_content.trim_end().to_string(),
            line_number: fence_line_number,
        });
    }

    blocks
}

pub fn has_subsection(content: &str, subsection: &str) -> bool {
    let header = format!("#### {}", subsection);
    content
        .lines()
        .any(|line| line.trim() == header || line.trim().starts_with(&(header.clone() + " ")))
}

pub fn extract_concept_references(content: &str) -> (Vec<ConceptReference>, Vec<String>) {
    let mut references = Vec::new();
    let mut diagnostics = Vec::new();
    let mut in_section = false;

    for (line_index, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("#### ") {
            in_section = trimmed == "#### Concept References";
            continue;
        }

        if !in_section || trimmed.is_empty() {
            continue;
        }

        let Some(entry) = trimmed.strip_prefix("* ") else {
            diagnostics.push(format!(
                "Concept References line {} must use '* [Label](concept-element-link)' syntax.",
                line_index + 1
            ));
            continue;
        };

        let Some((label, target)) = utils::extract_markdown_link(entry.trim()) else {
            diagnostics.push(format!(
                "Concept References line {} must use a Markdown link to a native concept element.",
                line_index + 1
            ));
            continue;
        };

        let label = label.trim();
        let target = target.trim();
        if label.is_empty() || target.is_empty() {
            diagnostics.push(format!(
                "Concept References line {} must contain a non-empty label and concept element link.",
                line_index + 1
            ));
            continue;
        }

        references.push(ConceptReference {
            label: label.to_string(),
            target: target.to_string(),
            line_number: line_index + 1,
        });
    }

    (references, diagnostics)
}

pub fn normalize_concept_reference_target(
    source_file_path: &str,
    target: &str,
) -> Result<String, ReqvireError> {
    if target.starts_with('#') {
        return Ok(format!("{}{}", source_file_path, target));
    }

    let git_root = crate::git_commands::get_git_root_dir()?;
    let file_parent = Path::new(source_file_path).parent().ok_or_else(|| {
        ReqvireError::PathError(format!(
            "Cannot determine parent directory of '{}'",
            source_file_path
        ))
    })?;
    let base_path = git_root.join(file_parent);
    utils::normalize_identifier(target, &base_path)
}

pub fn parse_external_ontology_sources(
    content: &str,
) -> (Vec<ParsedExternalOntologySource>, Vec<String>) {
    let mut sources = Vec::new();
    let mut diagnostics = Vec::new();
    let mut in_section = false;
    let mut section_line = 0;
    let mut values: BTreeMap<String, String> = BTreeMap::new();

    for (line_index, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("#### ") {
            if in_section {
                finish_external_ontology_section(
                    &mut sources,
                    &mut diagnostics,
                    &mut values,
                    section_line,
                );
            }
            in_section = trimmed == "#### External Ontology";
            section_line = line_index + 1;
            continue;
        }

        if !in_section || trimmed.is_empty() {
            continue;
        }

        let Some(entry) = trimmed
            .strip_prefix("* ")
            .or_else(|| trimmed.strip_prefix("- "))
        else {
            diagnostics.push(format!(
                "External Ontology section at line {} has invalid entry '{}'. Expected '* key: value'.",
                line_index + 1,
                trimmed
            ));
            continue;
        };
        let Some((key, value)) = entry.split_once(':') else {
            diagnostics.push(format!(
                "External Ontology section at line {} has invalid entry '{}'. Expected '* key: value'.",
                line_index + 1,
                trimmed
            ));
            continue;
        };
        values.insert(key.trim().to_string(), value.trim().to_string());
    }

    if in_section {
        finish_external_ontology_section(&mut sources, &mut diagnostics, &mut values, section_line);
    }

    (sources, diagnostics)
}

fn finish_external_ontology_section(
    sources: &mut Vec<ParsedExternalOntologySource>,
    diagnostics: &mut Vec<String>,
    values: &mut BTreeMap<String, String>,
    section_line: usize,
) {
    if values.is_empty() {
        return;
    }

    let prefix = required_external_ontology_value(values, diagnostics, section_line, "prefix");
    let namespace =
        required_external_ontology_value(values, diagnostics, section_line, "namespace");
    let resource = required_external_ontology_value(values, diagnostics, section_line, "resource");
    let source = required_external_ontology_value(values, diagnostics, section_line, "source");
    let format = values
        .get("format")
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "turtle".to_string());

    if let (Some(prefix), Some(namespace), Some(resource), Some(source)) =
        (prefix, namespace, resource, source)
    {
        sources.push(ParsedExternalOntologySource {
            prefix,
            namespace,
            resource,
            source,
            format,
            line_number: section_line,
        });
    }

    values.clear();
}

fn required_external_ontology_value(
    values: &BTreeMap<String, String>,
    diagnostics: &mut Vec<String>,
    section_line: usize,
    key: &str,
) -> Option<String> {
    values
        .get(key)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .or_else(|| {
            diagnostics.push(format!(
                "External Ontology section at line {} is missing required '{}' entry.",
                section_line, key
            ));
            None
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelFileType {
    Elements,
    SingleElement,
    Unsupported,
}

/// Returns an example of correctly formatted element markdown for error messages
fn get_element_example() -> &'static str {
    r#"
Example of correctly formatted element:

### Element Name

Brief description of the element.

#### Details
Additional details can go here.

You can use **markdown formatting**.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Parent Element](../ParentFile.md#parent-element)
  * satisfiedBy: [Implementation](../code/impl.rs)

#### Contract Bindings
  * [Reused Specification](Specifications.md#specification-name)
---
"#
}

/// Parses a single element from markdown string.
/// Used for CRUD operations (add command) to parse element from stdin or inline argument.
/// Returns the parsed Element or an error.
pub fn parse_single_element(content: &str, file_path: &str) -> Result<Element, ReqvireError> {
    let mut current_element: Option<Element> = None;
    let mut current_subsection = SubSection::Other("".to_string());
    let mut seen_subsections = HashSet::new();
    let mut in_details_block = false;
    let mut found_header = false;

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Handle <details> blocks
        if in_details_block {
            if let Some(element) = &mut current_element {
                element.add_content(&format!("{}\n", line));
            }
            if trimmed.starts_with("</details>") {
                in_details_block = false;
            }
            continue;
        }

        // Separator line ends the element
        if trimmed == "---" {
            break;
        }

        // Parse ### header (element name)
        if trimmed.starts_with("### ") {
            if found_header {
                return Err(ReqvireError::InvalidMarkdownStructure(
                    format!("Multiple ### headers found. Single element should have only one ### header.\n{}", get_element_example())
                ));
            }

            found_header = true;
            current_subsection = SubSection::Requirement;

            let element_name = trimmed
                .strip_prefix("### ")
                .unwrap_or(trimmed)
                .trim()
                .to_string();

            // file_path is already relative to git root, use it directly
            // Only normalize the fragment part (element name)
            let normalized_fragment = utils::normalize_fragment(&element_name);
            let normalized_id = format!("{}#{}", file_path, normalized_fragment);

            // Default element type is always 'requirement' (location-independent)
            let element_type = ElementType::Requirement(RequirementType::System);

            let new_element = Element::new(
                &element_name,
                &normalized_id,
                file_path, // Already relative
                line_num + 1,
                Some(element_type),
            );

            current_element = Some(new_element);

        // Parse #### subsections
        } else if trimmed.starts_with("#### ") && current_element.is_some() {
            let subsection_name = trimmed[5..].trim();
            if is_legacy_contract_bindings_section(subsection_name) {
                return Err(ReqvireError::InvalidMarkdownStructure(format!(
                    "Legacy subsection '{}' is not supported. Use '#### {}' or run `reqvire migrate --fix`.",
                    subsection_name, CONTRACT_BINDINGS_SECTION
                )));
            }

            let subsection = SubSection::parse(subsection_name);

            if !subsection.is_repeatable() && seen_subsections.contains(&subsection) {
                return Err(ReqvireError::DuplicateSubsection(format!(
                    "Duplicate subsection '{}'",
                    subsection.name()
                )));
            }
            if !subsection.is_repeatable() {
                seen_subsections.insert(subsection.clone());
            }

            // If transitioning to content-bearing subsection, add the header to content
            if subsection.is_content_bearing() {
                if let Some(element) = &mut current_element {
                    element.add_content(&format!("\n{}\n", line));
                }
            }

            current_subsection = subsection;

        // Handle level 5+ headers
        } else if trimmed.starts_with("#####")
            && current_element.is_some()
            && current_subsection != SubSection::Details
        {
            return Err(ReqvireError::InvalidMarkdownStructure(
                "Level 5+ headers (#####+) can only appear inside '#### Details' subsection"
                    .to_string(),
            ));

        // Parse content for Requirement or Details subsections
        } else if current_subsection == SubSection::Requirement
            || current_subsection.is_content_bearing()
        {
            if let Some(element) = &mut current_element {
                if trimmed.starts_with("<details") {
                    in_details_block = true;
                }
                element.add_content(&format!("{}\n", line));
            }

        // Parse metadata
        } else if current_subsection == SubSection::Metadata {
            if trimmed.is_empty() {
                continue;
            }
            if let Some(element) = &mut current_element {
                if let Some((key, value)) = utils::parse_metadata_line(trimmed) {
                    element.metadata.insert(key.clone(), value.clone());

                    if key.eq_ignore_ascii_case("type") {
                        element.set_type_from_metadata();
                    }
                } else {
                    return Err(ReqvireError::InvalidMetadataFormat(format!(
                        "Invalid metadata format: '{}'. Expected format: '  * key: value'\n{}",
                        trimmed,
                        get_element_example()
                    )));
                }
            }

        // Parse relations
        } else if current_subsection == SubSection::Relations {
            if let Some(element) = &mut current_element {
                if trimmed.starts_with("* ") {
                    let (relation_type, (text, link)) = utils::parse_relation_line(trimmed)
                        .map_err(|_| ReqvireError::InvalidRelationFormat(
                            format!("Invalid relation format: '{}'. Expected format: '  * relationType: [Text](link)'\n{}", trimmed, get_element_example())
                        ))?;

                    // Normalize relation target
                    // file_path is already relative to git root (e.g., "system-model/NewFile.md")
                    let git_root = crate::git_commands::get_git_root_dir()?;

                    let normalized_target = if link.starts_with('#') {
                        // Same-file reference: just prepend file_path (already git-root-relative)
                        format!("{}{}", file_path, link)
                    } else {
                        // External file reference: normalize relative to the target file's directory
                        // Get the directory of the target file as base path for normalization
                        let file_parent = Path::new(file_path).parent().ok_or_else(|| {
                            ReqvireError::PathError(format!(
                                "Cannot determine parent directory of '{}'",
                                file_path
                            ))
                        })?;
                        let base_path = git_root.join(file_parent);

                        utils::normalize_identifier(&link, &base_path)?
                    };

                    let relation = Relation::new(&relation_type, text, &normalized_target, None)?;

                    // Check for duplicate relations (same type and target)
                    let is_duplicate = element.relations.iter().any(|r| {
                        r.relation_type == relation.relation_type && r.target == relation.target
                    });
                    if is_duplicate {
                        return Err(ReqvireError::DuplicateRelation(format!(
                            "Duplicate relation '{}' to '{}'",
                            relation_type, normalized_target
                        )));
                    }
                    element.add_relation(relation);
                } else if !trimmed.is_empty() {
                    return Err(ReqvireError::InvalidRelationFormat(
                        format!("Invalid relations format: '{}'. Expected format: '  * relationType: [Text](link)'\n{}", trimmed, get_element_example())
                    ));
                }
            }

        // Parse contract_bindings
        } else if current_subsection == SubSection::ContractBinding {
            if let Some(element) = &mut current_element {
                if trimmed.starts_with("* ") || trimmed.starts_with("- ") {
                    match utils::parse_contract_bindings_line(trimmed) {
                        Ok(href) => {
                            if !href.contains('#') {
                                return Err(ReqvireError::InvalidContractBindingFormat(
                                    format!(
                                        "Invalid contract bindings target '{}'. Contract Bindings entries must use reusable element identifiers in the form 'file.md#element-id' or '#element-id'.",
                                        href
                                    )
                                ));
                            }

                            // Element identifier - use same normalization as relations
                            let git_root = crate::git_commands::get_git_root_dir()?;
                            let normalized = if href.starts_with('#') {
                                // Same-file reference: just prepend file_path
                                format!("{}{}", file_path, href)
                            } else {
                                // Cross-file reference: normalize relative to file's directory
                                let file_parent =
                                    Path::new(file_path).parent().ok_or_else(|| {
                                        ReqvireError::PathError(format!(
                                            "Cannot determine parent directory of '{}'",
                                            file_path
                                        ))
                                    })?;
                                let base_path = git_root.join(file_parent);
                                utils::normalize_identifier(&href, &base_path)?
                            };
                            let target = ContractBindingTarget::ElementIdentifier(normalized);

                            // Check for duplicate contract_bindings
                            if element
                                .contract_bindings
                                .iter()
                                .any(|a| a.target == target)
                            {
                                return Err(ReqvireError::DuplicateContractBinding(format!(
                                    "Duplicate contract_bindings '{}'",
                                    href
                                )));
                            }
                            element
                                .contract_bindings
                                .push(ContractBindingEntry {
                                    target,
                                    content_hash: None,
                                });
                        }
                        Err(e) => {
                            return Err(ReqvireError::InvalidContractBindingFormat(format!(
                                "Invalid contract_bindings format '{}': {}.\n{}",
                                trimmed,
                                e,
                                get_element_example()
                            )));
                        }
                    }
                } else if !trimmed.is_empty() {
                    return Err(ReqvireError::InvalidContractBindingFormat(format!(
                        "Invalid contract_bindings format: '{}'. Expected format: '  * [Text](link)'\n{}",
                        trimmed,
                        get_element_example()
                    )));
                }
            }
        }
    }

    // Finalize element
    if let Some(mut element) = current_element {
        if !found_header {
            return Err(ReqvireError::InvalidMarkdownStructure(format!(
                "Element must start with ### header.\n{}",
                get_element_example()
            )));
        }
        element.freeze_content();
        Ok(element)
    } else {
        Err(ReqvireError::InvalidMarkdownStructure(format!(
            "No element found in markdown string.\n{}",
            get_element_example()
        )))
    }
}

/// Detects the model file type from the first H1 heading.
pub fn detect_model_file_type(content: &str) -> ModelFileType {
    for line in content.lines() {
        let trimmed = line.trim();
        // Skip empty lines and non-heading content before first H1
        if trimmed.is_empty() {
            continue;
        }
        // Check if this is an H1 heading
        if trimmed.starts_with("# ") {
            return match trimmed {
                ELEMENTS_HEADER => ModelFileType::Elements,
                SINGLE_ELEMENT_HEADER => ModelFileType::SingleElement,
                _ => ModelFileType::Unsupported,
            };
        }
        // If we hit any other H1 or content that's not a heading, keep looking
        // (allow frontmatter, comments, etc. before the heading)
        if !trimmed.starts_with('#') && !trimmed.starts_with("---") && !trimmed.starts_with("<!--")
        {
            // Non-header, non-frontmatter content before H1 - skip but continue looking
            continue;
        }
    }
    ModelFileType::Unsupported
}

#[derive(Debug, Clone)]
pub struct ParsedSingleElementContract {
    pub element_type: ElementType,
    pub define_targets: Vec<String>,
}

/// Parses a `# Element` contract document and extracts metadata/define relations.
pub fn parse_single_element_contract(
    file: &str,
    content: &str,
    file_path: &Path,
) -> Result<ParsedSingleElementContract, ReqvireError> {
    if detect_model_file_type(content) != ModelFileType::SingleElement {
        return Err(ReqvireError::InvalidMarkdownStructure(format!(
            "File '{}' is not a supported single-element contract. First H1 must be '{}'.",
            file, SINGLE_ELEMENT_HEADER
        )));
    }

    let mut current_section: Option<&str> = None;
    let mut metadata_type: Option<ElementType> = None;
    let mut define_targets: Vec<String> = Vec::new();
    let mut seen_metadata = false;
    let mut seen_relations = false;
    let mut seen_element_name = false;

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with("## ") {
            let section = trimmed.strip_prefix("## ").unwrap_or(trimmed).trim();
            current_section = Some(section);
            if section.eq_ignore_ascii_case("Metadata") {
                seen_metadata = true;
            } else if section.eq_ignore_ascii_case("Relations") {
                seen_relations = true;
            } else if !section.eq_ignore_ascii_case(CONTRACT_BINDINGS_SECTION) {
                // Dynamic element name section header: `## <Element Name>`
                seen_element_name = true;
            }
            continue;
        }

        match current_section {
            Some(section) if section.eq_ignore_ascii_case("Metadata") => {
                if let Some((key, value)) = utils::parse_metadata_line(trimmed) {
                    if key.eq_ignore_ascii_case("type") {
                        metadata_type = Some(ElementType::from_metadata(&value));
                    }
                } else {
                    return Err(ReqvireError::InvalidMetadataFormat(format!(
                        "Invalid metadata format in '{}', line {}: '{}'. Expected '* key: value'.",
                        file,
                        line_num + 1,
                        trimmed
                    )));
                }
            }
            Some(section) if section.eq_ignore_ascii_case("Relations") => {
                if trimmed.starts_with("* ") {
                    let (relation_type, (_text, link)) = utils::parse_relation_line(trimmed)
                        .map_err(|_| ReqvireError::InvalidRelationFormat(format!(
                            "Invalid relation format in '{}', line {}: '{}'. Expected '* relationType: [Text](link)'.",
                            file,
                            line_num + 1,
                            trimmed
                        )))?;

                    if !matches!(relation_type.as_str(), "define" | "refine") {
                        return Err(ReqvireError::InvalidRelationFormat(format!(
                            "Single-element contract '{}' can only contain 'define' relations. Found '{}' at line {}.",
                            file, relation_type, line_num + 1
                        )));
                    }

                    let final_link = if link.starts_with('#') {
                        format!("{}{}", file, link)
                    } else {
                        link
                    };
                    let file_folder = file_path.parent().ok_or_else(|| {
                        ReqvireError::PathError(format!(
                            "Cannot determine parent directory for '{}'",
                            file_path.display()
                        ))
                    })?;
                    let normalized_target = utils::normalize_identifier(&final_link, file_folder)?;
                    define_targets.push(normalized_target);
                } else {
                    return Err(ReqvireError::InvalidRelationFormat(format!(
                        "Invalid relation entry in '{}', line {}: '{}'.",
                        file,
                        line_num + 1,
                        trimmed
                    )));
                }
            }
            _ => {}
        }
    }

    if !seen_metadata || !seen_relations || !seen_element_name {
        return Err(ReqvireError::InvalidMarkdownStructure(format!(
            "Single-element contract '{}' must include '## Metadata', '## Relations', and '## <Element Name>' sections.",
            file
        )));
    }

    let element_type = metadata_type.ok_or_else(|| {
        ReqvireError::InvalidMetadataFormat(format!(
            "Single-element contract '{}' must define metadata type.",
            file
        ))
    })?;

    if !element_type.is_contract() {
        return Err(ReqvireError::IncompatibleElementTypes(format!(
            "Single-element contract '{}' must use a contract type (constraint, behavior, specification).",
            file
        )));
    }

    if define_targets.is_empty() {
        return Err(ReqvireError::InvalidRelationFormat(format!(
            "Single-element contract '{}' must define at least one 'define' relation.",
            file
        )));
    }

    Ok(ParsedSingleElementContract {
        element_type,
        define_targets,
    })
}

fn parse_single_element_file(
    file: &str,
    content: &str,
    file_path: &Path,
    _git_commit: Option<&str>,
) -> (Vec<Element>, Vec<ReqvireError>, String) {
    let mut errors = Vec::new();
    let mut element_content = String::new();
    let mut element_relations: Vec<Relation> = Vec::new();
    let mut element_contract_bindings: Vec<ContractBindingEntry> = Vec::new();
    let mut metadata: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    enum DocSection {
        None,
        Metadata,
        Relations,
        ContractBinding,
        Document,
    }
    let mut section = DocSection::None;
    let mut seen_metadata = false;
    let mut seen_element_name = false;
    let mut element_name: Option<String> = None;
    let mut in_document = false;

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Skip header
        if line_num == 0 && trimmed == SINGLE_ELEMENT_HEADER {
            continue;
        }

        // Any content after `## <Element Name>` is treated as document body, including any header levels.
        if in_document {
            element_content.push_str(line);
            element_content.push('\n');
            continue;
        }

        if trimmed.starts_with("## ") {
            let sec = trimmed.trim_start_matches("## ").trim();
            section = match sec {
                "Metadata" => {
                    seen_metadata = true;
                    DocSection::Metadata
                }
                "Relations" => DocSection::Relations,
                CONTRACT_BINDINGS_SECTION => DocSection::ContractBinding,
                _ => {
                    // Dynamic element name section header (e.g., `## Change Propagation`)
                    seen_element_name = true;
                    element_name = Some(sec.to_string());
                    in_document = true;
                    DocSection::Document
                }
            };
            continue;
        }

        match section {
            DocSection::Metadata => {
                if trimmed.is_empty() {
                    continue;
                }
                if let Some((key, value)) = utils::parse_metadata_line(trimmed) {
                    metadata.insert(key, value);
                } else {
                    errors.push(ReqvireError::InvalidMetadataFormat(format!(
                        "Invalid metadata format in single-element file '{}', line {}: '{}'",
                        file,
                        line_num + 1,
                        trimmed
                    )));
                }
            }
            DocSection::Relations => {
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed.starts_with("* ") {
                    match utils::parse_relation_line(trimmed) {
                        Ok((relation_type, (text, link))) => {
                            let final_link = if link.starts_with('#') {
                                format!("{}{}", file, link)
                            } else {
                                link
                            };
                            if let Some(file_folder) = file_path.parent() {
                                match utils::normalize_identifier(&final_link, file_folder) {
                                    Ok(normalized_target) => {
                                        match Relation::new(&relation_type, text, &normalized_target, None) {
                                            Ok(rel) => {
                                                if !element_relations.iter().any(|r| r.relation_type == rel.relation_type && r.target == rel.target) {
                                                    element_relations.push(rel);
                                                } else {
                                                    errors.push(ReqvireError::DuplicateRelation(format!(
                                                        "Duplicate relation '{}' to '{}' in single-element file '{}' (line {})",
                                                        relation_type, normalized_target, file, line_num + 1
                                                    )));
                                                }
                                            }
                                            Err(_) => errors.push(ReqvireError::UnsupportedRelationType(format!(
                                                "'{}' in single-element file '{}', line {}. Valid types: {}",
                                                relation_type,
                                                file,
                                                line_num + 1,
                                                relation::supported_relation_types_list()
                                            ))),
                                        }
                                    }
                                    Err(e) => errors.push(ReqvireError::InvalidIdentifier(format!(
                                        "Failed to normalize relation target in single-element file '{}', line {}: {}",
                                        file,
                                        line_num + 1,
                                        e
                                    ))),
                                }
                            }
                        }
                        Err(_) => errors.push(ReqvireError::InvalidRelationFormat(format!(
                            "Invalid relation format in single-element file '{}', line {}: '{}'",
                            file,
                            line_num + 1,
                            trimmed
                        ))),
                    }
                } else {
                    errors.push(ReqvireError::InvalidRelationFormat(format!(
                        "Invalid relations entry in single-element file '{}', line {}: '{}'",
                        file,
                        line_num + 1,
                        trimmed
                    )));
                }
            }
            DocSection::ContractBinding => {
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed.starts_with("* ") || trimmed.starts_with("- ") {
                    match utils::parse_contract_bindings_line(trimmed) {
                        Ok(href) => {
                            if !href.contains('#') {
                                errors.push(ReqvireError::InvalidContractBindingFormat(format!(
                                    "Invalid contract bindings identifier in single-element file '{}', line {}: '{}'. Contract Bindings entries must use reusable element identifiers in the form 'file.md#element-id' or '#element-id'.",
                                    file,
                                    line_num + 1,
                                    href
                                )));
                                continue;
                            }

                            let href_to_normalize = if href.starts_with('#') {
                                format!("{}{}", file, href)
                            } else {
                                href.clone()
                            };
                            let file_dir = file_path
                                .parent()
                                .unwrap_or_else(|| Path::new("."))
                                .to_path_buf();
                            let target = match utils::normalize_identifier(
                                &href_to_normalize,
                                &file_dir,
                            ) {
                                Ok(normalized) => ContractBindingTarget::ElementIdentifier(normalized),
                                Err(e) => {
                                    errors.push(ReqvireError::InvalidContractBindingFormat(format!(
                                        "Invalid contract_bindings identifier in single-element file '{}', line {}: {}",
                                        file,
                                        line_num + 1,
                                        e
                                    )));
                                    continue;
                                }
                            };
                            let content_hash = None;

                            if !element_contract_bindings.iter().any(|a| a.target == target) {
                                element_contract_bindings.push(ContractBindingEntry {
                                    target,
                                    content_hash,
                                });
                            } else {
                                errors.push(ReqvireError::DuplicateContractBinding(format!(
                                    "Duplicate contract_bindings '{}' in single-element file '{}' (line {})",
                                    href,
                                    file,
                                    line_num + 1
                                )));
                            }
                        }
                        Err(e) => errors.push(ReqvireError::InvalidContractBindingFormat(format!(
                            "Invalid contract_bindings in single-element file '{}', line {}: {}",
                            file,
                            line_num + 1,
                            e
                        ))),
                    }
                } else {
                    errors.push(ReqvireError::InvalidContractBindingFormat(format!(
                        "Invalid contract_bindings entry in single-element file '{}', line {}: '{}'",
                        file,
                        line_num + 1,
                        trimmed
                    )));
                }
            }
            DocSection::Document | DocSection::None => {}
        }
    }

    if !seen_metadata || !seen_element_name {
        errors.push(ReqvireError::InvalidMarkdownStructure(format!(
            "Single-element file '{}' must include '## Metadata' and '## <Element Name>' sections.",
            file
        )));
    }

    let final_element_name = element_name.unwrap_or_else(|| {
        file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("document")
            .to_string()
    });
    let normalized_fragment = utils::normalize_fragment(&final_element_name);
    let raw_identifier = format!("{}#{}", file, normalized_fragment);
    let identifier = match file_path.parent() {
        Some(file_folder) => {
            utils::normalize_identifier(&raw_identifier, file_folder).unwrap_or(raw_identifier)
        }
        None => raw_identifier,
    };
    let relative_file = match utils::get_relative_path(file_path) {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(_) => file.to_string(),
    };

    let mut element = Element::new(
        &final_element_name,
        &identifier,
        &relative_file,
        1,
        Some(ElementType::Requirement(RequirementType::System)),
    );
    element.content = element_content;
    metadata.insert("_single_element_format".to_string(), "true".to_string());
    element.metadata = metadata;
    element.set_type_from_metadata();
    element.relations = element_relations;
    element.contract_bindings = element_contract_bindings;
    element.freeze_content();
    element.file_order_index = 0;

    (vec![element], errors, String::new())
}

/// Parses a markdown document and extracts elements with metadata and relations.
/// Returns: (elements, errors, page_content)
/// Only parses files where the first H1 heading is "# Elements" or "# Element".
/// If git_commit is Some, file contract_bindings hashes are computed from the git commit, not working directory.
pub fn parse_elements(
    file: &str,
    content: &str,
    file_path: &Path,
    git_commit: Option<&str>,
) -> (Vec<Element>, Vec<ReqvireError>, String) {
    match detect_model_file_type(content) {
        ModelFileType::Elements => {}
        ModelFileType::SingleElement => {
            return parse_single_element_file(file, content, file_path, git_commit);
        }
        ModelFileType::Unsupported => {
            debug!("Skipping file {} - unsupported model file header", file);
            return (Vec::new(), Vec::new(), String::new());
        }
    }

    let mut elements = Vec::new();
    let mut current_element: Option<Element> = None;
    let mut errors = Vec::new();
    let mut seen_identifiers = HashSet::new();
    let mut skip_current_element = false;
    let mut seen_subsections = HashSet::new();
    let mut in_details_block = false;

    let mut current_subsection = SubSection::Other("".to_string());

    // Content tracking variables
    let mut page_content = String::new();

    // File element order tracking
    let mut file_element_counter: usize = 0;

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        if in_details_block {
            if !skip_current_element {
                if let Some(element) = &mut current_element {
                    element.add_content(&format!("{}\n", line));
                }
            }

            if trimmed.starts_with("</details>") {
                in_details_block = false;
            }

            continue; // Skip any further processing while in <details>
        } else if trimmed == "---" {
            current_subsection = SubSection::Other("".to_string());
        } else if trimmed.starts_with("<details") {
            // Start of details block - set flag and add content if in element
            in_details_block = true;
            if let Some(element) = &mut current_element {
                if !skip_current_element {
                    element.add_content(&format!("{}\n", line));
                }
            }
            continue;
        } else if trimmed.starts_with("## ") {
            // Section headers (## ) are not allowed - report syntax error
            let section_name = trimmed.strip_prefix("## ").unwrap_or(trimmed).trim();
            errors.push(ReqvireError::InvalidMarkdownStructure(format!(
                "Section headers (## ) are not allowed in specification files. Found '## {}' at line {} in file '{}'. Use ### for element headers instead.",
                section_name,
                line_num + 1,
                file_path.display()
            )));
            continue;
        } else if trimmed.starts_with("### ") {
            current_subsection = SubSection::Requirement;

            if let Some(mut element) = current_element.take() {
                if !skip_current_element {
                    element.freeze_content();
                    elements.push(element);
                }
            }

            skip_current_element = false;
            seen_subsections.clear();

            let element_name = trimmed
                .strip_prefix("### ")
                .unwrap_or(trimmed)
                .trim()
                .to_string();

            match file_path.parent() {
                Some(file_folder) => {
                    let identifier = format!("{}#{}", file, element_name);

                    match utils::normalize_identifier(&identifier, file_folder) {
                        Ok(identifier) => {
                            let relative_file = match utils::get_relative_path(file_path) {
                                Ok(path) => path,
                                Err(err) => {
                                    debug!("Error: {}", &err);
                                    skip_current_element = true;
                                    errors.push(err);
                                    continue;
                                }
                            };

                            seen_identifiers.insert(identifier.clone());

                            // Default element type is always 'requirement' (location-independent)
                            let element_type = ElementType::Requirement(RequirementType::System);

                            let mut new_element = Element::new(
                                &element_name,
                                &identifier,
                                &relative_file.to_string_lossy(),
                                line_num + 1, // line_number is 1-indexed
                                Some(element_type),
                            );

                            // Set file order index
                            new_element.file_order_index = file_element_counter;
                            file_element_counter += 1;

                            current_element = Some(new_element);
                            debug!("Found element: {}", element_name);
                        }
                        Err(e) => {
                            let msg = format!(
                                "Failed to normalize identifier for '{}': {} (file: {}, line {})",
                                element_name,
                                e,
                                file_path.display(),
                                line_num + 1
                            );
                            errors.push(ReqvireError::InvalidIdentifier(msg.clone()));
                            debug!("Error: {}", msg);
                            skip_current_element = true;
                        }
                    }
                }
                None => {
                    let msg = format!(
                        "Failed to normalize identifier for '{}': {} (file: {}, line {})",
                        element_name,
                        "File folder not accessible.",
                        file_path.display(),
                        line_num + 1
                    );
                    errors.push(ReqvireError::InvalidIdentifier(msg.clone()));
                    debug!("Error: {}", msg);
                    skip_current_element = true;
                }
            }
        } else if trimmed.starts_with("#####")
            && current_element.is_some()
            && current_subsection != SubSection::Details
            && !skip_current_element
        {
            // Level 5+ headers are only allowed inside Details subsection
            let msg = format!(
                "Invalid header level in element '{}': Level 5+ headers (#####+) can only appear inside '#### Details' subsection (file: {}, line {})",
                current_element.as_ref().unwrap().name,
                file_path.display(),
                line_num + 1
            );
            errors.push(ReqvireError::InvalidMarkdownStructure(msg.clone()));
            debug!("Error: {}", msg);
        } else if trimmed.starts_with("#### ") && current_element.is_some() {
            let subsection_name = trimmed[5..].trim();

            if !skip_current_element && is_legacy_contract_bindings_section(subsection_name) {
                let msg = format!(
                    "Legacy subsection '{}' in element '{}' is not supported. Use '#### {}' or run `reqvire migrate --fix`. (file: {}, line {})",
                    subsection_name,
                    current_element.as_ref().unwrap().name,
                    CONTRACT_BINDINGS_SECTION,
                    file_path.display(),
                    line_num + 1
                );
                errors.push(ReqvireError::InvalidMarkdownStructure(msg.clone()));
                debug!("Error: {}", msg);
            }

            let subsection = SubSection::parse(subsection_name);

            if !skip_current_element {
                if !subsection.is_repeatable() && seen_subsections.contains(&subsection) {
                    let msg = format!(
                        "Duplicate subsection '{}' in element '{}' (file: {}, line {})",
                        subsection.name(),
                        current_element.as_ref().unwrap().name,
                        file_path.display(),
                        line_num + 1
                    );
                    errors.push(ReqvireError::DuplicateSubsection(msg.clone()));
                    debug!("Error: {}", msg);
                } else if !subsection.is_repeatable() {
                    seen_subsections.insert(subsection.clone());
                }
            }

            // If transitioning to content-bearing subsection, add the header to content
            if !skip_current_element {
                if let Some(element) = &mut current_element {
                    if subsection.is_content_bearing() {
                        element.add_content(&format!("\n{}\n", line));
                    }
                }
            }

            current_subsection = subsection;
        } else if (current_subsection == SubSection::Requirement
            || current_subsection.is_content_bearing())
            && current_element.is_some()
            && !skip_current_element
        {
            if let Some(element) = &mut current_element {
                if trimmed.starts_with("<details") {
                    in_details_block = true;
                }

                element.add_content(&format!("{}\n", line));
            }
        } else if in_details_block && !skip_current_element {
            // Still inside <details> block under 'Details' subsection
            if let Some(element) = &mut current_element {
                element.add_content(&format!("{}\n", line));
            }
        } else if current_subsection == SubSection::Metadata && !skip_current_element {
            if trimmed.is_empty() {
                continue;
            }
            if let Some(element) = &mut current_element {
                if let Some((key, value)) = utils::parse_metadata_line(trimmed) {
                    element.metadata.insert(key.clone(), value.clone());

                    if key.eq_ignore_ascii_case("type") {
                        element.set_type_from_metadata();
                    }
                } else {
                    let msg = format!(
                        "Element '{}' has invalid metadata format: '{}' (file: {}, line {})",
                        element.name,
                        trimmed,
                        file,
                        line_num + 1
                    );
                    errors.push(ReqvireError::InvalidMetadataFormat(msg.clone()));
                    debug!("Error: {}", msg);
                    current_subsection = SubSection::Other("".to_string());
                }
            }
        } else if current_subsection == SubSection::Relations && !skip_current_element {
            if let Some(element) = &mut current_element {
                if trimmed.starts_with("* ") {
                    match utils::parse_relation_line(trimmed) {
                        Ok((relation_type, (text, link))) => {
                            let final_link = if link.starts_with('#') {
                                format!("{}{}", file, link)
                            } else {
                                link
                            };

                            match file_path.parent() {
                                Some(file_folder) => {
                                    match utils::normalize_identifier(&final_link, file_folder) {
                                        Ok(normalized_target) => {
                                            // element_id will be populated later by GraphRegistry after all elements are registered
                                            match Relation::new(
                                                &relation_type,
                                                text,
                                                &normalized_target,
                                                None,
                                            ) {
                                                Ok(relation) => {
                                                    // Check for duplicate relations (same type and target)
                                                    let is_duplicate =
                                                        element.relations.iter().any(|r| {
                                                            r.relation_type
                                                                == relation.relation_type
                                                                && r.target == relation.target
                                                        });
                                                    if is_duplicate {
                                                        let msg = format!(
                                                            "Duplicate relation '{}' to '{}' in element '{}' (file: {}, line {})",
                                                            relation_type, normalized_target, element.name, file, line_num + 1
                                                        );
                                                        errors.push(
                                                            ReqvireError::DuplicateRelation(
                                                                msg.clone(),
                                                            ),
                                                        );
                                                        debug!("Warning: {}", msg);
                                                    } else {
                                                        element.add_relation(relation);
                                                    }
                                                }
                                                Err(_) => {
                                                    let msg = format!(
                                                        "'{}' in element '{}': (file: {}, line {}). Valid types: {}",
                                                        relation_type, element.name, file, line_num + 1,
                                                        relation::supported_relation_types_list()
                                                    );
                                                    errors.push(
                                                        ReqvireError::UnsupportedRelationType(
                                                            msg.clone(),
                                                        ),
                                                    );
                                                    debug!("Error: {}", msg);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            let msg = format!(
                                                "Failed to normalize identifier for '{}': {} (file: {}, line {})",
                                                element.name, e, file, line_num + 1
                                            );
                                            errors
                                                .push(ReqvireError::InvalidIdentifier(msg.clone()));
                                            debug!("Error: {}", msg);
                                        }
                                    }
                                }
                                None => {
                                    let msg = format!(
                                        "Failed to normalize identifier for '{}': {} (file: {}, line {})",
                                        trimmed,
                                        "File folder not accessible.",
                                        file_path.display(),
                                        line_num + 1
                                    );
                                    errors.push(ReqvireError::InvalidIdentifier(msg.clone()));
                                    debug!("Error: {}", msg);
                                }
                            }
                        }
                        Err(_) => {
                            let msg = format!(
                                "Element '{}' has invalid relation format: '{}'. (file: {}, line {})",
                                element.name, trimmed, file, line_num + 1
                            );
                            errors.push(ReqvireError::UnsupportedRelationType(msg.clone()));
                            debug!("Error: {}", msg);
                        }
                    }
                } else if trimmed.is_empty() {
                    // Ignore
                } else {
                    let msg = format!(
                        "Element '{}' has invalid relations format: '{}' (file: {}, line {})",
                        element.name,
                        trimmed,
                        file,
                        line_num + 1
                    );
                    errors.push(ReqvireError::InvalidRelationFormat(msg.clone()));
                    debug!("Error: {}", msg);
                    current_subsection = SubSection::Other("".to_string());
                }
            }
        } else if current_subsection == SubSection::ContractBinding && !skip_current_element {
            // Parse Contract Bindings subsection
            // Format: * [text](identifier) where identifier is:
            // - Same-file identifier (#fragment)
            // - Cross-file identifier (file.md#fragment)
            if let Some(element) = &mut current_element {
                if trimmed.starts_with("* ") || trimmed.starts_with("- ") {
                    match utils::parse_contract_bindings_line(trimmed) {
                        Ok(href) => {
                            if !href.contains('#') {
                                let msg = format!(
                                    "Invalid contract bindings identifier in element '{}': '{}' (file: {}, line {}). Contract Bindings entries must use reusable element identifiers in the form 'file.md#element-id' or '#element-id'.",
                                    element.name, href, file, line_num + 1
                                );
                                errors.push(ReqvireError::InvalidContractBindingFormat(
                                    msg.clone(),
                                ));
                                debug!("Error: {}", msg);
                                continue;
                            }

                            // This is an element identifier - normalize it like relation targets
                            // For same-file references (starting with #), prepend current file path
                            let href_to_normalize = if href.starts_with('#') {
                                format!("{}{}", file, href)
                            } else {
                                href.clone()
                            };
                            let file_dir = file_path
                                .parent()
                                .unwrap_or_else(|| Path::new("."))
                                .to_path_buf();
                            let (target, content_hash): (
                                ContractBindingTarget,
                                Option<String>,
                            ) = match utils::normalize_identifier(&href_to_normalize, &file_dir) {
                                Ok(normalized) => (
                                    ContractBindingTarget::ElementIdentifier(normalized),
                                    None,
                                ),
                                Err(e) => {
                                    let msg = format!(
                                            "Invalid contract_bindings identifier in element '{}': {} (file: {}, line {})",
                                            element.name, e, file, line_num + 1
                                        );
                                    errors.push(ReqvireError::InvalidContractBindingFormat(
                                        msg.clone(),
                                    ));
                                    debug!("Error: {}", msg);
                                    continue;
                                }
                            };

                            // Check for duplicates
                            if !element
                                .contract_bindings
                                .iter()
                                .any(|a| a.target == target)
                            {
                                element
                                    .contract_bindings
                                    .push(ContractBindingEntry {
                                        target,
                                        content_hash,
                                    });
                            } else {
                                let msg =
                                    format!(
                                    "Duplicate contract_bindings '{}' in element '{}' (file: {}, line {})",
                                    href, element.name, file, line_num + 1
                                );
                                errors.push(ReqvireError::DuplicateContractBinding(
                                    msg.clone(),
                                ));
                                debug!("Warning: {}", msg);
                            }
                        }
                        Err(e) => {
                            let msg = format!(
                                "Invalid contract_bindings in element '{}': {} (file: {}, line {})",
                                element.name,
                                e,
                                file,
                                line_num + 1
                            );
                            errors.push(ReqvireError::InvalidContractBindingFormat(
                                msg.clone(),
                            ));
                            debug!("Error: {}", msg);
                        }
                    }
                } else if !trimmed.is_empty() {
                    // Non-empty line that's not a bullet point - end of Contract Bindings subsection
                    current_subsection = SubSection::Other("".to_string());
                }
                // Empty lines are ignored within Contract Bindings subsection
            }
        } else if matches!(current_subsection, SubSection::Other(_)) {
            // Accumulate page content: everything outside of elements, but skip the # Elements title
            if !trimmed.starts_with("# ") {
                page_content.push_str(line);
                page_content.push('\n');
            }
        }
    }

    // Final element
    if let Some(mut element) = current_element.take() {
        if !skip_current_element {
            element.freeze_content();
            elements.push(element);
        }
    }

    (elements, errors, page_content.trim().to_string())
}
