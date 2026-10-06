# Elements

### Existing Element Selection

As a **System Engineer**, I want to select existing model elements consistently by name or canonical identifier, so that interactive commands and automated clients can use the same model references across operations and interfaces.

#### Metadata
  * type: capability

#### Relations
  * derivedFrom: [Defining Model Structure](ModelStructureFeature.md#defining-model-structure)
  * specifiedBy: [Existing Element Reference Resolution](#existing-element-reference-resolution)
---

### Existing Element Reference Resolution

When an operation argument selects an existing model element by name, the system SHALL also accept its canonical element identifier and resolve either form through the shared element-selection contract within the selected model context.

If a selector identifies different elements through its name and identifier interpretations, then the system SHALL reject the selection as ambiguous before executing the operation.

#### Metadata
  * type: requirement

#### Contract References
  * [Workspace Scope Specification](Specifications.md#workspace-scope-specification)
  * [Structure and Addressing in Markdown Documents Contract Specification](Specifications.md#structure-and-addressing-in-markdown-documents-contract-specification)

#### Relations
  * satisfiedBy: [element_selection.rs](../../crates/reqvire-core/src/element_selection.rs)
  * satisfiedBy: [graph_registry.rs](../../crates/reqvire-core/src/graph_registry.rs)
  * specify: [Existing Element Selection](#existing-element-selection)
  * definedBy: [Existing Element Selection Specification](#existing-element-selection-specification)
  * verifiedBy: [Existing Element Selection Verification](../Verifications/ModelStructure/ElementSelectionVerifications.md#existing-element-selection-verification)
---

### Existing Element Selection Specification

Shared implementation obligation for arguments that select existing Reqvire model elements.

#### Details

**Applicability and reuse**
- Every argument that currently accepts an existing model element name in a CLI command, MCP tool, or their shared core operation MUST also accept the canonical element identifier returned by Reqvire. This applies to positional arguments, named arguments, optional selectors and every member of a selector list, including arguments currently named `element_name`, `name`, `from`, `source`, `target`, `sources`, `old_target` or `new_target` when their declared role selects an existing element.
- Canonical element identifiers use the effective workspace-relative source path plus element fragment, such as `system-model/Requirements.md#authentication-requirement`. They are location-based references and can change after moving or renaming an element; this contract does not create a persistent identity or alias service.
- Requirements implementing an affected operation MUST bind this contract as an implementation obligation. A requirement that only documents or consumes the operation's output uses a Contract Reference instead. A requirement with separate content dependencies and selection obligations MAY express selection through a child requirement that binds this contract; it MUST NOT mix Contract Bindings and Contract References in one element.
- Core owns the shared resolution semantics. CLI and MCP adapters MUST delegate selection to that resolver and retain operation-specific validation and transport wrapping. No adapter-local fallback may select a different element.
- Literal-name registration and uniqueness checks MUST remain distinct from existing-element selection. Reusing the resolver MUST NOT reinterpret authored headings or replacement names inside low-level name lookup or validation helpers.

**Resolution and ambiguity**
- Resolve against the operation's validated model snapshot and selected effective workspace or MCP worktree context. A name or identifier MUST NOT select an element from another context, another worker, or an unaccepted physical file.
- Evaluate exact-name and canonical-identifier matches without fuzzy matching, case-insensitive name matching, bare-fragment guessing or searching other workspaces. Preserve an argument's established surrounding-whitespace handling and existing explicitly supported identifier normalization.
- A unique name match or identifier match selects that element. When both interpretations identify the same element, select it once. When they identify different elements, return an ambiguity diagnostic containing the input, argument and both canonical candidates; do not give either interpretation silent precedence.
- Unknown selectors MUST produce a not-found diagnostic rather than default traversal, whole-model coverage, creation of a replacement element, or fallback to another context.
- Apply the operation's permitted element types after resolution. Name and identifier forms MUST have identical type restrictions, ownership constraints, membership, evidence, validation and mutation behavior. Resolution MUST NOT bypass capability-only coverage scope, supported collect roots, requirement/contract endpoint restrictions, or merge/relink compatibility rules.
- Resolve all selectors for a multi-element operation against the same pre-operation snapshot before applying any candidate changes. Detect duplicate and self endpoints by resolved identity, including mixed name/identifier aliases, under the operation's existing duplicate and self-operation rules.
- Existing interfaces with explicitly identifier-only fields retain that domain. Existing name-based element selector fields gain canonical identifier support. Preserve mutually exclusive selector rules; where multiple selector fields are permitted, require them to identify the same element and reject contradictory selections rather than silently choosing a field.

**Included operation arguments**

| Operation family | Existing-element selectors governed by this contract | Other inputs retain their owning contracts |
|---|---|---|
| Model and submodels reports | `from` traversal or scope root | Traversal direction, type filters and omitted-scope defaults |
| Coverage | `from` capability root or subtree | Coverage classification, membership and evidence invariance |
| Collection | Starting capability, requirement, ontology, semantic query, concept scheme or concept | Direction and supported root types |
| Element and native concept reads | Existing source-element name or identifier selectors | Explicit semantic IRI selectors and native concept type restrictions |
| Managed query artifacts | Existing semantic-query source selected through a name-based selector | Explicit query IRIs, namespace filters, query text and artifact paths |
| Delete, move and rename | Existing subject element | Destination file, placement and literal `new_name` |
| Merge | Target and every source element | Type compatibility and content transformation |
| Link, unlink and relink | Element-valued source and target arguments, including old and replacement targets | Relation names and supported resource/evidence target domains |
| Contract Binding and Contract Reference mutations | Source requirement and existing contract target selectors | Contract ownership, scope, normalization and dependency rules |

**Excluded input roles and mixed domains**
- New element names, Markdown heading text, rename `new_name`, relation names, and other authored literals MUST remain literal values. Creating or overriding an element from authored Markdown does not reinterpret its heading as a selector.
- Name regexes, globs and other search filters, including `filter_name` and `filter_id`, MUST retain their declared filtering semantics. A filter does not become a single-element selector merely because it contains a model name or identifier.
- File/folder paths, asset paths, URLs, semantic IRIs, CURIEs, namespace selectors, Git refs, worktree IDs and query text MUST retain their existing domains and resolution rules.
- Mixed-domain relation or contract arguments MUST preserve their operation's existing domain detection and path/URL normalization. Apply this contract within the existing-element branch without intercepting a supported file, evidence URL or semantic IRI. Generic element selection adds no new filesystem target or fragment-only syntax.

**Compatibility, diagnostics and side effects**
- Preserve existing CLI command names, flag names, positional order, MCP tool names, argument keys and response field names. CLI help and MCP argument descriptions MUST advertise name-or-identifier support for affected selectors and the remaining operation-specific type restrictions.
- Successful name and identifier selections MUST expose the same canonical selected element and existing result/evidence fields. Transport-specific envelopes and context/revision metadata remain governed by their interface contracts.
- Selection failures MUST identify the operation, argument, supplied value, cause and relevant context. Report missing, ambiguous, wrong-type and contradictory selections distinctly. CLI fails with a nonzero status; schema-valid MCP calls use the existing structured tool-error envelope.
- Selection and type validation MUST finish before candidate mutation, file persistence, index updates, commits or publication. Rejection and dry-run MUST preserve the operation's existing accepted-state and no-side-effect guarantees. This contract does not enable writes or clear ownership/recovery restrictions.

#### Metadata
  * type: specification

#### Relations
  * define: [Existing Element Reference Resolution](#existing-element-reference-resolution)
---
