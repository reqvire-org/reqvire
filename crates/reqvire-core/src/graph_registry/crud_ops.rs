use super::*;

impl GraphRegistry {
    /// Updates an element's identifier and rewires all incoming relations
    pub fn update_identifier(&mut self, old_id: &str, new_id: &str) {
        if let Some(mut node) = self.nodes.remove(old_id) {
            node.element.identifier = new_id.to_string();

            // Update relations within this element (if any self-refs)
            for relation in &mut node.element.relations {
                if let LinkType::Identifier(ref mut link_id) = relation.target.link {
                    if link_id == old_id {
                        *link_id = new_id.to_string();
                    }
                }
            }

            // Reinsert with new ID
            self.nodes.insert(new_id.to_string(), node);

            // Update all relations pointing to this identifier
            for (_id, other_node) in self.nodes.iter_mut() {
                for relation in &mut other_node.element.relations {
                    if let LinkType::Identifier(ref mut link_id) = relation.target.link {
                        if link_id == old_id {
                            *link_id = new_id.to_string();
                        }
                    }
                }

                for relation_node in &mut other_node.relations {
                    if relation_node.element_node.element.identifier == old_id {
                        relation_node.element_node.element.identifier = new_id.to_string();
                    }
                }
            }
        }
    }

    /// Find element identifier by element name (globally unique)
    ///
    /// # Arguments
    /// * `element_name` - Element name to search for
    ///
    /// # Returns
    /// * Element identifier if found and unique
    /// * Error if not found or multiple matches
    pub fn find_element_by_name(&self, element_name: &str) -> Result<String, ReqvireError> {
        let search_name = element_name.trim();

        // Find all elements with matching name
        let matching: Vec<&String> = self
            .nodes
            .iter()
            .filter(|(_, node)| node.element.name == search_name)
            .map(|(id, _)| id)
            .collect();

        if matching.is_empty() {
            return Err(ReqvireError::MissingElement(format!(
                "Element not found: {}",
                element_name
            )));
        } else if matching.len() > 1 {
            return Err(ReqvireError::ProcessError(format!(
                "Multiple elements found with name '{}': {:?}",
                element_name, matching
            )));
        }

        Ok(matching[0].clone())
    }

    /// Moves an element to an existing file in the graph
    pub fn move_element_to_location(
        &mut self,
        element_id: &str,
        new_file_path: &str,
    ) -> Result<(), ReqvireError> {
        // Verify the target file exists in the graph (either has elements or is registered as a page)
        let target_has_elements = self
            .nodes
            .values()
            .any(|node| node.element.file_path == new_file_path);
        let target_is_page = self.pages.contains_key(new_file_path);

        if !target_has_elements && !target_is_page {
            return Err(ReqvireError::LocationNotFound(format!(
                "Target file '{}' does not exist in the graph",
                new_file_path
            )));
        }

        // '# Element' files represent exactly one implicit element.
        // Disallow moving additional elements into an existing single-element file.
        if target_has_elements && self.is_single_element_format_file(new_file_path) {
            let source_file_path = self
                .nodes
                .get(element_id)
                .map(|n| n.element.file_path.clone())
                .unwrap_or_default();
            if source_file_path != new_file_path {
                return Err(ReqvireError::InvalidOperation(format!(
                    "Cannot move element '{}' into '{}': target is a '# Element' file and can contain only one element.",
                    element_id, new_file_path
                )));
            }
        }

        if let Some(node) = self.nodes.get_mut(element_id) {
            let old_file_path = node.element.file_path.clone();

            node.element.file_path = new_file_path.to_string();

            // Update the element in all relation nodes that reference it
            for (_id, other_node) in self.nodes.iter_mut() {
                for relation_node in &mut other_node.relations {
                    if relation_node.element_node.element.identifier == element_id {
                        relation_node.element_node.element.file_path = new_file_path.to_string();
                    }
                }
            }

            log::debug!(
                "Moved element '{}' from '{}' to '{}'",
                element_id,
                old_file_path,
                new_file_path
            );

            Ok(())
        } else {
            Err(ReqvireError::MissingElement(format!(
                "Element '{}' not found in graph",
                element_id
            )))
        }
    }

    /// Adds a new file location to the graph (virtual - no filesystem changes)
    pub fn add_file_location(&mut self, new_file_path: &str) -> Result<(), ReqvireError> {
        // Check if the file already exists
        let file_exists = self
            .nodes
            .values()
            .any(|node| node.element.file_path == new_file_path);

        if file_exists {
            return Err(ReqvireError::LocationAlreadyExists(format!(
                "File '{}' already exists in the graph",
                new_file_path
            )));
        }

        // Create a virtual placeholder element to track this file location
        let virtual_id = format!("__virtual__{}", new_file_path);
        let virtual_element = Element::new(
            &format!("Virtual placeholder for {}", new_file_path),
            &virtual_id,
            new_file_path,
            0, // Virtual elements don't have real line numbers
            None,
        );

        self.nodes.insert(
            virtual_id,
            ElementNode {
                element: virtual_element,
                relations: Vec::new(),
            },
        );

        log::debug!("Added virtual file location '{}'", new_file_path);
        Ok(())
    }

    /// Moves element to a new file location (creates file location if needed)
    pub fn move_element_to_new_file(
        &mut self,
        element_id: &str,
        new_file_path: &str,
    ) -> Result<(), ReqvireError> {
        // Check if file exists, if not, create it virtually
        let file_exists = self
            .nodes
            .values()
            .any(|node| node.element.file_path == new_file_path);

        if !file_exists {
            self.add_file_location(new_file_path)?;
        }

        if let Some(node) = self.nodes.get_mut(element_id) {
            let old_file_path = node.element.file_path.clone();

            node.element.file_path = new_file_path.to_string();

            // Update the element in all relation nodes that reference it
            for (_id, other_node) in self.nodes.iter_mut() {
                for relation_node in &mut other_node.relations {
                    if relation_node.element_node.element.identifier == element_id {
                        relation_node.element_node.element.file_path = new_file_path.to_string();
                    }
                }
            }

            // Update relation identifiers for cross-file references
            self.update_relation_identifiers(element_id, &old_file_path, new_file_path);

            log::debug!(
                "Moved element '{}' from '{}' to new file '{}'",
                element_id,
                old_file_path,
                new_file_path
            );

            Ok(())
        } else {
            Err(ReqvireError::MissingElement(format!(
                "Element '{}' not found in graph",
                element_id
            )))
        }
    }

    /// Gets all available file locations in the graph
    pub fn get_available_locations(&self) -> Vec<String> {
        let mut locations = std::collections::BTreeSet::new();

        for node in self.nodes.values() {
            locations.insert(node.element.file_path.clone());
        }

        locations.into_iter().collect()
    }

    /// Gets all elements that would be affected by moving the specified element
    pub fn get_move_impact(&self, element_id: &str) -> Vec<String> {
        let mut affected_elements = Vec::new();

        // Find all elements that reference this element
        for (id, node) in &self.nodes {
            if id == element_id {
                continue; // Skip the element being moved
            }

            // Check if this element has relations pointing to the element being moved
            let has_reference = node.element.relations.iter().any(|relation| {
                matches!(&relation.target.link, LinkType::Identifier(link_id) if link_id == element_id)
            });

            if has_reference {
                affected_elements.push(id.clone());
            }
        }

        affected_elements.sort();
        affected_elements
    }

    pub fn get_impact_tree(&self, root_id: &str) -> ElementNode {
        let mut visited = BTreeSet::new();
        self.build_impact_tree_recursive(root_id, &mut visited)
    }

    fn build_impact_tree_recursive(
        &self,
        current_id: &str,
        visited: &mut BTreeSet<String>,
    ) -> ElementNode {
        if !visited.insert(current_id.to_string()) {
            // Already visited, stop recursion to prevent cycles
            let current_node = self
                .nodes
                .get(current_id)
                .expect("node not found in registry");
            return ElementNode {
                element: current_node.element.clone(),
                relations: Vec::new(), // Empty relations to break the cycle
            };
        }

        let current_node = self
            .nodes
            .get(current_id)
            .expect("node not found in registry");
        let mut child_nodes = Vec::new();

        for relation_node in &current_node.relations {
            let target_id = &relation_node.element_node.element.identifier;

            // Skip relations to already visited nodes to prevent cycles
            if visited.contains(target_id) {
                continue;
            }

            let subtree = self.build_impact_tree_recursive(target_id, visited);
            child_nodes.push(RelationNode {
                relation_trigger: relation_node.relation_trigger.clone(),
                element_node: subtree,
            });
        }

        ElementNode {
            element: current_node.element.clone(),
            relations: child_nodes,
        }
    }

    /// Gets all elements as a vector, sorted by identifier for deterministic output
    pub fn get_all_elements(&self) -> Vec<&Element> {
        let mut elements: Vec<&Element> = self.nodes.values().map(|node| &node.element).collect();
        elements.sort_by(|a, b| a.identifier.cmp(&b.identifier));
        elements
    }

    /// Change impact analysis with relation information
    pub fn change_impact_with_relation(
        &self,
        element: &Element,
    ) -> Vec<(String, Vec<crate::relation::Relation>)> {
        if let Some(node) = self.nodes.get(&element.identifier) {
            // Group original relations by target ID using BTreeMap for deterministic ordering
            let mut relations_by_target: std::collections::BTreeMap<
                String,
                Vec<crate::relation::Relation>,
            > = std::collections::BTreeMap::new();

            for relation in &node.element.relations {
                let target_id = match &relation.target.link {
                    crate::relation::LinkType::Identifier(ref target_id) => target_id.clone(),
                    crate::relation::LinkType::InternalPath(ref path) => {
                        path.to_string_lossy().to_string()
                    }
                    crate::relation::LinkType::ExternalUrl(_) => continue, // Skip external URLs for change impact
                };

                relations_by_target
                    .entry(target_id)
                    .or_default()
                    .push(relation.clone());
            }

            relations_by_target.into_iter().collect()
        } else {
            Vec::new()
        }
    }

    /// Gets a specific element by ID
    pub fn get_element(&self, element_id: &str) -> Option<&Element> {
        self.nodes.get(element_id).map(|node| &node.element)
    }

    /// Gets an element by its display name
    pub fn get_element_by_name(&self, name: &str) -> Option<&Element> {
        self.nodes
            .values()
            .map(|node| &node.element)
            .find(|elem| elem.name == name)
    }

    /// Creates a virtual file location
    pub fn create_virtual_file(&mut self, file_path: &str) -> Result<(), ReqvireError> {
        self.add_file_location(file_path)
    }

    fn relation_format_rank(element: &Element, relation_name: &str) -> u8 {
        if !element.element_type.is_concept_family() {
            return 100;
        }

        match relation_name {
            "derivedFrom" => 0,
            "broader" => 10,
            "narrower" => 20,
            "related" => 30,
            "exactMatch" => 40,
            "closeMatch" => 50,
            _ => 100,
        }
    }

    fn append_element_metadata(markdown: &mut String, element: &Element, heading: &str) {
        markdown.push_str(heading);
        markdown.push('\n');
        markdown.push_str(&format!(
            "  * type: {}\n",
            element.element_type.to_metadata_string()
        ));
        let mut custom_metadata: Vec<_> = element
            .metadata
            .iter()
            .filter(|(key, _)| *key != "type" && *key != "_single_element_format")
            .collect();
        custom_metadata.sort_by_key(|(key, _)| *key);
        for (key, value) in custom_metadata {
            markdown.push_str(&format!("  * {}: {}\n", key, value));
        }
        markdown.push('\n');
    }

