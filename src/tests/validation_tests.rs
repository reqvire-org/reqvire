use crate::error::ReqFlowError;
use crate::validation;
use crate::element::ElementRegistry;
use crate::element::Element;
use crate::relation::Relation;

#[test]
fn test_validate_markdown_structure_with_unique_elements() {
    let content = r#"# Document Title
    
## Section 1

### Element 1
This is element 1.

#### Relations
* relation1: target1

### Element 2
This is element 2.

#### Relations
* relation2: target2

## Section 2

### Element 3
This is element 3.

#### Relations
* relation3: target3
"#;
    
    let errors = validation::validate_markdown_structure(content).unwrap();
    assert!(errors.is_empty(), "Should not find any errors in valid document");
}

#[test]
fn test_validate_markdown_structure_with_duplicate_elements() {
    let content = r#"# Document Title
    
## Section 1

### Element 1
This is element 1.

### Element 1
This is a duplicate element.

## Section 2

### Element 2
This is element 2.
"#;
    
    let errors = validation::validate_markdown_structure(content).unwrap();
    assert_eq!(errors.len(), 1, "Should find one error for duplicate element");
    match &errors[0] {
        ReqFlowError::DuplicateElement(msg) => {
            assert!(msg.contains("Duplicate element 'Element 1'"), "Error should mention duplicate element: {}", msg);
        },
        _ => panic!("Wrong error type returned"),
    }
}

#[test]
fn test_validate_markdown_structure_with_repeated_subsections_in_different_elements() {
    let content = r#"# Document Title
    
## Section 1

### Element 1
This is element 1.

#### Relations
* relation1: target1

### Element 2
This is element 2.

#### Relations
* relation2: target2
"#;
    
    let errors = validation::validate_markdown_structure(content).unwrap();
    assert!(errors.is_empty(), "Should not find errors when Relations subsection appears in different elements");
}

#[test]
fn test_validate_markdown_structure_with_duplicate_subsections_in_same_element() {
    let content = r#"# Document Title
    
## Section 1

### Element 1
This is element 1.

#### Relations
* relation1: target1

#### Relations
* relation2: target2
"#;
    
    let errors = validation::validate_markdown_structure(content).unwrap();
    assert_eq!(errors.len(), 1, "Should find one error for duplicate subsection");
    match &errors[0] {
        ReqFlowError::DuplicateSubsection(msg) => {
            assert!(msg.contains("Duplicate subsection 'Relations'"), "Error should mention duplicate subsection: {}", msg);
            assert!(msg.contains("within element 'Element 1'"), "Error should mention the containing element: {}", msg);
        },
        _ => panic!("Wrong error type returned: {:?}", errors[0]),
    }
}

#[test]
fn test_validate_unique_relations() {
    // Create a registry and add elements with unique relations
    let mut registry = ElementRegistry::new();
    
    let mut element = Element::new("TestElement".to_string(), "test.md".to_string());
    element.add_relation(Relation::new("dependsOn".to_string(), "target1".to_string()));
    element.add_relation(Relation::new("satisfiedBy".to_string(), "target2".to_string()));
    element.add_relation(Relation::new("verifiedBy".to_string(), "target3".to_string()));
    
    registry.add_element(element).unwrap();
    
    // Validate relations
    let errors = validation::validate_relations(&registry).unwrap();
    assert!(errors.is_empty(), "Should not find any errors with unique relations");
}

#[test]
fn test_validate_duplicate_relations() {
    // Create a registry and add elements with duplicate relations
    let mut registry = ElementRegistry::new();
    
    let mut element = Element::new("TestElement".to_string(), "test.md".to_string());
    element.add_relation(Relation::new("dependsOn".to_string(), "target1".to_string()));
    element.add_relation(Relation::new("satisfiedBy".to_string(), "target2".to_string()));
    element.add_relation(Relation::new("dependsOn".to_string(), "target1".to_string())); // Duplicate
    
    registry.add_element(element).unwrap();
    
    // Validate relations
    let errors = validation::validate_relations(&registry).unwrap();
    assert_eq!(errors.len(), 1, "Should find one error for duplicate relation");
    
    match &errors[0] {
        ReqFlowError::DuplicateRelation(msg) => {
            assert!(msg.contains("Duplicate relation 'dependsOn:target1'"), 
                   "Error should mention duplicate relation type and target: {}", msg);
            assert!(msg.contains("TestElement"), 
                   "Error should mention the element name: {}", msg);
        },
        _ => panic!("Wrong error type returned: {:?}", errors[0]),
    }
}

