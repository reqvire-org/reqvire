# Elements

### Attachment Scope Validation Refinement Specification

#### Details
When validating attachments, the system is expected to enforce attachment scope constraints for refinement-element identifier targets and report errors with clear messages indicating the attaching element, the attachment target, and the reason for the violation.

Attachment scope validation is expected to enforce:
- Hierarchical independence from the refinement's defining hierarchy
- Upstream propagation within a hierarchy branch
- One-direction attachment flow between feature-root subgraphs

#### Metadata
  * type: specification
---

### Attachment Target Validation Refinement Specification

#### Details
Attachment targets support model element identifier references with family-specific compatibility rules.

**Identifier Targets:**
- Feature attachments must point to `ontology` elements only
- Requirement attachments must point to requirement-owned refinement element types only (`semantic-contract`, `constraint`, `behavior`, `specification`, `state`, `input-output`)
- Requirement attachment to `ontology` is invalid; requirements inherit ontology context from their owning feature path
- Normalized like relation targets (resolved to full identifier path)
- Validation is expected to reject identifiers pointing to non-attachable element types
- Validation is expected to reject unresolved identifiers
- Provides clear error message indicating the expected element type

This validation ensures that ontology context is owned by features and that requirement attachments reference reusable requirement-owned contracts.

#### Metadata
  * type: specification
---

### Default Requirement Type Assignment Refinement Specification

#### Details
When an element does not have a `#### Metadata` subsection with a `type` property, the system assigns the default type `requirement`.

This behavior is location-independent: all elements default to type `requirement` regardless of their folder location within the Git repository.

To use other element types, users must explicitly specify the type in the element's Metadata subsection, for example: `type: feature`.

Supported element types, type categories, refinement ownership semantics, and evidence-backed verification semantics are defined by the Reqvire ontology and semantic-contract model plus the Supported Element Types Specification.

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

### Feature Model Structure Specification

#### Details
Feature and requirement meanings are defined by the Reqvire core element, feature, requirement, relation, governance, and verification ontologies.

The validator shall enforce the structural rules derived from those contracts:
- Feature hierarchy may use `derive`/`derivedFrom` only between feature elements.
- Requirement hierarchy may use `derive`/`derivedFrom` only between requirement elements.
- Requirements specify features through `specify`; the inverse relation is `specifiedBy`.
- A top-level requirement must have `specify` pointing to exactly one feature.
- A child requirement may omit `specify` when it has `derivedFrom` pointing to another requirement; the owning feature is inherited through the requirement hierarchy.
- If a requirement has both `derivedFrom` and `specify`, the explicit `specify` feature must match the inherited feature.
- Features are not directly satisfied or verified; implementation and verification status roll up from requirements that specify them.
- Governance metadata is valid on feature and requirement elements only and inherits through the nearest parent in the same family or through the owning feature when a top-level requirement specifies a feature.

#### Metadata
  * type: specification

