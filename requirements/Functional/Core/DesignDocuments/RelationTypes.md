# Documents

## Metadata
  * type: specification

## Relations
  * refine: [Relation Types and behaviors](../ModelManagement.md#relation-types-and-behaviors)

## RelationTypes

# Relation Types Specification

## Relation Type Definition

A relation type in Reqvire:
- Defines a semantic connection between elements
- Specifies the directionality of the relationship
- Determines change propagation behavior
- May have an opposite/inverse relation type

## Core Concepts

### Relation Usage Categories

Relation usage categories are reusable model vocabulary and are defined by the Reqvire relation ontology. Implementations use those ontology terms to keep diagram rendering, reverse traversal, change propagation, and verification roll-up aligned.

## Comprehensive Relation Type Table

| Relation Type | Opposite Type | Forward Traversal | Reverse Traversal | Change Propagation | Description |
|---------------|---------------|-------------------|-------------------|-------------------|-------------|
| **derivedFrom** | derive | No | Yes | No | Links a child element to the parent element it is derived from |
| **derive** | derivedFrom | Yes | No | Yes | Links a parent element to child elements derived from it |
| **specify** | specifiedBy | No | Yes | No | Links a requirement to the feature it specifies |
| **specifiedBy** | specify | Yes | No | Yes | Links a feature to a requirement that specifies it |
| **satisfiedBy** | satisfy | Yes | No | Yes | Links a requirement to implementation elements that satisfy it |
| **satisfy** | satisfiedBy | No | Yes | No | Links an implementation to the requirement it satisfies |
| **refinedBy** | refine | Yes | No | Yes | Links a requirement to refinement elements |
| **refine** | refinedBy | No | Yes | No | Links a refinement element to the requirement it refines |
| **verifiedBy** | verify | Yes | No | Yes | Links a requirement to verification artifacts |
| **verify** | verifiedBy | No | Yes | No | Links a verification artifact to the requirement it verifies |
| **trace** | None | Yes | No | No | Establishes a trace relationship without change propagation |

## Relation Categories

Relation semantic categories are defined by the Reqvire relation ontology. This design document applies those categories to parser, validator, report, and impact-analysis behavior.

## Change Impact Rules

When an element changes, the impact propagates according to these rules:

1. **Hierarchical Changes**:
   - Changes to parent feature, requirement, or ontology elements propagate to all children in the same hierarchy family
   - This includes derivation relationships through `derive`

2. **Requirement Changes**:
   - Changes to requirements propagate to all satisfying implementations
   - Changes to requirements propagate to all refinement artifacts
   - Changes to requirements invalidate all verifications

3. **Implementation Changes**:
   - Changes to implementations rarely propagate upward to requirements
   - Implementations should be updated to maintain satisfaction

4. **Verification Changes**:
   - Changes to verification artifacts generally don't propagate
   - Verification updates may be needed after requirement changes

5. **Refinement Changes**:
   - Changes to requirements propagate to owned refinement artifacts via `refinedBy`
   - Refinements define and augment requirements; changes to the requirement may require updating its refinements

6. **Ontology Changes**:
   - Changes to ontology elements propagate through ontology hierarchy via `derive`
   - Changes to ontology content affect feature elements that attach the ontology, descendant feature contexts, requirements that specify those feature contexts, and semantic-contract shapes that reference reachable ontology terms

7. **Trace Relationships**:
   - Changes do not propagate through trace relationships
   - Trace relationships are used for documentation and discovery purposes only

## Element Type Relation Compatibility

This section defines which element types can use which relation types as source or target. These constraints ensure semantic consistency in the System model.

### Relation-Centric View

| Relation Type | Allowed Source Types | Allowed Target Types | Notes |
|---------------|---------------------|---------------------|-------|
| **derivedFrom** | feature, requirement, ontology | same source family only | Feature, requirement, and ontology hierarchy families stay separate |
| **derive** | feature, requirement, ontology | same source family only | Inverse of derivedFrom |
| **specify** | requirement | feature | Requirement specifies a feature |
| **specifiedBy** | feature | requirement | Feature is specified by a requirement |
| **satisfiedBy** | requirement, test-verification, formal-proof-verification | InternalPath (files) | System requirements and evidence-backed verifications link to implementation or evidence artifacts |
| **satisfy** | InternalPath (files) | requirement, test-verification, formal-proof-verification | Inverse of satisfiedBy (auto-generated) |
| **refinedBy** | feature, requirement | subtype-specific refinement types | Features own source refinements; requirements own semantic contracts and requirement-detail refinements |
| **refine** | refinement types | feature, requirement | Inverse of refinedBy (auto-generated), constrained by refinement subtype |
| **verifiedBy** | requirement | All verification types | Requirements link to verifications |
| **verify** | All verification types | requirement | Verifications link to requirements |
| **trace** | Any (except refinement types) | Any | Documentation/discovery, no type constraints |

### Element-Centric View

| Element Type | Can Use as Source | Can Be Target Of |
|--------------|-------------------|------------------|
| **feature** | derivedFrom, derive, specifiedBy, refinedBy, trace | derivedFrom, derive, specify, refine, trace |
| **requirement** | derivedFrom, derive, satisfiedBy, refinedBy, verifiedBy, trace | derivedFrom, derive, satisfy, refine, verify, trace |
| **test-verification** | verify, satisfiedBy, trace | verifiedBy, satisfy, trace |
| **analysis-verification** | verify, trace | verifiedBy, trace |
| **inspection-verification** | verify, trace | verifiedBy, trace |
| **demonstration-verification** | verify, trace | verifiedBy, trace |
| **formal-proof-verification** | verify, satisfiedBy, trace | verifiedBy, satisfy, trace |
| **ontology** | derivedFrom, derive, trace | derivedFrom, derive, trace, Feature Attachment |
| **source** | refine | refinedBy |
| **semantic-contract** | refine | refinedBy, Requirement Attachment |
| **constraint** | refine | refinedBy, Requirement Attachment |
| **behavior** | refine | refinedBy, Requirement Attachment |
| **specification** | refine | refinedBy, Requirement Attachment |
| **state** | refine | refinedBy, Requirement Attachment |
| **input-output** | refine | refinedBy, Requirement Attachment |
| **other** | trace | trace |

### Key Constraints

1. **derivedFrom/derive restricted to hierarchy families**: `feature` derives only from `feature`, `requirement` derives only from `requirement`, and `ontology` derives only from `ontology`. Feature, requirement, and ontology hierarchy must not be mixed through `derivedFrom`/`derive`.

2. **Feature-to-requirement bridge**: requirements use `specify` to point to the feature they specify. Features use `specifiedBy` to point to requirements that specify them. System requirements must have an immediate parent through either `derivedFrom` to another requirement or `specify` to a feature.

3. **Refinement types can only have refine relations**: Elements of type `source`, `semantic-contract`, `constraint`, `behavior`, `specification`, `state`, and `input-output` can only use `refine` relations to link to their compatible owner. `source` refines features; `semantic-contract`, `constraint`, `behavior`, `specification`, `state`, and `input-output` refine requirements. Each refinement can only be owned by one valid owner.

4. **satisfiedBy/satisfy restricted to implementable elements**: `satisfiedBy` links requirements and evidence-backed verifications to implementation/evidence files. Feature elements are not valid sources/targets for satisfaction relations.

5. **evidence-backed verification special cases**: Among verification types, `test-verification` and `formal-proof-verification` can use `satisfiedBy` relations. `test-verification` links to test implementations. `formal-proof-verification` links to formal proof artifacts, model-checking artifacts, theorem files, generated fixtures, or proof reports. Other verification types (`analysis-verification`, `inspection-verification`, `demonstration-verification`) cannot use `satisfiedBy`.

6. **Feature verification is roll-up only**: Feature elements are validated structurally and semantically, but are not directly verified through `verifiedBy`.

7. **formal-proof-verification evidence**: `formal-proof-verification` represents verification by formal proof, model checking, theorem proving, or other mathematically structured evidence. It verifies requirements through the same `verify`/`verifiedBy` relation family as other verification types and is expected to have at least one `satisfiedBy` artifact demonstrating the proof or generated evidence.

8. **other type conservative**: Elements with type `other` can only use `trace` relations to maintain flexibility while avoiding semantic conflicts.
