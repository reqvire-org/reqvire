# Elements

### Containment Specification

Reqvire implements containment hierarchy through filesystem structure.

#### Details
**Folder Structure:**
- Folders represent packages/subsystems
- Nested folders create containment hierarchy
- Folder names define namespace for contained elements

**File Structure:**
- Markdown files contain element definitions
- Elements within a file share the file's containment context
- File path determines element's position in hierarchy

**Element Identity:**
- Full identifier: `path/to/file.md#element-fragment`
- Containment derived from file location
- No explicit containment relations needed

#### Metadata
  * type: specification

#### Relations
  * refine: [Git Repository as Project Root](Functional/Core/ModelManagement.md#git-repository-as-project-root)
---

### Refinement Specification

Reqvire implements requirement refinement through explicit refinement elements linked to requirements.

#### Details
**Refinement Ownership:**
- Refinement content is captured in dedicated requirement-owned elements (`source`, `semantic-contract`, `semantic-query-contract`, `specification`, `constraint`, `behavior`, `state`, `input-output`)
- Requirement owns refinement via `refinedBy`; refinement points back via `refine`
- Refinement elements can be attached by external requirements when ownership constraints allow

**Usage:**
- Acceptance criteria and technical details reside in refinement elements
- Requirement text stays intent-focused (EARS-style), with concise detail pointers
- Clarifying information and rationale are captured in linked refinements
- Refinements provide attachment-ready specification contracts across submodels
- `semantic-query-contract` refinements capture requirement-owned declarative semantic queries over reachable semantic model context without a query-kind classification.
- `state` refinements capture lifecycle states, state machines, allowed transitions, terminal states, and state-dependent contract behavior.
- `input-output` refinements capture payloads, messages, documents, schemas, fixtures, and data contracts crossing system or component boundaries.

#### Metadata
  * type: specification

#### Relations
  * refine: [Refinement Element Structure Constraints](Functional/Core/ModelManagement.md#refinement-element-structure-constraints)
---

### Relation Semantics Specification

Reqvire implements relation semantics for ownership, hierarchy, verification, implementation satisfaction, attachments, and traceability.

#### Details
- Relation names, inverse names, allowed source/target families, ownership semantics, and change-impact propagation are defined by the Reqvire relation ontology.
- Implementation relation validators shall enforce the relation ontology together with element-type compatibility constraints.
- Report and mutation code shall use the same relation direction and propagation semantics so validation, collect, submodels, coverage, and change impact remain consistent.
- `trace` remains non-owning documentation traceability and must not be used as a substitute for hierarchy, refinement ownership, verification, satisfaction, or attachment dependencies.

#### Metadata
  * type: specification

#### Relations
  * refine: [Relation Types and behaviors](Functional/Core/ModelManagement.md#relation-types-and-behaviors)
---

### Supported Element Types Specification

Element types supported by the system for classification and behavior determination.

#### Details
The canonical type vocabulary is defined by the Reqvire core element, capability, requirement, ontology, semantic-contract, semantic-query-contract, and verification model contracts.

The implementation shall use those contracts as the authoritative source for:
- capability, requirement, refinement, verification, and custom type categories
- default element type semantics
- requirement-owned refinement type semantics
- evidence-backed verification type semantics

Parser-facing behavior remains:
- When `type` metadata is omitted, the element type is `requirement`.
- `type` metadata uses the exact element-type token declared in the semantic vocabulary.
- `other` and `other-TYPENAME` are custom trace-only types.
- `other-TYPENAME` requires at least one character after `other-`; `other-` alone is invalid.
- Custom types can only use `trace` relations.

#### Metadata
  * type: specification

#### Relations
  * refine: [Element Type Relation Compatibility](Functional/Core/ModelManagement.md#element-type-relation-compatibility)
---

### Traceability Reporting Specification

Reqvire provides traceability reports over the Reqvire capability, requirement, verification, refinement, attachment, and implementation graph.

#### Details
- Traceability reports shall use Reqvire relation semantics for traversal direction, ownership, and evidence links.
- Upward reports shall trace implementation and verification evidence to requirements and owning capability roots where applicable.
- Downstream reports shall trace capability roots to specified requirements and requirement descendants.
- Change-impact reports shall use propagation relations, attachments, semantic dependencies, and impact scope rules to identify affected elements.

#### Metadata
  * type: specification

#### Relations
  * refine: [Model Reports](Functional/Output/Reporting.md#model-reports)
---

### Verification Coverage Specification

Reqvire supports verification coverage analysis for requirement verification and capability roll-up.

#### Details
- Verification type vocabulary, evidence-backed verification semantics, and capability coverage vocabulary are defined by the Reqvire verification and verification rollup ontologies.
- Coverage reports shall classify verified and unverified requirements from `verifiedBy`/`verify` relations.
- Coverage reports shall use the ontology-defined evidence-backed flag to decide whether a verification requires `satisfiedBy` evidence for coverage satisfaction.
- Capability coverage shall be reported by rolling up coverage from requirements that specify each capability and from descendant capability subgraphs.

#### Metadata
  * type: specification

#### Relations
  * refine: [Verification Coverage Report](Functional/Output/Reporting.md#verification-coverage-report)
---
