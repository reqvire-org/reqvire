use crate::graph_registry::GraphRegistry;
use crate::error::ReqvireError;
use serde::Serialize;
use globset::{Glob, GlobMatcher};
use regex::Regex;

#[derive(Serialize)]
pub struct SectionsSummary {
    pub sections: Vec<SectionSummary>,
}

#[derive(Serialize)]
pub struct SectionSummary {
    pub file: String,
    pub name: String,
    pub content: String,
    pub section_order: usize,
}

pub struct SectionsFilters {
    file_glob: Option<GlobMatcher>,
    section_glob: Option<GlobMatcher>,
    content_re: Option<Regex>,
}

impl SectionsFilters {
    pub fn new(
        file: Option<&str>,
        section: Option<&str>,
        content: Option<&str>,
    ) -> Result<Self, ReqvireError> {
        fn compile_glob(pat: &str) -> Result<GlobMatcher, ReqvireError> {
            let glob = Glob::new(pat)
                .map_err(|e| ReqvireError::InvalidGlob(e.to_string()))?
                .compile_matcher();
            Ok(glob)
        }

        let file_glob = file.map(|p| compile_glob(p)).transpose()?;
        let section_glob = section.map(|p| compile_glob(p)).transpose()?;
        let content_re = match content {
            Some(r) => Some(Regex::new(r).map_err(|e| ReqvireError::InvalidRegex(e.to_string()))?),
            None => None,
        };

        Ok(Self {
            file_glob,
            section_glob,
            content_re,
        })
    }

    pub fn matches_file(&self, file_path: &str) -> bool {
        if let Some(ref glob) = self.file_glob {
            return glob.is_match(file_path);
        }
        true
    }

    pub fn matches_section(&self, section_name: &str, section_content: &str) -> bool {
        if let Some(ref glob) = self.section_glob {
            if !glob.is_match(section_name) {
                return false;
            }
        }

        if let Some(ref re) = self.content_re {
            if !re.is_match(section_content) {
                return false;
            }
        }

        true
    }
}

pub fn print_sections_summary(
    registry: &GraphRegistry,
    json_output: bool,
    filters: &SectionsFilters,
) {
    let summary = build_sections_summary(registry, filters);

    if json_output {
        let text = serde_json::to_string_pretty(&summary)
            .expect("failed to serialize sections summary");
        println!("{}", text);
    } else {
        print_sections_text(&summary);
    }
}

fn build_sections_summary(
    registry: &GraphRegistry,
    filters: &SectionsFilters,
) -> SectionsSummary {
    let mut sections = Vec::new();

    // Process sections
    for (section_key, section) in &registry.sections {
        let file_path = &section_key.file_path;
        let section_name = &section_key.section_name;
        let section_content = &section.content;

        if !filters.matches_file(file_path) {
            continue;
        }

        if !filters.matches_section(section_name, section_content) {
            continue;
        }

        sections.push(SectionSummary {
            file: file_path.clone(),
            name: section_name.clone(),
            content: section_content.clone(),
            section_order: section.section_order,
        });
    }

    // Sort by file first, then by section_order within each file
    sections.sort_by(|a, b| {
        a.file.cmp(&b.file).then(a.section_order.cmp(&b.section_order))
    });

    SectionsSummary { sections }
}

fn print_sections_text(summary: &SectionsSummary) {
    for section in &summary.sections {
        println!("📋 Section: {} (order: {})", section.name, section.section_order);
        println!("   File: {}", section.file);
        if !section.content.trim().is_empty() {
            println!("   Content: {}", section.content.trim());
        }
        println!(); // Add blank line between sections
    }
}