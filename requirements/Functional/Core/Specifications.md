# Elements

### Attachment Scope Validation Refinement Specification

#### Details
When validating attachments, the system is expected to enforce attachment scope constraints for refinement-element identifier targets and report errors with clear messages indicating the attaching element, the attachment target, and the reason for the violation.

Attachment scope validation is expected to enforce:
- Hierarchical independence from the refinement's defining hierarchy
- Upstream propagation within a hierarchy branch
- One-direction attachment flow between top-root subgraphs

#### Metadata
 * type: specification
---

### Element Size Estimate Model Build Specification

Element size estimates are expected to be optional model-build metadata.

#### Details
- The model build option is named `with_size_estimates`.
- The default value is `false`.
- When `with_size_estimates` is `true`, each parsed model element includes an optional `size_estimate` record.
- The `size_estimate` record contains `content_bytes`, `rendered_context_bytes`, and `estimated_tokens`.
- `content_bytes` is derived from authoritative element content.
- `rendered_context_bytes` is derived from the JSON evidence payload for the element, excluding the `size_estimate` field itself.
- `estimated_tokens` is a deterministic approximation derived from `rendered_context_bytes`.
- The size estimate is not part of the Markdown model source and must not be persisted into requirement files.
- Report-level aggregate estimates are not part of this specification.

#### Metadata
 * type: specification

