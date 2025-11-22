use crate::element;
use crate::relation;
use crate::error::ReqvireError;
use globset::{Glob, GlobMatcher};
use regex::Regex;


pub struct Filters {
    file_glob:    Option<GlobMatcher>,
    name_re:      Option<Regex>,
    type_pat:     Option<String>,
    content_re:   Option<Regex>,
    not_verified: bool,
    not_satisfied: bool,
    has_attachments: bool,
    attachment_glob: Option<GlobMatcher>,
}

impl Filters {
    /// Builds a Filters struct, or returns a ReqvireError::InvalidGlob / InvalidRegex
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
            let glob =Glob::new(pat)
                .map_err(|e| ReqvireError::InvalidGlob(e.to_string()))?
                .compile_matcher();
            Ok(glob)

        }

        let file_glob = file.map(|p| compile_glob(p)).transpose()?;
        let name_re = match name_regex {
            Some(r) => Some(Regex::new(r).map_err(|e| ReqvireError::InvalidRegex(e.to_string()))?),
            None => None,
        };
        let type_pat = typ.map(|s| s.to_lowercase());
        let content_re = match content {
            Some(r) => Some(Regex::new(r).map_err(|e| ReqvireError::InvalidRegex(e.to_string()))?),
            None => None,
        };
        let attachment_glob = attachment.map(|p| compile_glob(p)).transpose()?;

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
            let filter_type = element::ElementType::from_metadata(tp);
            if &e.element_type != &filter_type {
                return false;
            }
        }
        // 5) content regex
        if let Some(re) = &self.content_re {
            let text = e.content.clone();
            if !re.is_match(&text) {
                return false;
            }
        }

        // Pre-compute verify/satisfy counts for later filters
        let verified_count = e.relations.iter()
            .filter(|r| relation::is_verification_relation(r.relation_type))
            .count();

        let satisfied_count = e.relations.iter()
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
            let has_matching_attachment = e.attachments.iter()
                .any(|a| g.is_match(a.file_path.to_string_lossy().as_ref()));
            if !has_matching_attachment {
                return false;
            }
        }

        // passed all filters
        true
    }
}
