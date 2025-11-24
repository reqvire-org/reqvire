# Relation Types Specification

## Relation Type Definition

A relation type in Reqvire:
- Defines a semantic connection between elements
- Specifies the directionality of the relationship
- Determines change propagation behavior
- May have an opposite/inverse relation type

## Core Concepts

### Relation Usage Categories

Relations are categorized by their usage in different system functions:

1. **Diagram Rendering** - Relations that are rendered in visual diagrams to avoid duplicate arrows
   - Only one relation from each opposite pair is shown (e.g., `derive` but not `derivedFrom`)
   - Those are: `derive`, `satisfiedBy`, `verifiedBy`, `trace`

2. **Change Propagation** - Relations through which changes propagate to dependent elements
   - When an element changes, impact flows through these relation types
   - Those are: `derive`, `satisfiedBy`, `verifiedBy`

3. **Verification traces**: Relations through which propagation from the verification element to requirements in traced (verification roll-up)
   - Trace which requirements verification verifies: directly or indirecty
     - Parents inherit status from children via derive (e.g., ALL children verified => parent Verified).
   - Those are: `derivedFrom`

## Comprehensive Relation Type Table

| Relation Type | Opposite Type | Diagram Rendering | Change Propagation | Description |
|---------------|---------------|-------------------|-------------------|-------------|
| **derivedFrom** | derive | No | No | Links a child element to the parent element it is derived from |
| **derive** | derivedFrom | Yes | Yes | Links a parent element to child elements derived from it |
| **satisfiedBy** | satisfy | Yes | Yes | Links a requirement to elements that satisfy it |
| **satisfy** | satisfiedBy | No | No | Links an implementation to the requirement it satisfies |
| **verifiedBy** | verify | Yes | Yes | Links a requirement to verification artifacts |
| **verify** | verifiedBy | No | No | Links a verification artifact to the requirement it verifies |
| **trace** | None | Yes | No | Establishes a trace relationship without change propagation |

## Relation Categories

Relations are grouped into logical categories based on their semantic meaning:

### 1. Hierarchical/Transitive Relations

These relations define hierarchical structures and transitive ancestry within the model:
- **derivedFrom/derive**: Derivation of elements from higher-level elements

### 2. Satisfaction Relations

These relations connect requirements to implementations:

- **satisfiedBy/satisfy**: Links requirements to design, code, or architectural elements

### 3. Verification Relations

These relations connect requirements to verification elements:

- **verifiedBy/verify**: Links requirements to tests, validations, or other verification artifacts

### 4. Traceability Relations

These relations establish lightweight connections for documentation:

- **trace**: Simple non-directional traceability without strong semantic meaning or change propagation

## Change Impact Rules

When an element changes, the impact propagates according to these rules:

1. **Hierarchical Changes**:
   - Changes to parent elements propagate to all children
   - This includes derivation relationships

2. **Requirement Changes**:
   - Changes to requirements propagate to all satisfying implementations
   - Changes to requirements invalidate all verifications

3. **Implementation Changes**:
   - Changes to implementations rarely propagate upward to requirements
   - Implementations should be updated to maintain satisfaction

4. **Verification Changes**:
   - Changes to verification artifacts generally don't propagate
   - Verification updates may be needed after requirement changes

5. **Trace Relationships**:
   - Changes do not propagate through trace relationships
   - Trace relationships are used for documentation and discovery purposes only
