// Execute the exact packaged browser worker in a thread, adapting only the
// Web Worker message API. No ports, server, engine substitute, or npm install.
import assert from "node:assert/strict";
import { readdir, readFile } from "node:fs/promises";
import path from "node:path";
import { Worker } from "node:worker_threads";

const bundle = path.resolve(process.argv[2]);
const assets = path.join(bundle, "assets");
const names = await readdir(assets);
const workers = names.filter(name => /^flowLayout\.worker-.*\.js$/.test(name));
assert.equal(workers.length, 1, "exactly one local Flow worker must be packaged");
const source = await readFile(path.join(assets, workers[0]), "utf8");
const index = await readFile(path.join(bundle, "index.html"), "utf8");
const entries = [...index.matchAll(/<script\b(?=[^>]*\btype=["']module["'])[^>]*\bsrc=["']([^"']+)["']/g)].map(match => match[1]);
const scripts = await Promise.all(entries.map(entry => readFile(path.join(bundle, entry), "utf8")));
assert(scripts.some(script => script.includes(workers[0])), "application must reference its packaged worker");

async function layout(input, measure = false) {
  const worker = new Worker(`
    const { parentPort, workerData } = require("node:worker_threads");
    globalThis.self = globalThis;
    globalThis.postMessage = message => parentPort.postMessage({ type: "result", message });
    require("node:vm").runInThisContext(workerData);
    parentPort.on("message", input => {
      parentPort.postMessage({ type: "started" });
      self.onmessage({ data: { id: 0, cmd: "register", algorithms: ["layered"] } });
      self.onmessage({ data: { id: 1, cmd: "layout", graph: input, layoutOptions: {}, options: {} } });
    });
  `, { eval: true, workerData: source });
  let ticks = 0;
  let started = 0;
  let previous = 0;
  let maxGap = 0;
  let heartbeat;
  let timeout;
  try {
    return await new Promise((resolve, reject) => {
      timeout = setTimeout(() => reject(new Error("Packaged Flow worker did not respond")), 15000);
      worker.on("error", reject);
      worker.on("exit", code => reject(new Error(`Worker exited before response: ${code}`)));
      worker.on("message", event => {
        if (event.type === "started") {
          started = previous = performance.now();
          heartbeat = setInterval(() => {
            const now = performance.now();
            maxGap = Math.max(maxGap, now - previous);
            previous = now;
            ticks++;
          }, 10);
          return;
        }
        if (event.message.id !== 1) return;
        if (measure) {
          assert(ticks > 0, "main-thread heartbeat must continue during worker computation");
          console.error(`Flow worker: ${input.children.length} nodes, ${(performance.now() - started).toFixed(1)} ms layout, ${ticks} heartbeats, ${maxGap.toFixed(1)} ms maximum heartbeat gap`);
        }
        resolve(event.message.error ? { ok: false, error: event.message.error } : { ok: true, graph: event.message.data });
      });
      worker.postMessage(input);
    });
  } finally {
    clearTimeout(timeout);
    clearInterval(heartbeat);
    await worker.terminate();
  }
}

function input(direction, count = 5) {
  const children = Array.from({ length: count }, (_, i) => ({ id: `n${i}`, width: 280, height: 164 }));
  const pairs = count === 5 ? [[0, 1], [0, 2], [1, 3], [2, 3], [3, 4], [3, 4], [4, 4]]
    : children.slice(1).map((_, i) => [Math.floor(i / 2), i + 1]);
  return { id: "trace-layout", layoutOptions: {
    "elk.algorithm": "layered", "elk.direction": direction, "elk.edgeRouting": "ORTHOGONAL", "elk.randomSeed": "1",
  }, children, edges: pairs.map(([from, to], i) => ({ id: `e${i}`, sources: [`n${from}`], targets: [`n${to}`],
    labels: [{ text: "derive", width: 100, height: 28 }],
  })) };
}

for (const direction of ["RIGHT", "DOWN"]) {
  const request = input(direction);
  const result = await layout(request);
  assert.equal(result.ok, true);
  assert.deepEqual(result.graph.children.map(node => node.id), request.children.map(node => node.id));
  assert.deepEqual(result.graph.edges.map(edge => edge.id), request.edges.map(edge => edge.id));
  assert(result.graph.children.every(node => Number.isFinite(node.x) && Number.isFinite(node.y)));
  assert(result.graph.edges.every(edge => edge.sections.length > 0 && Number.isFinite(edge.labels[0].x)));
  const [root, child] = result.graph.children;
  assert(direction === "DOWN" ? child.y > root.y : child.x > root.x);
  assert.deepEqual(await layout(request), result, "packaged worker output must be deterministic");
  console.log(`PASS flow-worker-${direction.toLowerCase()}`);
}
for (const count of [0, 1]) {
  const response = await layout(input("DOWN", count));
  assert.equal(response.ok, true);
  assert.equal(response.graph.children.length, count);
}
console.log("PASS flow-worker-empty-isolated");
const invalid = input("DOWN");
invalid.edges[0].targets = ["missing"];
assert.equal((await layout(invalid)).ok, false);
console.log("PASS flow-worker-layout-error");
assert.equal((await layout(input("DOWN", 1000), true)).ok, true);
console.log("PASS flow-worker-responsive-main-thread");
