# Usecase diagrams


## ReqFlow usecase


## ReqFlow Tool Use Case Diagram

The use case diagram below highlights the primary interactions between the ReqFlow Tool and its users, including developers, CI/CD systems, and other actors. It captures the high-level functional behaviors that the tool is designed to support, from managing requirements to automating tasks in Git workflows.

```mermaid
flowchart LR
    subgraph "ReqFlow Tool Use Case Diagram"
        generateDiagrams((Generate Diagrams))
        validateStructure((Validate Markdown Structure))
        traceability((Generate Traceability Matrix))
        integrateCICD((Integrate with CI/CD Pipelines))
        automation((Automate Requirement Checks))
        handleDiffs((Trace Changes in Requirements))
    end

    user[Developer👤]
    ciSystem[CI/CD System🤖]

    %% User interactions
    user --> generateDiagrams
    user --> validateStructure
    user --> traceability

    %% CI/CD interactions
    ciSystem --> validateStructure
    ciSystem --> generateDiagrams
    ciSystem --> automation
    ciSystem --> traceability

    %% New relation for tracing changes
    handleDiffs -. include .-> traceability
    handleDiffs -. include .-> generateDiagrams
```

### Explanation of Use Cases


1. Generate Diagrams:
   - Automatically create diagrams, such as use case diagrams or traceability matrices, using Mermaid syntax from structured Markdown files.

2. Validate Markdown Structure:
   - Ensure that the Markdown files adhere to the defined conventions for requirements, relations, and documentation.

3. Generate Traceability Matrix:
   - Produce a matrix linking requirements to related artifacts like test cases, design specifications, and constraints.

4. Integrate with CI/CD Pipelines:
   - Use GitActions or other CI/CD tools to automate tasks such as validation, diagram generation, and traceability matrix updates.

5. Automate Requirement Checks:
   - Automatically verify the consistency of requirements, their relations, and compliance during CI/CD pipeline execution.

6. Trace Changes in Requirements:
   - Changes are traced through Git diffs, leveraging defined relationships to generate summary reports and highlight the impact of changes on downstream artifacts. 

