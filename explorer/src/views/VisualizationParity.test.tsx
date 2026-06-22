import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { useEffect } from "react";
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

vi.mock("graphology-layout-forceatlas2", () => ({
  default: {
    inferSettings: () => ({}),
    assign: vi.fn(),
  },
}));

vi.mock("sigma", () => ({
  default: class MockSigma {
    constructor(_graph: unknown, container: HTMLElement) {
      const canvas = document.createElement("canvas");
      canvas.setAttribute("data-testid", "mock-sigma-renderer");
      container.appendChild(canvas);
    }

    on() {}
    refresh() {}
    kill() {}
    getNodeDisplayData() {
      return { x: 0, y: 0 };
    }
    getCamera() {
      return {
        animatedReset: vi.fn(),
        animate: vi.fn(),
        getState: () => ({ ratio: 1 }),
      };
    }
  },
}));

function renderWithStore(view: React.ReactElement) {
  return render(
    <StoreProvider store={devFixture} schemaMismatch={null}>
      <ExplorerUiStateProvider>{view}</ExplorerUiStateProvider>
    </StoreProvider>,
  );
}

describe("native visualization parity views", () => {
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

  it("renders thesaurus concepts in the native Explorer shell route", () => {
    renderWithStore(<ThesaurusView onOpenElement={vi.fn()} />);

    expect(screen.getAllByText("Service Endpoint").length).toBeGreaterThan(0);
    expect(screen.getByText("Used by model")).toBeTruthy();
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
    expect(screen.queryByText("Summary")).toBeNull();
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
