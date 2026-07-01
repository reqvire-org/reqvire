import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { useEffect, useState } from "react";
import { describe, expect, it, vi } from "vitest";
import { ExplorerSidePane } from "../components/ExplorerSidePane";
import { ExplorerUiStateProvider, useExplorerUiState } from "../state/ExplorerUiState";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import {
  __testBuildTraceRollupMermaid,
  TracesView,
} from "./ReportViews";
import { KnowledgeGraphView } from "./GraphLibraryViews";
import { ThesaurusView } from "./ThesaurusView";

const mockSigmaConstruct = vi.hoisted(() => vi.fn());
const mockSigmaKill = vi.hoisted(() => vi.fn());
const mockSigmaHandlers = vi.hoisted(() => new Map<string, (event: { node: string }) => void>());
const mockAnimateNodes = vi.hoisted(() => vi.fn());
const mockNoverlapAssign = vi.hoisted(() => vi.fn());
const mockForceAtlasAssign = vi.hoisted(() => vi.fn());
const mockCameraAnimate = vi.hoisted(() => vi.fn());
const mockCameraReset = vi.hoisted(() => vi.fn());

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

vi.mock("sigma", () => ({
  default: class MockSigma {
    constructor(_graph: unknown, container: HTMLElement) {
      mockSigmaConstruct();
      const canvas = document.createElement("canvas");
      canvas.setAttribute("data-testid", "mock-sigma-renderer");
      container.appendChild(canvas);
    }

    on(event: string, handler: (event: { node: string }) => void) {
      mockSigmaHandlers.set(event, handler);
    }
    refresh() {}
    kill() {
      mockSigmaKill();
    }
    getNodeDisplayData() {
      return { x: 12, y: -8 };
    }
    getCamera() {
      return {
        animatedReset: mockCameraReset,
        animate: mockCameraAnimate,
        getState: () => ({ ratio: 1 }),
      };
    }
  },
}));

vi.mock("sigma/utils", () => ({
  animateNodes: mockAnimateNodes,
}));

function renderWithStore(view: React.ReactElement, store = devFixture) {
  return render(
    <StoreProvider store={store} schemaMismatch={null}>
      <ExplorerUiStateProvider>{view}</ExplorerUiStateProvider>
    </StoreProvider>,
  );
}

