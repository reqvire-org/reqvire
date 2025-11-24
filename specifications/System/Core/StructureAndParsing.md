# Requirements

### Specification File Identification

The system shall only parse markdown files that are identified as specification files. A markdown file is considered a specification file if and only if its first level-1 heading (`#`) is exactly `# Requirements`. Files not meeting this criterion shall be ignored during model parsing, even if they have a `.md` extension.

#### Details
- The `# Requirements` heading must be the first H1 header in the file
- Leading whitespace, blank lines, or frontmatter before the heading are allowed
- Files without `# Requirements` as first H1 are silently skipped (no error)
- This rule applies in addition to `.gitignore` and `.reqvireignore` exclusions
- The page title is not stored or tracked by the system

#### Relations
  * derivedFrom: [Managing System Models](../../UserStories.md#managing-system-models)
  * verifiedBy: [Specification File Identification Test](Verifications/ParsingVerifications.md#specification-file-identification-test)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
---

### Reserved Subsections Support

The system shall support the following reserved subsections with predefined structure and behavior: Relations, Details, Metadata, and Attachments.

#### Details
The system shall support following reserved subsections:
 * **Relations**: Define relationships between elements
 * **Details**: Extend requirement text with additional information
 * **Metadata**: Define element type and classification
 * **Attachments**: Link external documents

Each reserved subsection has specific parsing rules, validation requirements, and behaviors.

#### Attachments
  * [ReservedSubsections.md](DesignDocuments/ReservedSubsections.md)

#### Relations
  * derivedFrom: [Structure and Addressing in Markdown Documents](#structure-and-addressing-in-markdown-documents)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
---

### Structure and Addressing in Markdown Documents

The system shall implement semi-structured markdown format specifications that defines the structure, rules, and usage of **Elements**, **Subsections**, **Relations**, and **Identifiers** in Markdown (`.md`) documents following clearly defined specifications.

#### Attachments
  * [MarkdownStructure.md](DesignDocuments/MarkdownStructure.md)

#### Relations
  * derivedFrom: [Managing System Models](../../UserStories.md#managing-system-models)
  * satisfiedBy: [relation.rs](../../../core/src/relation.rs)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [subsection.rs](../../../core/src/subsection.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
  * satisfiedBy: [model.rs](../../../core/src/model.rs)
---

### Identifiers and Relations

The system shall implement  **Identifiers** and **Relations** following clearly defined specifications to ensure consistency, validity, and efficient querying and manipulation of these entities.

#### Attachments
  * [IdentifiersAndRelations.md](DesignDocuments/IdentifiersAndRelations.md)

#### Relations
  * derivedFrom: [Element Identity Model](#element-identity-model)
  * derivedFrom: [AI-Assisted System Model Management](../../UserStories.md#ai-assisted-system-model-management)
  * satisfiedBy: [relation.rs](../../../core/src/relation.rs)
---

### Element Identity Model

The system shall distinguish between element identity (ID) and element addressing (identifier) to support stable element tracking independent of file location.

#### Attachments
  * [ElementIdentity.md](DesignDocuments/ElementIdentity.md)

#### Relations
  * derivedFrom: [Structure and Addressing in Markdown Documents](#structure-and-addressing-in-markdown-documents)
  * satisfiedBy: [element.rs](../../../core/src/element.rs)
  * satisfiedBy: [parser.rs](../../../core/src/parser.rs)
---
