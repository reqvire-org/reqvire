# Elements

### Attachment Scope Validation Refinement Specification

Specification extracted from requirement "Attachment Scope Validation".

#### Details
When validating attachments (both refinement elements and file assets), the system shall enforce the attachment scope constraints and report errors with clear messages indicating the attaching element, the attachment target, and the reason for the violation.

#### Metadata
  * type: specification
---

### Attachment Target Validation Refinement Specification

Specification extracted from requirement "Attachment Target Validation".

#### Details
Attachment targets support two types of references:

**File Paths:**
- Normalized to git-root-relative paths
- Validated for file existence during model validation
- Standard markdown link format where link text equals href

**Element Identifiers:**
- Must point to Refinement element types only (constraint, behavior, specification)
- Normalized like relation targets (resolved to full identifier path)
- Validation shall reject identifiers pointing to non-Refinement elements
- Provides clear error message indicating the expected element type

This validation ensures that attachments either reference existing files or valid Refinement elements that provide supplementary documentation.

#### Metadata
  * type: specification
---

### Default Requirement Type Assignment Refinement Specification

Specification extracted from requirement "Default Requirement Type Assignment".

#### Details
When an element does not have a `#### Metadata` subsection with a `type` property, the system assigns the default type `requirement`.

This behavior is location-independent: all elements default to type `requirement` regardless of their folder location within the Git repository.

To use other element types, users must explicitly specify the type in the element's Metadata subsection, for example: `type: user-requirement`.

Supported element types:
- `requirement` (default)
- `user-requirement`
- `verification` / `test-verification`
- `analysis-verification`
- `inspection-verification`
- `demonstration-verification`
- `constraint` (refinement type)
- `behavior` (refinement type)
- `specification` (refinement type)
- `other`

#### Metadata
  * type: specification
---

### Element Type Metadata Specification

Specification for declaring element types in markdown documents through the Metadata subsection.

#### Details
Element types are identified through a reserved `type` metadata property in the Metadata subsection.

**Declaration Format:**
`  * type: <element-type>` within the Metadata subsection.

**Default Type Assignment:**
- When no `type` property is specified, elements default to type `requirement`
- This behavior is location-independent (applies regardless of file location)

#### Metadata
  * type: specification
---

### Git Repository Scope Specification

Path resolution and scope validation rules for Git repository-based project management.

#### Details
**Git Root Detection:**
- Git root is detected via `git rev-parse --show-toplevel`
- All internal paths are normalized to git-root-relative format for storage

**Path Resolution Rules:**
- All paths are resolved relative to the current working directory
- When run from the git repository root: paths are relative to the git root
- When run from a subdirectory: paths are relative to that subdirectory

**Processing Scope:**
- When run from git root: all files in the repository are processed
- When run from a subdirectory: processing is limited to files within that subdirectory scope

**Scope Boundary Validation:**
- Relations referencing elements outside the subdirectory scope report missing relation target errors
- References using relative paths (e.g., `../ParentFile.md#element`) that escape the subdirectory result in missing relation target errors
- Absolute paths pointing outside the subdirectory scope generate missing relation target errors
- Missing relation target errors clearly identify the unreachable reference due to subdirectory scope limitations

#### Metadata
  * type: specification
---

### Identifiers and Relations Refinement Specification

Specification extracted from requirement "Identifiers and Relations".

#### Details
The system shall implement **Identifiers** and **Relations** following clearly defined specifications to ensure consistency, validity, and efficient querying and manipulation of these entities.

#### Metadata
  * type: specification
---

### Ignore Files Specification

Rules for processing .gitignore and .reqvireignore exclusion patterns.

#### Details
**Pattern Sources:**
- `.gitignore` - Version control exclusions (files not tracked by Git)
- `.reqvireignore` - Reqvire-specific exclusions (files tracked by Git but excluded from requirements processing)

**Processing Rules:**
- ONLY the root .gitignore file shall be used (not nested .gitignore files in subdirectories)
- ONLY the root .reqvireignore file shall be used (not nested .reqvireignore files in subdirectories)
- .reqvireignore shall use the same format and syntax as .gitignore
- Patterns from .gitignore and .reqvireignore shall be combined
- Files matching ANY exclusion pattern shall be excluded from parsing as requirements

**Exclusion Behavior Differences:**
- Files excluded by `.gitignore`: completely excluded - cannot be parsed as structured markdown AND cannot be referenced in file relations
- Files excluded by `.reqvireignore`: excluded from parsing BUT can still be referenced in file relations (useful for design documents, diagrams)

**Fallback Behavior:**
- If .reqvireignore does not exist, process normally using only .gitignore patterns
- If .gitignore does not exist, process normally using only .reqvireignore patterns

#### Metadata
  * type: specification
---

### Internal Consistency Validator Refinement Specification

Specification extracted from requirement "Internal Consistency Validator".

#### Details
The consistency validator shall verify:
- **Global Element Name Uniqueness**: Element names are globally unique across all files in the model
- **Duplicate Detection**: Detect and report when multiple elements in different files share the same name
- **Location Reporting**: Report both file locations where duplicate element names occur
- **Clear Error Messages**: Error messages clearly indicate that element names must be globally unique
- **Circular Dependencies**: Detect and report circular dependency chains in requirements
- **Orphaned Elements**: Identify elements without proper traceability connections
- **Inconsistent Patterns**: Detect relationship patterns that violate model constraints

Rationale: Element names serve as stable IDs for element identity, independent of file location. Global uniqueness is essential for proper element identification and change tracking across the model.

#### Metadata
  * type: specification
---

