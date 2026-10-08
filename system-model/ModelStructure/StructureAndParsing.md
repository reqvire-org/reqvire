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
  * derive: [Relation Types and behaviors](ModelManagement.md#relation-types-and-behaviors)
  * derivedFrom: [Element Identity Model](#element-identity-model)
  * satisfiedBy: [relation.rs](../../crates/reqvire-core/src/relation.rs)
---

### Reserved Subsections Support

The system SHALL support reserved subsections with predefined structure and behavior.

#### Details
WHEN parsing a reserved subsection defined by the core element ontology, the system SHALL apply the syntax, validation, and serialization rules in the owned subsection contracts.

WHEN a requirement declares Contract Bindings or Contract References, the system SHALL retain their distinct implementation-obligation and content-dependency semantics.

IF a reserved subsection violates its applicable element-type or entry constraints, THEN the system SHALL report the invalid declaration.

#### Metadata
  * type: requirement

#### Contract Bindings
  * [Contract Reference Semantics Specification](Specifications.md#contract-reference-semantics-specification)

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
  * verifiedBy: [Contract Bindings Output Rendering Verification](../Verifications/Operations/ModelOperations/ContractBindingVerifications.md#contract-bindings-output-rendering-verification)
  * verifiedBy: [Contract Bindings Subsection Parsing Verification](../Verifications/Operations/ModelOperations/ContractBindingVerifications.md#contract-bindings-subsection-parsing-verification)
  * verifiedBy: [Contract Bindings Validation Verification](../Verifications/Operations/ModelOperations/ContractBindingVerifications.md#contract-bindings-validation-verification)
---

### Fenced Code Content Preservation

WHEN processing fenced code in model-element content, the system SHALL treat the code as literal content and preserve its boundaries and source text during parsing, model edits and formatting, while continuing normal structural parsing outside the code according to the owned specification.

#### Details
This requirement refines Markdown structure handling for code examples. It prevents literal headings and declarations from changing model structure or hiding later authored elements, and preserves code during serialization.

#### Metadata
  * type: requirement

#### Relations
  * derivedFrom: [Structure and Addressing in Markdown Documents](#structure-and-addressing-in-markdown-documents)
  * satisfiedBy: [crud_ops.rs](../../crates/reqvire-core/src/graph_registry/crud_ops.rs)
  * satisfiedBy: [parser.rs](../../crates/reqvire-core/src/parser.rs)
  * satisfiedBy: [queries.rs](../../crates/reqvire-core/src/semantic_contract/queries.rs)
  * verifiedBy: [Fenced Code Block Parsing Verification](../Verifications/ModelStructure/ParsingVerifications.md#fenced-code-block-parsing-verification)
---
