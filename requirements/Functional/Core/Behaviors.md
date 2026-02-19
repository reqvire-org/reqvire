# Elements

### Attachment Identifier CRUD Update Behavior

When a Refinement element is moved or renamed through CRUD operations, all attachment identifiers referencing that element must be updated using the same update mechanism as relation target updates.

#### Details
The update process follows these steps:

1. **Identify affected attachments**: Find all elements that have attachment identifiers pointing to the affected Refinement element
2. **Update identifier paths**: For each affected attachment:
   - **On move**: Update the file path portion of the identifier to reflect the new location
   - **On rename**: Update the element name portion of the identifier (fragment) to reflect the new name
3. **Preserve link text**: The display text of the markdown link is preserved
4. **File persistence**: Modified files are written back to disk with updated attachments

This behavior mirrors the existing relation target update behavior used when moving or renaming elements, ensuring consistency across the model.

#### Metadata
  * type: behavior
---

### Attachment Input Auto-Detection Behavior

When attaching or detaching via CLI commands, the system shall resolve attachment targets as refinement-element identifiers.

#### Details
The resolution follows this order:

1. **Identifier Parse**: Parse target input as an element identifier (for example `file.md#refinement-id`).
2. **Identifier Normalization**: Resolve and normalize identifier to full internal identifier format.
3. **Type Validation**: Confirm target element type is a Refinement type (`constraint`, `behavior`, `specification`).
4. **Error Handling**: If parsing or resolution fails, report a clear error message indicating that a refinement identifier target is required.

#### Metadata
  * type: behavior
---

### Subdirectory Auto-Detection Behavior

Describes how subdirectory scope detection and enforcement works.

#### Details
**Detection Steps:**

1. **Detect git root**: Run `git rev-parse --show-toplevel` to find repository root
2. **Determine relative scope**: Calculate current working directory path relative to git root
3. **Limit file processing**: Only process specification files within the current subdirectory
4. **Validate references**: References to elements outside the subdirectory scope generate missing target errors

**When run from git root:**
- Process all files in the repository
- No scope limitations apply

This behavior enables focused work on specific areas of large models while maintaining reference integrity.

#### Metadata
  * type: behavior
---

### Two-Pass Validation Behavior

Two-phase validation process for model parsing.

#### Details
**Pass 1: Element Collection**
- Parse all markdown files
- Extract elements with metadata
- Local validation (uniqueness, format, syntax)
- Report errors if found

**Pass 2: Graph Validation**
- Build in-memory model representation from elements
- Validate relations (existence, type compatibility)
- Cross-component validation
- Report errors if found

#### Metadata
  * type: behavior
---

### Type Validation Error Behavior

Error messages for invalid types shall include the list of valid types.

#### Details
**Element Type Errors:**
When an invalid element type is encountered (in metadata or filters):
- Error message shall include the invalid type value
- Error message shall list all valid element types
- Error message shall include the custom type pattern: "For custom types use: other-TYPENAME"

**Relation Type Errors:**
When an invalid relation type is encountered:
- Error message shall include the invalid relation type
- Error message shall list all valid relation types in sorted order

This enables users to quickly identify and fix type errors without consulting documentation.

#### Metadata
  * type: behavior
---

### Validation Error Reporting Behavior

Error message structure for validation issues.

#### Details
- File path and line number included
- Element name and relation details shown
- Color coding per Color Scheme Specification
- Actionable suggestions when possible

#### Metadata
  * type: behavior
---
