# Elements

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
- Optional terminal color coding follows the Functional Output Color Scheme Specification
- Actionable suggestions when possible

#### Metadata
  * type: behavior
---
