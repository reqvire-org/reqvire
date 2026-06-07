# Elements

### Component Reuse Verification

Test verifies components are reused across pages without duplication.

#### Details
**Test Procedure:**
1. Analyze generated HTML files
2. Verify navigation component appears identically in all pages
3. Count instances of duplicated code blocks
4. Measure total generated HTML size

**Pass Criteria:**
- Navigation HTML identical across all pages (except nav_prefix)
- Zero duplication of CSS styles across pages (external stylesheet)
- Total generated HTML size remains bounded by sharing compiled Explorer assets and reusable source-page layout components
- Source code organized in component modules

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Component-Based HTML Architecture](../HTMLGeneration.md#component-based-html-architecture)
---

### HTML Validity Verification

Test verifies generated HTML is valid and well-formed.

#### Details
**Test Procedure:**
1. Generate all HTML pages
2. Validate HTML using W3C validator or similar
3. Check for:
   - Proper DOCTYPE declaration
   - Valid HTML5 structure
   - Closed tags
   - Valid attributes
   - No duplicate IDs

**Pass Criteria:**
- All pages pass W3C HTML5 validation
- No HTML parsing errors
- Maud-generated HTML is well-formed

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-html-generation/test.sh)
  * verify: [Type-Safe HTML Generation](../HTMLGeneration.md#type-safe-html-generation)
---

### Integration Test Verification

Test verifies the primary SPA Explorer shell and supporting generated source/specification artifacts work correctly together.

#### Details
**Test Procedure:**
1. Generate the SPA Explorer and source/specification pages: `index.html`, compiled SPA assets, `ontologies.ttl`, and specification/source files. Do not generate standalone Explorer/report HTML entry points.
2. Verify each page contains:
   - Correct title
   - `index.html` as the primary SPA Explorer shell with a browser-local Project Store seed
   - Project Store sections for project, folders, files, resources, elements, relations, attachments, concept references, submodels, traces, coverage, ontology, knowledge graph, search, summaries, and routes
   - Hash route declarations for the current Explorer routes, including the primary Model route, right-tool Knowledge Graph/Ontologies/Traces/KN2 routes, file deep links, element detail, and search detail routes
   - Element-detail route/modal support so Explorer element links open `index.html#/elements/<identifier>` in a scrollable in-shell modal, with source-page anchors available only as secondary actions
   - No separate Explorer implementations or route adapters in retired standalone Explorer/report HTML entry points
   - The expanded left Explorer pane starts with active-view controls beside the persistent vertical `Explorer` edge strip and does not render primary left-pane links for retired Containment or Filesystem routes
   - Right vertical tool rail exposes specialist view icons for Knowledge Graph, Ontologies, Traces, and KN2
   - The native Explorer shell does not render a top header; global tool actions are exposed through the right vertical tool rail and shared right `Inspector` lane
   - Model mode controls and right-rail links target canonical `index.html#/<route>` hash routes, not retired standalone page targets
   - Canonical hash routes render native SPA view modules inside the `index.html` shell rather than leaving stale containment content visible; native view modules fill the available viewport between the left Explorer pane/strip and the right `Inspector` lane/tool rail and update route metadata/title for the active view
   - `index.html` references compiled Vite/React bundle assets and a compiled Tailwind stylesheet as local static assets, with no CDN-loaded framework, no CDN-loaded Tailwind, and no runtime Tailwind compiler script
   - No top-level Ontologies, Traces, KN2, TraceFlow, Coverage, or Resources links in the primary left-pane Explorer controls
   - No generated attribution footer
   - Right tool-rail help control and modal script
   - No static first-viewport title/prose block on primary Explorer pages
   - Model route exposes List, Grid, Sunburst, and Icicle modes; the separate Knowledge Graph route contains project element/fact graph data from the parsed model, including element nodes, relation facts, inspector evidence, resolved graph edge endpoints, and direct viewport sizing for a nonblank Sigma.js WebGL canvas
   - Knowledge Graph route graph data and renderer preserve the four primary system-model layers (ontologies, capabilities, requirements, and verifications) while treating requirement-owned refinements as subordinate requirement detail/contract nodes and custom/resource targets as supporting fact nodes
   - Knowledge Graph route graph data includes Reqvire root submodels, and the KN2/Cytoscape renderer is reachable through the SPA route table and seeds structural subgraph coloring from those root submodels rather than from attachment, concept-reference, verification, satisfaction, or trace overlays
   - Knowledge Graph route defaults resource targets off, normalizes exported node `type` values before filtering, and uses Graphology ForceAtlas2 with size-aware settings so the initial project graph is readable rather than blank or fully overlapped
   - KN2/Cytoscape defaults its structural relation set to `derive`, `specify`, and `refine`; verification/satisfaction, trace, attachments, and concept references are opt-in overlays that can be shown without changing submodel layout or cluster detection
   - Knowledge Graph route implements focus-neighborhood relation rendering so relation edges are hidden by default, direct incident edges appear on node rollover or click, clicked focus persists after pointer rollout, hover focus is additive with clicked focus for neighborhood exploration, empty-stage click clears focus, and unconnected nodes dim through a deterministic 20% canvas-background blend without altering search/filter state or using glow/highlight effects
   - Knowledge Graph route renders canonical Mermaid relation directions only, so opposite relation pairs such as `specify`/`specifiedBy` and `derivedFrom`/`derive` do not appear as duplicate parallel graph edges
   - Knowledge Graph route search results use graph role-color swatches, omit repeated parenthesized type suffixes, and center selected result nodes in the graph viewport
   - Knowledge Graph route inspector colors the selected element kind badge with the same role color used by the graph and legend, and renders long identifier/source facts as stacked field rows
   - Knowledge Graph route legend entries, search swatches, graph node fills, and inspector kind badges follow the same saturated role colors, with darker role borders reserved as subtle accents
   - `index.html#/knowledge-graph` exposes the project Knowledge Graph without a separate standalone Explorer page
   - Primary Explorer graph/report views share the same shell geometry: collapsible persistent left Explorer pane with vertical `Explorer` edge strip, expanded active-view controls, project tree only where that view owns one, central near-white workspace, collapsible 390px right search/inspector lane with vertical `Inspector` edge strip, neutral compact inspector headings, compact muted bottom summary strip when needed, and right vertical tool rail
   - Search, evidence, properties, selection, and inspector content is mounted inside the shared right `Inspector` lane for Model, file deep links, Knowledge Graph, Ontologies, Traces, KN2, Coverage, and Resources; committed renderers shall not retain a separate route-local sidebar outside the `Inspector` strip
   - Model route renders native List, Grid, Sunburst, and Icicle modes; Sunburst/Icicle use D3 partition renderers with click-to-drill/zoom; file-manager and D3 breadcrumbs align to the same usable lane after the left Explorer pane/strip and before the inspector/tool rail
   - Mermaid diagrams are sized per rendered diagram, not by a fixed full-viewport height applied to every graph on Model or Traces routes/source outputs
   - Mermaid initializer script is included once per generated source/report page that contains Mermaid diagrams
   - Expected content sections
3. Test relative links:
   - From root: nav_prefix = ""
   - From requirements/: nav_prefix = "../"
   - From requirements/System/: nav_prefix = "../../"
4. Verify visualizations load:
   - Mermaid diagrams render
   - D3 Sunburst/Icicle renders
   - D3 Sankey renders
5. Test navigation across generated source artifacts and canonical Explorer hash routes

**Pass Criteria:**
- All expected generated artifacts are emitted without errors
- `index.html` contains a valid Project Store seed before routed views render
- Current hash routes render canonical SPA view content without standalone Explorer adapters
- Generated source/specification artifacts contain required content sections
- Relative links resolve correctly from all nesting levels
- All visualizations load and display properly
- Explorer route links function from generated source/specification artifacts
- Source/specification artifact pages may still generate, but standalone Explorer/report HTML entry points are not emitted as shell destinations
- Static help content is available on demand through the help modal rather than consuming page space

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-html-generation/test.sh)
  * verify: [Component-Based HTML Architecture](../HTMLGeneration.md#component-based-html-architecture)
  * verify: [Responsive HTML Generation](../HTMLGeneration.md#responsive-html-generation)
---

### Mobile Responsiveness Verification

Test verifies HTML documentation is usable on mobile devices.

#### Details
**Test Procedure:**
1. Generate the Explorer SPA and supporting source/specification artifacts (`index.html`, compiled assets, `ontologies.ttl`, and at least one exported specification/source file)
2. Test on Chrome DevTools device emulation:
   - iPhone SE (375px)
   - iPad (768px)
   - Desktop (1920px)
3. Verify compact Explorer shell chrome:
   - Desktop: The expanded left Explorer pane exposes current-view controls without a top header or primary left view links
   - Desktop: Views with contextual right evidence expose a vertical `Inspector` edge strip that collapses and expands the shared 390px right `Inspector` lane without changing per-view geometry
   - Mobile: Left Explorer pane/strip, right Inspector strip, and right tool rail remain compact and usable without a top header
4. Verify content readability:
   - No horizontal scrolling
   - Text legible without zooming
   - Touch targets ≥ 44px

**Pass Criteria:**
- All pages render without horizontal scroll on all viewport sizes
- Headerless Explorer shell remains usable without horizontal page overflow
- All interactive elements accessible via touch on mobile

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-html-generation/test.sh)
  * verify: [Mobile-Friendly Documentation](../HTMLGeneration.md#mobile-friendly-documentation)
  * verify: [Responsive HTML Generation](../HTMLGeneration.md#responsive-html-generation)
---

### Responsive Design Verification

Test verifies responsive breakpoints and compiled Tailwind CSS integration in the static SPA bundle.

#### Details
**Test Procedure:**
1. Generate the exported SPA shell and pages
2. Take screenshots at breakpoints: 320px, 640px, 768px, 1024px, 1920px
3. Compare screenshots to expected layouts (visual regression)
4. Verify compiled CSS classes applied correctly:
   - Mobile-only classes (md:hidden)
   - Desktop-only classes (hidden md:flex)
   - Responsive spacing (px-4 md:px-8)

**Pass Criteria:**
- Screenshots match expected layouts at all breakpoints
- No layout breaks or overlapping elements
- Compiled Tailwind utility classes present in the generated SPA assets, served from a local compiled stylesheet rather than a CDN or runtime compiler
- Custom Reqvire theme colors applied through the build-time Tailwind config

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-html-generation/test.sh)
  * verify: [CSS Framework Integration](../HTMLGeneration.md#css-framework-integration)
  * verify: [Responsive HTML Generation](../HTMLGeneration.md#responsive-html-generation)
---
