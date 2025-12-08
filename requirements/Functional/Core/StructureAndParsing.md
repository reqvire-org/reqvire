# Elements

### Specification File Identification

The system shall only parse markdown files that are identified as specification files. A markdown file is considered a specification file if and only if its first level-1 heading (`#`) is exactly `# Elements`. Files not meeting this criterion shall be ignored during model parsing, even if they have a `.md` extension.

#### Details
- The `# Elements` heading must be the first H1 header in the file
- Leading whitespace, blank lines, or frontmatter before the heading are allowed
- Files without `# Elements` as first H1 are silently skipped (no error)
- This rule applies in addition to `.gitignore` and `.reqvireignore` exclusions
- The page title is not stored or tracked by the system

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Defining Model Structure](../../UserStories.md#defining-model-structure)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Specification File Identification Test](Verifications/ParsingVerifications.md#specification-file-identification-test)
---

### Structure and Addressing in Markdown Documents

The system shall implement semi-structured markdown format specifications that defines the structure, rules, and usage of **Elements**, **Subsections**, **Relations**, and **Identifiers** in Markdown (`.md`) documents following clearly defined specifications.

#### Metadata
  * type: requirement

#### Attachments
  * [MarkdownStructure.md](DesignDocuments/MarkdownStructure.md)

#### Relations
  * derive: [Element Identity Model](#element-identity-model)
  * derive: [Reserved Subsections Support](#reserved-subsections-support)
  * derivedFrom: [Defining Model Structure](../../UserStories.md#defining-model-structure)
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

#### Attachments
  * [ElementIdentity.md](DesignDocuments/ElementIdentity.md)

#### Relations
  * derive: [Identifiers and Relations](#identifiers-and-relations)
  * derive: [Internal Consistency Validator](Validation.md#internal-consistency-validator)
  * derive: [Change Impact Detection](../Processing/ChangeImpact.md#change-impact-detection)
  * derivedFrom: [Structure and Addressing in Markdown Documents](#structure-and-addressing-in-markdown-documents)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * verifiedBy: [Fragment Normalization Test](Verifications/ParsingVerifications.md#fragment-normalization-test)
---

### Identifiers and Relations

The system shall implement  **Identifiers** and **Relations** following clearly defined specifications to ensure consistency, validity, and efficient querying and manipulation of these entities.

#### Metadata
  * type: requirement

#### Attachments
  * [IdentifiersAndRelations.md](DesignDocuments/IdentifiersAndRelations.md)

#### Relations
  * derive: [Relation Types and behaviors](ModelManagement.md#relation-types-and-behaviors)
  * derivedFrom: [Element Identity Model](#element-identity-model)
  * satisfiedBy: [relation.rs](../../../core/src/relation.rs)
---

### Reserved Subsections Support

The system shall support the following reserved subsections with predefined structure and behavior: Relations, Details, Metadata, and Attachments.

#### Details
The system shall support following reserved subsections:
 * **Relations**: Define relationships between elements
 * **Details**: Extend requirement text with additional information
 * **Metadata**: Define element type and classification
 * **Attachments**: Link external documents and Refinement elements
   - Can contain file paths (markdown links where text equals href)
   - Can contain element identifiers (markdown links to Refinement elements ONLY)

Each reserved subsection has specific parsing rules, validation requirements, and behaviors.

#### Metadata
  * type: requirement

#### Attachments
  * [ReservedSubsections.md](DesignDocuments/ReservedSubsections.md)

#### Relations
  * derive: [Verification Type Categories](ModelManagement.md#verification-type-categories)
  * derive: [Attachment Target Validation](Validation.md#attachment-target-validation)
  * derive: [Create Element Operation](../Operations/ElementManipulation.md#create-element-operation)
  * derive: [Attachment Commands](../../Interfaces/CLI/Commands.md#attachment-commands)
  * derivedFrom: [Structure and Addressing in Markdown Documents](#structure-and-addressing-in-markdown-documents)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * satisfiedBy: [Element Type Metadata Specification](Specifications.md#element-type-metadata-specification)
  * verifiedBy: [Attachment Output Rendering Verification](Verifications/AttachmentsVerifications.md#attachment-output-rendering-verification)
  * verifiedBy: [Attachments Subsection Parsing Verification](Verifications/AttachmentsVerifications.md#attachments-subsection-parsing-verification)
  * verifiedBy: [Attachments Validation Verification](Verifications/AttachmentsVerifications.md#attachments-validation-verification)
  * verifiedBy: [Element Subsection Parsing Test](Verifications/ParsingVerifications.md#element-subsection-parsing-test)
  * verifiedBy: [Non-Reserved Subsections Content Test](Verifications/ParsingVerifications.md#non-reserved-subsections-content-test)
---
