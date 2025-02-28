#[cfg(test)]
mod tests {
    use std::path::Path;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use predicates::prelude::*;
    
    use crate::config::Config;
    use crate::element::{Element, ElementRegistry};
    use crate::relation::Relation;
    use crate::markdown;
    use crate::model::ModelManager;
    
    #[test]
    fn test_element_creation() {
        let element = Element::new("Test Element".to_string(), "test/path.md".to_string());
        assert_eq!(element.name, "Test Element");
        assert_eq!(element.file_path, "test/path.md");
        assert_eq!(element.content, "");
        assert_eq!(element.relations.len(), 0);
    }
    
    #[test]
    fn test_element_identifier() {
        let element = Element::new("Test Element".to_string(), "test/path.md".to_string());
        assert_eq!(element.identifier(), "test/path.md/Test Element");
    }
    
    #[test]
    fn test_element_anchor_id() {
        let element = Element::new("Test Element".to_string(), "test/path.md".to_string());
        assert_eq!(element.anchor_id(), "test-element");
    }
    
    #[test]
    fn test_element_registry() {
        let mut registry = ElementRegistry::new();
        let element1 = Element::new("Element 1".to_string(), "test/path1.md".to_string());
        let element2 = Element::new("Element 2".to_string(), "test/path2.md".to_string());
        
        registry.add_element(element1).unwrap();
        registry.add_element(element2).unwrap();
        
        assert!(registry.contains_element("test/path1.md/Element 1"));
        assert!(registry.contains_element("test/path2.md/Element 2"));
        assert!(!registry.contains_element("test/path3.md/Element 3"));
    }
    
    #[test]
    fn test_relation_creation() {
        let relation = Relation::new("dependsOn".to_string(), "Target Element".to_string());
        assert_eq!(relation.relation_type, "dependsOn");
        assert_eq!(relation.target, "Target Element");
        assert!(!relation.processed);
    }
    
    #[test]
    fn test_relation_to_markdown_link_same_document() {
        let relation = Relation::new("dependsOn".to_string(), "Target Element".to_string());
        let current_file = Path::new("test/path.md");
        
        let link = relation.to_markdown_link(current_file, false).unwrap();
        assert_eq!(link, "  * dependsOn: [Target Element](#target-element)");
    }
    
    #[test]
    fn test_relation_to_markdown_link_external_document() {
        let relation = Relation::new("dependsOn".to_string(), "path/to/file.md".to_string());
        let current_file = Path::new("test/path.md");
        
        let link = relation.to_markdown_link(current_file, false).unwrap();
        assert_eq!(link, "  * dependsOn: [path/to/file.md](path/to/file.md)");
    }
    
    #[test]
    fn test_relation_to_markdown_link_external_element() {
        let relation = Relation::new("dependsOn".to_string(), "path/to/file.md/Target Element".to_string());
        let current_file = Path::new("test/path.md");
        
        let link = relation.to_markdown_link(current_file, false).unwrap();
        assert_eq!(link, "  * dependsOn: [path/to/file.md/Target Element](path/to/file.md#target-element)");
    }
    
    #[test]
    fn test_parse_elements() {
        // Create a simple test to check that we can parse elements correctly
        let elements = markdown::parse_elements("### Test Element\nContent", "test.md").unwrap();
        assert!(!elements.is_empty());
        assert_eq!(elements[0].name, "Test Element");
    }
    
    #[test]
    fn test_requirements_file_filter() {
        use crate::utils::is_requirements_file;
        
        assert!(is_requirements_file("requirements.md"));
        assert!(is_requirements_file("SystemRequirements.md"));
        assert!(is_requirements_file("UserRequirements.md"));
        assert!(!is_requirements_file("document.md"));
        assert!(!is_requirements_file("specification.md"));
    }
    
    #[test]
    fn test_integration() {
        let temp = TempDir::new().unwrap();
        let input_dir = temp.child("input");
        let output_dir = temp.child("output");
        
        input_dir.create_dir_all().unwrap();
        output_dir.create_dir_all().unwrap();
        
        // Create test requirement file
        input_dir
            .child("requirements.md")
            .write_str(
                r#"# Requirements

## System Requirements

### Requirement 1
This is requirement 1.

#### Relations
  * dependsOn: Requirement 2

### Requirement 2
This is requirement 2.

#### Relations
  * verifiedBy: Test Case 1
"#,
            )
            .unwrap();
        
        // Process the files
        let mut model_manager = ModelManager::new_with_config(Config::default());
        model_manager
            .process_files(
                input_dir.path(),
                output_dir.path(),
                false, // Don't convert to HTML for this test
            )
            .unwrap();
        
        // Check that the output file exists
        output_dir
            .child("requirements.md")
            .assert(predicate::path::exists());
        
        // Check the content of the output file (should have links)
        let output_content = std::fs::read_to_string(output_dir.child("requirements.md").path()).unwrap();
        assert!(output_content.contains("  * dependsOn: [Requirement 2](#requirement-2"));
    }
}