import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it, vi } from "vitest";
import { ExplorerUiStateProvider } from "../state/ExplorerUiState";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import type { ExplorerProjectStore } from "../store/types";
import { OntologiesView } from "./OntologiesView";

const sigmaState = vi.hoisted(() => ({
  graphs: [] as Array<{ forEachNode: (callback: (node: string, attributes: Record<string, unknown>) => void) => void }>,
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
    constructor(graph: { forEachNode: (callback: (node: string, attributes: Record<string, unknown>) => void) => void }) {
      sigmaState.constructs += 1;
      sigmaState.graphs.push(graph);
    }
    refresh = vi.fn();
    kill = vi.fn(() => {
      sigmaState.kills += 1;
    });
    on = vi.fn();
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
  sigmaState.constructs = 0;
  sigmaState.kills = 0;
  mockAnimateNodes.mockReset();
  mockAnimateNodes.mockReturnValue(vi.fn());
  mockNoverlapAssign.mockClear();
  mockForceAtlasAssign.mockClear();
}

describe("OntologiesView", () => {
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
