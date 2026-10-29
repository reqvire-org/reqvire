# Graph presentation

`KnowledgeGraphCanvas` and `OntologyGraphScreen` render the production graph
implementations. They consume graph DTOs, filter/selection state and callbacks;
application adapters retain store access, routing and cached projections.
`ModelGraphControls` and `OntologyControlsPane` own the corresponding side panes.
The showcase supplies the same DTOs and APIs with local fixture state.

The public entry must remain safe for non-graph consumers. Sigma and custom GPU
programs load through dynamic imports when a graph mounts. `mountOntologyGraph`
returns a Promise of the renderer handle and accepts an optional abort signal;
a consumer that unmounts while loading must discard/destroy a late handle.
`OntologyGraphScreen` owns this cancellation and cleanup.

ForceAtlas layout keeps the existing bounded profiles, graph inputs and worker
protocol. Every finite job owns its worker; completion or cancellation releases
it. `ForceAtlasLayoutOwner` validates identities and retires superseded/context
jobs so stale results cannot alter the mounted graph. Selection, focus and
filter changes reuse the existing renderer and accepted positions. GPU and
animation resources are disposed by the owning implementation.

Renderer-only style writes are narrowly declared in `ds.config.json`. Visual
policy still lives in token-backed Linaria styles and palette helpers.

Existing `src/views/OntologiesView.test.tsx`, `VisualizationParity.test.tsx`
and `src/workers/forceAtlasLayoutEngine.test.ts` verify renderer reuse, layout
inputs/profiles, cancellation, failures, context retirement and disposal.
Run `npm test` plus application/showcase builds, then inspect graph worker
requests and interaction in the browser. No WebGL shim belongs in the public
entry or non-graph runtime.

## ForceAtlas browser profiling

From the repository root, export a current Project Store, then run:

```sh
./target/debug/reqvire export --output /tmp/reqvire-profile-seed
node explorer/scripts/profile-forceatlas.mjs \
  /tmp/reqvire-profile-seed/assets/project-store.js /tmp/reqvire-forceatlas-profile
```

Use new output directories. The profiler builds an isolated Explorer with
instrumentation around the actual Model Graph and Ontologies layout calls.
It records main-thread preparation and coordinate application, worker-request
wall time (including startup, transport, and computation), browser timer/frame
gaps and progress during pending work, visible/input dimensions, source hashes,
and coordinate checksums in `results.json`. Three runs per graph include a new
document and two route reentries; fixtures cover the current model, bounded
100–2,000-node trees, a denser graph, and hidden ontology nodes. Timings are
diagnostic observations; timer/frame gaps include the surrounding renderer task.
The probe checks topology dimensions, finite coordinates, and cold/warm parity
without timing thresholds. Larger fixtures must permit both timer and frame
progress while the worker is pending. It requires the shared test browser configuration
(`REQVIRE_TEST_BROWSER`, `CHROME_BIN`, or Chrome/Chromium on PATH).
`FORCEATLAS_PROFILE_SIZES=100 FORCEATLAS_PROFILE_TRIALS=1` provides a smaller run.

Synchronous phase counters separate projection/index preparation, seed positions,
graph population, palette/glyph resolution, Sigma startup, and accepted-coordinate,
baseline, overlap, focus, and refresh work. Phase wall times are inclusive: nested
phases such as palette resolution within graph population must not be summed.
Counters cover startup through 350 ms after accepted layout, including scheduled
focus-animation completion. Palette counts describe instrumented direct API
invocations; CSS token aliases can require additional underlying style reads.
The probe records focus animation target counts, rendered coordinates/stable
baselines, and a checksum of rendered node/edge colors, sizes, labels, types,
visibility, glyphs, and curvature. This checks visual attributes,
without claiming pixel/rendering parity. Seed and instrumentation hashes make
before/after comparisons reproducible.

Set `FORCEATLAS_PROFILE_SIGMA=1` for diagnostic counters around the actual
Sigma process, render, full/partial refresh, resize, clear, and label calls,
plus WebGL shader, buffer, texture, draw, and readback submissions. The probe
also checks every node's finite Sigma display-cache coordinates and records
their checksum, the actual WebGL renderer identity, document long tasks, calls
lasting at least 2 ms, and the intervals containing the largest timer/frame
gaps. Calls from a retired renderer/context cannot contribute to its route
replacement's counters. Direct WebGL timings measure synchronous submission;
they do not measure asynchronous GPU execution. Sigma counters include internal
graph-event and image-atlas refreshes; they differ from the explicit caller
refresh counter. Nested counters remain inclusive.

`FORCEATLAS_PROFILE_TRACE=1` enables those counters and captures a Chrome trace
per fixture/renderer, covering its cold and warm runs. Traces include renderer,
worker, GPU, compositor, and task events, with `reqvire:` phase/layout markers
for correlation. Trace completion has a bounded wait. Use
`FORCEATLAS_PROFILE_FIXTURES=current-model` to limit this larger diagnostic
capture. Compare traced results for correctness and attribution separately from
untraced timings: tracing adds overhead. Output directories retain the profiler
and browser-driver source snapshots and their hashes alongside renderer and
seed hashes.

For a saved baseline, set `FORCEATLAS_PROFILE_SOURCE_ROOT` to a directory
containing the repository-relative copies of the two renderer source files.
The isolated diagnostic build substitutes only those sources and records their
actual hashes; their other imports still use the current checkout. This enables
controlled comparisons without replacing files in the shared working tree.

The shared browser driver defaults to SwiftShader. For a native GPU probe, set
`FORCEATLAS_PROFILE_GPU=native`; this selects headless ANGLE/Vulkan using
Chrome's [documented native GPU flags](https://developer.chrome.com/blog/supercharge-web-ai-testing).
The profiler requires a working application WebGL context and rejects a software
renderer fallback using its unmasked renderer identity. A launch flag alone
does not establish hardware use. Host driver/device compatibility still governs
whether this mode works; a failed native probe does not provide native timing
evidence. The ordinary shell browser checks retain their SwiftShader default.
