use super::GraphRegistry;
use crate::element::Element;
use crate::error::{
    DiagnosticLink, DiagnosticTarget, ErrorContext, RepairGuidance, ReqvireError,
    ValidationDiagnostic,
};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::ops::Range;
use std::path::Path;

const CODE: &str = "model-element-link-in-prose";

/// Structured subsections have their own syntax and validation contracts.
/// Use Markdown heading events so literal headings inside code stay literal.
fn structured_ranges(content: &str) -> Vec<Range<usize>> {
    let mut headings = Vec::new();
    let mut heading = None;
    for (event, range) in Parser::new_ext(content, Options::all()).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                heading = Some((level, range.start, String::new()))
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some((_, _, title)) = &mut heading {
                    title.push_str(&text);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(entry) = heading.take() {
                    headings.push(entry);
                }
            }
            _ => {}
        }
    }
    headings
        .iter()
        .enumerate()
        .filter_map(|(index, (level, start, title))| {
            if *level != HeadingLevel::H4
                || !matches!(
                    title.as_str(),
                    "Concept References" | "Ontology" | "External Ontology" | "Shapes" | "Query"
                )
            {
                return None;
            }
            let end = headings[index + 1..]
                .iter()
                .find(|(next, _, _)| *next <= *level)
                .map_or(content.len(), |(_, offset, _)| *offset);
            Some(*start..end)
        })
        .collect()
}

fn step(
    message: impl Into<String>,
    section: Option<&str>,
    example: Option<String>,
) -> RepairGuidance {
    RepairGuidance {
        message: message.into(),
        section: section.map(str::to_owned),
        example,
    }
}

