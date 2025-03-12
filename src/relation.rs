use anyhow::Result;
use log::debug;
use std::path::Path;
use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::error::ReqFlowError;

// Define relation direction types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelationDirection {
    Forward,   // From source to target (e.g., "refine")
    Backward,  // From target to source (e.g., "refinedBy")
    Neutral    // Non-directional (e.g., "trace")
}

// Define a struct to hold relation type information
#[derive(Debug, Clone)]
pub struct RelationTypeInfo {
    pub name: &'static str,          // Name of the relation type
    pub direction: RelationDirection, // Direction of relation
    pub opposite: Option<&'static str>, // The opposite relation name, if any
    pub description: &'static str,    // Description of what the relation means
}

// Define only the exact relation types specified in DSD_RepresentationOfIdentifiersAndRelations.md
lazy_static! {
    pub static ref RELATION_TYPES: HashMap<&'static str, RelationTypeInfo> = {
        let mut m = HashMap::new();
        
        // Containment relations
        m.insert("containedBy", RelationTypeInfo {
            name: "containedBy", 
            direction: RelationDirection::Backward, 
            opposite: Some("contain"),
            description: "Element is contained by another element",
        });
        m.insert("contain", RelationTypeInfo {
            name: "contain", 
            direction: RelationDirection::Forward, 
            opposite: Some("containedBy"),
            description: "Element contains another element",
        });
        
        // Derive relations
        m.insert("derivedFrom", RelationTypeInfo {
            name: "derivedFrom", 
            direction: RelationDirection::Backward, 
            opposite: Some("derive"),
            description: "Element is source for a derived element",
        });
        m.insert("derive", RelationTypeInfo {
            name: "derive", 
            direction: RelationDirection::Forward, 
            opposite: Some("derivedFrom"),
            description: "Element is derived from another element",
        });
        
        // Refine relation
        m.insert("refine", RelationTypeInfo {
            name: "refine", 
            direction: RelationDirection::Forward, 
            opposite: None,
            description: "Element refines a higher-level element",
        });
        
        // Satisfy relations
        m.insert("satisfiedBy", RelationTypeInfo {
            name: "satisfiedBy", 
            direction: RelationDirection::Backward, 
            opposite: Some("satisfy"),
            description: "Element is satisfied by another element",
        });
        m.insert("satisfy", RelationTypeInfo {
            name: "satisfy", 
            direction: RelationDirection::Forward, 
            opposite: Some("satisfiedBy"),
            description: "Element satisfies another element",
        });
        
        // Verify relations
        m.insert("verifiedBy", RelationTypeInfo {
            name: "verifiedBy", 
            direction: RelationDirection::Backward, 
            opposite: Some("verify"),
            description: "Element is verified by another element",
        });
        m.insert("verify", RelationTypeInfo {
            name: "verify", 
            direction: RelationDirection::Forward, 
            opposite: Some("verifiedBy"),
            description: "Element verifies another element",
        });
        
        // Trace relations
        m.insert("tracedFrom", RelationTypeInfo {
            name: "tracedFrom", 
            direction: RelationDirection::Backward, 
            opposite: None,
            description: "Element is traced from another element",
        });
        m.insert("trace", RelationTypeInfo {
            name: "trace", 
            direction: RelationDirection::Neutral, 
            opposite: None,
            description: "Element is related to another element in a non-directional way",
        });
        
        m
    };
}

/// Check if a relation type is supported according to the DSD
pub fn is_supported_relation_type(relation_type: &str) -> bool {
    RELATION_TYPES.contains_key(relation_type)
}

/// Get the list of all supported relation types
pub fn get_supported_relation_types() -> Vec<&'static str> {
    RELATION_TYPES.keys().cloned().collect()
}

/// Get the list of valid parent relation types
/// These are relations that indicate an element has a parent (hierarchical relationship)
pub fn get_parent_relation_types() -> Vec<&'static str> {
    vec!["derivedFrom", "tracedFrom", "refine", "containedBy"]
}

