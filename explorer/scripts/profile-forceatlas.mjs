// Diagnostic only: build the actual Explorer into an isolated output directory,
// instrument its two owned worker requests, and run them in a real browser.
// Usage (from the repository root, after `reqvire export --output /tmp/seed`):
// node explorer/scripts/profile-forceatlas.mjs /tmp/seed/assets/project-store.js /tmp/profile
// Optional bounded smoke run: FORCEATLAS_PROFILE_SIZES=100 FORCEATLAS_PROFILE_TRIALS=1
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile, writeFile, mkdir } from 'node:fs/promises';
import { createServer } from 'node:http';
import os from 'node:os';
import path from 'node:path';
import vm from 'node:vm';
import { build } from 'vite';
import ts from 'typescript';
import { withBrowser, waitFor } from '../../tests/browser.mjs';

const root = path.resolve(import.meta.dirname, '..');
const [seedPath, outputPath] = process.argv.slice(2);
assert(seedPath && outputPath, 'Expected exported project-store.js and a new output directory');
const output = path.resolve(outputPath);
await mkdir(output); // Never overwrite an existing profile or production output.
const site = path.join(output, 'site');
const seedScript = await readFile(seedPath, 'utf8');
const seedContext = { window: {} };
vm.runInNewContext(seedScript, seedContext, { timeout: 5000 });
const seed = JSON.parse(JSON.stringify(seedContext.window.reqvireProjectStore));
assert(seed?.knowledge_graph && seed?.ontology?.graph_data, 'Expected a complete exported store');
const sizes = (process.env.FORCEATLAS_PROFILE_SIZES ?? '100,500,1000,2000').split(',').map(Number);
const trials = Number(process.env.FORCEATLAS_PROFILE_TRIALS ?? 3);
const gpuBackend = process.env.FORCEATLAS_PROFILE_GPU ?? 'swiftshader';
const traceProfile = process.env.FORCEATLAS_PROFILE_TRACE === '1';
const sigmaProfile = process.env.FORCEATLAS_PROFILE_SIGMA === '1' || gpuBackend === 'native' || traceProfile;
assert(['swiftshader', 'native'].includes(gpuBackend), 'GPU must be swiftshader or native');
assert(sizes.every(n => Number.isInteger(n) && n >= 2 && n <= 2000), 'Sizes must be 2..2000');
assert(Number.isInteger(trials) && trials >= 1 && trials <= 5, 'Trials must be 1..5');
const files = {
  knowledge: path.join(root, 'src/views/GraphLibraryViews.tsx'),
  ontology: path.join(root, 'src/lib/ontologyGraphRenderer.ts'),
};
// Read saved renderer sources for an isolated baseline without replacing files
// in the shared checkout. All other imports still come from the current tree.
const sourceRoot = process.env.FORCEATLAS_PROFILE_SOURCE_ROOT;
const sources = Object.fromEntries(await Promise.all(Object.entries(files).map(async ([kind, file]) =>
  [kind, await readFile(sourceRoot ? path.join(sourceRoot, path.relative(path.dirname(root), file)) : file, 'utf8')])));
const instrumentationSource = await readFile(import.meta.filename);
const browserDriverSource = await readFile(path.join(root, '../tests/browser.mjs'));
await writeFile(path.join(output, 'instrumentation.mjs'), instrumentationSource);
await writeFile(path.join(output, 'browser-driver.mjs'), browserDriverSource);
const instrumented = new Set();

