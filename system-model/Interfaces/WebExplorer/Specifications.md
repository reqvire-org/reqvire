# Elements

### Component-Based Explorer Architecture Contract Specification

#### Details
The Explorer UI system is expected to be organized into reusable components:

**Shared components:**
- Headerless Explorer shell with persistent vertical `Explorer` edge strip and active-view controls at the top of the expanded left pane
- Page metadata panels for supporting document/report pages
- On-demand view help modal opened from the right vertical tool rail

**Reusable layouts:**
- Base layout for standard pages
- Diagram layout for full-height visualizations

**Page-specific modules:**
- Index/Model Explorer SPA route
- Model view module
- Traces view module
- Ontologies explorer view module
- Supporting SPA routes such as Search, Files, Coverage, and Resources, plus source/specification content routes
- Individual specification pages

Each component is expected to be defined once and reused across SPA view modules to eliminate code duplication.

#### Metadata
  * type: specification
---

### Containment View Contract Bindings Links Contract Specification

#### Details
Model containment-mode contract_bindings rendering behavior:
- For each element with contract_bindings, preserves contract_bindings records in the Project Store data consumed by Model List/Grid and element-detail workflows.
- Uses the shared Explorer element-role and contract-subtype glyph treatment for contract_bindings records that target contract elements, with `contract-bindings-element` retained as data classification only.
- Element-contract_bindings records navigate to the referenced element detail/source route from supported Explorer surfaces.

#### Metadata
  * type: specification

