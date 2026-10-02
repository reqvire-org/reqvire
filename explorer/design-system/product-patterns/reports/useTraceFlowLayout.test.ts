import { act, renderHook, waitFor, cleanup } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { buildTraceFlowGraph, buildTraceFlowTopology, type TraceFlowGraph, type TraceFlowDirection } from "./traceFlowLayout";
import { useTraceFlowLayout } from "./useTraceFlowLayout";

vi.mock("./traceFlowLayout", async importOriginal => ({
  ...await importOriginal<typeof import("./traceFlowLayout")>(),
  buildTraceFlowGraph: vi.fn(),
}));
afterEach(() => { cleanup(); vi.resetAllMocks(); });

const topology = buildTraceFlowTopology({ verification: { id: "v", name: "Check", type: "test-verification", file: "Checks.md" }, requirements: [] });
const graph = (count: number): TraceFlowGraph => ({ ...topology, direction: "RIGHT", nodes: [], edges: [], totalCount: count,
  bounds: { x: 0, y: 0, width: 328, height: 212 },
});
const deferred = () => {
  let resolve!: (value: TraceFlowGraph) => void;
  let reject!: (error: Error) => void;
  const promise = new Promise<TraceFlowGraph>((resolvePromise, rejectPromise) => { resolve = resolvePromise; reject = rejectPromise; });
  return { promise, resolve, reject };
};

describe("asynchronous trace layout", () => {
  it("ignores obsolete direction layouts when the user switches back quickly", async () => {
    const vertical = deferred();
    vi.mocked(buildTraceFlowGraph).mockResolvedValueOnce(graph(1)).mockReturnValueOnce(vertical.promise).mockResolvedValueOnce(graph(2));
    const { result, rerender } = renderHook((direction: TraceFlowDirection) => useTraceFlowLayout(topology, direction), { initialProps: "RIGHT" });
    await waitFor(() => expect(result.current.graph?.totalCount).toBe(1));
    rerender("DOWN");
    expect(buildTraceFlowGraph).toHaveBeenLastCalledWith(topology, "DOWN");
    rerender("RIGHT");
    await waitFor(() => expect(result.current.graph?.totalCount).toBe(2));
    await act(async () => vertical.resolve({ ...graph(3), direction: "DOWN" }));
    expect(result.current.graph?.direction).toBe("RIGHT");
    expect(result.current.graph?.totalCount).toBe(2);
  });

  it("recovers from an initial failure and ignores results after unmount", async () => {
    const attempt = deferred();
    vi.mocked(buildTraceFlowGraph).mockRejectedValueOnce(new Error("Initial failure")).mockResolvedValueOnce(graph(1))
      .mockReturnValueOnce(attempt.promise);
    const { result, rerender, unmount } = renderHook(input => useTraceFlowLayout(input), { initialProps: topology });
    expect(result.current.pending).toBe(true);
    expect(result.current.graph).toBeNull();
    await waitFor(() => expect(result.current.failed).toBe(true));
    expect(result.current.pending).toBe(false);
    act(() => result.current.retry());
    await waitFor(() => expect(result.current.graph?.totalCount).toBe(1));
    rerender({ ...topology });
    unmount();
    await act(async () => attempt.resolve(graph(2)));
    expect(result.current.graph?.totalCount).toBe(1);
  });

  it("discards a stale completion after a newer request finishes", async () => {
    const old = deferred();
    const latest = deferred();
    vi.mocked(buildTraceFlowGraph).mockReturnValueOnce(old.promise).mockReturnValueOnce(latest.promise);
    const { result, rerender } = renderHook(input => useTraceFlowLayout(input), { initialProps: topology });
    rerender({ ...topology });
    await act(async () => latest.resolve(graph(2)));
    expect(result.current.graph?.totalCount).toBe(2);
    await act(async () => old.resolve(graph(1)));
    expect(result.current.graph?.totalCount).toBe(2);
    expect(result.current.pending).toBe(false);
  });

  it("keeps the displayed graph during updates and supports retry after failure", async () => {
    const update = deferred();
    vi.mocked(buildTraceFlowGraph).mockResolvedValueOnce(graph(1)).mockReturnValueOnce(update.promise).mockResolvedValueOnce(graph(3));
    const { result, rerender } = renderHook(input => useTraceFlowLayout(input), { initialProps: topology });
    await waitFor(() => expect(result.current.graph?.totalCount).toBe(1));
    rerender({ ...topology });
    expect(result.current.pending).toBe(true);
    expect(result.current.graph?.totalCount).toBe(1);
    await act(async () => update.reject(new Error("Layout failed")));
    expect(result.current.failed).toBe(true);
    expect(result.current.graph?.totalCount).toBe(1);
    act(() => result.current.retry());
    await waitFor(() => expect(result.current.graph?.totalCount).toBe(3));
    expect(result.current.failed).toBe(false);
  });

  it("ignores a stale rejection and does not relayout when input is unchanged", async () => {
    const old = deferred();
    vi.mocked(buildTraceFlowGraph).mockReturnValueOnce(old.promise).mockResolvedValueOnce(graph(2));
    const { result, rerender } = renderHook(input => useTraceFlowLayout(input), { initialProps: topology });
    rerender(topology);
    expect(buildTraceFlowGraph).toHaveBeenCalledTimes(1);
    rerender({ ...topology });
    await waitFor(() => expect(result.current.graph?.totalCount).toBe(2));
    await act(async () => old.reject(new Error("Old failure")));
    expect(result.current.failed).toBe(false);
    expect(result.current.graph?.totalCount).toBe(2);
  });
});
