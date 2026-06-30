# Elements

### Capability Collect Traversal Specification

#### Details
Collect supports `capability`, `requirement`, and `ontology` start elements.

Default collection excludes implementation/evidence relations (`satisfiedBy`, `satisfy`, `verify`, `verifiedBy`) from structural hierarchy views.

When starting from a `requirement`:
- UPSTREAM traverses requirement parents through `derivedFrom`, then crosses to the owning capability through `specify` or inherited capability ownership, then traverses parent capabilities through `derivedFrom`.
- DOWNSTREAM traverses child requirements through `derive` only and does not cross to capabilities.
- The collected content includes authored concept references, plus each traversed requirement's requirement-detail contracts and requirement-owned contract contract_bindings.

When starting from a `capability`:
- UPSTREAM traverses parent capabilities through `derivedFrom` only and does not include requirements that specify those capabilities.
- DOWNSTREAM traverses child capabilities through `derive`, requirements through `specifiedBy`, and requirement descendants through `derive`.
- The collected content includes authored concept references for capability, descendant capability, and requirement elements, requirement-detail contracts, and contract_bindings for requirement elements.

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
  * define: [Capability Collect Traversal](ModelManagement.md#capability-collect-traversal)
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
- Capabilities are not directly verified or directly satisfied; implementation and verification coverage roll up from requirements that specify them.
- Governance metadata is valid on capability and requirement elements only and inherits through the nearest parent in the same family or through the owning capability when a top-level requirement specifies a capability.

#### Metadata
  * type: specification

#### Relations
  * define: [Capability Model Structure](ModelManagement.md#capability-model-structure)
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
  * define: [Workspace Root Path Authority](ModelManagement.md#workspace-root-path-authority)
---

### Contract Element Structure Constraints Specification

#### Details
Contract elements serve as requirement-owned subordinate details that drive implementation. Their relation usage is restricted because:
- They represent atomic pieces of information focused on documenting requirements
- They are primarily referenced through the Contract Bindings subsection of other elements
- Their `define` relation links back to the requirement they define, establishing ownership
- Each contract can only be owned by one compatible requirement according to its subtype
- They do not define requirement governance metadata; governance context for a contract is obtained from its owning requirement

When a contract element contains relations other than `define`, the validator is expected to report an error indicating that only `define` relations are allowed for contract types.

When a contract element declares requirement governance metadata keys (`status`, `priority`, `risk`, or `owner`), the validator is expected to report an error indicating that governance metadata is only valid on capability and requirement elements.

#### Metadata
  * type: specification
---

### Contract Specification

Reqvire implements requirement contracts through explicit contract elements linked to requirements.

#### Details
**Contract Ownership:**
- Contract content is captured in dedicated requirement-owned elements (`source`, `specification`, `constraint`, `behavior`, `state`, `input-output`)
- Requirement owns contract elements via `definedBy`; contract elements point back via `define`
- Contract elements can be reused by external requirements when ownership constraints allow
- Semantic contracts are first-class SHACL shape contracts outside contract ownership; requirements are constrained by them through `constrainedBy`/`constrain`, and semantic contracts use ontology through `use`/`usedBy`.

**Usage:**
- Acceptance criteria and technical details reside in contract elements
- Requirement text stays intent-focused (EARS-style), with concise detail pointers
- Clarifying information and rationale are captured in linked contracts
- Contract elements provide contract-bindings-ready specification contracts across submodels
- `state` contract elements capture lifecycle states, state machines, allowed transitions, terminal states, and state-dependent contract behavior.
- `input-output` contract elements capture payloads, messages, documents, schemas, fixtures, and data contracts crossing system or component boundaries.

#### Metadata
  * type: specification

#### Relations
  * define: [Contract Element Structure Constraints](ModelManagement.md#contract-element-structure-constraints)
---

### Default Requirement Type Assignment Contract Specification

#### Details
When an element does not have a `#### Metadata` subsection with a `type` property, the system assigns the default type `requirement`.

This behavior is location-independent: all elements default to type `requirement` regardless of their folder location within the effective workspace.

To use other element types, users must explicitly specify the type in the element's Metadata subsection, for example: `type: capability`.

Supported element types, type categories, contract ownership semantics, and evidence-backed verification semantics are defined by the Reqvire ontology and semantic-contract model plus the Supported Element Types Specification.

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
  * define: [Opt-In Element Size Estimate Model Build](ModelManagement.md#opt-in-element-size-estimate-model-build)
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

### Ignore Files Specification

Rules for processing .gitignore and .reqvireignore exclusion patterns.

#### Details
**Pattern Sources:**
- `.gitignore` - Version control exclusions (files not tracked by Git)
- `.reqvireignore` - Reqvire-specific exclusions (files tracked by Git but excluded from requirements processing)

**Processing Rules:**
- For each eligible Git worktree, ONLY the worktree-root .gitignore file is expected to be used (not nested .gitignore files in subdirectories)
- ONLY the effective-workspace-root .reqvireignore file is expected to be used (not nested .reqvireignore files in subdirectories)
- .reqvireignore is expected to use the same format and syntax as .gitignore
- Patterns from .gitignore and .reqvireignore is expected to be combined
- Files matching ANY exclusion pattern is expected to be excluded from parsing as requirements

**Exclusion Behavior Differences:**
- Files excluded by `.gitignore`: completely excluded - cannot be parsed as structured markdown AND cannot be referenced in file relations
- Files excluded by `.reqvireignore`: excluded from parsing BUT can still be referenced in file relations (useful for design documents, diagrams)

**Fallback Behavior:**
- If .reqvireignore does not exist at the effective workspace root, process normally using only eligible Git-worktree .gitignore patterns
- If an eligible Git worktree has no root .gitignore, process normally using only .reqvireignore patterns for that worktree

#### Metadata
  * type: specification
---

### Ignoring Unstructured Documents Contract Specification

#### Details
Unstructured document exclusion behavior:
- Uses `.reqvireignore` at the effective workspace root with `.gitignore`-compatible syntax.
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
  * define: [Ignoring Unstructured Documents](Configuration.md#ignoring-unstructured-documents)
---

### In-Memory Model Build Cache Specification

#### Details
The cache is a static `Mutex<Option<CachedModel>>` global holding the most recently built `ModelManager` together with the `CacheKey` that produced it.

**Cache key:**
- `options: ModelBuildOptions` — the full build-option struct (including `lenient` and `with_size_estimates`). Two different option sets always produce different keys.
- `files: BTreeMap<PathBuf, FileFingerprint>` — a sorted map of every scanned markdown file to its content fingerprint.
- `FileFingerprint = { len: u64, content_hash: String }` — file byte length plus a content hash of the file contents.

**Fingerprint computation:**
- Files are discovered by scanning the same markdown files the parser would consider (`utils::scan_markdown_files`), using the same exclusion patterns.
- For each file, contents are read and hashed. Added, removed, or modified files change the fingerprint and force a rebuild.

**Load path (`load_cached_model`):**
1. Compute the fingerprint and key.
2. Lock the cache and compare keys. On a match, clone and return the stored model without re-parsing.
3. On a miss, release the lock, rebuild via `ModelManager::parse_and_validate_with_options`, then store a clone of the rebuilt model under the new key and return the clone.

**Invalidation (`invalidate`):**
- Clears the stored entry, forcing the next `load_cached_model` call to rebuild. Called after every CRUD write in `tool_interface.rs` (add, move, rename, remove, merge, relink, link, unlink, mv-file, mv-folder, mv-asset, rm-asset).

**Scope:**
- Only the current working tree is cached. Git-commit history scans (`parse_and_validate`) bypass the cache entirely.

#### Metadata
  * type: specification

#### Relations
  * define: [In-Memory Model Build Cache](ModelManagement.md#in-memory-model-build-cache)
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
  * define: [Ontology and Semantic Contract Model](ModelManagement.md#ontology-and-semantic-contract-model)
---

### Operation Command Contract Specification

Common contract fields for graph-backed model operations.

#### Details
Operation family vocabulary is defined by operation requirements and specifications. Operation specifications define command-facing behavior and API behavior as contracts.

Each graph-backed operation specification is expected to define:
- command name or API entry point
- operation family, such as mutation, report, validation, formatting, or relation maintenance
- accepted inputs and path interpretation
- output behavior for text, JSON, dry-run, and diff modes where supported
- whether the operation requires a valid parsed model before execution
- whether the operation can persist source-file changes
- validation gates that must pass before persistence
- rollback behavior and error reporting when a candidate mutation is rejected
- relation, contract_bindings, and semantic-contract consistency guarantees preserved by the operation

Concrete command names, flags, output fields, file paths, workflow steps, and persistence behavior belong in these operation specifications or behavior contracts.

#### Metadata
  * type: specification

#### Relations
  * define: [Element Manipulation Operations](ModelManagement.md#element-manipulation-operations)
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

Reqvire implements relation semantics for ownership, hierarchy, capability specification, semantic-contract constraint, semantic-contract ontology use, verification, implementation satisfaction, and contract_bindings.

#### Details
- Relation names, inverse names, allowed source/target families, ownership semantics, and change-impact propagation are defined by the Reqvire relation ontology.
- Relation families group authored relation tokens and inverse pairs by stable model meaning so semantic search and SPARQL queries can ask for hierarchy, capability specification, ownership, semantic-contract constraint, ontology-use dependency, verification, satisfaction, and cross-subgraph contract-dependency relations without hard-coding every relation string.
- Each relation rule is expected to declare exactly one relation family and one semantic pattern. Only hierarchy-family relations have transitive closure properties; the other families are direct relation families unless a separate ontology rule defines derived behavior.
- Implementation relation validators shall enforce the relation ontology together with element-type compatibility constraints.
- Report and mutation code shall use the same relation direction and propagation semantics so validation, collect, submodels, coverage, and change impact remain consistent.
- Authored relation tokens must map to a declared semantic relation family. Generic semantic escape-hatch relations are not part of the canonical model; authors should use a semantically specific relation family or ontology concept references.

#### Concept References
  * [Relation Family](../Thesaurus/Thesaurus.md#relation-family)
  * [Relation Rule](../Thesaurus/Thesaurus.md#relation-rule)
  * [Relation Semantic Pattern](../Thesaurus/Thesaurus.md#relation-semantic-pattern)

#### Metadata
  * type: specification

#### Relations
  * define: [Relation Types and behaviors](ModelManagement.md#relation-types-and-behaviors)
---

### Requirement Governance Metadata Specification

Specification for declaring requirement governance metadata keys and values through the Metadata subsection.

#### Details
Requirement governance metadata is declared in the `#### Metadata` subsection of governance-bearing elements (`capability` and `requirement`).

Requirement governance metadata covers requirement management accountability and decision context: `status` represents the requirement lifecycle state, `priority` represents planning importance, `risk` represents realization risk, and `owner` represents maintenance accountability.

Elements outside the governance-bearing family are not requirement governance metadata authors and must not declare `status`, `priority`, `risk`, or `owner` metadata. Non-governance-bearing elements include ontology elements, semantic-contract elements, verification elements, and contract elements (`source`, `constraint`, `behavior`, `specification`, `state`, and `input-output`). Requirement-owned contracts obtain governance context from their owning requirement instead of authored metadata; semantic contracts are governed through the requirements they constrain.

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

### Requirements Processing Contract Specification

#### Details
Requirements-processing scope behavior:
- Discovery starts at the effective workspace root and walks folders and subfolders deterministically.
- Only supported structured Markdown files remain eligible for model parsing after document-type detection.
- Applicable Git-worktree-root `.gitignore` exclusions remove files from structured parsing and model relation target eligibility when that file exists.
- Workspace-root `.reqvireignore` exclusions remove files from structured parsing while preserving file-reference eligibility where the relation contracts allow it.
- Git ignore metadata from discovered repositories must not change the workspace-root-relative identity of any remaining path.
- Remaining in-scope files are parsed through the structured model pipeline.
- Pass 1 collects elements and local document structure.
- Pass 2 builds graph registry relations and validates target existence, type compatibility, and model-level consistency.
- Processing diagnostics must report workspace-root-relative paths.

#### Metadata
  * type: specification

#### Relations
  * define: [Requirements Processing](Configuration.md#requirements-processing)
---

### Semantic Contract Structure Specification

#### Details
Semantic vocabulary and shape profile meaning are defined by the Reqvire ontology and semantic-contract model.

The implementation shall enforce the ontology and semantic-contract structure:
- Ontology elements define reusable vocabulary and model meaning.
- `semantic-contract` must not use `define`/`definedBy`; semantic contracts constrain requirements with `constrain`/`constrainedBy`.
- `semantic-contract` must use at least one ontology element through `use`/`usedBy`.
- `source`, `constraint`, `behavior`, `specification`, `state`, and `input-output` must not define a capability; they are requirement-owned subordinate details or contracts.
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
- The graph registry indexes ontology elements by `element.id`, `element.identifier`, derived IRI, ontology hierarchy, contract_bindings consumers, and parsed `Ontology` content.
- The graph registry indexes semantic contracts by `element.id`, `element.identifier`, derived IRI, constrained requirements, used ontology context, and parsed `Shapes` content.
- Reqvire builds a reusable semantic index from the graph registry for ontology validation, semantic-contract validation, ontology export, parsing each Turtle block once and reusing the parsed RDF quads for diagnostics, ontology term declarations, SHACL references, Turtle export, and JSON-LD export.

#### Metadata
  * type: specification

#### Relations
  * define: [Ontology and Semantic Contract Model](ModelManagement.md#ontology-and-semantic-contract-model)
---

### Specification File Identification Contract Specification

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

### Structure and Addressing in Markdown Documents Contract Specification

#### Details
This contract delegates the canonical Markdown grammar to `MarkdownStructure` and binds consumers to the same model-addressing rules used by the parser.

Consumers of this contract must treat the following as one structure contract:
- Supported document forms are the parser-recognized `# Elements` multi-element document and `# Element` single-element document.
- Element headings, reserved subsections, metadata blocks, relation lists, Contract Bindings, Concept References, ontology blocks, and SHACL shape blocks are interpreted according to the Markdown structure model.
- Element IDs are derived from element names; element identifiers are workspace-root-relative file paths plus element fragments.
- Relation targets and Contract Bindings targets use the same identifier resolution, normalization, and validation rules as model parsing.
- Presentation or documentation consumers must not invent a parallel Markdown grammar or alternative identifier scheme.

#### Metadata
  * type: specification

#### Relations
  * define: [Structure and Addressing in Markdown Documents](StructureAndParsing.md#structure-and-addressing-in-markdown-documents)
---

### Structured Markdown Files Search and Detection Contract Specification

#### Details
Structured markdown detection behavior:
1. Scans eligible Git-worktree files under the effective workspace root and its subfolders.
2. Marks files matching configured exclusion patterns as non-structured targets.
3. Marks non-`.md` files as non-structured targets.
4. Retains only eligible markdown files for structured parsing passes.

#### Metadata
  * type: specification

#### Relations
  * define: [Structured Markdown Files Search and Detection](Configuration.md#structured-markdown-files-search-and-detection)
---

### Supported Element Types Specification

Element types supported by the system for classification and behavior determination.

#### Details
The canonical type vocabulary is defined by the Reqvire core element, capability, requirement, ontology, semantic-contract, and verification model contracts.

The implementation shall use those contracts as the authoritative source for:
- capability, requirement, contract, verification, and custom type categories
- default element type semantics
- requirement-owned contract type semantics
- verification-objective planning hierarchy semantics
- evidence-backed verification type semantics

Validation-adjacent migration support shall expose versioned migration candidates for breaking model-contract changes without changing validation output by default. Migration candidates identify affected semantic areas, dry-run expectations, and whether deterministic source rewriting is safe.

Parser-facing behavior remains:
- When `type` metadata is omitted, the element type is `requirement`.
- `type` metadata uses the exact element-type token declared in the semantic vocabulary.
- `other` and `other-TYPENAME` are custom extension types that cannot author semantic relations.
- `other-TYPENAME` requires at least one character after `other-`; `other-` alone is invalid.
- Custom types should be used only when no supported canonical element type fits; semantic meaning should be expressed through ontology concept references or a specific supported type.

#### Metadata
  * type: specification

#### Relations
  * define: [Element Type Relation Compatibility](ModelManagement.md#element-type-relation-compatibility)
---

### Verification Type Selection Guidelines

Usage guidelines for selecting appropriate verification types.

#### Details
**Default Verification Type:**
- `verification` - Verification through testing (equivalent to `test-verification`)

**Type Selection Guidelines:**
- **Verification-objective**: Verification planning or grouping objective. It is the mandatory parent for concrete verification work through `derivedFrom` and does not use `verify` or `satisfiedBy`.
- **Test-verification**: Quantitative requirements, functional behavior, performance criteria
- **Analysis-verification**: Design constraints, architectural requirements, compliance with standards
- **Inspection-verification**: Documentation requirements, labeling, configuration settings
- **Demonstration-verification**: User-facing capabilities, workflow requirements, integration scenarios

#### Metadata
  * type: specification
---

### Workspace Scope Specification

Path resolution, Git worktree eligibility, and scope validation rules for workspace-root-based project management.

#### Details
The effective Reqvire workspace root defines path normalization, identifier storage, diagnostic paths, export paths, and consumer-visible model paths. The workspace root is the process working directory after startup workspace selection has been applied. Git worktree membership defines which files and artifacts under that workspace root are eligible to participate in the SOI model. Git repository roots, branches, remotes, and commits do not define Reqvire identifier roots.

**Workspace Root Detection:**
- When a caller provides explicit workspace selection, Reqvire enters that directory before executing the requested command.
- When no explicit workspace selection is provided, the current process working directory is the effective workspace root.
- The effective workspace root may be a Git repository root, a child of a Git repository root, or a parent directory containing one or more Git repositories.
- A non-Git workspace root is allowed only as a container for one or more descendant Git worktrees; files outside those descendant worktrees are ignored.
- Git root detection is used to classify eligible files and artifacts by Git worktree membership and to collect revision, branch, remote, and dirty-state metadata; it must not change path normalization.
- All internal paths are normalized to workspace-root-relative format for storage.

**Git Worktree Eligibility Rules:**
- A file or artifact is eligible for SOI model processing only when it is inside the effective workspace root and inside a Git worktree.
- A workspace-root descendant folder that is not inside any Git worktree is ignored, including Markdown files, local assets, implementation files, and evidence artifacts under that folder.
- When the workspace root is a child of a Git worktree, files under the workspace root remain eligible because they are inside that parent Git worktree; files outside the workspace root remain out of scope.
- When the workspace root is a parent of multiple Git repositories, only files under discovered descendant Git worktrees are eligible.
- Nested Git worktrees remain ordinary workspace subdirectories for addressing; their repository identity may be reported as metadata but does not alter the workspace-root-relative identifier.
- Local InternalPath evidence, static assets, implementation files, and resource targets must also be inside an eligible Git worktree before they are included in resources reports, exports, semantic model facts, or other consumer-facing evidence/resource views.

**Path Resolution Rules:**
- Operation path arguments are resolved relative to the effective workspace root because workspace selection is applied as a startup directory change before operation execution.
- Markdown links without a leading slash are resolved relative to the source document that contains the link.
- Markdown links that start with `/` are resolved relative to the effective workspace root, not relative to any Git repository root.
- Source, target, diagnostic, report, diff, export, semantic-export, and consumer-visible paths are emitted in workspace-root-relative form.

**Processing Scope:**
- Reqvire processes eligible Git-worktree files under the effective workspace root, subject to supported ignore and exclusion rules.
- Content outside the effective workspace root is out of scope even when it belongs to a parent Git repository.
- Content inside the effective workspace root but outside all Git worktrees is ignored rather than parsed as model source or included as local evidence/resource content.
- Nested Git repositories under the effective workspace root do not reset Reqvire path roots; their files remain addressed by workspace-root-relative paths such as `repo-a/system-model/File.md#element`.
- Markdown files parsed as model documents become source file containers for consumer views that expose source navigation.
- Relation and contract_bindings targets outside parsed model documents remain modeled resources or evidence targets, not source file containers

**Scope Boundary Validation:**
- Relations referencing elements outside the effective workspace root report missing relation target errors.
- References using relative paths (for example `../ParentFile.md#element`) that escape the workspace root result in missing relation target errors.
- Root-relative links are constrained to the effective workspace root; they cannot address parent directories or sibling workspaces.
- Absolute operating-system paths are not internal Reqvire identifiers. If accepted as command input, they must resolve under the effective workspace root before being stored as workspace-relative paths.
- Missing relation target errors clearly identify unreachable references due to workspace scope limitations.
- Model mutations must not persist files outside the effective workspace root or into workspace folders that are outside all eligible Git worktrees.

**Workspace-Root Consumer Rules:**
- `format --fix` may rewrite authored links to canonical source-relative Markdown links for readability, but the resolved targets must remain workspace-root-relative identifiers internally.
- Migration and mutation commands must write changed files by resolving registry paths against the effective workspace root; they must not prepend the caller's pre-startup directory, a parent Git path, or a nested Git repository path.
- `change-impact` compares workspace-root-relative model snapshots built from eligible Git-worktree files and artifacts. A Git commit argument is a single-repository convenience adapter for materializing a base workspace snapshot; it must not redefine identifier roots.
- Semantic export, model JSON, coverage, resources, traces, search, diagnostics, and diff output must expose workspace-root-relative file paths and element identifiers.
- Consumer records, static export manifests, source target metadata, and element target metadata must use workspace-root-relative paths for eligible Git-worktree content only.
- Tooling workspace state may include Git metadata for eligible worktrees, but tool inputs, outputs, evidence references, mutation diffs, and resources must use workspace-root-relative paths.
- Nested repositories may provide revision metadata in future integrations, but model relations and identifiers continue to use one workspace-root-relative path namespace.

#### Metadata
  * type: specification
---
