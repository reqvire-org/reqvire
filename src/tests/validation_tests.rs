use crate::error::ReqFlowError;
use crate::validation;

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