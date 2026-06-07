import { Theme } from "@radix-ui/themes";
import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ExplorerSidePane } from "../components/ExplorerSidePane";
import { ExplorerUiStateProvider } from "../components/ExplorerUiState";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import { TracesView } from "./ReportViews";
import { Kn2View, KnowledgeGraphView } from "./GraphLibraryViews";

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

vi.mock("cytoscape", () => {
  type MockElement = Record<string, any>;
  type MockNode = MockElement;
  type MockEdge = MockElement;

  function makeCollection(items: MockElement[]) {
    const collection = [...items] as MockElement[] & Record<string, any>;
    collection.show = () => collection.forEach((item) => item.show());
    collection.hide = () => collection.forEach((item) => item.hide());
    collection.addClass = () => collection;
    collection.removeClass = () => collection;
    collection.removeData = () => collection;
    collection.layout = () => ({ run: vi.fn() });
    collection.union = (other: MockElement[] | MockElement) =>
      makeCollection([
        ...collection,
        ...(Array.isArray(other) ? other : [other]),
      ]);
    collection.contains = (item: MockElement) => collection.includes(item);
    collection.difference = (item: MockElement) =>
      makeCollection(collection.filter((candidate) => candidate !== item));
    const originalFilter = Array.prototype.filter.bind(collection);
    (collection as any).filter = (predicate: (item: MockElement) => boolean) =>
      makeCollection(originalFilter(predicate));
    return collection;
  }

  function makeNode(data: Record<string, unknown>, getEdges: () => MockEdge[]): MockNode {
    let hidden = false;
    const node: MockNode = {
      id: () => String(data.id),
      data: (key?: string, value?: unknown) => {
        if (key && value !== undefined) data[key] = value;
        return key ? data[key] : data;
      },
      hide: () => {
        hidden = true;
      },
      show: () => {
        hidden = false;
      },
      hidden: () => hidden,
      empty: () => false,
      nonempty: () => true,
      connectedEdges: () =>
        makeCollection(
          getEdges().filter(
            (edge) => edge.source() === node || edge.target() === node,
          ),
        ),
      cy: () => mockCore,
      addClass: () => node,
      removeClass: () => node,
      removeData: () => node,
      animate: () => node,
      union: (other: MockElement[] | MockElement) =>
        makeCollection([node, ...(Array.isArray(other) ? other : [other])]),
    };
    return node;
  }

  function makeEdge(
    data: Record<string, unknown>,
    source: MockNode,
    target: MockNode,
  ): MockEdge {
    let hidden = false;
    const edge: MockEdge = {
      id: () => String(data.id),
      data: (key?: string) => (key ? data[key] : data),
      hide: () => {
        hidden = true;
      },
      show: () => {
        hidden = false;
      },
      hidden: () => hidden,
      source: () => source,
      target: () => target,
      connectedNodes: () => makeCollection([source, target]),
      addClass: () => edge,
      removeClass: () => edge,
    };
    return edge;
  }

  let mockCore: Record<string, unknown>;

  return {
    default: vi.fn((options: { container: HTMLElement; elements: { group: string; data: Record<string, unknown> }[] }) => {
      const canvas = document.createElement("canvas");
      canvas.setAttribute("data-testid", "mock-cytoscape-renderer");
      options.container.appendChild(canvas);
      const edgeData = options.elements.filter((element) => element.group === "edges");
      let edges: MockEdge[] = [];
      const nodes = options.elements
        .filter((element) => element.group === "nodes")
        .map((element) => makeNode(element.data, () => edges));
      const nodeById = new Map(nodes.map((node) => [node.id(), node]));
      edges = edgeData.map((element) =>
        makeEdge(
          element.data,
          nodeById.get(String(element.data.source)) ?? nodes[0],
          nodeById.get(String(element.data.target)) ?? nodes[0],
        ),
      );
      mockCore = {
        on: vi.fn(),
        destroy: vi.fn(),
        style: vi.fn(),
        resize: vi.fn(),
        zoom: () => 1,
        animate: vi.fn(),
        collection: () => makeCollection([]),
        elements: () => makeCollection([...nodes, ...edges]),
        nodes: (selector?: string) => {
          if (selector === ":visible") return makeCollection(nodes.filter((node) => !node.hidden()));
          if (selector?.includes("concept")) {
            return makeCollection(nodes.filter((node) => node.data("node_type") === "concept"));
          }
          return makeCollection(nodes);
        },
        edges: (selector?: string) =>
          selector === ":visible"
            ? makeCollection(edges.filter((edge) => !edge.hidden()))
            : makeCollection(edges),
        getElementById: (id: string) =>
          nodes.find((node) => node.id() === id) ?? {
            empty: () => true,
            nonempty: () => false,
          },
      };
      return mockCore;
    }),
  };
});

function renderWithStore(view: React.ReactElement) {
  return render(
    <Theme>
      <StoreProvider store={devFixture} schemaMismatch={null}>
        <ExplorerUiStateProvider>{view}</ExplorerUiStateProvider>
      </StoreProvider>
    </Theme>,
  );
}

describe("native visualization parity views", () => {
  it("renders Knowledge Graph as the native Sigma/Graphology project graph", () => {
    const { container } = renderWithStore(
      <KnowledgeGraphView frameTestId="knowledge-graph" onOpenElement={vi.fn()} />,
    );

    expect(container.querySelector('[data-view="knowledge-graph"]')).toBeTruthy();
    expect(screen.getByTestId("kg-sigma-canvas")).toBeTruthy();
    expect(screen.getByTestId("mock-sigma-renderer")).toBeTruthy();
    expect(screen.getByRole("img", { name: "Actual project elements and facts graph" })).toBeTruthy();
    expect(container.querySelector("iframe")).toBeNull();
  });

  it("renders traces as a native verification flow", () => {
    const { container } = renderWithStore(
      <TracesView onOpenElement={vi.fn()} />,
    );

    expect(screen.getByTestId("trace-flow")).toBeTruthy();
    expect(screen.getByRole("img", { name: "Verification trace flow" })).toBeTruthy();
    expect(screen.getAllByText("Example Verification").length).toBeGreaterThan(0);
    expect(container.querySelector("iframe")).toBeNull();
  });

  it("renders KN2 with native Cytoscape controls, overlays, and canvas", () => {
    const { container } = renderWithStore(
      <>
        <ExplorerSidePane
          activeView="kn2"
          open
          onToggle={vi.fn()}
          onNavigate={vi.fn()}
          onOpenElement={vi.fn()}
        />
        <Kn2View onOpenElement={vi.fn()} />
      </>,
    );

    expect(screen.getByTestId("kn2-cytoscape-canvas")).toBeTruthy();
    expect(screen.getByTestId("mock-cytoscape-renderer")).toBeTruthy();
    expect(screen.getByRole("img", { name: "Cytoscape project graph POC" })).toBeTruthy();
    expect(
      screen.getByRole("button", { name: "CoSE structural" }).classList.contains("is-active"),
    ).toBe(true);
    expect(container.querySelector("#kn2-cross-subgraph-overlay")).toBeTruthy();
    expect(container.querySelector("#kn2-verification-overlay")).toBeTruthy();
    expect(container.querySelector("#kn2-trace-overlay")).toBeTruthy();
    expect(container.querySelector("iframe")).toBeNull();
  });
});
