# Reserved Subsections Specification

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
* verifies: [Target Requirement](file.md#target)
```

## Details Subsection

Must be defined with a level 4 header: `#### Details`.

The Details subsection provides additional information directly related to the main requirement text.

**Parsing Rules:**
- When parsing `#### Details` subsections, any markdown headers or elements within `<details>...</details>` tags are skipped
- Content within the Details subsection is considered an **extension of the requirement text**
- It serves the same purpose as refinement relation in other MBSE tools and SysML
- Any statements in the Details subsection hold the same validity as the main requirement text

**Examples:**
```markdown
### My Requirement

The system shall perform action X.

#### Details
Additional context about action X:
- Constraint 1
- Constraint 2
- Performance requirement
```

## Metadata Subsection

Must be defined with a level 4 header: `#### Metadata`.

The Metadata subsection stores element properties including type and custom attributes.

**Parsing Rules:**
- Contains properties in list format: `* property_name: property_value`
- Property entries are listed as bullet points (`*`), with **two spaces** (`  *`) of indentation followed by property_name + ': ' + property_value
- May include any custom properties, not just `type`

**Reserved Properties:**

The following properties have special meaning:

- `type`: Defines the element type (supported types are defined in [Supported Element Types](ModelManagement.md#supported-element-types))
- Additional reserved properties may be defined in future releases

**Examples:**
```markdown
### My Element

This is a verification element.

#### Metadata
  * type: verification
  * priority: high
  * owner: team-a

#### Relations
* verifies: [Some Requirement](#some-requirement)
```

```markdown
### My Element

This is a verification element.

#### Details
Some details.

#### Metadata
  * type: verification
  * priority: high
  * owner: team-a

#### Relations
  * verifies: [Some Requirement](#some-requirement)
```

## Attachments Subsection

Must be defined with a level 4 header: `#### Attachments`.

The Attachments subsection links external resources to elements. Attachments support two target types:

1. **File Paths**: Links to external documents (PDFs, spreadsheets, images, etc.)
2. **Element Identifiers**: Links to Refinement elements within the model

### File Path Attachments

**Parsing Rules:**
- Support markdown link syntax: `* [path](path)`
- Link text equals path (git-root-relative)
- Many-to-many relationship (multiple requirements can link same document)
- Never parse attachment files (treat as opaque)
- Auto-cleanup: remove subsection when empty

**Validation Rules:**
- Verify file existence on filesystem
- Validate markdown link format: `[path](path)`
- Detect duplicate attachments within element
- Report missing attachment targets as validation errors

**Examples:**
```markdown
### System Performance Requirements

The system shall meet defined performance criteria.

#### Attachments
* [docs/SLO.pdf](docs/SLO.pdf)
* [docs/benchmarks.xlsx](docs/benchmarks.xlsx)
```

### Element Identifier Attachments

Element identifier attachments link to Refinement elements (constraint, behavior, specification types) within the model.

**Parsing Rules:**
- Support markdown link syntax with fragment identifiers: `* [Element Name](path#element-id)`
- Same-file references: `* [Element Name](#element-id)`
- Cross-file references: `* [Element Name](relative/path/file.md#element-id)`
- Identifiers are normalized using the same rules as relation targets
- Link text contains the element name

**Validation Rules:**
- Target element must exist in the model
- Target element must be a Refinement type (constraint, behavior, specification)
- Non-Refinement element identifiers are rejected with a validation error

**Examples:**
```markdown
### System Performance Requirements

The system shall meet defined performance criteria.

#### Attachments
* [docs/SLO.pdf](docs/SLO.pdf)
* [Response Time Constraint](Constraints.md#response-time-constraint)
* [Timeout Behavior](Behaviors.md#timeout-behavior)
```

See [Refinement Elements Specification](RefinementElements.md) for details on refinement element types.
