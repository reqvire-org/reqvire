# Refinement Elements Specification

## Overview

Refinement elements are specialized element types that provide detailed documentation for requirements and other model elements. They serve as atomic pieces of information that can be attached to parent elements to provide additional context, constraints, behavior descriptions, or detailed specifications.

## Refinement Element Types

Reqvire supports three refinement element types:

### 1. Constraint (`constraint`)

Constraints document limitations, bounds, or restrictions that apply to the system or requirement.

**Use cases:**
- Physical constraints (weight, size, power consumption)
- Performance constraints (latency, throughput, response time)
- Design constraints (technology choices, compatibility requirements)
- Environmental constraints (operating temperature, humidity)
- Regulatory constraints (compliance requirements)

**Example:**
```markdown
### Maximum Response Time

The system shall respond to user requests within 200 milliseconds under normal operating conditions.

#### Metadata
  * type: constraint
```

### 2. Behavior (`behavior`)

Behaviors document operational specifications, workflows, or functional descriptions that describe how the system operates.

**Use cases:**
- Operational workflows
- State machine descriptions
- Algorithm specifications
- Interaction patterns
- Error handling procedures

**Example:**
```markdown
### Session Timeout Behavior

When a user session remains inactive for 30 minutes, the system terminates the session and redirects the user to the login page with an appropriate message.

#### Metadata
  * type: behavior
```

### 3. Specification (`specification`)

Specifications document detailed technical descriptions, interfaces, or precise requirements that need formal documentation.

**Use cases:**
- Interface specifications
- Data format definitions
- Protocol descriptions
- API documentation
- Configuration specifications

**Example:**
```markdown
### API Response Format

All API responses shall use JSON format with the following structure:
- `status`: HTTP status code (integer)
- `data`: Response payload (object or array)
- `message`: Human-readable status message (string)

#### Metadata
  * type: specification
```

## Structure Rules

### No Relations Subsection

Refinement elements **cannot have a Relations subsection**. This is enforced during validation.

**Rationale:**
- Refinement elements are atomic documentation units
- They are referenced through Attachments, not through Relations
- Their content contributes to the parent element's documentation
- Keeping them relation-free simplifies their lifecycle management

**Invalid Example:**
```markdown
### Invalid Constraint

Some constraint text.

#### Metadata
  * type: constraint

#### Relations
  * derivedFrom: [Some Requirement](#some-requirement)  <!-- NOT ALLOWED -->
```

### Allowed Subsections

Refinement elements may contain:
- `#### Metadata` - Required for type specification
- `#### Details` - Optional additional information
- `#### Attachments` - Optional file attachments

## Attaching Refinement Elements

Refinement elements are attached to parent elements through the Attachments subsection using element identifier syntax:

```markdown
### Parent Requirement

The system shall meet performance requirements.

#### Attachments
  * [Maximum Response Time](Constraints.md#maximum-response-time)
  * [Session Timeout Behavior](Behaviors.md#session-timeout-behavior)
```

### Identifier Syntax

Attachments to refinement elements use markdown link syntax with element identifiers:
- Format: `[Element Name](path/to/file.md#element-id)`
- Same-file references: `[Element Name](#element-id)`
- Cross-file references: `[Element Name](relative/path/file.md#element-id)`

### Validation Rules

- Attachment identifiers must point to existing elements
- Target elements must be refinement types (constraint, behavior, specification)
- Non-refinement element identifiers are rejected during validation

## Change Impact

### Content Changes

When a refinement element's content changes:
- The content hash changes
- Change impact propagates to elements that attach this refinement
- Parent elements are marked as potentially affected

### Relocation Changes

When a refinement element is moved or renamed:
- Attachment identifiers in referencing elements are updated automatically
- This mirrors the behavior of relation target updates during CRUD operations

## Implementation Details

### Rust Representation

```rust
pub enum RefinementType {
    Constraint,
    Behavior,
    Specification,
}

pub enum ElementType {
    Requirement,
    UserRequirement,
    Verification(VerificationType),
    Refinement(RefinementType),
    Other,
}
```

### Metadata Parsing

The type is specified in the Metadata subsection:
```markdown
#### Metadata
  * type: constraint   // → ElementType::Refinement(RefinementType::Constraint)
  * type: behavior     // → ElementType::Refinement(RefinementType::Behavior)
  * type: specification // → ElementType::Refinement(RefinementType::Specification)
```

### Search Filtering

Refinement types support filtering via the search command:
```bash
./reqvire search --filter-type="constraint"
./reqvire search --filter-type="behavior"
./reqvire search --filter-type="specification"
```

## Best Practices

1. **Keep refinements focused**: Each refinement element should document a single concept
2. **Use appropriate types**: Choose constraint, behavior, or specification based on the nature of the information
3. **Group related refinements**: Organize refinement elements in dedicated files (e.g., `Constraints.md`, `Behaviors.md`)
4. **Attach to relevant elements**: Connect refinements to the requirements they support
5. **Maintain traceability**: Ensure all refinements are attached to at least one parent element
