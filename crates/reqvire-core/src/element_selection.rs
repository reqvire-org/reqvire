//! Existing-element lookup within one captured model context. Authored names,
//! filters and resource/semantic domains do not use this resolver.

use crate::error::ReqvireError;
use std::collections::BTreeSet;

/// Resolve both interpretations before choosing a subject. The caller supplies
/// the complete context, including elements outside its permitted type family;
/// type validation follows selection, so it cannot hide ambiguity.
pub fn resolve_reference<'a>(
    value: &str,
    argument: &str,
    candidates: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> Result<Option<&'a str>, ReqvireError> {
    let matches: BTreeSet<_> = candidates
        .into_iter()
        .filter(|(name, identifier)| *name == value || *identifier == value)
        .map(|(_, identifier)| identifier)
        .collect();
    if matches.len() > 1 {
        return Err(ReqvireError::AmbiguousElementSelection(format!(
            "{argument} '{value}' selects different elements in the selected model context: {}",
            matches.into_iter().collect::<Vec<_>>().join(", ")
        )));
    }
    Ok(matches.into_iter().next())
}

pub fn require_consistent(
    first: (&str, &str, &str),
    second: (&str, &str, &str),
) -> Result<(), ReqvireError> {
    if first.2 != second.2 {
        return Err(ReqvireError::ConflictingElementSelectors(format!(
            "{} '{}' selects '{}', but {} '{}' selects '{}' in the selected model context",
            first.0, first.1, first.2, second.0, second.1, second.2
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_identifiers_and_same_subject_interpretations() {
        let candidates = [("Subject", "nested/Model.md#subject"), ("self", "self")];
        for value in ["Subject", "nested/Model.md#subject"] {
            assert_eq!(
                resolve_reference(value, "read name", candidates).expect("resolved"),
                Some("nested/Model.md#subject")
            );
        }
        assert_eq!(
            resolve_reference("self", "read name", candidates).expect("resolved once"),
            Some("self")
        );
        for value in ["subject", " Subject ", "#subject", "other/Model.md#subject"] {
            assert_eq!(
                resolve_reference(value, "read name", candidates).expect("no match"),
                None
            );
        }
    }

    #[test]
    fn ambiguity_is_deterministic_and_names_both_candidates() {
        let candidates = [
            ("Subject", "Model.md#subject"),
            ("Model.md#subject", "Other.md#collision"),
        ];
        for values in [candidates, [candidates[1], candidates[0]]] {
            let error = resolve_reference("Model.md#subject", "rename element_name", values)
                .expect_err("ambiguous");
            assert_eq!(error.diagnostic_code(), "ambiguous_element_selection");
            let message = error.to_string();
            assert!(
                message.contains("rename element_name")
                    && message.contains("Model.md#subject")
                    && message.contains("Other.md#collision")
            );
        }
    }

    #[test]
    fn multiple_explicit_selectors_must_agree() {
        require_consistent(
            ("name", "Subject", "Model.md#subject"),
            ("identifier", "Model.md#subject", "Model.md#subject"),
        )
        .expect("consistent");
        let error = require_consistent(
            ("name", "Subject", "Model.md#subject"),
            ("identifier", "Other.md#subject", "Other.md#subject"),
        )
        .expect_err("conflict");
        assert_eq!(error.diagnostic_code(), "conflicting_element_selectors");
    }

    #[test]
    fn captured_registry_selection_preserves_literal_and_identifier_only_domains() {
        use crate::element::{Element, ElementType};
        use crate::graph_registry::GraphRegistry;
        let mut first = GraphRegistry::new();
        for (name, id, kind) in [
            ("Subject", "Model.md#subject", ElementType::Capability),
            (
                "Model.md#subject",
                "Other.md#collision",
                ElementType::Concept,
            ),
        ] {
            let element = Element::new(name, id, "Model.md", 1, Some(kind));
            first
                .register_element(element, "Model.md")
                .expect("registered");
        }
        assert_eq!(
            first
                .select_element("Subject", "read name")
                .expect("unique name")
                .identifier,
            "Model.md#subject"
        );
        assert_eq!(
            first
                .select_element("Other.md#collision", "read name")
                .expect("unique identifier")
                .name,
            "Model.md#subject"
        );
        assert_eq!(
            first
                .get_element_by_name("Model.md#subject")
                .expect("literal lookup")
                .identifier,
            "Other.md#collision"
        );
        assert!(first
            .select_element("Model.md#subject", "read name")
            .is_err());
        let explicit = crate::operations::read_element(&first, Some("Model.md#subject"), None)
            .expect("explicit identifier remains unambiguous");
        assert_eq!(explicit.name, "Subject");
        assert!(crate::operations::read_element(&first, Some("Subject"), None).is_err());
        crate::operations::read_element(
            &first,
            Some("Other.md#collision"),
            Some("Other.md#collision"),
        )
        .expect("same selected subject");
        let conflict =
            crate::operations::read_element(&first, Some("Other.md#collision"), Some("Subject"))
                .expect_err("contradictory selectors");
        assert_eq!(conflict.diagnostic_code(), "conflicting_element_selectors");
        let second = GraphRegistry::new();
        assert!(
            second.select_element("Subject", "read name").is_err(),
            "registry selection cannot escape its snapshot"
        );
    }
}