/// Get information about a relation type
pub fn get_relation_type_info(relation_type: &str) -> Option<&RelationTypeInfo> {
    RELATION_TYPES.get(relation_type)
}

/// Represents a relation between elements in the MBSE model
#[derive(Debug, Clone)]
pub struct Relation {
    /// Type of the relation (e.g., dependsOn, verifiedBy)
    pub relation_type: String,
    
    /// Target identifier of the relation (may be markdown link syntax or plain text)
    pub target: String,
    
    /// URL part of the target (if it's a markdown link)
    pub url: Option<String>,
    
    /// Whether the relation has been processed into a markdown link
    #[allow(dead_code)]
    pub processed: bool,
}

impl Relation {
    /// Create a new relation
    pub fn new(relation_type: String, target: String) -> Self {
        // Check if target is a markdown link and extract the URL part if it is
        let url = if target.contains("](") && target.contains("]") && target.contains("(") && target.contains(")") {
            // Extract URL from markdown link
            let url_start = target.find("(").unwrap() + 1;
            let url_end = target.rfind(")").unwrap();
            
            // Get the raw URL without markdown syntax
            let raw_url = target[url_start..url_end].to_string();
            
            // Fix URL if it contains '#' where it should have '/' in a file path
            // This fixes paths like "DesignSpecifications#Status.md"
            let fixed_url = if raw_url.contains('#') && !raw_url.starts_with('#') &&
                            raw_url.split('#').last().map_or(false, |last| last.contains('.')) {
                // This is likely a path with hash that should be slash
                raw_url.replace('#', "/")
            } else {
                raw_url
            };
            
            Some(fixed_url)
        } else {
            None
        };
        
        Self {
            relation_type,
            target,
            url,
            processed: false,
        }
    }

