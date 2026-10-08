import { act, renderHook, waitFor, cleanup } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { ElkNode } from "elkjs/lib/elk-api";
import { buildTraceFlowTopology, type TraceFlowDirection } from "./traceFlowLayout";
import { useTraceFlowLayout } from "./useTraceFlowLayout";

afterEach(cleanup);
const topology = buildTraceFlowTopology({ verification: { id: "v", name: "Check", type: "test-verification", file: "Checks.md" }, requirements: [] });

function controlledEngine() {
  const jobs: { resolve: () => void; reject: (error: Error) => void; cancel: ReturnType<typeof vi.fn> }[] = [];
  const engine = vi.fn((input: ElkNode) => {
    let resolve!: (value: ElkNode) => void;
    let reject!: (error: Error) => void;
    const result = new Promise<ElkNode>((yes, no) => { resolve = yes; reject = no; });
    const cancel = vi.fn(); // Deliberately allow late replies to test the stale-result guard too.
    jobs.push({ cancel, reject, resolve: () => resolve({ ...input, width: 328, height: 212,
      children: input.children?.map(node => ({ ...node, x: 24, y: 24 })),
    }) });
    return { result, cancel };
  });
  return { engine, jobs };
}

describe("cancellable trace layout", () => {
  it("terminates obsolete direction work and ignores its late completion", async () => {
    const { engine, jobs } = controlledEngine();
    const { result, rerender } = renderHook((direction: TraceFlowDirection) => useTraceFlowLayout(topology, direction, engine), { initialProps: "RIGHT" });
    expect(engine).toHaveBeenCalledTimes(1);
    await act(async () => jobs[0].resolve());
    const displayed = result.current.graph;
    rerender("DOWN");
    expect(result.current.graph).toBe(displayed);
    expect(engine.mock.calls[1][0].layoutOptions?.["elk.direction"]).toBe("DOWN");
    rerender("RIGHT");
    expect(jobs[1].cancel).toHaveBeenCalledOnce();
    await act(async () => jobs[2].resolve());
    const latest = result.current.graph;
    await act(async () => jobs[1].resolve());
    expect(result.current.graph).toBe(latest);
    expect(latest?.direction).toBe("RIGHT");
    expect(result.current.pending).toBe(false);
  });

  it("cancels topology/context replacement and teardown independently for simultaneous flows", async () => {
    const { engine, jobs } = controlledEngine();
    const first = renderHook(input => useTraceFlowLayout(input, "RIGHT", engine), { initialProps: topology });
    const second = renderHook(() => useTraceFlowLayout(topology, "DOWN", engine));
    expect(engine).toHaveBeenCalledTimes(2);
    first.rerender(topology);
    expect(engine).toHaveBeenCalledTimes(2);
    const replacement = { ...topology, nodes: topology.nodes.map(node => ({ ...node, element: { ...node.element, name: "Other branch" } })) };
    first.rerender(replacement);
    expect(jobs[0].cancel).toHaveBeenCalledOnce();
    expect(jobs[1].cancel).not.toHaveBeenCalled();
    await act(async () => { jobs[1].resolve(); jobs[2].resolve(); jobs[0].reject(new Error("Obsolete")); });
    expect(first.result.current.graph?.nodes[0].element.name).toBe("Other branch");
    expect(first.result.current.failed).toBe(false);
    expect(second.result.current.graph?.direction).toBe("DOWN");
    first.rerender({ ...replacement });
    first.unmount();
    expect(jobs[3].cancel).toHaveBeenCalledOnce();
    expect(jobs[1].cancel).not.toHaveBeenCalled();
    await act(async () => jobs[3].resolve());
  });

  it("retains the valid map on failure and retries without clearing it", async () => {
    const { engine, jobs } = controlledEngine();
    const { result, rerender } = renderHook(input => useTraceFlowLayout(input, "RIGHT", engine), { initialProps: topology });
    await act(async () => jobs[0].resolve());
    const previous = result.current.graph;
    rerender({ ...topology });
    expect(result.current.pending).toBe(true);
    await act(async () => jobs[1].reject(new Error("Worker failed")));
    expect(result.current.failed).toBe(true);
    expect(result.current.graph).toBe(previous);
    act(() => result.current.retry());
    expect(result.current.graph).toBe(previous);
    await act(async () => jobs[2].resolve());
    expect(result.current.failed).toBe(false);
    expect(result.current.pending).toBe(false);
  });

  it("handles synchronous worker creation failure and retries", async () => {
    const { engine, jobs } = controlledEngine();
    engine.mockImplementationOnce(() => { throw new Error("Worker blocked"); });
    const { result } = renderHook(() => useTraceFlowLayout(topology, "RIGHT", engine));
    await waitFor(() => expect(result.current.failed).toBe(true));
    expect(result.current.graph).toBeNull();
    act(() => result.current.retry());
    await act(async () => jobs[0].resolve());
    expect(result.current.graph?.nodes).toHaveLength(1);
    expect(result.current.failed).toBe(false);
  });
});
