// Exercise the emitted browser worker in real threads. Only Web Worker transport
// is adapted; output is compared with the pre-worker Graphology invocation.
import assert from 'node:assert/strict';
import { readdir, readFile } from 'node:fs/promises';
import { createRequire } from 'node:module';
import path from 'node:path';
import { Worker } from 'node:worker_threads';

const require = createRequire(new URL('../../explorer/package.json', import.meta.url));
const Graph = require('graphology'), forceAtlas2 = require('graphology-layout-forceatlas2');
const assets = path.join(path.resolve(process.argv[2]), 'assets');
const files = await readdir(assets);
const names = files.filter(name => /^forceAtlasLayout\.worker-.*\.js$/.test(name));
assert.equal(names.length, 1, 'one local ForceAtlas worker must be packaged');
const source = await readFile(path.join(assets, names[0]), 'utf8');
assert((await Promise.all(files.filter(name => name.endsWith('.js') && name !== names[0]).map(name => readFile(path.join(assets, name), 'utf8'))))
  .some(script => script.includes(names[0])), 'Explorer must reference its packaged worker');

function start(input) {
  const worker = new Worker(`
    const { parentPort, workerData } = require('node:worker_threads');
    globalThis.self = globalThis;
    globalThis.postMessage = message => parentPort.postMessage({ type: 'result', message });
    require('node:vm').runInThisContext(workerData);
    parentPort.on('message', input => {
      parentPort.postMessage({ type: 'started' });
      const started = performance.now();
      self.onmessage({ data: input });
      parentPort.postMessage({ type: 'duration', ms: performance.now() - started });
    });
  `, { eval: true, workerData: source });
  let ticks = 0, previous = 0, maxGap = 0, startTime = 0, heartbeat, deadline;
  const result = new Promise((resolve, reject) => {
    deadline = setTimeout(() => reject(new Error('ForceAtlas worker timeout')), 15000);
    worker.on('error', reject);
    worker.on('exit', code => reject(new Error(`ForceAtlas worker exited before its result: ${code}`)));
    worker.on('message', event => {
      if (event.type === 'started') {
        startTime = previous = performance.now();
        heartbeat = setInterval(() => { const now = performance.now(); maxGap = Math.max(maxGap, now - previous); previous = now; ticks++; }, 10);
      }
      if (event.type === 'result') resolve({ ...event.message, ticks, maxGap, wallMs: performance.now() - startTime });
    });
  });
  worker.postMessage(input);
  return { worker, result, async close() { clearTimeout(deadline); clearInterval(heartbeat); await worker.terminate(); } };
}
async function layout(input) { const task = start(input); try { return await task.result; } finally { await task.close(); } }
function fixture(count) {
  const nodes = Array.from({ length: count }, (_, i) => ({ id: `n${i}`, x: Math.cos(i) * (3 + i % 11), y: Math.sin(i) * (3 + i % 11), size: 8 }));
  const pairs = count === 5 ? [[0, 1], [0, 2], [1, 3], [2, 3], [3, 4], [3, 4], [4, 4]]
    : nodes.slice(1).map((_, i) => [Math.floor(i / 2), i + 1]);
  return { nodes, edges: pairs.map(([a, b], i) => ({ id: `e${i}`, source: `n${a}`, target: `n${b}` })) };
}
function baseline(input) {
  const graph = new Graph({ type: 'directed', multi: true, allowSelfLoops: true });
  input.nodes.forEach(({ id, ...attrs }) => graph.addNode(id, attrs));
  input.edges.forEach(({ id, source, target }) => graph.addDirectedEdgeWithKey(id, source, target));
  const clamp = (value, min, max) => Math.max(min, Math.min(max, value));
  const nodes = Math.max(1, graph.order), density = graph.size / nodes;
  // Original renderer profile with every node size fixed to 8 for this fixture.
  forceAtlas2.assign(graph, { iterations: nodes > 650 ? 170 : nodes > 350 ? 180 : 200, settings: {
    ...forceAtlas2.inferSettings(graph), adjustSizes: true, barnesHutOptimize: true,
    gravity: clamp(1.45 + Math.log10(Math.max(10, nodes)) * .48 + Math.min(density, 8) * .04, 1.5, 3.2),
    scalingRatio: clamp(5 + Math.sqrt(nodes) * .14 + .2 * 1.5 - Math.min(density, 8) * .35, 5, 13),
    slowDown: nodes > 650 ? 2.3 : 2,
  } });
  return graph.mapNodes((id, { x, y }) => ({ id, x, y }));
}
for (const count of [0, 1, 5]) {
  const input = fixture(count), result = await layout(input);
  assert.equal(result.ok, true);
  assert.deepEqual(result.positions, baseline(input));
  assert.deepEqual((await layout(input)).positions, result.positions);
}
console.log('PASS forceatlas-worker-topology-parity');
console.log('PASS forceatlas-worker-empty-isolated');
const invalid = fixture(5); invalid.edges[0].target = 'missing';
assert.equal((await layout(invalid)).ok, false);
console.log('PASS forceatlas-worker-invalid-input');
const large = fixture(1000), measured = await layout(large);
assert.equal(measured.ok, true);
assert(measured.ticks > 0, 'main thread must continue during ForceAtlas computation');
assert.deepEqual(measured.positions, baseline(large));
console.error(`ForceAtlas worker: 1000 nodes, ${measured.wallMs.toFixed(1)} ms, ${measured.ticks} heartbeats, ${measured.maxGap.toFixed(1)} ms maximum gap`);
console.log('PASS forceatlas-worker-responsive-main-thread');
const cancelled = start(fixture(2000)), independent = start(fixture(5));
const cancellation = assert.rejects(cancelled.result, /exited before/);
await cancelled.close(); await cancellation;
try { assert.equal((await independent.result).ok, true); } finally { await independent.close(); }
console.log('PASS forceatlas-worker-independent-cancellation');
