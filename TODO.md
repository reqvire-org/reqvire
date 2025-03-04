# ReqFlow Implementation TODO List

This document outlines the remaining features and improvements needed for the ReqFlow project based on the defined requirements and design specifications.

## Core Features

### ReqFlow tool settings/config
- [x] Implement YAML-based configuration file support
- [x] Add support for configurable SystemRequirements folder name
- [x] Add support for configurable DesignSpecifications folder name 
- [x] Support for referencing requirements across distributed repositories
- [x] Add validation for configuration file format


### Requirements Change Propagation
- [ ] Implement the Requirements Change Impact Propagation system as defined in `DSD_RequirementsChangePropagation.md`
- [ ] Add support for detecting requirement changes between commits
- [ ] Build mechanism to trace impact of changes through requirement relations
- [ ] Implement validation for affected verification artifacts
- [ ] Create update suggestions for affected elements

### Advanced Visualization
- [ ] Improve traceability matrix with more detailed relationship views
- [ ] Implement interactive Mermaid diagrams with clickable elements
- [ ] Add support for custom diagram viewpoints tailored to specific stakeholder needs
- [ ] Add ability to highlight changes in diagrams for better traceability
- [ ] Implement filters for relationships by type in diagram generation

### Reporting
- [ ] Implement comprehensive relationship reports
- [ ] Add structural change reports
- [ ] Create verification status reports
- [ ] Implement dependency reports
- [ ] Add support for exporting reports to standard formats (PDF, Excel)

### Verification Features
- [ ] Add verification checkboxes in traceability matrix
- [ ] Implement automatic unmarking of verification status on requirement changes
- [ ] Create validation for verification coverage

## Integration Features

### Git and GitHub Integration
- [ ] Implement automated pull request validations
- [ ] Add detailed change logs for pull requests
- [ ] Build support for CI/CD traceability matrix generation
- [ ] Implement automated diagram generation in CI/CD

### Code Alignment
- [ ] Add support for linking code with MBSE elements
- [ ] Implement code-to-model alignment validation
- [ ] Create code refactoring suggestions based on model changes

### Bootstrapping and Templates
- [ ] Create command to bootstrap a predefined directory and file structure
- [ ] Implement templates for different types of requirements
- [ ] Add validation for template compliance


## AI-Assistant Features

### AI Model Suggestions
- [ ] Design AI integration for suggesting model refinements
- [ ] Implement missing component recommendations
- [ ] Add validation fix suggestions
- [ ] Create structural update suggestions for model consistency

### AI Code Suggestions
- [ ] Implement code analysis for model alignment
- [ ] Design suggestion system for code refactoring
- [ ] Add conflict detection between code and model

## UI and Export Features

### Export Capabilities
- [ ] Add support for exporting diagrams in standard formats (PNG, SVG, PDF)
- [ ] Implement export options for traceability matrices
- [ ] Create HTML report templates with improved styling

## Infrastructure and Quality

### Performance Improvements
- [ ] Optimize model loading for large repositories
- [ ] Improve parallel processing of files
- [ ] Add incremental update support to avoid full reprocessing

### Documentation and Examples
- [ ] Complete installation guide
- [ ] Add detailed usage examples
- [ ] Create comprehensive API documentation
- [ ] Develop tutorial material for new users

### Testing
- [ ] Expand unit test coverage
- [ ] Add integration tests for end-to-end workflows
- [ ] Implement performance benchmarks

## Prioritization

### High Priority
0. ReqFlow tool settings/config - critical for further use of tool
1. Requirements Change Propagation - critical for traceability
2. Advanced Visualization Improvements - needed for better stakeholder communication
3. Reporting Features - essential for tracking status and coverage
4. Git/GitHub Integration - needed for workflow integration
5. Verification Features - supports validation workflow

### Medium Priority
1. Bootstrapping and Templates - helps with adoption
2. Performance Improvements - necessary for larger projects
3. Documentation and Examples - aids in user adoption

### Lower Priority
1. AI-Assistant Features - nice to have but complex
2. Code Alignment Features - useful but requires significant development
3. Advanced Export Capabilities - can be added incrementally
