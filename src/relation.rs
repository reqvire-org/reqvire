use anyhow::Result;
use log::info;
use std::path::Path;

use crate::error::ReqFlowError;

/// Represents a relation between elements in the MBSE model
#[derive(Debug, Clone)]
pub struct Relation {
    /// Type of the relation (e.g., dependsOn, verifiedBy)
    pub relation_type: String,
    
    /// Target identifier of the relation
    pub target: String,
    
    /// Whether the relation has been processed into a markdown link
    #[allow(dead_code)]
    pub processed: bool,
}

impl Relation {
    /// Create a new relation
    pub fn new(relation_type: String, target: String) -> Self {
        Self {
            relation_type,
            target,
            processed: false,
        }
    }

    /// Convert a relation to a markdown link
    /// The convert_to_html parameter determines whether to generate links to HTML files (true) or markdown files (false)
    pub fn to_markdown_link(&self, _current_file: &Path, convert_to_html: bool) -> Result<String, ReqFlowError> {
        let target = self.target.trim();
        
        // For debugging
        info!("Processing relation: {} -> {}", self.relation_type, target);
        
        // Any link should convert between .md and .html based on output mode
        let mut enhanced_target = target.to_string();
        
        // Check if this is a markdown link already (e.g., [text](url))
        if enhanced_target.contains("](") && enhanced_target.contains(")") {
            info!("Target appears to be a markdown link already: {}", enhanced_target);
            
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
                info!("Converted markdown link URL to HTML: {}", enhanced_target);
                
                // Return immediately with the formatted relation
                return Ok(format!("  * {}: {}", self.relation_type, enhanced_target));
            }
            
            // If it's already a properly formatted markdown link, just return it
            return Ok(format!("  * {}: {}", self.relation_type, enhanced_target));
        }
        
        // We need to specifically handle the case where the link already has a .md extension
        // and ensure it's converted to .html when in HTML mode
        if convert_to_html && enhanced_target.ends_with(".md") {
            info!("Converting .md link to .html: {}", enhanced_target);
            enhanced_target = enhanced_target.replace(".md", ".html");
        }
        
