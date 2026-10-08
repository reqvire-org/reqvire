import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { useEffect } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { ElkNode } from "elkjs/lib/elk-api";
import ELK from "elkjs/lib/elk.bundled.js";
import type { FlowLayoutEngine } from "@ds";
import { ExplorerUiStateProvider } from "../state/ExplorerUiState";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import type { ExplorerProjectStore } from "../store/types";
import { ExplorerSidePane } from "../components/ExplorerSidePane";
import * as traceProjection from "../lib/traces";
import * as flowProjection from "../lib/traceFlow";
import { TracesView } from "./ReportViews";

const layoutEngine = vi.hoisted(() => vi.fn());
vi.mock("../workers/flowLayoutEngine", () => ({ flowLayoutEngine: layoutEngine }));
// Actual TraceFlow orchestration and cards; browser checks own canvas/worker wiring.
vi.mock("@xyflow/react", async importOriginal => {
  const original = await importOriginal<typeof import("@xyflow/react")>();
  const instance = { setViewport: vi.fn(), zoomIn: vi.fn(), zoomOut: vi.fn(), zoomTo: vi.fn() };
  return { ...original, Handle: () => null, ReactFlow: ({ nodes, edges, nodeTypes, onInit }: {
    nodes: { id: string; type: string; data: unknown }[];
    edges: { id: string; source: string; target: string; label: string }[];
    nodeTypes: Record<string, React.ComponentType<{ data: unknown }>>;
    onInit: (flow: unknown) => void;
  }) => {
    useEffect(() => { onInit(instance); }, [onInit]);
    return <div data-testid="flow-canvas">{nodes.map(node => {
      const Card = nodeTypes[node.type];
      return <div key={node.id} data-flow-node={node.id}><Card data={node.data} /></div>;
    })}{edges.map(edge => <span key={edge.id} data-flow-edge={edge.id}
      data-source={edge.source} data-target={edge.target}>{edge.label}</span>)}</div>;
  } };
});

// Worker ownership is tested in-browser; jsdom uses the same ELK engine.
const elk = new ELK({ algorithms: ["layered"] });
const testFlowLayoutEngine: FlowLayoutEngine = input => ({ result: elk.layout(input), cancel: () => {} });

const file = "system-model/Specifications.md";
const verificationId = `${file}#verification`;
const onOpenElement = vi.fn();
function fixture(label: string, context = "original", parent = false): ExplorerProjectStore {
  const nodes = [
    { id: `${file}#requirement`, name: label, type: "requirement", is_directly_verified: true },
    ...(parent ? [{ id: `${file}#parent`, name: "Added Parent", type: "requirement", is_directly_verified: false }] : []),
  ];
  return {
    ...devFixture, project: { ...devFixture.project, worktree_id: context },
    elements: [
      { ...devFixture.elements[0], id: verificationId, name: "Shared Verification", element_type: "test-verification", type_family: "verification", file_path: file },
      ...nodes.map(node => ({ ...devFixture.elements[0], id: node.id, name: node.name, element_type: node.type, file_path: file })),
    ],
    traces: { files: { [file]: { verifications: [{
      identifier: verificationId, name: "Shared Verification", file, type: "test-verification",
      directly_verified_count: 1, total_requirements_in_tree: parent ? 2 : 1,
      trace_graph: { nodes, edges: parent ? [{ source: `${file}#requirement`, relation_type: "derivedFrom", target: `${file}#parent` }] : [] },
    }] } } },
  };
}
function withSecondTrace(store: ExplorerProjectStore): ExplorerProjectStore {
  const first = fixture("Second Requirement").traces.files[file].verifications[0];
  return { ...store, elements: [...store.elements, { ...store.elements[0], id: `${file}#second-verification`, name: "Second Verification" }],
    traces: { files: { [file]: { verifications: [
      ...store.traces.files[file].verifications,
      { ...first, identifier: `${file}#second-verification`, name: "Second Verification" },
    ] } } } };
}
function view(store: ExplorerProjectStore) {
  return <StoreProvider store={store} schemaMismatch={null}><ExplorerUiStateProvider>
    <ExplorerSidePane activeView="traces" open onToggle={vi.fn()} onNavigate={vi.fn()}
      onOpenElement={onOpenElement} onOpenOntologyNode={vi.fn()} />
    <TracesView onOpenElement={onOpenElement} />
  </ExplorerUiStateProvider></StoreProvider>;
}
const region = () => screen.getByRole("region", { name: "Verification trace flow" });
async function ready() { await waitFor(() => expect(region().getAttribute("aria-busy")).toBe("false")); }
const cards = () => [...region().querySelectorAll("[data-flow-node]")];
beforeEach(() => {
  localStorage.clear(); window.history.replaceState(null, "", "/#/traces");
  layoutEngine.mockReset().mockImplementation(testFlowLayoutEngine); onOpenElement.mockReset();
  vi.stubGlobal("ResizeObserver", class { observe() {} disconnect() {} });
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); vi.restoreAllMocks(); window.history.replaceState(null, "", "/"); });

