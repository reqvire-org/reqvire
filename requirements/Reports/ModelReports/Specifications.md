# Elements

### Collect Content Specification

Technical specification for content collection from capability, requirement, and ontology context.

#### Details
**Input Validation:**
- Element name is required positional argument
- Element must exist in the model
- Element must be a capability, requirement, or ontology element
- Error with non-zero exit if element not found or invalid type

**Traversal Rules:**
- Start from specified capability, requirement, or ontology element
- When direction is UPSTREAM from a requirement:
 - Traverse requirement `derivedFrom` parents, cross to the owning capability through `specify` or inherited ownership, then traverse capability `derivedFrom` parents
- When direction is UPSTREAM from a capability:
 - Traverse capability `derivedFrom` parents only
- When direction is UPSTREAM from an ontology:
 - Traverse ontology `derivedFrom` parents only
- When direction is DOWNSTREAM from a requirement:
 - Traverse child requirements through `derive` only
- When direction is DOWNSTREAM from a capability:
 - Traverse child capabilities through `derive`, requirements through `specifiedBy`, and requirement descendants through `derive`
- When direction is DOWNSTREAM from an ontology:
 - Traverse child ontology elements through `derive` and include semantic contracts that use each reachable ontology element through `use`/`usedBy`
- Include the starting element in output

**Content Collection:**
- Collect element content field (main body text including Details section)
- For each refinedBy target:
 - ElementIdentifier (refinement element): Include element's content
 - FilePath pointing to .md file: Read and include file content
 - FilePath pointing to other file types: Include as markdown link
- For each attachment:
 - FilePath pointing to .md file: Read and include file content
 - FilePath pointing to other file types: Include as markdown link
 - ElementIdentifier: Include referenced element's content
- Skip external URL attachments

**Output Ordering:**
- Flat list structure (no nesting)
- When direction is UPSTREAM: ancestors first (depth 0 = root), then starting element
- When direction is DOWNSTREAM: starting element first (depth 0), then descendants at increasing depth
- Same-depth elements sorted alphabetically by name or file path

**Error Handling:**
- Element not found: Error with message
- Element not a capability, requirement, or ontology type: Error with message
- Attachment file not found: Warning, continue with other content
- Circular reference: Detect and break cycle

#### Metadata
  * type: specification

