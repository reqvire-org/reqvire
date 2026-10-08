import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { flowLayoutEngine } from "./flowLayoutEngine";

class WorkerDouble {
  static instances: WorkerDouble[] = [];
  onmessage: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: ErrorEvent) => void) | null = null;
  onmessageerror: (() => void) | null = null;
  terminate = vi.fn();
  postMessage = vi.fn();
  constructor(readonly url: URL, readonly options: WorkerOptions) { WorkerDouble.instances.push(this); }
}
const input = { id: "trace-layout", children: [], edges: [] };
beforeEach(() => { WorkerDouble.instances = []; vi.stubGlobal("Worker", WorkerDouble); });
afterEach(() => vi.unstubAllGlobals());

describe("Flow worker ownership", () => {
  it("starts only on request and releases workers after completion", async () => {
    expect(WorkerDouble.instances).toHaveLength(0);
    const task = flowLayoutEngine(input);
    const [worker] = WorkerDouble.instances;
    expect(worker.url.pathname).toMatch(/\/flowLayout.worker.ts$/);
    expect(worker.options.type).toBe("module");
    expect(worker.postMessage).toHaveBeenCalledWith(expect.objectContaining({ cmd: "layout", graph: input }));
    const reply = worker.onmessage!;
    const graph = { ...input, width: 48, height: 48 };
    reply({ data: { id: 1, data: graph } } as MessageEvent);
    expect(await task.result).toBe(graph);
    task.cancel();
    reply({ data: { id: 1, error: new Error("Late error") } } as MessageEvent);
    expect(worker.terminate).toHaveBeenCalledOnce();
    expect(worker.onmessage).toBeNull();
    expect(worker.onerror).toBeNull();
    expect(worker.onmessageerror).toBeNull();
  });

  it("terminates only the cancelled job and rejects it even before the worker responds", async () => {
    const first = flowLayoutEngine(input);
    const second = flowLayoutEngine(input);
    const [a, b] = WorkerDouble.instances;
    const late = a.onmessage!;
    const cancelled = expect(first.result).rejects.toThrow("cancelled");
    first.cancel();
    first.cancel();
    late({ data: { id: 1, data: input } } as MessageEvent);
    await cancelled;
    expect(a.terminate).toHaveBeenCalledOnce();
    expect(b.terminate).not.toHaveBeenCalled();
    b.onmessage!({ data: { id: 1, data: input } } as MessageEvent);
    await expect(second.result).resolves.toBe(input);
  });

  it.each(["execution", "loading", "decode", "invalid"])("settles %s failure, cleans up, and permits retry", async failure => {
    const task = flowLayoutEngine(input);
    const [worker] = WorkerDouble.instances;
    const rejected = expect(task.result).rejects.toBeInstanceOf(Error);
    if (failure === "execution") worker.onmessage!({ data: { id: 1, error: new Error("ELK failed") } } as MessageEvent);
    if (failure === "loading") worker.onerror!(new ErrorEvent("error", { message: "Unable to load worker" }));
    if (failure === "decode") worker.onmessageerror!();
    if (failure === "invalid") worker.onmessage!({ data: { id: 1, data: { id: "other" } } } as MessageEvent);
    await rejected;
    expect(worker.terminate).toHaveBeenCalledOnce();
    const retry = flowLayoutEngine(input);
    WorkerDouble.instances[1].onmessage!({ data: { id: 1, data: input } } as MessageEvent);
    await expect(retry.result).resolves.toBe(input);
  });

  it("releases the worker if posting input fails", async () => {
    vi.stubGlobal("Worker", class extends WorkerDouble {
      postMessage = vi.fn((message: { cmd: string }) => { if (message.cmd === "layout") throw new Error("Clone failed"); });
    });
    const task = flowLayoutEngine(input);
    await expect(task.result).rejects.toThrow("Clone failed");
    expect(WorkerDouble.instances[0].terminate).toHaveBeenCalledOnce();
  });
});
