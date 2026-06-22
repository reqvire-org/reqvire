# Elements

### Specification File Identification

The system shall identify supported markdown model document types by the first level-1 heading (`#`) and apply type-specific processing rules.

#### Details
Supported model document types:
- `# Elements`: parsed as element collections.
- `# Element`: parsed as a single-element file with `## Metadata`, optional `## Relations`, and a dynamic `## <Actual Element Name>` section (the section heading itself is the element name) whose body may contain any markdown headers.

Unsupported first H1 headings shall be ignored by element parsing.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [Specification File Identification Contract Specification](Specifications.md#specification-file-identification-contract-specification)
  * satisfiedBy: [model.rs](../../crates/reqvire-core/src/model.rs)
  * satisfiedBy: [parser.rs](../../crates/reqvire-core/src/parser.rs)
  * specify: [Defining Model Structure](ModelStructureFeature.md#defining-model-structure)
  * verifiedBy: [Specification File Identification Test](../Verifications/ModelStructure/ParsingVerifications.md#specification-file-identification-test)
  * verifiedBy: [Single Element Contract Validation Test](../Verifications/Operations/Validation/ValidationVerifications.md#single-element-contract-validation-test)
---

### Structure and Addressing in Markdown Documents

The system shall implement semi-structured markdown format specifications that defines the structure, rules, and usage of **Elements**, **Subsections**, **Relations**, and **Identifiers** in Markdown (`.md`) documents following clearly defined specifications.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [MarkdownStructure](MarkdownStructure.md#markdownstructure)
  * definedBy: [Structure and Addressing in Markdown Documents Contract Specification](Specifications.md#structure-and-addressing-in-markdown-documents-contract-specification)
  * derive: [Element Identity Model](#element-identity-model)
  * derive: [Reserved Subsections Support](#reserved-subsections-support)
  * satisfiedBy: [element.rs](../../crates/reqvire-core/src/element.rs)
  * satisfiedBy: [model.rs](../../crates/reqvire-core/src/model.rs)
  * satisfiedBy: [parser.rs](../../crates/reqvire-core/src/parser.rs)
  * satisfiedBy: [relation.rs](../../crates/reqvire-core/src/relation.rs)
  * satisfiedBy: [subsection.rs](../../crates/reqvire-core/src/subsection.rs)
  * specify: [Defining Model Structure](ModelStructureFeature.md#defining-model-structure)
  * verifiedBy: [Format Command Requirements Verification](../Verifications/Operations/Formatting/FormattingVerifications.md#format-command-requirements-verification)
  * verifiedBy: [Invalid Header Structure Test](../Verifications/Operations/Validation/ValidationVerifications.md#invalid-header-structure-test)
---

### Element Identity Model

The system shall distinguish between element identity (ID) and element addressing (identifier) to support stable element tracking independent of file location.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [ElementIdentity](ElementIdentity.md#elementidentity)
  * derive: [Identifiers and Relations](#identifiers-and-relations)
  * derivedFrom: [Structure and Addressing in Markdown Documents](#structure-and-addressing-in-markdown-documents)
  * satisfiedBy: [element.rs](../../crates/reqvire-core/src/element.rs)
  * satisfiedBy: [parser.rs](../../crates/reqvire-core/src/parser.rs)
  * verifiedBy: [Fragment Normalization Test](../Verifications/ModelStructure/ParsingVerifications.md#fragment-normalization-test)
---

### Identifiers and Relations

The system shall implement  **Identifiers** and **Relations** following clearly defined specifications to ensure consistency, validity, and efficient querying and manipulation of these entities.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [IdentifiersAndRelations](IdentifiersAndRelations.md#identifiersandrelations)
  * definedBy: [Identifiers and Relations Contract Specification](Specifications.md#identifiers-and-relations-contract-specification)
  * derive: [Relation Types and behaviors](ModelManagement.md#relation-types-and-behaviors)
  * derivedFrom: [Element Identity Model](#element-identity-model)
  * satisfiedBy: [relation.rs](../../crates/reqvire-core/src/relation.rs)
---

### Reserved Subsections Support

The system shall support reserved subsections with predefined structure and behavior.

#### Details
Reserved subsection vocabulary is defined by the Reqvire core element ontology. Parser-facing support includes:
 * **Relations**: authored relation edges
 * **Details**: narrative element context
 * **Metadata**: element metadata, element type, and governance metadata where valid
 * **Reused Contract Context**: explicit reusable requirement-owned contract dependencies
 * **Concept References**: readable bindings from non-ontology, non-semantic-contract elements to generated native SKOS concepts
 * **Ontology**: ontology-element Turtle content
 * **External Ontology**: ontology-element local external vocabulary source declarations
 * **Shapes**: semantic-contract SHACL content
 * **Query**: semantic-contract SPARQL content when a contract owns query text

Each reserved subsection has specific parsing rules, validation requirements, and behaviors.

#### Metadata
  * type: requirement

#### Relations
  * definedBy: [ReservedSubsections](ReservedSubsections.md#reservedsubsections)
  * definedBy: [Element Type Metadata Specification](Specifications.md#element-type-metadata-specification)
  * definedBy: [Requirement Governance Metadata Specification](Specifications.md#requirement-governance-metadata-specification)
  * derive: [Verification Type Categories](ModelManagement.md#verification-type-categories)
  * derivedFrom: [Structure and Addressing in Markdown Documents](#structure-and-addressing-in-markdown-documents)
  * satisfiedBy: [element.rs](../../crates/reqvire-core/src/element.rs)
  * satisfiedBy: [parser.rs](../../crates/reqvire-core/src/parser.rs)
  * verifiedBy: [Element Subsection Parsing Test](../Verifications/ModelStructure/ParsingVerifications.md#element-subsection-parsing-test)
  * verifiedBy: [Non-Reserved Subsections Content Test](../Verifications/ModelStructure/ParsingVerifications.md#non-reserved-subsections-content-test)
  * verifiedBy: [Reused Contract Context Output Rendering Verification](../Verifications/Operations/ModelOperations/ReusedContractContextVerifications.md#reused-contract-context-output-rendering-verification)
  * verifiedBy: [Reused Contract Context Subsection Parsing Verification](../Verifications/Operations/ModelOperations/ReusedContractContextVerifications.md#reused-contract-context-subsection-parsing-verification)
  * verifiedBy: [Reused Contract Context Validation Verification](../Verifications/Operations/ModelOperations/ReusedContractContextVerifications.md#reused-contract-context-validation-verification)
---
