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

1. **Diagram Rendering (Forward)** - Relations that are rendered in visual diagrams to avoid duplicate arrows
   - Only one relation from each opposite pair is shown (e.g., `derive` but not `derivedFrom`)
   - Used for forward traversal (root to leaf) in model views
   - Those are: `derive`, `satisfiedBy`, `verifiedBy`, `trace`

2. **Reverse Traversal (Backward)** - Relations used for leaf-to-root traversal
   - Opposite of diagram rendering relations
   - Used for reverse model views and upward traceability
   - Those are: `derivedFrom`, `satisfy`, `verify`

3. **Change Propagation** - Relations through which changes propagate to dependent elements
   - When an element changes, impact flows through these relation types
   - Those are: `derive`, `satisfiedBy`, `verifiedBy`

4. **Verification traces**: Relations through which propagation from the verification element to requirements in traced (verification roll-up)
   - Trace which requirements verification verifies: directly or indirecty
     - Parents inherit status from children via derive (e.g., ALL children verified => parent Verified).
   - Those are: `derivedFrom`

## Comprehensive Relation Type Table

| Relation Type | Opposite Type | Forward Traversal | Reverse Traversal | Change Propagation | Description |
|---------------|---------------|-------------------|-------------------|-------------------|-------------|
| **derivedFrom** | derive | No | Yes | No | Links a child element to the parent element it is derived from |
| **derive** | derivedFrom | Yes | No | Yes | Links a parent element to child elements derived from it |
| **satisfiedBy** | satisfy | Yes | No | Yes | Links a requirement to elements that satisfy it |
| **satisfy** | satisfiedBy | No | Yes | No | Links an implementation to the requirement it satisfies |
| **verifiedBy** | verify | Yes | No | Yes | Links a requirement to verification artifacts |
| **verify** | verifiedBy | No | Yes | No | Links a verification artifact to the requirement it verifies |
| **trace** | None | Yes | No | No | Establishes a trace relationship without change propagation |

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

## Element Type Relation Compatibility

This section defines which element types can use which relation types as source or target. These constraints ensure semantic consistency in the System model.

### Relation-Centric View

| Relation Type | Allowed Source Types | Allowed Target Types | Notes |
|---------------|---------------------|---------------------|-------|
| **derivedFrom** | requirement, user-requirement | requirement, user-requirement | Hierarchical requirement decomposition only |
| **derive** | requirement, user-requirement | requirement, user-requirement | Inverse of derivedFrom |
| **satisfiedBy** | requirement, user-requirement, test-verification | InternalPath (files), refinement types | Requirements/tests link to implementations or refinements |
| **satisfy** | InternalPath (files), refinement types | requirement, user-requirement, test-verification | Inverse of satisfiedBy (auto-generated) |
| **verifiedBy** | requirement, user-requirement | All verification types | Requirements link to verifications |
| **verify** | All verification types | requirement, user-requirement | Verifications link to requirements |
| **trace** | Any (except refinement types) | Any | Documentation/discovery, no type constraints |

### Element-Centric View

| Element Type | Can Use as Source | Can Be Target Of |
|--------------|-------------------|------------------|
| **requirement** | derivedFrom, derive, satisfiedBy, verifiedBy, trace | derivedFrom, derive, satisfy, verify, trace |
| **user-requirement** | derivedFrom, derive, satisfiedBy, verifiedBy, trace | derivedFrom, derive, satisfy, verify, trace |
| **test-verification** | verify, satisfiedBy, trace | verifiedBy, satisfy, trace |
| **analysis-verification** | verify, trace | verifiedBy, trace |
| **inspection-verification** | verify, trace | verifiedBy, trace |
| **demonstration-verification** | verify, trace | verifiedBy, trace |
| **constraint** | satisfy | satisfy, Attachment |
| **behavior** | satisfy | satisfy, Attachment |
| **specification** | satisfy | satisfy, Attachment |
| **other** | trace | trace |

### Key Constraints

1. **derivedFrom/derive restricted to requirement types**: Only `requirement` and `user-requirement` elements can participate in derivation relationships. This ensures clean hierarchical requirement decomposition without mixing verification or other element types.

2. **Refinement types can only have satisfy relations**: Elements of type `constraint`, `behavior`, and `specification` can only use `satisfy` relations to link to requirements they fulfill. They can also be referenced via the Attachments subsection of other elements.

3. **test-verification special case**: Among verification types, only `test-verification` can use `satisfiedBy` relations (to link to test implementations). Other verification types (`analysis-verification`, `inspection-verification`, `demonstration-verification`) cannot use `satisfiedBy`.

4. **other type conservative**: Elements with type `other` can only use `trace` relations to maintain flexibility while avoiding semantic conflicts.