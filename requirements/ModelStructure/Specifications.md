# Elements

### Capability Collect Traversal Specification

#### Details
Collect supports `capability`, `requirement`, and `ontology` start elements.

Default collection excludes implementation/evidence relations (`satisfiedBy`, `verify`, `verifiedBy`) and generic `trace` relations.

When starting from a `requirement`:
- UPSTREAM traverses requirement parents through `derivedFrom`, then crosses to the owning capability through `specify` or inherited capability ownership, then traverses parent capabilities through `derivedFrom`.
- DOWNSTREAM traverses child requirements through `derive` only and does not cross to capabilities.
- The collected content includes ontology context inherited from the owning capability path, plus each traversed requirement's requirement-detail refinements and requirement-owned contract attachments.

When starting from a `capability`:
- UPSTREAM traverses parent capabilities through `derivedFrom` only and does not include requirements that specify those capabilities.
- DOWNSTREAM traverses child capabilities through `derive`, requirements through `specifiedBy`, and requirement descendants through `derive`.
- The collected content includes attached ontology context for capability elements, inherited ontology context for descendant capability and requirement elements, requirement-detail refinements, and attachments for requirement elements.

When starting from an `ontology`:
- UPSTREAM traverses parent ontology elements through `derivedFrom`.
- DOWNSTREAM traverses child ontology elements through `derive` and includes semantic contracts that use each reachable ontology element through `use`/`usedBy`.
- The collected content is for semantic authoring context; full RDF/SHACL export remains the responsibility of the `ontologies` command.

The `specifiedBy`/`specify` bridge is therefore directional:
- Requirement UPSTREAM uses the bridge to add capability context.
- Capability DOWNSTREAM uses the bridge to add specified requirement context.
- Capability UPSTREAM and requirement DOWNSTREAM do not use the bridge.

#### Metadata
  * type: specification

