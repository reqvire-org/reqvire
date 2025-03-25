# Relation Types Registry

This document serves as the single, authoritative source of truth for relation types in the ReqFlow system. It defines all supported relation types, their directionality, impact behaviors, and usage patterns.

## Relation Type Definition

A relation type in ReqFlow:
- Defines a semantic connection between elements
- Specifies the directionality of the relationship
- Determines change propagation behavior
- May have an opposite/inverse relation type

## Core Concepts

### Directionality

Relations have three possible directionality patterns:

1. **Forward** - The relation flows from the source element to the target element 
   - Example: `contain` points from a parent to a child element
   
2. **Backward** - The relation flows from the target element to the source element
   - Example: `derivedFrom` points from a child back to its parent element
   
3. **Neutral** - The relation has no inherent direction
   - Example: `trace` simply indicates a relationship without directionality

### Change Propagation

The direction of change propagation is not always the same as relation directionality:

- In hierarchical relationships, changes propagate downward from parents to children
- Some relations like `verifiedBy` specifically trigger invalidation rather than just change propagation 

## Comprehensive Relation Type Table

| Relation Type | Direction | Opposite Type | Change Propagation | Description |
|---------------|-----------|---------------|-------------------|-------------|
| **containedBy** | Backward | contain | Parent → Child | Links a child element to its containing parent element |
| **contain** | Forward | containedBy | Parent → Child | Links a parent element to the child elements it contains |
| **derivedFrom** | Backward | derive | Parent → Child | Links a child element to the parent element it is derived from |
| **derive** | Forward | derivedFrom | Parent → Child | Links a parent element to child elements derived from it |
| **refine** | Backward | refinedBy | Parent → Child | Links a child element to a parent element it refines with more detail |
| **refinedBy** | Forward | refine | Parent → Child | Links a parent element to child elements that refine it |
| **satisfiedBy** | Forward | satisfy | Requirement → Implementation | Links a requirement to elements that satisfy it |
| **satisfy** | Backward | satisfiedBy | Requirement → Implementation | Links an implementation to the requirement it satisfies |
| **verifiedBy** | Forward | verify | Requirement → Verification | Links a requirement to verification artifacts |
| **verify** | Backward | verifiedBy | Requirement → Verification | Links a verification artifact to the requirement it verifies |
| **trace** | Neutral | None | None (Documentation) | Establishes a trace relationship without change propagation |

## Relation Categories

Relations are grouped into logical categories based on their semantic meaning:

### 1. Parent-Child Hierarchical Relations

These relations define hierarchical structures within the model:

- **containedBy/contain**: Physical or logical containment hierarchy
- **derivedFrom/derive**: Derivation of elements from higher-level elements
- **refine/refinedBy**: Refinement relationships adding more detail

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
   - This includes containment, derivation, and refinement relationships

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

## Validation Rules

The system must validate relation usage according to these rules:

1. Only the relation types defined in this registry are allowed
2. Relations should connect elements of appropriate types
3. Circular dependencies should be detected and reported

## Implementation Notes

1. The RELATION_TYPES HashMap in relation.rs is the implementation of this registry
2. Each relation is implemented with the RelationTypeInfo structure containing:
   - name: The relation type name
   - direction: Forward, Backward, or Neutral directionality 
   - opposite: The opposite relation type, if any
   - description: Human-readable description

3. Relation directionality determines how relations are stored and queried in the model
4. The change impact algorithm uses these relation definitions to determine propagation