describe("native visualization parity views", () => {
  beforeEach(() => {
    mockSigmaConstruct.mockClear();
    mockSigmaKill.mockClear();
    mockSigmaHandlers.clear();
    mockNoverlapAssign.mockClear();
    mockForceAtlasAssign.mockClear();
    mockAnimateNodes.mockReset();
    mockAnimateNodes.mockReturnValue(vi.fn());
    mockCameraAnimate.mockClear();
    mockCameraReset.mockClear();
  });

  it("renders Graph as the native Sigma/Graphology project graph", async () => {
    const { container } = renderWithStore(
      <KnowledgeGraphView frameTestId="model-graph" onOpenElement={vi.fn()} />,
    );

    expect(container.querySelector('[data-view="model-graph"]')).toBeTruthy();
    expect(screen.getByTestId("kg-sigma-canvas")).toBeTruthy();
    await waitFor(() => expect(screen.getByTestId("mock-sigma-renderer")).toBeTruthy());
    expect(screen.getByRole("img", { name: "Actual project elements and facts graph" })).toBeTruthy();
    expect(container.querySelector("iframe")).toBeNull();
  });

  it("uses visible graph size and density to tune full graph ForceAtlas spacing", async () => {
    const buildGraphStore = (nodeCount: number, edgeModulo: number) => ({
      ...devFixture,
      knowledge_graph: {
        nodes: Array.from({ length: nodeCount }, (_item, index) => ({
          id: `system-model/Specifications.md#requirement-${index}`,
          identifier: `system-model/Specifications.md#requirement-${index}`,
          label: `Requirement ${index}`,
          type: "requirement",
          node_type: "requirement",
          element_type: "requirement",
          file_path: "system-model/Specifications.md",
        })),
        edges: Array.from({ length: Math.max(0, nodeCount - 1) }, (_item, index) => ({
          source: `system-model/Specifications.md#requirement-${index}`,
          target: `system-model/Specifications.md#requirement-${(index + edgeModulo) % nodeCount}`,
          label: "derivedFrom",
          kind: "derived_from",
        })),
      },
    });

    const sparse = renderWithStore(
      <KnowledgeGraphView frameTestId="model-graph" onOpenElement={vi.fn()} />,
      buildGraphStore(18, 1),
    );
    await waitFor(() => expect(mockForceAtlasAssign).toHaveBeenCalled());
    const sparseSettings = mockForceAtlasAssign.mock.calls.at(-1)?.[1]?.settings;
    sparse.unmount();

    mockForceAtlasAssign.mockClear();
    renderWithStore(
      <KnowledgeGraphView frameTestId="model-graph" onOpenElement={vi.fn()} />,
      buildGraphStore(120, 7),
    );
    await waitFor(() => expect(mockForceAtlasAssign).toHaveBeenCalled());
    const largerSettings = mockForceAtlasAssign.mock.calls.at(-1)?.[1]?.settings;

    expect(sparseSettings.scalingRatio).not.toBe(18);
    expect(largerSettings.scalingRatio).not.toBe(18);
    expect(largerSettings.scalingRatio).toBeGreaterThan(sparseSettings.scalingRatio);
    expect(largerSettings.gravity).toBeGreaterThan(sparseSettings.gravity);
  });

  it("keeps the project graph mounted when modal route handlers change", async () => {
    function GraphShell() {
      const [revision, setRevision] = useState(0);
      return (
        <>
          <button type="button" onClick={() => setRevision((value) => value + 1)}>
            refresh shell
          </button>
          <KnowledgeGraphView
            frameTestId="model-graph"
            onOpenElement={() => {
              void revision;
            }}
          />
        </>
      );
    }

    renderWithStore(<GraphShell />);

    await waitFor(() => expect(mockSigmaConstruct).toHaveBeenCalledTimes(1));
    fireEvent.click(screen.getByRole("button", { name: "refresh shell" }));

    expect(mockSigmaKill).not.toHaveBeenCalled();
    expect(mockSigmaConstruct).toHaveBeenCalledTimes(1);
  });

  it("relayouts selected graph neighborhoods with noverlap and animated node positions", async () => {
    renderWithStore(
      <KnowledgeGraphView frameTestId="model-graph" onOpenElement={vi.fn()} />,
    );

    await waitFor(() => expect(mockSigmaConstruct).toHaveBeenCalledTimes(1));
    const clickNode = mockSigmaHandlers.get("clickNode");
    expect(clickNode).toBeTruthy();
    act(() => {
      clickNode?.({ node: "system-model/Specifications.md#example-requirement" });
    });

    await waitFor(() => expect(mockNoverlapAssign).toHaveBeenCalled());
    expect(mockAnimateNodes).toHaveBeenCalled();
    const animateCall = mockAnimateNodes.mock.calls.at(-1);
    const targets = animateCall?.[1] as Record<string, { x: number; y: number }>;
    expect(targets["system-model/Specifications.md#example-requirement"]).toBeTruthy();
    expect(animateCall?.[2]).toMatchObject({ duration: 250, easing: "quadraticOut" });
    expect(typeof animateCall?.[3]).toBe("function");
    expect(mockCameraAnimate).toHaveBeenCalledWith(
      expect.objectContaining({ x: 12, y: -8, ratio: 1 }),
      { duration: 250, easing: "quadraticOut" },
    );
  });

  it("opens selected concept-reference targets as native model elements", () => {
    const conceptId = "system-model/Thesaurus/Thesaurus.md#service-endpoint";
    const openElement = vi.fn();
    const store = {
      ...devFixture,
      knowledge_graph: {
        ...devFixture.knowledge_graph,
        nodes: [
          ...(devFixture.knowledge_graph.nodes ?? []),
          {
            id: conceptId,
            identifier: conceptId,
            label: "Service Endpoint",
            type: "concept",
            node_type: "concept",
            element_type: "concept",
            file_path: "system-model/Thesaurus/Thesaurus.md",
            line_number: 70,
            link: "#/content/system-model/Thesaurus/Thesaurus.md#service-endpoint",
            description: "Endpoint concept referenced by the fixture requirement.",
          },
        ],
        edges: [
          ...(devFixture.knowledge_graph.edges ?? []),
          {
            source: "system-model/Specifications.md#example-requirement",
            target: conceptId,
            label: "conceptRef",
            kind: "concept-reference",
            authored: true,
          },
        ],
      },
    };

    function SelectedConceptPane() {
      const ui = useExplorerUiState();
      useEffect(() => {
        ui.setModelMode("graph");
        ui.setKnowledgeGraphSelectionId(conceptId);
      }, [ui]);
      return (
        <ExplorerSidePane
          activeView="model"
          open
          onToggle={vi.fn()}
          onNavigate={vi.fn()}
          onOpenElement={openElement}
          onOpenOntologyNode={vi.fn()}
        />
      );
    }

    renderWithStore(<SelectedConceptPane />, store);

    fireEvent.click(screen.getByRole("button", { name: /Service Endpoint/ }));
    expect(openElement).toHaveBeenCalledWith(conceptId);
  });

  it("renders thesaurus concepts in the native Explorer shell route", () => {
    renderWithStore(<ThesaurusView onOpenElement={vi.fn()} />);

    expect(screen.getByRole("img", { name: /Example Thesaurus concept map/ })).toBeTruthy();
    expect(screen.getAllByText("Service Endpoint").length).toBeGreaterThan(0);
    expect(screen.getByText("Concept scheme")).toBeTruthy();
  });

  it("uses the Explorer pane as the thesaurus concept tree", () => {
    renderWithStore(
      <ExplorerSidePane
        activeView="thesaurus"
        open
        onToggle={vi.fn()}
        onNavigate={vi.fn()}
        onOpenElement={vi.fn()}
        onOpenOntologyNode={vi.fn()}
      />,
    );

    expect(screen.getByRole("tree", { name: "Concept hierarchy" })).toBeTruthy();
    expect(screen.getByText("Example Thesaurus")).toBeTruthy();
    expect(screen.getByText("Service Endpoint")).toBeTruthy();
  });

  it("shows graph-linked resources in the model tree with folder structure", () => {
    window.location.hash = "#/model";
    renderWithStore(
      <ExplorerSidePane
        activeView="model"
        open
        onToggle={vi.fn()}
        onNavigate={vi.fn()}
        onOpenElement={vi.fn()}
        onOpenOntologyNode={vi.fn()}
      />,
    );

    const tree = screen.getByRole("tree", { name: "Project tree" });
    expect(within(tree).queryByText("reqvire workspace")).toBeNull();
    expect(within(tree).queryByText("reqvire @ dev-fixture")).toBeNull();
    expect(within(tree).getByText("Model")).toBeTruthy();
    expect(within(tree).getByText("Resources")).toBeTruthy();
    expect(within(tree).getAllByText("reqvire").length).toBeGreaterThanOrEqual(2);
    expect(within(tree).getAllByText("system-model").length).toBeGreaterThanOrEqual(1);

    const search = screen.getByRole("searchbox", { name: "Filter project tree" });
    fireEvent.change(search, { target: { value: "api-smoke" } });

    expect(within(tree).getByText("reqvire")).toBeTruthy();
    expect(within(tree).getByText("Evidence")).toBeTruthy();
    const resourceRow = within(tree).getByTitle("system-model/Evidence/api-smoke-report.json");
    expect(resourceRow.querySelector('[data-element-role="other"]')).toBeTruthy();
    expect(resourceRow.querySelector('[data-element-role="resource"]')).toBeNull();
    fireEvent.click(resourceRow);

    expect(window.location.hash).toBe("#/resources/resource:system-model/Evidence/api-smoke-report.json");
  });

  it("collapses a selected thesaurus branch when its row is clicked again", () => {
    function SelectedThesaurusPane() {
      const ui = useExplorerUiState();
      useEffect(() => {
        ui.setThesaurusSelectionId("urn:reqvire:test:concepts#ServiceEndpoint");
      }, [ui]);
      return (
        <ExplorerSidePane
          activeView="thesaurus"
          open
          onToggle={vi.fn()}
          onNavigate={vi.fn()}
          onOpenElement={vi.fn()}
          onOpenOntologyNode={vi.fn()}
        />
      );
    }

    renderWithStore(<SelectedThesaurusPane />);

    expect(screen.getByText("Service Endpoint")).toBeTruthy();
    fireEvent.click(screen.getByText("Example Thesaurus"));

    expect(screen.queryByText("Service Endpoint")).toBeNull();
  });

  it("renders traces as native verification rows", () => {
    const { container } = renderWithStore(
      <TracesView onOpenElement={vi.fn()} />,
    );

    expect(screen.getByTestId("trace-rows")).toBeTruthy();
    expect(screen.getAllByText("Example Verification").length).toBeGreaterThan(0);
    expect(container.querySelector("iframe")).toBeNull();
  });

  it("uses the Explorer pane as a verification trace tree", () => {
    renderWithStore(
      <ExplorerSidePane
        activeView="traces"
        open
        onToggle={vi.fn()}
        onNavigate={vi.fn()}
        onOpenElement={vi.fn()}
        onOpenOntologyNode={vi.fn()}
      />,
    );

    expect(screen.getByLabelText("Verification trace tree")).toBeTruthy();
    expect(screen.getByText("Summary")).toBeTruthy();
    expect(screen.getAllByText("Verifications").length).toBeGreaterThan(1);
    expect(screen.queryByText("Legend")).toBeNull();
    expect(screen.getByText("Specifications.md")).toBeTruthy();
    expect(screen.getByText("Example Verification")).toBeTruthy();
  });

  it("builds per-verification roll-up Mermaid diagrams from trace trees", () => {
    const mermaid = __testBuildTraceRollupMermaid(
      {
        id: "system-model/Traces.md#verify-api",
        name: "Verify API",
        file: "system-model/Traces.md",
        directCount: 1,
        totalCount: 2,
        requirementIds: ["system-model/Traces.md#api-response"],
        verificationType: "test-verification",
        traceTree: {
          requirements: [
            {
              id: "system-model/Traces.md#api-response",
              name: "API Response",
              type: "system-requirement",
              is_directly_verified: true,
              children: [
                {
                  id: "system-model/Traces.md#api-root",
                  name: "API Root",
                  type: "user-requirement",
                  is_directly_verified: false,
                  children: [],
                },
              ],
            },
          ],
        },
      },
      new Map(),
    );

    expect(mermaid).toContain("graph TD");
    expect(mermaid).toContain("classDef verification");
    expect(mermaid).not.toContain("var(--");
    expect(mermaid).toContain("subgraph");
    expect(mermaid).toContain("|verifies|");
    expect(mermaid).toContain("|derivedFrom|");
    expect(mermaid).toContain("Verify API");
    expect(mermaid).toContain("API Response");
    expect(mermaid).toContain("API Root");
  });

  it("uses the Explorer pane search for ontology graph filtering", () => {
    const filterOntologyGraph = vi.fn();
    window.filterOntologyGraph = filterOntologyGraph;

    const { container } = renderWithStore(
      <ExplorerSidePane
        activeView="ontologies"
        open
        onToggle={vi.fn()}
        onNavigate={vi.fn()}
        onOpenElement={vi.fn()}
        onOpenOntologyNode={vi.fn()}
      />,
    );

    const search = screen.getByRole("searchbox", { name: "Search Explorer" });
    expect(search.getAttribute("id")).toBe("ontology-graph-search");
    expect(container.querySelector("#ontology-graph-results")).toBeTruthy();

    fireEvent.change(search, { target: { value: "shape" } });

    expect(filterOntologyGraph).toHaveBeenCalledWith("shape");
    delete window.filterOntologyGraph;
  });

  it("renders the ontology relation legend with a compact line marker", () => {
    renderWithStore(
      <ExplorerSidePane
        activeView="ontologies"
        open
        onToggle={vi.fn()}
        onNavigate={vi.fn()}
        onOpenElement={vi.fn()}
        onOpenOntologyNode={vi.fn()}
      />,
    );

    expect(screen.getByRole("button", { name: "Relation" }).getAttribute("class")).toContain("togglerow--line");
  });

  it("shows selected ontology node link in the Explorer pane", () => {
    const openOntologyNode = vi.fn();
    const node = devFixture.ontology.graph_data?.nodes?.[0];
    expect(node).toBeTruthy();

    function SelectedOntologyPane() {
      const ui = useExplorerUiState();
      useEffect(() => {
        ui.setOntologySelectionId(node?.id ?? null);
      }, [ui]);
      return (
        <ExplorerSidePane
          activeView="ontologies"
          open
          onToggle={vi.fn()}
          onNavigate={vi.fn()}
          onOpenElement={vi.fn()}
          onOpenOntologyNode={openOntologyNode}
        />
      );
    }

    renderWithStore(<SelectedOntologyPane />);

    fireEvent.click(screen.getByRole("button", { name: new RegExp(node?.label ?? "", "i") }));
    expect(openOntologyNode).toHaveBeenCalledWith(node?.id);
  });
});