// Locate syntax rather than copying layout formulas or algorithms into a probe.
// Fail closed if the production call structure changes.
function instrument(source, file, kind) {
  const ast = ts.createSourceFile(file, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
  const calls = [], declarations = [], constructors = [];
  function visit(node) {
    if (ts.isCallExpression(node)) calls.push(node);
    if (ts.isNewExpression(node)) constructors.push(node);
    if (ts.isVariableStatement(node)) {
      declarations.push(node);
    }
    ts.forEachChild(node, visit);
  }
  visit(ast);
  const matching = name => calls.filter(call => call.expression.getText(ast) === name);
  const run = matching('layoutOwner.run');
  assert.equal(run.length, 1, `${kind}: expected one owned ForceAtlas request`);
  const named = name => declarations.filter(statement => statement.declarationList.declarations.some(d => d.name.getText(ast) === name));
  assert.equal(named('layoutGraph').length, 1, `${kind}: expected one layout graph`);
  const edits = [
    [named('layoutGraph')[0].getStart(ast), named('layoutGraph')[0].getStart(ast), `globalThis.__forceAtlasProfile.begin(${JSON.stringify(kind)});\n`],
    [run[0].expression.getStart(ast), run[0].expression.end,
      `globalThis.__forceAtlasProfile.run.bind(null, ${JSON.stringify(kind)}, layoutOwner)`],
    [run[0].arguments[0].end, run[0].arguments[0].end, ', graph'],
  ];
  const phase = (node, name, targets) => {
    edits.push([node.getStart(ast), node.getStart(ast), `globalThis.__forceAtlasProfile.measure(${JSON.stringify(kind)}, ${JSON.stringify(name)}, () => (`]);
    edits.push([node.end, node.end, `)${targets ? `, Object.keys(${targets}).length` : ''})`]);
  };
  const sigma = constructors.filter(node => node.expression.getText(ast) === 'Sigma');
  assert.equal(sigma.length, 1, `${kind}: expected one Sigma constructor`);
  phase(sigma[0], 'sigma-startup');
  if (sigmaProfile) {
    edits.push([sigma[0].getStart(ast), sigma[0].getStart(ast), `globalThis.__forceAtlasProfile.constructSigma(${JSON.stringify(kind)}, Sigma, () => (`]);
    edits.push([sigma[0].end, sigma[0].end, '))']);
  }
  const phases = {
    buildGraphData: 'projection', assignInitialPositions: 'seed', assignInitialSigmaPositions: 'seed',
    cssVar: 'palette', roleColorValue: 'palette', queryGlyphImage: 'glyph', constructGlyphImage: 'glyph',
    'positions.forEach': 'position-apply', recordOntologyLayoutBaseline: 'baseline',
    separateOverlappingSigmaNodes: 'overlap', runFocusedLayout: 'focus-layout', runFocusedOntologyLayout: 'focus-layout',
    applyOntologyLayoutPositions: 'position-apply',
    'renderer.refresh': 'sigma-refresh', 'renderer?.refresh': 'sigma-refresh',
  };
  calls.forEach(call => {
    const name = call.expression.getText(ast);
    if (phases[name]) phase(call, phases[name]);
    if (name === 'graph.updateEachNodeAttributes') {
      let parent = call.parent;
      while (parent && !ts.isFunctionDeclaration(parent)) parent = parent.parent;
      if (!['recordOntologyLayoutBaseline', 'applyOntologyLayoutPositions'].includes(parent?.name?.text)) phase(call, 'position-apply');
    }
    if (name === 'animateNodes') phase(call, 'focus-animation', call.arguments[1].getText(ast));
  });
  const populationStart = kind === 'knowledge'
    ? named('positionedNodes')[0].end
    : matching('assignInitialSigmaPositions')[0].parent.end;
  const populationEnd = kind === 'knowledge'
    ? named('requestLayout')[0].getStart(ast)
    : matching('applySigmaParallelEdgeCurvature')[0].getStart(ast);
  assert(populationStart < populationEnd);
  edits.push([populationStart, populationStart, `\nconst __populationStarted = performance.now();`]);
  edits.push([populationEnd, populationEnd, `globalThis.__forceAtlasProfile.recordPhase(${JSON.stringify(kind)}, 'graph-population', performance.now() - __populationStarted);\n`]);
  if (kind === 'ontology') {
    const start = named('rawConnectionCounts')[0].getStart(ast), end = named('selectedNodeId')[0].end;
    edits.push([start, start, 'const __projectionStarted = performance.now();\n']);
    edits.push([end, end, `\nglobalThis.__forceAtlasProfile.recordPhase('ontology', 'projection', performance.now() - __projectionStarted);`]);
  }
  for (const [start, end, replacement] of edits.sort((a, b) => b[0] - a[0])) {
    source = source.slice(0, start) + replacement + source.slice(end);
  }
  instrumented.add(kind);
  return source;
}

function installProfile({ sigmaProfile, traceProfile }) {
  const records = [], active = new Map();
  let phases = {};
  let generation = 0, constructing, observationStart = 0;
  let slowCalls = [];
  const sigmaKinds = new WeakMap(), glKinds = new WeakMap(), sigmaClasses = new WeakSet(), extensions = new WeakSet();
  let devices = {}, renderers = {};
  const longTasks = [];
  new PerformanceObserver(list => list.getEntries().forEach(entry => {
    if (longTasks.length < 200) longTasks.push({ start: entry.startTime, duration: entry.duration });
  })).observe({ type: 'longtask', buffered: true });
  function contextKind(gl) {
    let owner = glKinds.get(gl);
    if (!owner && constructing) {
      owner = constructing;
      glKinds.set(gl, owner);
      if (!devices[owner.kind]) {
        const ext = gl.getExtension('WEBGL_debug_renderer_info');
        devices[owner.kind] = {
          version: gl.getParameter(gl.VERSION), renderer: gl.getParameter(gl.RENDERER),
          unmaskedRenderer: ext ? gl.getParameter(ext.UNMASKED_RENDERER_WEBGL) : null,
          unmaskedVendor: ext ? gl.getParameter(ext.UNMASKED_VENDOR_WEBGL) : null,
        };
      }
    }
    return owner?.generation === generation ? owner.kind : null;
  }
  function wrapGl(prototype, name) {
    const original = prototype[name];
    if (typeof original !== 'function') return;
    prototype[name] = function (...args) {
      const kind = contextKind(this);
      const invoke = () => original.apply(this, args);
      const result = kind ? globalThis.__forceAtlasProfile.measure(kind, `webgl-${name}`, invoke) : invoke();
      if (name === 'getExtension' && result && args[0] === 'ANGLE_instanced_arrays' && !extensions.has(result)) {
        extensions.add(result);
        const draw = result.drawArraysInstancedANGLE;
        const gl = this;
        result.drawArraysInstancedANGLE = function (...drawArgs) {
          const owner = contextKind(gl);
          const run = () => draw.apply(this, drawArgs);
          return owner ? globalThis.__forceAtlasProfile.measure(owner, 'webgl-drawArraysInstancedANGLE', run) : run();
        };
      }
      return result;
    };
  }
  if (sigmaProfile) {
    for (const prototype of [WebGLRenderingContext.prototype, WebGL2RenderingContext.prototype]) {
      for (const name of ['getExtension', 'compileShader', 'linkProgram', 'bufferData', 'bufferSubData', 'texImage2D', 'drawArrays', 'drawElements', 'drawArraysInstanced', 'drawElementsInstanced', 'readPixels']) wrapGl(prototype, name);
    }
  }
  let lastTimer = performance.now(), lastFrame = performance.now();
  setInterval(() => {
    const now = performance.now();
    for (const record of records) {
      if (record.end >= lastTimer && record.start <= now) {
        if (now - lastTimer > record.timerGapMs) { record.timerGapMs = now - lastTimer; record.timerGapInterval = { start: lastTimer, end: now }; }
        if (now <= record.end) record.timerTicks++;
      }
    }
    lastTimer = now;
  }, 10);
  const frame = now => {
    for (const record of records) {
      if (record.end >= lastFrame && record.start <= now) {
        if (now - lastFrame > record.frameGapMs) { record.frameGapMs = now - lastFrame; record.frameGapInterval = { start: lastFrame, end: now }; }
        if (now <= record.end) record.frames++;
      }
    }
    lastFrame = now;
    requestAnimationFrame(frame);
  };
  requestAnimationFrame(frame);
  globalThis.__forceAtlasProfile = {
    records,
    resetPhases() { phases = {}; devices = {}; renderers = {}; slowCalls = []; observationStart = performance.now(); generation++; },
    phases(kind) { return phases[kind] || {}; },
    details(kind) {
      let cacheChecksum = 2166136261, cachedNodes = 0, cacheNonFinite = 0;
      const renderer = renderers[kind];
      renderer.getGraph().forEachNode(id => {
        const attrs = renderer.getNodeDisplayData(id);
        if (!attrs) throw new Error(`Missing Sigma display cache: ${id}`);
        cachedNodes++;
        if (!Number.isFinite(attrs.x) || !Number.isFinite(attrs.y)) cacheNonFinite++;
        for (const char of JSON.stringify([id, attrs.x, attrs.y])) cacheChecksum = Math.imul(cacheChecksum ^ char.charCodeAt(0), 16777619);
      });
      return { device: devices[kind], cacheChecksum: cacheChecksum >>> 0, cachedNodes, cacheNonFinite,
        observationStart, observationEnd: performance.now(),
        longTasks: longTasks.filter(entry => entry.start + entry.duration >= observationStart),
        slowCalls: slowCalls.filter(entry => entry.kind === kind) };
    },
    constructSigma(kind, Constructor, create) {
      if (!sigmaClasses.has(Constructor)) {
        sigmaClasses.add(Constructor);
        for (const name of ['process', 'render', 'refresh', 'resize', 'clear', 'renderLabels', 'renderEdgeLabels', 'renderHighlightedNodes']) {
          const original = Constructor.prototype[name];
          if (typeof original !== 'function') throw new Error(`Missing Sigma method: ${name}`);
          Constructor.prototype[name] = function (...args) {
            const owner = sigmaKinds.get(this) || constructing;
            const run = () => original.apply(this, args);
            const phaseName = `sigma-internal-${name}${name === 'refresh' ? args[0]?.partialGraph ? '-partial' : '-full' : ''}`;
            return owner?.generation === generation ? globalThis.__forceAtlasProfile.measure(owner.kind, phaseName, run) : run();
          };
        }
      }
      const previous = constructing;
      constructing = { kind, generation };
      try {
        const renderer = create();
        sigmaKinds.set(renderer, constructing);
        renderers[kind] = renderer;
        return renderer;
      } finally { constructing = previous; }
    },
    recordPhase(kind, name, ms, targets) {
      const group = phases[kind] ||= {};
      const phase = group[name] ||= { calls: 0, totalMs: 0, maxMs: 0 };
      phase.calls++;
      phase.totalMs += ms;
      phase.maxMs = Math.max(phase.maxMs, ms);
      if (targets !== undefined) phase.animationTargets = (phase.animationTargets || 0) + targets;
    },
    measure(kind, name, fn, targets) {
      const start = performance.now();
      try { return fn(); }
      finally {
        const end = performance.now();
        this.recordPhase(kind, name, end - start, targets);
        if (sigmaProfile && end - start >= 2 && slowCalls.length < 250) {
          slowCalls.push({ kind, name, start, end, duration: end - start });
          if (traceProfile) performance.measure(`reqvire:${kind}:${name}`, { start, end });
        }
      }
    },
    begin(kind) {
      const previous = active.get(kind);
      if (previous) { previous.cancelled = true; previous.end = performance.now(); }
      const record = { kind, start: performance.now(), end: Infinity, timerGapMs: 0, frameGapMs: 0, timerTicks: 0, frames: 0 };
      active.set(kind, record);
      records.push(record);
      if (traceProfile) performance.mark(`reqvire:${kind}:layout-start:${records.length - 1}`);
    },
    run(kind, owner, graph, renderedGraph, apply, fail) {
      const record = active.get(kind);
      record.prepareMs = performance.now() - record.start;
      record.nodes = graph.order;
      record.edges = graph.size;
      record.submittedAt = performance.now();
      owner.run(graph, positions => {
        record.layoutWallMs = performance.now() - record.submittedAt;
        // Reconstruct worker coordinates on the private probe graph for parity.
        positions.forEach(({ id, x, y }) => graph.mergeNodeAttributes(id, { x, y }));
        const started = performance.now();
        apply(positions);
        record.applyMs = performance.now() - started;
        globalThis.__forceAtlasProfile.finish(kind, graph, renderedGraph);
      }, error => {
        record.error = String(error);
        record.end = performance.now();
        fail(error);
      });
    },
    finish(kind, graph, renderedGraph) {
      const record = active.get(kind);
      record.end = performance.now();
      record.totalMs = record.end - record.start;
      if (traceProfile) performance.mark(`reqvire:${kind}:layout-end:${records.indexOf(record)}`);
      // Validate outside measured phase boundaries; retain a deterministic checksum
      // to detect changed results across cold and warm runs without dumping a graph.
      let checksum = 2166136261, nonFinite = 0;
      graph.forEachNode((id, attrs) => {
        if (!Number.isFinite(attrs.x) || !Number.isFinite(attrs.y)) nonFinite++;
        for (const char of `${id}:${attrs.x}:${attrs.y};`) checksum = Math.imul(checksum ^ char.charCodeAt(0), 16777619);
      });
      record.coordinateChecksum = checksum >>> 0;
      record.nonFinite = nonFinite;
      record.renderedNodes = renderedGraph.order;
      record.renderedVisibleNodes = renderedGraph.filterNodes((_id, attrs) => !attrs.hidden).length;
      record.renderedEligibleEdges = renderedGraph.filterEdges((_id, attrs) => !attrs.hidden).length;
      let visualChecksum = 2166136261;
      const visual = values => {
        for (const char of JSON.stringify(values)) visualChecksum = Math.imul(visualChecksum ^ char.charCodeAt(0), 16777619);
      };
      renderedGraph.forEachNode((id, attrs) => visual([id, attrs.type, attrs.color, attrs.size, attrs.label, attrs.fullLabel, attrs.hidden, attrs.image, attrs.mutedImage]));
      renderedGraph.forEachEdge((id, attrs, source, target) => visual([id, source, target, attrs.type, attrs.color, attrs.size, attrs.label, attrs.hidden, attrs.curvature]));
      record.visualChecksum = visualChecksum >>> 0;
      let positionChecksum = 2166136261;
      renderedGraph.forEachNode((id, attrs) => {
        for (const char of JSON.stringify([id, attrs.x, attrs.y, attrs.baseX, attrs.baseY])) positionChecksum = Math.imul(positionChecksum ^ char.charCodeAt(0), 16777619);
      });
      record.renderedPositionChecksum = positionChecksum >>> 0;
      const surface = kind === 'knowledge' ? document.querySelector('[data-testid="kg-sigma-canvas"]') : document.querySelector('#ontology-graph-container');
      record.canvasReady = Boolean(surface?.querySelector('canvas'));
      active.delete(kind);
    },
  };
}

await build({
  root,
  configFile: path.join(root, 'vite.config.ts'),
  logLevel: 'warn',
  plugins: [{
    name: 'forceatlas-diagnostic', enforce: 'pre',
    transform(source, id) {
      const kind = Object.keys(files).find(key => id === files[key]);
      if (kind) return { code: instrument(sources[kind], id, kind), map: null };
    },
    transformIndexHtml: { order: 'pre', handler: () => [{
      tag: 'script', children: `(${installProfile.toString()})(${JSON.stringify({ sigmaProfile, traceProfile })});`, injectTo: 'head-prepend',
    }] },
  }],
  build: { outDir: site, emptyOutDir: true },
});
assert.deepEqual([...instrumented].sort(), ['knowledge', 'ontology']);
const nodeCount = n => ({ elements: n, relations: n - 1 });
function synthetic(count, density = 1, hidden = 0) {
  const kgNodes = Array.from({ length: count + hidden }, (_, i) => ({
    id: `urn:profile:${i}`, identifier: `urn:profile:${i}`, label: `Node ${i}`,
    element_type: i < count ? 'requirement' : 'resource', node_type: i < count ? 'requirement' : 'resource', file_path: 'fixture.md',
  }));
  const edges = [];
  for (let i = 1; i < count; i++) edges.push({ source: kgNodes[Math.floor((i - 1) / 2)].id, target: kgNodes[i].id, label: 'derive', kind: 'relation' });
  for (let distance = 1; distance < density; distance++) {
    for (let i = 0; i < count; i++) edges.push({ source: kgNodes[i].id, target: kgNodes[(i + distance * 17) % count].id, label: 'specify', kind: 'relation' });
  }
  for (let i = count; i < count + hidden; i++) edges.push({ source: kgNodes[0].id, target: kgNodes[i].id, label: 'derive', kind: 'relation' });
  const nodes = kgNodes.map((node, i) => ({
    id: node.id, label: node.label, full_uri: node.id, semantic_type: 'class',
    layer: i < count ? 'authored' : 'external-source', source_kind: i < count ? 'ontology' : 'external-ontology',
    comment: '', rdf_types: [], type_evidence: [], constraints: [], badges: [], equivalence_group: '',
    inverse_properties: [], property_chains: [], domain: [], range: [], literal_values: [], slot_facets: [], constructs: [],
    sources: [{ source: 'fixture', source_name: 'fixture', file_path: 'fixture.md', line_number: 1, kind: 'ontology', link: '' }],
  }));
  const nodeById = new Map(nodes.map(node => [node.id, node]));
  return {
    ...seed,
    knowledge_graph: { nodes: kgNodes, edges, summary: nodeCount(count + hidden), submodels: [] },
    ontology: { ...seed.ontology, graph_data: { nodes, edges: edges.map(edge => ({
      source: edge.source, target: edge.target, label: 'subclass',
      layer: nodeById.get(edge.target).layer, source_kind: 'ontology',
    })) } },
  };
}
let fixtures = [
  { name: 'current-model', store: seed },
  ...sizes.map(count => ({ name: `tree-${count}`, store: synthetic(count), nodes: count, edges: count - 1 })),
  { name: 'dense-500', store: synthetic(500, 4), nodes: 500, edges: 1999 },
  // Hidden ontology layers must not contribute nodes or edges to worker input.
  { name: 'ontology-hidden-500-of-1000', store: synthetic(500, 1, 500), nodes: 500, ontologyNodes: 500,
    knowledgeNodes: 1000, knowledgeEdges: 999, edges: 499 },
];
if (process.env.FORCEATLAS_PROFILE_FIXTURES) {
  const requested = process.env.FORCEATLAS_PROFILE_FIXTURES.split(',');
  assert(requested.every(name => fixtures.some(fixture => fixture.name === name)), 'Unknown fixture');
  fixtures = fixtures.filter(fixture => requested.includes(fixture.name));
}
for (const fixture of fixtures) {
  if (fixture.edges) fixture.ontologyEdges = new Set(fixture.store.ontology.graph_data.edges
    .filter(edge => edge.layer === 'authored')
    .map(edge => `${edge.source}\u001f${edge.target}\u001f${edge.label}`)).size;
}
let currentStore = seed;
const server = createServer(async (req, res) => {
  try {
    const pathname = new URL(req.url, 'http://localhost').pathname;
    if (pathname === '/assets/project-store.js') {
      res.setHeader('Content-Type', 'text/javascript');
      res.setHeader('Cache-Control', 'no-store');
      res.end(`window.reqvireProjectStore = ${JSON.stringify(currentStore)};`);
      return;
    }
    const file = path.resolve(site, `.${pathname === '/' ? '/index.html' : pathname}`);
    if (!file.startsWith(site + path.sep)) { res.writeHead(403).end(); return; }
    const data = await readFile(file);
    res.setHeader('Content-Type', file.endsWith('.js') ? 'text/javascript' : file.endsWith('.css') ? 'text/css' : file.endsWith('.html') ? 'text/html' : 'application/octet-stream');
    res.end(data);
  } catch { res.writeHead(404).end(); }
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const baseUrl = `http://127.0.0.1:${server.address().port}`;
const result = {
  environment: { date: new Date().toISOString(), node: process.version, platform: process.platform,
    cpu: os.cpus()[0]?.model, cpus: os.cpus().length, memoryBytes: os.totalmem(),
    sourceHashes: Object.fromEntries(Object.entries(sources).map(([kind, source]) => [kind, createHash('sha256').update(source).digest('hex')])),
    instrumentationHash: createHash('sha256').update(instrumentationSource).digest('hex'),
    browserDriverHash: createHash('sha256').update(browserDriverSource).digest('hex'),
    seedHash: createHash('sha256').update(seedScript).digest('hex'),
    gpuBackend, sigmaProfile, traceProfile,
    sigmaVersion: JSON.parse(await readFile(path.join(root, 'node_modules/sigma/package.json'))).version,
    forceatlasVersion: JSON.parse(await readFile(path.join(root, 'node_modules/graphology-layout-forceatlas2/package.json'))).version },
  method: 'Actual worker-backed browser renderers. Synchronous phase calls have inclusive wall times (nested phases must not be summed). Phase counters cover startup through 350 ms after accepted layout, including focus animation. layoutWallMs includes worker startup, transport and execution. Timer/frame gaps include renderer construction. No timing thresholds.',
  samples: [],
};
try {
  await withBrowser(path.join(output, 'browser-profile'), async browser => {
    result.environment.browser = await browser.rpc('Browser.getVersion');
    async function stopTrace(file) {
      let timer;
      const completed = new Promise((resolve, reject) => {
        timer = setTimeout(() => reject(new Error('Timed out waiting for Chrome trace completion')), 15000);
        browser.on('Tracing.tracingComplete', resolve);
      });
      let stream;
      try { [, { stream }] = await Promise.all([browser.rpc('Tracing.end'), completed]); }
      finally { clearTimeout(timer); }
      const chunks = [];
      try {
        while (true) {
          const chunk = await browser.rpc('IO.read', { handle: stream, size: 1024 * 1024 });
          chunks.push(chunk.base64Encoded ? Buffer.from(chunk.data, 'base64').toString() : chunk.data);
          if (chunk.eof) break;
        }
        await writeFile(path.join(output, file), chunks.join(''));
      } finally { await browser.rpc('IO.close', { handle: stream }); }
    }
    for (const fixture of fixtures) {
      currentStore = fixture.store;
      for (const kind of ['knowledge', 'ontology']) {
        const route = kind === 'knowledge' ? '#/model' : '#/ontologies';
        if (traceProfile) await browser.rpc('Tracing.start', {
          categories: 'devtools.timeline,v8,blink.user_timing,gpu,cc,toplevel,disabled-by-default-gpu.service',
          transferMode: 'ReturnAsStream',
        });
        await browser.navigate(`${baseUrl}/?fixture=${fixture.name}&kind=${kind}${route}`);
        for (let trial = 0; trial < trials; trial++) {
          if (trial) {
            await browser.evaluate(() => globalThis.__forceAtlasProfile.resetPhases());
            await browser.evaluate(() => { location.hash = '#/files'; });
            await waitFor(() => browser.evaluate(() => !document.querySelector('canvas')));
            await browser.evaluate(route => { location.hash = route; }, route);
          }
          if (kind === 'knowledge') {
            await waitFor(() => browser.evaluate(() => Boolean(document.querySelector('[aria-label="Model layout"]'))));
            await browser.evaluate(() => {
              const button = [...document.querySelectorAll('[aria-label="Model layout"] button')].find(button => button.textContent.trim() === 'Graph');
              if (!button) throw new Error('Missing Graph mode');
              button.click();
            });
          }
          await waitFor(() => browser.evaluate((kind, trial) =>
            globalThis.__forceAtlasProfile.records.filter(record => record.kind === kind && !record.cancelled && Number.isFinite(record.end)).length > trial && Boolean(document.querySelector('canvas')), kind, trial), 30000);
          await browser.evaluate(() => new Promise(resolve => setTimeout(resolve, 350)));
          const record = await browser.evaluate((kind, trial) => globalThis.__forceAtlasProfile.records.filter(record => record.kind === kind && !record.cancelled && Number.isFinite(record.end))[trial], kind, trial);
          assert.equal(record.nonFinite, 0, `${fixture.name}: finite coordinates`);
          assert(!record.error);
          assert(record.canvasReady);
          assert.equal(record.nodes, record.renderedVisibleNodes, `${fixture.name}/${kind}: visible renderer nodes match layout input`);
          assert.equal(record.edges, record.renderedEligibleEdges, `${fixture.name}/${kind}: eligible renderer edges match layout input`);
          if (record.nodes >= 1000) { assert(record.timerTicks > 0); assert(record.frames > 0); }
          if (fixture.nodes) {
            assert.equal(record.nodes, kind === 'ontology' ? fixture.ontologyNodes ?? fixture.nodes : fixture.knowledgeNodes ?? fixture.nodes, `${fixture.name}/${kind}: layout nodes`);
            assert.equal(record.edges, kind === 'ontology' ? fixture.ontologyEdges : fixture.knowledgeEdges ?? fixture.edges, `${fixture.name}/${kind}: layout edges`);
          }
          const sample = { fixture: fixture.name, trial, temperature: trial ? 'warm-route-reentry' : 'cold-document',
            expectedVisibleNodes: kind === 'ontology' ? fixture.nodes : fixture.knowledgeNodes ?? fixture.nodes, ...record };
          sample.phases = await browser.evaluate(kind => globalThis.__forceAtlasProfile.phases(kind), kind);
          if (sigmaProfile) {
            sample.sigma = await browser.evaluate(kind => globalThis.__forceAtlasProfile.details(kind), kind);
            assert(sample.sigma.device?.unmaskedRenderer, 'Expected actual WebGL renderer identity');
            sample.sigma.hardware = !/swiftshader|llvmpipe|softpipe|software/i.test(sample.sigma.device.unmaskedRenderer);
            if (gpuBackend === 'native') assert(sample.sigma.hardware, 'Native GPU request used a software renderer');
            assert(sample.phases['sigma-internal-process'].calls > 0);
            assert(sample.phases['sigma-internal-render'].calls > 0);
            assert.equal(sample.sigma.cachedNodes, sample.renderedNodes);
            assert.equal(sample.sigma.cacheNonFinite, 0);
          }
          assert.equal(sample.phases['sigma-startup'].calls, 1);
          assert(sample.phases['position-apply'].calls > 0);
          assert(sample.phases['graph-population'].calls > 0);
          result.samples.push(sample);
          await writeFile(path.join(output, 'results.json'), JSON.stringify(result, null, 2));
          console.log(`${fixture.name}/${kind}/${trial}: ${record.nodes} nodes, ${record.edges} edges; worker wall ${record.layoutWallMs.toFixed(1)} ms; ${record.timerTicks} timer ticks, ${record.frames} frames; timer gap ${record.timerGapMs.toFixed(1)} ms; frame gap ${record.frameGapMs.toFixed(1)} ms`);
        }
        const checksums = result.samples.filter(sample => sample.fixture === fixture.name && sample.kind === kind).map(sample => sample.coordinateChecksum);
        assert.equal(new Set(checksums).size, 1, `${fixture.name}/${kind}: cold/warm coordinate parity`);
        assert.equal(new Set(result.samples.filter(sample => sample.fixture === fixture.name && sample.kind === kind).map(sample => sample.visualChecksum)).size, 1, `${fixture.name}/${kind}: cold/warm visual attribute parity`);
        if (traceProfile) await stopTrace(`${fixture.name}-${kind}.trace.json`);
      }
    }
  }, { gpuBackend });
} finally { await new Promise(resolve => server.close(resolve)); }
result.complete = true;
await writeFile(path.join(output, 'results.json'), JSON.stringify(result, null, 2));
console.log(`Profile retained: ${output}/results.json`);
