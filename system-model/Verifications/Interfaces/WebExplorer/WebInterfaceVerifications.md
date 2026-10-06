# Elements

### Web Explorer Interface Verification Objective

This objective groups verification that the served Web Explorer renders model data, assets, diagrams, navigation, responsive layouts, and export flows correctly.

#### Metadata
  * type: verification-objective

#### Relations
  * derive: [Component Reuse Verification](#component-reuse-verification)
  * derive: [Contract Bindings Link Serving Verification](#contract-bindings-link-serving-verification)
  * derive: [Diagram Contract Bindings Display Verification](#diagram-contract-bindings-display-verification)
  * derive: [Element Detail Inline Concept Reference Verification](#element-detail-inline-concept-reference-verification)
  * derive: [Explorer Automatic Store Refresh Verification](#explorer-automatic-store-refresh-verification)
  * derive: [Explorer Serve Verification](#explorer-serve-verification)
  * derive: [Export Command Verification](#export-command-verification)
  * derive: [Mobile Responsiveness Verification](#mobile-responsiveness-verification)
  * derive: [Model Containment Contract Bindings Links Verification](#model-containment-contract-bindings-links-verification)
  * derive: [Model View Element Navigation Test](#model-view-element-navigation-test)
  * derive: [Ontology Model Viewer Analysis Verification](#ontology-model-viewer-analysis-verification)
  * derive: [Responsive Design Verification](#responsive-design-verification)
  * derive: [Serve Command Verification](#serve-command-verification)
  * derive: [SPA Explorer Store Contract Verification](#spa-explorer-store-contract-verification)
  * derive: [Thesaurus Project Store Projection Verification](#thesaurus-project-store-projection-verification)
---

### Component Reuse Verification

This analysis verifies Explorer components are reused across routes without duplicated route-local renderers.

#### Details
Expected checks:
- `index.html` is the single Explorer SPA shell entry point.
- Browser chrome is implemented by shared Explorer components.
- Separate Explorer/report document entry points are not emitted.
- Source code is organized in reusable route and shell modules.

#### Metadata
  * type: analysis-verification

#### Relations
  * verify: [Component-Based Explorer Architecture](../../../Interfaces/WebExplorer/ExplorerRendering.md#component-based-explorer-architecture)
---

### Contract Bindings Link Serving Verification

This test verifies that the served Explorer preserves contract_bindings identifier links to referenced contract elements.

#### Details

##### Acceptance Criteria:
- System shall preserve all contract-identifier contract_bindings referenced by elements
- Contract Bindings identifier links shall resolve to referenced contract elements in Explorer content and element detail workflows
- Duplicate contract_bindings (same contract referenced multiple times) shall be processed consistently

##### Test Criteria:
- Create model with elements having contract_bindings
- Run the Explorer through the serve workflow or a Project Store fixture
- Verify contract_bindings links resolve to contract element records and source anchors
- Verify identifier targets are navigable from rendered content routes and element modals

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Contract Bindings Link Serving](../../../Interfaces/WebExplorer/Capabilities.md#contract-bindings-link-serving)
---

### Diagram Contract Bindings Display Verification

This test verifies that diagrams display contract_bindings links within element boxes.

#### Details

##### Acceptance Criteria:
- Element boxes in diagrams shall include bound contract element names
- Contract Bindings shall be prefixed with paperclip icon (📎)
- Contract Bindings shall appear below element name using line breaks
- Contract Bindings display shall not break diagram rendering
- Model and Traces diagram labels shall not expose full `file#fragment` contract_bindings identifiers as visible node text

##### Test Criteria:
- Create model with element having contract_bindings
- Generate diagram (format or model command)
- Verify Mermaid output contains multiline labels with contract_bindings
- Verify bound contract element names appear with 📎 prefix
- Verify Model route/source content and Traces route data use compact contract_bindings labels and still render Mermaid containers for the final graph where Mermaid output is present
- Verify diagram renders correctly with contract_bindings labels

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Diagram Contract Bindings Display](../../../Interfaces/WebExplorer/Capabilities.md#diagram-contract-bindings-display)
---

### Element Detail Inline Concept Reference Verification

This component test verifies that regular element-detail modals render authored concept references as inline native concept links instead of as a separate source subsection or ontology-node fallback.

#### Details

##### Acceptance Criteria:
- The regular element-detail modal shall hide the authored `#### Concept References` source subsection from rendered body content.
- Referenced native SKOS concepts shall be matched in prose by preferred label, alternative labels, and authored reference label.
- Matching prose terms shall render as inline links using the standard Explorer link color with no resting background and underline only on hover or focus, rather than as badges, glyphs, pills, or a separate Concept References section.
- Activating an inline concept-reference link shall open the referenced native `concept` element modal.
- Authored model concept references shall not open ontology-node modals.

##### Test Criteria:
- Render an element-detail modal for an element with a native concept reference whose label appears in the main body.
- Assert no `Concept References` section heading or raw concept IRI appears in the rendered modal body.
- Assert the matching prose term is rendered as an inline concept-reference control.
- Click the inline concept-reference control and assert the native concept element identifier is opened.
- Render an element-detail modal where the body uses an alternative label for the referenced native concept.
- Assert the alternative-label prose term opens the same native concept element.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [ElementDetailModal.test.tsx](../../../../explorer/src/components/ElementDetailModal.test.tsx)
  * verify: [SPA Explorer Shell and Project Store](../../../Interfaces/WebExplorer/Capabilities.md#spa-explorer-shell-and-project-store)
---

### Explorer Automatic Store Refresh Verification

Verify automatic refresh through the compiled Explorer in a real browser rather than only inspecting server responses.

#### Details

##### Acceptance Criteria
- An already-open file view served by mutation-enabled embedded MCP adopts successful MCP additions and edits without a page reload.
- Automatic refresh retains the current hash route, open element modal, shell layout state, and authored model-tree filter when their targets still exist.
- Automatic refresh updates source content and search results for changed records.
- The compiled browser downloads exactly the missing chunk hashes after a mutation and publishes a complete store equal to the server snapshot without requesting the full-store API.
- Refreshed immutable stores render Coverage, Traces, Ontologies, and Model Graph views without modifying their cached records or reloading the document.
- Rerender a trace with the same verification identifier but changed labels, requirement membership, and roll-up edges; repeat across worktrees sharing identifiers. Assert the current graph replaces the old graph, deferred obsolete renders cannot overwrite it, and file overviews do not prepare or lay out unselected traces. Same-context refresh preserves direction/disclosure/focus; a pending replacement context cannot retain the previous worktree's map.
- Consume a normalized reconvergent trace graph in the real report view and assert each node renders once with every incoming/outgoing relation, direct-verification links, and owning capability context. Compare CLI, exported, and served trace projections in the existing integration suites.
- A real MCP mutation between manifest and chunk retrieval produces `409`, followed by recovery to the latest complete snapshot.
- Missing and corrupt chunk responses leave the prior displayed content and committed revision intact, show a failure diagnostic, and recover automatically when valid responses resume.
- The shell has no manual Refresh action.
- Hidden documents suspend automatic refresh and check again when visible, catching up across several missed MCP mutations and a deletion.
- Plain and read-only embedded serving load selected worktree contexts on demand under existing cache/freshness rules without adding periodic refresh or preloading other branches; static exports issue no live API requests.
- Unit checks enforce an immediate visible check and the five-second interval, prevent overlapping checks, release timed-out checks for automatic retry, and discard late responses after hiding, unmount, or StrictMode cleanup.
- Client transactions preserve unchanged object identity, ordered current arrays, and unknown sections; deletion-only refreshes require no new chunks and remove obsolete cache entries.
- On repeated one-record updates, deep-freeze traversal must not revisit unchanged record subtrees already frozen by that client. New records and descendants of shallow-frozen seed objects become immutable; abandoned preparations and wrong-context responses leave the committed cache and revision intact.
- Replace only coverage/report sections and confirm element lookup, shared trace grouping, and prepared Model Flow identities are retained. Repeated Flow selections must reuse topology preparation while preserving roots, directed paths, splits, merges, and scope exclusions. Count preparation/input reads rather than asserting elapsed time. Confirm unused trace/Flow data stays unprepared, changed labels/types/files/relations/dependencies update results, and switching workspace/worktree identity rebuilds derived data even for reused identifiers and input objects.
- Confirm inactive views do not prepare project/file trees, repeated consumers reuse each projection, and report-only or Git status/revision updates do not reread their source hierarchy. Check file additions/removals/moves, resource paths and external resources, empty published folders, root/worktree grouping labels, nested worktree prefixes, and context isolation. Keep a tree filter and selection active through an element rename/type change and verify current results, counts, navigation, and source records without rebuilding unchanged file topology.
- Count search worker creations/builds across report-only updates, unrelated element/resource metadata changes, and equivalent replacement search records; retain the ready index and outstanding searches. Change searchable identifiers/kinds/titles/routes/text, membership/order, file/resource classification, and concrete element types, then assert current search results and filtering. Switch workspace/worktree identity with reused input objects and require a fresh worker. Reject retired requests and ignore saved late ready/error/result callbacks; cancel scheduled builds on replacement/unmount and exercise StrictMode cleanup. Preserve existing ranked-search assertions.
- Malformed or unsupported manifests/stores, incorrect chunk hashes or response membership/revisions, exhausted conflicts, and interrupted multi-batch downloads cannot advance the committed cursor or publish partial data. Subsequent valid responses recover from the last committed revision.
- Invalid seed advertisements trigger complete chunk retrieval, and content verification succeeds without Web Crypto.

##### Test Criteria
- Use the existing suite's headless Chromium/CDP conventions and the served compiled bundle.
- Wait for observable UI changes with bounded deadlines and assert the existing document remains loaded.
- Assert route, modal content, shell state, source view, search results, visibility transitions, and static export behavior explicitly.
- Record the compiled browser's real manifest/chunk requests and compare requested hashes with the complete manifest's missing hashes; compare its published store and revision with the server's full snapshot.
- Hold a chunk request at the transport boundary while a second real MCP mutation completes, then require the actual server `409` and latest-store recovery.
- Inject one missing or corrupted chunk response through the browser transport wrapper and assert retained content/revision, visible diagnostics, and the later complete valid store. Keep production assembly and UI recovery in the compiled bundle.
- While hidden, execute several MCP edits and a deletion, wait through periodic checks, and assert no live requests until the actual visibility transition resumes refresh.
- Run `manifestRefresh.test.ts` and `useLiveStore.test.ts` with Vitest for five-second timing, timeout/cancellation ownership, immutable cache reuse and pruning, validation failures, multi-batch interruption/conflicts, and cursor rollback. GitHub Actions runs this same Explorer unit suite and the standard headless shell E2E suite.
- Count descendant enumeration and freezing of reused records across successive refreshes at different fixture sizes; check nested mutation rejection and retained identities rather than timing thresholds. Keep full structural validation and existing corrupted-chunk, cancellation, and context-isolation checks active.
- Compare all named checks with the expected refresh output file.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Web Explorer Interface Verification Objective](#web-explorer-interface-verification-objective)
  * satisfiedBy: [manifestRefresh.test.ts](../../../../explorer/src/store/manifestRefresh.test.ts)
  * satisfiedBy: [StoreContext.test.tsx](../../../../explorer/src/store/StoreContext.test.tsx)
  * satisfiedBy: [FilesView.test.tsx](../../../../explorer/src/views/FilesView.test.tsx)
  * satisfiedBy: [SearchIndexContext.test.tsx](../../../../explorer/src/search/SearchIndexContext.test.tsx)
  * satisfiedBy: [TracesView.test.tsx](../../../../explorer/src/views/TracesView.test.tsx)
  * satisfiedBy: [useLiveStore.test.ts](../../../../explorer/src/store/useLiveStore.test.ts)
  * satisfiedBy: [test.sh](../../../../tests/test-serve-command/test.sh)
  * verify: [Explorer Automatic Store Refresh](../../../Interfaces/WebExplorer/Capabilities.md#explorer-automatic-store-refresh)
---

### Explorer Route Identifier Resolution Verification

Verify authored identifiers through the compiled Explorer served by the CLI in a real browser.

#### Details

##### Acceptance Criteria
- Raw and percent-encoded Unicode element, file, source-content, and resource routes resolve to the fixture records.
- Encoded literal percent sequences are decoded once and retain the authored filename.
- Clicking a modeled element, closing its modal, and opening its source page preserve the literal percent filename and underlying file context.
- Malformed percent escapes render a missing-record state within the Explorer without crashing.

##### Test Criteria
- Run the standard serve E2E suite against a temporary Git workspace containing Unicode and literal percent filenames and evidence resources.
- Load the served bundle in headless Chrome or Chromium and exercise the four route families and generated navigation controls.
- Compare normalized browser results with the checked-in expected fixture; any failed route or missing completion result fails the suite.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Web Explorer Interface Verification Objective](#web-explorer-interface-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-serve-command/test.sh)
  * verify: [Explorer Route Identifier Resolution](../../../Interfaces/WebExplorer/Capabilities.md#explorer-route-identifier-resolution)
---

### Explorer Scoped Coverage Verification

This verification checks consistent coverage scope selection, ranked hierarchical presentation, and compact evidence links in the existing Explorer.

#### Details
Expected checks:
- Open Coverage on a model with two independent roots, nested and empty capabilities, shared verification targets, an orphan and cross-root evidence. With no selection, assert the complete whole-model dashboard, metrics, type/source breakdowns, ranked drill-down and all four issue sections. Keep the left capability navigator and header worktree selector. Assert no mode selector, scope dropdown, Summary/Issues navigation pages, View issues button or empty capability-selection prompt. Selecting the Whole Model root clears selection and restores whole-model data and its canonical URL.
- Through nonvisual component checks, assert that the shared Coverage navigation offers every capability once, nested beneath one Whole Model root at level 1, with capability roots at level 2, parent-before-child depth indentation, alphabetical root/sibling ordering, authored names, canonical identifiers and accessible tree levels/selection/disclosure states. The Whole Model row has no capability identifier or published membership; its pointer/Enter/Space selection clears scope and its independent chevron only changes disclosure. Test an empty model retaining the selectable Whole Model root. Collapse the root, then follow a copied nested-capability link or history entry and reveal both the root and selected capability ancestors. Select nested and empty capabilities and retain the complete navigation hierarchy. Assert the page title identifies Whole Model or the selected capability and no Scope dropdown is rendered in either scope context. Collapse and expand branches without changing scope or URL history, use keyboard traversal and selection, and restore a previously hidden selected capability through history. Shared children use the deterministic display parent. Reverse scope and capability records and add inverse parent edges; assert identical navigation order, retained selection, unchanged published summaries and an unchanged input store.
- Through real browser navigation in served and exported Explorer, assert encoded capability scope URLs, explicit Whole model links and migration of retired mode/issue/detail links, URL precedence over conflicting saved preferences, bare-route preference resumption, reload and opening a copied link without preferences, one history entry per changed scope and no entry for a repeated selection, Back/Forward report and sidebar parity, retained scoped base routes after element details and return from another view, and retained document query parameters. Check unavailable/non-capability links and scope removal after refresh for explained canonical Whole model fallback. In served routing, switch between worktrees sharing capability identifiers and assert separate saved scopes, canonical worktree/selection pairs, and restoration through browser traversal. Component checks MUST retain the old accepted scope while a requested worktree is pending, and scope-only history MUST leave a same-context refresh active. These checks do not require visual comparisons.
- Inspect generated served and exported Project Store data before browser projection. Assert that `coverage.scope_index` has exactly one entry for every capability, including nested and empty capabilities, and no entries for other element types. Compare each entry's `scope` and `summary` with the shared scoped report for the same snapshot, including sorted distinct membership identifiers, external evidence exclusion from membership, and the whole-model-only orphan marker.
- Select each root, a nested capability and an empty capability. Compare every displayed metric, type/source breakdown, ranked row and issue list against the shared scoped report. Assert all three scoped issue sections follow the drill-down in DOM order, retain their scope after detail navigation, and reset their list disclosures on scope changes. Empty capabilities retain the full dashboard with zeros and explicit empty issues. Orphan metrics/list appear only after clearing scope through the Whole Model root. Scope/reset actions leave the input store unchanged.
- Inspect the compact summary in wide and constrained coverage panels with the Explorer pane open and collapsed. The three scoped metrics share one row in wide panels; the four whole-model metrics share one row when space permits. Smaller metric rings, reduced card spacing and compact breakdowns retain every label and count, with responsive reflow and no horizontal clipping.
- Assert capability and requirement verification bars use the shared green verification token while authored requirement markers retain their violet type color. Assert coverage metric bars retain Verification/Implementation headings, percentages and exact counts without visible Covered, Uncovered, Partial, Verified, Partially verified or Not verified status labels. Outstanding requirement blocker counts remain visible. Accessible metric summaries preserve the published state, including complete capabilities with zero local terminal units and external supporting consumers; standalone binding-consumer context/status badges remain.
- Inspect zero, complete and fractional metric percentages. Percentage text appears beside its label outside the decorative ring, remains available to assistive technology and does not overlap the stroke or expand wide-panel card height.
- Assert verification types and implementation sources remain visible directly below progress metrics and above drill-down, without a Report details disclosure. Compare all five inline type counts, including zeros, and every source count against the shared scoped report. Check the single type bar represents proportional positive counts without fabricated segments for zero totals; labels/counts remain accessible independently of color. Inspect compact wrapping legends and source bars in wide and constrained panels, with equal card heights and top-aligned contents when side by side. Scope changes, worktree changes and same-context refresh update the published counts without changing membership, input data or URL history.
- Assert parent-before-child ordering, depth indentation, and ascending verification/implementation ranking within roots and sibling groups, including name and identifier tie breaks. Ranking keeps each displayed subtree together. Multiple valid parent paths must not duplicate counts or hide reachable members; reordering input records must not change the display parent.
- Assert evidence links use the existing compact endpoint pattern, wrap without overflow at narrow widths, and open both element and resource targets. A requirement with several artifacts must not produce full-width action buttons.
- Confirm external binding consumers remain linked and open the existing detail workflow without changing scope membership.
- Confirm implementation percentages display terminal numerators and denominators from the shared report, while capability completeness remains independent in accessible metric summaries. A covered capability with external terminal consumers can display zero local terminal units without being assessed as incomplete from those local counts.
- Confirm that capability rows retain the ranked hierarchy while attached requirement rows initially remain collapsed. Expand a capability to reveal its attached requirements, then expand a requirement to reveal its immediate child requirements. Expand a child with descendants to reveal the next requirement level, then select a terminal requirement name to inspect its implementation evidence in element details. Inspect the shared report surface, element-type markers, indentation, and chevron disclosure state. Collapse the capability to hide its requirement details while preserving the ranked capability rows. Name links retain the existing element detail workflow independently of expansion controls.
- Expand a nested capability whose attached requirement has a parent displayed under another capability. Inspect that requirement and its descendants within the nested capability, retaining published status, element-detail navigation, and shared report totals. Confirm each requirement appears once within each capability disclosure, with deterministic parent choice and independent disclosure state across capability contexts.
- Inspect an uncovered parent with direct evidence: its child rows appear directly below it. Each child has its own published coverage status. The parent implementation metric retains the recursive blocker count on the right of its row. Terminal rows with and without artifacts retain their metrics and element-detail link without an expansion action. Requirements with children or binding consumers remain expandable.
- Inspect covered and uncovered terminal requirement rows: verification and implementation retain their published statuses and show the same colored bars and metric layout as capability rows, including full `100% · 1 / 1` and empty `0% · 0 / 1` values. Check an independently verified but unimplemented terminal, and check complete and incomplete parent bars against their published aggregate counts, retaining blocker statuses. Check binding-consumer progress across scope boundaries.
- Compare accessible verification metric summaries with published report values for complete, partial and zero coverage; implementation summaries retain the independent published classifications. Visual bar headings retain only the metric name and any meaningful blocker count, without verbal coverage states.
- Inspect visible capability and requirement rows in light and dark themes: consecutive rows alternate surface tones across the complete visible hierarchy, including after expanding and collapsing a branch. Inspect hierarchy branches and dependency lists for border-free row separation; rows within each list alternate the same tones. Compare verification and implementation column positions across capability and requirement depths on wide and narrow screens. Element identities and dependency links retain hierarchy indentation, with readable wrapping.
- At a 780px viewport, expand three successive requirement levels with the Explorer pane open and then collapsed. Confirm each child identity is indented beyond its parent, coverage columns and dependency statuses remain aligned, and the row layout adapts to the available panel width. Repeat on a wide viewport and a narrow viewport.
- Compare immediate contribution identifiers with the displayed child rows and additional dependency links. Contract consumers appear under Binding consumers with their published status and applicable outside-scope marker. A shared child chosen for display under another parent remains linked under Additional child requirements. Check that all reported immediate dependencies remain reachable.
- Inspect binding consumers and additional child requirement groups under nested requirements in wide and constrained panels. Group headings align with the owning requirement's name text; linked element markers and names align with immediate child rows using the same disclosure space. Dependency status right edges align with the shared implementation column.
- Expand a parent and its descendants and confirm that the coverage hierarchy contains no artifact lists or Direct / Via dependencies labels. Select the terminal requirement name, inspect its satisfiedBy relations in the element detail modal, and follow a local artifact link to the shared file-content viewer. Confirm that the artifact content is displayed and the element dialog closes. Return to Coverage and reload; confirm coverage counts and scope remain unchanged. Compare the right edges of binding consumer statuses with the requirement status column.
- Assert the unified Coverage view always retains the left capability tree with every capability nested under its selectable Whole Model root. Page titles identify Whole Model or the selected capability; selecting a capability renders its full dashboard and detailed issues below. Selecting Whole Model selects the whole-model dashboard and its orphan diagnostics.
- Open the showcase coverage fixture through the real Explorer application, expand its capability and requirement rows, and use its actual detail and resource workflows.
- In the design-system coverage mock, assert through its real Explorer application that the same exported Coverage navigation displays the model-derived hierarchy and selects a nested capability with the published counts. The mock MUST NOT author a separate capability list or hierarchy rendering.
- Use long requirement and binding consumer names. Assert retained element-detail targets and wrapping without horizontal overflow at narrow widths.
- In capability scope, assert that orphan diagnostics are labelled whole-model-only and that activating the Whole Model root restores the complete whole-model dashboard, including the orphan section.
- Reload and refresh valid snapshots while preserving available scope identifiers and ranked hierarchy. Ignore previously stored root-summary or flat-ranked display preferences. Remove the selected capability in a later snapshot and assert explained fallback to Whole model with internally consistent counts.
- Verify empty-capability behavior and project-specific state isolation. Remove required scope data or requirement aggregate counts and assert a store diagnostic before rendering.
- Exercise the same scoped projection through served and exported Explorer data, with the Coverage view and sidebar agreeing in both cases.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Web Explorer Interface Verification Objective](#web-explorer-interface-verification-objective)
  * satisfiedBy: [MockShell.test.tsx](../../../../explorer/design-system/showcase/MockShell.test.tsx)
  * satisfiedBy: [loadStore.test.ts](../../../../explorer/src/store/loadStore.test.ts)
  * satisfiedBy: [CoverageNavigation.test.tsx](../../../../explorer/src/state/CoverageNavigation.test.tsx)
  * satisfiedBy: [CoverageView.test.tsx](../../../../explorer/src/views/CoverageView.test.tsx)
  * satisfiedBy: [CoverageNavigation.test.tsx](../../../../explorer/design-system/product-patterns/reports/CoverageNavigation.test.tsx)
  * satisfiedBy: [routes.test.ts](../../../../explorer/src/router/routes.test.ts)
  * satisfiedBy: [useLiveStore.test.ts](../../../../explorer/src/store/useLiveStore.test.ts)
  * satisfiedBy: [browser-navigation.mjs](../../../../tests/test-scoped-coverage/browser-navigation.mjs)
  * satisfiedBy: [test.sh](../../../../tests/test-scoped-coverage/test.sh)
  * verify: [Explorer Scoped Coverage](../../../Interfaces/WebExplorer/Capabilities.md#explorer-scoped-coverage)
---

### Explorer Native Trace Flow Verification

This test verifies production served/exported trace rendering and the shared native trace-flow pattern used by the design-system mock.

#### Details
- Open the compiled production `#/traces` route in both served and exported Explorer. Assert real React Flow cards and locally packaged worker layout replace the former Mermaid roll-up. Compare all canonical node identifiers and directed labeled edges against the CLI trace projection for reconvergent requirement graphs, directly verified ancestors, owning capabilities, and capability parents. Count unique requirements separately from capabilities.
- Select a verification and its file overview using the production left pane. File overviews remain interactive without flow preparation or workers for unselected traces. Empty projections and deleted selections remain usable. Unrelated Coverage refreshes reuse shared trace grouping and flow preparation.
- In the real served/exported browser, exercise direction changes, canonical element-detail routes, modal Close/Back, source-file routes and return to Traces. No navigation reloads the document; actual packaged worker assets are local and load successfully. Repeat graph equality checks after direction changes.
- Replace immutable trace/element inputs with changed labels, source files, membership, and edges under reused identifiers. Switch workspace/worktree context while old layout is pending and assert cancellation, rejection of delayed results, and absence of the previous context's map. Worker failure exposes retry and successful retry recovers the selected flow.
- From the Mocks Explorer shell, the Traces navigation item opens the native trace map. Direct loading of `#/traces` and returning from another Explorer view use the same entry point. The mock presents one Explorer navigation menu with shared shell theme controls.
- A simple trace retains verification-to-requirement `verifies` and child-to-ancestor `derivedFrom` direction.
- Branching traces merge shared requirement identifiers into one node and count unique requirements, preserving all distinct relations.
- A trace that splits, merges, splits again, and merges again preserves every relation, including a shortcut across intermediate ranks. Cards and relation labels remain separate, and routed connections avoid unrelated card interiors.
- Reordering requirements and parent identifiers leaves graph positions and routes unchanged.
- Compare complete ELK input against the per-node scan semantics for splits/merges, parallel relations, self-loops, isolated nodes, empty graphs, escaped identifiers, and both orientations. Count endpoint reads on increasing sparse graphs to confirm a single edge pass after sorting. Switching direction or replacing topology must not reuse stale ports or mutate input data.
- The direction control switches between Left to right and Top to bottom. Both preserve node identities, relation direction, counts, branch disclosure, and pinned path focus. ELK layout places incoming/outgoing connections on west/east sides for horizontal flow and north/south sides for vertical flow, with orthogonal segments and distinct relation labels.
- While layout is pending, the current map remains available. Out-of-order layout completions and failures cannot replace a newer result, including rapid switches between layout directions. Layout failures display a retry action; retry can recover without resetting an already displayed map.
- Exercise real worker layout in both orientations and verify parity with the layout engine, including splits/merges, parallel edges, self-loops, and empty/isolated graphs. Replacing scope, context, or direction and unmounting terminates obsolete workers; simultaneous flows remain independent. Check worker startup, execution, message, and loading errors, retry, stale results, and cleanup after success/failure. Verify served/exported and showcase worker assets are local and loadable, and measure main-thread responsiveness separately from layout duration.
- Collapsing a branch hides only nodes no longer reachable through the visible trace and preserves full-trace counts; Expand all restores its nodes and relations.
- Graph node names appear beside the colored element glyph, with context labels below. Clicking the name or card body opens the application's Model element-detail modal through the canonical element route, displaying fixture content, type, metadata, and incoming/outgoing relations. Related-element navigation and Back preserve the trace behind the modal; Close returns to the trace. Source navigation opens the fixture source page. Focus and disclosure controls do not open the modal. Selecting a file shows its verification overview.
- Check that the rendered node wrapper and card accept pointer input before exercising name clicks, card clicks, and hover, including when node dragging and selection are disabled.
- Hover and keyboard focus highlight connections into and out of the chosen node to the end of every visible directed path, including repeated splits and merges. Highlighted lines and arrowheads use the accent token and stronger stroke treatment, while sibling-only paths remain dimmed. Pointer exit clears temporary highlighting. A separate Focus path action pins the highlight after pointer exit; clearing restores all paths without moving cards or resetting zoom.
- The mock provides simple, branching, long-name, and empty examples through the public design-system pattern, with pan/zoom controls and light/dark theme selection.
- Nodes use shared glyphs and token-based styling. Fitting caps zoom at actual size; zoom controls allow inspecting dense traces at reading scale.
- For a large layout that needs less than 20% zoom, assert that initial framing and Fit include all card bounds on desktop and narrow canvases. Check both directions, manual zoom-out, returning from 100%, canvas resize, and switching back to a small scope. Verify layout bounds contain routed connections and relation labels as well as cards.
- Exercise wheel zoom, toolbar zoom, actual size, Fit, and pan in both directions while leaving the pointer away from cards. Verify that viewport navigation retains routed geometry, labels, arrowheads, card identity, pinned paths, and keyboard focus. Simulate oversized and zero-sized SVG text measurements and assert that labels remain visible at their centered route positions with a compact outline around the text. In a browser, inspect long connections for continuous rendering immediately after navigation, including in the expanded view.
- Expand the active trace to a full-page overlay. Verify that the same canvas remains mounted, direction and branch disclosure persist, and pinned paths remain highlighted. Exercise direction, zoom, Fit, card detail, and source actions from the overlay. Close and Escape return to the embedded view with focus restored to Expand flow. Escape in an element-detail modal closes those details while keeping the expanded flow open.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Web Explorer Interface Verification Objective](#web-explorer-interface-verification-objective)
  * verify: [Traces View Generation](../../../Interfaces/WebExplorer/Capabilities.md#traces-view-generation)
  * satisfiedBy: [traceFlow.test.ts](../../../../explorer/src/lib/traceFlow.test.ts)
  * satisfiedBy: [TracesView.test.tsx](../../../../explorer/src/views/TracesView.test.tsx)
  * satisfiedBy: [browser-flow.mjs](../../../../tests/test-verification-traces/browser-flow.mjs)
  * satisfiedBy: [test.sh](../../../../tests/test-verification-traces/test.sh)
  * satisfiedBy: [TraceFlow.test.ts](../../../../explorer/design-system/product-patterns/reports/TraceFlow.test.ts)
  * satisfiedBy: [traceFlowLayoutInput.test.ts](../../../../explorer/design-system/product-patterns/reports/traceFlowLayoutInput.test.ts)
  * satisfiedBy: [flowLayoutEngine.test.ts](../../../../explorer/src/workers/flowLayoutEngine.test.ts)
  * satisfiedBy: [check-flow-worker.mjs](../../../../tests/test-export-command/check-flow-worker.mjs)
  * satisfiedBy: [useTraceFlowLayout.test.ts](../../../../explorer/design-system/product-patterns/reports/useTraceFlowLayout.test.ts)
  * satisfiedBy: [TraceFlowViewport.test.tsx](../../../../explorer/design-system/showcase/TraceFlowViewport.test.tsx)
  * satisfiedBy: [TraceFlowMock.test.tsx](../../../../explorer/design-system/showcase/TraceFlowMock.test.tsx)
---

### Explorer Serve Verification

This test verifies that the system serves the native SPA Explorer shell with Model route containment modes and Project Store data.

#### Details

##### Acceptance Criteria:
- System shall serve `index.html` as the primary SPA Explorer shell and browser-local Project Store host
- `index.html` shall contain a Project Store seed before Explorer views render
- The Model route shall display folders, files, elements, and the project graph through native List, Grid, Graph, and Flow modes.
- The Model project tree shall initialize with top-level `Model` and `Resources` branches and their Git worktree identity folders expanded, so first useful content folders/files are visible while deeper folder and file element rows remain collapsed until user action or selected-descendant reveal behavior requires expansion.
- Modeled-element Grid cards shall use a single leading element marker, keep the title close to that marker, and render adjacent type badges without repeating the marker dot, shape, or glyph.
- Graph mode shall render the project knowledge graph with pan/zoom, search/focus, selected-node state, and graph filters in the Model left pane.
- The native Explorer shell shall not render primary left-pane view links; Ontologies and Traces are reached as specialist Explorer views, while the project Knowledge Graph is reached as Model Graph mode.
- The native Explorer shell shall expose the shared collapsible vertical `Explorer` edge strip and a compact right tool rail; views with contextual evidence use left-pane selected-item links and shared detail modals instead of defining route-local right-side geometry.
- Old Explorer page URLs shall not be generated; equivalent content shall be reachable through SPA routes and source-document links.
- Links in diagrams and text shall resolve through Explorer content routes or Project Store source-content records
- Paths in served content shall maintain the original relative structure
- Project Store file records shall expose source content generated directly from modeled element source files, without depending on generated Markdown files on disk.
- Relation-backed implementation files, evidence files, scripts, images, documentation files, and other local resource targets shall remain Project Store resources for relation semantics, may carry source-preview content when the local file exists, and shall be visible in Explorer as resource/evidence entries under a top-level `Resources` branch without appearing as modeled file containers unless the same path also contains modeled elements.
- System should work in environments without Git repositories

##### Test Criteria:
- Command exits with success (0) return code
- The served root URL returns `index.html`
- `/assets/project-store.js` contains an Explorer Project Store seed
- The Project Store seed includes required sections for files, resources, elements, relations, contract_bindings, concept references, submodels, traces, coverage, ontology, knowledge graph, search, summaries, and routes
- The Project Store seed distinguishes modeled file containers from modeled resources/evidence files
- Project Store file records include normalized source Markdown content derived from the registry for modeled files.
- Relation-backed local resource targets are present as resources with source-preview content when the local file exists, are reachable through Explorer resource/evidence navigation with folder structure under the `Resources` branch, and resource-only paths are absent from the Project Store `files` and `folders` hierarchy.
- Nonexistent local targets, unsupported parsed pages, unrelated repository files, and external URLs are absent from the Project Store file-tree hierarchy
- Embedded MCP mutations refresh the runtime snapshot, and the served seed and live JSON reflect the updated records under the owning Served Explorer Runtime Freshness Verification.
- Hash routes for primary Model, file deep links, Ontologies, and Traces views plus supporting Coverage, Resources, Elements, and Search workflows are declared; the project Knowledge Graph is not a separate hash route
- Retired Explorer page URLs are absent from generated output and canonical route mappings
- Explorer content preserves the structure and information from the source files
- Model tree initial render shows top-level `Model` and `Resources` branches with Git worktree identity folders expanded and does not eagerly expand deeper child folders or file element rows.
- Modeled-element Grid card markup renders one `ElementIcon` per element card and uses text-only type badges when the icon is already present.
- Mermaid click links resolve through canonical Explorer routes or source anchors
- Both GitHub-style URLs and direct file paths in mermaid click links are handled correctly
- Paths should not have duplicated folder names (e.g., specifications/specifications)
- Missing embedded asset paths return 404 while non-asset browser routes return the SPA shell for client-side routing
- Existing workspace-root-relative static assets referenced by Markdown image or document links return the file bytes with an appropriate content type
- Repository asset requests reject parent-directory traversal and unsupported static asset extensions

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-serve-command/test.sh)
  * verify: [Served Explorer Browser Interface](../../../Interfaces/WebExplorer/Capabilities.md#served-explorer-browser-interface)
---

### Export Command Verification

This test verifies that the export command writes a complete self-contained static Explorer site to the specified output directory.

#### Details

##### Acceptance Criteria:
- System shall write `index.html` to the output directory
- System shall write `assets/project-store.js` containing `window.reqvireProjectStore`
- System shall write `ontologies.ttl` to the output directory
- System shall write all other embedded SPA bundle assets to the output directory
- System shall copy eligible Git-worktree static assets referenced by rendered workspace content using their workspace-root-relative output paths
- Output directory shall be self-contained and serve correctly from a static file host

##### Test Criteria:
- Run `reqvire export --output <tmpdir>` on a minimal model workspace
- Verify `index.html` exists and contains the Explorer SPA shell
- Verify `assets/project-store.js` exists and contains `reqvireProjectStore`
- Verify `ontologies.ttl` exists
- Verify an exported workspace containing a Markdown image such as `![Diagram](images/diagram.png)` includes `images/diagram.png` in the output and the exported Explorer renders it without a broken image request
- Verify the exported assets reference no external CDN resources

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-export-command/test.sh)
  * verify: [Export Command](../../../Interfaces/WebExplorer/Capabilities.md#export-command)
---

### Mobile Responsiveness Verification

This test verifies the Explorer is usable on mobile devices.

#### Details
Expected checks:
- Desktop and mobile viewports can use the Explorer shell without horizontal page overflow.
- Left Explorer pane and right tool rail remain compact and usable without a top header.
- Touch targets remain usable on common mobile viewports.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Mobile-Friendly Explorer](../../../Interfaces/WebExplorer/ExplorerRendering.md#mobile-friendly-explorer)
  * verify: [Responsive Explorer Rendering](../../../Interfaces/WebExplorer/ExplorerRendering.md#responsive-explorer-rendering)
---

### Model Containment Contract Bindings Links Verification

This test verifies that the served Explorer Model containment data preserves contract_bindings links from modeled elements to referenced contract elements.

#### Details

##### Acceptance Criteria:
- Elements with contract_bindings shall expose contract_bindings records or equivalent Project Store containment contract_bindings records
- Element contract_bindings records that target contract elements shall use the shared Explorer element-role and subtype glyph contract rather than a report-specific symbol
- Element contract_bindings shall be clickable from supported Explorer surfaces and navigate to the referenced element detail/source route

##### Test Criteria:
- Create model with element having contract_bindings
- Run the Explorer through the serve workflow or a Project Store fixture
- Verify Model List/Grid data contains contract_bindings records or equivalent Project Store containment contract_bindings records
- Verify contract-bindings-element records have links to element definitions or element-detail routes

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Containment View Contract Bindings Links](../../../Interfaces/WebExplorer/Capabilities.md#containment-view-contract-bindings-links)
---

### Explorer Model Flow Verification

Verify the Model Flow mode through the same application and design-system patterns used by the showcase.

#### Details
- Select Flow from the existing Model layout selector, switch between all four modes, and retain the selected folder, file, or element.
- Select an element and a capability in the left project tree while Flow is active: the right workspace updates to that selection's relation scope and retains the Model route. The detail dialog opens only after activating a Flow card name or body, independently of tree selection.
- At the project root, assert that Flow starts with root capabilities above their child capabilities and specifying requirements, then descends through requirement decomposition to verification and evidence. Switch to horizontal layout and retain that ordering. Assert corresponding inverse labels for the displayed direction and unchanged canonical store facts; Traces keep their verification-to-ancestor direction.
- Compare Flow nodes and labelled directed connections with the Project Store, including different element types, evidence endpoints, contract bindings and references, and generated inverse relations. Shared endpoints appear once; different relations between the same endpoints remain distinct.
- Select a file or folder and assert that its elements and their immediate relation endpoints remain reachable, together with requirement and capability ancestry back to the owning root capabilities. Select an element and assert its full incoming and outgoing paths across repeated splits and merges. Include an isolated element, an empty folder, and a cyclic non-hierarchical relation graph.
- Switch between horizontal and vertical layout and exercise path focus and clearing. Hover or keyboard-focus a card and inspect full-path highlighting through splits and merges, accent lines and arrowheads, faded unrelated connections, and unchanged card positions.
- Click a card body and name to open the existing element-detail modal, inspect metadata and incoming/outgoing relations, navigate a related element, go back, and close to the same Flow workspace. Source and focus controls perform their own actions.
- Confirm React Flow's rendered node wrapper and card accept pointer input while node dragging and selection are disabled. Check pointer eligibility before dispatching card and hover interactions so synthetic events cannot mask a browser hit-testing failure.
- Render a large Model Flow requiring less than 20% zoom on desktop and narrow canvases. Assert that initial framing and Fit keep all cards inside the canvas in both directions, manual zoom-out remains available, and Fit restores the overview after 100%. Resize the canvas and select a small scope to check updated framing and the 100% fitting cap. Confirm complete layout bounds include connection routes and relation labels.
- Hold canvas initialization and automatic-fit scheduling in a controlled renderer fixture. Navigation remains unavailable before the current-direction canvas initializes; a user zoom after readiness survives queued animation-frame callbacks. Repeat across a direction change and rapid switch back; delayed initialization callbacks from unmounted canvases cannot replace the current canvas handle.
- Check connection rendering after wheel and toolbar zoom, pan, Fit, actual size, canvas resizing, and overlay changes. Preserve routing, centered labels, arrowheads, node identity, pinned path, and keyboard focus. Include oversized and zero-sized SVG text measurements in regression tests. Inspect long routes and the compact outlines around label text in the browser immediately after navigation, with the pointer away from cards, to verify continuity at the new scale.
- Open and close the full-page Flow overlay using its controls and Escape. Assert that scope, direction, pinned path, and canvas identity are retained. Verify overlay keyboard focus containment and focus restoration, usable flow controls, and shared element details above the overlay; closing the detail modal leaves Flow expanded. Source navigation exits to the requested source page.
- Exercise the shared Flow worker cancellation, isolation, and recovery checks from Explorer Native Trace Flow Verification. In served Explorer, open Model Flow and switch orientation; node identities must survive, and the worker must load from the local packaged asset. Execute the exported worker in a real thread and verify routing, error responses, deterministic output, empty graphs, and a responsive main thread during large layout.
- Run the projection and application interaction tests, design-system adherence checks, and both Explorer and showcase builds.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Web Explorer Interface Verification Objective](#web-explorer-interface-verification-objective)
  * verify: [Model-Centric View Generation](../../../Interfaces/WebExplorer/Capabilities.md#model-centric-view-generation)
  * verify: [Model View Element Navigation](../../../Interfaces/WebExplorer/Capabilities.md#model-view-element-navigation)
  * satisfiedBy: [modelFlow.test.ts](../../../../explorer/src/lib/modelFlow.test.ts)
  * satisfiedBy: [TraceFlow.test.ts](../../../../explorer/design-system/product-patterns/reports/TraceFlow.test.ts)
  * satisfiedBy: [traceFlowLayoutInput.test.ts](../../../../explorer/design-system/product-patterns/reports/traceFlowLayoutInput.test.ts)
  * satisfiedBy: [flowLayoutEngine.test.ts](../../../../explorer/src/workers/flowLayoutEngine.test.ts)
  * satisfiedBy: [check-flow-worker.mjs](../../../../tests/test-export-command/check-flow-worker.mjs)
  * satisfiedBy: [TraceFlowMock.test.tsx](../../../../explorer/design-system/showcase/TraceFlowMock.test.tsx)
  * satisfiedBy: [route-check.mjs](../../../../tests/test-serve-command/scripts/route-check.mjs)
---

### Model View Element Navigation Test

Test verifies that element names in the model-centric view are clickable links.

#### Test Steps
1. Run `reqvire model` command to generate model report
2. Verify output contains element headers as markdown links
3. Verify links follow format `[Element Name](file_path#fragment)`

#### Expected Results
- Element names are rendered as markdown links
- Links point to source file with element fragment

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-model-command/test.sh)
  * verify: [Model View Element Navigation](../../../Interfaces/WebExplorer/Capabilities.md#model-view-element-navigation)
---

### Ontology Model Viewer Analysis Verification

This analysis verifies that the Ontologies page behaves as an ontology model viewer rather than a raw RDF triple viewer.

#### Details
Expected analysis checks:
- Confirm the primary Ontologies visualization, search data, and ontology node modal construct metadata are built from `SemanticIndex.ontology_projection`, the same generated ontology construct projection used by full semantic export.
- Confirm ontology graph data includes explicit `authored`, `concepts`, `reqvire-context`, and `external-source` layer semantics; authored structural ontology/projection facts are controlled by the Core layer, SKOS concept nodes, SKOS concept taxonomy edges, plus one-way `mapsToConcept` bridge edges are controlled by the Concepts layer, generated semantic context is hidden until its layer is enabled and is limited to model-to-term `declaresTerm` and `referencesTerm` provenance, and external-source vocabulary is a separate opt-in layer.
- Confirm the primary viewer does not render `rdf:type` edges, OWL/RDFS metaclass nodes, RDF list plumbing, anonymous SHACL property-shape blank nodes, or generic literal plumbing as the main user-facing graph.
- Confirm classes, object properties, datatype properties, RDF properties, named individuals, datatypes, restrictions, class expressions, SHACL node shapes, SHACL property shapes, and generic resources are classified into distinct semantic kinds when present, while property terms are projected as graph relationship semantics rather than standalone graph nodes.
- Confirm datatype-property literal values are not graph nodes or visibility filter layers, but remain searchable and appear in the selected subject node modal as predicate/value evidence.
- Confirm a named IRI typed only by a declared ontology class, without explicit `owl:NamedIndividual`, is shown in graph data and the ontology node modal as a named individual while retaining its `∈` membership construct evidence.
- Confirm visual coloring is driven by semantic kind, not by source provenance, so a class referenced by SHACL remains class-colored, property metadata remains property-typed in links/modal/search, and actual SHACL node shapes and property shapes use SHACL-specific colors.
- Confirm RDF, RDFS, XSD, OWL reserved vocabulary and core SHACL shape syntax do not require local External Ontology source files and are not presented as imported external-source vocabulary. The external-source layer is reserved for used external subset triples derived from actual `#### External Ontology` dependencies, unused raw external terms are absent from graph data and search, and external-source metadata identifies `used_subset` materialization.
- Confirm object and datatype properties are first-class relationship semantics with aggregated domain and range information rendered as labeled links and modal property usage evidence, not as standalone graph nodes.
- Confirm SHACL node-shape target classes and property usage rows receive derived slot/facet modal sections from property-shape paths, datatype/class range constraints, node kind, cardinality, pattern, allowed values, and source-shape evidence.
- Confirm target-class slot/facet sections are labeled as class slots, property usage sections are labeled as usages of the selected property by target classes, and repeated usages with different target classes or source shapes are not presented as duplicate property definitions.
- Confirm raw SHACL evidence is shown only when direct raw constraints are bound to the inspected node and that classes or property metadata with only normalized SHACL overlays do not show an empty raw-evidence section.
- Confirm equivalence groups use stable deterministic identifiers derived from canonical member lists.
- Confirm domain/range, subclass/member-of, disjointness, equivalence groups, inverse properties, property chains, property characteristics, class-expression/restriction constructs, SHACL overlays, provenance/source citations, and symbols are represented from generated semantic projection constructs when present in the ontology input.
- Confirm class-expression nodes used as property domain/range expressions display contextual labels for actual union-valued ontology constraints while preserving expression members and property usage evidence in the ontology node modal, and confirm `define` does not render as a `Capability ∪ Requirement` range expression because contract ownership is requirement-only.
- Confirm ontology viewer symbols are defined with semantic meaning, raw Unicode code point, rendered Unicode character, allowed viewer usage, tooltip text, and accessible labels.
- Confirm ontology modal badges render the symbol and semantic label without rendering the raw Unicode code point as visible badge text, and that visible badge labels prefer domain wording such as `Subclass`.
- Confirm the ontology node modal uses a single-column content flow with RDF type, full URI, OWL document, description, and notation at the top before projection constructs, literal values, raw evidence, and sources.
- Confirm subclass and membership badges are directional and are not mirrored onto superclass or class-object nodes solely because those nodes are construct targets.
- Confirm source citations remain available as modal/search evidence, link to served source route fragments, and the served `ontologies.ttl` artifact remains available for raw RDF/Turtle auditability and downstream tooling.
- Confirm OWL ontology document IRIs typed as `owl:Ontology` do not render as ontology graph nodes, `rdfs:isDefinedBy` does not render as a canvas edge, and selected authored ontology terms expose their OWL document IRI in modal/search metadata.
- Confirm the Ontologies SPA route opens directly in the shared Explorer shell below its persistent header, fills the available workspace beside the left Explorer pane, places the `.ttl` download action in the Ontologies left pane with the summary controls, and does not render a retired route-local action bar, raw Turtle/source-block list, page header preamble, shared content-card footer, or route-local right sidebar.
- Confirm the Ontologies SPA route paints the shell, graph canvas, and design-system spinner loading notice before deferred Sigma/ForceAtlas renderer construction starts, and retains a layout notice until the owned ForceAtlas worker settles.
- Confirm the first ontology layout applies the caller's active filter snapshot before selecting nodes and edges. Hidden layers and disabled roles are absent from the input; eligible edges retain visible endpoints, including an empty visible input. Changed-filter and reset requests use the latest visible topology.
- Hold ontology worker responses while replacing context, changing filters, resetting, disposing, or dragging nodes. Confirm obsolete results cannot change coordinates, notices, or camera state; failed jobs preserve the mounted map and can be retried. Two mounted renderers retain independent jobs and controls when one resets or disposes.
- Confirm the ontology graph uses the Sigma.js, Graphology, and ForceAtlas2 rendering engine already used by the project knowledge graph while preserving ontology-specific projection and filter semantics; confirm the full-graph ForceAtlas pass is computed from currently visible ontology nodes and edges with bounded settings derived from node count, edge density, and average rendered node size instead of fixed repulsion constants; and confirm it uses Sigma curved-arrow edge programs to separate parallel edges and labels between the same nodes.
- Confirm ontology graph property and construct edge labels are anchored on the same curved connector geometry as the rendered Sigma edge rather than at unrelated straight-line chord positions.
- Confirm normal ontology property relationships render as solid labeled Sigma arrows, while OWL set-operator/class-expression member links use a dedicated Sigma/WebGL edge program, render as unlabeled dashed structural connectors with an open diamond marker at the anonymous construct/source side and an arrowhead at the member target side, retain expression kind/member evidence in node labels and the ontology node modal, and do not draw connector strokes or markers from the edge-label canvas hook.
- Confirm ontology render nodes use one construct-class contract derived from semantic type and projection construct evidence, and that restriction/class-expression classification drives glyph rendering, visibility filters, construct-only node gating, and focused-neighborhood behavior consistently.
- Confirm OWL set-operator/class-expression and restriction nodes render as compact construct circles through Sigma `nodeProgramClasses` and `@sigma/node-image` rather than as ordinary named ontology classes, with Sigma providing the circular node-color background, a construct-kind node label, and an inline SVG pictogram providing a bold semantic glyph without PNG/raster sprites, SVG border detail, transparent square image backgrounds, or custom construct-node hover overlays; out-of-focus construct nodes must dim through the same Sigma reducer path as ordinary nodes.
- Confirm glyph-only construct circles appear only for actual OWL anonymous construct nodes, such as `owl:unionOf`, `owl:intersectionOf`, `owl:complementOf`, and `owl:Restriction`, when their `Class expressions` or `Restrictions` visibility controls and focused edge visibility rules make the construct neighborhood visible.
- Confirm non-property construct labels use `@sigma/edge-curve`'s curved-label renderer so labels follow the same curve geometry as the Sigma edge program.
- Confirm subclass connectors render through a dedicated Sigma/WebGL notation edge program with the same dashed connector stroke style as class-expression construct links and a hollow triangle marker at the superclass target side, while keeping readable `Subclass of` labels anchored on the connector.
- Confirm restriction constructs render as explicit restriction glyph nodes with Sigma-native restriction connector arrows for `on property` and filler/target evidence, and are not flattened into ordinary domain/range property edges.
- Confirm ontology node labels use Sigma default node-label and hover rendering, selected or hovered ordinary ontology nodes display their full labels even when normal unfocused labels are truncated for density, and construct glyph nodes show construct-kind labels while showing their symbols through Sigma image-node rendering.
- Confirm the ontology graph uses ontology-diagram visual conventions: class-like concepts, restrictions, and class expressions render as compact circular or elliptical anchors; properties render as labeled domain/range relationship edges near their anchors rather than graph boxes; datatype/resource/SHACL nodes remain visually distinct; relation edges use directed ontology-diagram connector lines with visible direction arrowheads; and the graph supports Sigma pan/zoom, selected-node centering through Sigma display-coordinate mapping, and reset-driven layout.
- Confirm ontology relationship edges are hidden in the default full-graph view and appear only for edges in the active focus tree, plus enabled member/filler edges one semantic step beyond visible construct-only nodes, through Sigma's native edge reducer and `@sigma/edge-curve` edge programs, with relation visibility controls limiting which focused edges are eligible. With no pinned selection, hover fades unrelated nodes. With a pinned selection, nodes outside the selected focus tree are hidden; rolling over a visible node inside that selected tree opens the rollover tree, dims selected-tree nodes outside the rollover tree to the same low-strength treatment, and keeps nodes outside both trees hidden.
- Confirm Sigma z-index and highlighted-node rendering are enabled so the active ontology focus tree is painted through Sigma's normal focus path, selected/focused nodes render above focused-neighbor nodes, focused-neighbor nodes render above focused edges, focused edges render above unrelated or muted graph items, and focused edges are not rendered through a separate focused-edge canvas overlay.
- Confirm generic SHACL overlay edges render as unlabeled overlay lines while retaining modal/projection evidence, unless an edge carries a more specific ontology/SHACL relation label.
- Confirm relation connectors and arrowheads remain visible but subtle enough not to overflow or dominate labels.
- Confirm circular class-anchor size is bounded and grows from graph connection degree rather than label length, so highly connected concepts are visually emphasized while low-degree concepts remain compact.
- Confirm graph nodes, property link labels, modal badges, and legend swatches resolve through the ontology semantic role palette consistently, including separate role tokens for class anchors, SKOS concepts, property semantics, datatypes, named individuals, SHACL shapes, resources, restrictions, class expressions, external references, and the shared graph canvas surface.
- Confirm search, focus, modal detail, filters, and the compact legend operate over semantic ontology roles and OWL constructs rather than generic RDF predicate edges.
- Reapply equivalent filter membership in a different order, change graph selection, and rerender the surrounding view. Confirm no repeated graph-wide attribute updates or focused layout. Same-size membership replacements and individual-toggle-then-sync changes must still update node visibility.
- Confirm selecting an ontology graph node computes the visible ontology focus tree, applies bounded Graphology no-overlap layout to that focus tree from stable post-ForceAtlas baseline coordinates with spacing scaled by focus-tree size, restores every graph node outside the current focus tree to the stable baseline, restores the full graph when selection is cleared or hidden by filters, centers the selected node through Sigma display-coordinate mapping after focus layout updates without changing zoom, and animates resulting node coordinates through Sigma node animation without rerunning full ForceAtlas layout or accumulating coordinate drift.
- Assert initial accepted layout schedules no unchanged-coordinate animation and one explicit completion refresh. Reset must retain its separate pending/accepted refreshes and fit the accepted graph. Exercise real focus coordinates, complete their animation, then clear or hide selection; require only changed nodes as targets, including a changed hidden node, exact baseline restoration, one immediate and one animation-completion refresh, and no camera movement for clear/filter. Repeated stable clear and selecting an isolated stable node must schedule no animation; isolated selection still centers the node. Selection replacement, clear, filters, reset, disposal, and dragging must cancel the prior animation; a cancelled focus animation must not overwrite dragged coordinates.
- Complete controlled separated, colliding, and empty ontology worker results. Require one bulk event containing final collision-adjusted coordinates and baselines for every rendered node, with `x`, `y`, `baseX`, and `baseY` hints for nonempty input and baseline-only hints for empty input. Verify complete event snapshots, reversed result IDs, exact collision offsets, hidden coordinates/current baselines, retained unrelated attributes, no individual attribute events, and existing refresh/camera behavior.
- Confirm ontology graph nodes can be dragged through Sigma pointer events, updating in-memory Graphology coordinates so users can uncover overlapped relation lines or labels, and confirm the visible view controls expose `Reset` without a separate `Fit` button.
- Confirm the Ontologies and Model/project graph left legend/filter panels use the shared graph control width and selected-control treatment: active controls use selected-control background/foreground tokens and inactive/hover controls use shared warm-neutral surface tokens.
- Confirm selected Model Graph nodes expose a selected-element link in the left Explorer pane that opens the shared element-detail modal, and selected Ontologies graph nodes expose a selected ontology-node link in the left Explorer pane that opens the ontology element modal.
- Confirm the detailed semantic type color key and construct notation key are passive, while the `Show` group contains the active role and relation visibility controls.
- Confirm Ontologies left-pane overlay controls expose Core, Concepts, Semantic Context, and External Sources as active layer filters with node counts, and that toggling layer controls changes graph visibility without changing ontology node modal evidence for selected nodes. Confirm Core and Concepts are shown by default, Concepts can be hidden for structural-only inspection, Core can be hidden for thesaurus-only inspection, Concepts uses the `--rdf-concept` role token for SKOS concept nodes, and no generated inverse `mappedFrom` bridge edge is rendered.
- Confirm the passive type legend exposes separate color swatches only for semantic kinds that can render as nodes, including classes, SKOS concepts, named individuals, datatypes, restrictions, class expressions, SHACL node shapes, SHACL property shapes, and generic resources; property kinds remain visible through property links, modal badges, search metadata, and relation visibility controls.
- Confirm the `Show` visibility group exposes one shared button design for datatype property links, object property links, class disjointness, restrictions, class expressions, SHACL shapes, resources, and external references without replacing Reqvire's richer passive type and notation legends, and that active means shown on the canvas.
- Confirm ontology terms and class-membership context are not exposed as hideable toggles, property links are controlled through datatype-property and object-property visibility controls, and the single SHACL shapes role filter controls both SHACL shape nodes and SHACL overlay relations.
- Confirm selected and hovered focus trees are computed from currently visible relation filters, so disabling object-property, datatype-property, restriction, class-expression, disjointness, or SHACL relation categories removes neighbors connected only by those hidden relations from the focused subgraph without filtering evidence from the ontology element modal; visible construct-only nodes expand the focus through their enabled construct links so union/intersection/complement members and restriction fillers are visible from the selected context.
- Confirm the default filter state opens with datatype-property links, object-property links, class disjointness, restrictions, class expressions, SHACL shapes, resources, and external references shown, while ontology terms and class-membership context remain always available.
- Confirm role filters are hard gates for node visibility, so disabling SHACL shapes hides both SHACL shape nodes and SHACL overlay relations without requiring a second SHACL slot-overlay checkbox.
- Confirm the passive `Notation` legend covers domain/range, subclass, membership, disjointness, equivalence, inverse, property chain, property characteristic, restriction, class-expression, and SHACL-overlay constructs without exposing those rows as a second construct-filter panel.
- Confirm relation styling is passive visual notation rather than a selectable filter, and the active `Show` relation controls determine which clutter categories are shown.
- Confirm equivalence, inverse-property, property-chain, and property-characteristic constructs remain visible as passive legend/modal evidence rather than active filters while they have no direct canvas-visible toggle effect.
- Confirm visibility controls affect construct-only canvas nodes, construct-specific canvas edges, and graph node badges without making nodes visible when their role filter is disabled, while the ontology node modal continues to show the selected node's full evidence.
- Confirm graph-registry provenance and generated-projection provenance are not exposed as graph-wide filter axes; source/provenance evidence remains available in the ontology node modal and Project Store data.
- Confirm active filters combine inclusively within one category and narrow together across different active categories on the canvas without narrowing the ontology element modal evidence.

Review artifacts: [OntologiesView.test.tsx](../../../../explorer/src/views/OntologiesView.test.tsx), [OntologyNodeDetailModal.test.tsx](../../../../explorer/src/components/OntologyNodeDetailModal.test.tsx), [forceAtlasLayout.test.ts](../../../../explorer/src/lib/forceAtlasLayout.test.ts), [forceAtlasLayoutEngine.test.ts](../../../../explorer/src/workers/forceAtlasLayoutEngine.test.ts).

#### Metadata
  * type: demonstration-verification

#### Relations
  * verify: [Ontology Construct Grouping](../../../Interfaces/WebExplorer/Capabilities.md#ontology-construct-grouping)
  * verify: [Ontology Property-Centric Visualization](../../../Interfaces/WebExplorer/Capabilities.md#ontology-property-centric-visualization)
  * verify: [Ontology Symbol and Badge Vocabulary](../../../Interfaces/WebExplorer/Capabilities.md#ontology-symbol-and-badge-vocabulary)
  * verify: [OWL Semantic Ontology Projection](../../../Interfaces/WebExplorer/Capabilities.md#owl-semantic-ontology-projection)
  * verify: [Ontology Projection Subgraph Materialization](../../../Reports/ModelReports/ReportingRequirements.md#ontology-projection-subgraph-materialization)
---

### Responsive Design Verification

This test verifies responsive breakpoints and compiled Explorer design-system CSS integration.

#### Details
Expected checks:
- Explorer layout works at mobile, tablet, and desktop widths.
- No layout breaks or overlapping controls are present.
- Compiled CSS and font assets are local and no runtime CSS compiler or CDN framework is required.

#### Metadata
  * type: test-verification

#### Relations
  * verify: [Explorer Design System Styling](../../../Interfaces/WebExplorer/ExplorerRendering.md#explorer-design-system-styling)
  * verify: [Responsive Explorer Rendering](../../../Interfaces/WebExplorer/ExplorerRendering.md#responsive-explorer-rendering)
---

### SPA Explorer Store Contract Verification

This test verifies that `index.html` is the central SPA Explorer shell and contains or loads the normalized browser-local Project Store required by all current views.

#### Details

##### Acceptance Criteria:
- Execute the real Rust embedding build script in isolated prebuilt and requested-rebuild fixtures. Existing output must not mask a failed npm build or missing npm. Unchanged assets/manifest retain modification times, changed assets update, removed assets disappear, and a missing bundle fails. Source watches exclude generated showcase/kit output and requested-build output; prebuilt mode watches production output. In a real minimal Cargo fixture, unchanged rebuilds stay fresh despite generated showcase output; adding public assets under the tracked public root or changing authored lint/typecheck inputs triggers a rebuild. Absent optional paths do not force repeated build-script execution.
- Execute the actual npm build compositions with instrumented leaf stages: the default build runs every guard and type check once before icons/application bundling, independent bundle targets omit checks, and the checked aggregate additionally builds the standalone kit once. Inject each guard/typecheck/bundle failure and require a nonzero result without running later stages. Inspect PR/release/Pages job compositions for one checked frontend build before prebuilt Cargo consumption and retained PR kit validation.
- Execute the actual Make dependency target in isolated fixtures: initial installation occurs once, unchanged repeated builds skip installation, both package files invalidate it, deleting dependencies reinstalls them, and an installation failure cannot create a success receipt or reach bundling. Retry after failure must reinstall successfully. Confirm clean dependency setup and real app/kit outputs separately from the instrumented orchestration checks.
- `index.html` shall be the primary Explorer shell and Project Store host, served as a native SPA built with Vite/TypeScript/React and the Reqvire Explorer design system.
- The served shell shall reference local compiled bundle, stylesheet, design-system, and font assets with no CDN-loaded framework, no CDN-loaded styling runtime, and no runtime CSS compiler.
- The Project Store seed shall be present before view rendering and shall include a schema/version marker.
- The Project Store `project` section shall include effective workspace root identity and eligible Git worktree metadata when Git metadata is available, and the Model tree shall render `Model` and `Resources` as visible top-level rows without a visible workspace-root wrapper. Each branch shall group local entries under eligible Git worktree identity folders so single-repo and multi-repo workspaces expose repository/worktree identity.
- The store shall expose normalized top-level sections for project, folders, files, resources, elements, relations, contract_bindings, concept references, thesaurus, submodels, traces, coverage, ontology, knowledge graph, search, summaries, and routes.
- File containers and modeled resources shall be represented as separate record families with explicit cross-references when the same path appears in both roles.
- Route definitions shall include canonical hash routes for current views and element/file/search detail workflows.
- Element-detail routes shall open a Project Store-backed scrollable modal/dialog in the Explorer shell instead of using the source content route as the primary element navigation target.
- Element-detail modal headers shall show only the actual element type as a text badge and shall not show an additional type-family/kind badge, marker dot, shape, or glyph when the element type is more specific.
- Opening a related element from within the element-detail modal shall show a compact back control whose accessible label names the previous element and shall not render a duplicate visible `From:` context line.
- Regular element-detail modals shall render model concept references as standard link-colored inline links on matching prose terms, with underline only on hover or focus, hide the authored `#### Concept References` source subsection, match native concept preferred labels, alternative labels, and authored reference labels, expose the concept IRI as tooltip/location metadata, and open the referenced native concept element modal when activated.
- Concept relation lists in the element-detail modal shall exclude the selected concept itself, and reciprocal ontology edges for the same concept pair shall be deduplicated before rendering.
- Element-detail modals shall expose source navigation as a secondary action using the source anchor.
- The Coverage route shall render the capability navigator and selectable Whole Model tree root under its owning contract, keeping complete summaries, breakdowns, drill-down and scoped issue lists in the main view.
- Separate Explorer/report document entry points shall not be generated.
- Missing or malformed store seed data shall be detectable by automated checks and visible to users as an Explorer diagnostic.

##### Test Criteria:
- Run `reqvire serve` on a minimal model with at least one capability, requirement, verification relation, satisfiedBy evidence file, contract_bindings or concept-reference fact, and ontology term when available.
- Run the Explorer component/unit tests that cover element-detail modal back context rendering, inline native concept-reference modal routing, alternative-label concept-reference matching, hidden authored concept-reference subsection rendering, and exclusion of self-referential concept relation rows from the modal.
- Parse the generated store seed from `index.html` or its referenced static asset.
- Assert all required top-level store sections exist.
- Assert the `project` section exposes effective workspace identity and available Git worktree metadata when running with source-control metadata, and that Explorer tree roots render top-level `Model` and `Resources` branches with Git worktree identity grouping folders.
- Assert at least one Markdown source path appears in `files`.
- Assert at least one implementation or evidence path referenced by `satisfiedBy` or contract_bindings appears in `resources`.
- Assert `files` records carry source navigation metadata while `resources` records carry referring-fact evidence.
- Assert canonical routes include primary `#/model`, specialist `#/ontologies` and `#/traces`, and supporting `#/files`, `#/files/<path>`, `#/coverage`, `#/resources`, `#/elements/<identifier>`, and `#/search`; do not require a separate Containment route or a separate Knowledge Graph route/page file.
- Assert at least one Explorer element link or search result targets `#/elements/<identifier>` and that the element-detail UI contains a modal/dialog marker plus a secondary source link.
- Assert element-detail modal headers render the actual element type as the only visible type badge and do not render a marker dot, shape, or glyph inside that badge; for example a `behavior` element shall not also show a separate `contract` badge.
- Assert the Model view List/Grid modes render from Project Store `folders` and `files` without an iframe or third-party file-manager widget, expose breadcrumb navigation, sortable file rows, grid cards, central workspace search, icon/color legends, source-page secondary actions, and modeled-element rows that open the shared element-detail modal. Assert clicking anywhere on a Grid mode folder/file card opens or selects that card's item, while the source-page secondary action remains a separate control. Assert `#/files` and `#/files/<path>` deep-link into that behavior without creating a separate primary Filesystem view.
- Assert the Model Graph mode paints the shell, graph canvas, and design-system spinner loading notice before deferred Sigma/ForceAtlas graph construction starts, retains a layout notice until its owned worker settles, computes full-graph ForceAtlas layout from currently visible nodes and edges with bounded settings derived from visible node count, edge density, and average rendered node size, and keeps layout quality while using cached adjacency/focus lookup for interaction.
- Hold Model Graph worker responses during filter and context replacement, including equal projection references from another worktree. Require obsolete success/failure to be ignored, cancellation on disposal, and failure/retry without rebuilding Sigma. Verify the shared engine bounds each owner to one active worker, cancels independently, and rejects incomplete, duplicate, or non-finite coordinates.
- Construct a large Model Graph with repeated known types, contract subtypes, and distinct unknown role names. Assert each canonical node/edge palette token is read once and produces the expected colors. Replace context with the same projection and changed palette values; require fresh construction colors. Change palette values while mounted and exercise a focused edge; require current color resolution without changing the graph's existing node colors.
- Execute the actual exported ForceAtlas worker asset and compare exact coordinates with the prior direct algorithm for empty, isolated, split/join, parallel-edge, and self-loop inputs. Check a malformed input fails, a 1,000-node job permits main-thread timer progress, and terminating a 2,000-node job leaves an independent worker functional.
- Open Model Graph and Ontologies in the served browser, verify local ForceAtlas worker URLs, at most one active worker per mounted graph, completed layout notices, and worker termination on route exit. Rapid ontology reset replacement must complete the newest job.
- Complete controlled full, filtered-subset, and empty Model Graph worker results. Require one bulk event containing every accepted `x`/`y` and `baseX`/`baseY` value with those field hints; empty input emits none. Check reversed result IDs, complete observer snapshots, untouched hidden-node coordinates/baselines and unrelated attributes, retained renderer identity, and no camera movement. Compare exact worker, rendered-position/baseline, visual-attribute, and finite Sigma display-cache checksums across cold/warm browser runs of both renderers before and after batching.
- Hold a Model Graph worker result with no selection, a focused neighborhood, or an isolated selection; then accept it and require one immediate focus-owned refresh, retained renderer identity, exact new baselines, and no camera movement/reset. Preserve explicit-selection centering and cancellation of the prior focus animation. Complete a real accepted focus animation and require its separate completion refresh, then clear selection and complete restoration; require exact baseline restoration with one immediate and one completion refresh and no camera movement. Retry a failed layout without rebuilding Sigma and require one accepted refresh.
- Assert selecting a Model Graph node computes a visible focused neighborhood, applies bounded Graphology no-overlap layout from stable post-ForceAtlas baseline coordinates to that neighborhood only with spacing scaled by neighborhood size, keeps unrelated graph nodes out of the layout update, restores nodes from the previous focus when they leave scope, centers selected nodes through Sigma display-coordinate mapping after focus layout updates without changing zoom, and animates resulting node coordinates through Sigma node animation without accumulating coordinate drift.
- Assert the Model tree, grid cards, modeled-element lists, relation/contract_bindings endpoints, and element legends use the shared Explorer `ElementIcon` type glyphs, that capability, semantic-contract, and verification-objective elements use their own role colors as plain squares with no glyph, that verification-objective uses the darker verification-objective token distinct from concrete verification, that inline concept-reference terms in element content use standard link color with no glyph or pill and underline only on hover or focus, that evidence-file artifacts and resource/evidence tree leaves use the neutral/default treatment rather than the yellow file/source resource token, and that contract-family subtypes keep the shared contract color while rendering distinct glyph marks for `source`, `specification`, `constraint`, `behavior`, `state`, and `input-output`.
- Assert selecting a folder, file, or modeled element in the left Model project tree updates the active Model workspace mode: List/Grid browse the selected folder or file, and Graph focuses the matching graph node when one exists; their modeled-element rows open the shared element-detail modal. Flow tree selection renders the selected relation scope in the main workspace, with element details opened separately from the Flow card.
- Assert the Search route's left-pane result-type controls do not render a duplicate passive legend for the same result-type colors and labels.
- Assert the Coverage route retains the shared left pane and header worktree control, with capability scope selection and an selectable Whole Model tree root. Its full main dashboard follows the owning Coverage contract and the left pane does not duplicate its summaries, breakdowns or issue lists.
- Assert the Explorer builds its ranked search index in a browser worker after the initial shell render, keeps non-search Explorer views interactive during indexing, and returns BM25-style ranked results that prioritize title matches over path/result-kind matches and body/content matches.
- Assert prefix and fuzzy search terms can find matching Project Store search documents, and result-kind controls filter ranked results without rebuilding the index.
- Assert `index.html` loads local compiled SPA bundle/stylesheet assets and contains no framework/styling CDN reference and no runtime CSS compiler.
- Assert canonical SPA routes are sufficient for Explorer navigation and that no retired Explorer page adapters or separate Explorer/report implementations are emitted.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-explorer-build/test.sh)
  * satisfiedBy: [loadStore.test.ts](../../../../explorer/src/store/loadStore.test.ts)
  * satisfiedBy: [useLiveStore.test.ts](../../../../explorer/src/store/useLiveStore.test.ts)
  * satisfiedBy: [VisualizationParity.test.tsx](../../../../explorer/src/views/VisualizationParity.test.tsx)
  * satisfiedBy: [OntologiesView.test.tsx](../../../../explorer/src/views/OntologiesView.test.tsx)
  * satisfiedBy: [forceAtlasLayout.test.ts](../../../../explorer/src/lib/forceAtlasLayout.test.ts)
  * satisfiedBy: [forceAtlasLayoutEngine.test.ts](../../../../explorer/src/workers/forceAtlasLayoutEngine.test.ts)
  * satisfiedBy: [check-forceatlas-worker.mjs](../../../../tests/test-export-command/check-forceatlas-worker.mjs)
  * satisfiedBy: [route-check.mjs](../../../../tests/test-serve-command/scripts/route-check.mjs)
  * verify: [SPA Explorer Shell and Project Store](../../../Interfaces/WebExplorer/Capabilities.md#spa-explorer-shell-and-project-store)
---

### Serve Command Verification

This test verifies that the serve command starts an HTTP server for the embedded Explorer and generated model runtime data.

#### Details

##### Acceptance Criteria:
- System shall start HTTP server on specified host and port
- System shall display clickable terminal link to the server URL
- System shall serve index.html when accessing root URL
- System shall serve embedded Explorer assets and generated Project Store data with correct paths
- System shall serve generated `ontologies.ttl`
- System shall expose `/mcp` on the same listener only when `--enable-mcp` is present
- System shall expose embedded MCP mutation tools only when both `--enable-mcp` and `--enable-mutations` are present
- System shall return index.html for non-asset browser routes
- System shall not return Explorer `index.html` for `/mcp` requests when embedded MCP is enabled
- System shall return 404 for missing asset paths
- System shall set correct Content-Type headers for different file types
- System shall run in quiet mode without verbose runtime-generation output
- System shall not automatically open browser window
- System shall display instructions for Ctrl-C stop

##### Test Criteria:
- Command starts successfully and displays server URL with instructions
- Server responds to HTTP requests on specified port
- Root URL (/) serves index.html
- Decode the served Project Store seed and assert modeled source files and resource-only evidence are in their respective collections, regardless of JSON whitespace. Retain the response and fixture when a check fails.
- HTML files are served with text/html content type
- SVG files are served with image/svg+xml content type
- Missing embedded asset paths return 404 status
- Non-asset browser routes return index.html for SPA fallback
- `reqvire serve --enable-mcp` accepts MCP protocol requests at `/mcp` while root and SPA routes still serve Explorer content
- `reqvire serve --enable-mcp` omits mutation tools from MCP `tools/list`
- `reqvire serve --enable-mcp --enable-mutations` includes mutation tools in MCP `tools/list`
- After an embedded MCP mutation changes model files, a subsequent `assets/project-store.js` request returns regenerated Project Store data that reflects the current workspace.
- Runtime data responses include no-store cache control to avoid stale browser datastores after mutation.
- `reqvire serve --enable-mutations` without `--enable-mcp` fails CLI argument validation
- Runtime-generation verbose output is suppressed (quiet mode active)
- Count original model and runtime builds during actual CLI startup: one of each for plain, read-only MCP, and writable serving with either commit policy. Reject an invalid original model before listener startup and reject dirty writable admission before any model/runtime build. Exercise aggregate workspaces outside Git with eligible child repositories in plain and read-only MCP modes; reject writable aggregate admission before model/runtime construction. Include invalid aggregates and empty workspaces. Use a deliberately invalid listener address to inspect the startup pipeline without requiring a socket.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [test.sh](../../../../tests/test-serve-command/test.sh)
  * verify: [Serve Command](../../../Interfaces/WebExplorer/Capabilities.md#serve-command)
  * satisfiedBy: [startup-check.py](../../../../tests/test-serve-command/scripts/startup-check.py)
  * verify: [Serve Command Embedded MCP Endpoint](../../../Interfaces/WebExplorer/Capabilities.md#serve-command-embedded-mcp-endpoint)
---

### Served Explorer Runtime Freshness Verification

Verify that immutable published runtime data and its manifest adopt embedded MCP mutations while preserving the running server.

Browser traffic assertions and missing/corrupt chunk injection must recognize worktree-selected endpoint URLs, including their query parameters. Confirm the intended fault was applied before asserting failure recovery.

#### Details

##### Acceptance Criteria
- Mutation-enabled embedded MCP serving exposes changed model records after successful MCP additions and edits.
- Changed model content changes the published manifest revision; unchanged conditional requests return `304` with an empty body and retain the revision.
- The manifest revision and chunk hashes match independent SHA-256 calculations over the exact transferred UTF-8 JSON text. Array order, repeated references, empty arrays, Unicode, numeric representations, unknown sections, and deletions survive manifest reconstruction.
- Chunk batches contain only requested chunks and deduplicate repeated requests. Invalid hashes, oversized requests, absent chunks, and superseded revisions return their specified errors.
- A captured snapshot retains its original chunks after a newer snapshot is published; an HTTP request using the superseded revision returns `409`.
- Manifest reads return the cached snapshot while the actual parent worker control gate is held. Valid deeply nested generated JSON does not gain an additional manifest-construction depth limit.
- Manifest HEAD responses have no body and retain revision headers. Strong, weak, list, and wildcard conditional tags return the expected statuses; unsupported methods on the full-store route return `405`, and missing API paths return `404`.
- Generated seed data, the full JSON store, and the manifest revision agree after mutations, and generated ontology data remains available.
- Verify context labels exist before worker serialization, the pipe carries a structured store rather than nested JSON text, and parent publication consumes the owned runtime payload. Count store serialization work for seed/full JSON and preserve the exact existing manifest/chunk goldens. Exercise script-sensitive characters, Unicode, numeric values, deep nesting, context identity, and failed-runtime retention through the publication boundary.
- Failed parent runtime publication returns `503` for manifest and seed endpoints while retaining the last valid captured snapshot and accepted MCP model reads. Regenerating the runtime from accepted worker inputs clears the diagnostic and permits a matching conditional `304` without advancing identical runtime content's revision.
- Plain serving and read-only embedded MCP expose snapshot APIs for branch selection but do not advertise periodic live refresh.
- The same server processes remain alive and one initialized MCP client can issue successful reads after each runtime refresh without reinitialization.

##### Test Criteria
- Use the standard serve shell suite's temporary Git workspace, fixture files, expected output, and real headless browser driver.
- Exercise updates through mutation-enabled embedded MCP and use plain and read-only embedded MCP servers to check refresh gating.
- Compare conditional live responses and revisions, independently hash a manifest and returned raw chunk text, check the served seed against the full JSON store, and verify continued MCP reads.
- Run the `live_store::tests` and `serve::tests` modules in the CLI Rust unit suite for reconstruction, deduplication, request errors, deeply nested generated JSON, and actual worker-backed immutable snapshot publication, held-control-gate reads, and cached-error recovery.
- Pin representative serialized chunk, ontology, and manifest bytes and their pre-extraction SHA-256 values. After consolidation into the shared core primitive, require identical bytes, digests, ETags, and the existing manifest protocol; keep the fixture values independent of the production helper.
- Check that existing Rust Explorer hashing and model revision callers use the one shared core primitive and that the CLI no longer has a second SHA-256/hexadecimal implementation. The existing browser verifier remains independent and validates the unchanged wire contract.
- The existing `live_store::tests` module pins an independently calculated complete manifest and revision, including hashes for Unicode/numeric chunks and ontology bytes. The existing serve and browser refresh suites verify the shared primitive's wire compatibility through the same runtime and protocol.
- Require exact equality with the expected refresh result file; preserve browser and request diagnostics on failure.

##### Cache Correctness Regression Scope
MCP Cache and Runtime Coherence Verification owns the additional rejected-mutation/preview refresh gating and superseded-build publication cases. Its cache integration and adapter regressions pass. Existing live-store and browser evidence establishes wire/snapshot behavior; the dedicated cache and adapter assertions establish source-cache/publication guarantees.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Web Explorer Interface Verification Objective](#web-explorer-interface-verification-objective)
  * satisfiedBy: [live_store.rs](../../../../crates/reqvire-cli/src/live_store.rs)
  * satisfiedBy: [explorer_runtime.rs](../../../../crates/reqvire-core/src/explorer_runtime.rs)
  * satisfiedBy: [mcp_worktrees.rs](../../../../crates/reqvire-cli/src/mcp_worktrees.rs)
  * satisfiedBy: [serve.rs](../../../../crates/reqvire-cli/src/serve.rs)
  * satisfiedBy: [test.sh](../../../../tests/test-serve-command/test.sh)
  * verify: [Served Explorer Runtime Freshness](../../../Interfaces/WebExplorer/Capabilities.md#served-explorer-runtime-freshness)
---

### Thesaurus Project Store Projection Verification

This test verifies that the Thesaurus Explorer route is backed by a native Project Store thesaurus projection instead of ontology graph provenance.

#### Details
- Render the navigator with a valid Thesaurus projection but no ontology graph concepts; confirm canonical concept labels and scheme membership remain available. Exercise multiple schemes, external source identity, missing/cross-scheme parents, deep ancestry, defensive cycle handling, search ancestor retention, and selected-branch expansion.
- Count parent-field reads during hierarchy preparation on a large fixture and confirm linear reads; repeated consumers of the same immutable projection reuse the prepared index, while a replacement projection refreshes it.

##### Acceptance Criteria:
- The exported Project Store shall include a top-level `thesaurus` projection with `schemes` and `concepts`.
- Each concept-scheme row shall preserve distinct SKOS identity and native Reqvire `concept-scheme` element identity.
- Each concept row shall preserve distinct SKOS identity and native Reqvire `concept` element identity.
- Concept rows shall expose scheme membership, taxonomy parent identity, related concept identity, SKOS authoring fields, source navigation, model usage, and ontology mapping usage needed by the Thesaurus route.
- Thesaurus map activation shall use native concept or concept-scheme element IDs from the `thesaurus` projection, not ontology graph node source/provenance data.
- Ontology bridge evidence from `reqvire:mapsToConcept` shall appear as mapping usage without making the mapped ontology term the concept's navigation target.

##### Test Criteria:
- Export a model containing a native concept scheme, native concepts, concept taxonomy, related concepts, and an ontology term mapped to one concept through `reqvire:mapsToConcept`.
- Parse `assets/project-store.js` and assert the top-level `thesaurus` projection exists.
- Assert concept-scheme `element_id` resolves to a Project Store element with type `concept-scheme`.
- Assert each concept `element_id` resolves to a Project Store element with type `concept`.
- Assert the narrower concept's `parent_id` references the broader concept SKOS id.
- Assert related concept ids, labels, scope note, source href, and ontology `maps_to` evidence are preserved.
- Assert the concept `element_id` differs from ontology graph provenance for the same SKOS concept node, preventing Thesaurus map clicks from opening ontology elements.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [thesaurus.test.ts](../../../../explorer/src/lib/thesaurus.test.ts)
  * satisfiedBy: [VisualizationParity.test.tsx](../../../../explorer/src/views/VisualizationParity.test.tsx)
  * derivedFrom: [Web Explorer Interface Verification Objective](#web-explorer-interface-verification-objective)
  * satisfiedBy: [test.sh](../../../../tests/test-thesaurus-project-store/test.sh)
  * verify: [Thesaurus View Generation](../../../Interfaces/WebExplorer/Capabilities.md#thesaurus-view-generation)
---

### Explorer Semantic Query Presentation Verification

Verify managed query identity and vocabulary context across Explorer and the shared design system.

#### Details
Check that semantic-query icons and badges use the shared Q marker, that native queries occur once in ontology graph data with source provenance and vocabulary/output edges, and that graph rendering preserves their circular Q marker, name, filter behavior, and property dependency targets. Inspect query form, exact text, used ontologies, declared outputs, and source navigation in the detail dialog. Exercise exported Project Store generation from a native query fixture. Verify that query hover and selection render `uses vocabulary` and `declares output` connectors with their property targets, that query/layer filters hide them, and that re-enabling filters restores eligible focused relations.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Web Explorer Interface Verification Objective](#web-explorer-interface-verification-objective)
  * verify: [OWL Semantic Ontology Projection](../../../Interfaces/WebExplorer/Capabilities.md#owl-semantic-ontology-projection)
  * verify: [Ontology Property-Centric Visualization](../../../Interfaces/WebExplorer/Capabilities.md#ontology-property-centric-visualization)
  * verify: [Ontology Symbol and Badge Vocabulary](../../../Interfaces/WebExplorer/Capabilities.md#ontology-symbol-and-badge-vocabulary)
  * satisfiedBy: [semantic_queries.rs](../../../../crates/reqvire-core/tests/semantic_queries.rs)
  * satisfiedBy: [OntologiesView.test.tsx](../../../../explorer/src/views/OntologiesView.test.tsx)
  * satisfiedBy: [OntologyNodeDetailModal.test.tsx](../../../../explorer/src/components/OntologyNodeDetailModal.test.tsx)
  * satisfiedBy: [test.sh](../../../../tests/test-semantic-queries/test.sh)
---

### Explorer Worktree Selection Verification

Verify one branch-browsing experience backed by independent worktree contexts with plain serving, read-only MCP, and writable MCP.

#### Details
Acceptance checks:
- Start two admitted contexts with different models and assets but overlapping relative filenames and element identifiers. Verify the shell selector uses shared controls, labels branches, supports keyboard selection, and lists local branches without loading them or creating worktrees during inventory reads. Mutation-enabled first selection must complete ordinary admission before loading its target.
- Confirm one compact branch picker follows the Reqvire brand in the shared header across all views, including when the left pane is collapsed or resized. No separate full-width branch toolbar or repeated "Viewing" label is rendered. Check light/dark themes, constrained viewports and long branch/path values: the trigger truncates while complete values remain inspectable in the opened menu and tooltip. Header tabs reflow into a separate scrollable row at constrained widths; the picker, menu and header actions remain usable without horizontal clipping. Assert production and design-system mocks share the header context slot and compact picker.
- Exercise arrow keys, Home/End, type-ahead, Enter/Space, Escape, Tab, and outside dismissal. Unavailable choices cannot be selected. During pending and failed switches, the trigger and selection marker continue identifying the displayed model even when worktrees share a branch label.
- In design-system Patterns, select different available branches and verify labels and disabled unavailable choices. In Mocks, switch between distinct fixture models from the same shell selector; assert the model and branch change together, view navigation remains available, prior-context modals close, and URL/history/reload retain the chosen fixture without contacting MCP.
- Open two browser tabs, select different contexts, and exercise Model, Flow, Ontologies, Traces, Coverage, search, element modals, source pages, and asset downloads. Each result and label must belong to the selected context; neither selection changes Git checkouts, MCP routing, or the other tab.
- Mutate each context through MCP and verify only its selected views refresh; test both automatic-commit policies. Confirm inventory reads do not scan/build models and requests for initialized runtimes do not rebuild them.
- Delay a previous context's manifest, chunks, source response, and asset request while switching. Confirm target publication is atomic, late responses cannot overwrite it, and cache keys include context even when fingerprints match. Validate path traversal and wrong-context requests are rejected.
- Check URL selection, reload, back/forward, per-context coverage scope, retained view mode, closed old-context modals, and invalid selection explanations. Remove or fail a selected context and assert a labelled stale/unavailable view rather than fallback to another branch.
- Hold target loading and assert a centered loading dialog, branch label, spinner, inert background and retained model; pending outside/Escape interaction cannot dismiss it. Release successful publication and assert the new model appears and dialog closes. Inject a failure, verify the diagnostic and dismiss through Close, Escape and outside click; retain the prior model and restore its URL/focus. Periodic refresh must not retry the failed selection; explicit retry and superseding requests must recover without stale adoption.
- Inject target runtime generation, manifest, chunk, and network failures; keep the prior valid model and label together, then recover by explicit selection. Check no switch tool appears in MCP discovery.
- Visit, remove, and recreate many context IDs with overlapping element identifiers. After successful inventory, revisiting an inactive absent ID requires a fresh client and complete chunk validation; listed IDs retain their cached identities. Keep displayed/selected clients through pending and failed switches, then release absent prior contexts once unprotected. Failed or malformed inventories, HTTP-success inventory error payloads, and late cancelled/timed-out responses cannot evict clients or replace choices. A delayed inventory cannot evict clients created after that request began. Pruning itself issues no model loads or chunk downloads.
- Populate several existing worktrees and branches without worktrees, including an invalid unselected model. Measure worker starts and model/runtime builds: startup loads only the original context, inventory loads none, and selecting a branch loads only that target. In every serving mode, the same picker lists all choices without prevalidation. First selection of a branch without a worktree creates one managed worktree, applies the mode's admission rules, and loads its model, ontology and assets through the common pipeline. Repeated unchanged and concurrent selections reuse its worktree, completed model, and runtime. Changing its local branch tip before first selection uses the current tip.
- Confirm preparing a missing worktree leaves existing checkouts, indexes, files, branch refs, and other clients' routing unchanged while registering only the required worktree. Read-only selection takes no ownership; mutation-enabled selection takes ownership only through normal admission. Race preparation with another checkout and require reuse or an explicit conflict without forcing duplicate checkouts. Failed preparation releases only ownership acquired by that attempt and cleans up only newly created unchanged assets, preserving pre-existing work and reporting residual paths. Normal shutdown preserves successful worktrees and their changes.
- In plain and read-only embedded serving, exercise the same loading and cache checks against reused and newly created worktrees. Include valid dirty content, repeat unchanged selections, then edit selected-context sources, root exclusions, and used external ontology inputs. The next demand load/read must follow existing invalidation and rebuild rules without restart; equal-length preserved-timestamp edits remain detectable. Invalid changed inputs return diagnostics rather than stale success, and repaired inputs recover. Other branches are not scanned or loaded, mutation tools remain unavailable, and no new periodic model polling is introduced. Static exports retain one snapshot.
- In mutation-enabled serving, attempt first selection of targets with staged, unstaged, and non-ignored untracked changes. Each fails before model parsing/runtime construction, retains the displayed branch and all files/index/refs, and offers no read-only fallback. After changes are resolved, normal admission validates and loads the branch; author-identity and ownership conflicts retain their existing failure behavior. Exercise both reused and newly created worktrees.
- With either automatic-commit policy, reselect a healthy active context and reuse its accepted model without source freshness scans or rebuilds. With commits disabled, accepted uncommitted writes must not cause a new clean-start failure on reselection. External edits must not be imported into that accepted model. Accepted mutations update only their context's runtime and derived cache state. Stop and restart with pending changes to confirm the existing clean-start rule still rejects admission.

##### Evidence scope
The Rust worktree and HTTP router regressions exercise on-demand admission, dirty-target rejection, accepted-snapshot reuse, read-only source/exclusion/preview invalidation, and repair recovery. Explorer hook and application tests cover explicit loading, cached periodic reads, failed-switch retention, and retry selection. The HTTP/browser cases in `tests/test-mcp-worktrees` execute the compiled Explorer in plain, read-only embedded and writable serving, including held target loading, scrim/background behavior, error dismissal and independent-tab navigation.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Web Explorer Interface Verification Objective](#web-explorer-interface-verification-objective)
  * satisfiedBy: [mcp_worktrees.rs](../../../../crates/reqvire-cli/src/mcp_worktrees.rs)
  * satisfiedBy: [serve.rs](../../../../crates/reqvire-cli/src/serve.rs)
  * satisfiedBy: [WorktreeLoadDialog.test.tsx](../../../../explorer/design-system/product-patterns/shell/WorktreeLoadDialog.test.tsx)
  * satisfiedBy: [useLiveStore.test.ts](../../../../explorer/src/store/useLiveStore.test.ts)
  * satisfiedBy: [App.test.tsx](../../../../explorer/src/App.test.tsx)
  * verify: [Explorer Worktree Selection](../../../Interfaces/WebExplorer/Capabilities.md#explorer-worktree-selection)
---

### Explorer Worktree Runtime Isolation Verification

Verify that the served Explorer runtime fulfills its shared context-isolation and live-store implementation obligations.

#### Details
- After a real worker persistence/recovery failure, keep the selected accepted store, manifest/chunks and ontology bytes readable with the same hashes. Assert recovery metadata and `X-Reqvire-Recovery-Required` on both changed and `304` manifest responses. Raw local assets must reject with that context's diagnostic. Worker exit still makes the context unavailable; another context remains usable.
- In Explorer hook and shell tests, retain the last accepted snapshot and show the recovery warning through unchanged polls and fresh loads. Switching contexts must not carry another branch's warning; failed or aborted requests must not overwrite adopted context state. The warning must identify disabled writes without claiming that a successful snapshot read failed.
Acceptance checks:
- In writable and read-only serving, admit two contexts with colliding identifiers and filenames but different content. Assert inventory, seed, manifest, chunks, full store, ontology download, source rendering, and eligible asset routes return only the selected context and carry matching identity.
- Exercise missing selectors on legacy original-context routes and explicit unknown, removed, stopped, and runtime-unavailable IDs. Explicit failures must never return another branch's data or the SPA fallback shell. Reject path traversal, encoded traversal, and symlink escapes under the existing asset boundary.
- Confirm reused and newly created worktrees initialize on demand through the same fixed-root worker pipeline after the mode's admission checks, with concurrent identical loads sharing one preparation outside the asynchronous request executor. Startup/inventory never initialize unrequested worktrees. Unchanged read-only demand loads reuse existing core cache entries; changed inputs rebuild only the selected context under existing invalidation rules. Mutation-mode reads reuse accepted snapshots; manifest/chunk transfer and inventory do not initiate model scans/builds. Successful mutations replace only their own runtime, failed generation retains labelled last-valid data, and a later allowed initialization/refresh recovers.
- Interleave publication and context removal with in-flight reads; each response retains one captured immutable snapshot or reports that context unavailable. Identical model fingerprints must not conflate context identity or request routing.
- Verify branch/HEAD status after explicit commit or push without misreporting a model revision change, and exercise both commit policies. Backend requests cannot change another browser's selection or switch an existing Git checkout; preparing a missing worktree is isolated to the selected branch.
- Use disposable Git worktrees with actual private worker processes and the embedded HTTP router for backend checks. Run the HTTP/browser E2E suite against the built binary for transport and browser integration.
- Construct the serving state through the production startup path and confirm Git-backed modes retain only their worker-backed runtime. Initial seed, store, manifest and chunk responses must agree with that published snapshot and unchanged reads must reuse it.

#### Metadata
  * type: test-verification

#### Relations
  * satisfiedBy: [useLiveStore.test.ts](../../../../explorer/src/store/useLiveStore.test.ts)
  * satisfiedBy: [App.test.tsx](../../../../explorer/src/App.test.tsx)
  * satisfiedBy: [mcp_worktrees.rs](../../../../crates/reqvire-cli/src/mcp_worktrees.rs)
  * satisfiedBy: [serve.rs](../../../../crates/reqvire-cli/src/serve.rs)
  * derivedFrom: [Web Explorer Interface Verification Objective](#web-explorer-interface-verification-objective)
  * verify: [Explorer Worktree Runtime Isolation](../../../Interfaces/WebExplorer/Capabilities.md#explorer-worktree-runtime-isolation)
---

### Explorer Shareable Selection Navigation Verification

Verify persistent selections through the shared route and preference contract in served and exported Explorer views.

#### Details
Acceptance checks:
- Round-trip canonical element identifiers, file paths and semantic IRIs containing fragments, Unicode, spaces, percent and plus characters. Model mode and selection, Traces verification/file overview, Thesaurus canonical concept and Ontologies node must resolve correctly; Coverage scope retains its owning browser checks.
- Explicit links override saved preferences. Bare routes resume valid context preferences. Select twice and assert one history entry, then exercise copied links, reload, Back/Forward, view return and element-modal close. Traces file/verification updates must be atomic.
- Reject wrong-kind, missing and deleted identities with an explained overview fallback and replacement history. Exercise overlapping identifiers in different worktrees and avoid rewriting the target selection from the retained model while loading.
- Functional browser checks compare selected identifiers, routes and published data in actual served/exported bundles; visual screenshot comparison is not required.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Web Explorer Interface Verification Objective](#web-explorer-interface-verification-objective)
  * verify: [Explorer Shareable Selection Navigation](../../../Interfaces/WebExplorer/Capabilities.md#explorer-shareable-selection-navigation)
  * verify: [Model Selection Sharing](../../../Interfaces/WebExplorer/Capabilities.md#model-selection-sharing)
  * verify: [Trace Selection Sharing](../../../Interfaces/WebExplorer/Capabilities.md#trace-selection-sharing)
  * verify: [Thesaurus Selection Sharing](../../../Interfaces/WebExplorer/Capabilities.md#thesaurus-selection-sharing)
  * verify: [Ontology Selection Sharing](../../../Interfaces/WebExplorer/Capabilities.md#ontology-selection-sharing)
  * satisfiedBy: [routes.test.ts](../../../../explorer/src/router/routes.test.ts)
  * satisfiedBy: [SelectionNavigation.test.tsx](../../../../explorer/src/state/SelectionNavigation.test.tsx)
  * satisfiedBy: [browser-flow.mjs](../../../../tests/test-verification-traces/browser-flow.mjs)
---
