# User Requirements

## Managing MBSE Models

### Bootstrap model struture

The system shall provide a command to automatically bootstrap a predefined directory and file structure for an MBSE model, ensuring compliance with ReqFlow methodology.

Relations:
 * refine: UserStories.md/Managing MBSE Models
 * verifiedBy:


## Generate Diagrams

### Select Custom Diagram Viewpoints

The system shall allow users to select custom viewpoints for diagrams, tailored to specific stakeholder needs.

Relations:
 * refine: UserStories.md/Generate Diagrams
 * verifiedBy:


### Export Diagrams in Standard Formats

The system shall allow users to export generated diagrams in standard formats (e.g., PNG, SVG, PDF) for easy sharing and presentation.

Relations:
 * refine: UserStories.md/Generate Diagrams
 * verifiedBy:



### Highlight Changes in Diagrams

The system shall provide an option to highlight changes made to the model in the generated diagrams for better traceability.

Relations:
 * refine: UserStories.md/Generate Diagrams
 * verifiedBy:


### Visualize Model Relationships

The system shall provide visual representations of relationships within the MBSE model in the diagrams, enabling users to understand dependencies and their impact.

Relations:
 * refine: UserStories.md/Generate Diagrams
 * verifiedBy:



### Filter Relationships by Type

The system shall allow users to filter relationships in the MBSE model by type, such as dependency, refinement, or verification when generating diagrams.

Relations:
 * refine: UserStories.md/Generate Diagrams
 * verifiedBy:


## Automate Diagram Generation


### Store Automated Diagrams in Designated Locations

The system shall store automatically generated diagrams in pre-configured locations in the model repository.

Relations:
 * refine: UserStories.md/Automate Diagram Generation
 * verifiedBy:



## Aligning Design with Code

### Trace Code to MBSE Model

The system shall enable users to trace implemented code components back to corresponding elements in the MBSE model.

Relations:
 * refine: UserStories.md/Aligning Design with Code
 * verifiedBy:


### Suggest Code Refactoring

The system shall suggest code refactoring opportunities to better align with the structure and relationships in the MBSE model.

Relations:
 * refine: UserStories.md/Aligning Design with Code
 * verifiedBy:


## Validating Structures

### Validate Markdown Structure

The system shall validate the Markdown structure of MBSE documentation to ensure compliance with formatting standards.

Relations:
 * refine: UserStories.md/Validating Structures
 * verifiedBy:

### Validate Filesystem Structure

The system shall validate the organization of files and folders in the repository to ensure consistency with the MBSE methodology.

Relations:
 * refine: UserStories.md/Validating Structures
 * verifiedBy:

### Validate Internal Consistency

The system shall check the internal consistency of the MBSE model, ensuring that relationships and elements align correctly.

Relations:
 * refine: UserStories.md/Validating Structures
 * verifiedBy:


### Validate Cross-Component Dependencies

The system shall validate dependencies across different components of the MBSE model to identify mismatches or gaps.

Relations:
 * refine: UserStories.md/Validating Structures
 * verifiedBy:


## Integrate with GitHub Workflows


### Automate Pull Request Validations

The system shall automate validations of pull requests in the GitHub workflow to ensure model consistency before merging.

Relations:
 * refine: UserStories.md/Integrate with GitHub Workflows
 * verifiedBy:


### Generate Change Logs for Pull Requests

The system shall generate detailed change logs for pull requests, summarizing modifications to the MBSE model and related components.

Relations:
 * refine: UserStories.md/Integrate with GitHub Workflows
 * verifiedBy:


## Grouping Title: AI-Driven Code Suggestions

### Analyze Code for Alignment ---> Needs more work

The system shall allow AI agents to analyze code and identify deviations from the MBSE model.

Relations:
 * refine: UserStories.md/AI-Driven Code Suggestions
 * verifiedBy:



### Suggest Refactoring for MBSE Consistency  ---> Needs more work


The system shall enable AI agents to suggest refactoring opportunities to ensure code consistency with the MBSE model.

Relations:
 * refine: UserStories.md/AI-Driven Code Suggestions
 * verifiedBy:

### Highlight Potential Code-Model Conflicts --> also too advanced for now

The system shall allow AI agents to highlight potential conflicts between code and the MBSE model, providing recommendations for resolution.

Relations:
 * refine: UserStories.md/AI-Driven Code Suggestions
 * verifiedBy:


## AI-Driven Model Suggestions

### Suggest Refinements to Model Relationships

The system shall enable AI agents to suggest refinements to relationships within the MBSE model to improve consistency and traceability.

Relations:
 * refine: UserStories.md/AI-Driven Model Suggestions
 * verifiedBy:


### Recommend Missing Components