        // If we're generating HTML and this is a directory path without an extension,
        // we need to add the appropriate extension
        if convert_to_html && 
           enhanced_target.contains('/') && 
           !enhanced_target.contains('.') {
            info!("Adding .html extension to directory path: {}", enhanced_target);
            enhanced_target = format!("{}.html", enhanced_target);
        } else if !convert_to_html && 
                 enhanced_target.contains('/') && 
                 !enhanced_target.contains('.') {
            info!("Adding .md extension to directory path: {}", enhanced_target);
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
            } else {
                if target.ends_with(".html") {
                    target.replace(".html", ".md")
                } else {
                    target.to_string()
                }
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
                info!("Processing as document reference: {}", target);
                
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
    pub fn validate_type(&self) -> Result<(), ReqFlowError> {
        // Normalize the type by trimming whitespace
        let normalized_type = self.relation_type.trim();
        
        // More relaxed validation: 
        // Relation type should be at least 2 characters and at most 80 characters
        if normalized_type.len() < 2 || normalized_type.len() > 80 {
            return Err(ReqFlowError::InvalidRelationFormat(format!(
                "Relation type '{}' must be between 2 and 80 characters",
                self.relation_type
            )));
        }
        
        // Very basic check - allow any alphanumeric characters for relation types to be more permissive
        // This allows camelCase and other formats commonly used in ReqFlow
        if !normalized_type.chars().all(|c| c.is_alphanumeric()) {
            return Err(ReqFlowError::InvalidRelationFormat(format!(
                "Relation type '{}' must contain only alphanumeric characters",
                self.relation_type
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
        // Regex to find .md in URL part of markdown links
        static ref MD_LINK_REGEX: Regex = Regex::new(r"(\]\([^)]+)\.md(\))").unwrap();
    }
    
    // Debug output
    info!("Processing relations in document: {}", current_file.display());
    
    // Process only relations sections
    let mut in_relations_section = false;
    let mut updated_lines = Vec::new();
    let mut relations_found = 0;
    
    for line in content.lines() {
        // Recognize various Relations section headers
        if line.trim() == "#### Relations" || line.trim() == "### Relations" || 
           line.trim() == "Relations:" || line.trim() == "## Relations" {
            info!("Found Relations section in {}", current_file.display());
            in_relations_section = true;
            updated_lines.push(line.to_string());
        } else if in_relations_section && !(line.trim().starts_with("* ") || 
                                          line.trim().starts_with("  * ") ||
                                          line.trim().starts_with("- ") || 
                                          line.trim().starts_with("  - ") ||
                                          FALLBACK_RELATION_REGEX.is_match(line)) {
            info!("Exiting relations section at line: {}", line);
            in_relations_section = false;
            updated_lines.push(line.to_string());
        } else if in_relations_section {
            info!("Processing relation line: {}", line);
            if MARKDOWN_LINK_REGEX.is_match(line) {
                // Process existing markdown links to convert .md to .html if needed
                info!("Line contains a markdown link, processing: {}", line);
                
                // Extract relation type and target (the markdown link) from the line
                let relation_type_and_link = RELATION_REGEX.captures(line)
                    .or_else(|| FALLBACK_RELATION_REGEX.captures(line));
                
                if let Some(captures) = relation_type_and_link {
                    // If we could extract relation type and target
                    let relation_type = captures.get(1).unwrap().as_str().to_string();
                    let target = captures.get(2).unwrap().as_str().trim().to_string();
                    
                    info!("Extracted relation from markdown link line: {} -> {}", relation_type, target);
                    
                    // Create a relation object and let its to_markdown_link method handle conversion
                    let relation = Relation::new(relation_type, target);
                    match relation.to_markdown_link(current_file, convert_to_html) {
                        Ok(converted_line) => {
                            info!("Converted markdown link through relation: {}", converted_line);
                            updated_lines.push(converted_line);
                        },
                        Err(e) => {
                            info!("Error converting relation with markdown link: {}", e);
                            // Fall back to regex-based conversion
                            if convert_to_html {
                                let converted_line = MD_LINK_REGEX.replace_all(line, "${1}.html${2}").to_string();
                                info!("Fallback conversion: {} -> {}", line, converted_line);
                                updated_lines.push(converted_line);
                            } else {
                                updated_lines.push(line.to_string());
                            }
                        }
                    }
                } else {
                    // If we couldn't extract relation and target, fall back to regex-based conversion
                    info!("Could not extract relation from markdown link line, using regex");
                    if convert_to_html {
                        let converted_line = MD_LINK_REGEX.replace_all(line, "${1}.html${2}").to_string();
                        
                        if converted_line != line {
                            info!("Converted markdown link: {} -> {}", line, converted_line);
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
                        info!("Trying fallback relation detection for: {}", line);
                        FALLBACK_RELATION_REGEX.captures(line)
                    });
                
                if let Some(captures) = captures {
                    relations_found += 1;
                    let relation_type = captures.get(1).unwrap().as_str().to_string();
                    let target = captures.get(2).unwrap().as_str().trim().to_string();
                    
                    info!("Found relation: {} -> {}", relation_type, target);
                    
                    let relation = Relation::new(relation_type, target);
                    match relation.to_markdown_link(current_file, convert_to_html) {
                        Ok(link) => {
                            info!("Converted to link: {}", link);
                            
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
                            info!("Error converting relation to link: {}", e);
                            updated_lines.push(line.to_string())
                        },
                    }
                } else {
                    // If the line doesn't match the relation pattern, keep it as is
                    info!("Line doesn't match relation pattern, keeping as is");
                    updated_lines.push(line.to_string());
                }
            }
        } else {
            updated_lines.push(line.to_string());
        }
    }
    
    info!("Processed {} relations in document {}", relations_found, current_file.display());
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