    /// Convert a relation to a markdown link
    /// The convert_to_html parameter determines whether to generate links to HTML files (true) or markdown files (false)
    pub fn to_markdown_link(&self, _current_file: &Path, convert_to_html: bool) -> Result<String, ReqFlowError> {
        let target = self.target.trim();
        
        // For debugging
        debug!("Processing relation: {} -> {}", self.relation_type, target);
        
        // Any link should convert between .md and .html based on output mode
        let mut enhanced_target = target.to_string();
        
        // Check if this is a markdown link already (e.g., [text](url))
        if enhanced_target.contains("](") && enhanced_target.contains(")") {
            debug!("Target appears to be a markdown link already: {}", enhanced_target);
            
            // Extract the URL part from the markdown link
            let url_start = enhanced_target.find("](").unwrap() + 2;
            let url_end = enhanced_target.rfind(")").unwrap();
            let url = &enhanced_target[url_start..url_end];
            
            if convert_to_html && url.ends_with(".md") {
                // Replace .md with .html in the URL part only
                let new_url = url.replace(".md", ".html");
                enhanced_target = format!("{}{}{}",
                    &enhanced_target[0..url_start],
                    new_url,
                    &enhanced_target[url_end..]
                );
                debug!("Converted markdown link URL to HTML: {}", enhanced_target);
                
                // Return immediately with the formatted relation
                return Ok(format!("  * {}: {}", self.relation_type, enhanced_target));
            }
            
            // If it's already a properly formatted markdown link, just return it
            return Ok(format!("  * {}: {}", self.relation_type, enhanced_target));
        }
        
        // We need to specifically handle the case where the link already has a .md extension
        // and ensure it's converted to .html when in HTML mode
        if convert_to_html && enhanced_target.ends_with(".md") {
            debug!("Converting .md link to .html: {}", enhanced_target);
            enhanced_target = enhanced_target.replace(".md", ".html");
        }
        
        // If we're generating HTML and this is a directory path without an extension,
        // we need to add the appropriate extension
        if convert_to_html && 
           enhanced_target.contains('/') && 
           !enhanced_target.contains('.') {
            debug!("Adding .html extension to directory path: {}", enhanced_target);
            enhanced_target = format!("{}.html", enhanced_target);
        } else if !convert_to_html && 
                 enhanced_target.contains('/') && 
                 !enhanced_target.contains('.') {
            debug!("Adding .md extension to directory path: {}", enhanced_target);
            enhanced_target = format!("{}.md", enhanced_target);
        }
        
        // Use the enhanced target for processing
        let target = enhanced_target.as_str();
        
        // According to ReqFlow specification, identifiers can be:
        // 1. Relative path with fragment: 'relative_path/file.md/element name'
        // 2. File with fragment: 'file.md/element name'
        // 3. File only: 'file.md'
        // 4. Element name only: 'element name'
        
        let link = if target.contains('/') {
            // Case 1 & 2: Path with fragment or file with fragment
            if target.ends_with(".md") || target.ends_with(".html") {
                // Case 3: File only reference (ends with extension)
                // Always use the enhanced_target which already has the correct extension
                format!("[{}]({})", self.target, target)
            } else {
                // Case 1 & 2: Contains path separator and not ending with .md
                // Check if this is a path/file.md/element format
                if target.contains(".md/") {
                    // This is path/file.md/element format
                    // Split at the last occurrence of .md/ to get file path and element
                    let md_pos = target.rfind(".md/").unwrap();
                    let file_part = &target[0..md_pos+3]; // Include the .md
                    let element_part = &target[md_pos+4..]; // Skip the .md/
                    
                    // We need to ensure .md files are properly converted to .html in HTML mode
                    let file_part = if convert_to_html && file_part.ends_with(".md") {
                        file_part.replace(".md", ".html")
                    } else if !convert_to_html && file_part.ends_with(".html") {
                        file_part.replace(".html", ".md")
                    } else {
                        file_part.to_string()
                    };
                    
                    let anchor = element_part.replace(' ', "-").to_lowercase();
                    format!("[{}]({}#{})", self.target, file_part, anchor) // Use original target for display
                } else if target.contains(".html/") {
                    // Similar handling for .html/ format
                    let html_pos = target.rfind(".html/").unwrap();
                    let file_part = &target[0..html_pos+5]; // Include the .html
                    let element_part = &target[html_pos+6..]; // Skip the .html/
                    
                    let file_part = if convert_to_html {
                        file_part.to_string()
                    } else {
                        file_part.replace(".html", ".md")
                    };
                    
                    let anchor = element_part.replace(' ', "-").to_lowercase();
                    format!("[{}]({}#{})", self.target, file_part, anchor) // Use original target for display
                } else {
                    // This might be a path reference without file extension
                    // Split at the last /
                    let parts: Vec<&str> = target.rsplitn(2, '/').collect();
                    if parts.len() != 2 {
                        return Err(ReqFlowError::InvalidIdentifier(target.to_string()));
                    }
                    
                    let (element_part, file_part) = (parts[0], parts[1]);
                    
                    // Add proper extension
                    let file_with_ext = if convert_to_html {
                        format!("{}.html", file_part)
                    } else {
                        format!("{}.md", file_part)
                    };
                    
                    let anchor = element_part.replace(' ', "-").to_lowercase();
                    format!("[{}]({}#{})", self.target, file_with_ext, anchor) // Use original target for display
                }
            }
        } else if target.ends_with(".md") || target.ends_with(".html") {
            // Case 3: Simple file reference with extension
            let target_file = if convert_to_html {
                target.replace(".md", ".html")
            } else if target.ends_with(".html") {
                target.replace(".html", ".md")
            } else {
                target.to_string()
            };
            format!("[{}]({})", self.target, target_file) // Use original target for display
        } else {
            // Case 4: Element name only or document without extension
            
            // Check if this is a likely document reference
            let is_likely_doc = target.contains("DSD") || 
                                target.contains("Specification") || 
                                target.contains("Document") || 
                                target.to_lowercase() == "status";
            
            if is_likely_doc {
                debug!("Processing as document reference: {}", target);
                
                // For "Status DSD" we need to handle it specially
                if target.to_lowercase() == "status" {
                    let file_name = if convert_to_html {
                        "Status_DSD.html"
                    } else {
                        "Status_DSD.md"
                    };
                    format!("[{}]({})", self.target, file_name) // Use original target for display
                } else {
                    // Normal document reference - add file extension
                    let file_name = if convert_to_html {
                        format!("{}.html", target)
                    } else {
                        format!("{}.md", target)
                    };
                    format!("[{}]({})", self.target, file_name) // Use original target for display
                }
            } else {
                // Same-document element reference
                let anchor = target.replace(' ', "-").to_lowercase();
                format!("[{}](#{})", self.target, anchor) // Use original target for display
            }
        };
        
        Ok(format!("  * {}: {}", self.relation_type, link))
    }