impl GraphRegistry {
    /// Evaluate the complete supplied registry, including otherwise untouched prose.
    /// This deliberately has no disk, cache, or automatic-rewrite dependency.
    pub(crate) fn validate_model_element_prose_links(&self) -> Vec<ReqvireError> {
        let mut elements: Vec<_> = self.nodes.values().map(|node| &node.element).collect();
        elements.sort_by(|a, b| a.identifier.cmp(&b.identifier));
        let mut errors = Vec::new();
        for source in elements {
            let excluded = structured_ranges(&source.content);
            let mut link = None;
            for (event, range) in
                Parser::new_ext(&source.content, Options::all()).into_offset_iter()
            {
                match event {
                    Event::Start(Tag::Link { dest_url, .. }) => {
                        if excluded.iter().any(|span| span.contains(&range.start)) {
                            continue;
                        }
                        if let Some(target) = self.prose_link_target(source, &dest_url) {
                            link = Some((target, dest_url.to_string(), range.start, String::new()));
                        }
                    }
                    Event::Text(text) | Event::Code(text) => {
                        if let Some((_, _, _, label)) = &mut link {
                            label.push_str(&text);
                        }
                    }
                    Event::SoftBreak | Event::HardBreak => {
                        if let Some((_, _, _, label)) = &mut link {
                            label.push(' ');
                        }
                    }
                    Event::End(TagEnd::Link) => {
                        if let Some((target, destination, offset, label)) = link.take() {
                            errors.push(self.prose_link_error(
                                source,
                                target,
                                &destination,
                                &label,
                                offset,
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        errors
    }

    fn prose_link_target<'a>(&'a self, source: &Element, destination: &str) -> Option<&'a Element> {
        if crate::utils::is_external_url(destination) {
            return None;
        }
        let destination = destination.strip_prefix('#').map_or_else(
            || destination.to_string(),
            |fragment| {
                format!(
                    "{}#{}",
                    source.file_path,
                    crate::utils::normalize_fragment(fragment)
                )
            },
        );
        if let Some(identifier) =
            self.normalize_relation_identifier_for_source(&source.file_path, &destination)
        {
            return self.get_element(&identifier);
        }
        // A file-only link identifies a registered single-element document.
        if destination.contains('#') {
            return None;
        }
        let base = Path::new(&source.file_path)
            .parent()
            .unwrap_or_else(|| Path::new("."));
        let normalized = crate::utils::normalize_identifier(&destination, base).ok()?;
        self.nodes
            .values()
            .map(|node| &node.element)
            .find(|target| {
                (target.file_path == normalized || target.file_path == destination)
                    && target
                        .metadata
                        .get("_single_element_format")
                        .is_some_and(|value| value == "true")
            })
    }

    fn contract_owner<'a>(&'a self, target: &Element) -> Option<&'a Element> {
        let mut owners: Vec<_> = target
            .relations
            .iter()
            .filter(|relation| relation.relation_type.name == "define")
            .filter_map(|relation| self.get_element(relation.target.link.as_str()))
            .filter(|owner| owner.element_type.is_requirement())
            .collect();
        owners.sort_by(|a, b| a.identifier.cmp(&b.identifier));
        owners.first().copied()
    }

    fn prose_link_error(
        &self,
        source: &Element,
        target: &Element,
        destination: &str,
        label: &str,
        offset: usize,
    ) -> ReqvireError {
        let content_line = source.content[..offset]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count();
        let line_text = source.content.lines().nth(content_line).unwrap_or("");
        let line = source
            .content_source_lines
            .get(content_line)
            .filter(|(_, original)| original == line_text)
            .map_or(source.line_number + content_line + 1, |(line, _)| *line);
        let owner = target
            .element_type
            .is_contract()
            .then(|| self.contract_owner(target))
            .flatten();
        let base = Path::new(&source.file_path)
            .parent()
            .unwrap_or_else(|| Path::new("."));
        let relative_file = pathdiff::diff_paths(&target.file_path, base)
            .unwrap_or_else(|| target.file_path.clone().into());
        let fragment = crate::utils::extract_path_and_fragment(&target.identifier)
            .1
            .unwrap_or(&target.id);
        let relative = if source.file_path == target.file_path {
            format!("#{}", fragment)
        } else {
            format!("{}#{}", relative_file.to_string_lossy(), fragment)
        };
        let template = |section| format!("#### {}\n  * [{}]({})", section, target.name, relative);
        let mut guidance = vec![step(format!(
            "Replace the hyperlink [{}]({}) with readable plain domain wording. Renaming its label or adding a structural declaration alone does not remove the violation. Do not copy the target specification or duplicate its owner's obligation.", label, destination), None, None)];
        if source.element_type.is_ontology() {
            guidance.push(step("Express structural semantics through compatible Turtle vocabulary or ontology imports; use reqvire:mapsToConcept for an applicable structural-to-concept bridge. Ontology elements cannot author Concept References or contract dependency sections.", None, None));
        } else if source.element_type.is_semantic_contract() {
            guidance.push(step("Use the permitted ontology-use or requirement-constraint channels according to the intended meaning and target type. Semantic-contract elements cannot author Concept References or contract dependency sections.", None, None));
        } else if target.element_type.is_concept() {
            guidance.push(step("If the wording denotes this native concept, bind the terminology through Concept References on the source element.", Some("Concept References"), Some(template("Concept References"))));
        } else if target.element_type.is_contract() {
            if source.element_type.is_requirement() && owner.is_some() {
                guidance.push(step("For a review dependency on this requirement-owned contract, use Contract References on the source requirement.", Some("Contract References"), Some(template("Contract References"))));
                guidance.push(step("For an independent implementation obligation, use Contract Bindings instead. These are alternatives: use only one section on a requirement, never both. Decompose meaningful obligations into separate requirements if different dependency kinds are needed; do not infer intent from the hyperlink.", Some("Contract Bindings"), Some(template("Contract Bindings"))));
            } else if source.element_type.is_requirement() {
                guidance.push(step("The target has no known owning requirement in this snapshot. Review its ownership before selecting a valid review or implementation dependency; do not invent an owner or duplicate the target's obligation.", None, None));
            } else {
                let source_owner = self.contract_owner(source);
                let responsible = source_owner.map_or_else(
                    || "the responsible requirement".to_string(),
                    |owner| {
                        format!(
                            "the owning requirement '{}' ({})",
                            owner.name, owner.identifier
                        )
                    },
                );
                guidance.push(step(format!("Declare any contract dependency on {} after reviewing its intent: Contract References for review, or Contract Bindings for an independent implementation obligation. Do not add these sections to this source type, transfer definedBy ownership, or attach hierarchy relations to the contract.", responsible), None, None));
            }
        } else {
            guidance.push(step("If a structural relationship is intended, declare an explicit relation permitted for these source and target types in Relations. Select its meaning from the relation contract; the hyperlink cannot establish intent, ownership, or a new obligation.", Some("Relations"), None));
        }
        let mut diagnostic = ValidationDiagnostic::new(CODE,
            format!("Authored prose hyperlink [{}]({}) resolves to a model element; prose hyperlinks do not declare valid model dependencies.", label, destination))
            .with_context(ErrorContext::new().with_file(&source.file_path).with_line(line)
                .with_element_id(&source.identifier).with_element_type(source.element_type.as_str()));
        diagnostic.target = Some(DiagnosticTarget {
            element_id: target.identifier.clone(),
            element_type: target.element_type.as_str().to_string(),
            owner: owner.map(|element| element.identifier.clone()),
        });
        diagnostic.link = Some(DiagnosticLink {
            text: label.to_string(),
            destination: destination.to_string(),
        });
        diagnostic.guidance = guidance;
        ReqvireError::ModelElementLinkInProse(Box::new(diagnostic))
    }
}
