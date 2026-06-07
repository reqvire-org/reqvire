use anyhow::Result;

use crate::error::ReqvireError;
use crate::filesystem;
use crate::graph_registry::GraphRegistry;
use log::debug;

use crate::parser;
use crate::utils;
use globset::GlobSet;

#[derive(Debug)]
pub struct ModelManager {
    /// In-memory graph registry of elements and relations
    pub graph_registry: GraphRegistry,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ModelBuildOptions {
    pub lenient: bool,
    pub with_size_estimates: bool,
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelManager {
    /// Creates a new ModelManager
    pub fn new() -> Self {
        Self {
            graph_registry: GraphRegistry::new(),
        }
    }

    pub fn parse_and_validate(
        &mut self,
        git_commit_hash: Option<&str>,
        excluded_filename_patterns: &GlobSet,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
        self.parse_and_validate_with_mode(git_commit_hash, excluded_filename_patterns, false)
    }

    pub fn parse_and_validate_with_mode(
        &mut self,
        git_commit_hash: Option<&str>,
        excluded_filename_patterns: &GlobSet,
        lenient: bool,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
        self.parse_and_validate_with_options(
            git_commit_hash,
            excluded_filename_patterns,
            ModelBuildOptions {
                lenient,
                with_size_estimates: false,
            },
        )
    }

    pub fn parse_and_validate_with_options(
        &mut self,
        git_commit_hash: Option<&str>,
        excluded_filename_patterns: &GlobSet,
        options: ModelBuildOptions,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!(
            "Starting two-pass validation architecture (lenient={}, with_size_estimates={})",
            options.lenient, options.with_size_estimates
        );
        // Reset state so repeated parse/validate calls always start from a clean model.
        self.graph_registry = GraphRegistry::new();

        // Pass 1: Element collection with local validation
        let pass1_errors =
            self.pass1_collect_elements(git_commit_hash, excluded_filename_patterns)?;

        // If Pass 1 has errors, return them as an error (unless lenient mode)
        if !pass1_errors.is_empty() {
            debug!(
                "Pass 1 validation failed with {} errors",
                pass1_errors.len()
            );
            if !options.lenient {
                return Err(ReqvireError::ValidationError(pass1_errors));
            }
            debug!("Lenient mode: continuing despite Pass 1 errors");
        }

        debug!("Pass 1 completed, proceeding to Pass 2");

        // Pass 2: Graph construction and relation validation
        let pass2_errors = self.pass2_build_relations(excluded_filename_patterns)?;

        // If Pass 2 has errors, return them as an error (unless lenient mode)
        if !pass2_errors.is_empty() {
            debug!(
                "Pass 2 validation failed with {} errors",
                pass2_errors.len()
            );
            if !options.lenient {
                return Err(ReqvireError::ValidationError(pass2_errors));
            }
            debug!("Lenient mode: continuing despite Pass 2 errors");
        }

        if options.with_size_estimates {
            self.graph_registry.populate_size_estimates()?;
        }

        debug!("Validation completed");
        Ok(Vec::new())
    }

    /// Pass 1: Parse documents and collect elements with local validation
    fn pass1_collect_elements(
        &mut self,
        git_commit_hash: Option<&str>,
        excluded_filename_patterns: &GlobSet,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
        let mut errors = Vec::new();

        // Track all element locations for global uniqueness checking
        let mut all_element_locations: Vec<(String, String, usize)> = Vec::new(); // (name, file_path, line_number)

        let files = utils::scan_markdown_files(git_commit_hash, excluded_filename_patterns);
        debug!("Pass 1: Found {} markdown files.", files.len());

        let file_iterator = filesystem::FileReaderIterator::new(git_commit_hash, files);
        for file_result in file_iterator {
            match file_result {
                Err(e) => return Err(e),
                Ok((path, file_name, file_content)) => {
                    debug!("Pass 1: Processing file: {}", file_name);

                    let relative_path_str = utils::get_relative_path(&path)?
                        .to_string_lossy()
                        .to_string();

                    // Parse Elements and page content
                    let (elements, parse_errors, page_content) =
                        parser::parse_elements(&file_name, &file_content, &path, git_commit_hash);

                    // Collect parse-time errors
                    errors.extend(parse_errors);

                    // Register page content
                    self.graph_registry
                        .register_page(relative_path_str.clone(), page_content);

                    // Track element locations for global uniqueness checking
                    for element in &elements {
                        all_element_locations.push((
                            element.name.clone(),
                            element.file_path.clone(),
                            element.line_number,
                        ));
                    }

                    // Register parsed elements with local validation
                    for element in elements {
                        if let Err(e) = self
                            .graph_registry
                            .register_element(element, &relative_path_str)
                        {
                            errors.push(e);
                        }
                    }
                }
            }
        }

        // Global uniqueness validation: Check for duplicate element names across all files
        let mut name_locations: std::collections::HashMap<String, Vec<(String, usize)>> =
            std::collections::HashMap::new();

        for (name, file_path, line_number) in all_element_locations {
            name_locations
                .entry(name)
                .or_default()
                .push((file_path, line_number));
        }

        // Report duplicates with both locations
        for (name, locations) in name_locations.iter() {
            if locations.len() > 1 {
                // Sort locations for consistent error messages
                let mut sorted_locations = locations.clone();
                sorted_locations.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

                // For each duplicate after the first, report it with both locations
                for i in 1..sorted_locations.len() {
                    let msg = format!(
                        "'{}' found in {}:{} and {}:{}",
                        name,
                        sorted_locations[0].0,
                        sorted_locations[0].1,
                        sorted_locations[i].0,
                        sorted_locations[i].1
                    );
                    errors.push(ReqvireError::DuplicateElement(msg));
                }
            }
        }

        Ok(errors)
    }

    /// Pass 2: Build relations and validate graph structure
    fn pass2_build_relations(
        &mut self,
        excluded_filename_patterns: &GlobSet,
    ) -> Result<Vec<ReqvireError>, ReqvireError> {
        debug!("Pass 2: Delegating to GraphRegistry for relation building and validation");
        self.graph_registry
            .build_relations(excluded_filename_patterns)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_extract_path_and_fragment() {
        // Test file reference with fragment.
        let input = "/user/repo#readme";
        let (file, frag) = crate::utils::extract_path_and_fragment(input);
        assert_eq!(file, "/user/repo");
        assert_eq!(frag, Some("readme"));

        // Test fragment-only with leading '#'.
        let input = "#intro";
        let (file, frag) = crate::utils::extract_path_and_fragment(input);
        assert_eq!(file, "");
        assert_eq!(frag, Some("intro"));

        // Test file only.
        let input = "document.md";
        let (file, frag) = crate::utils::extract_path_and_fragment(input);
        assert_eq!(file, "document.md");
        assert_eq!(frag, None);

        // Test fragment-only without '#' (treated as fragment-only)
        let input = "onlyfragment";
        let (file, frag) = crate::utils::extract_path_and_fragment(input);
        assert_eq!(file, "");
        assert_eq!(frag, Some("onlyfragment"));
    }
}