#### Relations
  * define: [Containment View Contract Bindings Links](Capabilities.md#containment-view-contract-bindings-links)
---

### Contract Bindings Link Serving Contract Specification

#### Details
Contract Bindings link behavior in the served Explorer:
- Collects contract_bindings references from `element.contract_bindings` across the model.
- Resolves each contract_bindings as a contract element identifier target.
- Skips duplicate identifier processing when the same contract is referenced by multiple elements.
- Keeps contract-bindings-link evidence available in Project Store data for content routes, graph views, search results, and element modals.

This keeps served Explorer content complete with navigable contract contract_bindings links.

#### Metadata
  * type: specification

#### Relations
  * define: [Contract Bindings Link Serving](Capabilities.md#contract-bindings-link-serving)
---

### Diagram Contract Bindings Display Contract Specification

#### Details
Diagram contract_bindings rendering behavior in Mermaid output:
- Renders contract_bindings links under the element name inside node labels.
- Prefixes each contract_bindings with paperclip icon (`📎`).
- Displays referenced contract element names.
- Does not display full contract_bindings identifier paths in node labels; identifier paths remain link targets or structured report data.
- Produces clickable links to the referenced contract element.
- Uses Mermaid multiline label formatting (`<br/>`).
- Applies the same compact display-name rule to Model, Knowledge Graph, and Traces diagrams so a late-page verification or graph view cannot be widened or clipped by full `file#fragment` contract_bindings identifiers.

Example node:
```
elementId["Element Name<br/>📎 Deterministic Output Specification"]
```

#### Metadata
  * type: specification

#### Relations
  * define: [Diagram Contract Bindings Display](Capabilities.md#diagram-contract-bindings-display)
---

### Explorer Branding Specification

Specification for Reqvire branding elements in the served Explorer.

#### Details
**Logo and Branding:**
- The native Explorer shell is headerless; Reqvire branding is expected to appear in the left Explorer pane or browser document metadata rather than global top chrome.
- A favicon is expected to be included for browser tab identification
- Apple touch icons is expected to be included for mobile device support
- All brand assets are expected to be bundled with the Explorer assets and served from the Explorer asset tree

**Explorer Design:**
The system is expected to design and implement the static Explorer shell with consistent layout, styling, and navigation for browsing the System model.

#### Metadata
  * type: specification
---

### Explorer Color and Type Palette Specification

The Explorer interface owns the visual color, glyph, and badge semantics used by browser-rendered model views.

#### Details
Explorer palette behavior:
- Use semantic design-system tokens as the canonical source for interface surfaces, type colors, graph colors, Mermaid class definitions, badges, and type glyphs.
- Treat primitive color values as token implementation details. Explorer code and specifications must name colors by purpose, such as model canvas, selected row, capability node, requirement node, verification node, ontology node, resource node, and relation edge.
- Use local Geist font assets and warm-neutral Reqvire surfaces for Explorer chrome, source pages, modals, graph canvases, and routed report views.
- Use one role palette across the Model tree, List/Grid rows, Graph mode, Search results, Ontologies, Traces, source content, and element modals.
- Render semantic-contract elements with their own SHACL-profile role token as a plain square with no glyph. Render contract-family subtypes with one shared contract hue and distinct type glyph marks for `source`, `specification`, `constraint`, `behavior`, `state`, and `input-output`.
- Render capability elements as a plain capability-colored square with no dark hub wrapper or inner pip, matching the simple square treatment used by other non-contract element types.
- Render `verification-objective` elements with their own darker green verification-objective role token as a plain square with no glyph, distinct from concrete verification elements that verify requirements or capabilities.
- Render model element concept references inline in element-detail modal prose and source-page prose when the referenced native SKOS concept preferred label, alternative label, or authored reference label appears in the content. Inline concept-reference links use normal text color at rest with a dotted underline, no resting background, no glyph, no badge, and no pill; on hover or focus both text and underline use the standard Explorer link color. Activating the link opens the native concept element modal. The authored `#### Concept References` source subsection must be hidden from regular element modal content rendering and source-page rendering. Concept-reference links must not open ontology-node modals. Evidence-file and artifact targets use the neutral/default role treatment and must not share the concept-reference color.
- Relation and contract_bindings endpoints must use the shared `ElementIcon` marker contract for both element targets and non-element resource/artifact targets; they must not use ad hoc tiny pips with local color mappings.
- In element-detail modal headers, render only the actual element type badge. Type family remains available to design-system components for color/glyph semantics but must not appear as a redundant visible kind badge next to the actual type.
- In compact element rows or cards where an `ElementIcon` already appears next to the element title, any adjacent type badge must be text-only and must not repeat the element marker dot, shape, or glyph.
- Keep color as a secondary cue: every type color must be paired with text, glyph shape, or accessible label.

The Explorer element color contract uses saturated role colors for graph nodes and muted tints for badges and cards:

| Role | Purpose |
|------|---------|
| Capability | Capability roots and capability-owned context |
| Requirement / contract | Requirement obligations and requirement-owned contracts |
| Verification | Verification elements and evidence-backed tests |
| Ontology | Authored ontology vocabulary and ontology-derived terms |
| Concept | SKOS concept nodes used by conceptual taxonomy and concept-reference targets |
| Concept reference | Concept-reference facts and edges from model elements to SKOS concepts |
| Resource | Local implementation, document, or resource targets referenced by the model |
| Other/default | Folders, files, evidence-file artifacts, unresolved external targets, and generic model infrastructure |

Raw hexadecimal values belong in design-system token files or generated assets, not in route-local renderer code. Programmatic renderers must resolve semantic tokens at runtime or build time through the Explorer design-system palette API.

#### Metadata
  * type: specification

#### Relations
  * define: [Web Interface Color Scheme](Capabilities.md#web-interface-color-scheme)
---

### Explorer Design System Styling Contract Specification

#### Details
Explorer design-system styling behavior:
- Uses the local Reqvire Explorer design-system token set, component classes, and compiled application CSS as the only required browser styling contract.
- Ships deterministic `assets/explorer.css` referenced by `index.html`, so the embedded Explorer renders with no remote stylesheet, framework CDN, or runtime CSS compiler.
- Uses local Geist font assets and warm-neutral Reqvire theme tokens for typography, surfaces, borders, shadows, semantic element colors, and graph colors.
- Keeps responsive layout behavior in the local Explorer stylesheet and design-system components rather than requiring route-local utility styling or third-party component themes.
- Keeps iconography, buttons, modals, tree rows, toggles, tabs, breadcrumbs, badges, and graph legends aligned through the shared design-system component and token API.

The design-system styling contract provides:
- Deterministic, offline-capable styling because all CSS and font assets are generated at Explorer build time and served locally.
- One product color and typography system shared by Model, Graph, Ontologies, Traces, Search, source content, resources, and element modals.
- A stable token boundary for programmatic renderers such as Sigma and Mermaid without requiring duplicated route-local color tables.

#### Metadata
  * type: specification

#### Relations
  * define: [Explorer Design System Styling](ExplorerRendering.md#explorer-design-system-styling)
---

### Explorer Mermaid Diagram Style Specification

Browser-rendered Mermaid diagrams must follow the Explorer design-system visual contract while preserving deterministic generated Mermaid text.

#### Details
Explorer Mermaid rendering behavior:
- Generated Mermaid source may use semantic class names for element roles and container roles. The Explorer renderer resolves those classes through design-system tokens before Mermaid rendering.
- Class definitions must use concrete Mermaid-safe colors only after resolving semantic tokens; renderer code must not hardcode a duplicate color table.
- Folder and file subgraphs show physical containment, while element nodes use role-specific classes and source links.
- Mermaid diagrams embedded in source pages, model content, and trace roll-up views use the same typography, background, border, zoom/pan controls, source-copy control, and selected/hovered visual treatment as the rest of the Explorer.
- Each Mermaid diagram initializes independently after its route content is visible. Rendering many diagrams must not block the initial Explorer shell or require all diagrams to finish before the page becomes usable.
- Small diagrams shrink to natural height. Large diagrams receive their own pan/zoom viewport and must not force unrelated page content or later diagrams into a fixed full-screen scroll region.
- Contract Bindings labels in Model and Traces diagrams use display names rather than full `file#fragment` identifiers, while the full identifier remains the link target and structured data value.

#### Metadata
  * type: specification

#### Relations
  * define: [Diagram Contract Bindings Display](Capabilities.md#diagram-contract-bindings-display)
---

### Explorer Navigation Chrome Specification

Specification for the headerless Explorer shell, persistent left project tree, and active-view controls in the static SPA.

#### Details
The native Explorer application must not render a top header. Application chrome is provided by a persistent full-height left Explorer pane, a central workspace, and a narrow right vertical tool rail.

The left Explorer pane must expose its `Explorer` title as a persistent vertical edge strip on the pane boundary, not as a horizontal header. Clicking the strip must collapse or expand the pane, and the strip must include a compact midline affordance indicating the collapse/expand action. The expanded pane must be wide enough for per-view controls and tree rows, with a nominal desktop width of about 340px. When collapsed, the pane must remain as the same narrow fixed vertical `Explorer` strip, and the central workspace must resize to use the recovered width without covering the right tool rail.

Active-view controls must start at the top of the expanded left pane content area, beside the persistent vertical `Explorer` strip. In the Model route those controls expose compact List, Grid, and Graph mode buttons. Specialist views render their own filters, reset actions, legends, or notation keys there instead of adding primary left-pane view links.

For Model and file drill-in routes, below the Model controls, the left pane must render a filesystem-equivalent navigation tree. The tree must provide the same compact quick-filter input treatment used by specialist panes and must filter folders, files, and modeled-element rows without creating a second search surface. While a quick filter is active, matching folders/files/elements and their ancestors must be visible and temporarily expanded; clearing the filter restores the prior manual tree expansion state. The tree must show folders and files for all document types. When a file contains multiple modeled elements, the file row must expand to show those element rows. Non-element documents and single-element files may remain a single file row to avoid redundant tree depth. Specialist views must not inherit this tree unless they explicitly define their own domain-specific navigator.

The Model project tree must initialize with only the root project node expanded. First-level folders and files must be visible, but child folders and file element rows must remain collapsed until the user expands them or a later explicit selection workflow requires revealing a selected descendant.

The Model tree root must identify the served Git repository and current branch when that metadata is available from the Project Store, using a compact label such as `repo @ branch`. It must not use a generic `Project` label when repository identity is known.

The Coverage route must define its own compact coverage explorer in the left pane rather than reusing the Model project tree or duplicating dashboard summaries and legends. The coverage explorer must list Overview, Capability coverage, Unverified requirements, Unimplemented requirements, Unsatisfied verifications, and Orphaned verifications with counts derived from Project Store coverage data. Coverage explorer rows must use the shared pane navigation row styling with hover and selected states. Selecting a coverage explorer item must scroll and mark the matching section in the central Coverage workspace, while coverage row/item clicks continue to open the shared element-detail modal when they target a modeled element.

The Thesaurus route must define a concept-scheme navigator in the left pane rather than reusing the Model project tree or rendering filesystem/folder metaphors. The navigator must show concept schemes and their broader/narrower concept hierarchy directly, use concept-scheme and concept glyphs, provide the same compact quick-filter input treatment as other pane filters, temporarily expand matching concept branches while filtering, and keep counts in the shared bottom-summary placement.

Selecting a folder, file, or modeled element in the Model project tree must update the active Model workspace mode. List and Grid modes must browse to the selected folder or file; Graph mode must focus the matching graph node when one exists; modeled-element selection must open the shared element-detail modal. The tree selection must be visibly marked in the left pane and must return file drill-in routes to the Model workspace instead of leaving the middle pane on a disconnected Files view.

The right vertical tool rail must expose icon actions for specialist views and tools, including Ontologies, Traces, Search, source/document views, settings, and help as applicable. Ontologies and Traces are separate routed views, but they must not appear as left-pane primary mode links.

When the Explorer shell renders the primary top view navigation, the canonical order must be Thesaurus, Model, Ontologies, Traces, and Coverage. Thesaurus appears first because curated terminology is the conceptual entry point into the model; Model remains the default workspace route until a separate requirement changes the default route.

The left Explorer pane must not expose Ontologies, Traces, Coverage, or Resources as primary Explorer links. Specialist views are reached through top navigation, tool actions, or supporting canonical routes, while source/specification artifact pages remain secondary document outputs.

Explorer view links must use canonical hash routes. Source-document destinations must resolve through the SPA content route backed by Project Store source-content records and metadata.

The persistent left pane and right tool rail must be:
- Always visible while the central workspace scrolls, pans, or zooms
- Consistent across all primary and specialist Explorer views
- Clearly visible and accessible
- Compact enough that tree, tool, and detail chrome do not waste graph or document reading space
- Free of generated attribution footers in served pages

Primary and supporting SPA view modules must not render their own duplicate left navigation panels, hidden left-panel alternates, or top navigation headers. View-specific mode buttons, filters, search inputs, reset actions, legends, and selected-node summary links must render at the top of the expanded left Explorer pane using the shared compact control design, unboxed, before any view-owned tree. Control/filter groups must use the design-system `PaneControlSection` pattern. Navigation trees, including the Model project tree and Thesaurus concept hierarchy, must use `PaneTree` directly rather than wrapping tree navigation in a control/filter section. The central workspace is reserved for canvas/list/grid/content rendering.

Left-pane quick filters must use the shared `PaneSearchForm` treatment across Model, Thesaurus, Traces, Ontologies, and any later specialist pane. The quick-filter pattern owns the tokenized top and inline inset from the pane edges, plus the single tokenized vertical gap before the next pane block. The first block after the quick filter, whether a tree, section label, or filter/control section, must not add extra top padding, duplicate vertical gap, or a divider line below the search input.

Every left pane that renders summary counts must use the shared bottom-summary placement. The summary must stay at the pane bottom with a clear tokenized gap above the `Summary` label so counts do not push interactive filters, selection context, legends, or concept/navigation trees down the pane.

Workspace routes that need a top breadcrumb/control row must use the shared design-system `WorkspaceShell` pattern directly. Workspace breadcrumbs must only render when they express a real path or action, such as `Model / Graph` where the root crumb returns to Model grid mode; single-view routes must not show a redundant non-functional crumb. Route-level aggregate counts belong in the left Explorer pane bottom summary, not in the top-right workspace toolbar, unless the count is intrinsic to a local list/table control. Model Graph mode controls must use shared segmented-control visual treatment instead of route-local toolbar styling. The Thesaurus route identity is `Thesaurus`; its workspace is the deterministic concept-scheme Map view. Thesaurus must not expose a Browse/Map mode switch. Thesaurus Map must use the shared design-system product pattern backed by React Flow for node interaction and deterministic concept-scheme mind-map layout; Explorer route code may pass data and actions but must not own route-local map styling. Route modules must not keep view-specific aliases or compatibility wrappers for shared workspace chrome.

Primary Explorer routes must not spend first-viewport space on static view titles, top headers, or explanatory prose. Model List/Grid/Graph, Ontologies, and Traces view explanations belong in an on-demand help modal opened from the shared help action. Source/specification artifact pages may keep document-specific headings inside their own content.

#### Metadata
  * type: specification
---

### Explorer Serve Pipeline Specification

Technical specification for the Explorer serve runtime pipeline.

#### Details
**Runtime Data Setup:**
- Assemble Project Store data in memory from the validated registry, semantic index, report projections, modeled resources, and existing graph-referenced local resource/evidence files.
- Resolve and preserve contract_bindings identifier links to contract elements.

**Serve Pipeline:**
1. Serve the embedded Explorer shell: `index.html` plus deterministic compiled Vite/TypeScript/React and Reqvire Explorer design-system assets, with no CDN-loaded framework or stylesheet.
2. Serve `assets/project-store.js` containing the normalized browser-local project snapshot consumed by the SPA view modules.
3. Generate Model Graph, Traces, and Ontologies projections for native SPA routes, plus coverage, resources, file, search, element-detail, raw Markdown content, and Model containment projections for supporting routed workflows.
4. Serve `ontologies.ttl` from the semantic index for raw ontology/SHACL audit and downstream tooling.
5. Return the embedded `index.html` shell for non-asset browser routes so SPA navigation remains stable.

**Browser Rendering Contract:**
- Rust serve runtime must not render Markdown, Mermaid, ontology, knowledge-graph, trace, or source-document HTML
- The Explorer SPA must render Markdown/source content, Mermaid diagrams, ontology views, knowledge graph views, traces, coverage, resources, search, and element details from Project Store data
- Internal Markdown links must resolve in the browser to canonical `index.html#/content/<path>` or `index.html#/elements/<identifier>` routes

**Output:**
- Serve the Explorer shell, generated data assets, and ontology artifacts over HTTP.
- Keep generated Explorer artifacts in memory for the HTTP runtime.

**Source Protection:**
- Never modify original repository files
- Browser runtime data generation happens in memory

**Related System Elements:**
- Ensure that related system elements are available in the Project Store so the served model remains consistent

#### Metadata
  * type: specification
---

### Explorer Store Seed Data Output Specification

Explorer serve runtime generation must emit a normalized browser-local Project Store seed that can initialize the native `index.html` SPA Explorer shell.

#### Details
The seed output must:
- Be generated from the validated graph registry, semantic index, report projections, and graph-referenced local resources during Explorer runtime generation.
- Seed source-file content directly from the in-memory graph registry and graph-referenced local resources: files with modeled elements use normalized registry-generated Markdown, while existing local implementation/evidence/resource files use captured raw source content when available.
- Keep implementation files, evidence files, scripts, images, and other local relation targets as Project Store `resources` for relation semantics, and include them in Project Store `files`/`folders` only when they are existing repository-relative local files referenced by the graph registry.
- Build the Project Store `files` and `folders` sections only from modeled element source files and existing graph-referenced local resource/evidence files. Unrelated git-tree files, unsupported parsed pages, nonexistent local targets, and external URLs must not become Explorer file-tree or file-search records.
- Preserve full repository-relative hierarchy for every Project Store file path so the Model tree renders folders, files, and file-owned elements without flattening single-element files or registry-linked resource files.
- Be deterministic for unchanged model input, excluding explicitly documented volatile metadata.
- Be available to the native SPA view modules in `index.html` before any Explorer view attempts to render, and be consumable from local static assets without a CDN-loaded framework or stylesheet.
- Contain required top-level sections for `project`, `folders`, `files`, `resources`, `elements`, `relations`, `contract_bindings`, `concept_refs`, `thesaurus`, `submodels`, `traces`, `coverage`, `ontology`, `knowledge_graph`, `search`, `summaries`, and `routes`.
- Represent file containers and modeled resources as distinct record families.
- Preserve source links from elements, ontology terms, traces, coverage records, search documents, and resource references back to served file containers when a browsable source file exists.
- Preserve element detail route data so Explorer links can open `index.html#/elements/<identifier>` as a Project Store-backed modal while retaining source-page anchors as secondary source actions.
- Preserve concept-reference records with enough resolved native concept data for element-detail rendering to match inline prose against concept preferred labels, alternative labels, and authored reference labels, and to open the referenced native concept element modal.
- Preserve relation evidence, including authored versus generated/opposite provenance, without forcing every view to consume duplicate opposite relation edges.
- Preserve canonical SPA route data and source/report links without emitting retired Explorer page route mappings.
- Include a schema/version marker so future Project Store schema changes can be detected by the browser shell.
- Fail closed or expose a visible diagnostic when required seed data is missing, malformed, or incompatible with the shell schema version.

The seed may be embedded inline in `index.html` or loaded as a static asset, but it must remain browser-local to the served Explorer shell.

#### Metadata
  * type: specification

#### Relations
  * define: [SPA Explorer Shell and Project Store](Capabilities.md#spa-explorer-shell-and-project-store)
---

### Explorer Verification Trace Rendering Specification

The Traces Explorer view must render verification flow and roll-up diagrams through the shared Explorer visual system.

#### Details
Trace rendering behavior:
- Use the Project Store `traces` projection as data input and keep trace tree construction in Functional Processing.
- Render Trace flow and Rows modes as native Explorer route states, with controls in the left Explorer pane rather than floating page-local toolbars.
- Provide the shared compact left-pane quick filter for trace files and verification rows. While a trace filter is active, matching trace folders, files, verifications, and their ancestors must remain visible and temporarily expanded.
- Render trace file, verification, direct requirement, and roll-up requirement counts in the shared left-pane bottom summary placement.
- Use the shared Explorer role palette for files, capabilities, requirements, and verifications.
- Show selected trace elements through the left-pane selected-item link and shared element-detail modal rather than route-local floating inspectors.
- Render per-verification Mermaid roll-up diagrams progressively and independently, preserving pan/zoom interaction for large diagrams without introducing nested page scroll bugs.
- Verification trace diagrams must display directly verified requirements and requirement roll-up context with compact labels and source links.

#### Metadata
  * type: specification

#### Relations
  * define: [Traces View Generation](Capabilities.md#traces-view-generation)
---

### Export Command Contract Specification

#### Details
Export command behavior:
- Write the embedded Explorer shell, generated Project Store data, and ontology artifact to the requested output directory.
- Preserve repository-relative path layout for copied workspace static assets so Markdown image and document links rewritten by the Explorer resolve from the exported static site.
- Copy local static asset file types used by rendered workspace content, including PNG, JPEG, GIF, WebP, SVG, PDF, text, CSV, JSON, JSON-LD, Turtle, and TTL files.
- Skip generated, dependency, VCS, and transient directories such as `.git`, `.index`, `node_modules`, `target`, `tmp`, and the output directory itself.
- Do not copy Markdown model source files as static assets; source Markdown content remains represented through the Project Store.

#### Metadata
  * type: specification

#### Relations
  * define: [Export Command](Capabilities.md#export-command)
---

### Model Browser and Graph Specification

Specification for native Model browsing and graph visualizations.

#### Details
The Model route's List, Grid, and Graph modes are expected to display the physical and relational project-store structure as native Explorer visualizations:
1. Root node representing the model root
2. Folder nodes
3. File nodes containing element children
4. Element nodes with type-specific icons and colors
5. Resource/evidence targets when enabled by the graph controls
6. Clickable elements that open the shared in-shell element detail modal while preserving the current Model mode

The Model containment modes must provide:
- List and Grid browsing over the same Project Store folder/file/element hierarchy. List-style discovery is provided by Search and Model List/Grid modes rather than by a separate Containment List mode.
- Graph mode rendered with the same knowledge-graph data, role colors, filters, overlays, and source links as the project graph renderer.
- Folder/file drill-in behavior in List and Grid, plus graph pan/zoom/focus behavior in Graph mode.
- When Graph is active inside the Model route, selecting a folder, file, or modeled element in the left project tree must update the graph focus when a matching graph node exists. Element selections must also expose a selected-element link and open the shared element-detail modal while preserving the Model mode.
- Modeled-element Grid cards must align the leading element marker, title, and type badge with minimal spacing. The card must use a single leading `ElementIcon` as the visual type marker, and its type badge must render the type text without an additional marker glyph.
- A persistent full-height left Explorer pane with a vertical `Explorer` edge strip; the expanded pane starts with active-view controls, then renders a filesystem-equivalent project tree of folders, files, and modeled elements only for Model/file drill-in workflows.
- A narrow right vertical tool rail for switching contextual tools such as Search, Ontologies, Traces, source/document view, settings, and help.
- A responsive workspace layout where List, Grid, and Graph grow to the available space between the left Explorer pane/strip and the right tool rail.

The Model view remains the default Explorer route and visual overview of the model structure, while `index.html` remains the primary SPA shell.

#### Metadata
  * type: specification
---

### Model View Element Navigation Contract Specification

#### Details
Model-view element navigation behavior:
- Element-name headers render as hyperlinks.
- Links target element source file plus fragment identifier.
- Link format: `[Element Name](file_path#element-fragment)`.
- Navigation enables direct jump from model view to definition.

#### Metadata
  * type: specification

#### Relations
  * define: [Model View Element Navigation](Capabilities.md#model-view-element-navigation)
---

### Model-Centric View Generation Contract Specification

#### Details
Model-centric view generation behavior:
- Uses model roots selected by default traversal rules as top-level entries.
- Presents the canonical Model route as three compact icon-selected modes in the shared left Explorer pane: List, Grid, and Graph.
- Uses List and Grid modes as Reqvire-native filesystem/model navigation over folders, files, and modeled elements. Uses Graph mode as the Project Store knowledge-graph visualization. Filesystem discovery and list-style discovery belong to Model List/Grid and Search, not to separate primary Explorer pages.
- Keeps Model mode controls in the left Explorer pane before the project tree. Model list/grid/containment views must not render a local top toolbar for mode selection, role filters, or overlay toggles.
- Drives the active Model workspace from the left project-tree selection across all three Model modes so List/Grid browse selected folders or files, Graph focuses matching graph nodes, and selected modeled elements open the shared Project Store-backed modal.
- Expands relations recursively with full target element details.
- Includes summary metadata for element and relation counts.
- Generates Mermaid diagrams for nested relation structures.
- Sizes each Mermaid diagram after rendering based on its own SVG bounds. Small diagrams shrink to their natural height, while large diagrams receive an independent pan/zoom viewport; pages with many diagrams must not force every diagram, including the last one, into a fixed full-viewport-height panel.
- Loads the Mermaid initializer exactly once per Explorer runtime so diagrams are not parsed, mutated, or pan/zoom-initialized twice.
- Produces model-view output consumed by `index.html#/model` without generating a separate Model document entry point.

#### Metadata
  * type: specification

#### Relations
  * define: [Model-Centric View Generation](Capabilities.md#model-centric-view-generation)
---

### OWL Semantic Ontology Projection Contract Specification

#### Details
Semantic projection behavior:
- Classifies resources into semantic node kinds, including OWL/RDFS class, object property, datatype property, RDF property, named individual, SHACL node shape, SHACL property shape, datatype, and generic RDF resource when no stronger kind is known. Literal values are not primary graph nodes; they are subject-owned modal/search evidence.
- Promotes otherwise-generic named resources to named-individual view nodes when their RDF type evidence references a class declared in the same ontology graph, even when the authored RDF does not explicitly include `owl:NamedIndividual`.
- Consumes generated ontology projection facts derived from o-kernel construct classifications and reused through `SemanticIndex` before full semantic export or Explorer rendering.
- Preserves full IRI, compact label, RDF types, source element identifiers, source file paths, source line numbers, comments, related SHACL constraints, normalized slot/facet evidence, optional raw SHACL evidence, and projection provenance in the Explorer ontology model derived from `SemanticIndex.ontology_projection`.
- Derives SHACL slot/facet records from property-shape blank nodes and reuses them to the target class plus named property metadata as viewer-facing construct evidence. Target class nodes present those records as slots of the class; property usage rows present those records as class-specific usages of the selected property.
- Separates direct-authored generated facts from inferred facts. Direct-authored facts may drive semantic export and Explorer rendering now; inferred facts require a later inference or materialization requirement.
- Suppresses `rdf:type` edges, OWL/RDFS metaclass nodes such as `owl:Class`, `owl:ObjectProperty`, `owl:DatatypeProperty`, and `rdfs:Class`, and RDF list plumbing from the primary graph.
- Suppresses anonymous blank nodes from the primary graph unless the blank node represents a meaningful semantic construct such as a property chain, equivalence group, SHACL shape, or collection member.
- Retains unmodeled RDF statements only as modal/source evidence, not as graph nodes and edges.

#### Metadata
  * type: specification

#### Relations
  * define: [OWL Semantic Ontology Projection](Capabilities.md#owl-semantic-ontology-projection)
---

### Ontologies View Generation Contract Specification

#### Details
Ontologies view generation behavior:
- Uses the semantic index built from graph-registry ontology and semantic-contract elements.
- Displays summary counts for ontology blocks, shape blocks, RDF quads, total blocks, and the `ontologies.ttl` download action in the Ontologies left Explorer pane near the reset/search controls.
- Aligns ontology summary and download controls with the shared left-pane summary treatment: compact entries, muted surface tokens, visible grouping, wrapping when needed, and no clipped `Download .ttl` action.
- Builds the browser visualization, search index, and ontology node modal metadata from `SemanticIndex.ontology_projection` facts; raw quads may support labels, comments, RDF type evidence, SHACL constraint display, and generic low-level links, but must not be a separate authoritative extraction path for OWL/RDFS construct metadata.
- Emits `ontology.graph_data.nodes[]` and `ontology.graph_data.edges[]` with explicit `layer` and `source_kind` fields. The canonical layers are `authored`, `concepts`, `reqvire-context`, and `external-source`; authored OWL/RDFS/SHACL structural projection facts are controlled by the Core layer, exported SKOS concept and concept-scheme nodes, SKOS concept taxonomy edges, and concept bridge edges are controlled by the Concepts layer, generated semantic context is an opt-in layer limited to model-to-term `declaresTerm` and `referencesTerm` provenance, and external vocabulary triples are limited to the used external subset derived from explicit external source ingestion.
- Emits `ontology.external_materialization`, `ontology.external_sources`, and `ontology.external_counts` metadata so Explorer can distinguish declared external sources from the materialized used subset and report declared-source, used-source, visible-declaration, materialized-term, raw-triple, and materialized-triple counts without exposing raw unused dependency content.
- Exposes Ontologies left-pane overlay controls for Core, Concepts, Semantic Context, and External Sources. Core controls authored structural ontology nodes and projection edges. Concepts controls exported curated `skos:Concept` and `skos:ConceptScheme` nodes, SKOS taxonomy edges such as `skos:inScheme`, `skos:hasTopConcept`, `skos:broader`, `skos:narrower`, and `skos:related`, plus one-way `reqvire:mapsToConcept` bridge edges. The exported graph-data contract keeps directional SKOS predicates directional, but canonicalizes symmetric concept associations such as `skos:related`, `skos:exactMatch`, `skos:closeMatch`, and `skos:relatedMatch` to one conceptual edge so reciprocal RDF triples do not render as circular duplicate links. Core and Concepts are shown by default; Semantic Context and External Sources are opt-in so model or external-vocabulary facts do not change the primary authored ontology view.
- Does not expose the raw RDF triple graph as the primary user-facing ontology visualization.
- Reuses the same generated ontology construct projection that full semantic export emits so the Ontologies SPA route uses the canonical semantic projection.
- Uses the Explorer-bundled Ontologies route renderer over `ontology.projection`, declarations, shape references, diagnostics, and `ontologies.ttl` link data. Rust serve runtime generation must not provide executable ontology renderer JavaScript or CSS through the Project Store.
- Opens directly on the ontology explorer without a separate page header, descriptive preamble, top-level summary-card band, footer, or shared padded content card.
- Uses a dense canvas layout that fills the available viewport inside the headerless Explorer shell so ontology graph space is prioritized.
- Uses Sigma.js over a Graphology graph with ForceAtlas2 layout for the ontology visualization, matching the project knowledge graph rendering engine while preserving ontology-specific projection and filter semantics. Uses Sigma curved-arrow edge programs and the edge-curve parallel-edge indexer for ontology relationship edges so parallel edges between the same nodes bend apart instead of stacking labels on top of each other. The Ontologies route must paint its shell and graph-canvas loading notice with the design-system spinner glyph before Sigma/ForceAtlas construction is scheduled in a post-paint task.
- Uses an ontology-diagram visual language rather than a raw RDF graph: class-like terms, restrictions, and class expressions render as compact circular or elliptical anchors; object/datatype/RDF properties render as labeled relationship edges and modal evidence rather than standalone graph nodes; datatype, named-individual, SHACL, and generic resource nodes remain visually distinct; relation edges render as directed ontology-diagram connectors with visible arrowheads. Layout should favor class-centered neighborhoods where property labels stay close to their domain/range class anchors, similar to established ontology diagram viewers.
- Draws relation connectors with subtle arrowheads and muted strokes so direction remains visible without arrows visually overflowing or dominating node labels.
- Styles ontology edges in a WebVOWL-aligned visual grammar while retaining Reqvire colors: normal ontology property relationships use solid labeled Sigma arrows with compact colored label badges on the edge; subclass relationships use the same dashed connector stroke style as class-expression construct links but with a hollow triangle marker at the superclass target side; edge labels are anchored on the same curved connector geometry as the rendered Sigma edge; and relation strokes remain dark enough to read against the ontology canvas.
- Renders OWL set-operator/class-expression member links with a dedicated Sigma/WebGL edge program, matching WebVOWL's structural set-operator behavior while preserving the expression kind and members in node labels and modal evidence. Class-expression member links must render as dashed curved connectors with an open diamond marker at the anonymous construct/source side and an arrowhead at the member target side. Dotted, dashed, hollow-triangle, or diamond-marker styling must not be drawn from the edge-label canvas hook.
- Classifies renderable ontology nodes through a single construct-class contract derived from semantic type and projection construct evidence. Restriction and class-expression classification drives glyph rendering, relation visibility controls, construct-only node gating, and focused-neighborhood behavior consistently.
- Renders OWL set-operator/class-expression and restriction nodes as compact construct circles using Sigma `nodeProgramClasses` with `@sigma/node-image`, so union/intersection/complement and restriction expressions read like WebVOWL construct nodes instead of ordinary named ontology classes. Sigma must own the circular node background from the node color, while an inline SVG pictogram supplies a bold semantic glyph inside the clipped image node, and the Sigma node label must name the construct kind. Construct glyphs must not use PNG/raster sprites, SVG border detail, transparent square image backgrounds, or custom construct-node hover overlays. Out-of-focus construct nodes must dim through the same Sigma reducer path as ordinary nodes.
- Shows glyph-only construct circles only for actual OWL anonymous construct nodes, such as `owl:unionOf`, `owl:intersectionOf`, `owl:complementOf`, and `owl:Restriction`, when the corresponding `Class expressions` or `Restrictions` visibility control allows them and the focused edge visibility rules make their neighborhood visible.
- Uses `@sigma/edge-curve`'s curved-label renderer for non-property construct labels so labels follow the same curve geometry as the Sigma edge program.
- Renders subclass connectors through a dedicated Sigma/WebGL notation edge program while keeping the readable `Subclass of` label anchored on the connector.
- Renders restriction constructs as explicit restriction glyph nodes plus Sigma-native restriction connector arrows to their `on property` and filler/target evidence, rather than rendering restrictions as ordinary domain/range property edges.
- Keeps generic SHACL overlay relationship labels off the graph canvas because the overlay line and modal evidence are sufficient; only more specific ontology/SHACL relation labels should render as edge text.
- Uses Sigma's default node-label and hover rendering for ontology node labels. Normal graph labels may be density-truncated, but the selected or hovered ordinary ontology node label must switch to the full term label; construct glyph nodes show their construct-kind label through Sigma labels and their semantic symbol through Sigma image-node rendering.
- Hides ontology relationship edges in the default full-graph view to avoid a hairball, matching the Knowledge Graph behavior. On node hover or click selection, shows ontology edges directly incident to the focused node set through Sigma's native edge reducer and `@sigma/edge-curve` edge programs; active relation visibility controls determine which focused edges are eligible to appear. If the visible focused neighborhood reaches an enabled construct-only node, such as an OWL union, intersection, complement, or restriction, the focus expands through that construct node so its member/filler links are visible too.
- Enables Sigma z-index and highlighted-node rendering so the active ontology focus tree is painted through Sigma's normal focus path. Focused nodes must sort above focused-neighbor nodes, focused-neighbor nodes must sort above focused edges, and focused edges must sort above unrelated or muted graph items. Focused edges must be native Sigma edge-program output rather than a separate focused-edge canvas overlay.
- Sizes circular class anchors by graph connection degree within bounded minimum and maximum sizes so highly connected ontology concepts read as stronger anchors without making low-degree concepts unreadable or letting label length dominate circle size.
- Uses the ontology semantic role palette: class anchors, SKOS concept nodes, property link labels, modal property badges, datatype leaves, named individuals, SHACL shapes, generic resources, restrictions, class expressions, and external references each resolve through ontology role tokens, with the graph canvas using the shared Explorer base surface. SKOS concepts use the dedicated `--rdf-concept` token so concept taxonomy nodes are visually distinct from structural ontology classes; unresolved concept-reference facts use the darker `--concept-reference` token in element-detail surfaces.
- Opens selected ontology node detail in an ontology element modal wide enough for source links, long identifiers, property cards, slots/facets, projection constructs, and source citations.
- Does not render a raw Turtle/source-block list in the Ontologies route.
- Preserves source element identifier, source name, file path, line number, and block kind as modal/search evidence.
- Renders source citations in the modal as links to the served source route and element fragment.
- Provides the served `ontologies.ttl` download link in the Ontologies left Explorer pane alongside summary counts and reset controls.
- Colors rendered nodes by semantic role rather than by provenance. Classes, SKOS concepts, named individuals, datatypes, restrictions, class expressions, SHACL node shapes, SHACL property shapes, and generic RDF resources each use distinct legend swatches and graph colors, while object/datatype/RDF property kinds remain visible through property link labels, modal badges, and search metadata instead of graph-node fill colors.
- Treats a named IRI with `rdf:type` pointing to a declared ontology class as a named individual for node color, search badge, and modal kind when the node has no stronger semantic role such as class, property, shape, datatype, restriction, or class expression. The `rdf:type` statement remains represented as membership construct evidence rather than as a generic RDF edge.
- Reserves SHACL colors for actual SHACL node shapes and property shapes. SHACL references to ontology terms remain evidence on the referenced term and do not recolor classes, individuals, datatypes, restrictions, class expressions, or property metadata as SHACL shapes.
- Keeps SHACL references to ontology terms as source/construct evidence without recoloring the referenced ontology class or property metadata as a SHACL shape.
- Treats built-in vocabulary references from XSD, RDF, RDFS, OWL, and SHACL namespaces as external references. External references remain available for datatype/range audit, but they are hidden by default to prevent built-in terms such as `xsd:string` from cluttering the primary ontology map.
- Does not render literal values as primary graph nodes or as a visibility filter layer. Literal object values from datatype properties remain searchable and are shown in the modal as predicate/value evidence owned by the selected subject node.
- Labels class-expression nodes with their property usage context when the expression is used as a property domain or range, for example a valid union-valued range from the ontology model, so OWL domain/range constraints do not look like authored Reqvire model relations. The `define` relation specifically must not render as `Capability ∪ Requirement`; its range is `Requirement` only.
- Derives class and property slot facets from SHACL node shapes by combining `sh:targetClass`, `sh:property`, `sh:path`, `sh:datatype`, `sh:class`, `sh:nodeKind`, `sh:minCount`, `sh:maxCount`, `sh:pattern`, and `sh:in`. The target class modal must show those slots and facets directly, with source-shape evidence, without requiring users to inspect the shape node first.
- Reuses Contract SHACL-derived slot facets to the named property metadata as well when the `sh:path` value is a named property already present in the graph model. On a selected class or term, those records represent property usages as slots; they must be labeled as property usages rather than as duplicate property definitions.

Interaction behavior:
- Provides search over ontology labels, IRIs, semantic kinds, source elements, and SHACL constraint terms.
- Provides node or construct focus, neighbor highlighting, and an ontology element modal for full IRI, semantic kind, RDF type evidence, comments, datatype-property literal values, source citations, property usages, domain/range, property characteristics, equivalence membership, inverse relationships, property chains, normalized SHACL-derived slots/facets, and optional raw SHACL evidence.
- Treats graph visibility controls as canvas-only controls. Role and relation visibility controls must not remove facts, properties, badges, SHACL facets, constructs, or source evidence from the ontology element modal for the selected node.
- Shows raw SHACL evidence only when direct raw constraints are bound to the inspected node. Class and property nodes that only receive normalized SHACL overlays must not show an empty raw-evidence section, and normalized slots/facets remain the primary readable representation of those SHACL overlays.
- Provides one `Show` visibility group using a shared button design for canvas visibility filters: datatype property links, object property links, class disjointness, restrictions, class expressions, SHACL shapes, resources, and external references. Ontology terms and class-membership context are always available rather than exposed as toggles. For these visibility toggles, active means shown on the canvas. Equivalence, inverse properties, property chains, property characteristics, and SHACL overlay notation remain passive notation/modal evidence until they have a direct canvas-visible control.
- Renders the Ontologies `Reset`, active `Show` visibility controls, passive `Types` color key, and passive `Notation` legend in the shared left Explorer pane; the Ontologies route must not inherit the Model project tree, and the ontology canvas must not render a duplicate in-canvas legend/filter box.
- Preserves interactive graph manipulation while filtering, focusing, panning, zooming, dragging nodes, and resizing; filtering must not reset the current camera, focusing must center the selected node, and reset must rerun the ForceAtlas2 layout and fit the graph without losing ontology-diagram visual conventions. The viewer exposes a single visible `Reset` view control rather than separate fit and reset buttons.
- Provides a compact passive color key for detailed semantic node roles in the left Explorer pane, separate from the active visibility-layer filter controls.
- Keeps Reqvire's richer passive type and construct legends available alongside the active `Show` controls, because Reqvire exposes additional semantic constructs beyond class disjointness and set operators.
- Treats only filter-control entries as selectable filter-in controls in the interactive viewer. The default filter state must open as a complete authored ontology map: ontology terms and class-membership context are always present; datatype-property links, object-property links, disjointness, restrictions, class expressions, SHACL shapes, resources, and external references are shown by default. Relationship edges may still be visually emphasized only on hover or selection focus to avoid a hairball.
- Uses shared selected-control tokens, warm-neutral search-result hover rows, and generic evidence cards so filter and selection state reads as part of the explorer theme rather than as unrelated route-local UI chrome. A persistent left project tree appears only in Model and file drill-in workflows, or in a specialist view that defines its own domain-specific navigator.
- Keeps detailed type-color entries passive so users do not have to treat every color swatch as a visibility control.
- Supports grouped visibility controls for SHACL shapes, generic RDF resources, and used external vocabulary references. The single SHACL shapes role filter controls both SHACL shape nodes and their SHACL overlay relations. Property links are controlled by the datatype-property and object-property visibility filters. Detailed semantic types remain visible through color, modal kind, and search badges.
- Computes selected, hovered, and rollover focus trees from currently visible relation filters. If a relation filter hides the edge category that connected a neighbor to the focus node, that neighbor must no longer be visible as part of the selected or rollover focus tree, while ontology node modal data remains an unfiltered evidence view for the selected node. Construct-only nodes expand the focused tree one semantic step further through their enabled construct links so users can reason about union/intersection/complement members and restriction fillers without selecting the anonymous construct first.
- Keeps the external-reference role filter available; enabling or disabling it controls built-in vocabulary nodes used by datatype constraints, ranges, and other audit-oriented constructs.
- Presents domain/range, subclass, membership, disjointness, equivalence, inverse, property chain, property characteristic, restriction, class-expression, and SHACL-overlay constructs in a passive left-pane `Notation` legend rather than as a second active construct-filter panel.
- Renders Ontologies quick-search results with the same RDF type color swatches used by the graph and passive `Types` legend, and shows right-side badges only for actual construct or relationship notation symbols from the passive `Notation` legend. Search results must not introduce separate type-letter glyphs, local colors, or page-specific legend semantics outside the shared RDF/notation token system.
- Treats relation styling as passive visual notation, not as an independent filter layer. The active `Show` relation controls determine which categories are shown, such as property links, disjointness, restrictions, and class expressions; class membership remains available as backbone context, and SHACL overlay relations follow the single SHACL shapes role filter.
- Applies relation visibility controls to construct-specific edges, construct-only nodes, and graph node badges without making nodes visible when their role filter is disabled. The ontology element modal remains an unfiltered evidence view for the selected node.
- Does not expose or render graph-registry provenance or generated-projection provenance as graph-wide visual layers. Registry/projection provenance remains modal and Project Store evidence, not a visual layer selector and not a reason to add otherwise non-semantic debug nodes to the primary graph.
- Treats construct-only visual nodes, including generated restriction and class-expression nodes, as semantic construct nodes controlled by their matching relation visibility control rather than by a separate provenance filter.
- Combines active controls predictably on the canvas: role controls are hard gates for authored node/resource visibility, and unchecked relation controls remove their named categories from property links, edges, construct-only nodes, and graph badges.
- Keeps filtered-out canvas nodes, edges, and graph badges visually suppressed or hidden without clearing or narrowing the current modal detail selection.
- Applies directional subclass and membership badges only to the subject side of the construct; superclass and class-object nodes must not display `⊆` or `∈` merely because another node points to them.
- Keeps raw Turtle content available through `ontologies.ttl` instead of duplicating the serialized text in the Ontologies viewer.

#### Metadata
  * type: specification

#### Relations
  * define: [Ontologies View Generation](Capabilities.md#ontologies-view-generation)
---

### Ontology Construct Grouping Contract Specification

#### Details
OWL construct grouping behavior:
- Computes equivalence groups for `owl:equivalentClass`, `owl:equivalentProperty`, and `owl:sameAs` using deterministic connected components over the equivalent resources.
- Assigns each equivalence group a stable identifier derived from a canonical sorted member list rather than a random UUID.
- Renders equivalence groups as collapsible group nodes or grouped regions so users can inspect group membership without requiring pairwise equivalence edges to dominate the graph.
- Represents `owl:inverseOf` as inverse property metadata with clear visual treatment where applicable and modal evidence.
- Parses `owl:propertyChainAxiom` RDF lists into ordered chain members and reuses the ordered chain to the defining object property.
- Renders property chains as collapsible ordered chain constructs, preserving the member order from the RDF list.
- Allows an object property to participate in multiple property chains without duplicating graph nodes for the property.
- Presents SHACL node shapes and property shapes as an overlay on referenced ontology terms and as modal constraints, not as raw blank-node plumbing mixed with the ontology model.
- Presents anonymous OWL class-expression blank nodes such as `owl:unionOf`, `owl:intersectionOf`, and `owl:complementOf` as structural modal constructs with expression kind, ordered members, and usage context. Raw blank-node identifiers are available only in collapsible raw details.

#### Metadata
  * type: specification

#### Relations
  * define: [Ontology Construct Grouping](Capabilities.md#ontology-construct-grouping)
---

### Ontology Property-Centric Visualization Contract Specification

#### Details
Property-centric visualization behavior:
- Renders object properties and datatype properties as first-class property semantics: labeled domain/range links in the graph plus property usage rows in the ontology node modal, not standalone property nodes.
- Aggregates all `rdfs:domain` classes and all `rdfs:range` classes or datatypes for each property.
- Renders one deduplicated labeled relationship for each meaningful domain/range pair so that one property with many domains or ranges does not become a cluttered duplicate subgraph.
- Keeps property detail, including full URI, semantic kind, domain/range terms, inverse/equivalence/chain evidence, characteristics, source citations, and SHACL slot facets, in the ontology node modal for selected classes, individuals, and terms.
- Distinguishes object-property ranges that point to classes from datatype-property ranges that point to datatypes or literal constraints.
- Shows property characteristics as compact badges or modal attributes instead of overloading the primary label by default.
- Shows SHACL-derived slot facets on target classes and property usage rows, including datatype/class range constraints, node kind, min/max cardinality, pattern constraints, allowed values, and source shape.
- Recognizes OWL property characteristic types including functional, inverse functional, transitive, symmetric, asymmetric, reflexive, and irreflexive properties.

#### Metadata
  * type: specification

#### Relations
  * define: [Ontology Property-Centric Visualization](Capabilities.md#ontology-property-centric-visualization)
---

### Ontology Symbol and Badge Vocabulary Contract Specification

#### Details
The ontology viewer symbol vocabulary defines canonical rendered symbols for OWL and set-theoretic concepts. The viewer must render the `Rendered Unicode Character` value for visual badges and must keep the `Raw Unicode` code point available in source/specification evidence.

Symbols must:
- Appear only in approved viewer locations listed in the table.
- Have tooltip text and modal text that names the semantic concept.
- Have accessible labels when rendered in interactive controls, badges, edges, or group headers.
- Supplement normal labels; labels such as property names and class names must remain searchable without symbol prefixes.
- Keep raw Unicode code points in source/specification evidence and semantic data; do not render raw code points as visible badge text in the ontology modal.
- Prefer domain labels such as `Subclass` over low-level set-theory labels such as `Subset or equal` when rendering visible ontology badges.
- Use a font stack that can render mathematical and arrow symbols consistently.

| Concept | Raw Unicode | Rendered Unicode Character | Viewer Usage | Tooltip / Accessible Label |
|---------|-------------|----------------------------|--------------|----------------------------|
| Disjointness | U+27C2 | ⟂ | Class expression badge, construct group | Disjointness |
| Logical AND / Set Intersection | U+2229 | ∩ | Class expression badge, construct group | Intersection |
| Logical OR / Set Union | U+222A | ∪ | Class expression badge, construct group | Union |
| Logical Implication | U+21D2 | ⇒ | Rule or implication edge label | Implies |
| Logical Equivalence | U+21D4 | ⇔ | Equivalence edge label, equivalence group header | Equivalent |
| Universal Quantifier | U+2200 | ∀ | Restriction badge, modal field | Universal restriction |
| Existential Quantifier | U+2203 | ∃ | Restriction badge, modal field | Existential restriction |
| Set Membership | U+2208 | ∈ | Modal field, class membership edge label | Member of |
| Set Non-Membership | U+2209 | ∉ | Modal field, class exclusion edge label | Not member of |
| Set Inclusion | U+2286 | ⊆ | Subclass or inclusion badge | Subset or equal |
| Proper Subset | U+2282 | ⊂ | Strict inclusion badge | Proper subset |
| Set Difference | U+2216 | ∖ | Class expression badge | Set difference |
| Symmetric Relation | U+2194 | ↔ | Property characteristic badge | Symmetric property |
| Inverse Relation | U+27F2 | ⟲ | Inverse property edge label, property badge | Inverse property |
| Reflexive Relation | U+25CB | ○ | Property characteristic badge | Reflexive property |
| Transitive Relation | U+25B3 | △ | Property characteristic badge | Transitive property |
| Antisymmetric / Asymmetric Relation | U+21AE | ↮ | Property characteristic badge | Asymmetric property |
| Inverse Functional | U+2190 | ← | Property characteristic badge | Inverse functional property |
| Functional | U+2192 | → | Property characteristic badge | Functional property |

#### Metadata
  * type: specification

#### Relations
  * define: [Ontology Symbol and Badge Vocabulary](Capabilities.md#ontology-symbol-and-badge-vocabulary)
---

### Project Knowledge Graph View Contract Specification

#### Details
Project knowledge graph view generation behavior:
- Uses the parsed `GraphRegistry` after validation and opposite-relation propagation so the view reflects the actual current model state.
- Exposes the project Knowledge Graph as the Model route's Graph mode during serve workflows without generating a separate Knowledge Graph document entry point.
- Builds graph nodes from actual Reqvire elements and resource targets, not from raw RDF triples or ontology vocabulary definitions.
- Builds graph edges from actual relation facts, contract_bindings facts, and concept-reference facts. User-authored and generated opposite relation facts must remain distinguishable as evidence in the graph data and detail modal. Concept-reference facts render as edges from model elements to SKOS concept nodes and must not create concept-reference nodes.
- Keeps `knowledge_graph.nodes` as a slim graph-rendering projection containing only graph identity, role, source/opening, label, and short description fields. Detail-modal evidence such as metadata, governance, incoming/outgoing relations, contract bindings, and concept references remains owned by the normalized Project Store records and must not be duplicated into every graph node.
- Emits Reqvire root submodel metadata alongside graph nodes and edges so Knowledge Graph variants can align visual subgraphs with the same capability-rooted submodel boundaries reported by the `submodels` command.
- Classifies element nodes into four primary system-model layers: ontology definitions, capabilities, requirements, and verification-family elements. `verification-objective` nodes are verification planning nodes, while concrete verification nodes carry `verify`/evidence facts. Requirement-owned contracts are subordinate requirement detail/contract nodes, not an additional system-model layer. Custom/other elements and resource targets are supporting project-fact nodes, not additional model layers.
- Separates system-model layer membership from relation semantics. Layer membership controls node role color and filtering; relation semantics control whether an edge is structural (`derive`, `specify`, `define`) or an overlay/evidence fact (`reuse`, `concept-reference`, `satisfiedBy`, `verifiedBy`, file target, or external target).
- Shows file-path and external URL targets as resource nodes only when actual project facts reference them.
- Emits SKOS concept targets as ontology-role nodes in the main Knowledge Graph when actual project elements reference them. Concept-reference facts render as element-to-concept edges, while detailed SKOS/OWL/RDFS/SHACL vocabulary exploration remains the responsibility of the Ontologies view.
- Provides search over element names, identifiers, file paths, relation facts, contract_bindings facts, governance, metadata, and concept references.
- Is rendered as the canonical `Graph` mode inside `index.html#/model`; the Model route owns List, Grid, and Graph modes.
- Receives role visibility filters and overlay toggles from the Model left Explorer pane when Graph mode is active. The graph controls are reserved for filters, overlays, legends, and reset/layout controls. The graph canvas must not render a page-local `Model Show` toolbar, top control band, or floating boxed filter surface.
- Preserves the four-layer model when overlays are enabled: concept references, requirement-owned contract details/contracts, verification evidence, satisfaction evidence, and trace links may connect across layers or subordinate nodes, but they must not imply a new layer or a new capability-root submodel boundary.
- Renders search results with the same role-color swatch used by the graph and legend, and omits redundant parenthesized type suffixes so result lists remain scan-friendly.
- Centers the graph viewport on a node selected from search or direct graph click.
- Exposes selected-node detail through the left Explorer pane: the selected node remains pinned in the graph, a compact selected-element link appears near the pane summary, and that link opens the shared element-detail modal with element type, identifier, source file/line link, description, governance, metadata, incoming facts, outgoing facts, contract_bindings, and concept references.
- Renders long modal fields such as element identifiers and source paths as stacked label/value field rows so labels do not consume horizontal space from long values.
- Colors detail badges with the same role color used by the graph node and legend entry so node type recognition is consistent across graph, pane summary, and modal detail.
- Uses a dense full-height WebGL graph canvas and persistent left Explorer pane so graph space is prioritized without a top header or always-open detail pane.
- Uses the same explorer surface language as Model List/Grid/Graph, Traces, and Ontologies views: one shared base surface for route background, canvas, Explorer pane, detail modals, and control panels; a narrow right vertical tool rail; selected-control tokens for active filter controls; and matching muted hover/evidence surfaces. The left pane background, inactive controls, search-result hover rows, selected rows, file-manager selections, and generic evidence cards must use shared Reqvire surface tokens rather than page-local colors.
- Opens in a readable project-element default state: core element roles are visible, while file-path and external-URL resource targets are opt-in through the left filter because resource leaves can dominate dense project graphs.
- Renders with Sigma.js over a Graphology graph so the project fact graph uses WebGL rendering rather than SVG.
- Uses Sigma 3 default node-label and hover rendering for project fact graph labels, without a custom node-label canvas renderer or disabled hover renderer. Normal graph labels may be density-truncated, but the selected or hovered node label must switch to the full element label.
- Uses Graphology ForceAtlas2 over the full rendered project graph with size-aware settings, Barnes-Hut optimization, deterministic initial positions, and a visible initialization diagnostic if renderer startup fails. The Model Graph route must paint its shell and graph-canvas loading notice with the design-system spinner glyph before Sigma/ForceAtlas construction is scheduled in a post-paint task. Interaction reducers must use precomputed adjacency and cached focus neighborhoods so hover and pinned-selection behavior does not repeatedly scan the full edge list.
- Preserves Sigma pan/zoom, search, filters, click inspection, and relation topology while treating resource targets as opt-in visual noise controls.
- Keeps contract_bindings, concept-reference, verification/satisfaction, and trace as opt-in overlay edge groups in the main Knowledge Graph renderer; those overlay facts may appear during focused exploration while preserving full-graph layout quality.
- Hides relation edges in the default full-graph view to avoid a hairball. On node hover without a pinned selection, shows only relation edges directly incident to the hovered node, keeps the hovered node and directly connected neighbors visually prominent, and fades unconnected nodes to low visual strength without changing active filters or graph data. The dimmed color must be computed as a 20% original-node-color blend over the graph canvas background rather than by passing WebGL-sensitive alpha color strings to Sigma.
- Uses pinned focus-tree exploration for clicked selections. When a node is clicked, the selected node and its currently visible direct neighborhood become the pinned selection tree; all graph nodes outside that selection tree are hidden from the canvas, not merely dimmed. A clicked node remains the pinned focus after pointer rollout; clicking empty graph space clears the pinned focus and returns to the default edge-hidden view.
- Supports temporary rollover refinement inside a pinned selection tree. When the pointer rolls over a visible node in the selected tree, the hovered node's own currently visible direct neighborhood becomes the active rollover tree. Nodes from the pinned selection tree that are not part of the rollover tree remain visible but fade to the same 20% dimmed treatment used for deselected nodes; nodes outside both the selected tree and rollover tree remain hidden. Rollover refinement must not clear the clicked selection.
- Does not use renderer glow/highlight effects for selection; selection is communicated through pinned focus-tree visibility, forced label visibility, direct incident edges, hidden out-of-tree nodes, and dimmed selected-tree context during rollover refinement.
- Draws relation facts in the same canonical directions used by Mermaid diagrams: `derive`, `specifiedBy`, `satisfiedBy`, `definedBy`, and `verifiedBy`. Opposite propagated facts such as `derivedFrom`, `specify`, `satisfy`, `define`, and `verify` must be reversed and deduplicated into the canonical visual edge rather than rendered as duplicate parallel relations.
- Owns its viewport sizing directly and uses shell-provided minimum viewport dimensions so the WebGL graph remains visible.
- Emits graph data with source and target identifiers that all resolve to Project Store graph nodes so Graphology edge insertion cannot fail before drawing.
- Uses one semantic role color contract across the Knowledge Graph legend, graph nodes, search swatches, and detail kind badges: capability, requirement/contract, verification-objective, concrete verification, ontology, resource, and other/default all resolve through the Explorer design-system palette API.
- Uses darker role borders only as subtle accents so legend entries, graph nodes, search result swatches, and detail badges read as one consistent visual system instead of mixing pastel controls with saturated graph nodes.

#### Metadata
  * type: specification

#### Relations
  * define: [Project Knowledge Graph View](Capabilities.md#project-knowledge-graph-view)
---

### Responsive Explorer Rendering Contract Specification

#### Details
Responsive Explorer behavior:
- Supports viewport widths from 320px (mobile) through 1920px+ (desktop).
- Uses mobile-first CSS with progressive enhancement.
- Keeps compact left Explorer view controls usable on narrow viewports without exposing report-only pages as primary navigation.
- Scales typography and spacing by responsive breakpoints.

Breakpoints:
- `sm`: 640px and up.
- `md`: 768px and up.
- `lg`: 1024px and up.
- `xl`: 1280px and up.

#### Metadata
  * type: specification

#### Relations
  * define: [Responsive Explorer Rendering](ExplorerRendering.md#responsive-explorer-rendering)
---

### SPA Explorer Store Contract Specification

#### Details
`index.html` is the single-page Reqvire Explorer shell. It owns the browser-local Project Store and uses hash routing to present primary Explorer views and supporting report/detail workflows from one normalized project snapshot. The canonical target is a native static SPA whose view content is rendered by compiled application code; this contract defines store shape and the build/runtime target, not route-local graph rewrites.

**SPA Build and Runtime Target**
- The Explorer shell must be a native static single-page application built with Vite, TypeScript, and React, emitting a static `index.html` plus deterministic `assets/explorer.js` and `assets/explorer.css` bundles.
- The shell must use the Reqvire Explorer design system for accessible layout chrome, controls, dialogs, iconography, typography, and semantic element colors, with local compiled CSS and local Geist font assets.
- Styling must come from compiled local Explorer CSS and design-system tokens. The embedded shell must not load React, styling frameworks, fonts, or other runtime assets from a CDN, and must not embed a runtime CSS compiler.
- The primary Explorer view is Model. The Model route must provide compact icon-selected List, Grid, and Graph modes: List/Grid browse the Project Store filesystem/model, and Graph renders the Project Store knowledge graph. Ontologies and Traces remain separate specialist views. Supporting workflows (Coverage, Resources, Search, Summary, File deep links, and Element Detail) must be native SPA view modules that read from the browser-local Project Store.
- The static bundle must run from local files and simple static servers without a build step, remote service, or server-side rendering at view time.

**Project Store Host**
- The served `index.html` must contain or load the authoritative browser-local Project Store seed for the loaded project.
- The Project Store `project` section must include the repository name and current branch when Git metadata is available. Its root display label must combine those values, for example `repo @ branch`, so file-tree and trace-tree roots identify the served repository snapshot.
- The Project Store is an immutable generated snapshot for the served project unless a future requirement explicitly adds browser mutation.
- Browser interactions may keep ephemeral UI state, filters, focus, layout, and route parameters separately from the generated model snapshot.
- The Project Store must be view-neutral: the primary Model view and its Graph mode, specialist Ontologies and Traces views, plus supporting Coverage, Resources, Search, Summary, File deep-link, and Element Detail workflows read from the same normalized records instead of from page-local ad hoc JSON islands.
- Store identifiers must be stable within one served project snapshot and deterministic across repeated serve runtime generations for unchanged model content.

**Required Store Schema Sections**
- `project`: project identity, repository name when available, current branch when available, Reqvire version when available, generation timestamp policy, workspace-relative root label, and aggregate counts.
- `files`: source/document file containers keyed by repository-relative path, with display path, source route path, parent folder, child element ids, local asset links, and containment metadata.
- `folders`: virtual filesystem folder containers used by containment and file navigation.
- `resources`: modeled resource and evidence-file targets referenced by relations or contract_bindings, keyed separately from `files`.
- `elements`: normalized Reqvire elements keyed by full identifier, including name, type, canonical type family, source file path, line number when available, content summary, governance metadata, authored metadata, and source anchor.
- `relations`: normalized relation facts with source id, target id or target resource id, canonical relation direction, authored relation token, generated/opposite provenance, source location evidence, and relation family.
- `contract_bindings`: contract_bindings facts for reusable requirement-owned contract contract_bindings, local file resources, and external resources.
- `concept_refs`: concept-reference facts with source element id, target native concept element id, authored label, derived generated SKOS concept IRI, source evidence, and SKOS concept linkage.
- `thesaurus`: native concept-scheme and concept projection rows for Thesaurus route identity, navigation, map layout, source links, SKOS properties, taxonomy, related concepts, exact/close mappings, model usage, and ontology mapping usage. Thesaurus rows keep SKOS resource ids separate from native Reqvire `element_id` values.
- `submodels`: capability-rooted submodel summaries matching the `submodels` command boundary contract.
- `traces`: verification trace paths and requirement/capability trace summaries needed by the Traces view.
- `coverage`: verification and implementation coverage records, including leaf requirement status, evidence links, coverage source type, and capability roll-up summaries.
- `ontology`: ontology terms, source blocks, projection constructs, SHACL-derived slots/facets, symbol metadata, and `ontologies.ttl` artifact link.
- `knowledge_graph`: slim graph-ready node and edge ids derived from the normalized element, relation, contract_bindings, concept-reference, submodel, and resource records. Concept-reference targets are represented as ontology-role SKOS concept nodes, not concept-reference nodes. Detail evidence is read from the normalized records rather than duplicated into graph nodes.
- `search`: search documents for elements, files, resources, ontology terms, relation facts, traces, coverage records, and summaries.
- `summaries`: aggregate counts and status summaries needed by dashboards, document headers, Explorer panes, selected-detail modals, and empty states.
- `routes`: canonical route definitions for the SPA Explorer and supporting source/report workflows.

**Files Versus Resources**
- `files` are browser-local filesystem/source containers. They represent Markdown/source documents that contain model elements and are used for containment, file navigation, source links, and breadcrumbs.
- `resources` are modeled or evidence targets referenced by the model, such as implementation files, proof artifacts, linked evidence documents, external URLs, or local non-Markdown files.
- A path may appear as a `file` only when it is included as a Project Store source/document container for browsing. A path appears as a `resource` when it is referenced as evidence or a modeled target by relation, contract_bindings, or resource-report facts.
- When the same repository-relative path is both browsable and referenced as evidence, the store must preserve both identities and link them through an explicit cross-reference instead of collapsing the resource into the file container.
- Resources must retain relation evidence, referring elements, relation types, external/local classification, and availability/copy status when known.

**Relation and Trace Semantics**
- The Project Store must preserve Reqvire's current relation model and canonical directions used by diagrams and graph views.
- Capability records may author concept references and be specified by requirements; verification coverage is computed from verified requirements. Capabilities must not be treated as directly satisfied implementation units.
- Requirement records may own contracts, be constrained by semantic contracts, be satisfied by implementation/evidence, be verified, derive child requirements, and reuse compatible requirement-owned contract elements.
- Opposite/generated relation facts must remain available as evidence but must not create duplicate canonical graph edges.
- Verification traces and coverage records must be derivable from the same element and relation records used by the Model and Knowledge Graph views.

**Route Contract**
- Canonical SPA routes must use `index.html#/<view>` hash routes so the served Explorer works from local files and simple static servers.
- Required primary canonical routes are `#/thesaurus` and `#/model`, displayed in that order in shell navigation. `#/model` remains the default empty-hash route.
- Required specialist routes are `#/ontologies`, `#/traces`, and `#/coverage`, displayed after Thesaurus and Model in shell navigation; the project knowledge graph is the Graph mode of `#/model`.
- Required supporting canonical routes are `#/files`, `#/files/<path>`, `#/coverage`, `#/resources`, `#/elements/<identifier>`, and `#/search`. File routes deep-link to the Model view's List/Grid filesystem browser behavior rather than creating a separate primary Filesystem mode.
- Query-style route state may be represented after the hash, for example selected element id, search query, filters, and focused graph node.
- The default empty hash route must open the Model view unless a future requirement changes the default view.
- Primary and specialist view routes must render native SPA view modules inside the `index.html` Explorer shell from the browser-local Project Store. A route change must swap the active view module and must not leave stale containment content visible when the route id is `thesaurus`, `model`, `traces`, `ontologies`, or `coverage`.
- Native view modules must fill the full viewport between the persistent left Explorer pane and right tool rail while preserving the canonical `index.html#/<view>` browser URL.
- Route changes must update the document title and route metadata to match the active Explorer view.
- The Thesaurus route must render selected SKOS concept details from Project Store semantic concept data, not from the ontology graph canvas state. The selected concept detail must expose source navigation as a selected-concept action using that concept's source route/link metadata; it must not use a page-level source button that only focuses an ontology graph node.
- Routes must be deep-linkable: loading `index.html#/elements/<identifier>` must open the selected element inside the Explorer shell without leaving the current view family.
- Element-detail routes must render as an in-shell, scrollable modal/dialog over the active Explorer view. The modal must use Project Store element records as the primary data source and must show at minimum element name, type, source file, source anchor, metadata, governance context, content, relations, contract_bindings, inline concept-reference links, and available verification/coverage/resource evidence.
- Element-detail modal headers must show the actual element type as the single primary text badge. They must not show a second canonical family/kind badge, marker dot, shape, or glyph when the actual element type already carries the meaningful user-facing classification.
- Element-detail modal titles must use the compact detail title scale so the type badge, title, back affordance, and close affordance fit comfortably without display-sized typography.
- Element-detail modal relation navigation must maintain a local previous-element stack. Opening a related element from the modal must replace the modal content with the related element and show a compact back icon button whose accessible label and browser tooltip name the previous element.
- Element-detail modal relation and Contract Bindings sections must be expanded by default, use the shared detail disclosure pattern, and keep target activation inside the modal navigation stack.
- Element-detail modal headers must not render a second visible previous-element context line such as `From:` when the back button already carries the previous element name through its accessible label/title.
- Element-detail and source-page rendering for ordinary model elements must not add a synthetic `Content` section heading. The element body is the default prose section because Reqvire source Markdown has no authored `Content` subsection. Real authored headings such as `Details`, and concept-specific semantic headings such as `Definition`, remain visible.
- Element-detail modal and source-page concept references must resolve to native `concept` elements through Project Store `target_element_id`, render matching prose terms as inline concept links, keep the derived SKOS concept IRI as tooltip/location metadata, and open the native concept element modal. Source-page concept references are the exception to source-page navigation: activating them opens the concept modal rather than replacing the main pane with the concept source page. They must not render as a separate `Concept References` detail section and must not route through ontology-node fallback behavior.
- Element-detail modals must provide a secondary source action that opens the served source route and fragment when exact source browsing is needed. The source action must not be the primary navigation target for graph, search, containment, or list element clicks.
- Closing the element-detail modal must return to the underlying Explorer route and preserve view context such as graph focus, filters, and search state when feasible.
- Opening or closing element-detail and ontology-detail modals must not tear down or rebuild long-lived graph/map renderers behind the modal unless the renderer's own graph data or configuration changed. Sigma, React Flow, Mermaid, and other canvas/SVG renderer lifecycle effects must depend on renderer data/configuration only; route, modal, selection, and open callbacks must be read through stable latest-value refs at event time.
- Source-document element links must remain available as secondary source browsing destinations, but normal Explorer element navigation must prefer `index.html#/elements/<identifier>`.
- When the active main route is a source content page, the left model tree acts as source navigation: selecting a file opens that file's source content route, and selecting an element opens that element's source route and fragment in the main pane instead of opening an element-detail modal. Outside source browsing, model tree element selection must continue to open the element-detail modal.
- Model tree folder rows and expandable file rows must toggle expansion when selected from the row, not only from the twist affordance.
- Source content routes for modeled Markdown files must render modeled elements through the shared detail-body product patterns rather than raw Markdown sections. They must promote metadata/governance into the same badge strip used by element modals, render content with inline native concept links, strip only canonical reserved detail subsections (`Concept References` and `Contract Bindings`) from prose rendering, and collapse relation and Contract Bindings sections by default. Legacy headings such as `Reused Contract Context` must be rejected by core validation or repaired by migration, not handled as Explorer display fallbacks.
- Source content routes with element fragments must scroll to the matching modeled source element after the shared detail-body rendering has mounted, so `#/content/<file>.md#<element-fragment>` deep links and left-tree source navigation land on the requested element.
- Source content route element titles must use a compact document title scale smaller than display headings while still remaining visually primary within each source element block.
- Source-page relation and Contract Bindings target activation must stay in source browsing context: clicking a target on a source page opens that target's served source route and fragment, while clicking a target inside an element modal opens that target inside the modal. Source-page concept-reference links are excluded from this rule and must open the native concept modal.

**SPA View Compatibility Policy**
- Explorer views are SPA routes under `index.html`; separate Explorer/report document entry points must not be generated.
- Old Explorer page URLs must not be emitted as separate route outputs. Compatibility, when explicitly required by deployment, must be handled outside the served Explorer bundle and must not introduce separate Explorer UI implementations or data models.
- Source/document content must be rendered by the Explorer content route from Project Store Markdown records; normal Explorer navigation must target canonical hash routes.

**Browser-Local Virtual Filesystem Semantics**
- The store must expose a virtual filesystem tree built from `folders` and `files` records.
- The virtual filesystem root must use the Project Store root label, preferring `repository @ branch` when Git metadata is available.
- Folder/file containment is physical repository/source organization only; it does not define logical capability, requirement, ontology, or verification ownership.
- Element detail views must resolve source file containers through element source metadata, not by inferring model ownership from folders.
- Resource navigation must use `resources` records and referring facts; it must not imply that evidence files contain model elements unless a corresponding `files` record exists.
- The Model view's List and Grid modes must render native read-only file-manager/model-browser views from the Project Store virtual filesystem, including breadcrumb navigation, sortable list columns, grid cards, search across folders/files/modeled elements, color/icon legends, file selection, source-page secondary actions, and modeled-element rows that open the shared Explorer element-detail modal. In Grid mode, the full folder/file card surface must be the primary open/select target; opening must not be limited to the title text.
- Model tree rows, grid cards, modeled-element lists, and element legends must use the shared Explorer `ElementIcon` type glyph system. Semantic-contract elements must use their own SHACL-profile color as a plain square with no glyph; contract-family elements must retain the shared contract hue while using distinct subtype glyph marks for `source`, `specification`, `constraint`, `behavior`, `state`, and `input-output`, so users can distinguish element families and contract subtypes without relying on text labels alone.
- The Model project tree must share one Model selection state with the active Model workspace. Selecting a folder, file, or modeled element in the left tree must drive the current List, Grid, or Graph workspace selection/focus rather than navigating to an unrelated middle-pane route.
- The supporting `#/files/<path>` route must deep-link into the same Model List/Grid filesystem browser behavior for a selected file or folder; it must not introduce a separate primary Filesystem mode.
- Model List/Grid must use the Explorer design-system shell and styling contract; it must not import a third-party file-manager stylesheet or mount an external file-manager widget that visually diverges from the Explorer application.

**Search and Detail Semantics**
- Search documents must include enough normalized ids to route to element detail, file detail, resource detail, ontology term detail, trace detail, or coverage detail without rebuilding view-local indexes from HTML text.
- Search must render as a full-width result-list workspace between the left Explorer pane and right tool rail. Search results must carry the canonical route/detail information directly without requiring a separate detail pane.
- The Search left-pane controls must expose reset and result-type filter controls directly. They must not render a second passive legend for the same result-type swatches when the filter controls already carry the visible type colors and labels.
- Element detail must show source, content, governance, metadata, relations, contract_bindings, inline concept-reference links, verification traces, coverage status, ontology context, and resource evidence from the Project Store in a scrollable route-backed modal.
- Element detail must retain a direct source route link using the element's served source route and anchor so users can inspect the source content without making that content route the primary Explorer destination.
- Store consumers must tolerate unknown future fields and must reject or visibly diagnose a missing required store seed.

#### Metadata
  * type: specification

#### Relations
  * define: [SPA Explorer Shell and Project Store](Capabilities.md#spa-explorer-shell-and-project-store)
---

### Serve Command Contract Specification

#### Details
Serve command behavior:
- Accept `--host <HOST>` option to specify the bind address (default: localhost)
- Accept `--port <PORT>` option to specify the server port (default: 8080)
- Accept `--enable-mcp` to also expose the Reqvire MCP Streamable HTTP endpoint at `/mcp` on the same HTTP listener.
- Accept `--enable-mutations` only when `--enable-mcp` is present, and use it to enable mutation tools for the embedded MCP endpoint.
- Assemble the embedded Explorer shell, Project Store data, and ontology artifact in memory
- Serve `assets/project-store.js` and `ontologies.ttl` from the materialized in-memory runtime assets. Browser refreshes and direct HTTP GET/HEAD requests for those assets must not parse, validate, or regenerate model data from disk.
- Refresh the materialized runtime Project Store data and ontology artifact after successful embedded MCP write mutations, so subsequent Explorer reloads observe MCP-authored model changes without making ordinary browser refresh the regeneration trigger.
- Serialize embedded MCP write mutation execution and runtime asset refresh so the served runtime store is refreshed only after the mutation has completed and never from a partial filesystem update.
- Populate Project Store source-file records from modeled element source files and existing graph-referenced local implementation/evidence/resource files, without using generated Markdown files on disk as an intermediate runtime artifact
- Keep relation-backed implementation/evidence/source targets as Project Store resources for relation semantics, and include only existing repository-relative local targets in the Model tree file hierarchy
- Start an HTTP server serving embedded Explorer assets and generated runtime data
- Serve existing repository-relative local static asset files, including images and documents referenced from Markdown content, from their repository-relative request paths while rejecting absolute paths, parent-directory traversal, and unsupported asset extensions
- Serve `index.html` for the root URL so the SPA Explorer shell is the default entry point
- Return `index.html` for non-asset browser routes so SPA navigation can handle deep links
- Preserve `/mcp` as an MCP endpoint when embedded MCP is enabled; SPA fallback routing must not intercept MCP protocol requests.
- Display clickable server URL for user to open in browser
- Display the `/mcp` endpoint URL when embedded MCP is enabled.
- Display instructions to press Ctrl-C to stop server
- Continue serving until terminated by the user (Ctrl-C)

#### Metadata
  * type: specification

#### Relations
  * define: [Serve Command](Capabilities.md#serve-command)
---

### Thesaurus View Generation Contract Specification

#### Details
Thesaurus view generation behavior:
- Renders the canonical `#/thesaurus` route from the Project Store `thesaurus` projection, not from ontology graph canvas state or ontology graph provenance.
- Treats native `concept-scheme` elements as standalone roots and native `concept` elements as SKOS resources generated from Markdown source.
- Uses the left Explorer pane for concept-scheme navigation, concept filtering, and compact summary counts instead of the Model filesystem tree.
- Shows concept-scheme and concept glyphs from the design system, with distinct native concept and concept-scheme color roles.
- Builds the concept-scheme navigator and Map workspace only from the Project Store `thesaurus.schemes[]` and `thesaurus.concepts[]` projection. Concept scheme and concept rows must carry separate SKOS identity (`id`) and native Reqvire element identity (`element_id`), and concept rows must carry canonical `scheme_id`, `scheme_element_id`, `scheme_label`, `parent_id`, normalized child/narrower concept ids, normalized related concept ids, normalized mapping ids, model usage, ontology mapping usage, and source navigation metadata.
- Must not group Thesaurus concepts by source element name, ontology document IRI, namespace, filesystem path, ontology graph node source/provenance, or element-hierarchy fallback when canonical `thesaurus` projection data is absent. Missing required projection data is a store contract problem, not a UI fallback opportunity.
- Populates concept scheme membership from generated native concept data equivalent to `skos:inScheme`, `skos:topConceptOf`, or scheme-owned `skos:hasTopConcept`; broader/narrower hierarchy remains separate normalized taxonomy evidence and must not redefine scheme ownership.
- Presents selected concept details using generated SKOS fields from the normalized concept projection: the modal title carries the preferred label derived from the element name; the body promotes definition from main body content, alternative labels, scope notes, examples, broader/narrower hierarchy, related concepts, exact/close mapping relations, and ontology bridge mappings. These relation lists must exclude the selected concept itself and deduplicate reciprocal graph edges before rendering.
- Shows ontology bridge context as usage evidence: ontology terms that author `reqvire:mapsToConcept` to the selected native concept appear as mapped ontology terms without making the concept an ontology child.
- Provides a single scheme-scoped Map workspace for the selected concept's scheme; Thesaurus must not expose a Browse/Map segmented mode control.
- Renders Thesaurus Map through the shared design-system product pattern using React Flow for node interaction and deterministic concept-scheme mind-map layout. The map canvas must be transparent, full-bleed, and full-height over the available workspace surface; concept-scheme nodes render as compact boxed anchors, concept nodes render as boxed label-first mind-map entries without role labels, and taxonomy edges render as colored curved branches. Concept nodes with `related` concept associations expose a compact focus affordance that isolates the selected concept and its related concepts around the focused node, hides unrelated map nodes and edges, centers the focus without changing zoom, and restores the full map when the user clicks the canvas background.
- React Flow node activation callbacks for the Thesaurus Map must follow the shared renderer-boundary callback rule: renderer/map lifecycle state is keyed by concepts, layout, focus, and selection data, while open/select callbacks are read through latest-value refs so parent route re-renders do not rebuild the map.
- Selecting a concept from the left Thesaurus navigator must keep the active scheme Map view open and center the matching map node in the workspace without changing the user's current map zoom level.
- Opens the shared element-detail modal using `thesaurus.schemes[].element_id` for concept-scheme map nodes and `thesaurus.concepts[].element_id` for concept map nodes. Activating a Thesaurus map node must never open an ontology graph node merely because ontology graph provenance or mapping data names the same SKOS IRI. Concept elements must use a SKOS-optimized modal body that keeps the standard element modal frame but promotes definition, scheme, alternative labels, scope note, examples, top concepts, broader/narrower/related concepts, exact/close matches, mapped ontology terms, and model usage before `Authored relations`, which contains remaining Reqvire Markdown relation evidence. The relation lists shown in this modal must not contain the selected concept itself or circular duplicate endpoints. The Thesaurus route must not create a separate concept-specific modal frame for this.
- Keeps source navigation anchored to the Markdown `concept-scheme` or `concept` element.

#### Metadata
  * type: specification

#### Relations
  * define: [Thesaurus View Generation](Capabilities.md#thesaurus-view-generation)
---

### Web Interface Style Specification

Styling conventions for the served Explorer web interface.

#### Details
**Page Layout:**
- No top header in the native Explorer application
- Optional on-demand help modal for the current view
- Primary Explorer graph/report views use one shared application shell: collapsible persistent full-height left Explorer pane with a vertical `Explorer` edge strip, full-height shared canvas/content workspace, route/detail modals when inspection needs more than a compact pane summary, compact bottom summary strip when needed, and a narrow right vertical tool rail.
- The right vertical tool rail provides compact icon access to Search, Model, Ontologies, Traces, Settings, and Help so specialist views can return to Model without requiring the left primary switcher.
- The left Explorer pane does not render primary view links. Model and file drill-in routes show Model mode controls followed by the shared project tree. Specialist views such as Traces, Ontologies, and Coverage start with that specialist view's controls or domain navigator and must not render the shared project file tree unless that specialist view explicitly defines its own hierarchical navigator.
- Search left-pane controls use their result-type filter buttons as the visible type key and must not duplicate those same colors in a separate passive legend.
- Ontologies must not render the shared project file tree in the left Explorer pane; the Ontologies left pane is reserved for ontology reset, active visibility filters, passive type/color key, and passive notation legend.
- Coverage must render compact route-level counts in the left Explorer pane bottom summary, while the central Coverage workspace owns KPI cards, coverage breakdowns, and gap cards. Header stat rows must not duplicate counts that are already present in left-pane navigation rows or bottom summaries.
- View controls use compact selected-control tokens and shared muted hover surfaces; per-view controls belong at the top of the expanded left Explorer pane or in a detail modal when they are inspection-specific, not in a top header or floating boxed toolbar.
- View modules must not define page-local Explorer-pane variants, hidden navigation alternates, or route-local sidebars. Shell chrome, edge strips, and collapse state are owned by the shared Explorer shell components.
- Views with contextual search, evidence, properties, or selection details must use the left Explorer pane for compact selected-item links and shared modal components for full detail. They must not mount a separate route-local sidebar, even when embedding a committed renderer such as the ontology Sigma renderer.
- Graph/canvas/list/grid content should not be occluded by floating toolbars; controls that affect the current view should live in the left Explorer pane unless they belong inside a focused detail modal.
- Model workspace breadcrumbs, list/grid content, and graph canvases start after the persistent left pane plus a small gutter and end before the right tool rail.
- Programmatic graph/map renderers are long-lived view infrastructure. Renderer mount effects must not depend directly on unstable UI callback props; use the Explorer design-system latest-ref helper for event callbacks and keep renderer teardown tied to graph data/configuration changes.

**Typography:**
- Use the local Geist text font and Geist Mono for code, identifiers, file paths, and RDF/Turtle fragments.
- Heading hierarchy follows document structure while route chrome and compact panels use the Explorer design-system type scale.
- Long identifiers use wrapping, truncation, or copy affordances rather than forcing modal or pane overflow.

**Theme and Color Tokens:**
- Use semantic design-system tokens rather than primitive color names or route-local literal colors.
- Warm-neutral surface tokens define the shared application background, left Explorer pane, right tool rail, modal body, source content, graph canvas, hover rows, and selected rows.
- Text tokens define primary, secondary, muted, inverse, link, and code text. Static labels use muted text tokens; accent text is reserved for links, focus, active state, and deliberate product emphasis.
- Border, shadow, radius, spacing, control-height, and transition tokens come from the Explorer design system and must not be redefined per route.
- Programmatic renderers resolve graph, Mermaid, and badge colors through the Explorer palette API.

**Element Cards and Badges:**
- Element cards, grid tiles, list rows, search results, graph nodes, relation pills, and modal badges use the same role tokens. Element icons and legends carry the glyph contract; modal and adjacent type badges stay text-only when a separate element marker already identifies the element type.
- Capability, requirement, verification-objective, concrete verification, ontology, resource, and contract-family roles are encoded by role token, text label, and glyph. Color is never the only type cue.
- Semantic-contract elements use their own SHACL-profile role token as a plain square with no glyph. Contract-family subtypes use a shared contract hue with distinct glyphs for source, specification, constraint, behavior, state, and input-output elements.

**Navigation:**
- Breadcrumb trail for element hierarchy
- Clickable relation links
- Collapsible sections for long content
- No generated footer attribution in served Explorer pages

#### Metadata
  * type: specification
---
