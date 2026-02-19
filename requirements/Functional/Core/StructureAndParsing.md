# Elements

### Specification File Identification

The system shall identify supported markdown model document types by the first level-1 heading (`#`) and apply type-specific processing rules.

#### Details
Supported model document types:
- `# Elements`: parsed as element collections.
- `# Documents`: parsed as a single-element document format with `## Metadata`, optional `## Relations`, and a dynamic `## <Actual Element Name>` section (the section heading itself is the element name) whose body may contain any markdown headers.

Unsupported first H1 headings shall be ignored by element parsing.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Defining Model Structure](../../UserStories.md#defining-model-structure)
  * refinedBy: [Specification File Identification Refinement Specification](Specifications.md#specification-file-identification-refinement-specification)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Specification File Identification Test](Verifications/ParsingVerifications.md#specification-file-identification-test)
  * verifiedBy: [Document Refinement Validation Test](Verifications/ValidationVerifications.md#document-refinement-validation-test)
---

### Structure and Addressing in Markdown Documents

The system shall implement semi-structured markdown format specifications that defines the structure, rules, and usage of **Elements**, **Subsections**, **Relations**, and **Identifiers** in Markdown (`.md`) documents following clearly defined specifications.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Element Identity Model](#element-identity-model)
  * derive: [Reserved Subsections Support](#reserved-subsections-support)
  * derivedFrom: [Defining Model Structure](../../UserStories.md#defining-model-structure)
  * refinedBy: [MarkdownStructure](DesignDocuments/MarkdownStructure.md#markdownstructure)
  * refinedBy: [Structure and Addressing in Markdown Documents Refinement Specification](Specifications.md#structure-and-addressing-in-markdown-documents-refinement-specification)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * satisfiedBy: [relation.rs](../../../core/src/relation.rs)
  * satisfiedBy: [subsection.rs](../../../core/src/subsection.rs)
  * verifiedBy: [Invalid Header Structure Test](Verifications/ValidationVerifications.md#invalid-header-structure-test)
  * verifiedBy: [Format Command Requirements Verification](../Operations/Verifications/FormattingVerifications.md#format-command-requirements-verification)
---

### Element Identity Model

The system shall distinguish between element identity (ID) and element addressing (identifier) to support stable element tracking independent of file location.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Identifiers and Relations](#identifiers-and-relations)
  * derive: [Internal Consistency Validator](Validation.md#internal-consistency-validator)
  * derive: [Change Impact Detection](../Processing/ChangeImpact.md#change-impact-detection)
  * derivedFrom: [Structure and Addressing in Markdown Documents](#structure-and-addressing-in-markdown-documents)
  * refinedBy: [ElementIdentity](DesignDocuments/ElementIdentity.md#elementidentity)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Fragment Normalization Test](Verifications/ParsingVerifications.md#fragment-normalization-test)
---

### Identifiers and Relations

The system shall implement  **Identifiers** and **Relations** following clearly defined specifications to ensure consistency, validity, and efficient querying and manipulation of these entities.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Relation Types and behaviors](ModelManagement.md#relation-types-and-behaviors)
  * derivedFrom: [Element Identity Model](#element-identity-model)
  * refinedBy: [IdentifiersAndRelations](DesignDocuments/IdentifiersAndRelations.md#identifiersandrelations)
  * refinedBy: [Identifiers and Relations Refinement Specification](Specifications.md#identifiers-and-relations-refinement-specification)
  * satisfiedBy: [relation.rs](../../../core/src/relation.rs)
---

### Reserved Subsections Support

The system shall support the following reserved subsections with predefined structure and behavior: Relations, Details, Metadata, and Attachments.

#### Details
The system shall support following reserved subsections:
 * **Relations**: Define relationships between elements
 * **Details**: Extend requirement text with additional information
 * **Metadata**: Define element type and classification
 * **Attachments**: Link Refinement elements
   - Can contain element identifiers only (markdown links to Refinement elements ONLY)

Each reserved subsection has specific parsing rules, validation requirements, and behaviors.

#### Metadata
  * type: requirement

#### Relations
  * derive: [Verification Type Categories](ModelManagement.md#verification-type-categories)
  * derive: [Attachment Target Validation](Validation.md#attachment-target-validation)
  * derive: [Attachment Commands](../../Interfaces/CLI/Commands.md#attachment-commands)
  * derivedFrom: [Structure and Addressing in Markdown Documents](#structure-and-addressing-in-markdown-documents)
  * refinedBy: [ReservedSubsections](DesignDocuments/ReservedSubsections.md#reservedsubsections)
  * refinedBy: [Element Type Metadata Specification](Specifications.md#element-type-metadata-specification)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Attachment Output Rendering Verification](Verifications/AttachmentsVerifications.md#attachment-output-rendering-verification)
  * verifiedBy: [Attachments Subsection Parsing Verification](Verifications/AttachmentsVerifications.md#attachments-subsection-parsing-verification)
  * verifiedBy: [Attachments Validation Verification](Verifications/AttachmentsVerifications.md#attachments-validation-verification)
  * verifiedBy: [Element Subsection Parsing Test](Verifications/ParsingVerifications.md#element-subsection-parsing-test)
  * verifiedBy: [Non-Reserved Subsections Content Test](Verifications/ParsingVerifications.md#non-reserved-subsections-content-test)
---