#[test]
fn test_validate_multiple_duplicate_relations() {
    // Create a registry with multiple elements, some having duplicate relations
    let mut registry = ElementRegistry::new();
    
    // Element 1 with unique relations
    let mut element1 = Element::new("Element1".to_string(), "test1.md".to_string());
    element1.add_relation(Relation::new("dependsOn".to_string(), "target1".to_string()));
    element1.add_relation(Relation::new("satisfiedBy".to_string(), "target2".to_string()));
    registry.add_element(element1).unwrap();
    
    // Element 2 with duplicate relations
    let mut element2 = Element::new("Element2".to_string(), "test2.md".to_string());
    element2.add_relation(Relation::new("verifiedBy".to_string(), "target3".to_string()));
    element2.add_relation(Relation::new("verifiedBy".to_string(), "target3".to_string())); // Duplicate
    element2.add_relation(Relation::new("satisfiedBy".to_string(), "target4".to_string()));
    element2.add_relation(Relation::new("satisfiedBy".to_string(), "target4".to_string())); // Duplicate
    registry.add_element(element2).unwrap();
    
    // Validate relations
    let errors = validation::validate_relations(&registry).unwrap();
    assert_eq!(errors.len(), 2, "Should find two errors for duplicate relations");
    
    // Check that errors are about the right element and relations
    let error_strings: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
    let has_verifiedby_error = error_strings.iter().any(|s| s.contains("verifiedBy:target3") && s.contains("Element2"));
    let has_satisfiedby_error = error_strings.iter().any(|s| s.contains("satisfiedBy:target4") && s.contains("Element2"));
    
    assert!(has_verifiedby_error, "Should have error about duplicate verifiedBy relation");
    assert!(has_satisfiedby_error, "Should have error about duplicate satisfiedBy relation");
}

#[test]
fn test_validate_markdown_with_relations() {
    // Create a simple element with relations directly
    let mut registry = ElementRegistry::new();
    
    // Create element with duplicate relations
    let mut element1 = Element::new("Element With Duplicate Relations".to_string(), "test.md".to_string());
    element1.add_relation(Relation::new("dependsOn".to_string(), "target1".to_string()));
    element1.add_relation(Relation::new("satisfiedBy".to_string(), "target2".to_string()));
    element1.add_relation(Relation::new("dependsOn".to_string(), "target1".to_string())); // Duplicate
    
    // Create another element with unique relations
    let mut element2 = Element::new("Another Element".to_string(), "test.md".to_string());
    element2.add_relation(Relation::new("verifiedBy".to_string(), "target3".to_string()));
    element2.add_relation(Relation::new("tracedFrom".to_string(), "target4".to_string()));
    
    registry.add_element(element1).unwrap();
    registry.add_element(element2).unwrap();
    
    // Validate relations
    let errors = validation::validate_relations(&registry).unwrap();
    println!("Found {} validation errors:", errors.len());
    for error in &errors {
        println!("  - {}", error);
    }
    
    assert_eq!(errors.len(), 1, "Should find one error for duplicate relation");
    
    match &errors[0] {
        ReqFlowError::DuplicateRelation(msg) => {
            assert!(msg.contains("Duplicate relation 'dependsOn:target1'"), 
                   "Error should mention duplicate relation type and target: {}", msg);
            assert!(msg.contains("Element With Duplicate Relations"), 
                   "Error should mention the element name: {}", msg);
        },
        _ => panic!("Wrong error type returned: {:?}", errors[0]),
    }
}