#### Relations
  * refine: [Capability Collect Traversal](ModelManagement.md#capability-collect-traversal)
---

### Capability Model Structure Specification

#### Details
Capability and requirement meanings are defined by the Reqvire core element, capability, requirement, relation, governance, and verification ontologies.

The validator shall enforce the structural rules derived from those contracts:
- Capability hierarchy may use `derive`/`derivedFrom` only between capability elements.
- Requirement hierarchy may use `derive`/`derivedFrom` only between requirement elements.
- Requirements specify capabilities through `specify`; the inverse relation is `specifiedBy`.
- A top-level requirement must have `specify` pointing to exactly one capability.
- A child requirement may omit `specify` when it has `derivedFrom` pointing to another requirement; the owning capability is inherited through the requirement hierarchy.
- If a requirement has both `derivedFrom` and `specify`, the explicit `specify` capability must match the inherited capability.
- Capabilities may be directly verified but are not directly satisfied; implementation coverage rolls up from requirements that specify them.
- Governance metadata is valid on capability and requirement elements only and inherits through the nearest parent in the same family or through the owning capability when a top-level requirement specifies a capability.

#### Metadata
  * type: specification

#### Relations
  * refine: [Capability Model Structure](ModelManagement.md#capability-model-structure)
---

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
  * refine: [Git Repository as Project Root](ModelManagement.md#git-repository-as-project-root)
---

### Default Requirement Type Assignment Refinement Specification

#### Details
When an element does not have a `#### Metadata` subsection with a `type` property, the system assigns the default type `requirement`.

This behavior is location-independent: all elements default to type `requirement` regardless of their folder location within the Git repository.

To use other element types, users must explicitly specify the type in the element's Metadata subsection, for example: `type: capability`.

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

### Git Repository Scope Specification

Path resolution and scope validation rules for Git repository-based project management.

#### Details
Git repository scope defines source-file discovery and path normalization. It does not define logical model ownership and does not by itself classify a referenced path as a modeled resource or evidence file.

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
- Markdown files parsed as model documents become source file containers in the Explorer Project Store when the served Explorer runtime data is generated
- Relation and attachment targets outside parsed model documents remain modeled resources or evidence targets, not source file containers

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

### Ontology Annotation Convention Specification

#### Details
Authored ontology vocabulary shall use standard RDF/RDFS annotation properties for generic presentation metadata:
- Use `rdfs:label` for optional human-readable presentation labels when no more specific domain slot is needed.
- Use `rdfs:comment` for optional explanatory annotations on classes, properties, individuals, and axioms.
- Do not replace domain slots with annotations. A value that is a canonical authored token, parser field, export field, interface enum value, report kind, rule condition, queryable attribute, or controlled-vocabulary payload remains a declared ontology property even if its local name ends with `Name` or `Meaning`.
- Controlled-vocabulary individuals shall carry their formal semantics through IRI identity, typed class membership, hierarchy, and axioms. Their literal tokens or definitions shall be modeled with domain properties when those literals are part of the system contract.
- Do not introduce custom `*Name` or `*Meaning` datatype properties for purely generic labels or descriptions that have no separate domain semantics. If SHACL currently validates such a presentation-only field, refactor the SHACL path to `rdfs:label` or `rdfs:comment`; keep a custom token property only when the literal is consumed by parser, CLI/API, report, query, validation, or payload contracts.
- Do not keep deprecated presentation-only ontology properties in authored Reqvire ontology source. Refactoring history belongs in version control; the active ontology vocabulary should expose only current semantic terms and current contract tokens.
- SHACL shapes may require `rdfs:label` and `rdfs:comment` when the annotation itself is the intended contract. Those built-in RDFS annotation paths are treated as external vocabulary and do not require declaration by a Reqvire ontology element.

#### Metadata
  * type: specification

#### Relations
  * refine: [Ontology and Semantic Contract Model](ModelManagement.md#ontology-and-semantic-contract-model)
---

### Operation Command Contract Specification

Common contract fields for graph-backed model operations.

#### Details
Operation family vocabulary is defined by the Reqvire operation ontology. Operation specifications define command-facing behavior, not ontology vocabulary.

Each graph-backed operation specification is expected to define:
- command name or API entry point
- operation family, such as mutation, report, validation, formatting, or relation maintenance
- accepted inputs and path interpretation
- output behavior for text, JSON, dry-run, and diff modes where supported
- whether the operation requires a valid parsed model before execution
- whether the operation can persist source-file changes
- validation gates that must pass before persistence
- rollback behavior and error reporting when a candidate mutation is rejected
- relation, attachment, and semantic-contract consistency guarantees preserved by the operation

Concrete command names, flags, output fields, file paths, workflow steps, and persistence behavior belong in these operation specifications or behavior refinements.

#### Metadata
  * type: specification

#### Relations
  * refine: [Element Manipulation Operations](ModelManagement.md#element-manipulation-operations)
---

### Refinement Element Structure Constraints Refinement Specification

#### Details
Refinement elements serve as requirement-owned subordinate details or contracts that drive implementation. Their relation usage is restricted because:
- They represent atomic pieces of information focused on documenting requirements
- They are primarily referenced through the Attachments subsection of other elements
- Their `refine` relation links back to the requirement they refine, establishing ownership
- Each refinement can only be owned by one compatible requirement according to its subtype
- They do not define requirement governance metadata; governance context for a refinement is obtained from its owning requirement

When a Refinement element contains relations other than `refine`, the validator is expected to report an error indicating that only `refine` relations are allowed for refinement types.

When a Refinement element declares requirement governance metadata keys (`status`, `priority`, `risk`, or `owner`), the validator is expected to report an error indicating that governance metadata is only valid on capability and requirement elements.

#### Metadata
  * type: specification
---

### Refinement Specification

Reqvire implements requirement refinement through explicit refinement elements linked to requirements.

#### Details
**Refinement Ownership:**
- Refinement content is captured in dedicated requirement-owned elements (`source`, `specification`, `constraint`, `behavior`, `state`, `input-output`)
- Requirement owns refinement via `refinedBy`; refinement points back via `refine`
- Refinement elements can be attached by external requirements when ownership constraints allow
- Semantic contracts are first-class SHACL shape contracts outside refinement ownership; requirements are constrained by them through `constrainedBy`/`constrain`, and semantic contracts use ontology through `use`/`usedBy`.

**Usage:**
- Acceptance criteria and technical details reside in refinement elements
- Requirement text stays intent-focused (EARS-style), with concise detail pointers
- Clarifying information and rationale are captured in linked refinements
- Refinements provide attachment-ready specification contracts across submodels
- `state` refinements capture lifecycle states, state machines, allowed transitions, terminal states, and state-dependent contract behavior.
- `input-output` refinements capture payloads, messages, documents, schemas, fixtures, and data contracts crossing system or component boundaries.

#### Metadata
  * type: specification

#### Relations
  * refine: [Refinement Element Structure Constraints](ModelManagement.md#refinement-element-structure-constraints)
---

### Relation Operations Specification

Technical specification for relation link and unlink operations.

#### Details
**Source Resolution:**
- Source parameter accepts either an existing internal file path OR an element name
- Resolution order: first check if source exists as internal file path, if not search for element by name in registry
- Source must resolve to an existing element or file; report error if not found

**Target Resolution:**
- Target parameter must always be an existing element name
- Target must exist in the element registry; report error if not found

**Link Operation:**
- Create Relations subsection in source element if doesn't exist
- Add relation entry with format `* <relation-type>: [target-name](target-path)`
- Validate relation type against supported types
- Validate element type compatibility for the relation
- Skip if relation already exists (idempotent)

**Unlink Operation:**
- Remove relation entry from source element's Relations subsection
- Remove Relations subsection if no relations remain
- Report error if relation doesn't exist

#### Metadata
  * type: specification
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
  * refine: [Relation Types and behaviors](ModelManagement.md#relation-types-and-behaviors)
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
Requirement governance metadata is declared in the `#### Metadata` subsection of governance-bearing elements (`capability` and `requirement`).

Requirement governance metadata covers requirement management accountability and decision context: `status` represents the requirement lifecycle state, `priority` represents planning importance, `risk` represents realization risk, and `owner` represents maintenance accountability.

Elements outside the governance-bearing family are not requirement governance metadata authors and must not declare `status`, `priority`, `risk`, or `owner` metadata. Non-governance-bearing elements include ontology elements, semantic-contract elements, verification elements, and refinement elements (`source`, `constraint`, `behavior`, `specification`, `state`, and `input-output`). Requirement-owned refinements obtain governance context from their owning requirement instead of authored metadata; semantic contracts are governed through the requirements they constrain.

When any non-governance-bearing element declares requirement governance metadata keys, the validator is expected to report an error indicating that governance metadata is only valid on capability and requirement elements.

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

### Semantic Contract Structure Specification

#### Details
Semantic vocabulary and shape profile meaning are defined by the Reqvire ontology and semantic-contract model.

The implementation shall enforce the ontology and semantic-contract structure:
- Ontology elements define reusable vocabulary and model meaning.
- `semantic-contract` must not use `refine`/`refinedBy`; semantic contracts constrain requirements with `constrain`/`constrainedBy`.
- `semantic-contract` must use at least one ontology element through `use`/`usedBy`.
- `source`, `constraint`, `behavior`, `specification`, `state`, and `input-output` must not refine a capability; they are requirement-owned subordinate details or contracts.
- Semantic contracts define closed-world SHACL profiles over terms reachable from the contract's explicit ontology-use context and must not define local ontology terms.
- Ontology and semantic-contract elements use reserved type-specific subsections:
  - `ontology`: `#### Ontology` is required with exactly one fenced Turtle block; `#### Shapes` is forbidden.
  - `semantic-contract`: `#### Ontology` is forbidden; `#### Shapes` is required with exactly one fenced Turtle block.
- These reserved subsections are stored as ontology and semantic-contract ADT fields, not only as generic content.
- Top parent ontology elements in ontology hierarchy subgraphs must define non-empty `ontology_base` and `ontology_prefix` metadata. Child ontology elements inherit document-base and canonical prefix context from their ontology parent path.
- The root ontology Turtle block should explicitly declare `<ontology_base> a owl:Ontology` for authored OWL document identity.
- Reqvire derives one ontology document declaration per distinct resolved `ontology_base`; the ontology document IRI is `ontology_base`, and the term namespace is `<ontology_base>#`.
- Ontology elements that inherit the same `ontology_base` contribute vocabulary to the same generated `owl:Ontology` document declaration. A `derivedFrom` relation between ontology elements becomes `owl:imports` only when the source and target resolve to different ontology bases.
- Reqvire uses inherited `ontology_prefix` as the canonical CURIE label for the derived term namespace. Authored Turtle that uses the inherited prefix must explicitly declare it to `<ontology_base>#`; missing or conflicting declarations fail validation.
- Reqvire derives the semantic contract IRI as `urn:reqvire:semantic-contract:<element.id>`.
- The graph registry indexes ontology elements by `element.id`, `element.identifier`, derived IRI, ontology hierarchy, attachment consumers, and parsed `Ontology` content.
- The graph registry indexes semantic contracts by `element.id`, `element.identifier`, derived IRI, constrained requirements, used ontology context, and parsed `Shapes` content.
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
  - Built-in RDFS annotation paths `rdfs:label` and `rdfs:comment` are allowed as external annotation properties in `sh:path` and do not require local ontology term declarations.
  - Validation errors for missing semantic declarations must include the referencing semantic-contract identifier, reference kind, referenced IRI, and guidance to define the term or update/remove the SHACL reference before deleting or editing the declaring contract.
  - Declared references must also be reachable from the referencing semantic contract's ontology-use context.
  - A semantic-contract ontology-use context contains ontology elements linked through `use`/`usedBy` plus ontology ancestors reachable through ontology hierarchy.
  - A SHACL reference to a term declared outside reachable ontology context is a validation error, not a lint issue.
  - Validation errors for outside-context semantic references must include the referencing semantic-contract identifier, reference kind, referenced IRI, declaring ontology identifier, and guidance to add a `use` relation to the declaring ontology or an ontology descendant with the declaring ontology in its hierarchy.
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
 - `# Element`: parse as single-element model file with `## Metadata`, `## Relations` (optional), and `## <Actual Element Name>` body section where the section heading text defines the element name
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

### Supported Element Types Specification

Element types supported by the system for classification and behavior determination.

#### Details
The canonical type vocabulary is defined by the Reqvire core element, capability, requirement, ontology, semantic-contract, and verification model contracts.

The implementation shall use those contracts as the authoritative source for:
- capability, requirement, refinement, verification, and custom type categories
- default element type semantics
- requirement-owned refinement type semantics
- verification-objective planning hierarchy semantics
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
  * refine: [Element Type Relation Compatibility](ModelManagement.md#element-type-relation-compatibility)
---

### Verification Type Selection Guidelines

Usage guidelines for selecting appropriate verification types.

#### Details
**Default Verification Type:**
- `verification` - Verification through testing (equivalent to `test-verification`)

**Type Selection Guidelines:**
- **Verification-objective**: Verification planning or grouping objective. It organizes concrete verification work with `derivedFrom` but does not use `verify` or `satisfiedBy`.
- **Test-verification**: Quantitative requirements, functional behavior, performance criteria
- **Analysis-verification**: Design constraints, architectural requirements, compliance with standards
- **Inspection-verification**: Documentation requirements, labeling, configuration settings
- **Demonstration-verification**: User-facing capabilities, workflow requirements, integration scenarios

#### Metadata
  * type: specification
---
