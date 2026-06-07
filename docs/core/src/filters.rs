use crate::element;
use crate::error::ReqvireError;
use crate::relation;
use globset::{Glob, GlobMatcher};
use regex::Regex;

pub struct Filters {
    file_glob: Option<GlobMatcher>,
    name_re: Option<Regex>,
    type_pat: Option<String>,
    content_re: Option<Regex>,
    not_verified: bool,
    not_satisfied: bool,
    has_attachments: bool,
    attachment_glob: Option<GlobMatcher>,
}

impl Filters {
    /// Builds a Filters struct, or returns a ReqvireError::InvalidGlob / InvalidRegex
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        file: Option<&str>,
        name_regex: Option<&str>,
        typ: Option<&str>,
        content: Option<&str>,
        is_not_verified: bool,
        is_not_satisfied: bool,
        has_attachments: bool,
        attachment: Option<&str>,
    ) -> Result<Self, ReqvireError> {
        fn compile_glob(pat: &str) -> Result<GlobMatcher, ReqvireError> {
            let glob = Glob::new(pat)
                .map_err(|e| ReqvireError::InvalidGlob(e.to_string()))?
                .compile_matcher();
            Ok(glob)
        }

        let file_glob = file.map(compile_glob).transpose()?;
        let name_re = match name_regex {
            Some(r) => Some(Regex::new(r).map_err(|e| ReqvireError::InvalidRegex(e.to_string()))?),
            None => None,
        };
        // Validate element type if provided
        let type_pat = if let Some(t) = typ {
            let lowercase = t.to_lowercase();
            if !element::is_valid_element_type(&lowercase) {
                return Err(ReqvireError::ProcessError(format!(
                    "Invalid element type '{}'. Valid types: {}",
                    t,
                    element::element_types_help()
                )));
            }
            Some(lowercase)
        } else {
            None
        };
        let content_re = match content {
            Some(r) => Some(Regex::new(r).map_err(|e| ReqvireError::InvalidRegex(e.to_string()))?),
            None => None,
        };
        let attachment_glob = attachment.map(compile_glob).transpose()?;

        Ok(Filters {
            file_glob,
            name_re,
            type_pat,
            content_re,
            not_verified: is_not_verified,
            not_satisfied: is_not_satisfied,
            has_attachments,
            attachment_glob,
        })
    }

    /// Returns true if this element passes *all* of the user's filters.
    pub fn matches(&self, e: &element::Element) -> bool {
        // 1) file glob
        if let Some(g) = &self.file_glob {
            if !g.is_match(&e.file_path) {
                return false;
            }
        }
        // 2) name regex
        if let Some(re) = &self.name_re {
            if !re.is_match(&e.name) {
                return false;
            }
        }
        // 3) type filter
        if let Some(tp) = &self.type_pat {
            // Handle "other-TYPENAME" pattern for custom types
            if let Some(custom_type_name) = tp.strip_prefix("other-") {
                // Extract the custom type name after "other-"
                match &e.element_type {
                    element::ElementType::Other(actual_name) => {
                        if actual_name.to_lowercase() != custom_type_name {
                            return false;
                        }
                    }
                    _ => return false, // Not an Other type
                }
            } else {
                let filter_type = element::ElementType::from_metadata(tp);
                if e.element_type != filter_type {
                    return false;
                }
            }
        }
        // 5) content regex
        if let Some(re) = &self.content_re {
            if !re.is_match(&e.content) {
                return false;
            }
        }

        // Pre-compute verify/satisfy counts for later filters
        let verified_count = e
            .relations
            .iter()
            .filter(|r| relation::is_verification_relation(r.relation_type))
            .count();

        let satisfied_count = e
            .relations
            .iter()
            .filter(|r| relation::is_satisfaction_relation(r.relation_type))
            .count();

        // 6) not_verified: exclude any element that *has* a verified relation
        if self.not_verified && verified_count > 0 {
            return false;
        }
        // 7) not_satisfied: exclude any element that *has* a satisfied relation
        if self.not_satisfied && satisfied_count > 0 {
            return false;
        }
        // 8) has_attachments: only include elements that have at least one attachment
        if self.has_attachments && e.attachments.is_empty() {
            return false;
        }
        // 9) attachment_glob: only include elements with attachments matching the glob
        if let Some(g) = &self.attachment_glob {
            let has_matching_attachment = e
                .attachments
                .iter()
                .any(|a| g.is_match(a.target.as_str().as_str()));
            if !has_matching_attachment {
                return false;
            }
        }

        // passed all filters
        true
    }
}