#### Relations
  * refine: [Collect Capability and Requirement Context](ReportingRequirements.md#collect-capability-and-requirement-context)
---

### Collect Output Format Specification

Output format specification for collect command text and JSON modes.

#### Details
**Text Format:**
Each collected content block followed by source citation and separator:

```
[Content from element or attachment]

— Source: [Element Name](file.md#element-id)


```

**Citation Formats:**
| Source Type | Citation Format |
|-------------|-----------------|
| Element | `— Source: [Element Name](file.md#element-id)` |
| Refinement Element (via refinedBy) | `— Source: [Refinement Name](file.md#refinement-id) refining [Element Name](file.md#element-id)` |
| Attachment Element | `— Source: [Refinement Name](file.md#refinement-id) attached to [Element Name](file.md#element-id)` |

**JSON Format:**
```json
{
 "starting_element": "file.md#element-id",
 "items": [
 {
 "name": "Element Name",
 "identifier": "file.md#element-id",
 "file_path": "path/to/file.md",
 "element_type": "requirement",
 "content": "The collected content...",
 "depth": 0,
 "source_type": "element"
 },
 {
 "name": "Attached Refinement",
 "identifier": "Refinements.md#deterministic-output-specification",
 "file_path": "Refinements.md",
 "element_type": "specification",
 "content": "Technical refinement details...",
 "depth": 0,
 "source_type": "attachment_element",
 "attached_to": "file.md#element-id"
 }
 ],
 "metadata": {
 "element_count": 5,
 "refinement_count": 3,
 "attachment_count": 1,
 "total_items": 9
 }
}
```

**Source Type Values:**
Collect source type vocabulary is defined by the Reqvire report ontology.

#### Metadata
  * type: specification
---

### Color Scheme Specification

Color coding for terminal and CLI text output.

#### Details
Terminal colors are optional presentation hints for human-readable CLI output:

| Color | Meaning | Usage |
|-------|---------|-------|
| Red | Error | Validation errors, failed operations |
| Yellow | Warning | Lint issues needing review, deprecations |
| Green | Success | Added content, passed checks |
| Cyan | Info | Element names, identifiers |
| White/Default | Normal | Regular content |

Explorer route colors, element glyphs, graph colors, Mermaid rendering colors, and browser interaction states are WebInterface presentation contracts and are specified by the Explorer design-system and WebInterface rendering specifications.

#### Metadata
  * type: specification
---

### Comma-Separated Type Filter Parsing Refinement Specification

#### Details
`--filter-type` parsing behavior:
- Splits input by comma delimiter.
- Trims whitespace around each token.
- Normalizes type tokens to lowercase for case-insensitive matching.
- Validates each parsed type against supported element types.
- Reports clear errors for invalid type tokens.

Filtering behavior:
- Matches elements when any parsed type matches (OR semantics).
- Supports custom type syntax using `other-TYPENAME`.
- Preserves single-type query behavior as a compatible subset.

#### Metadata
  * type: specification

#### Relations
  * refine: [Comma-Separated Type Filter Parsing](ReportingRequirements.md#comma-separated-type-filter-parsing)
---

### Containment View Report Refinement Specification

#### Details
The containment report and Model containment modes show the physical organization of the model:
- Root folder → Subfolders → Files → Elements
- Sections skipped (elements shown directly under files)

The system is expected to generate reports in multiple formats:
- Mermaid diagrams for visualization
- JSON for programmatic access
- Project Store data for WebInterface Explorer integration

The system is expected to include design documents:
- Files in DesignDocuments folders displayed alongside specifications
- Design documents visually distinguished from specification elements
- Clickable navigation to document files

Explorer presentation, layout, mode controls, graph rendering, and shell integration are owned by the WebInterface specifications. Functional Output owns the deterministic report and Project Store data consumed by those views.

#### Metadata
  * type: specification
---

### Deterministic Output Specification

Technical specification for ensuring deterministic, reproducible output across all report generation operations.

#### Details
All report outputs are expected to use deterministic ordering to enable reliable testing, version control, and reproducible builds.

**Ordering Rules:**

1. **Element Ordering**: Elements is expected to be sorted by identifier before iteration to ensure consistent processing order across all operations
2. **Relation Ordering**: Relations within each element is expected to be sorted by relation type name and then by target identifier before rendering
3. **Section Ordering**: Sections within files is expected to be sorted alphabetically when order is not semantically significant
4. **File Ordering**: Files within folders is expected to be sorted alphabetically

**Benefits of Determinism:**
- Running the same operation multiple times produces byte-identical output
- Automated tests can reliably compare expected and actual outputs using simple diff tools without special normalization
- Version control diffs are meaningful and reflect actual changes rather than random ordering variations
- Continuous integration pipelines produce consistent, reproducible results

**Applies to:**
- Model summary reporting (text and JSON formats)
- Verification tracing (upward traceability trees from verifications to requirements)
- Coverage reporting (verification coverage analysis)
- Change impact analysis (reports showing propagation of changes)
- Validation reporting (model validation error reports)
- Linting (model quality issue reports)

#### Metadata
  * type: specification
---

### Diagram Relation Filtering Specification

Technical specification for relation filtering in diagram generation to render only forward relations while ensuring complete element hierarchy representation.

#### Details
**Auto-Generated Diagram Identification:**
The system is expected to embed a unique identification marker "REQVIRE-AUTOGENERATED-DIAGRAM" as a comment within all auto-generated mermaid diagrams to distinguish them from user-created diagrams.

The marker must be:
- Embedded as a mermaid comment line using the `%% REQVIRE-AUTOGENERATED-DIAGRAM` format
- Present in every auto-generated diagram
- Not present in user-created custom diagrams

This enables the system to:
- Reliably identify auto-generated diagrams regardless of their location in documents
- Support mixed documents containing both auto-generated and custom diagrams

**Diagram Relation Filtering Rules:**
When generating diagrams, the system is expected to apply the following relation filtering rules:

1. **Diagram Relation Filtering**: Only relations specified in the DIAGRAM_RELATIONS list is expected to be rendered to prevent duplicate arrows representing the same logical relationship
2. **Complete Hierarchy Inclusion**: Start with file-local parent requirements but include all children even if they are defined outside of the file
3. **List-Based Rendering**: Relations is expected to be rendered according to the DIAGRAM_RELATIONS list which defines which relation from each opposite pair should be shown

**Filtering Benefits:**
The filtering ensures that:
- Bidirectional relationships (e.g., `derivedFrom`/`derive`) appear as single arrows using the relation specified in DIAGRAM_RELATIONS
- Hierarchical context is preserved by starting from local parents and showing all derived children regardless of file location
- Diagram readability is maintained while accurately representing the complete model structure

#### Metadata
  * type: specification
---

### File Diagram Attachment Display Refinement Specification

#### Details
File-diagram attachment rendering behavior:
- Renders attachments below the element name using `<br/>` separators.
- Prefixes each attachment entry with `📎`.
- Displays attached refinement element names.
- Emits clickable attachment links to refinement identifier targets.
- Renders each attachment on its own line.
- For elements without attachments, renders only the element name.

Label format example:
`Element Name<br/>📎 Deterministic Output Specification<br/>📎 Rate Limits`

#### Metadata
  * type: specification

#### Relations
  * refine: [File Diagram Attachment Display](DiagramGeneration.md#file-diagram-attachment-display)
---

### Flexible Search Type Filtering Refinement Specification

#### Details
Users is expected to be able to specify multiple element types in a single search operation using comma-separated values (e.g., `requirement,test-verification,behavior`).

This capability enables:
- Searching across related type categories (capabilities, requirements, verification types, and refinement types)
- Building complex queries without multiple search invocations
- Improved workflow efficiency for model analysis and reporting

#### Metadata
  * type: specification
---

### Implementation Coverage Output Structure Specification

Technical specification for implementation coverage report output structure.

#### Details
**Output Requirements (Text):**
- Summary subsection with totals, covered/uncovered counts, and percentage.
- Covered requirements list grouped by file with source classification and evidence identifiers.
- Uncovered requirements list grouped by file.

**Output Requirements (JSON):**
- `summary` includes:
 - `total_requirements_in_scope`
 - `covered_requirements`
 - `uncovered_requirements`
 - `implementation_coverage_percentage`
 - `coverage_sources` object keyed by implementation coverage source names defined by the Reqvire report ontology
- `covered_requirements` contains per-element:
 - `identifier`, `name`
 - `coverage_source`
 - `evidence` (identifier list)
- `uncovered_requirements` contains per-element:
 - `identifier`, `name`
- Coverage percentage values in summary is expected to be emitted with at most 2 decimal places.

#### Metadata
  * type: specification

#### Relations
  * refine: [Requirement Implementation Coverage Report](ReportingRequirements.md#requirement-implementation-coverage-report)
---

### Interactive Mermaid Diagram Node Behavior Refinement Specification

#### Details
Clickable mermaid diagrams links by default must use relative links to the git repository.

CLI flag options must be provided that can change default behavior to use stable github repository links:
* diagrams click links are not working on Github if not using stable github repository links
* from another side that pollutes PR diffs thus choice must be given to the user
* Commands that generate diagrams (`generate-diagrams`, `serve`) must expose `--links-with-blobs` CLI flag for that purpose
* The flag defaults to `false` (use relative paths)

When generating diagram node links and when `--links-with-blobs` flag is set to `true`, the system is expected to:
- Use stable git repository links (`{repository-url}/blob/{commit-hash}/{file-path}`) when git repository information is available
- Fallback to relative markdown links when git repository information is not available
- Use the current commit hash to ensure links remain stable even as the repository evolves
- Match the same link format used in traceability matrices and change impact reports
- Preserve interactive behavior across all generated diagrams

The `traces` command is expected to always use relative paths (hardcoded to `false`, no flag needed).

The `change-impact` command is expected to continue to use GitHub blob URLs by default (unchanged behavior).

#### Metadata
  * type: specification
---

### JSON Element Size Estimate Output Specification

JSON report outputs are expected to preserve element size-estimate metadata when the parsed model contains it.

#### Details
- JSON output serializers that include model elements shall include the element `size_estimate` field when present.
- JSON output serializers shall omit `size_estimate` when the model was not built with size estimates enabled.
- Text, Markdown, Mermaid, D3, HTML, and other non-JSON outputs shall not display size-estimate metadata.
- The initial output scope is element payloads in `model --json` and equivalent structured model evidence.
- Nested relation element targets in model JSON shall preserve their own `size_estimate` fields when present.
- Aggregate report-level `size_estimate` records are deferred and shall not be added by this specification.

#### Metadata
  * type: specification

#### Relations
  * refine: [JSON Element Size Estimate Exposure](ReportingRequirements.md#json-element-size-estimate-exposure)
---

### JSON Output Structure

Standard JSON output structure for CLI commands that support the `--json` flag.

#### Details
JSON output conventions:

**Structure:**
- Root object with semantic field names (not abbreviated)
- Arrays for collections (elements, relations, files)
- Nested objects for hierarchical data
- Consistent field naming using snake_case

**Common Fields:**
- `identifier`: Full element identifier (file#fragment)
- `name`: Display name of element
- `type`: Element type string
- `file_path`: Relative path from git root
- `relations`: Array of relation objects with `type` and `target` fields
- `attachments`: Array of refinement element identifier strings

**Error Handling:**
- Error responses include `error` field with message
- Successful responses omit error field entirely
- Exit code accompanies JSON (0=success, non-zero=error)

**File Output:**
- When `--output <FILE>` is provided alongside `--json`, write JSON to file instead of stdout
- Print confirmation message to stdout: `✅ Output saved to <filepath>`
- `--output` without `--json` is an error

#### Metadata
  * type: specification

#### Relations
  * refine: [Model Reports](ReportingRequirements.md#model-reports)
---

### Markdown Report Style Specification

Style guidelines for markdown text report output (model, coverage, traces, containment commands).

#### Details
**Document Structure:**
- Title as H1 header
- Major sections as H2 headers
- Subsections as H3 headers
- Element listings as bullet points or tables

**Formatting Conventions:**
- Element names in backticks: `Element Name`
- File paths in backticks: `path/to/file.md`
- Identifiers in backticks: `file.md#element-id`
- Relation types in bold: **derivedFrom**, **verifiedBy**
- Counts and percentages: `15 (75%)`

**Tables:**
- Use markdown tables for structured data
- Align columns appropriately (left for text, right for numbers)
- Include header row with separator

**Lists:**
- Hierarchical bullet lists for tree structures
- Numbered lists for sequential steps
- Indentation shows nesting (2 spaces per level)

**Code Blocks:**
- Mermaid diagrams in ```mermaid blocks
- JSON output in ```json blocks

#### Metadata
  * type: specification
---

### Mermaid Diagram Generation Specification

Technical specification for Mermaid diagram generation approach and structure.

#### Details
**Diagram Generation Approach:**
Diagram generation follows a file-based approach:
- One diagram is generated per specification file
- The diagram shows all elements in the file and their relationships
- External related resources are displayed as linked boxes to the actual resource

**Diagram Styling:**
The system is expected to implement diagram styling including:
- Containment structure with nested subgraphs for physical organization
- Element type-specific CSS classes for visual differentiation
- Relation-specific line styles and colors
- Interactive highlighting on hover
- Consistent background and border styling

**Model Structure Visualization:**
The system is expected to provide visualization of the complete model structure showing an element-centric view with nested relations:
- Display elements with their properties (identifier, name, type, file location)
- Show relations nested inside elements with full target details
- Support recursive nesting for element-to-element relations
- Handle file path and external URL relations
- Provide metadata about total elements and relations
- Use consistent visual styling with mermaid diagrams showing hash-based node identifiers

#### Metadata
  * type: specification
---

### Mermaid Interactive Capabilities Specification

Technical specification for interactive Mermaid diagram navigation and filtering capabilities.

#### Details
**Model Navigation and Filtering:**
Users is expected to be able to generate and view model structure diagrams from any starting point:
- Default view shows ontology roots and capability roots according to model hierarchy traversal rules
- Filter from specific element using --from flag
- Generate complete model structure with nested relations showing element details recursively
- Mermaid diagrams display all nested relations recursively

**Diagram Output:**
The system is expected to generate Mermaid diagrams embedded in markdown format for visual representation of the model structure. When pure Mermaid output is requested, the system emits Mermaid text only, without Markdown prose or fenced code blocks.

**Interactive Capabilities:**
The visualization helps users:
- Understand the model's logical structure
- Navigate relationships between elements
- Explore the model from any starting point
- Filter and focus on specific subtrees of the model

#### Metadata
  * type: specification
---

### Model Diagram Output Formats Refinement Specification

#### Details
Model output format rules:
- Markdown format includes embedded Mermaid diagram with model structure.
- Pure Mermaid format includes only Mermaid flowchart text with no Markdown wrapper.
- Markdown shows hierarchical structure using containment subgraphs (folders > files > elements).
- Mermaid diagrams use folder and file subgraphs to visually group elements by physical location.
- JSON format uses structured data with folders, files, sections, elements, relations, and attachments.
- Both formats represent the same filtered or complete model data.
- Element attachments are included as an array of refinement element identifier strings in both formats.

#### Metadata
  * type: specification

#### Relations
  * refine: [Model Diagram Output Formats](ReportingRequirements.md#model-diagram-output-formats)
---

### Ontology Collection Output Specification

The ontology collection output defines how Reqvire exposes ontology and SHACL content for serve-time Explorer rendering and downstream semantic tooling.

#### Details
The output shall:
- Collect all ontology `#### Ontology` and semantic-contract `#### Shapes` fenced Turtle blocks from the graph registry.
- Use the reusable semantic index built for ontology and semantic-contract validation so Turtle parsing is performed once per block.
- Emit RDF/Turtle by default, preserving collected block content with source comments.
- Emit JSON-LD when requested by the CLI `--jsonld` option.
- Support full semantic model export when requested by the CLI `--full` option or MCP `full: true` argument.
- In full semantic model export mode, append RDF triples for Reqvire model context:
  - all parsed capability, requirement, ontology, semantic-contract, verification, and refinement elements
  - element id, identifier, name, type, file path, and source line
  - internal model relations, including `derive`, `derivedFrom`, `specify`, `specifiedBy`, `refine`, `refinedBy`, `verify`, `verifiedBy`, `satisfy`, `satisfiedBy`, and `trace`
  - element attachments, including capability-to-ontology attachments and requirement-to-contract attachments
  - concept references from model elements to ontology terms
  - ontology term declaration edges from ontology elements to declared terms
  - semantic-contract shape reference edges from semantic contracts to referenced ontology terms
  - generated ontology projection facts for direct-authored OWL/RDFS/SHACL constructs
- Include source element identifier, source name, file path, section kind, and line number in the semantic index used by rendering and semantic output.
- Avoid requiring a persistent RDF store for this collection path; full semantic output uses the existing in-memory RDF projection over the graph registry and semantic index, extended with a generated ontology construct subprojection. Persistent RDF stores, SPARQL-backed search, general query execution, query output, and inferred reasoning are reserved for later query support.

**Explorer Serve:**
- Explorer serve shall include `ontologies.ttl`.
- Explorer serve shall include ontology collection and projection data in the Project Store so the WebInterface Ontologies route can render the ontology model viewer, source citations, search, modal evidence, and `ontologies.ttl` download action from one semantic projection.
- Browser presentation details such as canvas layout, visibility controls, legends, graph colors, modal sections, and construct rendering are owned by the WebInterface Ontologies view specifications.

#### Metadata
  * type: specification

#### Relations
  * refine: [Ontology and Shapes Collection](ReportingRequirements.md#ontology-and-shapes-collection)
---

### Ontology Projection Subgraph Materialization Specification

#### Details
Projection subgraph generation behavior:
- Extend the existing full semantic export in-memory RDF projection with a generated `reqvire:OntologyProjectionGraph` subprojection after RDF/Turtle and SHACL parsing has produced semantic-index quads.
- Attach generated projection data to `SemanticIndex` as structured Rust data, not as a renderer-local object. The `SemanticIndex` projection data is the authoritative in-process source for full Turtle output, full JSON-LD output, and Ontologies Explorer rendering.
- Store generated construct facts in the existing semantic export context as in-memory RDF statements derived from `SemanticIndex` projection data. Generated facts are not written back to authored Markdown ontology or semantic-contract blocks.
- Keep projection data deterministic and serializable from `SemanticIndex` without reparsing raw Turtle in the Ontologies renderer.
- Use stable generated IRIs or blank-node identifiers derived from canonical source evidence and construct membership so repeated exports remain deterministic.
- Materialize one `reqvire:OntologyConstructProjection` record per projection pass or construct family and one or more `reqvire:OntologyConstruct` records for extracted constructs.
- Record `reqvire:projectionDerivationMode "direct-authored"` for facts derived only from authored quads without reasoning.
- Record `reqvire:constructSourceBlock`, source element metadata, source line, construct subject, construct object, construct property, construct member, and `reqvire:constructSequenceIndex` where order matters.
- Attach `reqvire:OntologySymbol` facts for rendered symbols with `reqvire:symbolConceptName`, `reqvire:rawUnicodeCodePoint`, and `reqvire:renderedUnicodeCharacter`.

Direct-authored construct families:
- `rdfs:domain` and `rdfs:range` become property-domain and property-range constructs.
- `rdfs:subClassOf` becomes inclusion constructs using `U+2286` / `⊆` unless strictness is separately proven.
- `rdf:type` assertions for named individuals or typed resources may become membership constructs using `U+2208` / `∈`.
- `owl:disjointWith` becomes disjointness constructs using `U+27C2` / `⟂`.
- `owl:equivalentClass`, `owl:equivalentProperty`, and `owl:sameAs` become equivalence-group constructs using stable connected components.
- `owl:inverseOf` becomes inverse-property constructs using `U+27F2` / `⟲`.
- `owl:propertyChainAxiom` RDF lists become ordered property-chain constructs preserving list member order.
- `rdf:type` declarations of OWL property characteristics become property-characteristic constructs for functional, inverse-functional, symmetric, asymmetric, reflexive, irreflexive, and transitive properties.
- `owl:Restriction` with `owl:onProperty`, `owl:allValuesFrom`, `owl:someValuesFrom`, cardinality predicates, `owl:hasValue`, or similar authored restriction predicates becomes restriction constructs using the matching symbol vocabulary.
- `owl:intersectionOf`, `owl:unionOf`, and `owl:complementOf` RDF list or expression structures become class-expression constructs. Set difference uses `U+2216` / `∖` only when represented by an explicit supported class-expression pattern.
- SHACL node shapes and property shapes become shape-overlay constructs over their target classes, paths, datatypes, class constraints, node kinds, cardinality constraints, and allowed-value lists.
- SHACL node-shape target classes plus property-shape paths and facets become viewer-facing slot/facet records attached to the target class and, when present, the named property node. On target classes these records define normalized class slots; on named properties they define source-backed usages of that property as a slot by each target class.
- Class-expression projection records shall preserve list members in RDF list order and expose usage evidence so consumers can distinguish the expression itself from the property, subclass, or restriction construct that references it.

Consumer behavior:
- `reqvire ontologies --full` and `reqvire ontologies --full --jsonld` include generated ontology projection subgraph facts.
- Default `reqvire ontologies` and served `ontologies.ttl` include generated ontology document declarations plus authored ontology and SHACL blocks, without generated ontology projection subgraph facts. Generated ontology document declarations use the resolved `ontology_base` as the `owl:Ontology` IRI; ontology elements in the same base are contributors to that document.
- The Ontologies SPA route must build from the same generated projection facts used by full semantic export instead of maintaining a separate route-local construct model.
- The projection subgraph is a reusable semantic data product for later SPARQL-backed search, semantic validation, or inferred-materialization work, but those later features require their own execution requirements.

#### Metadata
  * type: specification

#### Relations
  * refine: [Ontology Projection Subgraph Materialization](ReportingRequirements.md#ontology-projection-subgraph-materialization)
---

### Report Command Catalog Specification

Report command catalog for model reporting capabilities.

#### Details
Report commands:
- `collect`: capability-or-requirement context collection, with upstream or downstream traversal and source citations.
- `coverage`: requirement verification coverage, evidence-backed verification satisfaction, implementation coverage, and capability coverage rollup.
- `traces`: verification-to-capability-root traceability trees.
- `search`: model element search with relations, attachments, semantic-contract fields, and effective governance metadata.
- `submodels`: independent capability-rooted subgraphs, cross-submodel couplings, and summary totals.
- `resources`: relation file targets and attachment identifier targets grouped by resource.
- `ontologies`: collected ontology `Ontology` and semantic-contract `Shapes` Turtle blocks.
- `model`: logical model graph traversal with optional direction and type filtering.
- `lint`: model quality findings grouped for auto-fix or manual review.
- `change-impact`: changed, added, removed, relocated, and impacted model elements with propagation evidence.
- `containment`: folder, file, and element containment.
- `serve`: local Explorer server.

#### Metadata
  * type: specification

#### Relations
  * refine: [Model Reports](ReportingRequirements.md#model-reports)
---

### Requirement Governance Metadata JSON Output Specification

Structured model evidence outputs are expected to expose effective requirement governance metadata with stable source information.

#### Details
- Full search JSON output and equivalent MCP structured search results shall include `governance_metadata` for governance-bearing element payloads (`capability` and `requirement`).
- Non-governance-bearing element payloads shall omit `governance_metadata`.
- The `governance_metadata` object shall contain `status`, `priority`, `risk`, and `owner` entries.
- Each entry shall contain:
 - `value`: the effective value after explicit, inherited, and default resolution.
 - `source`: one of `explicit`, `inherited`, or `default`.
 - `source_identifier`: the requirement ancestor identifier that supplied the value when `source` is `inherited`.
- `source_identifier` shall be omitted when `source` is `explicit` or `default`.
- `owner.value` shall be a string and shall be empty when the effective owner is unassigned.
- Enum values and defaults shall match the Requirement Governance Metadata Specification.
- Full search JSON output shall include global governance summary counters under `global_counters.total_governance_metadata`.
- Governance summary counters shall be computed from effective metadata for matched governance-bearing elements only.
- Governance summary counters shall include `status`, `priority`, `risk`, and `owner` maps.
- Status, priority, and risk summary maps shall include all accepted enum values with zero counts when no matched requirement has that value.
- Owner summary maps shall count effective owner values and represent empty/unassigned owner as `unassigned`.
- Full text search output shall render equivalent governance summary counts in the summary section.
- Short search output shall omit global counters, including governance summary counters.

Example:

```json
{
 "governance_metadata": {
 "status": {
 "value": "approved",
 "source": "inherited",
 "source_identifier": "requirements/System.md#parent-requirement"
 },
 "priority": {
 "value": "high",
 "source": "explicit"
 },
 "risk": {
 "value": "low",
 "source": "default"
 },
 "owner": {
 "value": "Platform Team",
 "source": "inherited",
 "source_identifier": "requirements/System.md#parent-requirement"
 }
 }
}
```

#### Metadata
  * type: specification

#### Relations
  * refine: [Search Report Generator](ReportingRequirements.md#search-report-generator)
---

### Requirement Implementation Coverage Logic Specification

Technical specification for requirement implementation coverage classification logic.

#### Details
Implementation coverage source vocabulary is defined by the Reqvire report ontology.

Implementation coverage scope includes only elements of type `requirement`. Elements of type `capability` are excluded from direct implementation coverage and receive implementation coverage through capability roll-up.

The report shall classify each requirement using the semantic coverage source vocabulary and the available `satisfiedBy`, `refinedBy`, attachment, and child requirement evidence.

#### Metadata
  * type: specification

#### Relations
  * refine: [Requirement Implementation Coverage Report](ReportingRequirements.md#requirement-implementation-coverage-report)
---

### Requirement Submodels Report Specification

Technical specification for submodels report structure and deterministic ordering.

#### Details
**Submodel Boundary Principle:**
Canonical submodel, capability-root submodel, scoped submodel, and cross-submodel coupling concepts are defined by the Reqvire report ontology.

**Refactor Rule:**
When a relation crosses intended submodel boundaries, either:
1. Move/reparent to restore hierarchical ownership, or
2. Replace cross-boundary hierarchy links with attachment-based refinement contracts.

**Refactor Procedure:**
Apply boundary refactoring recursively, top-down:
1. Start from each capability root and inspect its first-level capability and requirement children.
2. For each first-level child, inspect all direct children and relation edges.
3. Continue recursively for each descendant branch until leaf requirements.
4. At each level, enforce:
 - hierarchical relations remain internal to that branch/submodel,
 - cross-branch dependencies are attachment contracts.
5. If a cross-boundary hierarchical relation is found, either:
 - move/reparent to restore ownership, or
 - replace with attachment-based refinement contract.
6. Re-run validation and submodel analysis after each boundary slice before continuing recursion.

**Internal Sub-Boundaries:**
A submodel may contain internal sub-boundaries (nested domains) with separate ownership and lifecycle.
Cross-internal-boundary dependencies should be modeled as explicit attachment contracts when they represent contractual dependency, not hierarchical ownership.

**Submodel Resolution Rules:**
- A submodel root is a capability element with no capability parent relation.
- Capability submodel membership is resolved through capability `derivedFrom`/`derive` hierarchy.
- Requirement membership is resolved through `specify`/`specifiedBy` and requirement hierarchy.
- Each requirement is expected to be assigned to one resolved capability root for report grouping.
- For full report generation, submodel entries are capability roots, not requirement roots.
- For capability-scoped report generation, the selected capability is reported as the scoped submodel and its requirement count includes requirements that specify the selected capability or descendant capabilities, including descendant requirements.
- For requirement-scoped report generation, the selected requirement defines a scope boundary; it is not itself reported as a submodel entry.

**Report Content:**
- List all discovered submodels with:
 - root identifier and display name
 - root element type
 - requirement count in that submodel
- List cross-submodel requirement couplings:
 - source requirement
 - relation type
 - target requirement
 - source and target root context

**Cross-Submodel Coupling Scope:**
- Include requirement-to-requirement relations where source and target belong to different capability roots.
- Use explicit relation targets only (no inferred transitive links).
- For scope-scoped report generation, include only couplings where the source or target requirement is inside the selected scope.

**Scope Resolution Rules:**
- In scope mode, select a capability or requirement by name.
- Capability scope computes the selected capability subtree through capability hierarchy, then includes requirements that specify those capabilities and their requirement descendants. The selected capability appears as the scoped submodel entry.
- Requirement scope computes the selected requirement subtree through requirement hierarchy. The selected requirement is excluded from the `submodels` output, and first-level child requirement branches are reported as scoped requirement submodels.
- If a selected requirement boundary has no children in the induced subtree, `submodels` output is empty.

**Output Formats:**
*Text/Markdown Format:*
- Human-readable sectioned report with deterministic ordering
- Introductory text states that submodels are independent capability-rooted subgraphs resolved through capability ownership relations.
- Markdown links for source/target/root identifiers
- Summary section with totals
- When filtered by scope:
 - output submodels discovered within selected scope
 - output the selected capability when filtering by capability
 - do not output the selected requirement when filtering by requirement
 - summary counts are computed from filtered output only

*JSON Format:*
- Structured arrays for `submodels` and `cross_submodel_couplings`
- Summary object with deterministic count fields
- Stable sort order for reproducible automation output
- When filtered by scope:
 - JSON includes only filtered-scope submodel data and relevant couplings
 - selected capability scope appears in `submodels` array
 - selected requirement scope is excluded from `submodels` array

**Summary Section Semantics:**
- Summary reports:
 - `Submodels`: number of scoped top-level submodel roots returned
 - `Requirements`: number of requirements counted across returned scoped submodels
 - `Cross-Submodel Couplings`: number of qualifying couplings for the active scope
- In scoped mode, all summary fields are derived from scoped submodel and coupling sets only.

#### Metadata
  * type: specification
---

### Resources Report Format Specification

Technical specification for resources report structure and output formats.

#### Details
**Report Structure:**
The resources report is expected to consist of two sections:
1. Relations section
2. Attachments section

**Relations Section:**
- Files from InternalPath relation targets (satisfiedBy, trace, etc.)
- Shows relation type and source element for each reference
- Sorted by relation type, then by element identifier
- Each file lists all elements that reference it with their relation types

**Attachments Section:**
- Refinement element identifiers from attachment targets
- Shows source element for each reference
- Sorted by element identifier
- Each refinement identifier lists all elements that attach it

**Output Formats:**

*Text/Markdown Format:*
- Human-readable with markdown links
- Entries listed alphabetically by path (relations) and identifier (attachments)
- Element references shown as clickable markdown links
- Clear section headers separating Relations and Attachments

*JSON Format:*
- Structured data for programmatic use
- Same logical structure as text format
- Includes file paths, element identifiers, relation types
- Suitable for automated processing and integration

**Explorer Serve:**
- Resources report data available in the Explorer as a supporting route or report link, not as a primary left Explorer view link
- Shows complete list of referenced files with element traceability
- Maintains same structure as text/JSON outputs
- Provides clickable navigation between resources and elements

#### Metadata
  * type: specification
---

### SysML Rendering Specification

SysML notation standards for relationship rendering in diagrams.

#### Details
Each relationship type is represented using SysML standard notation with specific arrow direction.

**Derive Relations:**
| Relation | Stereotype | Line Style | Arrow Direction |
|----------|------------|------------|-----------------|
| derive | «deriveReqt» | dashed | Parent → Child (derived) |
| derivedFrom | «deriveReqt» | dashed | Child → Parent (source) |

**Verify Relations:**
| Relation | Stereotype | Line Style | Arrow Direction |
|----------|------------|------------|-----------------|
| verify | «verify» | dashed | Verification → Requirement |
| verifiedBy | «verify» | dashed | Requirement → Verification |

**Satisfy Relations:**
| Relation | Stereotype | Line Style | Arrow Direction |
|----------|------------|------------|-----------------|
| satisfy | «satisfy» | solid | Implementation → Requirement |
| satisfiedBy | «satisfy» | solid | Requirement → Implementation |

**Trace Relations:**
| Relation | Stereotype | Line Style | Arrow Direction |
|----------|------------|------------|-----------------|
| trace | «trace» | dashed | Tracing → Traced (neutral) |

**Arrowhead Style:**
All relation types use open (hollow) arrowheads per SysML specification.

#### Metadata
  * type: specification
---

### Text Output Formatting

Human-readable text output conventions for CLI commands.

#### Details
Default text output (when neither `--json` nor other format flags specified):

**Hierarchical Display:**
- Group elements by file, then by section
- Use indentation to show containment
- Display element name with type indicator

**Element Information:**
- Full element name and identifier
- Element type in brackets: `[requirement]`, `[test-verification]`
- Relations listed with target identifiers
- Attachments listed as refinement element identifiers

**Formatting:**
- Color output when terminal supports it (errors in red, warnings in yellow)
- Git-style diff format for change previews
- Line numbers for file references

**Consistency:**
- Deterministic ordering (alphabetical by identifier)
- Consistent spacing and alignment
- No trailing whitespace

#### Metadata
  * type: specification
---

### Trace Relation Non-Directional Behavior Refinement Specification

#### Details
Canonical trace relation meaning, direction, and change-impact propagation semantics are defined by the Reqvire relation ontology.

The implementation behavior is expected to include:

1. **Circular Dependency Exclusion**:
- Trace relations is expected to not be traversed during circular dependency detection
- The cycle detection algorithm is expected to skip trace relations to prevent false positive cycles
- Trace relations exist solely for traceability and documentation purposes

2. **Non-Propagation Behavior**:
- Changes is expected to not propagate through trace relations
- Trace relations is expected to not be included in change impact analysis
- Impact trees is expected to not traverse trace relation connections

3. **Bidirectional Traceability**:
- Trace relations is expected to provide bidirectional navigational capability
- Users can navigate from source to target and target to source
- Both directions are semantically equivalent for traceability purposes

4. **Validation Behavior**:
- Trace relations is expected to be validated for target existence
- Trace relations is expected to not require type compatibility validation
- Trace relations can connect any element type to any other element type

This ensures that trace relations serve their intended purpose of establishing lightweight traceability connections without creating artificial dependency constraints or participating in architectural validation logic.

#### Metadata
  * type: specification
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
  * refine: [Model Reports](ReportingRequirements.md#model-reports)
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
  * refine: [Verification Coverage Report](ReportingRequirements.md#verification-coverage-report)
---
