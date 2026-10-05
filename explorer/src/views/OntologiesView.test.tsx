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

type NodeEventHandler = (event: { node: string }) => void;
type GraphReducer = (id: string, attributes: Record<string, unknown>) => Record<string, unknown>;
type RendererSettings = { nodeReducer: GraphReducer; edgeReducer: GraphReducer };

const sigmaState = vi.hoisted(() => ({
  graphs: [] as Graph[],
  settings: [] as RendererSettings[],
  handlers: [] as Array<Map<string, NodeEventHandler>>,
  constructs: 0,
  kills: 0,
}));
const mockAnimateNodes = vi.hoisted(() => vi.fn());
const mockNoverlapAssign = vi.hoisted(() => vi.fn());
const mockForceAtlasAssign = vi.hoisted(() => vi.fn());

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
    constructor(graph: Graph, _container: HTMLElement, settings: RendererSettings) {
      sigmaState.constructs += 1;
      sigmaState.graphs.push(graph);
      sigmaState.settings.push(settings);
      sigmaState.handlers.push(this.handlers);
    }
    refresh = vi.fn();
    kill = vi.fn(() => {
      sigmaState.kills += 1;
    });
    on = vi.fn((name: string, handler: NodeEventHandler) => this.handlers.set(name, handler));
    getNodeDisplayData = vi.fn(() => ({ x: 4, y: -3 }));
    getCamera = () => ({
      animate: vi.fn(),
      animatedReset: vi.fn(),
      getState: () => ({ ratio: 1 }),
    });
  },
}));

function renderWithStore(store: ExplorerProjectStore = devFixture) {
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
  sigmaState.constructs = 0;
  sigmaState.kills = 0;
  mockAnimateNodes.mockReset();
  mockAnimateNodes.mockReturnValue(vi.fn());
  mockNoverlapAssign.mockClear();
  mockForceAtlasAssign.mockClear();
}

describe("OntologiesView", () => {
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

  it("relayouts selected ontology neighborhoods with noverlap and animated node positions", async () => {
    renderWithStore();

    await waitFor(() => expect(sigmaState.graphs[0]).toBeTruthy());
    const graph = sigmaState.graphs[0] as unknown as {
      nodes: () => string[];
      degree: (node: string) => number;
    };
    const focusedNode = graph.nodes().find((node) => graph.degree(node) > 0);
    expect(focusedNode).toBeTruthy();

    const focusOntologyNode = (
      window as Window & { focusOntologyNode?: (nodeId: string) => void }
    ).focusOntologyNode;
    expect(focusOntologyNode).toBeTruthy();
    act(() => {
      focusOntologyNode?.(focusedNode ?? "");
    });

    await waitFor(() => expect(mockNoverlapAssign).toHaveBeenCalled());
    expect(mockAnimateNodes).toHaveBeenCalled();
    expect(mockAnimateNodes.mock.calls.at(-1)?.[2]).toMatchObject({
      duration: 250,
      easing: "quadraticOut",
    });

    mockAnimateNodes.mockClear();
    const clearOntologySelection = (
      window as Window & { clearOntologySelection?: () => void }
    ).clearOntologySelection;
    expect(clearOntologySelection).toBeTruthy();
    act(() => {
      clearOntologySelection?.();
    });

    await waitFor(() => expect(mockAnimateNodes).toHaveBeenCalled());
    const restoreTargets = mockAnimateNodes.mock.calls.at(-1)?.[1] as Record<string, { x: number; y: number }>;
    expect(Object.keys(restoreTargets).length).toBe(graph.nodes().length);
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