#### Relations
 * refine: [Opt-In Element Size Estimate Model Build](ModelManagement.md#opt-in-element-size-estimate-model-build)
---

### Attachment Target Validation Refinement Specification

#### Details
Attachment targets support refinement-element identifier references only.

**Identifier Targets:**
- Must point to Refinement element types only (`constraint`, `behavior`, `specification`, `state`, `input-output`)
- Normalized like relation targets (resolved to full identifier path)
- Validation is expected to reject identifiers pointing to non-Refinement elements
- Validation is expected to reject unresolved identifiers
- Provides clear error message indicating the expected element type

This validation ensures that attachments reference valid Refinement elements that provide supplementary documentation.

#### Metadata
 * type: specification
---

### Default Requirement Type Assignment Refinement Specification

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
- `formal-proof-verification`
- `constraint` (refinement type)
- `behavior` (refinement type)
- `specification` (refinement type)
- `state` (refinement type)
- `input-output` (refinement type)
- `other`

#### Metadata
 * type: specification
---

### Element Type Metadata Specification

Specification for declaring element types in markdown documents through the Metadata subsection.

#### Details
Element types are identified through a reserved `type` metadata property in the Metadata subsection.

**Declaration Format:**
` * type: <element-type>` within the Metadata subsection.

**Default Type Assignment:**
- When no `type` property is specified, elements default to type `requirement`
- This behavior is location-independent (applies regardless of file location)

#### Metadata
 * type: specification
---

### Requirement Governance Metadata Specification

Specification for declaring requirement governance metadata keys and values through the Metadata subsection.

#### Details
Requirement governance metadata is declared in the `#### Metadata` subsection of requirement-family elements (`requirement` and `user-requirement`).

Requirement governance metadata covers requirement management accountability and decision context: `status` represents the requirement lifecycle state, `priority` represents planning importance, `risk` represents realization risk, and `owner` represents maintenance accountability.

Elements outside the requirement family are not requirement governance metadata authors and must not declare `status`, `priority`, `risk`, or `owner` metadata. Refinement elements (`constraint`, `behavior`, `specification`, `state`, and `input-output`) obtain governance context from their directly owning requirement instead of authored metadata.

When any non-requirement-family element declares requirement governance metadata keys, the validator is expected to report an error indicating that governance metadata is only valid on requirement-family elements.

**Declaration Format:**
- ` * status: <status-value>`
- ` * priority: <priority-value>`
- ` * risk: <risk-value>`
- ` * owner: <owner-value>`

**Status Values:**
- `draft`: The requirement is being authored or revised and is not ready for formal review.
- `review`: The requirement is ready for, or currently under, stakeholder or engineering review.
- `approved`: The requirement definition has completed review and is accepted as authoritative for downstream work.

Default: `approved`.

Only explicit `status: approved` metadata indicates that the requirement itself has been approved. Inherited or default status values are effective model context and must not be treated as approval evidence.

**Priority Values:**
- `low`: Useful or desirable, but deferrable without major mission, stakeholder, or integration impact.
- `medium`: Normal planning importance; expected to be delivered unless schedule, cost, or scope tradeoffs require adjustment.
- `high`: Important to mission, stakeholder value, integration, or compliance and should be protected during tradeoffs.
- `critical`: Essential; failure to satisfy creates unacceptable mission, safety, compliance, contractual, or release impact.

Default: `medium`.

**Risk Values:**
- `low`: Requirement realization is well understood, stable, feasible, and straightforward to verify.
- `medium`: Requirement realization has manageable uncertainty, moderate implementation or verification complexity, or limited downstream coupling.
- `high`: Requirement realization has significant technical uncertainty, volatility, verification difficulty, integration exposure, or likely downstream rework.
- `critical`: Requirement realization has severe uncertainty or exposure where failure, change, or non-compliance may materially affect mission, safety, compliance, cost, or schedule.

Default: `low`.

Risk represents requirement realization risk in the systems engineering sense: uncertainty and exposure associated with implementing, integrating, changing, or verifying the requirement. Risk is distinct from priority and does not by itself define hazard severity.

**Owner Value:**
- `owner` is a free-form string identifying the accountable person, role, or team responsible for maintaining the requirement.

Default: absent / unassigned. Structured effective metadata represents an unassigned owner as an empty string.

Effective value inheritance and persistence behavior are defined by the Requirement Governance Metadata Inheritance Behavior.

#### Metadata
 * type: specification
---

### Excluded File Relation Validation Refinement Specification

#### Details
Excluded-file relation validation behavior:
1. Registers files matching exclusion patterns in registry context for relation-target validation.
2. Skips internal element parsing/validation for excluded files.
3. Preserves ability to validate references that point to excluded file paths.

#### Metadata
 * type: specification

#### Relations
 * refine: [Excluded File Relation Validation](Validation.md#excluded-file-relation-validation)
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

#### Details
The system is expected to implement **Identifiers** and **Relations** following clearly defined specifications to ensure consistency, validity, and efficient querying and manipulation of these entities.

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
- ONLY the root .gitignore file is expected to be used (not nested .gitignore files in subdirectories)
- ONLY the root .reqvireignore file is expected to be used (not nested .reqvireignore files in subdirectories)
- .reqvireignore is expected to use the same format and syntax as .gitignore
- Patterns from .gitignore and .reqvireignore is expected to be combined
- Files matching ANY exclusion pattern is expected to be excluded from parsing as requirements

**Exclusion Behavior Differences:**
- Files excluded by `.gitignore`: completely excluded - cannot be parsed as structured markdown AND cannot be referenced in file relations
- Files excluded by `.reqvireignore`: excluded from parsing BUT can still be referenced in file relations (useful for design documents, diagrams)

**Fallback Behavior:**
- If .reqvireignore does not exist, process normally using only .gitignore patterns
- If .gitignore does not exist, process normally using only .reqvireignore patterns

#### Metadata
 * type: specification
---

### Ignoring Unstructured Documents Refinement Specification

#### Details
Unstructured document exclusion behavior:
- Uses `.reqvireignore` at repository root with `.gitignore`-compatible syntax.
- Supports glob patterns for structured-document processing exclusions.
- Keeps excluded files in repository scope while excluding them from structured parsing flow.
- Provides Reqvire-specific exclusions distinct from Git tracking behavior.

Example `.reqvireignore` patterns:
```.reqvireignore
# Example patterns to exclude from structured documents processing
**/Logical*.md
**/Physical*.md
**/draft-*.md
examples/**
```

#### Metadata
 * type: specification

#### Relations
 * refine: [Ignoring Unstructured Documents](Configuration.md#ignoring-unstructured-documents)
---

### Integrated Validation Refinement Specification

#### Details
Integrated validation execution behavior:
- Commands are split into model-dependent commands and raw-file commands.
- Model-dependent commands invoke two-pass validation before execution and stop on validation failures.
- Raw-file commands skip model validation when their behavior operates directly on file content.
- Validation gating ensures commands needing graph consistency do not run with invalid model state.

#### Metadata
 * type: specification

#### Relations
 * refine: [Integrated Validation](Validation.md#integrated-validation)
---

### Internal Consistency Validator Refinement Specification

#### Details
The consistency validator is expected to verify:
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

#### Details
Refinement elements serve as detailed documentation that augments requirements and drives implementation. Their relation usage is restricted because:
- They represent atomic pieces of information focused on documenting requirements
- They are primarily referenced through the Attachments subsection of other elements
- Their `refine` relation links back to the requirement they refine, establishing ownership
- Each refinement can only be owned by one requirement (uniqueness constraint)
- They do not define requirement governance metadata; governance context for a refinement is obtained from its directly owning requirement

When a Refinement element contains relations other than `refine`, the validator is expected to report an error indicating that only `refine` relations are allowed for refinement types.

When a Refinement element declares requirement governance metadata keys (`status`, `priority`, `risk`, or `owner`), the validator is expected to report an error indicating that governance metadata is only valid on requirement-family elements.

#### Metadata
 * type: specification
---

### Relation Element Type Validator Refinement Specification

#### Details
The validator enforces the constraints defined in the [Element Type Relation Compatibility](DesignDocuments/RelationTypes.md#element-type-relation-compatibility) specification:

- For `derivedFrom`/`derive` relations, validate that both source and target are requirement types (`requirement` or `user-requirement`)
- For `verifiedBy`/`verify` relations, validate that one endpoint is a requirement element and the other is a verification element
- For `satisfiedBy`/`satisfy` relations, validate that one endpoint is a system requirement (`requirement`), test-verification element, or formal-proof-verification element and the other is an implementation or evidence artifact; `user-requirement` is expected to not use `satisfiedBy`/`satisfy`
- For `refinedBy`, require identifier targets that resolve to refinement elements (`constraint`, `behavior`, `specification`, `state`, `input-output`)
- For `refinedBy`, reject plain file-path targets (InternalPath), including `# Documents` file links without element fragments
- For verification elements with `satisfiedBy` relations, validate that only evidence-backed verification elements (`test-verification` and `formal-proof-verification`) may use satisfiedBy (other verification types should not have satisfiedBy relations)
- `trace` relations are always allowed for any non-refinement element type
- Refinement types (`constraint`, `behavior`, `specification`, `state`, `input-output`) can only have `refine` relations and cannot have Attachments subsections
- Warnings should be issued when relation endpoints have incompatible element types

This validation occurs:
- During model parsing and validation (model.rs, parser.rs)
- During link operations at CRUD time (graph_registry.rs)

#### Metadata
 * type: specification
---

### Relation Types and behaviors Refinement Specification

#### Details
The system is expected to implement relations following clearly defined specifications for types and behaviors.

#### Metadata
 * type: specification
---

### Requirements Processing Refinement Specification

#### Details
Requirements-processing scope behavior:
- Applies `.gitignore` exclusions for files not in version-control processing scope.
- Applies `.reqvireignore` exclusions for files excluded from requirements parsing.
- Parses remaining in-scope files through the structured model pipeline.

#### Metadata
 * type: specification

#### Relations
 * refine: [Requirements Processing](Configuration.md#requirements-processing)
---

### Requirements Processing Specification

Specification for how requirements files are discovered and processed.

#### Details
**File Discovery:**
- Parse all .md files from git repository root
- Apply .gitignore and .reqvireignore exclusions

**Processing Pipeline:**
- Pass 1: Element collection and local validation
- Pass 2: Graph construction and relation validation
- GraphRegistry built from ElementRegistry after Pass 1

#### Metadata
 * type: specification
---

### Specification File Identification Refinement Specification

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

#### Details
The system is expected to implement semi-structured markdown format specifications that defines the structure, rules, and usage of **Elements**, **Subsections**, **Relations**, and **Identifiers** in Markdown (`.md`) documents following clearly defined specifications.

#### Metadata
 * type: specification
---

### Structured Markdown Files Search and Detection Refinement Specification

#### Details
Structured markdown detection behavior:
1. Scans files in repository root and subfolders.
2. Marks files matching configured exclusion patterns as non-structured targets.
3. Marks non-`.md` files as non-structured targets.
4. Retains only eligible markdown files for structured parsing passes.

#### Metadata
 * type: specification

#### Relations
 * refine: [Structured Markdown Files Search and Detection](Configuration.md#structured-markdown-files-search-and-detection)
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