#### Relations
  * refine: [Feature Model Structure](ModelManagement.md#feature-model-structure)
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
- Their `refine` relation links back to the feature or requirement they refine, establishing ownership
- Each refinement can only be owned by one feature or requirement according to its subtype
- They do not define requirement governance metadata; governance context for a refinement is obtained from its owning feature or requirement

When a Refinement element contains relations other than `refine`, the validator is expected to report an error indicating that only `refine` relations are allowed for refinement types.

When a Refinement element declares requirement governance metadata keys (`status`, `priority`, `risk`, or `owner`), the validator is expected to report an error indicating that governance metadata is only valid on feature and requirement elements.

#### Metadata
  * type: specification
---

### Relation Element Type Validator Refinement Specification

#### Details
The validator enforces the Reqvire relation ontology together with the canonical element type vocabulary.

Validation shall check:
- relation endpoint families and inverse relation compatibility from the relation ontology
- ontology, feature, requirement, and requirement-owned refinement compatibility from the ontology, feature, requirement, and semantic-contract contracts
- evidence-backed verification compatibility from the verification contracts
- trace-only behavior for custom `other` and `other-TYPENAME` element types
- refinement restrictions: refinement elements use only `refine` relations and cannot have Attachments subsections
- `refinedBy` targets resolve to element identifiers, not plain file paths or `# Documents` file links without element fragments

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

### Requirement Governance Metadata Specification

Specification for declaring requirement governance metadata keys and values through the Metadata subsection.

#### Details
Requirement governance metadata is declared in the `#### Metadata` subsection of governance-bearing elements (`feature` and `requirement`).

Requirement governance metadata covers requirement management accountability and decision context: `status` represents the requirement lifecycle state, `priority` represents planning importance, `risk` represents realization risk, and `owner` represents maintenance accountability.

Elements outside the governance-bearing family are not requirement governance metadata authors and must not declare `status`, `priority`, `risk`, or `owner` metadata. Refinement elements (`source`, `semantic-contract`, `constraint`, `behavior`, `specification`, `state`, and `input-output`) obtain governance context from their owning feature or requirement instead of authored metadata.

When any non-governance-bearing element declares requirement governance metadata keys, the validator is expected to report an error indicating that governance metadata is only valid on feature and requirement elements.

Allowed values, default values, value meanings, inheritance source order, and persistence semantics are defined by the Reqvire governance ontology.

**Declaration Format:**
- ` * status: <status-value>`
- ` * priority: <priority-value>`
- ` * risk: <risk-value>`
- ` * owner: <owner-value>`

Only explicit `status: approved` metadata indicates that the element itself has been approved. Inherited or default status values are effective model context and must not be treated as approval evidence.

Risk represents requirement realization risk in the systems engineering sense: uncertainty and exposure associated with implementing, integrating, changing, or verifying the requirement. Risk is distinct from priority and does not by itself define hazard severity.

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

### Semantic Contract Reference Context Validation Specification

Technical specification for validating requirement-owned semantic-contract SHACL references against reachable ontology context.

#### Details
Semantic-contract SHACL references must resolve through reachable ontology context. Semantic reference validation issue kinds are defined by the Reqvire validation ontology. Missing references and references declared outside reachable ontology context are validation errors, not lint issues.

The validation rule is scoped through the semantic-contract owner:
- A semantic-contract must refine exactly one requirement.
- The owning requirement resolves exactly one owning feature through `specify`/`specifiedBy` and requirement hierarchy inheritance.
- The requirement ontology context is inherited from the owning feature and ancestor features in the feature hierarchy.
- Ontology hierarchy reachable from feature-attached ontology is part of the inherited context.
- Ontology elements outside inherited feature context or reachable ontology hierarchy are not considered reachable.

The validation rule inspects semantic-contract `#### Shapes` sections and checks these SHACL IRI references:
- `sh:targetClass`
- `sh:path`
- `sh:class`

For each referenced IRI, validation determines whether the IRI is declared by an ontology element and reachable from the semantic-contract owner context:
- If the IRI is not declared anywhere in Reqvire ontology elements, validation reports a missing semantic declaration and CRUD operations that would create that condition are blocked.
- If the IRI is declared by an ontology element outside the reachable owner context, validation reports an outside-context semantic reference and CRUD operations that would create that condition are blocked.
- Outside-context errors include the declaring ontology identifier and guidance to attach the declaring ontology to the owning or consuming feature when that dependency is intentional.

The rule is intentionally strict:
- It does not infer or create attachments.
- It does not rewrite Turtle.
- It enforces ontology hierarchy and feature-level ontology attachments as the only valid semantic dependency paths.
- It prevents model changes that would bypass change-impact traceability.

#### Metadata
  * type: specification

#### Relations
  * refine: [Semantic Contract Reference Context Validation](Validation.md#semantic-contract-reference-context-validation)
---

### Semantic Contract Structure Specification

#### Details
Semantic vocabulary and shape profile meaning are defined by the Reqvire ontology and semantic-contract model.

The implementation shall enforce the ontology and semantic-contract structure:
- Ontology elements define reusable vocabulary and model meaning.
- `semantic-contract` must not refine a feature.
- `semantic-contract` refining a requirement means a SHACL profile.
- Requirement-owned shape contracts define closed-world SHACL profiles over terms reachable from the owning requirement context and must not define local ontology terms.
- Ontology and semantic-contract elements use reserved type-specific subsections:
  - `ontology`: `#### Ontology` is required with exactly one fenced Turtle block; `#### Shapes` is forbidden.
  - `semantic-contract`: `#### Ontology` is forbidden; `#### Shapes` is required with exactly one fenced Turtle block.
- These reserved subsections are stored as ontology and semantic-contract ADT fields, not only as generic content.
- Reqvire derives ontology IRIs as `urn:reqvire:ontology:<element.id>`.
- Reqvire derives the semantic contract IRI as `urn:reqvire:semantic-contract:<element.id>`.
- The graph registry indexes ontology elements by `element.id`, `element.identifier`, derived IRI, ontology hierarchy, attachment consumers, and parsed `Ontology` content.
- The graph registry indexes semantic contracts by `element.id`, `element.identifier`, derived IRI, owning requirement, reachable ontology context, and parsed `Shapes` content.
- Reqvire builds a reusable semantic index from the graph registry for ontology validation, semantic-contract validation, ontology export, parsing each Turtle block once and reusing the parsed RDF quads for diagnostics, ontology term declarations, SHACL references, Turtle export, and JSON-LD export.
- `reqvire validate` parses `Ontology` and `Shapes` Turtle content with Oxigraph and inspects the parsed RDF graph instead of using raw text matching.
- `reqvire validate` treats ontology term declarations as globally owned by one ontology element:
  - The same ontology term IRI must not be declared by multiple ontology elements.
  - The same ontology term IRI must not be declared with conflicting roles, such as both `owl:Class` and `owl:DatatypeProperty`.
  - This validation applies to declared ontology terms, not to derived ontology element IRIs.
  - Duplicate and conflicting declaration issue kinds are defined by the Reqvire validation ontology.
- When `Shapes` exists, validation performs lightweight SHACL sanity checks:
  - The shapes graph contains at least one `sh:NodeShape` or `sh:PropertyShape`.
  - Each `sh:NodeShape` has at least one IRI `sh:targetClass`.
  - `sh:targetClass` values are declared by at least one ontology element in the Reqvire model.
  - Referenced `sh:property` shapes define exactly one IRI `sh:path`.
  - `sh:path` values are declared by at least one ontology element in the Reqvire model.
  - `sh:class` values are declared by at least one ontology element in the Reqvire model.
  - Missing declarations are validation errors because they create dangling semantic references.
  - Validation errors for missing semantic declarations must include the referencing semantic-contract identifier, reference kind, referenced IRI, and guidance to define the term or update/remove the SHACL reference before deleting or editing the declaring contract.
  - Declared references must also be reachable from the referencing semantic contract's owner context.
  - A feature context contains ontology elements attached by the feature and inherited through valid feature hierarchy traversal, plus ontology hierarchy reachable from those ontology elements.
  - A requirement-owned shape contract resolves its owner context through the owning requirement's feature context. Child requirements inherit the same feature context through requirement hierarchy.
  - A SHACL reference to a term declared outside reachable ontology context is a validation error, not a lint issue.
  - Validation errors for outside-context semantic references must include the referencing semantic-contract identifier, reference kind, referenced IRI, declaring ontology identifier, owning requirement, owning feature context, and guidance to attach the declaring ontology to the owning or consuming feature or move the declaration into reachable ontology context.
  - Supported constraint terms are checked for basic shape validity: `sh:minCount`, `sh:maxCount`, `sh:datatype`, `sh:class`, `sh:nodeKind`, `sh:pattern`, and `sh:in`.
  - `sh:maxCount` must be greater than or equal to `sh:minCount` when both are present.
  - `sh:in` must point to a valid RDF list.
- OWL reasoning and full SHACL conformance execution are not required for the initial semantic contract validator; they may be added later through optional adapters once the dependency footprint is acceptable.

#### Metadata
  * type: specification

#### Relations
  * refine: [Ontology and Semantic Contract Model](ModelManagement.md#ontology-and-semantic-contract-model)
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
