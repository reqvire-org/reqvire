# ReqFlow Implementation TODO List

This document outlines the remaining features and improvements needed for the ReqFlow project based on the defined requirements and design specifications.

## Core Features

## Integration Features

### Git and GitHub Integration
- [ ] Implement automated pull request validations
- [ ] Add detailed change logs for pull requests
- [ ] Build support for CI/CD traceability matrix generation
- [ ] Implement automated diagram generation in CI/CD


## MBSE Model Inconsistencies and Improvements

### 1. Complete Relation Types Implementation

**Remaining Task**:
- [ ] Update relation.rs to match the new RelationTypesRegistry.md document

### 2. Conflicting Path Resolution Rules

**Issue**: `DesignSpecifications/IdentifiersAndRelations.md` and `DesignSpecifications/StructureAndAddressingInDocuments.md` have different rules for path resolution:
- One states fragments must appear in original form while the other describes specific normalization rules
- One mentions that identifiers must not use absolute paths, while another explicitly describes handling absolute paths

**Detailed Fix Suggestions**:
- [ ] Consolidate path resolution rules into a single authoritative "IdentifierNormalization.md" document
- [ ] Clearly define the normalization process with explicit examples:
  - [ ] When to keep fragments in original form vs. when to normalize
  - [ ] Exact algorithm for normalizing paths (with pseudocode)
  - [ ] Complete set of examples covering all edge cases
- [ ] Add tests that verify the identifier normalization works as specified
- [ ] Implement a dedicated IdentifierNormalizer component in the code
- [ ] Add validation that verifies all identifiers in the model conform to the rules

### 3. Circular Dependencies in Element Definitions

**Issue**: There's some circular referencing between `ElementsInDocument.md` and `IdentifiersAndRelations.md`:
- Each document refers to the other for "more details"
- The documents have some overlapping content with minor differences

**Detailed Fix Suggestions**:
- [ ] Create a clear hierarchy of specification documents:
  - [ ] "ElementDefinition.md" - defines what an element is and its structure
  - [ ] "IdentifierDefinition.md" - defines identifiers building on element concepts
  - [ ] "RelationDefinition.md" - defines relations building on both elements and identifiers
- [ ] Remove duplicate content across documents
- [ ] Create a visual diagram showing the relationships between these concepts
- [ ] Add cross-references in each document to the others, without circular dependencies
- [ ] Use a consistent terminology across all documents


### 4. Inconsistent Metadata Structure Definitions

**Issue**: The metadata structure is defined differently across documents and lacks validation enforcement:
- Some examples show metadata with different indentation patterns
- The required properties in metadata sections are not clearly defined

**Detailed Fix Suggestions**:
- [ ] Create a specific document "MetadataStructure.md" that defines:
  - [ ] Exact formatting requirements (indentation, bullet types)
  - [ ] Required vs. optional metadata properties
  - [ ] Valid values and types for each property
  - [ ] Schema validation rules for metadata
- [ ] Add a linting component specifically for metadata validation
- [ ] Implement automatic correction of common metadata formatting issues
- [ ] Add tests for various metadata edge cases and malformed input
- [ ] Document the inheritance/override rules for metadata across related elements

### 5. Incomplete Requirements for AI Integration

**Issue**: While there are several requirements related to AI capabilities (AI-driven code suggestions, AI-driven model suggestions), many are marked as "needs more work" and lack concrete implementation:
- Requirements like "Analyze Code for Alignment" are marked as incomplete
- The LLM Context Command has minimal implementation details

**Detailed Fix Suggestions**:
- [ ] Complete the definitions of AI-related requirements:
  - [ ] "Analyze Code for Alignment" - Define specific code patterns to search for
  - [ ] "Highlight Potential Code-Model Conflicts" - Define conflict detection rules
  - [ ] "Suggest Refactoring for MBSE Consistency" - Define refactoring patterns
- [ ] Enhance the LLM Context Command with:
  - [ ] Specific LLM prompt templates for different tasks
  - [ ] Context windowing for large models
  - [ ] Input/output validation for LLM interactions
- [ ] Add new requirements for:
  - [ ] "AI-Assisted Relation Discovery" - Finding missing relationships
  - [ ] "Requirements Quality Improvement" - Suggestions for better clarity
  - [ ] "Automated Test Generation" - Creating verification tests from requirements
- [ ] Create proper implementations for each AI feature
- [ ] Add tests for AI integration points

### 6. Validation and Documentation Gaps

**Issue**: Several verification requirements lack corresponding test implementations:
- Many system requirements are not traced to specific verification test cases
- Validation tests for relation types are defined but missing complete implementations

**Detailed Fix Suggestions**:
- [ ] Conduct a full traceability analysis:
  - [ ] Identify all requirements without verification relationships
  - [ ] Create verification test cases for each uncovered requirement
- [ ] Complete the validation test implementations:
  - [ ] Add tests for all relation type validations
  - [ ] Add tests for identifier/path normalization
  - [ ] Add tests for change impact propagation
- [ ] Create a comprehensive test plan document that maps:
  - [ ] Each requirement to its verification tests
  - [ ] Each feature to its test coverage
  - [ ] Each module to its unit tests
- [ ] Implement automated verification gap analysis in CI/CD
- [ ] Add quality metrics for test coverage in reports

### 7. Inconsistency in Path Handling

**Issue**: Path handling between external folders and specification folders is inconsistently defined:
- The logic for path normalization differs between different documents
- The handling of excluded patterns isn't fully specified

**Detailed Fix Suggestions**:
- [ ] Create a dedicated "PathHandling.md" specification that covers:
  - [ ] Exact rules for path normalization in different contexts
  - [ ] Complete algorithm for path resolution across folders
  - [ ] Rules for excluded patterns and their scope
  - [ ] Examples covering all edge cases
- [ ] Implement a dedicated PathResolver component that:
  - [ ] Handles all path normalization consistently
  - [ ] Applies pattern exclusions consistently
  - [ ] Provides clear error messages for invalid paths
- [ ] Add tests that verify:
  - [ ] Paths are normalized consistently across contexts
  - [ ] External folders are handled correctly
  - [ ] Exclusion patterns work as expected
- [ ] Update all code using paths to use this component

### 8. Relation Directionality Implementation

**Remaining Tasks**:
- [ ] Update the relation model in the code to explicitly model:
  - [ ] Forward relations (source → target)
  - [ ] Backward relations (target → source)
- [ ] Add validation rules that ensure relations are properly used
- [ ] Implement consistent change impact handling for all relation types
- [ ] Add documentation comments in the code explaining directionality handling

### 9. Mismatch Between Design and Implementation

**Issue**: The system design in `DesignSpecifications/*.md` doesn't always match the actual implementation in the code:
- Some ADT definitions in the docs don't match what's in the source code
- Implementation details are sometimes spread across multiple files

**Detailed Fix Suggestions**:
- [ ] Conduct a full audit of design documentation vs. implementation:
  - [ ] Compare ADT definitions in docs to actual code
  - [ ] Document discrepancies in a gap analysis
- [ ] Choose a strategy for each discrepancy:
  - [ ] Update the documentation to match the code
  - [ ] Refactor the code to match the documentation
- [ ] Add a validation step in CI/CD that:
  - [ ] Extracts ADT definitions from the code
  - [ ] Compares them to the documentation
  - [ ] Flags discrepancies for review
- [ ] Consolidate implementation details:
  - [ ] Group related functionality in logical modules
  - [ ] Document cross-file dependencies
  - [ ] Add implementation overview diagrams
- [ ] Create architectural decision records (ADRs) for any major deviations
