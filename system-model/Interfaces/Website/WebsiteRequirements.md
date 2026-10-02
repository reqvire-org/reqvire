# Elements

### Public Documentation Website

The system SHALL publish a public documentation website that explains Reqvire concepts, workflows, interfaces, ontology authoring, verification, implementation coverage, and AI-assistant integration using terminology aligned with the validated system model.

#### Details
The system SHALL maintain traceability from documented model and interface contracts to the website requirements and their page implementation artifacts.

When a documented model or interface contract changes, the system SHALL identify the affected documentation through change-impact reporting.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Public Documentation Website Source Context Specification](WebsiteSpecifications.md#public-documentation-website-source-context-specification)
  * derive: [Website Assistant Integration Documentation](#website-assistant-integration-documentation)
  * derive: [Website Command and Workflow Documentation](#website-command-and-workflow-documentation)
  * derive: [Website Home Messaging](#website-home-messaging)
  * derive: [Website Implementation Coverage Documentation](#website-implementation-coverage-documentation)
  * derive: [Website Modeling Language Documentation](#website-modeling-language-documentation)
  * derive: [Website Ontology Documentation](#website-ontology-documentation)
  * derive: [Website Requirements and Contracts Documentation](#website-requirements-and-contracts-documentation)
  * derive: [Website Semantic Model Documentation](#website-semantic-model-documentation)
  * derive: [Website Strategic Positioning Documentation](#website-strategic-positioning-documentation)
  * derive: [Website Verification Documentation](#website-verification-documentation)
  * satisfiedBy: [App.tsx](../../../website/src/App.tsx)
  * satisfiedBy: [AppLayout.tsx](../../../website/src/components/AppLayout.tsx)
  * satisfiedBy: [Sidebar.tsx](../../../website/src/components/Sidebar.tsx)
  * specify: [Public Documentation Website Interface](../InterfacesFeature.md#public-documentation-website-interface)
---

### Website Assistant Integration Documentation

The system SHALL document MCP, coding-assistant integrations, prompt workflows, and assistant-scoped Reqvire context using current protocol and model-tool terminology.

#### Details
The system SHALL explain scoped coverage requests and results according to the MCP and coverage contracts referenced by this requirement.

The system SHALL provide usage examples for whole-model and capability-scoped coverage tool requests.

WHEN documenting MCP model search, the system SHALL explain contract-reference filters according to the referenced model-evidence and search-filtering contracts.

WHEN documenting browser MCP clients, the system SHALL explain origin configuration for standalone and embedded endpoints according to the referenced transport contracts.

WHEN documenting MCP deployments, the system SHALL explain listening addresses and accepted endpoint hostnames according to the referenced transport contracts.

#### Metadata
  * type: requirement

#### Contract References
  * [MCP Managed Query Artifacts Specification](../MCP/Specifications.md#mcp-managed-query-artifacts-specification)
  * [MCP Model Evidence Tools Specification](../MCP/Specifications.md#mcp-model-evidence-tools-specification)
  * [MCP Streamable HTTP Transport Safety Specification](../MCP/Specifications.md#mcp-streamable-http-transport-safety-specification)
  * [Serve Command Embedded MCP Endpoint Specification](../MCP/Specifications.md#serve-command-embedded-mcp-endpoint-specification)
  * [SearchFiltering](../../Reports/ModelReports/SearchFiltering.md#searchfiltering)
  * [Workspace Scope Specification](../../ModelStructure/Specifications.md#workspace-scope-specification)
  * [MCP Prompt Guidance Specification](../MCP/Specifications.md#mcp-prompt-guidance-specification)
  * [MCP Protocol Standard Conformance Specification](../MCP/Specifications.md#mcp-protocol-standard-conformance-specification)
  * [MCP Semantic Query Tools Specification](../MCP/Specifications.md#mcp-semantic-query-tools-specification)
  * [MCP Coverage Scope Selection Specification](../MCP/Specifications.md#mcp-coverage-scope-selection-specification)
  * [Coverage Scope Specification](../../Reports/ModelReports/Specifications.md#coverage-scope-specification)

#### Relations
  * definedBy: [Website Assistant Integration Documentation Specification](WebsiteSpecifications.md#website-assistant-integration-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [CodingAssistants.tsx](../../../website/src/pages/CodingAssistants.tsx)
  * satisfiedBy: [Integrations.tsx](../../../website/src/pages/Integrations.tsx)
  * satisfiedBy: [McpServer.tsx](../../../website/src/pages/McpServer.tsx)
---

### Website Command and Workflow Documentation

The system SHALL document everyday CLI workflows, model commands, report commands, mutation commands, change impact, and advanced workflow patterns.

#### Details
The system SHALL provide scoped coverage command examples according to the CLI and coverage contracts referenced by this requirement.

The system SHALL explain coverage scope selection and ranked hierarchical capability presentation according to the Explorer contract referenced by this requirement.

#### Metadata
  * type: requirement

#### Contract References
  * [CLI Managed Query Artifacts Specification](../CLI/Specifications.md#cli-managed-query-artifacts-specification)
  * [Contract Reference Mutation Specification](../../ModelStructure/Specifications.md#contract-reference-mutation-specification)
  * [Workspace Scope Specification](../../ModelStructure/Specifications.md#workspace-scope-specification)
  * [CLI Interface Structure Contract Specification](../CLI/Specifications.md#cli-interface-structure-contract-specification)
  * [Collect Content Specification](../../Reports/ModelReports/Specifications.md#collect-content-specification)
  * [Report Command Catalog Specification](../../Reports/ModelReports/Specifications.md#report-command-catalog-specification)
  * [Serve Command Contract Specification](../WebExplorer/Specifications.md#serve-command-contract-specification)
  * [Explorer Live Store Refresh Input Output](../WebExplorer/Specifications.md#explorer-live-store-refresh-input-output)
  * [Explorer Automatic Store Refresh Specification](../WebExplorer/Specifications.md#explorer-automatic-store-refresh-specification)
  * [CLI Coverage Scope Selection Specification](../CLI/Specifications.md#cli-coverage-scope-selection-specification)
  * [Explorer Coverage Scope and Display Specification](../WebExplorer/Specifications.md#explorer-coverage-scope-and-display-specification)
  * [Coverage Scope Specification](../../Reports/ModelReports/Specifications.md#coverage-scope-specification)

#### Relations
  * definedBy: [Website Command and Workflow Documentation Specification](WebsiteSpecifications.md#website-command-and-workflow-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [Advanced.tsx](../../../website/src/pages/Advanced.tsx)
  * satisfiedBy: [UserGuide.tsx](../../../website/src/pages/UserGuide.tsx)
---

### Website Home Messaging

The system SHALL present Reqvire's public homepage as a concise semantic-engineering overview with links to major conceptual and workflow pages.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Website Home Messaging Specification](WebsiteSpecifications.md#website-home-messaging-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [graph-hierarchy.svg](../../../website/public/images/graph-hierarchy.svg)
  * satisfiedBy: [Home.tsx](../../../website/src/pages/Home.tsx)
---

### Website Implementation Coverage Documentation

The system SHALL document requirement implementation coverage, verification coverage, traceability, and how implementation/evidence artifacts relate to model elements.

#### Details
The system SHALL explain the distinction between scoped report membership and supporting evidence according to the coverage contracts referenced by this requirement.

The system SHALL illustrate coverage supported by contract consumers outside a selected scope.

The system SHALL explain rejection of cyclic fulfillment dependencies according to the implementation coverage contract referenced by this requirement.

The system SHALL explain how contract binding placement identifies the requirement scope responsible for implementation according to the binding constraint referenced by this requirement.

#### Metadata
  * type: requirement

#### Contract References
  * [Contract Reference Semantics Specification](../../ModelStructure/Specifications.md#contract-reference-semantics-specification)
  * [Requirement Implementation Coverage Logic Specification](../../Implementation/Traceability/Specifications.md#requirement-implementation-coverage-logic-specification)
  * [Verification Coverage Specification](../../Reports/ModelReports/Specifications.md#verification-coverage-specification)
  * [Coverage Scope Specification](../../Reports/ModelReports/Specifications.md#coverage-scope-specification)
  * [Contract Bindings Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#contract-bindings-hierarchical-independence-constraint)

#### Relations
  * definedBy: [Website Implementation Coverage Documentation Specification](WebsiteSpecifications.md#website-implementation-coverage-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [ImplementationCoverage.tsx](../../../website/src/pages/ImplementationCoverage.tsx)
---

### Website Modeling Language Documentation

The system SHALL document Reqvire Markdown element syntax, relation syntax, model file structure, and user-facing modeling-language examples.

#### Details
The system SHALL demonstrate canonical Contract Bindings and Contract References subsections with their distinct semantics.

The system SHALL describe requirements as verification targets and capability verification coverage as a roll-up of requirement coverage.

#### Metadata
  * type: requirement

#### Contract References
  * [Contract Reference Semantics Specification](../../ModelStructure/Specifications.md#contract-reference-semantics-specification)
  * [ReservedSubsections](../../ModelStructure/ReservedSubsections.md#reservedsubsections)
  * [Verification Coverage Specification](../../Reports/ModelReports/Specifications.md#verification-coverage-specification)
  * [Specification File Identification Contract Specification](../../ModelStructure/Specifications.md#specification-file-identification-contract-specification)
  * [Structure and Addressing in Markdown Documents Contract Specification](../../ModelStructure/Specifications.md#structure-and-addressing-in-markdown-documents-contract-specification)

#### Relations
  * definedBy: [Website Modeling Language Documentation Specification](WebsiteSpecifications.md#website-modeling-language-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [ModelingLanguage.tsx](../../../website/src/pages/ModelingLanguage.tsx)
---

### Website Ontology Documentation

The system SHALL document ontology authoring, external ontology sources, built-in reserved vocabulary behavior, semantic contracts, managed semantic queries, validation, and ontology export modes.

#### Metadata
  * type: requirement

#### Contract References
  * [Semantic Query Authoring Specification](../../Semantics/SemanticQuerySpecifications.md#semantic-query-authoring-specification)
  * [Semantic Query Context Validation Specification](../../Semantics/SemanticQuerySpecifications.md#semantic-query-context-validation-specification)
  * [Local External Ontology Source Specification](../../Semantics/SemanticModelSpecifications.md#local-external-ontology-source-specification)
  * [Ontology Collection Output Specification](../../Reports/ModelReports/Specifications.md#ontology-collection-output-specification)
  * [Ontology Projection Subgraph Materialization Specification](../../Reports/ModelReports/Specifications.md#ontology-projection-subgraph-materialization-specification)
  * [OWL Reserved Vocabulary Recognition Specification](../../Semantics/SemanticModelSpecifications.md#owl-reserved-vocabulary-recognition-specification)
  * [Semantic Contract Structure Specification](../../ModelStructure/Specifications.md#semantic-contract-structure-specification)

#### Relations
  * definedBy: [Website Ontology Documentation Specification](WebsiteSpecifications.md#website-ontology-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [Ontologies.tsx](../../../website/src/pages/Ontologies.tsx)
---

### Website Requirements and Contracts Documentation

The system SHALL document capability, requirement, contract, semantic-contract, concept-reference, contract-binding, contract-reference, and governance rules using current Reqvire terminology.

#### Details
The system SHALL distinguish implementation obligations declared through Contract Bindings from change-impact dependencies declared through Contract References.

The system SHALL explain that a requirement selects one of these two dependency sections.

The system SHALL explain child and parent contract binding placement, implementation responsibility, and inherited bindings according to the binding constraint referenced by this requirement.

#### Metadata
  * type: requirement

#### Contract References
  * [Contract Reference Semantics Specification](../../ModelStructure/Specifications.md#contract-reference-semantics-specification)
  * [Relation Semantics Specification](../../ModelStructure/Specifications.md#relation-semantics-specification)
  * [Semantic Contract Structure Specification](../../ModelStructure/Specifications.md#semantic-contract-structure-specification)
  * [Requirement Governance Metadata Specification](../../ModelStructure/Specifications.md#requirement-governance-metadata-specification)
  * [Contract Bindings Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#contract-bindings-hierarchical-independence-constraint)

#### Relations
  * definedBy: [Website Requirements and Contracts Documentation Specification](WebsiteSpecifications.md#website-requirements-and-contracts-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [RequirementsCapabilities.tsx](../../../website/src/pages/RequirementsCapabilities.tsx)
---

### Website Semantic Model Documentation

The system SHALL document the Reqvire semantic model, including element types, relation semantics, ownership, contract bindings, concept references, verification links, and implementation evidence.

#### Details
The system SHALL explain how report scope relates to validated ownership and cross-scope evidence according to the contracts referenced by this requirement.

The system SHALL distinguish coverage scope selection from submodel report scope selection.

The system SHALL explain contract binding scope within requirement hierarchies according to the binding constraint referenced by this requirement.

#### Metadata
  * type: requirement

#### Contract References
  * [Contract Reference Semantics Specification](../../ModelStructure/Specifications.md#contract-reference-semantics-specification)
  * [Relation Semantics Specification](../../ModelStructure/Specifications.md#relation-semantics-specification)
  * [Semantic Contract Structure Specification](../../ModelStructure/Specifications.md#semantic-contract-structure-specification)
  * [Requirement Governance Metadata Specification](../../ModelStructure/Specifications.md#requirement-governance-metadata-specification)
  * [Requirement Submodels Report Specification](../../Reports/ModelReports/Specifications.md#requirement-submodels-report-specification)
  * [Coverage Scope Specification](../../Reports/ModelReports/Specifications.md#coverage-scope-specification)
  * [Contract Bindings Hierarchical Independence Constraint](../../ModelStructure/Constraints.md#contract-bindings-hierarchical-independence-constraint)

#### Relations
  * definedBy: [Website Semantic Model Documentation Specification](WebsiteSpecifications.md#website-semantic-model-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [SemanticModel.tsx](../../../website/src/pages/SemanticModel.tsx)
  * satisfiedBy: [Submodels.tsx](../../../website/src/pages/Submodels.tsx)
---

### Website Strategic Positioning Documentation

The system SHALL explain Reqvire as a semantic engineering framework for AI-assisted, traceable, verifiable software engineering.

#### Metadata
  * type: requirement

#### Contract References
  * [Semantic Contract Structure Specification](../../ModelStructure/Specifications.md#semantic-contract-structure-specification)
  * [Relation Semantics Specification](../../ModelStructure/Specifications.md#relation-semantics-specification)

#### Relations
  * definedBy: [Website Strategic Positioning Documentation Specification](WebsiteSpecifications.md#website-strategic-positioning-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [StrategicVision.tsx](../../../website/src/pages/StrategicVision.tsx)
---

### Website Verification Documentation

The system SHALL document verification objectives, concrete verification types, evidence-backed verification behavior, coverage, traces, and verification roll-up semantics.

#### Details
The system SHALL explain verification membership and aggregation for scoped reports according to the coverage contracts referenced by this requirement.

The system SHALL explain the availability of whole-model orphan diagnostics when describing scoped coverage reports.

#### Metadata
  * type: requirement

#### Contract References
  * [Verification Coverage Specification](../../Reports/ModelReports/Specifications.md#verification-coverage-specification)
  * [Verification Roll-up Specification](../../Verification/Traceability/Specifications.md#verification-roll-up-specification)
  * [Verification Trace Tree Construction](../../Verification/Traceability/Specifications.md#verification-trace-tree-construction)
  * [Coverage Scope Specification](../../Reports/ModelReports/Specifications.md#coverage-scope-specification)

#### Relations
  * definedBy: [Website Verification Documentation Specification](WebsiteSpecifications.md#website-verification-documentation-specification)
  * derivedFrom: [Public Documentation Website](#public-documentation-website)
  * satisfiedBy: [Verifications.tsx](../../../website/src/pages/Verifications.tsx)
---
