# Requirements

### Coexistence of Structured and Unstructured Documents

The system shall allow structured markdown and unstructured. (eg., markdown, PDFs, DOCX, raw text) documents to coexist within the same MBSE model.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing System Models](../UserStories.md#managing-system-models)
---

### Relation Types and behaviors

The system shall implement relations following clearly defined specifications for types and behaviors.

#### Details
<details>
<summary>View Full Specification</summary>


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
     - Parents inherit status from children via «derive» (e.g., ALL children verified => parent Verified).  
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
   

</details>

#### Relations
  * derivedFrom: [Identifiers and Relations](StructureAndParsing.md#identifiers-and-relations)
  * derivedFrom: [Managing System Models](../UserStories.md#managing-system-models)
  * satisfiedBy: [relation.rs](../../core/src/relation.rs)
---

### Efficient Processing

The system shall process structured documents and relations to extract model-relevant information efficiently.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing System Models](../UserStories.md#managing-system-models)
  * derivedFrom: [Promote Automation and Efficiency](../UserStories.md#promote-automation-and-efficiency)
---

### Default Requirement Type Assignment

The system shall automatically assign the **default type `requirement`** to all elements if not explicitly specified in their `metadata` subsection.

#### Details
<details>
<summary>Type Assignment Rules</summary>

When an element does not have a `#### Metadata` subsection with a `type` property, the system assigns the default type `requirement`.

**This behavior is location-independent:** All elements default to type `requirement` regardless of their folder location within the Git repository.

**To use other element types**, users must explicitly specify the type in the element's Metadata subsection:
```markdown
#### Metadata
  * type: user-requirement
```

**Supported element types:**
- `requirement` (default)
- `user-requirement`
- `verification` / `test-verification`
- `analysis-verification`
- `inspection-verification`
- `demonstration-verification`
- `other`

</details>

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing System Models](../UserStories.md#managing-system-models)
---

### Template-Based Model Bootstrapping

The system shall enable systems engineers to quickly bootstrap new MBSE models from predefined templates stored in Git repositories, accelerating project initialization and promoting best-practice model structures.

#### Details
Template Bootstrapping Capabilities

Users can initialize new models using the CLI with templates from Git repositories:
- Discover available templates within a specified repository
- Select and apply templates interactively
- Bootstrap model structure with predefined files, folders, and requirements

Templates are consumed from Git repositories only, with support for repositories containing multiple templates alongside other content.

**Example usage:**
```bash
reqvire init --template <github-repo-url>
```

The system discovers all available templates in the repository and allows the user to select which template to apply.

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing System Models](../UserStories.md#managing-system-models)
---

### Git Repository as Project Root

The system shall treat the **root directory of the Git repository as the project's base** for all file and folder references, streamlining configuration and promoting a self-contained project structure.

#### Details
All paths specified in Reqvire commands will be resolved relative to the current working directory:
- When run from the git repository root: paths are relative to the git root
- When run from a subdirectory: paths are relative to that subdirectory, and processing is limited to files within that subdirectory scope

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing System Models](../UserStories.md#managing-system-models)
---

### Element Manipulation Operations

The system shall provide programmatic manipulation of model elements through operations including, but not limited to, creating new elements, deleting existing elements, moving elements between locations, and renaming elements while maintaining model integrity and traceability.

#### Details
All manipulation operations shall:
- Maintain model integrity and consistency
- Update or remove affected relations automatically
- Preserve traceability where appropriate

#### Metadata
  * type: user-requirement

#### Relations
  * derivedFrom: [Managing System Models](../UserStories.md#managing-system-models)
---

### Verification Type Categories

The system shall support defined verifications categories.

#### Details
The following verification types are supported:

1. **Default Verification Type**
   - `verification` - Verification through testing (equivalent to `test-verification`)

2. **Specific Verification Types**
   - `test-verification` - Explicit verification through testing with documented test procedures
   - `analysis-verification` - Verification through formal analysis of documentation or code
   - `inspection-verification` - Verification through formal inspection or review
   - `demonstration-verification` - Verification through demonstration in a realistic environment

The appropriate verification type should be selected based on the nature of the requirement:
- **Test-verification**: Used when formal test procedures with expected outcomes are required
- **Analysis-verification**: Used when requirements can be verified through analysis of documentation or code
- **Inspection-verification**: Used when requirements can be verified through review of artifacts
- **Demonstration-verification**: Used when requirements can be verified by demonstrating functionality

#### Relations
  * derivedFrom: [Reserved Subsections Support](Subsections.md#reserved-subsections-support)
---

### Supported Element Types

The system shall support predefined element types for classification and behavior determination.

#### Details
Element types are identified through a reserved "type" metadata property. The following types are supported:

1. **requirement**: System requirement
2. **user-requirement**: User requirement
3. **verification**: For verification tests and validation procedures (equivalent to test-verification)
4. **test-verification**: For verification tests and validation procedures
5. **analysis-verification**: For verification through formal analysis of documentation or code
6. **inspection-verification**: For verification through formal inspection or review
7. **demonstration-verification**: For verification through demonstration in a realistic environment
8. **other**: Custom element types defined by users

#### Relations
  * derivedFrom: [Reserved Subsections Support](Subsections.md#reserved-subsections-support)
  * satisfiedBy: [element.rs](../../core/src/element.rs)
  * satisfiedBy: [parser.rs](../../core/src/parser.rs)
---
