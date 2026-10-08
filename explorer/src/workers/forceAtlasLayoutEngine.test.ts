import Graph from "graphology";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { forceAtlasLayoutEngine, forceAtlasInput, type ForceAtlasTask } from "./forceAtlasLayoutEngine";
import { ForceAtlasLayoutOwner } from "./forceAtlasLayoutOwner";
import type { ForceAtlasPositions } from "../lib/forceAtlasLayout";

class WorkerDouble {
  static instances: WorkerDouble[] = [];
  onmessage: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: ErrorEvent) => void) | null = null;
  onmessageerror: (() => void) | null = null;
  terminate = vi.fn();
  postMessage = vi.fn();
  constructor(readonly url: URL, readonly options: WorkerOptions) { WorkerDouble.instances.push(this); }
}
const input = { nodes: [{ id: "same", x: 1, y: 2, size: 8 }], edges: [] };
const positions = [{ id: "same", x: 4, y: 5 }];
beforeEach(() => { WorkerDouble.instances = []; vi.stubGlobal("Worker", WorkerDouble); });
afterEach(() => vi.unstubAllGlobals());

describe("ForceAtlas worker ownership", () => {
  it("uses a native local worker and releases it after a complete validated result", async () => {
    const task = forceAtlasLayoutEngine(input);
    const worker = WorkerDouble.instances[0];
    expect(worker.url.pathname).toMatch(/\/forceAtlasLayout.worker.ts$/);
    expect(worker.options.type).toBe("module");
    expect(worker.postMessage).toHaveBeenCalledWith(input);
    const late = worker.onmessage!;
    late({ data: { ok: true, positions } } as MessageEvent);
    await expect(task.result).resolves.toEqual(positions);
    task.cancel();
    late({ data: { ok: false, error: "late" } } as MessageEvent);
    expect(worker.terminate).toHaveBeenCalledOnce();
    expect(worker.onmessage).toBeNull();
  });

  it("cancels one active worker without cancelling another mounted graph", async () => {
    const first = forceAtlasLayoutEngine(input), second = forceAtlasLayoutEngine(input);
    const cancelled = expect(first.result).rejects.toThrow("cancelled");
    const [a, b] = WorkerDouble.instances;
    const late = a.onmessage!;
    first.cancel(); first.cancel();
    late({ data: { ok: true, positions } } as MessageEvent);
    await cancelled;
    expect(a.terminate).toHaveBeenCalledOnce();
    expect(b.terminate).not.toHaveBeenCalled();
    b.onmessage!({ data: { ok: true, positions } } as MessageEvent);
    await expect(second.result).resolves.toEqual(positions);
  });

  it.each([null, [], [{ id: "other", x: 4, y: 5 }], [{ id: "same", x: NaN, y: 5 }],
    [positions[0], positions[0]]])("rejects invalid or incomplete positions: %j", async bad => {
    const task = forceAtlasLayoutEngine(input);
    const rejected = expect(task.result).rejects.toThrow("Invalid ForceAtlas response");
    WorkerDouble.instances[0].onmessage!({ data: { ok: true, positions: bad } } as MessageEvent);
    await rejected;
    expect(WorkerDouble.instances[0].terminate).toHaveBeenCalledOnce();
  });

  it.each(["startup", "post", "load", "decode", "execution"])("settles %s failure and permits retry", async mode => {
    if (mode === "startup") vi.stubGlobal("Worker", class { constructor() { throw new Error("startup"); } });
    if (mode === "post") vi.stubGlobal("Worker", class extends WorkerDouble { postMessage = vi.fn(() => { throw new Error("post"); }); });
    const task = forceAtlasLayoutEngine(input);
    const rejected = expect(task.result).rejects.toBeInstanceOf(Error);
    const worker = WorkerDouble.instances[0];
    if (mode === "load") worker.onerror!(new ErrorEvent("error", { message: "load" }));
    if (mode === "decode") worker.onmessageerror!();
    if (mode === "execution") worker.onmessage!({ data: { ok: false, error: "execution" } } as MessageEvent);
    await rejected;
    if (worker) expect(worker.terminate).toHaveBeenCalledOnce();
    vi.stubGlobal("Worker", WorkerDouble);
    const retry = forceAtlasLayoutEngine(input);
    WorkerDouble.instances.at(-1)!.onmessage!({ data: { ok: true, positions } } as MessageEvent);
    await expect(retry.result).resolves.toEqual(positions);
  });

  it("snapshots positions and retains parallel edges and self-loops without mutable graph references", () => {
    const graph = new Graph({ multi: true, allowSelfLoops: true, type: "directed" });
    graph.addNode("same", { x: 1, y: 2, size: 8, unrelated: "not transferred" });
    graph.addDirectedEdgeWithKey("a", "same", "same"); graph.addDirectedEdgeWithKey("b", "same", "same");
    const snapshot = forceAtlasInput(graph);
    graph.setNodeAttribute("same", "x", 999);
    expect(snapshot.nodes).toEqual(input.nodes);
    expect(snapshot.edges).toEqual([{ id: "a", source: "same", target: "same" }, { id: "b", source: "same", target: "same" }]);
  });
});

it("bounds replacement work and rejects late success/failure across topology, reset and disposal", async () => {
  const tasks: Array<ForceAtlasTask & { cancel: ReturnType<typeof vi.fn>; resolve: (value: ForceAtlasPositions) => void; reject: (error: Error) => void }> = [];
  const engine = vi.fn(() => {
    let resolve!: (value: ForceAtlasPositions) => void, reject!: (error: Error) => void;
    const task = { result: new Promise<ForceAtlasPositions>((yes, no) => { resolve = yes; reject = no; }), cancel: vi.fn(), resolve, reject };
    tasks.push(task); return task;
  });
  const owner = new ForceAtlasLayoutOwner(engine), independent = new ForceAtlasLayoutOwner(engine);
  const graph = new Graph(); graph.addNode("same", { x: 1, y: 2, size: 8 });
  const apply = vi.fn(), fail = vi.fn(), otherApply = vi.fn();
  owner.run(graph, apply, fail); independent.run(graph, otherApply, fail);
  for (let i = 0; i < 20; i++) { graph.setNodeAttribute("same", "x", i); owner.run(graph, apply, fail); }
  expect(tasks[1].cancel).not.toHaveBeenCalled();
  expect(tasks.filter(task => task.cancel.mock.calls.length === 0)).toHaveLength(2);
  tasks[0].resolve(positions); tasks[2].reject(new Error("retired")); tasks[1].resolve(positions);
  await Promise.resolve(); await Promise.resolve();
  expect(apply).not.toHaveBeenCalled(); expect(fail).not.toHaveBeenCalled(); expect(otherApply).toHaveBeenCalledOnce();
  tasks.at(-1)!.resolve(positions); await Promise.resolve();
  expect(apply).toHaveBeenCalledOnce();
  owner.run(graph, apply, fail); owner.cancel(); tasks.at(-1)!.resolve(positions);
  await Promise.resolve(); expect(apply).toHaveBeenCalledOnce();
});
