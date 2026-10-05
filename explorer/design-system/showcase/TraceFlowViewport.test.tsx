import { act, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { ElementFlow } from "../product-patterns/reports/TraceFlow";

const renderer = vi.hoisted(() => ({ onInit: (_flow: unknown) => {}, zoom: 1 }));
const graphs = vi.hoisted(() => Object.fromEntries(["DOWN", "RIGHT"].map(direction => [direction, {
  direction, bounds: { x: 0, y: 0, width: 300, height: 180 }, edges: [],
  nodes: [{ id: "root", element: { id: "root", name: "Root", file: "Model.md" }, type: "capability", context: "Model", parentCount: 0, position: { x: 0, y: 0 } }],
}])));
vi.mock("../product-patterns/reports/useTraceFlowLayout", () => ({
  useTraceFlowLayout: (_topology: unknown, direction: string) => ({ graph: graphs[direction], pending: false, failed: false, retry: vi.fn() }),
}));
vi.mock("@xyflow/react", async importOriginal => ({
  ...await importOriginal<typeof import("@xyflow/react")>(),
  ReactFlow: ({ onInit }: { onInit: (flow: unknown) => void }) => { renderer.onInit = onInit; return <div />; },
}));
function setup() {
  renderer.zoom = 1;
  vi.spyOn(HTMLElement.prototype, "offsetWidth", "get").mockReturnValue(1000);
  vi.spyOn(HTMLElement.prototype, "offsetHeight", "get").mockReturnValue(700);
  vi.stubGlobal("ResizeObserver", class { observe() {} disconnect() {} });
  const frames = new Map<number, FrameRequestCallback>();
  let id = 0;
  vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => { frames.set(++id, callback); return id; });
  vi.stubGlobal("cancelAnimationFrame", (frame: number) => frames.delete(frame));
  const initialize = () => act(() => renderer.onInit({
    setViewport: ({ zoom }: { zoom: number }) => { renderer.zoom = zoom; return Promise.resolve(true); },
    zoomOut: () => { renderer.zoom /= 1.2; return Promise.resolve(true); },
  }));
  render(<ElementFlow layoutEngine={vi.fn()} data={{ id: "test", title: "Test", nodes: [], edges: [] }} onOpenElement={vi.fn()} onOpenSource={vi.fn()} />);
  return { initialize, frames };
}
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
describe("Flow viewport ownership", () => {
  it("waits for the current direction's canvas before enabling navigation", () => {
    const { initialize } = setup();
    const region = screen.getByRole("region", { name: "Model flow" });
    expect(region.getAttribute("aria-busy")).toBe("true");
    expect((screen.getByRole("button", { name: "Zoom out" }) as HTMLButtonElement).disabled).toBe(true);
    initialize();
    expect(region.getAttribute("aria-busy")).toBe("false");
    fireEvent.click(screen.getByRole("button", { name: "Left to right" }));
    expect(region.getAttribute("aria-busy")).toBe("true");
    fireEvent.click(screen.getByRole("button", { name: "Top to bottom" }));
    expect(region.getAttribute("aria-busy")).toBe("true");
    initialize();
    expect(region.getAttribute("aria-busy")).toBe("false");
  });
  it("ignores delayed initialization from previously unmounted canvases", () => {
    const { initialize } = setup();
    const originalInit = renderer.onInit;
    initialize();
    fireEvent.click(screen.getByRole("button", { name: "Left to right" }));
    const intermediateInit = renderer.onInit;
    fireEvent.click(screen.getByRole("button", { name: "Top to bottom" }));
    initialize();
    const staleFlow = { setViewport: vi.fn(), zoomOut: vi.fn() };
    act(() => originalInit(staleFlow));
    fireEvent.click(screen.getByRole("button", { name: "Zoom out" }));
    expect(staleFlow.zoomOut).not.toHaveBeenCalled();
    act(() => intermediateInit(staleFlow));
    expect(screen.getByRole("region", { name: "Model flow" }).getAttribute("aria-busy")).toBe("false");
  });
  it("does not let a scheduled automatic fit overwrite a user's zoom", () => {
    const { initialize, frames } = setup();
    initialize();
    fireEvent.click(screen.getByRole("button", { name: "Zoom out" }));
    const zoom = renderer.zoom;
    expect(zoom).toBeLessThan(1);
    act(() => { for (const callback of frames.values()) callback(0); });
    expect(renderer.zoom).toBe(zoom);
  });
});
