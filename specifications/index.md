# ReqFlow Specifications Index

This index provides a structured overview of all specification documents.

## Root Documents

- [Usecases.md](Usecases.md) - ReqFlow use case: The use case diagram below highlights the primary interactions between the ReqFlow Tool and its users, including developers, CI/CD systems, and oth...
- [MissionRequirements.md](MissionRequirements.md) - Mission requirements: Mission requirements represent the high-level mission / enterprise  objectives, needs and measures of effectiveness, that a system must fulfill to ...
- [PhysicalArchitecture.md](PhysicalArchitecture.md) - Physical Architecture for ReqFlow: The Physical Architecture represents the concrete systems, services, and components that implement the functionality of ReqFlow. It defines the dep...
- [UserRequirements.md](UserRequirements.md) - User Requirements: The system shall allow structured markdown and unstructured (eg., markdown, PDFs, DOCX, raw text) documents to coexist within the same MBSE model.
- [MOEs.md](MOEs.md) - MOEs: The **MOEs** and **KPPs** for **ReqFlow** are designed to evaluate how effectively the methodology, toolset, and language deliver value to users an...
- [LogicalArchitecture.md](LogicalArchitecture.md) - Logical Architecture for ReqFlow: The Logical Architecture for ReqFlow defines the high-level functional organization of the tool, focusing on the main components that deliver its c...
- [UserStories.md](UserStories.md) - User stories: 1. System Engineer: Focused on managing system models, ensuring alignment with project requirements, and validating structures.  
2. SOI Developer:...

## DesignSpecifications

- [ElementsInDocument.md](DesignSpecifications/ElementsInDocument.md) - Design Specification Document: Structure and Addressing in Markdown Documents: This document defines the structure, rules, and usage of **Elements**, **Subsections** including **Relations**, and **Details**, as well as **Ident...
- [IdentifiersAndRelations.md](DesignSpecifications/IdentifiersAndRelations.md) - Specification Document: Representation of Identifiers and Relations in the System: This document defines how **Identifiers** and **Relations** are to be represented within the system after being parsed from Markdown documents.  
T...
- [RequirementsChangePropagation.md](DesignSpecifications/RequirementsChangePropagation.md) - Design Specification Document: Requirements Change Impact Propagation: This document defines how requirements changes propagate through relationships within a Markdown-based requirements model. It specifies how changes...
- [StructureAndAddressingInDocuments.md](DesignSpecifications/StructureAndAddressingInDocuments.md) - Design Specification Document: Structure and Addressing in Markdown Documents: This document defines the structure, rules, and usage of **Elements**, **Relations**, and **Identifiers** in Markdown (`.md`) documents.

## SystemRequirements

- [Requirements.md](SystemRequirements/Requirements.md) - System Requirements: ```mermaid
graph LR;
  %% Graph styling
  classDef requirement fill:#f9d6d6,stroke:#f55f5f,stroke-width:1px;
  classDef satisfies fill:#fff2cc,stro...

## SystemRequirements/ModelManagement

- [RequirementsDiagrams.md](SystemRequirements/ModelManagement/RequirementsDiagrams.md) - System Requirements

## Verifications

- [InitCommand.md](Verifications/InitCommand.md) - Initialization Command Verifications: This document verifies the requirements for the ReqFlow initialization command.
- [LintingTests.md](Verifications/LintingTests.md) - Linting Tests: This document verifies the requirements for ReqFlow's linting functionality.
- [ValidationTests.md](Verifications/ValidationTests.md) - Validation Tests: This document verifies the requirements for ReqFlow's validation functionality.

