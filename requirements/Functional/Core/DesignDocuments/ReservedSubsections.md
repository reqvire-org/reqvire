# Documents

## Metadata
  * type: specification

## Relations
  * refine: [Reserved Subsections Support](../StructureAndParsing.md#reserved-subsections-support)

## ReservedSubsections

# Reserved Subsections Specification

Reserved subsection vocabulary is defined by the Reqvire core element ontology. This specification defines parser-facing syntax, validation, and serialization behavior for those ontology-defined subsection concepts.

## Relations Subsection

Must be defined with a level 4 header: `#### Relations`.

The Relations subsection defines relationships between elements using markdown bullet list syntax.

**Parsing Rules:**
- Relation entries are bullet points starting with `*`
- Format: `* relationType: [Element Name](path#element-id)`
- Duplicate relation entries within the same `#### Relations` subsection are not allowed

**Examples:**
```markdown
#### Relations
* derivedFrom: [Parent Requirement](../path/file.md#parent)
* satisfiedBy: [Implementation](../../src/module.rs)
* verify: [Target Requirement](file.md#target)
```

## Details Subsection

Must be defined with a level 4 header: `#### Details`.

The Details subsection provides supporting requirement context and intent-level clarification.

**Parsing Rules:**
- When parsing `#### Details` subsections, any markdown headers or elements within `<details>...</details>` tags are skipped
- Content within the Details subsection is part of the requirement narrative context
- Details subsection does **not** replace refinement relations
- Technical specifications, constraints, and behaviors that define implementation contracts should be captured in explicit refinement elements linked via `refinedBy`
- Any normative statements in the Details subsection are interpreted as requirement-level statements unless moved to dedicated refinement elements

**Examples:**
```markdown
### My Requirement

The system shall perform action X.

#### Details
Implementation details shall follow associated refinement specifications.
Additional context about action X:
- Operational assumption 1
- Clarification 2
```

## Metadata Subsection

Must be defined with a level 4 header: `#### Metadata`.

The Metadata subsection stores element properties including type, requirement governance metadata where valid, and custom attributes.

**Parsing Rules:**
- Contains properties in list format: `* property_name: property_value`
- Property entries are listed as bullet points (`*`), with **two spaces** (`  *`) of indentation followed by property_name + ': ' + property_value
- May include custom properties, but reserved property names have the constraints defined by their owning specifications

**Reserved Properties:**

The following properties have special meaning:

- `type`: Defines the element type (supported types are defined in [Supported Element Types Specification](../Specifications.md#supported-element-types-specification))
- `status`, `priority`, `risk`, `owner`: Define governance metadata for capability and requirement elements only (defined in [Requirement Governance Metadata Specification](../Specifications.md#requirement-governance-metadata-specification))
- Additional reserved properties may be defined in future releases

**Examples:**
```markdown
### My Element

This is a verification element.

#### Metadata
  * type: verification
  * domain: safety
  * review_method: inspection

#### Relations
* verify: [Some Requirement](#some-requirement)
```

```markdown
### My Element

This is a verification element.

#### Details
Some details.

#### Metadata
  * type: verification
  * domain: safety
  * review_method: inspection

#### Relations
  * verify: [Some Requirement](#some-requirement)
```

## Attachments Subsection

Must be defined with a level 4 header: `#### Attachments`.

The Attachments subsection links an element to explicit dependency elements. Capability attachments consume ontology context. Requirement attachments consume reusable requirement-owned contracts.

### Element Identifier Attachments

Element identifier attachments link to model elements that are attachable under the Reqvire relation and attachment compatibility model.

**Parsing Rules:**
- Support markdown link syntax with fragment identifiers: `* [Element Name](path#element-id)`
- Same-file references: `* [Element Name](#element-id)`
- Cross-file references: `* [Element Name](relative/path/file.md#element-id)`
- Identifiers are normalized using the same rules as relation targets
- Link text contains the element name

**Validation Rules:**
- Target element must exist in the model
- Capability attachment targets must be ontology elements.
- Requirement attachment targets must be requirement-owned `semantic-contract`, `semantic-query-contract`, `constraint`, `behavior`, `specification`, `state`, or `input-output` elements.
- Requirement-owned refinement targets must have exactly one compatible `refine` relation before they are attachable.
- Non-attachable element identifiers are rejected with a validation error.
- Redundant same-hierarchy attachments and invalid cross-subgraph attachment flow are rejected by attachment validation.

**Examples:**
```markdown
### API Capability

Capability context.

#### Attachments
* [API Ontology](../Ontologies/Interfaces.md#api-ontology)
```

```markdown
### API Requirement

The system shall expose an API contract.

#### Attachments
* [Reusable Payload Shape](Contracts.md#reusable-payload-shape)
```
