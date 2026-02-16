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
---

### Refinement Specification

Reqvire implements requirement refinement through explicit refinement elements linked to requirements.

#### Details
**SysML Refine Stereotype:**
- Refinement content is captured in dedicated elements (`specification`, `constraint`, `behavior`)
- Requirement owns refinement via `refinedBy`; refinement points back via `refine`
- Refinement elements can be attached by external requirements when ownership constraints allow

**Usage:**
- Acceptance criteria and technical details reside in refinement elements
- Requirement text stays intent-focused (EARS-style), with concise detail pointers
- Clarifying information and rationale are captured in linked refinements
- Refinements provide attachment-ready specification contracts across submodels

#### Metadata
  * type: specification
---

### Relation Semantics Specification

Reqvire implements SysML relation stereotypes for requirements management.

#### Details
**Requirement Decomposition:**
- `derive`/`derivedFrom`: SysML DeriveReqt stereotype
- Parent requirements decompose into child requirements
- Maintains traceability from stakeholder needs to system requirements

**Verification Linkage:**
- `verify`/`verifiedBy`: SysML Verify stereotype
- Links test cases and verification methods to requirements
- Enables verification coverage analysis

**Implementation Satisfaction:**
- `satisfy`/`satisfiedBy`: SysML Satisfy stereotype
- Links implementation artifacts (code) to requirements
- Demonstrates requirement fulfillment

**Refinement Ownership:**
- `refine`/`refinedBy`: SysML Refine stereotype
- Links refinement elements (specification, constraint, behavior) and specification files to requirements
- Establishes ownership: each refinement can only be owned by one requirement
- Together with the requirement, refinements drive implementation

**General Traceability:**
- `trace`: SysML Trace stereotype
- Loose coupling between related elements
- Cross-reference without hierarchical dependency

#### Metadata
  * type: specification
---

### Supported Element Types Specification

Element types supported by the system for classification and behavior determination, aligned with SysML and systems engineering standards.

#### Details
**Requirement Types:**

| Type | Description |
|------|-------------|
| `requirement` | System requirement (default type if not specified) |
| `user-requirement` | User requirement representing stakeholder needs |

**Verification Types:**

| Type | Description |
|------|-------------|
| `verification` | Verification through testing (equivalent to `test-verification`) |
| `test-verification` | Explicit verification through testing with documented test procedures |
| `analysis-verification` | Verification through formal analysis of documentation or code |
| `inspection-verification` | Verification through formal inspection or review |
| `demonstration-verification` | Verification through demonstration in a realistic environment |

**Refinement Types:**

| Type | Description | Constraints |
|------|-------------|-------------|
| `constraint` | Documents constraints that limit or bound the system | Only `refine` relations allowed |
| `behavior` | Documents behavior details and operational specifications | Only `refine` relations allowed |
| `specification` | Documents detailed specifications and technical descriptions | Only `refine` relations allowed |

**Other Types:**

| Type | Description |
|------|-------------|
| `other` | Generic custom element type |
| `other-TYPENAME` | Named custom element type (e.g., `other-interface`, `other-actor`) |

**Custom Type Pattern:**
- Use `other-TYPENAME` where TYPENAME is the custom type name
- TYPENAME must be at least one character (e.g., `other-x` is valid, `other-` alone is not)
- Custom types can only use `trace` relations

#### Metadata
  * type: specification
---

### Traceability Reporting Specification

Reqvire provides traceability reports per ISO/IEC/IEEE 29148.

#### Details
**Upward Traceability:**
- Trace from implementation to requirements
- Trace from tests to requirements
- Verification coverage reports

**Downward Traceability:**
- Trace from stakeholder needs to system requirements
- Derive chain visualization
- Model-centric hierarchical views

**Bidirectional Traceability:**
- Impact analysis for requirement changes
- Affected elements identification
- Change propagation tracking

#### Metadata
  * type: specification
---

### Verification Coverage Specification

Reqvire supports verification coverage analysis per INCOSE best practices.

#### Details
**Coverage Metrics:**
- Requirements with verification links (verifiedBy)
- Requirements without verification (coverage gaps)
- Verification completeness percentage

**Coverage Reports:**
- List unverified requirements
- Group by element type
- Identify critical gaps

**Verification Types:**
- test-verification: Automated or manual tests
- analysis-verification: Analysis and review
- inspection-verification: Physical inspection
- demonstration-verification: Demonstration of capability

#### Metadata
  * type: specification
---
