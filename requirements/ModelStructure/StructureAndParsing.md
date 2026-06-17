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
  * refinedBy: [Specification File Identification Refinement Specification](Specifications.md#specification-file-identification-refinement-specification)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * specify: [Defining Model Structure](ModelStructureFeature.md#defining-model-structure)
  * verifiedBy: [Specification File Identification Test](../Verifications/ModelStructure/ParsingVerifications.md#specification-file-identification-test)
  * verifiedBy: [Single Element Refinement Validation Test](../Verifications/Operations/Validation/ValidationVerifications.md#single-element-refinement-validation-test)
---

### Structure and Addressing in Markdown Documents

The system shall implement semi-structured markdown format specifications that defines the structure, rules, and usage of **Elements**, **Subsections**, **Relations**, and **Identifiers** in Markdown (`.md`) documents following clearly defined specifications.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Element Identity Model](#element-identity-model)
  * derive: [Reserved Subsections Support](#reserved-subsections-support)
  * refinedBy: [MarkdownStructure](MarkdownStructure.md#markdownstructure)
  * refinedBy: [Structure and Addressing in Markdown Documents Refinement Specification](Specifications.md#structure-and-addressing-in-markdown-documents-refinement-specification)
  * satisfiedBy: [element.rs](../../core/src/element.rs)
  * satisfiedBy: [model.rs](../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * satisfiedBy: [relation.rs](../../core/src/relation.rs)
  * satisfiedBy: [subsection.rs](../../core/src/subsection.rs)
  * specify: [Defining Model Structure](ModelStructureFeature.md#defining-model-structure)
  * verifiedBy: [Format Command Requirements Verification](../Verifications/Operations/Formatting/FormattingVerifications.md#format-command-requirements-verification)
  * verifiedBy: [Invalid Header Structure Test](../Verifications/Operations/Validation/ValidationVerifications.md#invalid-header-structure-test)
---

### Element Identity Model

The system shall distinguish between element identity (ID) and element addressing (identifier) to support stable element tracking independent of file location.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Identifiers and Relations](#identifiers-and-relations)
  * derivedFrom: [Structure and Addressing in Markdown Documents](#structure-and-addressing-in-markdown-documents)
  * refinedBy: [ElementIdentity](ElementIdentity.md#elementidentity)
  * satisfiedBy: [element.rs](../../core/src/element.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * verifiedBy: [Fragment Normalization Test](../Verifications/ModelStructure/ParsingVerifications.md#fragment-normalization-test)
---

### Identifiers and Relations

The system shall implement  **Identifiers** and **Relations** following clearly defined specifications to ensure consistency, validity, and efficient querying and manipulation of these entities.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Relation Types and behaviors](ModelManagement.md#relation-types-and-behaviors)
  * derivedFrom: [Element Identity Model](#element-identity-model)
  * refinedBy: [IdentifiersAndRelations](IdentifiersAndRelations.md#identifiersandrelations)
  * refinedBy: [Identifiers and Relations Refinement Specification](Specifications.md#identifiers-and-relations-refinement-specification)
  * satisfiedBy: [relation.rs](../../core/src/relation.rs)
---

### Reserved Subsections Support

The system shall support reserved subsections with predefined structure and behavior.

#### Details
Reserved subsection vocabulary is defined by the Reqvire core element ontology. Parser-facing support includes:
 * **Relations**: authored relation edges
 * **Details**: narrative element context
 * **Metadata**: element metadata, element type, and governance metadata where valid
 * **Attachments**: explicit attached ontology or requirement-owned contract dependencies
 * **Concept References**: readable bindings to reachable ontology terms
 * **Ontology**: ontology-element Turtle content
 * **Shapes**: semantic-contract SHACL content

Each reserved subsection has specific parsing rules, validation requirements, and behaviors.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Verification Type Categories](ModelManagement.md#verification-type-categories)
  * derivedFrom: [Structure and Addressing in Markdown Documents](#structure-and-addressing-in-markdown-documents)
  * refinedBy: [ReservedSubsections](ReservedSubsections.md#reservedsubsections)
  * refinedBy: [Element Type Metadata Specification](Specifications.md#element-type-metadata-specification)
  * refinedBy: [Requirement Governance Metadata Specification](Specifications.md#requirement-governance-metadata-specification)
  * satisfiedBy: [element.rs](../../core/src/element.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
  * verifiedBy: [Element Subsection Parsing Test](../Verifications/ModelStructure/ParsingVerifications.md#element-subsection-parsing-test)
  * verifiedBy: [Non-Reserved Subsections Content Test](../Verifications/ModelStructure/ParsingVerifications.md#non-reserved-subsections-content-test)
  * verifiedBy: [Attachment Output Rendering Verification](../Verifications/Operations/ModelOperations/AttachmentsVerifications.md#attachment-output-rendering-verification)
  * verifiedBy: [Attachments Subsection Parsing Verification](../Verifications/Operations/ModelOperations/AttachmentsVerifications.md#attachments-subsection-parsing-verification)
  * verifiedBy: [Attachments Validation Verification](../Verifications/Operations/ModelOperations/AttachmentsVerifications.md#attachments-validation-verification)
---