### Refinement Element Structure Constraints Refinement Specification

Specification extracted from requirement "Refinement Element Structure Constraints".

#### Details
Refinement elements serve as detailed documentation that augments requirements and drives implementation. Their relation usage is restricted because:
- They represent atomic pieces of information focused on documenting requirements
- They are primarily referenced through the Attachments subsection of other elements
- Their `refine` relation links back to the requirement they refine, establishing ownership
- Each refinement can only be owned by one requirement (uniqueness constraint)

When a Refinement element contains relations other than `refine`, the validator shall report an error indicating that only `refine` relations are allowed for refinement types.

#### Metadata
  * type: specification
---

### Relation Element Type Validator Refinement Specification

Specification extracted from requirement "Relation Element Type Validator".

#### Details
The validator enforces the constraints defined in the [Element Type Relation Compatibility](DesignDocuments/RelationTypes.md#element-type-relation-compatibility) specification:

- For `derivedFrom`/`derive` relations, validate that both source and target are requirement types (`requirement` or `user-requirement`)
- For `verifiedBy`/`verify` relations, validate that one endpoint is a requirement element and the other is a verification element
- For `satisfiedBy`/`satisfy` relations, validate that one endpoint is a system requirement (`requirement`) or test-verification element and the other is an implementation element; `user-requirement` shall not use `satisfiedBy`/`satisfy`
- For `refinedBy`, require identifier targets that resolve to refinement elements (constraint, behavior, specification)
- For `refinedBy`, reject plain file-path targets (InternalPath), including `# Documents` file links without element fragments
- For verification elements with `satisfiedBy` relations, validate that only test-verification elements may use satisfiedBy (other verification types should not have satisfiedBy relations)
- `trace` relations are always allowed for any non-refinement element type
- Refinement types (`constraint`, `behavior`, `specification`) can only have `refine` relations and cannot have Attachments subsections
- Warnings should be issued when relation endpoints have incompatible element types

This validation occurs:
- During model parsing and validation (model.rs, parser.rs)
- During link operations at CRUD time (graph_registry.rs)

#### Metadata
  * type: specification
---

### Relation Types and behaviors Refinement Specification

Specification extracted from requirement "Relation Types and behaviors".

#### Details
The system shall implement relations following clearly defined specifications for types and behaviors.

#### Metadata
  * type: specification
---

### Requirements Processing Specification

Specification for how requirements files are discovered and processed.

#### Details
**File Discovery:**
- Parse all .md files from git repository root
- Apply .gitignore and .reqvireignore exclusions
- Reserved files (README.md, LICENSE.md) are excluded

**Processing Pipeline:**
- Pass 1: Element collection and local validation
- Pass 2: Graph construction and relation validation
- GraphRegistry built from ElementRegistry after Pass 1

#### Metadata
  * type: specification
---

### Reserved Files Specification

Reserved repository documentation filenames automatically excluded from structured markdown processing.

#### Details
The following filenames are reserved for general repository documentation and are automatically excluded from requirements parsing:

**Reserved Filenames:**
- `README.md` - Project overview and documentation
- `CHANGELOG.md`, `CHANGES.md` - Version history and release notes
- `CONTRIBUTING.md` - Contribution guidelines
- `LICENSE.md` - License information
- `CODE_OF_CONDUCT.md` - Community conduct standards
- `SECURITY.md` - Security policies and vulnerability reporting
- `AUTHORS.md` - Project contributors and credits
- `ROADMAP.md` - Project roadmap and future plans

**Exclusion Rules:**
- Reserved filenames are excluded from structured markdown parsing across the entire repository
- Reserved files can be referenced in file relations to elements (excluded from parsing but linkable)
- Exclusion is combined with .gitignore and .reqvireignore patterns
- Files matching reserved filenames are excluded regardless of ignore file configuration

**Scope:**
- Applies to exact filename matches (case-sensitive on case-sensitive filesystems)
- Applies at all directory levels in the repository
- Takes precedence before ignore pattern evaluation

#### Metadata
  * type: specification
---

### Specification File Identification Refinement Specification

Specification extracted from requirement "Specification File Identification".

#### Details
- Supported first H1 headings:
  - `# Elements`: parse as multi-element model file
  - `# Documents`: parse as single-element model file with `## Metadata`, `## Relations` (optional), and `## <Actual Element Name>` body section where the section heading text defines the element name
- Leading whitespace, blank lines, or frontmatter before the heading are allowed
- Files without a supported first H1 are silently skipped (no error)
- This rule applies in addition to `.gitignore` and `.reqvireignore` exclusions
- The page title is not stored or tracked by the system

#### Metadata
  * type: specification
---

### Structure and Addressing in Markdown Documents Refinement Specification

Specification extracted from requirement "Structure and Addressing in Markdown Documents".

#### Details
The system shall implement semi-structured markdown format specifications that defines the structure, rules, and usage of **Elements**, **Subsections**, **Relations**, and **Identifiers** in Markdown (`.md`) documents following clearly defined specifications.

#### Metadata
  * type: specification
---

### Verification Type Selection Guidelines

Usage guidelines for selecting appropriate verification types.

#### Details
**Default Verification Type:**
- `verification` - Verification through testing (equivalent to `test-verification`)

**Type Selection Guidelines:**
- **Test-verification**: Quantitative requirements, functional behavior, performance criteria
- **Analysis-verification**: Design constraints, architectural requirements, compliance with standards
- **Inspection-verification**: Documentation requirements, labeling, configuration settings
- **Demonstration-verification**: User-facing features, workflow requirements, integration scenarios

#### Metadata
  * type: specification
---
