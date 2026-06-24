import { fireEvent, render, screen, waitFor } from "@testing-library/react";
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

vi.mock("graphology-layout-forceatlas2", () => ({
  default: {
    inferSettings: () => ({}),
    assign: vi.fn(),
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
    getNodeDisplayData = vi.fn();
    getCamera = () => ({
      animate: vi.fn(),
      animatedReset: vi.fn(),
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