    /// Validate that the relation type follows the required format
    #[allow(dead_code)]
    pub fn validate_type(&self,element_file_path:String, element_name:String) -> Result<(), ReqFlowError> {
        // Normalize the type by trimming whitespace
        let normalized_type = self.relation_type.trim();
        
        // Check if the relation type is supported according to the DSD
        if !is_supported_relation_type(normalized_type) {
            let supported_types = get_supported_relation_types().join(", ");
            return Err(ReqFlowError::UnsupportedRelationType(format!(
                "'{} (in file '{}' and element '{}')'. Supported types are: {}",
                normalized_type,element_file_path, element_name, supported_types
            )));
        }
        
        Ok(())
    }
}

/// Process a relations section in markdown
pub fn process_relations(content: &str, current_file: &Path, convert_to_html: bool) -> Result<String, ReqFlowError> {
    use regex::Regex;
    
    lazy_static::lazy_static! {
        // Super flexible regex pattern that handles multiple bullet and indentation styles
        static ref RELATION_REGEX: Regex = Regex::new(r"(?:^\s*[\*\-]|\s{2}[\*\-])\s*(\w+):\s*(.+)").unwrap();
        static ref MARKDOWN_LINK_REGEX: Regex = Regex::new(r"\[.+?\]\(.+?\)").unwrap();
        // Fallback regex that just looks for relation pattern anywhere in a line
        static ref FALLBACK_RELATION_REGEX: Regex = Regex::new(r"(\w+):\s*([A-Za-z0-9_\./]+)").unwrap();
        // Regex to find .md in URL part of markdown links - Enhanced to handle various formats
        static ref MD_LINK_REGEX: Regex = Regex::new(r"(\]\([^)]+)\.md(\))").unwrap();
    }
    
    // Debug output
    debug!("Processing relations in document: {}", current_file.display());
    
    // Process only relations sections
    let mut in_relations_section = false;
    let mut updated_lines = Vec::new();
    let mut relations_found = 0;
    
    for line in content.lines() {
        // Recognize various Relations section headers
        if line.trim() == "#### Relations" || line.trim() == "### Relations" || 
           line.trim() == "Relations:" || line.trim() == "## Relations" {
            debug!("Found Relations section in {}", current_file.display());
            in_relations_section = true;
            updated_lines.push(line.to_string());
        } else if in_relations_section && !(line.trim().starts_with("* ") || 
                                          line.trim().starts_with("  * ") ||
                                          line.trim().starts_with("- ") || 
                                          line.trim().starts_with("  - ") ||
                                          FALLBACK_RELATION_REGEX.is_match(line)) {
            debug!("Exiting relations section at line: {}", line);
            in_relations_section = false;
            updated_lines.push(line.to_string());
        } else if in_relations_section {
            debug!("Processing relation line: {}", line);
            if MARKDOWN_LINK_REGEX.is_match(line) {
                // Process existing markdown links to convert .md to .html if needed
                debug!("Line contains a markdown link, processing: {}", line);
                
                // Extract relation type and target (the markdown link) from the line
                let relation_type_and_link = RELATION_REGEX.captures(line)
                    .or_else(|| FALLBACK_RELATION_REGEX.captures(line));
                
                if let Some(captures) = relation_type_and_link {
                    // If we could extract relation type and target
                    let relation_type = captures.get(1).unwrap().as_str().to_string();
                    let target = captures.get(2).unwrap().as_str().trim().to_string();
                    
                    debug!("Extracted relation from markdown link line: {} -> {}", relation_type, target);
                    
                    // Create a relation object and let its to_markdown_link method handle conversion
                    let relation = Relation::new(relation_type, target);
                    match relation.to_markdown_link(current_file, convert_to_html) {
                        Ok(converted_line) => {
                            debug!("Converted markdown link through relation: {}", converted_line);
                            updated_lines.push(converted_line);
                        },
                        Err(e) => {
                            debug!("Error converting relation with markdown link: {}", e);
                            // Fall back to regex-based conversion
                            if convert_to_html {
                                let converted_line = MD_LINK_REGEX.replace_all(line, "${1}.html${2}").to_string();
                                debug!("Fallback conversion: {} -> {}", line, converted_line);
                                updated_lines.push(converted_line);
                            } else {
                                updated_lines.push(line.to_string());
                            }
                        }
                    }
                } else {
                    // If we couldn't extract relation and target, fall back to regex-based conversion
                    debug!("Could not extract relation from markdown link line, using regex");
                    if convert_to_html {
                        let converted_line = MD_LINK_REGEX.replace_all(line, "${1}.html${2}").to_string();
                        
                        if converted_line != line {
                            debug!("Converted markdown link: {} -> {}", line, converted_line);
                            updated_lines.push(converted_line);
                        } else {
                            updated_lines.push(line.to_string());
                        }
                    } else {
                        updated_lines.push(line.to_string());
                    }
                }
            } else {
                // Process relation to create a link - try multiple approaches
                let captures = RELATION_REGEX.captures(line)
                    .or_else(|| {
                        // If standard regex fails, try the fallback
                        debug!("Trying fallback relation detection for: {}", line);
                        FALLBACK_RELATION_REGEX.captures(line)
                    });
                
                if let Some(captures) = captures {
                    relations_found += 1;
                    let relation_type = captures.get(1).unwrap().as_str().to_string();
                    let target = captures.get(2).unwrap().as_str().trim().to_string();
                    
                    debug!("Found relation: {} -> {}", relation_type, target);
                    
                    let relation = Relation::new(relation_type, target);
                    match relation.to_markdown_link(current_file, convert_to_html) {
                        Ok(link) => {
                            debug!("Converted to link: {}", link);
                            
                            // Preserve the original bullet style (- or *)
                            let bullet = if line.trim().starts_with("- ") || line.trim().starts_with("  - ") {
                                "-"
                            } else {
                                "*"
                            };
                            
                            // Replace the bullet in the link if needed
                            let link = if bullet == "-" {
                                link.replace("  * ", "  - ")
                            } else {
                                link
                            };
                            
                            updated_lines.push(link)
                        },
                        Err(e) => {
                            debug!("Error converting relation to link: {}", e);
                            updated_lines.push(line.to_string())
                        },
                    }
                } else {
                    // If the line doesn't match the relation pattern, keep it as is
                    debug!("Line doesn't match relation pattern, keeping as is");
                    updated_lines.push(line.to_string());
                }
            }
        } else {
            updated_lines.push(line.to_string());
        }
    }
    
    debug!("Processed {} relations in document {}", relations_found, current_file.display());
    Ok(updated_lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use regex::Regex;

    #[test]
    fn test_markdown_link_conversion() {
        // Test various relation targets and how they convert to links
        let test_cases = vec![
            // Format: (relation_type, target, convert_to_html, expected_output)
            
            // Test case 1: Basic .md conversion to .html
            ("satisfiedBy", "DesignSpecifications/EventNotifications.md", true, 
             "  * satisfiedBy: [DesignSpecifications/EventNotifications.md](DesignSpecifications/EventNotifications.html)"),
             
            // Test case 2: Directory path gets .html extension
            ("satisfiedBy", "DesignSpecifications/EventNotifications", true, 
             "  * satisfiedBy: [DesignSpecifications/EventNotifications](DesignSpecifications/EventNotifications.html)"),
             
            // Test case 3: Directory path gets .md extension in markdown mode
            ("satisfiedBy", "DesignSpecifications/EventNotifications", false, 
             "  * satisfiedBy: [DesignSpecifications/EventNotifications](DesignSpecifications/EventNotifications.md)"),
             
            // Test case 4: .md file stays as .md in markdown mode
            ("satisfiedBy", "DesignSpecifications/EventNotifications.md", false, 
             "  * satisfiedBy: [DesignSpecifications/EventNotifications.md](DesignSpecifications/EventNotifications.md)"),
             
            // Test case 5: Element reference in file
            ("satisfiedBy", "DesignSpecifications/EventNotifications.md/Element Name", true, 
             "  * satisfiedBy: [DesignSpecifications/EventNotifications.md/Element Name](DesignSpecifications/EventNotifications.html#element-name)"),
             
            // Test case 6: Already formatted markdown link - convert to HTML
            ("satisfiedBy", "[DesignSpecifications/ActivityFeed.md](DesignSpecifications/ActivityFeed.md)", true, 
             "  * satisfiedBy: [DesignSpecifications/ActivityFeed.md](DesignSpecifications/ActivityFeed.html)"),
             
            // Test case 7: Already formatted markdown link - keep as markdown
            ("satisfiedBy", "[DesignSpecifications/ActivityFeed.md](DesignSpecifications/ActivityFeed.md)", false, 
             "  * satisfiedBy: [DesignSpecifications/ActivityFeed.md](DesignSpecifications/ActivityFeed.md)"),
        ];
        
        // Test pre-existing markdown links
        let markdown_text = "  * satisfiedBy: [DesignSpecifications/ActivityFeed.md](DesignSpecifications/ActivityFeed.md)";
        let expected_html = "  * satisfiedBy: [DesignSpecifications/ActivityFeed.md](DesignSpecifications/ActivityFeed.html)";
        
        // Verify that our regex patterns match correctly
        let link_regex = Regex::new(r"\[.+?\]\(.+?\)").unwrap();
        assert!(link_regex.is_match(markdown_text), "Markdown link regex pattern failed to match test case");
        
        // Make sure our relation pattern also matches correctly
        let relation_regex = Regex::new(r"(?:^\s*[\*\-]|\s{2}[\*\-])\s*(\w+):\s*(.+)").unwrap();
        let captures = relation_regex.captures(markdown_text);
        assert!(captures.is_some(), "Relation regex failed to match");
        
        if let Some(captures) = captures {
            assert_eq!(captures.get(1).unwrap().as_str(), "satisfiedBy");
            assert!(captures.get(2).unwrap().as_str().contains("[DesignSpecifications"));
        }
        
        // Test that process_relations correctly converts .md links to .html links
        // We need to include a Relations section header to trigger the processing
        let markdown_with_header = format!("#### Relations\n{}", markdown_text);
        let result = process_relations(&markdown_with_header, Path::new("test.md"), true).unwrap();
        
        // Expect the output to have the header and converted links
        let expected_with_header = format!("#### Relations\n{}", expected_html);
        assert_eq!(result, expected_with_header, "Failed to convert existing markdown link to HTML");
        
        for (relation_type, target, convert_to_html, expected) in test_cases {
            let relation = Relation::new(relation_type.to_string(), target.to_string());
            let result = relation.to_markdown_link(Path::new("current_file.md"), convert_to_html).unwrap();
            assert_eq!(result, expected, "Failed test case: {} -> {} (convert_to_html={})", relation_type, target, convert_to_html);
        }
    }
}