    fn sorted_relations_for_format<'a>(
        element: &'a Element,
        with_full_relations: bool,
    ) -> Vec<&'a Relation> {
        let mut relations_to_include: Vec<_> = if with_full_relations {
            element.relations.iter().collect()
        } else {
            element
                .relations
                .iter()
                .filter(|relation| relation.user_created)
                .collect()
        };
        relations_to_include.sort_by(|a, b| {
            Self::relation_format_rank(element, a.relation_type.name)
                .cmp(&Self::relation_format_rank(element, b.relation_type.name))
                .then(a.relation_type.name.cmp(b.relation_type.name))
                .then(a.target.link.as_str().cmp(b.target.link.as_str()))
        });
        relations_to_include.dedup_by(|a, b| {
            a.relation_type.name == b.relation_type.name
                && a.target.link.as_str() == b.target.link.as_str()
        });
        relations_to_include
    }

    pub(super) fn element_to_markdown_with_context(
        &self,
        element: &Element,
        _current_file: &str,
        with_full_relations: bool,
    ) -> String {
        let mut markdown = String::new();

        // Add the element header
        markdown.push_str(&format!("### {}\n\n", element.name));

        // Add the element content
        if !element.content.trim().is_empty() {
            let content = self.normalize_concept_reference_links_for_format(element, _current_file);
            markdown.push_str(content.trim_end());
            markdown.push('\n');
        }

        // Always include metadata to preserve structure during CRUD operations.
        Self::append_element_metadata(&mut markdown, element, "#### Metadata");

        // Add contract_bindings subsection if there are contract_bindings
        // Deduplicate contract_bindings by target, keeping first occurrence
        let mut seen_contract_bindings: rustc_hash::FxHashSet<String> =
            rustc_hash::FxHashSet::default();
        let unique_contract_bindings: Vec<_> = element
            .contract_bindings
            .iter()
            .filter(|a| seen_contract_bindings.insert(a.target.as_str()))
            .collect();

        if !unique_contract_bindings.is_empty() {
            markdown.push_str("#### ");
            markdown.push_str(CONTRACT_BINDINGS_SECTION);
            markdown.push('\n');
            for contract_bindings in unique_contract_bindings {
                match &contract_bindings.target {
                    crate::element::ContractBindingTarget::FilePath(file_path) => {
                        // ContractBindingEntry paths are stored as git-root-relative paths
                        let contract_binding_path = file_path.to_string_lossy().to_string();

                        // Make the path relative to the current file's directory (same as relations)
                        let current_file_path = std::path::PathBuf::from(_current_file);
                        let current_folder = current_file_path
                            .parent()
                            .unwrap_or_else(|| std::path::Path::new("."))
                            .to_path_buf();

                        // Use to_relative_identifier like we do for InternalPath relations
                        // Prepend "/" to indicate git-root-relative path
                        let absolute_path = format!("/{}", contract_binding_path);
                        let relative_path = crate::utils::to_relative_identifier(
                            &absolute_path,
                            &current_folder,
                            false,
                        )
                        .unwrap_or_else(|_| contract_binding_path.clone());

                        // Use filename as display text for cleaner markdown
                        let display_name = file_path
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or(&contract_binding_path);

                        markdown.push_str(&format!("  * [{}]({})\n", display_name, relative_path));
                    }
                    crate::element::ContractBindingTarget::ElementIdentifier(identifier) => {
                        // Element identifier contract_bindings - format as markdown link
                        let current_file_path = std::path::PathBuf::from(_current_file);
                        let current_folder = current_file_path
                            .parent()
                            .unwrap_or_else(|| std::path::Path::new("."))
                            .to_path_buf();

                        // Use to_relative_identifier to make identifier relative to current file
                        let relative_id =
                            crate::utils::to_relative_identifier(identifier, &current_folder, true)
                                .unwrap_or_else(|_| identifier.clone());

                        // Look up actual element name from registry for human-readable display
                        let display_name = self
                            .get_element(identifier)
                            .map(|e| e.name.clone())
                            .unwrap_or_else(|| {
                                // Fallback to identifier fragment if element not found
                                identifier
                                    .split('#')
                                    .next_back()
                                    .unwrap_or(identifier)
                                    .to_string()
                            });
                        markdown.push_str(&format!("  * [{}]({})\n", display_name, relative_id));
                    }
                }
            }
            markdown.push('\n');
        }

        let relations_to_include = Self::sorted_relations_for_format(element, with_full_relations);
        if !relations_to_include.is_empty() {
            markdown.push_str("#### Relations\n");
            for relation in relations_to_include {
                // Format relation target based on type
                // Format as proper markdown link using element name when possible
                let target_text = match &relation.target.link {
                    LinkType::ExternalUrl(url) => {
                        // For external URLs, preserve the original markdown link format
                        format!("[{}]({})", relation.target.text, url)
                    }
                    LinkType::Identifier(target_id) => {
                        // Extract fragment to look up the target element
                        let fragment = if let Some(fragment_pos) = target_id.find('#') {
                            &target_id[fragment_pos + 1..]
                        } else {
                            target_id
                        };

                        // Use actual element name if available, otherwise fallback to fragment conversion
                        // First try to lookup by full target_id, then by fragment only
                        let display_name = if let Some(target_node) = self.nodes.get(target_id) {
                            target_node.element.name.clone()
                        } else if let Some(target_node) = self.nodes.get(fragment) {
                            target_node.element.name.clone()
                        } else {
                            // Fallback: convert fragment to title case
                            fragment
                                .replace('-', " ")
                                .split_whitespace()
                                .map(|word| {
                                    let mut chars = word.chars();
                                    match chars.next() {
                                        None => String::new(),
                                        Some(first) => {
                                            first.to_uppercase().collect::<String>()
                                                + chars.as_str()
                                        }
                                    }
                                })
                                .collect::<Vec<String>>()
                                .join(" ")
                        };

                        // Check if target is in the same file
                        let target_file = if let Some(file_pos) = target_id.find('#') {
                            &target_id[..file_pos]
                        } else {
                            target_id
                        };

                        // Get current file path for comparison
                        let current_file_path = std::path::PathBuf::from(_current_file);
                        let current_file_str = _current_file;

                        // If target is in the same file, use just the fragment
                        if target_file.is_empty()
                            || target_file == current_file_str
                            || target_id.starts_with('#')
                        {
                            format!("[{}](#{})", display_name, fragment)
                        } else {
                            // Make the link relative using just the folder of the current file
                            let current_folder = current_file_path
                                .parent()
                                .unwrap_or_else(|| std::path::Path::new("."))
                                .to_path_buf();

                            let relative_link = crate::utils::to_relative_identifier(
                                relation.target.link.as_str(),
                                &current_folder,
                                false,
                            )
                            .unwrap_or_else(|_| relation.target.link.as_str().to_string());

                            format!("[{}]({})", display_name, relative_link)
                        }
                    }
                    LinkType::InternalPath(path) => {
                        // For InternalPath, use the filename as display text and full relative path as link
                        let path_str = path.to_str().unwrap_or("invalid_path");
                        let display_name = std::path::Path::new(path_str)
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or(path_str);

                        // Make the path relative using just the folder of the current file
                        let current_file_path = std::path::PathBuf::from(_current_file);
                        let current_folder = current_file_path
                            .parent()
                            .unwrap_or_else(|| std::path::Path::new("."))
                            .to_path_buf();

                        let relative_link = crate::utils::to_relative_identifier(
                            relation.target.link.as_str(),
                            &current_folder,
                            false,
                        )
                        .unwrap_or_else(|_| relation.target.link.as_str().to_string());

                        format!("[{}]({})", display_name, relative_link)
                    }
                };

                markdown.push_str(&format!(
                    "  * {}: {}\n",
                    relation.relation_type.name, target_text
                ));
            }
            markdown.push('\n');
        }

        // Apply generic formatting to ensure exactly one blank line before all #### headers
        Self::ensure_blank_lines_before_subsections(&markdown)
    }

    fn normalize_concept_reference_links_for_format(
        &self,
        element: &Element,
        current_file: &str,
    ) -> String {
        let mut output = Vec::new();
        let mut in_section = false;

        for line in element.content.split_inclusive('\n') {
            let (body, suffix) = if let Some(body) = line.strip_suffix("\r\n") {
                (body, "\r\n")
            } else if let Some(body) = line.strip_suffix('\n') {
                (body, "\n")
            } else {
                (line, "")
            };
            let trimmed = body.trim();
            if trimmed.starts_with("#### ") {
                in_section = trimmed == "#### Concept References";
                output.push(line.to_string());
                continue;
            }
            if !in_section {
                output.push(line.to_string());
                continue;
            }
            let Some(entry) = trimmed.strip_prefix("* ") else {
                output.push(line.to_string());
                continue;
            };
            let Some((label, target)) = crate::utils::extract_markdown_link(entry) else {
                output.push(line.to_string());
                continue;
            };
            let Ok(target_id) =
                crate::parser::normalize_concept_reference_target(&element.file_path, &target)
            else {
                output.push(line.to_string());
                continue;
            };
            if !self
                .nodes
                .get(&target_id)
                .is_some_and(|node| node.element.element_type.is_concept())
            {
                output.push(line.to_string());
                continue;
            }
            let Some(link) =
                crate::utils::concept_reference_relative_link(current_file, &target_id).ok()
            else {
                output.push(line.to_string());
                continue;
            };
            let leading_len = body.len() - body.trim_start().len();
            output.push(format!(
                "{}* [{}]({}){}",
                &body[..leading_len],
                label.trim(),
                link,
                suffix
            ));
        }

        output.concat()
    }

    /// Ensures every #### header has exactly one blank line before it (skips content inside <details> blocks)
    /// and removes blank lines immediately after #### headers
    fn ensure_blank_lines_before_subsections(content: &str) -> String {
        let mut result = String::new();
        let mut in_details = false;

        for line in content.lines() {
            let trimmed_line = line.trim_start().to_lowercase();

            // Track <details> blocks
            if trimmed_line.starts_with("<details") {
                in_details = true;
            }

            // Add blank line before #### headers (if not in <details>)
            if !in_details && line.trim_start().starts_with("####") {
                // Remove any trailing newlines
                while result.ends_with('\n') {
                    result.pop();
                }
                if !result.is_empty() {
                    result.push_str("\n\n");
                }
            }

            // Skip blank lines immediately after #### headers
            if !in_details && line.trim().is_empty() {
                // Check if the previous line was a #### header
                let prev_line_is_header = result
                    .lines()
                    .last()
                    .is_some_and(|l| l.trim_start().starts_with("####"));
                if prev_line_is_header {
                    continue;
                }
            }

            result.push_str(line);
            result.push('\n');

            // Track end of <details> blocks
            if trimmed_line.starts_with("</details>") {
                in_details = false;
            }
        }

        // Trim end
        let trimmed = result.trim_end();
        if trimmed.is_empty() {
            String::new()
        } else {
            format!("{}\n", trimmed)
        }
    }

    /// Groups elements by their file path and orders them following Element Ordering Behavior
    pub fn group_elements_by_location(&self) -> FxHashMap<String, Vec<&Element>> {
        let mut file_elements: FxHashMap<String, Vec<&Element>> = FxHashMap::default();

        for node in self.nodes.values() {
            let element = &node.element;

            // Skip virtual placeholder elements
            if element.identifier.starts_with("__virtual__") {
                continue;
            }

            file_elements
                .entry(element.file_path.clone())
                .or_default()
                .push(element);
        }

        // Apply Element Ordering Behavior to each file
        for elements in file_elements.values_mut() {
            self.order_elements_hierarchically(elements);
        }

        file_elements
    }

    /// Orders elements following Element Ordering Behavior:
    /// - Parent elements appear before their children (file-local derivedFrom hierarchy)
    /// - Root elements (no file-local parent) sorted alphabetically
    /// - Siblings at each level sorted alphabetically
    fn order_elements_hierarchically(&self, elements: &mut Vec<&Element>) {
        if elements.len() <= 1 {
            return;
        }

        // Build a map of element fragment (slug) -> index for quick lookup
        // The fragment is the part after # in the identifier (e.g., "parent-a" from "file.md#parent-a")
        let fragment_to_idx: FxHashMap<String, usize> = elements
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let fragment = e
                    .identifier
                    .split('#')
                    .next_back()
                    .unwrap_or(&e.identifier)
                    .to_string();
                (fragment, i)
            })
            .collect();

        // Build parent -> children map based on file-local derivedFrom relations
        // Using indices to avoid lifetime issues
        let mut children_map: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
        let mut has_parent: FxHashSet<usize> = FxHashSet::default();

        for (idx, element) in elements.iter().enumerate() {
            // Find file-local derivedFrom relations
            for relation in &element.relations {
                if relation.relation_type.name == "derivedFrom" {
                    // Check if target is in the same file
                    if let Some(target_id) = &relation.target.element_id {
                        // target_id is the fragment (slug) like "parent-a"
                        // Check if this target exists in the same file
                        if let Some(&parent_idx) = fragment_to_idx.get(target_id) {
                            // This element has a file-local parent
                            children_map.entry(parent_idx).or_default().push(idx);
                            has_parent.insert(idx);
                        }
                    }
                }
            }
        }

        // Identify root element indices (those without file-local parents)
        let mut roots: Vec<usize> = (0..elements.len())
            .filter(|idx| !has_parent.contains(idx))
            .collect();

        // Sort roots alphabetically by element name
        roots.sort_by(|&a, &b| elements[a].name.cmp(&elements[b].name));

        // Sort children at each level alphabetically by element name
        for children in children_map.values_mut() {
            children.sort_by(|&a, &b| elements[a].name.cmp(&elements[b].name));
        }

        // Build ordered list using depth-first traversal with stack (iterative)
        let mut ordered_indices: Vec<usize> = Vec::with_capacity(elements.len());
        let mut visited: FxHashSet<usize> = FxHashSet::default();

        // Process roots in reverse order so they come out in correct order
        let mut stack: Vec<usize> = Vec::new();
        for &root in roots.iter().rev() {
            stack.push(root);
        }

        while let Some(idx) = stack.pop() {
            if visited.contains(&idx) {
                continue;
            }
            visited.insert(idx);
            ordered_indices.push(idx);

            // Push children in reverse alphabetical order so they come out in correct order
            if let Some(children) = children_map.get(&idx) {
                for &child_idx in children.iter().rev() {
                    if !visited.contains(&child_idx) {
                        stack.push(child_idx);
                    }
                }
            }
        }

        // Reorder elements based on ordered indices
        let original: Vec<&Element> = std::mem::take(elements);
        for idx in ordered_indices {
            elements.push(original[idx]);
        }
    }

    fn single_element_file_markdown(
        &self,
        file_path: &str,
        element: &Element,
        with_full_relations: bool,
    ) -> String {
        let mut markdown = String::new();
        markdown.push_str("# Element\n\n");

        Self::append_element_metadata(&mut markdown, element, "## Metadata");

        let relations_to_include = Self::sorted_relations_for_format(element, with_full_relations);
        if !relations_to_include.is_empty() {
            markdown.push_str("## Relations\n");
            for relation in relations_to_include {
                let target_text = match &relation.target.link {
                    LinkType::ExternalUrl(url) => format!("[{}]({})", relation.target.text, url),
                    LinkType::Identifier(target_id) => {
                        let current_file_path = PathBuf::from(file_path);
                        let current_folder = current_file_path
                            .parent()
                            .unwrap_or_else(|| Path::new("."))
                            .to_path_buf();
                        let relative_id =
                            crate::utils::to_relative_identifier(target_id, &current_folder, true)
                                .unwrap_or_else(|_| target_id.clone());
                        let display_name = self
                            .get_element(target_id)
                            .map(|e| e.name.clone())
                            .unwrap_or_else(|| relation.target.text.clone());
                        format!("[{}]({})", display_name, relative_id)
                    }
                    LinkType::InternalPath(path) => {
                        let current_file_path = PathBuf::from(file_path);
                        let current_folder = current_file_path
                            .parent()
                            .unwrap_or_else(|| Path::new("."))
                            .to_path_buf();
                        let path_str = path.to_string_lossy().to_string();
                        let absolute_path = format!("/{}", path_str);
                        let relative_path = crate::utils::to_relative_identifier(
                            &absolute_path,
                            &current_folder,
                            false,
                        )
                        .unwrap_or(path_str.clone());
                        let display_name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or(&path_str);
                        format!("[{}]({})", display_name, relative_path)
                    }
                };

                markdown.push_str(&format!(
                    "  * {}: {}\n",
                    relation.relation_type.name, target_text
                ));
            }
            markdown.push('\n');
        }

        if !element.contract_bindings.is_empty() {
            markdown.push_str("## ");
            markdown.push_str(CONTRACT_BINDINGS_SECTION);
            markdown.push('\n');
            for contract_bindings in &element.contract_bindings {
                match &contract_bindings.target {
                    crate::element::ContractBindingTarget::FilePath(path) => {
                        let path_str = path.to_string_lossy().to_string();
                        markdown.push_str(&format!("  * [{}]({})\n", path_str, path_str));
                    }
                    crate::element::ContractBindingTarget::ElementIdentifier(id) => {
                        let display = self
                            .get_element(id)
                            .map(|e| e.name.clone())
                            .unwrap_or_else(|| id.clone());
                        markdown.push_str(&format!("  * [{}]({})\n", display, id));
                    }
                }
            }
            markdown.push('\n');
        }

        markdown.push_str(&format!("## {}\n\n", element.name));
        if !element.content.trim().is_empty() {
            markdown.push_str(element.content.trim_end());
            markdown.push('\n');
        }

        markdown
    }

    /// Generates markdown content for a file
    /// When with_full_relations is true, includes all relations (user-created and auto-generated)
    pub fn generate_file_markdown(
        &self,
        file_path: &str,
        elements: &[&Element],
        with_full_relations: bool,
    ) -> String {
        if elements.len() == 1
            && elements[0]
                .metadata
                .get("_single_element_format")
                .map(|v| v == "true")
                .unwrap_or(false)
        {
            return self.single_element_file_markdown(file_path, elements[0], with_full_relations);
        }

        let mut markdown = String::new();

        // All specification files must have "# Elements" as the page header
        markdown.push_str("# Elements\n\n");

        // Add page content if available
        if let Some(page) = self.pages.get(file_path) {
            if !page.frontmatter_content.trim().is_empty() {
                markdown.push_str(&page.frontmatter_content);
                if !page.frontmatter_content.ends_with('\n') {
                    markdown.push('\n');
                }
                markdown.push('\n');
            }
        }

        // Add elements in file order
        for (i, element) in elements.iter().enumerate() {
            // Add separator before each element (except the first)
            if i > 0 {
                markdown.push_str("---\n\n");
            }
            markdown.push_str(&self.element_to_markdown_with_context(
                element,
                file_path,
                with_full_relations,
            ));
        }

        // Add final separator after the last element (if there were any elements)
        if !elements.is_empty() {
            markdown.push_str("---\n\n");
        }

        markdown
    }

    /// Copies InternalPath files to the output directory
    fn copy_internal_path_files(
        &self,
        internal_paths: &FxHashSet<PathBuf>,
        output_dir: &Path,
    ) -> Result<usize, ReqvireError> {
        let base_dir = match git_commands::get_git_root_dir() {
            Ok(git_root) => git_root,
            Err(_) => {
                // If Git repository root can't be found, use the current working directory
                std::env::current_dir().map_err(|e| {
                    ReqvireError::PathError(format!("Failed to get current directory: {}", e))
                })?
            }
        };

        let mut files_copied = 0;

        for internal_path in internal_paths {
            // Resolve the source path relative to base directory
            let src_path = if internal_path.is_absolute() {
                internal_path.clone()
            } else {
                base_dir.join(internal_path)
            };

            // Skip if source file doesn't exist
            if !src_path.is_file() {
                warn!("Skipping missing InternalPath file: {:?}", src_path);
                continue;
            }

            // Build the destination path
            let dst_path = output_dir.join(internal_path);

            // Skip if source and destination are the same (in-place operations)
            if src_path == dst_path {
                debug!(
                    "Skipping InternalPath file (same source and destination): {:?}",
                    src_path
                );
                continue;
            }

            // Create parent directories if needed
            if let Some(parent_dir) = dst_path.parent() {
                fs::create_dir_all(parent_dir).map_err(ReqvireError::IoError)?;
            }

            // Copy the file
            match fs::copy(&src_path, &dst_path) {
                Ok(_) => {
                    debug!("Copied InternalPath file: {:?} -> {:?}", src_path, dst_path);
                    files_copied += 1;
                }
                Err(e) => {
                    warn!("Failed to copy InternalPath file {:?}: {}", src_path, e);
                }
            }
        }

        Ok(files_copied)
    }

    /// Rename an element while updating all relations
    ///
    /// # Arguments
    /// * `element_id` - Current element identifier
    /// * `new_name` - New name for the element
    ///
    /// # Returns
    /// * New element identifier after rename
    pub fn rename_element(
        &mut self,
        element_id: &str,
        new_name: &str,
    ) -> Result<String, ReqvireError> {
        // Validate element exists
        let node = self.nodes.get(element_id).ok_or_else(|| {
            ReqvireError::MissingElement(format!("Element '{}' not found", element_id))
        })?;

        let file_path = node.element.file_path.clone();
        let _old_name = node.element.name.clone();

        // Generate new identifier (slug from new name - same logic as markdown heading to ID)
        let new_slug = new_name.trim().replace(' ', "-").to_lowercase();
        let new_identifier = format!("{}#{}", file_path, new_slug);

        // Check if new identifier already exists (globally unique check)
        if self.nodes.contains_key(&new_identifier) {
            return Err(ReqvireError::DuplicateElement(format!(
                "An element with name '{}' already exists (identifier: {})",
                new_name, new_identifier
            )));
        }

        // Find all files with relations to this element
        let mut modified_files = vec![file_path.clone()];
        for node in self.nodes.values() {
            let has_relation = node.element.relations.iter().any(
                |rel| matches!(&rel.target.link, LinkType::Identifier(id) if id == element_id),
            );

            if has_relation {
                let file = node.element.file_path.clone();
                if !modified_files.contains(&file) {
                    modified_files.push(file);
                }
            }
        }

        // Find all files with contract_bindings pointing to this element
        for file in self.find_files_with_contract_bindings_to(element_id) {
            if !modified_files.contains(&file) {
                modified_files.push(file);
            }
        }

        // Update the element's name and identifier in the node
        if let Some(node) = self.nodes.get_mut(element_id) {
            node.element.name = new_name.to_string();
            node.element.identifier = new_identifier.clone();
        }

        // Move node in the map (remove old key, insert with new key)
        if let Some(node) = self.nodes.remove(element_id) {
            self.nodes.insert(new_identifier.clone(), node);
        }

        // Update all relations (both forward and backward)
        // Update relations in all elements that reference the old identifier
        let old_id = element_id.to_string();
        for node in self.nodes.values_mut() {
            for relation in &mut node.element.relations {
                if let LinkType::Identifier(ref mut id) = relation.target.link {
                    if id == &old_id {
                        *id = new_identifier.clone();
                        // Update the text reference too
                        relation.target.text = new_name.to_string();
                    }
                }
            }
        }

        // Update all contract_bindings identifiers pointing to this element
        self.update_contract_bindings_identifiers(&old_id, &new_identifier);

        // Track all modified files
        for file in &modified_files {
            self.modified_files.insert(file.clone());
        }

        Ok(new_identifier)
    }

    /// Move entire file with all its elements to a new location
    /// Updates all element identifiers and relations referencing moved elements
    pub fn move_file(
        &mut self,
        source_file: &str,
        target_file: &str,
        squash: bool,
    ) -> Result<Vec<(String, String)>, ReqvireError> {
        // Validate source file exists in the model
        let elements_in_source: Vec<String> = self
            .nodes
            .values()
            .filter(|node| node.element.file_path == source_file)
            .map(|node| node.element.identifier.clone())
            .collect();

        if elements_in_source.is_empty() {
            return Err(ReqvireError::LocationNotFound(format!(
                "Source file '{}' not found or contains no elements",
                source_file
            )));
        }

        // Validate target file doesn't exist (unless squash mode)
        let target_exists = self
            .nodes
            .values()
            .any(|node| node.element.file_path == target_file);

        if target_exists && !squash {
            return Err(ReqvireError::DuplicateElement(format!(
                "Target file '{}' already exists",
                target_file
            )));
        }

        // '# Element' files represent one implicit element.
        // Squashing multiple elements into such file would violate the format.
        if squash && target_exists && self.is_single_element_format_file(target_file) {
            return Err(ReqvireError::InvalidOperation(format!(
                "Cannot use --squash into '{}': target is a '# Element' file and can contain only one element.",
                target_file
            )));
        }

        // Track old -> new identifier mappings
        let mut identifier_mappings: Vec<(String, String)> = Vec::new();
        let mut modified_files = vec![source_file.to_string()];

        // In squash mode, move elements to target file
        if squash && target_exists {
            // Move each element to target file
            for old_id in &elements_in_source {
                let slug = if let Some(pos) = old_id.rfind('#') {
                    &old_id[pos + 1..]
                } else {
                    continue;
                };
                let new_id = format!("{}#{}", target_file, slug);

                // Update element
                if let Some(node) = self.nodes.get_mut(old_id) {
                    node.element.file_path = target_file.to_string();
                    node.element.identifier = new_id.clone();
                }

                identifier_mappings.push((old_id.clone(), new_id.clone()));
            }
        } else {
            // Normal mode: move entire file (keep sections as-is)
            for old_id in &elements_in_source {
                let slug = if let Some(pos) = old_id.rfind('#') {
                    &old_id[pos + 1..]
                } else {
                    continue;
                };
                let new_id = format!("{}#{}", target_file, slug);
                identifier_mappings.push((old_id.clone(), new_id.clone()));
            }

            // Update all elements in the source file
            for (old_id, new_id) in &identifier_mappings {
                if let Some(node) = self.nodes.get_mut(old_id) {
                    node.element.file_path = target_file.to_string();
                    node.element.identifier = new_id.clone();
                }
            }
        }

        // Find all files with relations to elements in the source file
        for node in self.nodes.values() {
            let has_relation = node.element.relations.iter().any(|rel| {
                if let LinkType::Identifier(id) = &rel.target.link {
                    elements_in_source.contains(id)
                } else {
                    false
                }
            });
            if has_relation {
                let file = node.element.file_path.clone();
                if !modified_files.contains(&file) {
                    modified_files.push(file);
                }
            }
        }

        // Find all files with contract_bindings to elements in the source file
        for old_id in &elements_in_source {
            for file in self.find_files_with_contract_bindings_to(old_id) {
                if !modified_files.contains(&file) {
                    modified_files.push(file);
                }
            }
        }

        // Move nodes in FxHashMap (remove old key, insert with new key)
        for (old_id, new_id) in &identifier_mappings {
            if let Some(node) = self.nodes.remove(old_id) {
                self.nodes.insert(new_id.clone(), node);
            }
        }

        // Update all relations pointing to moved elements
        for (old_id, new_id) in &identifier_mappings {
            for node in self.nodes.values_mut() {
                for relation in &mut node.element.relations {
                    if let LinkType::Identifier(ref mut target_id) = relation.target.link {
                        if target_id == old_id {
                            *target_id = new_id.clone();
                        }
                    }
                }
            }
        }

        // Update all contract_bindings identifiers pointing to moved elements
        for (old_id, new_id) in &identifier_mappings {
            self.update_contract_bindings_identifiers(old_id, new_id);
        }

        modified_files.push(target_file.to_string());

        for file in &modified_files {
            self.modified_files.insert(file.clone());
        }

        Ok(identifier_mappings)
    }

    /// Flushes all elements to markdown files and copies InternalPath files to the specified directory
    /// When with_full_relations is true, includes all relations (user-created and auto-generated inverse relations)
    pub fn flush_to_directory(
        &self,
        output_dir: &Path,
        with_full_relations: bool,
    ) -> Result<(usize, usize), ReqvireError> {
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            fs::create_dir_all(output_dir).map_err(ReqvireError::IoError)?;
        }

        // Generate and write markdown files
        let grouped_elements = self.group_elements_by_location();
        let mut markdown_files_written = 0;

        for (file_path, elements) in grouped_elements {
            self.write_grouped_markdown_file(
                &file_path,
                &elements,
                output_dir,
                with_full_relations,
            )?;
            markdown_files_written += 1;
        }

        // Copy InternalPath files
        let internal_paths = self.collect_internal_path_targets();
        let internal_files_copied = self.copy_internal_path_files(&internal_paths, output_dir)?;

        log::info!(
            "Successfully flushed {} markdown files and copied {} internal files to {}",
            markdown_files_written,
            internal_files_copied,
            output_dir.display()
        );

        Ok((markdown_files_written, internal_files_copied))
    }

    /// Flushes elements from specific files to markdown files and copies related InternalPath files
    /// When with_full_relations is true, includes all relations (user-created and auto-generated inverse relations)
    pub fn flush_files_to_directory(
        &self,
        file_paths: &[String],
        output_dir: &Path,
        with_full_relations: bool,
    ) -> Result<(usize, usize), ReqvireError> {
        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            fs::create_dir_all(output_dir).map_err(ReqvireError::IoError)?;
        }

        let grouped_elements = self.group_elements_by_location();
        let mut markdown_files_written = 0;
        let mut related_internal_paths = FxHashSet::default();

        for file_path in file_paths {
            if let Some(elements) = grouped_elements.get(file_path) {
                self.write_grouped_markdown_file(
                    file_path,
                    elements,
                    output_dir,
                    with_full_relations,
                )?;

                // Collect InternalPath relations from elements in this file
                for element in elements {
                    for relation in &element.relations {
                        if let LinkType::InternalPath(ref path) = relation.target.link {
                            related_internal_paths.insert(path.clone());
                        }
                    }
                }

                markdown_files_written += 1;
            }
        }

        // Copy related InternalPath files
        let internal_files_copied =
            self.copy_internal_path_files(&related_internal_paths, output_dir)?;

        log::info!(
            "Successfully flushed {} markdown files and copied {} internal files to {}",
            markdown_files_written,
            internal_files_copied,
            output_dir.display()
        );

        Ok((markdown_files_written, internal_files_copied))
    }

    fn write_grouped_markdown_file(
        &self,
        file_path: &str,
        elements: &[&Element],
        output_dir: &Path,
        with_full_relations: bool,
    ) -> Result<(), ReqvireError> {
        let markdown_content =
            self.generate_file_markdown(file_path, elements, with_full_relations);
        let output_file_path = output_dir.join(file_path);

        if let Some(parent_dir) = output_file_path.parent() {
            fs::create_dir_all(parent_dir).map_err(ReqvireError::IoError)?;
        }

        fs::write(&output_file_path, markdown_content).map_err(ReqvireError::IoError)?;

        debug!(
            "Flushed {} elements to {}",
            elements.len(),
            output_file_path.display()
        );

        Ok(())
    }

    // Dynamic graph manipulation methods

    /// Updates relation identifiers when elements move between files
    fn update_relation_identifiers(
        &mut self,
        moved_element_id: &str,
        _old_file_path: &str,
        new_file_path: &str,
    ) {
        // Extract just the fragment (element name) from the moved element's identifier
        let moved_fragment = moved_element_id
            .split('#')
            .next_back()
            .unwrap_or(moved_element_id);

        // 1. Update relations FROM other elements TO the moved element
        let source_node_ids: Vec<String> = self.nodes.keys().cloned().collect();
        for source_id in source_node_ids {
            if source_id == moved_element_id {
                continue;
            }

            let source_file_path = self
                .nodes
                .get(&source_id)
                .map(|node| node.element.file_path.clone());
            let mut relations = self
                .nodes
                .get(&source_id)
                .map(|node| node.element.relations.clone())
                .unwrap_or_default();

            if relations.is_empty() {
                continue;
            }

            let Some(source_file_path) = source_file_path else {
                continue;
            };

            let mut changed = false;
            let canonical_target = format!("{}#{}", new_file_path, moved_fragment);
            for relation in &mut relations {
                if let crate::relation::LinkType::Identifier(ref mut target_id) =
                    relation.target.link
                {
                    if self.relation_targets_same_identifier(
                        &source_file_path,
                        target_id,
                        moved_element_id,
                    ) {
                        *target_id = canonical_target.clone();
                        relation.target.text = canonical_target.clone();
                        relation.target.element_id = Some(canonical_target.clone());
                        changed = true;
                    }
                }
            }

            if changed {
                if let Some(source_node) = self.nodes.get_mut(&source_id) {
                    source_node.element.relations = relations;
                }
            }
        }

        // 2. Update relations FROM the moved element TO other elements
        let moved_node_file = self
            .nodes
            .get(moved_element_id)
            .map(|node| node.element.file_path.clone());
        if let (Some(moved_node_file), Some(mut relations)) = (
            moved_node_file,
            self.nodes
                .get(moved_element_id)
                .map(|node| node.element.relations.clone()),
        ) {
            let mut changed = false;
            for relation in &mut relations {
                if let crate::relation::LinkType::Identifier(ref mut target_id) =
                    relation.target.link
                {
                    if let Some(resolved_target) =
                        self.normalize_relation_identifier_for_source(&moved_node_file, target_id)
                    {
                        if let Some(target_node) = self.nodes.get(&resolved_target) {
                            let target_file_path = target_node.element.file_path.clone();
                            let target_fragment =
                                crate::utils::extract_path_and_fragment(&resolved_target)
                                    .1
                                    .unwrap_or(&resolved_target);
                            let canonical_target =
                                format!("{}#{}", target_file_path, target_fragment);
                            *target_id = canonical_target.clone();
                            relation.target.text = canonical_target;
                            relation.target.element_id = Some(resolved_target);
                            changed = true;
                        }
                    }
                }
            }

            if changed {
                if let Some(moved_node) = self.nodes.get_mut(moved_element_id) {
                    moved_node.element.relations = relations;
                }
            }
        }
    }

    /// Updates contract_bindings identifiers when a Contract element is moved or renamed
    /// Similar to update_relation_identifiers but for contract_bindings references
    fn update_contract_bindings_identifiers(&mut self, old_identifier: &str, new_identifier: &str) {
        // Find and update all contract_bindings identifiers pointing to the old identifier
        for node in self.nodes.values_mut() {
            for contract_bindings in &mut node.element.contract_bindings {
                if let crate::element::ContractBindingTarget::ElementIdentifier(ref mut id) =
                    contract_bindings.target
                {
                    if id == old_identifier {
                        *id = new_identifier.to_string();
                    }
                }
            }
        }
    }

    /// Finds all files that have contract_bindings pointing to the given element identifier
    fn find_files_with_contract_bindings_to(&self, element_id: &str) -> Vec<String> {
        let mut files = Vec::new();
        for node in self.nodes.values() {
            let has_contract_bindings = node.element.contract_bindings.iter().any(|att| {
                matches!(&att.target, crate::element::ContractBindingTarget::ElementIdentifier(id) if id == element_id)
            });
            if has_contract_bindings {
                let file = node.element.file_path.clone();
                if !files.contains(&file) {
                    files.push(file);
                }
            }
        }
        files
    }

    /// Adds a new element to the graph
    pub fn add_element(&mut self, element: Element) -> Result<(), ReqvireError> {
        let element_id = element.identifier.clone();

        if self.nodes.contains_key(&element_id) {
            return Err(ReqvireError::ElementMoveError(format!(
                "Element '{}' already exists in the graph",
                element_id
            )));
        }

        self.nodes.insert(
            element_id,
            ElementNode {
                element,
                relations: Vec::new(),
            },
        );

        Ok(())
    }

    /// Removes an element from the graph and all relations pointing to it
    pub fn remove_element(&mut self, element_id: &str) -> Result<(), ReqvireError> {
        if !self.nodes.contains_key(element_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Element '{}' not found in the graph",
                element_id
            )));
        }

        // Remove the element itself
        self.nodes.remove(element_id);

        // Remove all relations pointing to this element from graph structure
        for node in self.nodes.values_mut() {
            node.relations
                .retain(|rel| rel.element_node.element.identifier != element_id);
        }

        // Remove all relations pointing to this element from element's own relations list
        let mut node_ids: Vec<String> = self.nodes.keys().cloned().collect();
        for node_id in node_ids.drain(..) {
            let source_file_path = self
                .nodes
                .get(&node_id)
                .map(|node| node.element.file_path.clone());
            let mut relations = self
                .nodes
                .get(&node_id)
                .map(|node| node.element.relations.clone())
                .unwrap_or_default();

            let Some(source_file_path) = source_file_path else {
                continue;
            };

            let mut filtered = Vec::new();
            for relation in relations.drain(..) {
                let keep = match &relation.target.link {
                    crate::relation::LinkType::Identifier(target) => !self
                        .relation_targets_same_identifier(&source_file_path, target, element_id),
                    _ => true,
                };
                if keep {
                    filtered.push(relation);
                }
            }
            if let Some(mut_node) = self.nodes.get_mut(&node_id) {
                mut_node.element.relations = filtered;
            }
        }

        Ok(())
    }

    /// Adds a relation between two elements in the graph
    pub fn add_relation(
        &mut self,
        source_id: &str,
        target_id: &str,
        relation_type: &str,
    ) -> Result<(), ReqvireError> {
        // Validate both elements exist
        if !self.nodes.contains_key(source_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Source element '{}' not found",
                source_id
            )));
        }
        if !self.nodes.contains_key(target_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Target element '{}' not found",
                target_id
            )));
        }

        // Check if relation type is valid for impact propagation
        if !relation::IMPACT_PROPAGATION_RELATIONS.contains(&relation_type) {
            return Err(ReqvireError::ProcessError(format!(
                "Relation type '{}' is not valid for impact propagation",
                relation_type
            )));
        }

        // Get the target node to create the relation
        let target_node = self
            .nodes
            .get(target_id)
            .expect("node not found in registry")
            .clone();

        // Add the relation to the source element
        let source_node = self
            .nodes
            .get_mut(source_id)
            .expect("node not found in registry");

        // Check if relation already exists
        let relation_exists = source_node.relations.iter().any(|rel| {
            rel.element_node.element.identifier == target_id
                && rel.relation_trigger == relation_type
        });

        if relation_exists {
            return Err(ReqvireError::ProcessError(format!(
                "Relation '{}' from '{}' to '{}' already exists",
                relation_type, source_id, target_id
            )));
        }

        source_node.relations.push(RelationNode {
            relation_trigger: relation_type.to_string(),
            element_node: target_node,
        });

        Ok(())
    }

    /// Removes a specific relation between two elements (graph structure only)
    pub fn remove_relation(
        &mut self,
        source_id: &str,
        target_id: &str,
        relation_type: &str,
    ) -> Result<(), ReqvireError> {
        if !self.nodes.contains_key(source_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Source element '{}' not found",
                source_id
            )));
        }

        let source_node = self
            .nodes
            .get_mut(source_id)
            .expect("node not found in registry");
        let initial_count = source_node.relations.len();

        source_node.relations.retain(|rel| {
            !(rel.element_node.element.identifier == target_id
                && rel.relation_trigger == relation_type)
        });

        if source_node.relations.len() == initial_count {
            return Err(ReqvireError::ProcessError(format!(
                "Relation '{}' from '{}' to '{}' not found",
                relation_type, source_id, target_id
            )));
        }

        Ok(())
    }

    /// Removes a relation from an element's relations array with bidirectional handling
    /// This removes the relation from element.relations (which gets written to markdown)
    /// and also removes the opposite relation if one exists
    pub fn remove_element_relation(
        &mut self,
        element_id: &str,
        target_id: &str,
        relation_type: &str,
    ) -> Result<(), ReqvireError> {
        // Check if source element exists
        if !self.nodes.contains_key(element_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Element '{}' not found",
                element_id
            )));
        }

        // Check if target element exists
        if !self.nodes.contains_key(target_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Target element '{}' not found",
                target_id
            )));
        }

        // Remove the relation from source element's relations array
        let source_node = self
            .nodes
            .get_mut(element_id)
            .expect("node not found in registry");
        let initial_count = source_node.element.relations.len();

        source_node.element.relations.retain(|rel| {
            !(rel.relation_type.name == relation_type &&
              matches!(&rel.target.link, crate::relation::LinkType::Identifier(id) if id == target_id))
        });

        if source_node.element.relations.len() == initial_count {
            return Err(ReqvireError::ProcessError(format!(
                "Relation '{}' from '{}' to '{}' not found",
                relation_type, element_id, target_id
            )));
        }

        // Check if this relation type has an opposite (bidirectional)
        if let Some(relation_info) = crate::relation::RELATION_TYPES.get(relation_type) {
            if let Some(opposite_type) = relation_info.opposite {
                // Remove the opposite relation from target element
                let target_node = self
                    .nodes
                    .get_mut(target_id)
                    .expect("node not found in registry");
                target_node.element.relations.retain(|rel| {
                    !(rel.relation_type.name == opposite_type &&
                      matches!(&rel.target.link, crate::relation::LinkType::Identifier(id) if id == element_id))
                });
            }
        }

        Ok(())
    }

    /// Remove a contract binding from an element
    pub fn remove_element_contract_bindings(
        &mut self,
        element_id: &str,
        contract_bindings: &str,
    ) -> Result<(), ReqvireError> {
        if let Some(node) = self.nodes.get_mut(element_id) {
            let original_len = node.element.contract_bindings.len();
            node.element
                .contract_bindings
                .retain(|a| a.target.as_str() != contract_bindings);

            if node.element.contract_bindings.len() < original_len {
                self.modified_files.insert(node.element.file_path.clone());
                Ok(())
            } else {
                Err(ReqvireError::ProcessError(format!(
                    "ContractBindingEntry '{}' not found on element '{}'",
                    contract_bindings, element_id
                )))
            }
        } else {
            Err(ReqvireError::ProcessError(format!(
                "Element '{}' not found",
                element_id
            )))
        }
    }

    /// Lists all relations for a given element
    pub fn list_relations(&self, element_id: &str) -> Result<Vec<(String, String)>, ReqvireError> {
        let node = self.nodes.get(element_id).ok_or_else(|| {
            ReqvireError::LocationNotFound(format!("Element '{}' not found", element_id))
        })?;

        let relations = node
            .relations
            .iter()
            .map(|rel| {
                (
                    rel.relation_trigger.clone(),
                    rel.element_node.element.identifier.clone(),
                )
            })
            .collect();

        Ok(relations)
    }

    /// Adds a relation to an element with full validation and target resolution
    /// This is the comprehensive method used by CRUD operations
    ///
    /// # Arguments
    /// * `source_id` - Source element identifier
    /// * `target` - Target (element name, URL, or file path)
    /// * `relation_type` - Relation type name
    /// * `git_root` - Git root path for file resolution
    ///
    /// # Returns
    /// Modified file path
    pub fn add_element_relation_full(
        &mut self,
        source_id: &str,
        target: &str,
        relation_type: &str,
        git_root: &std::path::Path,
    ) -> Result<String, ReqvireError> {
        use crate::relation::{
            get_relation_element_type_description, validate_relation_element_types, LinkType,
            Relation, RelationTarget, LEGACY_CONTRACT_RELATIONS, RELATION_TYPES,
        };
        use std::path::PathBuf;

        // Validate source element exists
        if !self.nodes.contains_key(source_id) {
            return Err(ReqvireError::ElementNotFound(format!(
                "Source element '{}' not found",
                source_id
            )));
        }

        // Validate relation type
        if LEGACY_CONTRACT_RELATIONS.contains(&relation_type) {
            let replacement = if relation_type == "refinedBy" {
                "definedBy"
            } else {
                "define"
            };
            return Err(ReqvireError::UnsupportedRelationType(format!(
                "Legacy relation type '{}'. Use '{}' for requirement-owned contract elements, or run `reqvire migrate` on existing sources.",
                relation_type, replacement
            )));
        }

        if !RELATION_TYPES.contains_key(relation_type) {
            return Err(ReqvireError::UnsupportedRelationType(format!(
                "Invalid relation type '{}'. Valid types: {}",
                relation_type,
                crate::relation::supported_relation_types_list()
            )));
        }

        // Get source element info
        let source_node = self
            .nodes
            .get(source_id)
            .expect("node not found in registry");
        let source_name = source_node.element.name.clone();
        let source_file_path = source_node.element.file_path.clone();
        let source_type = source_node.element.element_type.clone();

        // Determine target type: element name, external URL, or internal path
        let is_external_url = crate::utils::is_external_url(target);
        let is_internal_path = !is_external_url
            && (target.ends_with(".md") || target.contains('/') || git_root.join(target).exists());

        // Resolve target and create relation components
        let (target_display_name, relation_target_link, target_id_for_check, element_id_opt) =
            if is_external_url {
                // External URL - use as-is
                (
                    target.to_string(),
                    LinkType::ExternalUrl(target.to_string()),
                    target.to_string(),
                    None,
                )
            } else if is_internal_path {
                // Internal file path
                let source_folder = crate::utils::get_parent_dir(&source_file_path);
                let target_type = crate::element::ElementType::File;

                if !validate_relation_element_types(relation_type, &source_type, &target_type) {
                    let description = get_relation_element_type_description(relation_type)
                        .unwrap_or_else(|| {
                            format!(
                                "Relation '{}' is not compatible with source type '{}' and internal file targets",
                                relation_type,
                                source_type.as_str()
                            )
                        });
                    return Err(ReqvireError::IncompatibleElementTypes(format!(
                        "Relation '{}' from '{}' ({}) to '{}' (file) has incompatible element types. {}",
                        relation_type,
                        source_name,
                        source_type.as_str(),
                        target,
                        description
                    )));
                }

                // Calculate relative path from source file to target
                let target_path = PathBuf::from(target);
                let relative_path = pathdiff::diff_paths(&target_path, &source_folder)
                    .unwrap_or_else(|| target_path.clone());

                // Extract filename for display name
                let display = target_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| target.to_string());

                (
                    display,
                    LinkType::InternalPath(relative_path),
                    target.to_string(),
                    None,
                )
            } else {
                // Element name - resolve to get identifier
                let target_element = self.get_element_by_name(target).ok_or_else(|| {
                    ReqvireError::ElementNotFound(format!("Target element '{}' not found", target))
                })?;

                let target_id = target_element.identifier.clone();
                let target_display_name = target_element.name.clone();
                let target_type = target_element.element_type.clone();

                if source_node.element.contract_bindings.iter().any(|a| {
                    let contract_bindings_target = a.target.as_str();
                    contract_bindings_target == target_id
                        || self
                            .resolve_relation_identifier(
                                &source_node.element,
                                &contract_bindings_target,
                            )
                            .is_some_and(|resolved| resolved == target_id)
                        || self.relation_targets_same_identifier(
                            &source_file_path,
                            &contract_bindings_target,
                            &target_id,
                        )
                }) {
                    return Err(ReqvireError::CrossSectionDuplicate(format!(
                        "Target '{}' already exists in Contract Bindings of '{}'. Cannot add to Relations.",
                        target, source_name
                    )));
                }

                if !validate_relation_element_types(relation_type, &source_type, &target_type) {
                    let description = get_relation_element_type_description(relation_type)
                        .unwrap_or_else(|| {
                            format!(
                                "Relation '{}' is not compatible with source type '{}' and target type '{}'",
                                relation_type,
                                source_type.as_str(),
                                target_type.as_str()
                            )
                        });
                    return Err(ReqvireError::IncompatibleElementTypes(format!(
                        "Relation '{}' from '{}' ({}) to '{}' ({}) has incompatible element types. {}",
                        relation_type,
                        source_name,
                        source_type.as_str(),
                        target_display_name,
                        target_type.as_str(),
                        description
                    )));
                }

                let relation_target = LinkType::Identifier(target_id.clone());

                // Extract element ID (fragment) for change tracking
                let (_path, fragment_opt) = crate::utils::extract_path_and_fragment(&target_id);
                let element_id = fragment_opt.map(|s| s.to_string());

                (target_display_name, relation_target, target_id, element_id)
            };

        // Get source node again (mutable this time)
        let source_node = self
            .nodes
            .get(source_id)
            .expect("node not found in registry");

        // Validate: Check if relation already exists (idempotent)
        let relation_exists = source_node.element.relations.iter().any(|r| {
            r.user_created
                && r.relation_type.name == relation_type
                && r.target.link.as_str() == target_id_for_check
        });

        if relation_exists {
            return Err(ReqvireError::RelationError(format!(
                "Relation '{}' from '{}' to '{}' already exists",
                relation_type, source_name, target
            )));
        }

        // Validate: Check for cross-section duplicate (target in Contract Bindings)
        let in_contract_bindings = source_node
            .element
            .contract_bindings
            .iter()
            .any(|a| a.target.as_str() == target_id_for_check);

        if in_contract_bindings {
            return Err(ReqvireError::CrossSectionDuplicate(format!(
                "Target '{}' already exists in Contract Bindings of '{}'. Cannot add to Relations.",
                target, source_name
            )));
        }

        // Create the relation
        let relation_type_info = RELATION_TYPES
            .get(relation_type)
            .expect("relation type not defined");
        let relation = Relation {
            relation_type: relation_type_info,
            target: RelationTarget {
                text: target_display_name,
                link: relation_target_link,
                element_id: element_id_opt,
            },
            user_created: true,
        };

        // Get source element info for opposite relation before mutation
        let source_node = self
            .nodes
            .get(source_id)
            .expect("node not found in registry");
        let source_name = source_node.element.name.clone();
        let source_element_id = source_node.element.id.clone();
        let file_path = source_node.element.file_path.clone();

        // Add relation to source element
        let source_node = self
            .nodes
            .get_mut(source_id)
            .expect("node not found in registry");
        source_node.element.relations.push(relation.clone());

        // Mark file as modified
        self.modified_files.insert(file_path.clone());

        // CRITICAL: Maintain bidirectional consistency for in-memory model
        // Use helper to add opposite relation to target element (if applicable)
        self.add_opposite_to_target(&relation, source_id, &source_name, &source_element_id);

        Ok(file_path)
    }

    /// Removes a relation from an element with full target resolution
    /// This is the comprehensive method used by CRUD operations
    ///
    /// # Arguments
    /// * `source_id` - Source element identifier
    /// * `target` - Target (element name, URL, or file path)
    ///
    /// # Returns
    /// Tuple of (modified file path, relation type, target display name) or None if no relation found
    pub fn remove_element_relation_full(
        &mut self,
        source_id: &str,
        target: &str,
    ) -> Result<Option<(String, String, String)>, ReqvireError> {
        // Validate source element exists
        if !self.nodes.contains_key(source_id) {
            return Err(ReqvireError::ElementNotFound(format!(
                "Source element '{}' not found",
                source_id
            )));
        }

        let source_node = self
            .nodes
            .get(source_id)
            .expect("node not found in registry");
        let source_file_path = source_node.element.file_path.clone();

        // Try to resolve target as element name first
        let target_id_to_find = if let Some(target_element) = self.get_element_by_name(target) {
            target_element.identifier.clone()
        } else {
            let normalized_target =
                crate::utils::normalize_relation_identifier_for_registry(&source_file_path, target);

            if self.nodes.contains_key(&normalized_target) {
                normalized_target
            } else {
                target.to_string()
            }
        };

        // Find matching relation (check both user_created and auto-generated)
        // This allows unlinking from either side of a bidirectional relation
        let relation_match = source_node
            .element
            .relations
            .iter()
            .find(|r| r.target.link.as_str() == target_id_to_find)
            .cloned(); // Clone to avoid borrow issues

        if let Some(relation) = relation_match {
            let relation_type = relation.relation_type.name.to_string();
            let target_display_name = relation.target.text.clone();
            let relation_type_info = crate::relation::RELATION_TYPES
                .get(relation_type.as_str())
                .expect("relation type not defined");
            let source_relation_was_user_created = relation.user_created;

            // Remove the relation (both user_created and auto-generated)
            let source_node = self
                .nodes
                .get_mut(source_id)
                .expect("node not found in registry");
            source_node.element.relations.retain(|r| {
                !(r.relation_type.name == relation_type
                    && r.target.link.as_str() == target_id_to_find)
            });

            // Mark source file as modified only if relation was user_created (written to file)
            if source_relation_was_user_created {
                self.modified_files.insert(source_file_path.clone());
            }

            // CRITICAL: Maintain bidirectional consistency for in-memory model
            // Use helper to remove opposite relation from target element (if applicable)
            if let Some(opposite_type_name) = relation_type_info.opposite {
                self.remove_opposite_from_target(&target_id_to_find, source_id, opposite_type_name);
            }

            Ok(Some((source_file_path, relation_type, target_display_name)))
        } else {
            // No relation found - could be a contract binding (handled by crud layer)
            Ok(None)
        }
    }

    /// Gets statistics about the graph
    pub fn get_graph_stats(&self) -> (usize, usize) {
        let element_count = self.nodes.len();
        let relation_count = self.nodes.values().map(|node| node.relations.len()).sum();

        (element_count, relation_count)
    }

    // ================================
    // CRUD Operations (Add, Delete, Move)
    // ================================

    /// Creates an element from markdown string and adds it to the graph
    /// Used by CLI add command
    pub fn create_element_from_string(
        &mut self,
        markdown: &str,
        target_file: &str,
        excluded_patterns: &GlobSet,
    ) -> Result<Element, ReqvireError> {
        // Validate target path
        let validation = crate::utils::validate_target_path(target_file, None, excluded_patterns)?;

        if !validation.is_valid {
            return Err(ReqvireError::InvalidPath(
                validation
                    .error_message
                    .unwrap_or_else(|| "Invalid target path".to_string()),
            ));
        }

        // Parse element from markdown string
        let element = crate::parser::parse_single_element(markdown, target_file)?;

        // Check for duplicate element name (global uniqueness)
        if self.nodes.contains_key(&element.identifier) {
            return Err(ReqvireError::DuplicateElement(format!(
                "Element '{}' already exists in the model",
                element.name
            )));
        }

        // Validate that all relation targets exist in the model
        // External links (http://, https://, etc.) are allowed and not validated
        for relation in &element.relations {
            if let crate::relation::LinkType::Identifier(target_id) = &relation.target.link {
                // Check if this is an external link using the predefined list
                let is_external = crate::utils::EXTERNAL_SCHEMES
                    .iter()
                    .any(|scheme| target_id.starts_with(scheme));

                // If not external, validate that the target exists
                if !is_external && !self.nodes.contains_key(target_id) {
                    return Err(ReqvireError::MissingElement(
                        format!(
                            "Relation target '{}' does not exist in the model. Cannot add element '{}' with relation to non-existent element.",
                            target_id,
                            element.name
                        )
                    ));
                }
            }
        }

        // Auto-create file if needed
        if validation.needs_file_creation {
            self.add_file_location(target_file)?;

            // Add page content (file header based on filename)
            let file_stem = Path::new(target_file)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Document");

            self.register_page(target_file.to_string(), format!("# {}\n", file_stem));
        }

        // Set file_order_index: append to end of file
        let mut new_element = element.clone();
        let max_index = self
            .nodes
            .values()
            .filter(|node| node.element.file_path == target_file)
            .map(|node| node.element.file_order_index)
            .max()
            .unwrap_or(0);
        new_element.file_order_index = max_index + 1;

        // Add to graph
        self.add_element(new_element.clone())?;

        // Populate element_id for all relations (including the newly added element)
        // This is necessary for hierarchical ordering to recognize parent-child relationships
        self.populate_relation_element_ids();
        // CRITICAL: Maintain bidirectional consistency for in-memory model
        // Use helper to create opposite relations for all relations in the newly added element
        let new_element_id = new_element.identifier.clone();
        let new_element_name = new_element.name.clone();
        let new_element_fragment_id = new_element.id.clone();
        let relations_to_process: Vec<_> = self
            .nodes
            .get(&new_element_id)
            .expect("node not found in registry")
            .element
            .relations
            .clone();

        for relation in relations_to_process {
            self.add_opposite_to_target(
                &relation,
                &new_element_id,
                &new_element_name,
                &new_element_fragment_id,
            );
        }

        // Track modified file
        self.modified_files.insert(target_file.to_string());

        Ok(new_element)
    }

    /// Check if removing an element would orphan any children
    ///
    /// Returns a sorted list of child element names that would be orphaned
    fn check_for_orphaned_children(&self, element_id: &str) -> Result<Vec<String>, ReqvireError> {
        let mut orphaned_children: Vec<String> = Vec::new();
        let hierarchical_types = crate::relation::get_hierarchical_relation_types();

        for child_node in self.nodes.values() {
            // Count how many hierarchical parent relations this child has to the element being deleted
            let mut parents_to_target = 0;
            let mut total_parents = 0;

            for rel in &child_node.element.relations {
                let target_id = match &rel.target.link {
                    crate::relation::LinkType::Identifier(id) => id.as_str(),
                    _ => continue, // Skip external links
                };
                if hierarchical_types.contains(&rel.relation_type.name) {
                    total_parents += 1;
                    if target_id == element_id {
                        parents_to_target += 1;
                    }
                }
            }

            // If child only has hierarchical parent relations to the target element, it will be orphaned
            if parents_to_target > 0 && total_parents == parents_to_target {
                orphaned_children.push(child_node.element.name.clone());
            }
        }

        orphaned_children.sort();
        Ok(orphaned_children)
    }

    /// Enhanced remove element that tracks modifications and performs cleanup
    pub fn remove_element_with_cleanup(
        &mut self,
        element_id: &str,
    ) -> Result<Vec<String>, ReqvireError> {
        if !self.nodes.contains_key(element_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Element '{}' not found in the graph",
                element_id
            )));
        }

        // Get element info before removal
        let element = &self
            .nodes
            .get(element_id)
            .expect("node not found in registry")
            .element;
        let element_name = element.name.clone();
        let file_path = element.file_path.clone();

        // Validate: Check for orphaned children before removal
        let orphaned_children = self.check_for_orphaned_children(element_id)?;
        if !orphaned_children.is_empty() {
            return Err(ReqvireError::InvalidOperation(
                format!(
                    "Cannot delete '{}' because it has {} child element(s) with parent hierarchical relations that would become orphaned: {}.\n\n\
                    To proceed, either:\n\
                    1. Delete the child elements first, or\n\
                    2. Update the child elements to link to a different parent element",
                    element_name,
                    orphaned_children.len(),
                    orphaned_children.join(", ")
                )
            ));
        }

        // Track all files that will be modified
        let mut modified_files = vec![file_path.clone()];

        // Find all elements with relations pointing to this element
        for (other_id, node) in self.nodes.iter() {
            if other_id != element_id {
                let source_file_path = node.element.file_path.clone();
                let has_relation_to_target = node.element.relations.iter().any(|rel| {
                    matches!(
                        &rel.target.link,
                        LinkType::Identifier(target_id)
                            if self.relation_targets_same_identifier(
                                &source_file_path,
                                target_id,
                                element_id,
                            )
                    )
                });

                if has_relation_to_target {
                    let other_file = node.element.file_path.clone();
                    if !modified_files.contains(&other_file) {
                        modified_files.push(other_file);
                    }
                }
            }
        }

        // Remove element and relations
        self.remove_element(element_id)?;

        // Track modified files
        for file in &modified_files {
            self.modified_files.insert(file.clone());
        }

        Ok(modified_files)
    }

    /// Checks if a file has no elements remaining
    pub fn is_file_empty(&self, file_path: &str) -> bool {
        !self
            .nodes
            .values()
            .any(|node| node.element.file_path == file_path)
    }

    /// Comprehensive move operation with full relation updates and file tracking
    pub fn move_element_comprehensive(
        &mut self,
        element_id: &str,
        target_file: &str,
        excluded_patterns: &GlobSet,
    ) -> Result<(String, Vec<String>), ReqvireError> {
        // Validate element exists
        if !self.nodes.contains_key(element_id) {
            return Err(ReqvireError::LocationNotFound(format!(
                "Element '{}' not found",
                element_id
            )));
        }

        // Get source file before move
        let source_file = self
            .nodes
            .get(element_id)
            .expect("node not found in registry")
            .element
            .file_path
            .clone();

        // Validate target path
        let validation = crate::utils::validate_target_path(target_file, None, excluded_patterns)?;

        if !validation.is_valid {
            return Err(ReqvireError::InvalidPath(
                validation
                    .error_message
                    .unwrap_or_else(|| "Invalid target path".to_string()),
            ));
        }

        // Auto-create file if needed
        if validation.needs_file_creation {
            self.add_file_location(target_file)?;

            let file_stem = Path::new(target_file)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Document");

            self.register_page(target_file.to_string(), format!("# {}\n", file_stem));
        }

        // Perform the move using existing move_element_to_location
        let old_identifier = element_id.to_string();

        // Find all files with relations to this element BEFORE updating relations
        let mut modified_files = vec![source_file.clone()];
        if target_file != source_file {
            modified_files.push(target_file.to_string());
        }

        for node in self.nodes.values() {
            let source_file_path = node.element.file_path.clone();
            let has_relation = node.element.relations.iter().any(|rel| {
                matches!(
                    &rel.target.link,
                    LinkType::Identifier(target_id)
                        if self.relation_targets_same_identifier(
                            &source_file_path,
                            target_id,
                            &old_identifier,
                        )
                )
            });

            if has_relation {
                let file = node.element.file_path.clone();
                if !modified_files.contains(&file) {
                    modified_files.push(file);
                }
            }
        }

        // Find all files with contract_bindings pointing to this element
        for file in self.find_files_with_contract_bindings_to(element_id) {
            if !modified_files.contains(&file) {
                modified_files.push(file);
            }
        }

        // Now perform the move
        self.move_element_to_location(element_id, target_file)?;

        // Update all relations (TO and FROM the moved element)
        self.update_relation_identifiers(&old_identifier, &source_file, target_file);

        // Construct the new identifier (file path changed, fragment stays the same)
        let fragment = old_identifier.split('#').next_back().unwrap_or("");
        let new_identifier = format!("{}#{}", target_file, fragment);

        // Re-key the node in the FxHashMap: remove with old key, update identifier, insert with new key
        if let Some(mut node) = self.nodes.remove(&old_identifier) {
            node.element.identifier = new_identifier.clone();
            self.nodes.insert(new_identifier.clone(), node);
        }

        // Update all contract_bindings identifiers pointing to this element
        self.update_contract_bindings_identifiers(&old_identifier, &new_identifier);

        // CRITICAL: Recreate opposite relations with updated identifiers
        // After moving, opposite relations pointing to the old identifier must be updated
        self.recreate_opposites_after_move(&old_identifier, &new_identifier);

        // Track all modified files
        for file in &modified_files {
            self.modified_files.insert(file.clone());
        }

        Ok((new_identifier, modified_files))
    }

    /// Merge multiple source elements into a target element
    ///
    /// # Arguments
    /// * `target_id` - Identifier of the target element (must exist)
    /// * `source_ids` - Identifiers of source elements to merge into target (must exist)
    ///
    /// # Behavior
    /// - Source content is appended to target's Details section
    /// - Source Details sections become "Merged Details (source name)" subsections
    /// - Relations and contract_bindings are merged with deduplication
    /// - Source elements are deleted after successful merge
    /// - Relations pointing to source elements are redirected to target
    pub fn merge_elements(
        &mut self,
        target_id: &str,
        source_ids: &[String],
    ) -> Result<(), ReqvireError> {
        // Validate target exists
        if !self.nodes.contains_key(target_id) {
            return Err(ReqvireError::ElementNotFound(format!(
                "Target element '{}' not found",
                target_id
            )));
        }

        // Get target element data first (needed for validation)
        let target_node = self
            .nodes
            .get(target_id)
            .expect("node not found in registry");
        let target_name = target_node.element.name.clone();
        let target_type = target_node.element.element_type.clone();
        let target_file_path = target_node.element.file_path.clone();
        let target_is_single_element = self.is_single_element_format_file(&target_file_path);

        // Validate all sources exist and collect their data
        #[allow(clippy::type_complexity)]
        let mut source_data: Vec<(
            String,
            String,
            String,
            Vec<crate::relation::Relation>,
            Vec<crate::element::ContractBindingEntry>,
            Element,
        )> = Vec::new();
        for source_id in source_ids {
            let source_node = self.nodes.get(source_id).ok_or_else(|| {
                ReqvireError::ElementNotFound(format!("Source element '{}' not found", source_id))
            })?;

            let source_element = &source_node.element;
            let source_file_path = source_element.file_path.clone();
            let source_is_single_element = self.is_single_element_format_file(&source_file_path);

            // Validate: Check if source would merge into itself
            if source_id == target_id {
                return Err(ReqvireError::InvalidOperation(
                    "Cannot merge element into itself".to_string(),
                ));
            }

            // Merging single-element source content into # Elements target is disallowed.
            // '# Element' bodies permit headers that violate # Elements parsing constraints.
            if source_is_single_element && !target_is_single_element {
                return Err(ReqvireError::InvalidOperation(format!(
                    "Cannot merge '{}' into '{}': source is in a '# Element' file and target is in a '# Elements' file. This conversion can break '# Elements' parsing rules and must be performed manually.",
                    source_element.name, target_name
                )));
            }

            // Validate: Check type compatibility
            if !target_type.is_merge_compatible(&source_element.element_type) {
                return Err(ReqvireError::MergeTypeMismatch(format!(
                    "Cannot merge '{}' ({}) into '{}' ({}): type mismatch. \
                     Elements must be in the same category (requirement/verification/contract/other).",
                    source_element.name, source_element.element_type.as_str(),
                    target_name, target_type.as_str()
                )));
            }

            source_data.push((
                source_id.clone(),
                source_element.name.clone(),
                source_element.content.clone(),
                source_element
                    .relations
                    .iter()
                    .filter(|r| r.user_created)
                    .cloned()
                    .collect(),
                source_element.contract_bindings.clone(),
                source_element.clone(),
            ));
        }

        // Re-get target element data (needed after validation)
        let target_node = self
            .nodes
            .get(target_id)
            .expect("node not found in registry");
        let mut merged_content = String::new();
        let mut merged_relations: Vec<crate::relation::Relation> = target_node
            .element
            .relations
            .iter()
            .filter(|r| r.user_created)
            .cloned()
            .collect();
        let mut merged_contract_bindings: Vec<crate::element::ContractBindingEntry> =
            target_node.element.contract_bindings.clone();
        let target_is_ontology = target_type.is_ontology();
        let target_element_for_merge = target_node.element.clone();
        let mut merged_source_ids: FxHashSet<String> = source_ids.iter().cloned().collect();
        merged_source_ids.insert(target_id.to_string());

        // Process each source element
        for (
            source_id,
            source_name,
            source_content,
            source_relations,
            source_contract_bindings,
            source_element,
        ) in &source_data
        {
            // Extract main content and details from source
            let (main_content, details_content) =
                if target_is_ontology && source_element.element_type.is_ontology() {
                    (extract_leading_prose(source_content), String::new())
                } else {
                    extract_content_parts(source_content)
                };

            // Add main content to merged content (will go into target's Details)
            if !main_content.trim().is_empty() {
                merged_content.push_str(&format!("\n{}\n", main_content.trim()));
            }

            // Add details to "Merged Details (element name)" subsection
            if !details_content.trim().is_empty() {
                merged_content.push_str(&format!(
                    "\n#### Merged Details ({})\n{}\n",
                    source_name,
                    details_content.trim()
                ));
            }

            // Collect relations
            for rel in source_relations {
                let skip_relation = match &rel.target.link {
                    LinkType::Identifier(relation_target) => {
                        self.resolve_relation_identifier(source_element, relation_target)
                            .is_some_and(|resolved| merged_source_ids.contains(&resolved))
                            || merged_source_ids.contains(relation_target)
                    }
                    _ => false,
                };
                if !skip_relation {
                    merged_relations.push(rel.clone());
                }
            }

            // Collect contract_bindings
            for att in source_contract_bindings {
                merged_contract_bindings.push(att.clone());
            }

            // Track source file as modified
            let source_file = self
                .nodes
                .get(source_id)
                .expect("node not found in registry")
                .element
                .file_path
                .clone();
            self.modified_files.insert(source_file);
        }

        // Deduplicate relations by (relation_type, target)
        let mut seen_relations: FxHashSet<(String, String)> = FxHashSet::default();
        merged_relations.retain(|r| {
            let key = (
                r.relation_type.name.to_string(),
                r.target.link.as_str().to_string(),
            );
            if seen_relations.contains(&key) {
                false
            } else {
                seen_relations.insert(key);
                true
            }
        });

        // Deduplicate contract_bindings by target
        let mut seen_contract_bindings: FxHashSet<String> = FxHashSet::default();
        merged_contract_bindings.retain(|a| {
            let key = a.target.as_str().to_string();
            if seen_contract_bindings.contains(&key) {
                false
            } else {
                seen_contract_bindings.insert(key);
                true
            }
        });

        // Validate contract_bindings scope constraints for target element
        for contract_bindings in &merged_contract_bindings {
            if let crate::element::ContractBindingTarget::ElementIdentifier(ref att_id) =
                contract_bindings.target
            {
                // Check orphan contract constraint
                if !self.contract_has_define_relation(att_id) {
                    let contract_binding_name = self
                        .nodes
                        .get(att_id)
                        .map(|n| n.element.name.as_str())
                        .unwrap_or(att_id);
                    return Err(ReqvireError::InvalidContractBindingTarget(
                        format!(
                            "'{}' has no define relation. Contracts must define a requirement before they can be reused; contracts are requirement-owned only. Capabilities use concept references for SKOS concepts and are specified by requirements; verification coverage rolls up from verified requirements.",
                            contract_binding_name
                        ),
                    ));
                }

                // Check hierarchical independence constraint
                let defining_reqs = self.get_defining_requirements(att_id);
                for defining_req_id in defining_reqs {
                    if self.is_in_hierarchy(target_id, &defining_req_id) {
                        let contract_binding_name = self
                            .nodes
                            .get(att_id)
                            .map(|n| n.element.name.as_str())
                            .unwrap_or(att_id);
                        return Err(ReqvireError::InvalidContractBindingScope(
                            format!(
                                "'{}' cannot be bound to '{}' because it is within the contract's defining hierarchy. Contract Bindings are only allowed from elements outside the definedBy chain.",
                                contract_binding_name,
                                target_name
                            ),
                        ));
                    }
                }

                if let Some(msg) = self.build_contract_bindings_direction_scope_error(
                    att_id,
                    target_id,
                    &target_name,
                    None,
                ) {
                    return Err(ReqvireError::InvalidContractBindingScope(msg));
                }
            }
        }

        // Check for cross-section duplicates
        let relation_targets: FxHashSet<String> = merged_relations
            .iter()
            .map(|r| r.target.link.as_str().to_string())
            .collect();

        for contract_bindings in &merged_contract_bindings {
            let target = contract_bindings.target.as_str();
            if relation_targets.contains(&target) {
                return Err(ReqvireError::MergeCrossSectionDuplicate(format!(
                    "Target '{}' would appear in both Relations and Contract Bindings after merge. Remove one before merging.",
                    target
                )));
            }
        }

        // Update target element with merged data
        {
            let target_node = self
                .nodes
                .get_mut(target_id)
                .expect("node not found in registry");
            let target_element = &mut target_node.element;

            // Merge content into target's Details section
            if !merged_content.trim().is_empty() {
                target_element.content =
                    merge_content_into_details(&target_element.content, &merged_content);
            }

            if target_is_ontology {
                let merged_ontology_block = merge_ontology_blocks_into_target(
                    &target_element.content,
                    &target_element_for_merge,
                    &source_data
                        .iter()
                        .map(|(_, _, _, _, _, element)| element.clone())
                        .collect::<Vec<_>>(),
                )?;
                target_element.content = replace_single_fenced_subsection(
                    &target_element.content,
                    "Ontology",
                    &merged_ontology_block,
                )?;
            }

            target_element.relations = merged_relations;
            target_element.contract_bindings = merged_contract_bindings;
        }

        self.modified_files.insert(target_file_path);

        // CRITICAL: Before removing sources, handle opposite relations
        // Find all elements with relations TO sources and recreate their opposites to point to target
        let target_node = self
            .nodes
            .get(target_id)
            .expect("node not found in registry");
        let target_name = target_node.element.name.clone();
        let target_element_id = target_node.element.id.clone();

        for (source_id, _, _, _, _, _) in &source_data {
            // Find all elements that have user_created relations pointing TO this source
            let elements_with_relations_to_source: Vec<(String, Vec<Relation>)> = self.nodes.iter()
                .filter(|(id, _)| *id != source_id && *id != target_id)
                .filter_map(|(referrer_id, node)| {
                    let rels: Vec<_> = node.element.relations.iter()
                        .filter(|r| r.user_created)
                        .filter(|r| matches!(&r.target.link, LinkType::Identifier(id) if id == source_id))
                        .cloned()
                        .collect();
                    if rels.is_empty() {
                        None
                    } else {
                        Some((referrer_id.clone(), rels))
                    }
                })
                .collect();

            // For each element with relations to source, update opposite relations
            for (referrer_id, relations_to_source) in elements_with_relations_to_source {
                for relation in relations_to_source {
                    if let Some(opposite_type_name) = relation.relation_type.opposite {
                        // Remove old opposite pointing to source
                        self.remove_opposite_from_target(
                            source_id,
                            &referrer_id,
                            opposite_type_name,
                        );

                        // Create new opposite pointing to target (will be added to target after merge)
                        // Note: The referrer's relation will be redirected by redirect_relations_to_target(),
                        // so we create opposite on target now
                        self.add_opposite_to_target(
                            &relation,
                            target_id,
                            &target_name,
                            &target_element_id,
                        );
                    }
                }
            }
        }

        // Redirect relations from other elements pointing to sources to point to target
        for (source_id, _, _, _, _, _) in &source_data {
            self.redirect_relations_to_target(source_id, target_id)?;
        }

        // Remove source elements (this also removes them from the graph)
        for (source_id, _, _, _, _, _) in &source_data {
            self.remove_element(source_id)?;
        }

        Ok(())
    }

    /// Redirect all relations pointing to source_id to point to target_id
    fn redirect_relations_to_target(
        &mut self,
        source_id: &str,
        target_id: &str,
    ) -> Result<(), ReqvireError> {
        // Find all nodes with relations pointing to source_id
        let nodes_to_update: Vec<String> = self
            .nodes
            .iter()
            .filter(|(id, node)| {
                *id != source_id
                    && *id != target_id
                    && node.element.relations.iter().any(|r| match &r.target.link {
                        LinkType::Identifier(ref id) => self.relation_targets_same_identifier(
                            &node.element.file_path,
                            id,
                            source_id,
                        ),
                        _ => false,
                    })
            })
            .map(|(id, _)| id.clone())
            .collect();

        for node_id in nodes_to_update {
            let mut relations = self
                .nodes
                .get(&node_id)
                .map(|node| node.element.relations.clone())
                .unwrap_or_default();
            let file_path = self
                .nodes
                .get(&node_id)
                .map(|node| node.element.file_path.clone())
                .unwrap_or_default();
            let mut changed = false;
            for relation in &mut relations {
                if let LinkType::Identifier(ref id) = relation.target.link {
                    if self.relation_targets_same_identifier(&file_path, id, source_id) {
                        relation.target = crate::relation::RelationTarget {
                            text: target_id.to_string(),
                            link: LinkType::Identifier(target_id.to_string()),
                            element_id: Some(target_id.to_string()),
                        };
                        changed = true;
                    }
                }
            }
            if changed {
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    node.element.relations = relations;
                    self.modified_files.insert(file_path);
                }
            }
        }

        // CRITICAL: Also redirect opposite relations (auto-generated, user_created=false)
        // Get target element info for creating correct opposite targets
        let (target_name, target_element_id) = if let Some(target_node) = self.nodes.get(target_id)
        {
            (
                target_node.element.name.clone(),
                target_node.element.id.clone(),
            )
        } else {
            return Ok(()); // Target doesn't exist
        };

        // Find and update opposite relations pointing to source
        for node_id in self.nodes.keys().cloned().collect::<Vec<_>>() {
            if node_id == source_id || node_id == target_id {
                continue;
            }

            let source_file_path = self
                .nodes
                .get(&node_id)
                .map(|node| node.element.file_path.clone());
            let mut relations = self
                .nodes
                .get(&node_id)
                .map(|node| node.element.relations.clone())
                .unwrap_or_default();

            let Some(source_file_path) = source_file_path else {
                continue;
            };

            let mut changed = false;

            // Update auto-generated opposite relations pointing to source
            for relation in &mut relations {
                if !relation.user_created {
                    // Only auto-generated opposites
                    if let LinkType::Identifier(ref id) = relation.target.link {
                        if self.relation_targets_same_identifier(&source_file_path, id, source_id) {
                            relation.target = crate::relation::RelationTarget {
                                text: target_name.clone(),
                                link: LinkType::Identifier(target_id.to_string()),
                                element_id: Some(target_element_id.clone()),
                            };
                            // Note: Do NOT mark file as modified - opposite is user_created=false
                            changed = true;
                        }
                    }
                }
            }
            if changed {
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    node.element.relations = relations;
                }
            }
        }

        Ok(())
    }

    /// Flushes only modified files to directory (optimization)
    pub fn flush_modified_files(&mut self, directory: &Path) -> Result<(), ReqvireError> {
        if self.modified_files.is_empty() {
            return Ok(());
        }

        let file_vec: Vec<String> = self.modified_files.iter().cloned().collect();
        let _result = self.flush_files_to_directory(&file_vec, directory, false)?;

        // Check for and delete empty files (files with no elements)
        let grouped_elements = self.group_elements_by_location();
        for file_path in &file_vec {
            if !grouped_elements.contains_key(file_path) {
                // This file has no elements, delete it
                let file_full_path = directory.join(file_path);
                if file_full_path.exists() {
                    fs::remove_file(&file_full_path).map_err(ReqvireError::IoError)?;
                    log::info!("Deleted empty file: {}", file_path);
                }
            }
        }

        self.modified_files.clear();
        Ok(())
    }

    /// Clears the modified files tracking
    pub fn clear_modified_files(&mut self) {
        self.modified_files.clear();
    }
}