describe("native trace snapshot ownership", () => {
  it("uses canonical navigation and defers unselected traces", async () => {
    const prepared = vi.spyOn(flowProjection, "buildVerificationFlow");
    render(view(withSecondTrace(fixture("Old Requirement")))); await ready();
    expect(layoutEngine).toHaveBeenCalledTimes(1); expect(prepared).toHaveBeenCalledTimes(1);
    expect(cards()).toHaveLength(2);
    fireEvent.click(region().querySelector<HTMLAnchorElement>('a[aria-label="Old Requirement"]')!);
    expect(onOpenElement).toHaveBeenLastCalledWith(`${file}#requirement`);
    fireEvent.click(region().querySelector<HTMLAnchorElement>('a[aria-label="Open source for Old Requirement"]')!);
    expect(window.location.hash).toBe("#/content/system-model/Specifications.md");
    fireEvent.click(screen.getAllByText("Specifications.md")[0]);
    expect(screen.getByTestId("trace-rows")).toBeTruthy();
    expect(screen.queryByRole("region", { name: "Verification trace flow" })).toBeNull();
    expect(layoutEngine).toHaveBeenCalledTimes(1); expect(prepared).toHaveBeenCalledTimes(1);
    fireEvent.click(screen.getByRole("button", { name: "Second Verification" })); await ready();
    expect(layoutEngine).toHaveBeenCalledTimes(2);
    expect(cards()[0].getAttribute("data-flow-node")).toBe(`${file}#second-verification`);
  });
  it("shares grouping and prepared flow across coverage-only refreshes", async () => {
    const grouping = vi.spyOn(traceProjection, "buildTraceFiles");
    const prepared = vi.spyOn(flowProjection, "buildVerificationFlow");
    const store = fixture("Old Requirement"); const mounted = render(view(store)); await ready();
    expect(grouping).toHaveBeenCalledTimes(1); expect(prepared).toHaveBeenCalledTimes(1);
    mounted.rerender(view({ ...store, coverage: { ...store.coverage, summary: { ...store.coverage.summary, total_leaf_requirements: 99 } } })); await ready();
    expect(grouping).toHaveBeenCalledTimes(1); expect(prepared).toHaveBeenCalledTimes(1);
    expect(layoutEngine).toHaveBeenCalledTimes(1);
    mounted.rerender(view(fixture("New Requirement", "original", true))); await ready();
    expect(grouping).toHaveBeenCalledTimes(2);
    expect(cards().map(card => card.textContent).join()).toContain("Added Parent");
    expect(region().textContent).not.toContain("Old Requirement");
  });
  it.each(["original", "replacement"])("replaces reused identifiers and labels on refresh/context %s", async context => {
    const mounted = render(view(fixture("Old Requirement"))); await ready();
    mounted.rerender(view(fixture("New Requirement", context, true))); await ready();
    expect(cards()).toHaveLength(3); expect(region().textContent).toContain("New Requirement");
    expect(region().textContent).not.toContain("Old Requirement");
  });
  it("cancels stale work and rejects its completion after a newer snapshot", async () => {
    let finish!: (graph: ElkNode) => void; let oldInput!: ElkNode; const cancel = vi.fn();
    layoutEngine.mockImplementationOnce(input => {
      oldInput = input;
      return { result: new Promise<ElkNode>(resolve => { finish = resolve; }), cancel };
    });
    const mounted = render(view(fixture("Old Requirement")));
    await waitFor(() => expect(layoutEngine).toHaveBeenCalledTimes(1));
    mounted.rerender(view(fixture("New Requirement", "replacement", true))); await ready();
    expect(cancel).toHaveBeenCalledOnce();
    await act(async () => finish(await testFlowLayoutEngine(oldInput).result));
    expect(region().textContent).toContain("New Requirement");
    expect(region().textContent).not.toContain("Old Requirement"); expect(cards()).toHaveLength(3);
  });
  it("retains same-context controls but never shows another worktree's pending graph", async () => {
    const mounted = render(view(fixture("Old Requirement"))); await ready();
    fireEvent.click(screen.getByRole("button", { name: "Top to bottom" })); await ready();
    mounted.rerender(view(fixture("New Requirement", "original", true))); await ready();
    expect(screen.getByRole("button", { name: "Top to bottom" }).getAttribute("aria-pressed")).toBe("true");
    layoutEngine.mockImplementationOnce(() => ({ result: new Promise(() => {}), cancel: vi.fn() }));
    mounted.rerender(view(fixture("Other Requirement", "replacement")));
    await waitFor(() => expect(screen.getByRole("status").textContent).toContain("Loading flow"));
    expect(screen.queryByTestId("flow-canvas")).toBeNull();
  });
  it("provides retry after a worker failure", async () => {
    const cancel = vi.fn();
    layoutEngine.mockImplementationOnce(() => ({ result: Promise.reject(new Error("worker load failed")), cancel }));
    render(view(fixture("Old Requirement")));
    await waitFor(() => expect(screen.getByRole("alert").textContent).toContain("Couldn’t lay out"));
    fireEvent.click(screen.getByRole("button", { name: "Retry" })); await ready();
    expect(cards()).toHaveLength(2); expect(cancel).toHaveBeenCalledOnce();
  });
  it("handles deleted selections and an empty store without mounting a flow", async () => {
    const store = withSecondTrace(fixture("Old Requirement")); const mounted = render(view(store)); await ready();
    const second = store.traces.files[file].verifications[1];
    mounted.rerender(view({ ...store, elements: store.elements.filter(element => element.id !== verificationId),
      traces: { files: { [file]: { verifications: [second] } } } }));
    await waitFor(() => expect(screen.getByTestId("trace-rows")).toBeTruthy());
    expect(screen.queryByRole("region", { name: "Verification trace flow" })).toBeNull();
    mounted.rerender(view({ ...store, elements: [], traces: { files: {} } }));
    await waitFor(() => expect(screen.getByText("No verification traces in store.")).toBeTruthy());
  });
});