The system shall allow AI agents to recommend missing components or elements based on gaps in the MBSE model.

Relations:
 * refine: UserStories.md/AI-Driven Model Suggestions
 * verifiedBy:


### Propose Validation Fixes

The system shall enable AI agents to propose fixes for validation errors in the MBSE model.

Relations:
 * refine: UserStories.md/AI-Driven Model Suggestions
 * verifiedBy:


## Provide Reports


### Generate Relationship Reports

The system shall generate reports summarizing the relationships in the MBSE model, including counts and types of connections.

Relations:
 * refine: UserStories.md/Provide Reports
 * verifiedBy:


### Generate Structural Change Reports

The system shall generate detailed reports summarizing the impact of structural changes, including affected relationships and components.

Relations:
 * refine: UserStories.md/Provide Reports
 * verifiedBy:


### Provide Validation Reports

The system shall generate detailed validation reports, highlighting any inconsistencies or errors in the MBSE model structure.

Relations:
 * refine: UserStories.md/Provide Reports
 * verifiedBy:



### Generate Summary Reports

The system shall allow users to generate summary reports highlighting key metrics and statuses within the MBSE model.

Relations:
 * refine: UserStories.md/Provide Reports
 * verifiedBy:


### Generate Dependency Reports

The system shall generate reports summarizing dependencies between requirements, components, and test cases in the MBSE model.

Relations:
 * refine: UserStories.md/Provide Reports
 * verifiedBy:


### Export Reports to Standard Formats

The system shall allow users to export generated reports in standard formats (e.g., PDF, Excel) for external sharing.

Relations:
 * refine: UserStories.md/Provide Reports
 * verifiedBy:



## Trace Changes in MBSE Model


### Tracing Structural Changes

When tracing structural changes, the system shall analyze the MBSE model and diffs to identify affected components and generate a report of impacted elements and structures, so that the user can review the changes and decide on further actions.

Relations:
 * refine: UserStories.md/Trace Changes in MBSE Model


### Suggest Structural Updates


The system shall suggest updates to maintain structural consistency when changes are introduced to the MBSE model.

Relations:
 * refine: UserStories.md/Trace Changes in MBSE Model



### AI Feedback on Structural Changes

When a report of impacted elements is fed into the AI agents' context, the system shall provide suggestions for propagating changes across the MBSE model and allow the user to approve or reject each suggestion, so that updates can be applied consistently and committed to the model after user validation.

Relations:
 * derivedFrom: Suggest Structural Updates


## Generate Traceability Matrix			

### Create Traceability Matrices

The system shall create a traceability matrices when requested by a user or as part of CI/CD actions.

Relations:
 * refine: UserStories.md/Generate Traceability Matrix
 * verifiedBy:


### Support Relation-Based Views

The system shall generate traceability matrix views based on relations to requirements, such as:

1. **VerifiedBy**: Mapping requirements to their verification methods.
2. **SatisfiedBy**: Mapping system components to the requirements they satisfy.
3. **TracedFrom**: Mapping requirements to their parent or related elements.

Relations:
 * containedBy: Create Traceability Matrices
 * verifiedBy:
 

### Interactive Mermaid Diagrams

The system shall include Mermaid diagrams in the traceability matrix that provide interactive links to related elements in other documents, enabling navigation and exploration of dependencies.

Relations:
 * containedBy: Create Traceability Matrices
 * verifiedBy:

### Markdown-Based Default Format

The system shall generate the traceability matrix in Markdown format by default, adhering to ReqFlow's markdown-first methodology.

Relations:
 * containedBy: Create Traceability Matrices
 * verifiedBy:

### Save matrices to designated files

The system shall save the generated traceability matrices as a Markdown documents with Mermaid diagrams

Relations:
 * containedBy: Create Traceability Matrices
 * verifiedBy:



### Include Verification Checkboxes

The system shall include checkboxes in the traceability matrix for each verification entry, allowing users to manually mark verification as completed and commit the updated status.

Relations:
 * refine: UserStories.md/Generate Traceability Matrix
 * verifiedBy:


### Handle Affected Verifications on Model Changes

The system shall uncheck verification checkboxes in the traceability matrix and save updated if a diff affects the related requirements or components, ensuring re-validation is required.

Relations:
 * refine: UserStories.md/Generate Traceability Matrix
 * verifiedBy:


### Export Traceability Matrix

The system shall provide an option to export the traceability matrix in formats such as Excel or PDF for external sharing and analysis.

Relations:
 * refine: UserStories.md/Generate Traceability Matrix
 * verifiedBy:


### Support CI/CD Integration

The system shall integrate with CI/CD pipelines to generate or update traceability matrices as part of automated workflows.

Relations:
 * refine: UserStories.md/Automate Traceability Matrix
 * verifiedBy:




