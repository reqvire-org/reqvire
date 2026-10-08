import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { useState } from "react";
import type Graph from "graphology";
import { describe, expect, it, vi } from "vitest";
import { ExplorerUiStateProvider, ONTOLOGY_DEFAULT_FILTERS } from "../state/ExplorerUiState";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import type { ExplorerProjectStore } from "../store/types";
import { ElementIcon, TypeBadge } from "@ds";
import { semanticQueryStore, queryId } from "../store/fixtures/semanticQueryGraph";
import { OntologiesView } from "./OntologiesView";
import { mountOntologyGraph } from "../lib/ontologyGraphRenderer";

type NodeEventHandler = (event: { node: string }) => void;
type GraphReducer = (id: string, attributes: Record<string, unknown>) => Record<string, unknown>;
type RendererSettings = { nodeReducer: GraphReducer; edgeReducer: GraphReducer };

const sigmaState = vi.hoisted(() => ({
  graphs: [] as Graph[],
  settings: [] as RendererSettings[],
  handlers: [] as Array<Map<string, NodeEventHandler>>,
  refreshes: [] as Array<ReturnType<typeof vi.fn>>,
  cameras: [] as Array<{ animate: ReturnType<typeof vi.fn>; animatedReset: ReturnType<typeof vi.fn> }>,
  constructs: 0,
  kills: 0,
}));
const mockAnimateNodes = vi.hoisted(() => vi.fn());
const mockNoverlapAssign = vi.hoisted(() => vi.fn());
const mockForceAtlasAssign = vi.hoisted(() => vi.fn());
const mockForceAtlasEngine = vi.hoisted(() => vi.fn());

vi.mock("../workers/forceAtlasLayoutEngine", async importOriginal => {
  const original = await importOriginal<typeof import("../workers/forceAtlasLayoutEngine")>();
  const { runForceAtlasLayout } = await import("../lib/forceAtlasLayout");
  mockForceAtlasEngine.mockImplementation(input => ({ result: Promise.resolve(runForceAtlasLayout(input)), cancel: vi.fn() }));
  return { ...original, forceAtlasLayoutEngine: mockForceAtlasEngine };
});

vi.mock("graphology-layout-forceatlas2", () => ({
  default: {
    inferSettings: () => ({}),
    assign: mockForceAtlasAssign,
  },
}));

vi.mock("graphology-layout-noverlap", () => ({
  default: {
    assign: mockNoverlapAssign,
  },
}));

vi.mock("@sigma/edge-curve", () => ({
  createDrawCurvedEdgeLabel: () => vi.fn(),
  createEdgeCurveProgram: () => class MockCurvedEdgeProgram {},
  indexParallelEdgesIndex: vi.fn(),
}));

vi.mock("@sigma/node-image", () => ({
  createNodeImageProgram: () => class MockNodeImageProgram {},
}));

vi.mock("sigma/rendering", () => ({
  EdgeProgram: class MockEdgeProgram {},
}));

vi.mock("sigma/utils", () => ({
  animateNodes: mockAnimateNodes,
  floatColor: () => 0,
}));

vi.mock("sigma", () => ({
  default: class MockSigma {
    handlers = new Map<string, NodeEventHandler>();
    camera = {
      animate: vi.fn(),
      animatedReset: vi.fn(),
      getState: () => ({ x: 10, y: 20, ratio: 0.6 }),
    };
    constructor(graph: Graph, _container: HTMLElement, settings: RendererSettings) {
      sigmaState.constructs += 1;
      sigmaState.graphs.push(graph);
      sigmaState.settings.push(settings);
      sigmaState.handlers.push(this.handlers);
      sigmaState.refreshes.push(this.refresh);
      sigmaState.cameras.push(this.camera);
    }
    refresh = vi.fn();
    kill = vi.fn(() => {
      sigmaState.kills += 1;
    });
    on = vi.fn((name: string, handler: NodeEventHandler) => this.handlers.set(name, handler));
    getNodeDisplayData = vi.fn(() => ({ x: 4, y: -3 }));
    getCustomBBox = () => ({});
    viewportToGraph = (event: { x: number; y: number }) => event;
    getCamera = () => this.camera;
  },
}));

function renderWithStore(store: ExplorerProjectStore = devFixture) {
  localStorage.clear(); window.history.replaceState(null, "", "/#/ontologies");
  setupWebGLMock();
  resetSigmaState();
  return render(
    <>
      <StoreProvider store={store} schemaMismatch={null}>
        <ExplorerUiStateProvider>
          <OntologiesView />
        </ExplorerUiStateProvider>
      </StoreProvider>
    </>,
  );
}

function setupWebGLMock() {
  Object.defineProperty(globalThis, "WebGLRenderingContext", {
    configurable: true,
    value: { FLOAT: 5126, TRIANGLES: 4, UNSIGNED_BYTE: 5121 },
  });
}

function resetSigmaState() {
  sigmaState.graphs.length = 0;
  sigmaState.settings.length = 0;
  sigmaState.handlers.length = 0;
  sigmaState.refreshes.length = 0;
  sigmaState.cameras.length = 0;
  sigmaState.constructs = 0;
  sigmaState.kills = 0;
  mockAnimateNodes.mockReset();
  mockAnimateNodes.mockReturnValue(vi.fn());
  mockNoverlapAssign.mockClear();
  mockForceAtlasAssign.mockClear();
  mockForceAtlasEngine.mockClear();
}

describe("OntologiesView", () => {
  it.each(["separated", "colliding", "empty"])("publishes ontology coordinates and the final baseline together for %s input", async mode => {
    setupWebGLMock(); resetSigmaState();
    let resolve!: (positions: { id: string; x: number; y: number }[]) => void;
    mockForceAtlasEngine.mockImplementationOnce(() => ({ result: new Promise(yes => { resolve = yes; }), cancel: vi.fn() }));
    const data = semanticQueryStore.ontology.graph_data!;
    const onLayoutState = vi.fn();
    const handle = mountOntologyGraph(document.createElement("div"), {
      ...data, nodes: [...data.nodes!, { ...data.nodes![1], id: "urn:hidden", full_uri: "urn:hidden", layer: "external-source" }],
    }, { onLayoutState, ...(mode === "empty" ? { initialFilters: [] } : {}) });
    try {
      const graph = sigmaState.graphs[0];
      // Hidden coordinates are excluded from the worker, but their current
      // position must still become the new stable baseline.
      graph.mergeNodeAttributes("urn:hidden", { x: 900, y: -900, retained: "custom" });
      const before = new Map(graph.mapNodes((id, attrs) => [id, { ...attrs }]));
      const input = mockForceAtlasEngine.mock.calls[0][0] as { nodes: { id: string }[] };
      const positions = input.nodes.map(({ id }, i) => ({ id, x: mode === "colliding" ? 7 : 11 + i * 20, y: mode === "colliding" ? 3 : -7 - i * 30 })).reverse();
      expect(positions).toHaveLength(mode === "empty" ? 0 : 3);
      const snapshots: Array<{ hints: unknown; nodes: unknown }> = [];
      const individual = vi.fn();
      graph.on("nodeAttributesUpdated", individual);
      graph.on("eachNodeAttributesUpdated", ({ hints }) => snapshots.push({ hints, nodes: graph.mapNodes((id, attrs) => [id, { ...attrs }]) }));
      await act(async () => resolve(positions));
      await waitFor(() => expect(onLayoutState).toHaveBeenLastCalledWith("ready"));
      expect(snapshots).toHaveLength(1);
      expect(individual).not.toHaveBeenCalled();
      const accepted = new Map(positions.map(position => [position.id, position]));
      let collisionIndex = 0;
      const expected = graph.nodes().map(id => {
        const position = accepted.get(id);
        const attrs = { ...before.get(id), ...(position ? { x: position.x, y: position.y } : {}) };
        if (position && mode === "colliding") {
          attrs.x = 7 + Math.cos(collisionIndex) * collisionIndex * 0.12;
          attrs.y = 3 + Math.sin(collisionIndex) * collisionIndex * 0.12;
          collisionIndex++;
        }
        return [id, { ...attrs, baseX: attrs.x, baseY: attrs.y }];
      });
      expect(graph.mapNodes((id, attrs) => [id, attrs])).toEqual(expected);
      expect(snapshots[0]).toEqual({ hints: { attributes: positions.length ? ["x", "y", "baseX", "baseY"] : ["baseX", "baseY"] }, nodes: expected });
      expect(mockAnimateNodes).not.toHaveBeenCalled();
      expect(sigmaState.refreshes[0]).toHaveBeenCalledOnce();
      expect(sigmaState.cameras[0].animate).not.toHaveBeenCalled();
    } finally { handle.destroy(); }
  });

  it("accepts initial and reset baselines without unchanged-coordinate animations or repeated completion refreshes", async () => {
    setupWebGLMock(); resetSigmaState();
    const data = semanticQueryStore.ontology.graph_data!;
    const onLayoutState = vi.fn();
    const handle = mountOntologyGraph(document.createElement("div"), {
      ...data, nodes: [...data.nodes!, { ...data.nodes![1], id: "urn:hidden", full_uri: "urn:hidden", layer: "external-source" }],
    }, { onLayoutState });
    try {
      await waitFor(() => expect(onLayoutState).toHaveBeenLastCalledWith("ready"));
      const graph = sigmaState.graphs[0];
      expect(graph.getNodeAttribute("urn:hidden", "hidden")).toBe(true);
      expect(mockAnimateNodes).not.toHaveBeenCalled();
      expect(sigmaState.refreshes[0]).toHaveBeenCalledOnce();
      expect(sigmaState.cameras[0].animate).not.toHaveBeenCalled();
      expect(sigmaState.cameras[0].animatedReset).not.toHaveBeenCalled();
      graph.forEachNode((_id, attributes) => {
        expect(attributes.x).toBe(attributes.baseX);
        expect(attributes.y).toBe(attributes.baseY);
      });
      sigmaState.refreshes[0].mockClear();
      onLayoutState.mockClear();
      handle.resetLayout();
      await waitFor(() => expect(onLayoutState).toHaveBeenLastCalledWith("ready"));
      expect(mockAnimateNodes).not.toHaveBeenCalled();
      // Pending reset and accepted completion are separate updates.
      expect(sigmaState.refreshes[0]).toHaveBeenCalledTimes(2);
      expect(sigmaState.cameras[0].animatedReset).toHaveBeenCalledExactlyOnceWith({ duration: 250 });
      graph.forEachNode((_id, attributes) => {
        expect(Number.isFinite(attributes.x) && Number.isFinite(attributes.y)).toBe(true);
        expect(attributes.x).toBe(attributes.baseX);
        expect(attributes.y).toBe(attributes.baseY);
      });
    } finally { handle.destroy(); }
  });

  it.each(["selection", "clear", "filter", "reset", "drag", "dispose"])("cancels an active focus animation on %s", async action => {
    setupWebGLMock(); resetSigmaState();
    const onLayoutState = vi.fn();
    const handle = mountOntologyGraph(document.createElement("div"), semanticQueryStore.ontology.graph_data!, { onLayoutState });
    try {
      await waitFor(() => expect(onLayoutState).toHaveBeenLastCalledWith("ready"));
      let cancelled = false;
      const cancel = vi.fn(() => { cancelled = true; });
      mockAnimateNodes.mockClear();
      mockAnimateNodes.mockReturnValueOnce(cancel);
      handle.focusNode(queryId);
      expect(mockAnimateNodes).toHaveBeenCalledOnce();
      const animation = mockAnimateNodes.mock.calls[0];
      const neighbor = "https://example.org/items#Item";
      if (action === "selection") handle.focusNode(neighbor);
      if (action === "clear") handle.clearSelection();
      if (action === "filter") handle.setFilter("role", "semantic-query", false);
      if (action === "reset") handle.resetLayout();
      if (action === "dispose") handle.destroy();
      if (action === "drag") {
        sigmaState.handlers[0].get("downNode")!({ node: neighbor });
        const move = sigmaState.handlers[0].get("moveBody") as unknown as (input: { event: { x: number; y: number } }) => void;
        move({ event: { x: 111, y: 222 } });
        // Model Sigma's cancellation: a retired animation cannot finish later.
        if (!cancelled) {
          const targets = animation[1] as Record<string, { x: number; y: number }>;
          Object.entries(targets).forEach(([id, target]) => sigmaState.graphs[0].mergeNodeAttributes(id, target));
          animation[3]?.();
        }
        expect(sigmaState.graphs[0].getNodeAttribute(neighbor, "x")).toBe(111);
        expect(sigmaState.graphs[0].getNodeAttribute(neighbor, "y")).toBe(222);
      }
      expect(cancel).toHaveBeenCalledOnce();
    } finally { handle.destroy(); }
  });

  it("cancels a pending layout before dragging and never overwrites the dragged coordinates", async () => {
    let late!: (positions: { id: string; x: number; y: number }[]) => void;
    const cancel = vi.fn();
    mockForceAtlasEngine.mockImplementationOnce(() => ({ result: new Promise(yes => { late = yes; }), cancel }));
    const view = renderWithStore(semanticQueryStore);
    await waitFor(() => expect(sigmaState.constructs).toBe(1));
    act(() => sigmaState.handlers[0].get("downNode")!({ node: queryId }));
    const move = sigmaState.handlers[0].get("moveBody") as unknown as (input: { event: { x: number; y: number } }) => void;
    act(() => move({ event: { x: 111, y: 222 } }));
    expect(cancel).toHaveBeenCalledOnce();
    act(() => late([{ id: queryId, x: 9999, y: 9999 }]));
    await Promise.resolve();
    expect(sigmaState.graphs[0].getNodeAttribute(queryId, "x")).toBe(111);
    expect(sigmaState.graphs[0].getNodeAttribute(queryId, "y")).toBe(222);
    view.unmount();
  });

  it("cancels obsolete filter/reset jobs and retains the map after failure until retry", async () => {
    let reject!: (error: Error) => void;
    const cancel = vi.fn();
    mockForceAtlasEngine.mockImplementationOnce(() => ({ result: new Promise((_resolve, no) => { reject = no; }), cancel }));
    renderWithStore(semanticQueryStore);
    await waitFor(() => expect(sigmaState.graphs[0]).toBeTruthy());
    expect(screen.getByText("Laying out ontology...")).toBeTruthy();
    const graph = sigmaState.graphs[0];
    const before = graph.getNodeAttribute(queryId, "x");
    act(() => reject(new Error("failed")));
    await waitFor(() => expect(screen.getByRole("button", { name: "Retry layout" })).toBeTruthy());
    expect(graph.getNodeAttribute(queryId, "x")).toBe(before);
    fireEvent.click(screen.getByRole("button", { name: "Retry layout" }));
    await waitFor(() => expect(screen.queryByText("Laying out ontology...")).toBeNull());
    expect(sigmaState.constructs).toBe(1);
    let late!: (positions: { id: string; x: number; y: number }[]) => void;
    mockForceAtlasEngine.mockImplementationOnce(() => ({ result: new Promise(yes => { late = yes; }), cancel }));
    act(() => window.resetOntologyGraphLayout?.());
    act(() => window.setOntologyGraphFilter?.("role", "semantic-query", false));
    expect(cancel).toHaveBeenCalledOnce();
    const latest = mockForceAtlasAssign.mock.calls.at(-1)![0] as Graph;
    expect(latest.hasNode(queryId)).toBe(false);
    act(() => late([{ id: queryId, x: 9999, y: 9999 }]));
    await waitFor(() => expect(screen.queryByText("Laying out ontology...")).toBeNull());
    expect(graph.getNodeAttribute(queryId, "x")).not.toBe(9999);
  });

  it("retires equal-ID context jobs even when the ontology graph object is shared", async () => {
    let late!: (positions: { id: string; x: number; y: number }[]) => void;
    const cancel = vi.fn();
    mockForceAtlasEngine.mockImplementationOnce(() => ({ result: new Promise(yes => { late = yes; }), cancel }));
    const view = renderWithStore(semanticQueryStore);
    await waitFor(() => expect(sigmaState.constructs).toBe(1));
    const old = sigmaState.graphs[0];
    const before = old.getNodeAttribute(queryId, "x");
    const next = { ...semanticQueryStore, project: { ...semanticQueryStore.project, worktree_id: "other" } };
    view.rerender(<StoreProvider store={next} schemaMismatch={null}><ExplorerUiStateProvider><OntologiesView /></ExplorerUiStateProvider></StoreProvider>);
    await waitFor(() => expect(sigmaState.constructs).toBe(2));
    expect(cancel).toHaveBeenCalledOnce();
    act(() => late([{ id: queryId, x: 9999, y: 9999 }]));
    await Promise.resolve();
    expect(old.getNodeAttribute(queryId, "x")).toBe(before);
    expect(sigmaState.graphs[1].getNodeAttribute(queryId, "x")).not.toBe(9999);
  });

  it("excludes hidden layers and disabled query endpoints from the first ForceAtlas input", () => {
    setupWebGLMock();
    resetSigmaState();
    const data = semanticQueryStore.ontology.graph_data!;
    const template = data.nodes![1];
    const container = document.createElement("div");
    const handle = mountOntologyGraph(container, {
      nodes: [...data.nodes!, { ...template, id: "urn:hidden", full_uri: "urn:hidden", layer: "external-source" }],
      edges: [...data.edges!, { source: template.id, target: "urn:hidden", label: "subclass", layer: "external-source", source_kind: "ontology" }],
    }, { initialFilters: ONTOLOGY_DEFAULT_FILTERS.filter(value => value !== "semantic-query") });
    try {
      const input = mockForceAtlasAssign.mock.calls[0][0] as Graph;
      expect(input.nodes()).toEqual([template.id, "https://example.org/items#name"]);
      expect(input.size).toBe(0);
      expect(sigmaState.graphs[0].getNodeAttribute("urn:hidden", "hidden")).toBe(true);
      expect(sigmaState.graphs[0].getNodeAttribute(queryId, "hidden")).toBe(true);
      handle.setFilter("layer", "layer-external-source", true);
      handle.resetLayout();
      const replacement = mockForceAtlasAssign.mock.calls.at(-1)![0] as Graph;
      expect(replacement.nodes()).toEqual([template.id, "https://example.org/items#name", "urn:hidden"]);
      expect(replacement.size).toBe(1);
    } finally { handle.destroy(); }
  });

  it("uses the caller's initial layer membership before running any layout", () => {
    setupWebGLMock();
    resetSigmaState();
    const data = semanticQueryStore.ontology.graph_data!;
    const handle = mountOntologyGraph(document.createElement("div"), data, { initialFilters: [] });
    try {
      const input = mockForceAtlasAssign.mock.calls[0][0] as Graph;
      expect(input.order).toBe(0);
      expect(input.size).toBe(0);
    } finally { handle.destroy(); }
  });

  it("keeps reset and disposal owned by one renderer when another is mounted", async () => {
    setupWebGLMock(); resetSigmaState();
    const cancellations = [vi.fn(), vi.fn(), vi.fn()];
    for (const cancel of cancellations) mockForceAtlasEngine.mockImplementationOnce(() => ({ result: new Promise(() => {}), cancel }));
    const first = mountOntologyGraph(document.createElement("div"), semanticQueryStore.ontology.graph_data!);
    const second = mountOntologyGraph(document.createElement("div"), semanticQueryStore.ontology.graph_data!);
    first.resetLayout();
    expect(cancellations[0]).toHaveBeenCalledOnce();
    expect(cancellations[1]).not.toHaveBeenCalled();
    first.destroy();
    expect(cancellations[2]).toHaveBeenCalledOnce();
    expect(window.resetOntologyGraphLayout).toBeDefined();
    expect(cancellations[1]).not.toHaveBeenCalled();
    second.destroy();
    expect(cancellations[1]).toHaveBeenCalledOnce();
    await Promise.resolve();
  });

  it("does not resynchronize unchanged filters on selection or surrounding rerenders", async () => {
    const { rerender } = renderWithStore(semanticQueryStore);
    await waitFor(() => expect(sigmaState.graphs[0]).toBeTruthy());
    const sync = vi.spyOn(window, "syncOntologyGraphFilters");
    act(() => window.focusOntologyNode?.(queryId));
    expect(sync).not.toHaveBeenCalled();
    rerender(<StoreProvider store={semanticQueryStore} schemaMismatch={null}>
      <ExplorerUiStateProvider><OntologiesView /></ExplorerUiStateProvider>
    </StoreProvider>);
    expect(sync).not.toHaveBeenCalled();
    sync.mockRestore();
  });

  it("skips equivalent filter application but honors changed membership and individual toggles", async () => {
    renderWithStore(semanticQueryStore);
    await waitFor(() => expect(sigmaState.graphs[0]).toBeTruthy());
    const graph = sigmaState.graphs[0];
    act(() => window.focusOntologyNode?.(queryId));
    const setAttribute = vi.spyOn(graph, "setNodeAttribute");
    mockNoverlapAssign.mockClear();
    act(() => window.syncOntologyGraphFilters?.([...ONTOLOGY_DEFAULT_FILTERS].reverse()));
    expect(setAttribute).not.toHaveBeenCalled();
    expect(mockNoverlapAssign).not.toHaveBeenCalled();
    const withoutQueries = [...ONTOLOGY_DEFAULT_FILTERS].map(value => value === "semantic-query" ? "layer-external-source" : value);
    act(() => window.syncOntologyGraphFilters?.(withoutQueries));
    expect(graph.getNodeAttribute(queryId, "hidden")).toBe(true);
    expect(setAttribute).toHaveBeenCalled();
    act(() => window.syncOntologyGraphFilters?.([...ONTOLOGY_DEFAULT_FILTERS]));
    expect(graph.getNodeAttribute(queryId, "hidden")).toBe(false);
    act(() => window.setOntologyGraphFilter?.("role", "semantic-query", false));
    expect(graph.getNodeAttribute(queryId, "hidden")).toBe(true);
    act(() => window.syncOntologyGraphFilters?.([...ONTOLOGY_DEFAULT_FILTERS]));
    expect(graph.getNodeAttribute(queryId, "hidden")).toBe(false);
    setAttribute.mockClear();
    act(() => window.setOntologyGraphFilter?.("role", "semantic-query", true));
    expect(setAttribute).not.toHaveBeenCalled();
  });

  it("renders query glyphs and retains vocabulary property links, with a query visibility filter", async () => {
    renderWithStore(semanticQueryStore);
    await waitFor(() => expect(sigmaState.graphs[0]).toBeTruthy());
    const graph = sigmaState.graphs[0] as unknown as {
      hasNode: (id: string) => boolean;
      getNodeAttributes: (id: string) => Record<string, unknown>;
      hasDirectedEdge: (from: string, to: string) => boolean;
    };
    const query = graph.getNodeAttributes(queryId);
    expect(query.type).toBe("queryGlyph");
    expect(query.fullLabel).toBe("Item labels");
    expect(decodeURIComponent(String(query.image))).toContain(">Q</text>");
    expect(decodeURIComponent(String(query.image))).toContain("<circle ");
    expect(decodeURIComponent(String(query.image))).not.toContain("<rect ");
    expect(query.hidden).toBe(false);
    expect(graph.hasNode("https://example.org/items#name")).toBe(true);
    expect(graph.getNodeAttributes("https://example.org/items#name").hidden).toBe(false);
    expect(graph.hasDirectedEdge(queryId, "https://example.org/items#name")).toBe(true);
    expect(graph.hasDirectedEdge(queryId, "https://example.org/items#Item")).toBe(true);
    act(() => window.setOntologyGraphFilter?.("role", "semantic-query", false));
    expect(graph.getNodeAttributes(queryId).hidden).toBe(true);
    act(() => window.setOntologyGraphFilter?.("role", "semantic-query", true));
    expect(graph.getNodeAttributes(queryId).hidden).toBe(false);
  });

  it.each(["hover", "selection"])("shows query relations and their vocabulary targets on %s, respecting visibility filters", async (interaction) => {
    renderWithStore(semanticQueryStore);
    await waitFor(() => expect(sigmaState.graphs[0]).toBeTruthy());
    const graph = sigmaState.graphs[0] as unknown as {
      edges: () => string[];
      getEdgeAttributes: (id: string) => Record<string, unknown>;
      getNodeAttributes: (id: string) => Record<string, unknown>;
    };
    const settings = sigmaState.settings[0];
    const renderedEdges = () => graph.edges().map(id => settings.edgeReducer(id, graph.getEdgeAttributes(id)));
    expect(renderedEdges()).toHaveLength(2);
    expect(renderedEdges().every(edge => edge.hidden)).toBe(true);
    act(() => {
      if (interaction === "hover") sigmaState.handlers[0].get("enterNode")?.({ node: queryId });
      else window.focusOntologyNode?.(queryId);
    });
    const visible = renderedEdges().filter(edge => !edge.hidden);
    expect(visible.map(edge => edge.label).sort()).toEqual(["declares output", "uses vocabulary"]);
    for (const edge of visible) {
      const target = String(edge.target);
      const node = settings.nodeReducer(target, graph.getNodeAttributes(target));
      expect(node.hidden).toBe(false);
      expect(node.inFocusNeighborhood).toBe(true);
      expect(node.label).toBeTruthy();
    }
    for (const [category, value] of [["role", "semantic-query"], ["layer", "layer-authored"]]) {
      act(() => window.setOntologyGraphFilter?.(category, value, false));
      expect(renderedEdges().every(edge => edge.hidden)).toBe(true);
      act(() => window.setOntologyGraphFilter?.(category, value, true));
      expect(renderedEdges().filter(edge => !edge.hidden)).toHaveLength(2);
    }
  });

  it("uses the shared query marker in element icons and type badges", () => {
    render(<><ElementIcon type="semantic-query" /><TypeBadge type="semantic-query" /></>);
    expect(screen.getAllByText("Q")).toHaveLength(2);
  });

  it("renders the TypeScript ontology graph without injected renderer assets", () => {
    const { container } = renderWithStore();

    const graph = screen.getByRole("img", { name: "Ontology and SHACL relationship graph" });
    expect(graph).toBeTruthy();
    expect(container.querySelector('[data-view="ontologies"]')).toBeTruthy();
    expect(container.querySelector("#ontology-graph-container")).toBeTruthy();
    expect(container.querySelector("iframe")).toBeNull();
    expect(container.querySelector('script[type="module"]')).toBeNull();
    expect(container.querySelector("#ontology-graph-search")).toBeNull();
  });

  it("does not pass exported ontology node type values to Sigma renderer programs", async () => {
    const graphData = devFixture.ontology.graph_data;
    expect(graphData).toBeTruthy();
    const store: ExplorerProjectStore = {
      ...devFixture,
      ontology: {
        ...devFixture.ontology,
        graph_data: {
          ...graphData,
          nodes: (graphData?.nodes ?? []).map((node, index) =>
            index === 0 ? { ...node, type: "owl" } : node,
          ),
        },
      },
    };

    renderWithStore(store);

    await waitFor(() => expect(sigmaState.graphs[0]).toBeTruthy());

    const rendererTypes: unknown[] = [];
    sigmaState.graphs[0].forEachNode((_node, attributes) => {
      rendererTypes.push(attributes.type);
    });
    expect(rendererTypes).not.toContain("owl");
    expect(rendererTypes.every((type) => type === "circle" || type === "constructGlyph")).toBe(true);
  });

  it("uses visible ontology graph size and density to tune full graph ForceAtlas spacing", async () => {
    renderWithStore();

    await waitFor(() => expect(mockForceAtlasAssign).toHaveBeenCalled());
    const settings = mockForceAtlasAssign.mock.calls.at(-1)?.[1]?.settings;

    expect(settings.scalingRatio).not.toBe(16);
    expect(settings.scalingRatio).toBeGreaterThanOrEqual(5);
    expect(settings.scalingRatio).toBeLessThanOrEqual(13);
    expect(settings.gravity).toBeGreaterThanOrEqual(1.5);
    expect(settings.gravity).toBeLessThanOrEqual(3.2);
  });

  it("keeps the ontology graph mounted when the shell re-renders", async () => {
    setupWebGLMock();
    resetSigmaState();

    function OntologyShell() {
      const [revision, setRevision] = useState(0);
      return (
        <>
          <button type="button" onClick={() => setRevision((value) => value + 1)}>
            refresh shell {revision}
          </button>
          <StoreProvider store={devFixture} schemaMismatch={null}>
            <ExplorerUiStateProvider>
              <OntologiesView />
            </ExplorerUiStateProvider>
          </StoreProvider>
        </>
      );
    }

    render(<OntologyShell />);

    await waitFor(() => expect(sigmaState.constructs).toBe(1));
    fireEvent.click(screen.getByRole("button", { name: /refresh shell/ }));

    expect(sigmaState.kills).toBe(0);
    expect(sigmaState.constructs).toBe(1);
  });

  it.each(["clear", "hide-selection"])("animates changed focus/restoration coordinates only and preserves the camera on %s", async action => {
    setupWebGLMock(); resetSigmaState();
    const data = semanticQueryStore.ontology.graph_data!;
    const onLayoutState = vi.fn();
    const handle = mountOntologyGraph(document.createElement("div"), {
      ...data, nodes: [...data.nodes!,
        { ...data.nodes![1], id: "urn:unrelated", full_uri: "urn:unrelated" },
        { ...data.nodes![1], id: "urn:hidden", full_uri: "urn:hidden", layer: "external-source" },
      ],
    }, { onLayoutState });
    try {
      await waitFor(() => expect(onLayoutState).toHaveBeenLastCalledWith("ready"));
      const graph = sigmaState.graphs[0];
      const baseline = new Map(graph.nodes().map(id => [id, { x: graph.getNodeAttribute(id, "baseX"), y: graph.getNodeAttribute(id, "baseY") }]));
      const completeAnimation = () => {
        const [animationGraph, targets, , complete] = mockAnimateNodes.mock.calls.at(-1)!;
        Object.entries(targets as Record<string, { x: number; y: number }>).forEach(([id, target]) => (animationGraph as Graph).mergeNodeAttributes(id, target));
        complete?.();
      };
      mockAnimateNodes.mockClear();
      sigmaState.refreshes[0].mockClear();
      handle.focusNode(queryId);
      expect(mockNoverlapAssign).toHaveBeenCalledOnce();
      expect((mockNoverlapAssign.mock.calls[0][0] as Graph).nodes().sort()).toEqual([queryId, "https://example.org/items#Item", "https://example.org/items#name"].sort());
      expect(mockAnimateNodes).toHaveBeenCalledOnce();
      const focusTargets = mockAnimateNodes.mock.calls[0][1] as Record<string, { x: number; y: number }>;
      expect(Object.keys(focusTargets).sort()).toEqual(["https://example.org/items#Item", "https://example.org/items#name"]);
      expect(mockAnimateNodes.mock.calls[0][2]).toMatchObject({ duration: 250, easing: "quadraticOut" });
      expect(sigmaState.refreshes[0]).toHaveBeenCalledOnce();
      completeAnimation();
      expect(sigmaState.refreshes[0]).toHaveBeenCalledTimes(2);
      expect(sigmaState.cameras[0].animate).toHaveBeenLastCalledWith({ x: 4, y: -3, ratio: 0.6 }, { duration: 280 });
      expect(graph.getNodeAttribute(queryId, "x")).toBe(baseline.get(queryId)!.x);
      // Hidden nodes that actually moved must still return to the baseline.
      graph.mergeNodeAttributes("urn:hidden", { x: baseline.get("urn:hidden")!.x + 9 });
      mockAnimateNodes.mockClear();
      sigmaState.refreshes[0].mockClear();
      sigmaState.cameras[0].animate.mockClear();
      if (action === "clear") handle.clearSelection();
      else {
        mockForceAtlasEngine.mockImplementationOnce(() => ({ result: new Promise(() => {}), cancel: vi.fn() }));
        handle.setFilter("role", "semantic-query", false);
        expect(graph.getNodeAttribute(queryId, "hidden")).toBe(true);
      }
      expect(mockAnimateNodes).toHaveBeenCalledOnce();
      const restoreTargets = mockAnimateNodes.mock.calls[0][1] as Record<string, { x: number; y: number }>;
      expect(Object.keys(restoreTargets).sort()).toEqual(["https://example.org/items#Item", "https://example.org/items#name", "urn:hidden"]);
      expect(sigmaState.refreshes[0]).toHaveBeenCalledOnce();
      completeAnimation();
      expect(sigmaState.refreshes[0]).toHaveBeenCalledTimes(2);
      graph.forEachNode((id, attributes) => {
        expect({ x: attributes.x, y: attributes.y }).toEqual(baseline.get(id));
      });
      expect(sigmaState.cameras[0].animate).not.toHaveBeenCalled();
      expect(sigmaState.cameras[0].animatedReset).not.toHaveBeenCalled();
      mockAnimateNodes.mockClear();
      sigmaState.refreshes[0].mockClear();
      handle.clearSelection();
      expect(mockAnimateNodes).not.toHaveBeenCalled();
      expect(sigmaState.refreshes[0]).toHaveBeenCalledOnce();
      expect(mockForceAtlasEngine).toHaveBeenCalledTimes(action === "clear" ? 1 : 2);
      sigmaState.refreshes[0].mockClear();
      handle.focusNode("urn:unrelated");
      expect(mockAnimateNodes).not.toHaveBeenCalled();
      expect(sigmaState.refreshes[0]).toHaveBeenCalledOnce();
      expect(sigmaState.cameras[0].animate).toHaveBeenCalledExactlyOnceWith({ x: 4, y: -3, ratio: 0.6 }, { duration: 280 });
      expect(mockForceAtlasEngine).toHaveBeenCalledTimes(action === "clear" ? 1 : 2);
    } finally { handle.destroy(); }
  });

  it("keeps Explorer external-source graph data limited to used external subset terms", () => {
    const graphNodes = devFixture.ontology.graph_data?.nodes ?? [];
    const externalNodes = graphNodes.filter((node) => node.layer === "external-source");

    expect(externalNodes.length).toBeGreaterThan(0);
    expect(externalNodes.every((node) => node.source_kind === "external-ontology")).toBe(true);
    expect(externalNodes.every((node) => node.sources.some((source) => source.kind === "external-used-subset"))).toBe(true);
    expect(externalNodes.every((node) =>
      node.constraints.some(
        (constraint) => constraint.property === "external_materialization" && constraint.value === "used_subset",
      ),
    )).toBe(true);
    expect(graphNodes.some((node) => node.id.includes("UnusedExternal"))).toBe(false);
  });

});
