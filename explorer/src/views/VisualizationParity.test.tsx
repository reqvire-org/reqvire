import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { useEffect, useState } from "react";
import type Graph from "graphology";
import { describe, expect, it, vi } from "vitest";
import { ExplorerSidePane } from "../components/ExplorerSidePane";
import { ExplorerUiStateProvider, useExplorerUiState } from "../state/ExplorerUiState";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import {
  TracesView,
} from "./ReportViews";
import { KnowledgeGraphView } from "./GraphLibraryViews";
import { writeExplorerHash } from "../router/location";
import { routeForSelection, routeForElement } from "../router/routes";
import { buildVerificationFlow } from "../lib/traceFlow";
import { ThesaurusView } from "./ThesaurusView";

const mockSigmaConstruct = vi.hoisted(() => vi.fn());
const mockSigmaKill = vi.hoisted(() => vi.fn());
const mockSigmaRefresh = vi.hoisted(() => vi.fn());
const mockSigmaHandlers = vi.hoisted(() => new Map<string, (event: { node: string }) => void>());
const mockAnimateNodes = vi.hoisted(() => vi.fn());
const mockNoverlapAssign = vi.hoisted(() => vi.fn());
const mockForceAtlasAssign = vi.hoisted(() => vi.fn());
const mockForceAtlasEngine = vi.hoisted(() => vi.fn());
const mockCameraAnimate = vi.hoisted(() => vi.fn());
const mockCameraReset = vi.hoisted(() => vi.fn());

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

vi.mock("sigma", () => ({
  default: class MockSigma {
    constructor(graph: Graph, container: HTMLElement, settings: unknown) {
      mockSigmaConstruct(graph, settings);
      const canvas = document.createElement("canvas");
      canvas.setAttribute("data-testid", "mock-sigma-renderer");
      container.appendChild(canvas);
    }

    on(event: string, handler: (event: { node: string }) => void) {
      mockSigmaHandlers.set(event, handler);
    }
    refresh() { mockSigmaRefresh(); }
    kill() {
      mockSigmaKill();
    }
    getNodeDisplayData() {
      return { x: 12, y: -8 };
    }
    getCustomBBox() { return {}; }
    viewportToGraph(event: { x: number; y: number }) { return event; }
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
    localStorage.clear();
    window.history.replaceState(null, "", "/#/model");
    mockSigmaConstruct.mockClear();
    mockSigmaKill.mockClear();
    mockSigmaRefresh.mockClear();
    mockSigmaHandlers.clear();
    mockNoverlapAssign.mockClear();
    mockForceAtlasAssign.mockClear();
    mockForceAtlasEngine.mockClear();
    mockAnimateNodes.mockReset();
    mockAnimateNodes.mockReturnValue(vi.fn());
    mockCameraAnimate.mockClear();
    mockCameraReset.mockClear();
  });

  it.each(["unselected", "focused", "restoration", "isolated"])("lets focus handling own the accepted-layout refresh for %s state", async mode => {
    const canvasContext = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    let resolve!: (positions: { id: string; x: number; y: number }[]) => void;
    mockForceAtlasEngine.mockImplementationOnce(() => ({ result: new Promise(yes => { resolve = yes; }), cancel: vi.fn() }));
    function Selection() {
      const ui = useExplorerUiState();
      return <><button onClick={() => ui.setKnowledgeGraphSelectionId(mode === "isolated" ? "c" : "a")}>Select test node</button><button onClick={() => ui.setKnowledgeGraphSelectionId(null)}>Clear test selection</button></>;
    }
    const nodes = ["a", "b", "c"].map(id => ({ id, identifier: id, label: id, element_type: "requirement", file_path: "fixture.md" }));
    const view = renderWithStore(<><Selection /><KnowledgeGraphView frameTestId="model-graph" /></>, { ...devFixture, knowledge_graph: { nodes, edges: [{ source: "a", target: "b", label: "derive", kind: "relation" }] } });
    try {
      await waitFor(() => expect(mockSigmaConstruct).toHaveBeenCalledOnce());
      const graph = mockSigmaConstruct.mock.calls[0][0] as Graph;
      const cancelPrevious = vi.fn();
      if (mode !== "unselected") {
        mockAnimateNodes.mockReturnValueOnce(cancelPrevious);
        fireEvent.click(screen.getByRole("button", { name: "Select test node" }));
        expect(mockCameraAnimate).toHaveBeenCalledWith({ x: 12, y: -8, ratio: 1 }, { duration: 250, easing: "quadraticOut" });
      }
      const previousWasAnimated = mockAnimateNodes.mock.calls.length > 0;
      mockAnimateNodes.mockClear(); mockSigmaRefresh.mockClear(); mockCameraAnimate.mockClear(); mockCameraReset.mockClear();
      const positions = nodes.map(({ id }, i) => ({ id, x: 100 + i * 40, y: -30 - i * 30 }));
      await act(async () => resolve(positions));
      await waitFor(() => expect(screen.queryByText("Laying out graph...")).toBeNull());
      expect(mockSigmaRefresh).toHaveBeenCalledOnce();
      expect(mockSigmaConstruct).toHaveBeenCalledOnce();
      expect(mockForceAtlasEngine).toHaveBeenCalledOnce();
      expect(mockCameraAnimate).not.toHaveBeenCalled();
      expect(mockCameraReset).not.toHaveBeenCalled();
      if (previousWasAnimated) expect(cancelPrevious).toHaveBeenCalledOnce();
      graph.forEachNode((id, attrs) => {
        const position = positions.find(position => position.id === id)!;
        expect({ x: attrs.x, y: attrs.y, baseX: attrs.baseX, baseY: attrs.baseY }).toEqual({ x: position.x, y: position.y, baseX: position.x, baseY: position.y });
      });
      if (mode === "unselected" || mode === "isolated") expect(mockAnimateNodes).not.toHaveBeenCalled();
      else {
        expect(mockAnimateNodes).toHaveBeenCalledOnce();
        const completeAnimation = () => {
          const [animationGraph, targets, settings, complete] = mockAnimateNodes.mock.calls.at(-1)!;
          expect(animationGraph).toBe(graph);
          expect(settings).toEqual({ duration: 250, easing: "quadraticOut" });
          Object.entries(targets as Record<string, { x: number; y: number }>).forEach(([id, target]) => graph.mergeNodeAttributes(id, target));
          act(() => complete());
        };
        const targets = mockAnimateNodes.mock.calls[0][1] as Record<string, { x: number; y: number }>;
        expect(Object.keys(targets).sort()).toEqual(["a", "b"]);
        expect(targets.a).toEqual({ x: 100, y: -30 });
        expect(targets.b).not.toEqual({ x: 140, y: -60 });
        completeAnimation();
        expect(mockSigmaRefresh).toHaveBeenCalledTimes(2);
        expect(mockCameraAnimate).not.toHaveBeenCalled();
        expect(graph.getNodeAttribute("c", "x")).toBe(180);
        if (mode === "restoration") {
          mockAnimateNodes.mockClear(); mockSigmaRefresh.mockClear();
          fireEvent.click(screen.getByRole("button", { name: "Clear test selection" }));
          expect(mockSigmaRefresh).toHaveBeenCalledOnce();
          expect(mockAnimateNodes).toHaveBeenCalledOnce();
          completeAnimation();
          expect(mockSigmaRefresh).toHaveBeenCalledTimes(2);
          graph.forEachNode((_id, attrs) => {
            expect(attrs.x).toBe(attrs.baseX); expect(attrs.y).toBe(attrs.baseY);
          });
          expect(mockCameraAnimate).not.toHaveBeenCalled();
          expect(mockCameraReset).not.toHaveBeenCalled();
        }
      }
    } finally { view.unmount(); canvasContext.mockRestore(); }
  });

  it.each(["all", "subset", "empty"])("publishes accepted Model Graph coordinates together with baselines for %s input", async mode => {
    const canvasContext = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const jobs: Array<(positions: { id: string; x: number; y: number }[]) => void> = [];
    const deferred = () => ({ result: new Promise<{ id: string; x: number; y: number }[]>(resolve => jobs.push(resolve)), cancel: vi.fn() });
    for (let i = 0; i < (mode === "all" ? 1 : mode === "subset" ? 2 : 3); i++) mockForceAtlasEngine.mockImplementationOnce(deferred);
    function Filters() {
      const ui = useExplorerUiState();
      return <><button onClick={() => ui.toggleModelType("requirement")}>Hide requirements</button><button onClick={() => ui.toggleModelType("capability")}>Hide capabilities</button></>;
    }
    const nodes = ["requirement", "requirement", "capability"].map((type, i) => ({ id: `n${i}`, identifier: `n${i}`, label: `Node ${i}`, element_type: type, file_path: "fixture.md" }));
    const view = renderWithStore(<><Filters /><KnowledgeGraphView frameTestId="model-graph" /></>, { ...devFixture, knowledge_graph: { nodes, edges: [] } });
    try {
      await waitFor(() => expect(mockSigmaConstruct).toHaveBeenCalledOnce());
      if (mode !== "all") {
        fireEvent.click(screen.getByRole("button", { name: "Hide requirements" }));
        await waitFor(() => expect(mockForceAtlasEngine).toHaveBeenCalledTimes(2));
      }
      if (mode === "empty") {
        fireEvent.click(screen.getByRole("button", { name: "Hide capabilities" }));
        await waitFor(() => expect(mockForceAtlasEngine).toHaveBeenCalledTimes(3));
      }
      const graph = mockSigmaConstruct.mock.calls[0][0] as Graph;
      const before = new Map(graph.mapNodes((id, attrs) => [id, { ...attrs }]));
      const input = mockForceAtlasEngine.mock.calls.at(-1)![0] as { nodes: { id: string }[] };
      const positions = input.nodes.map(({ id }, i) => ({ id, x: 31 + i * 40, y: -17 - i * 20 })).reverse();
      expect(positions).toHaveLength(mode === "all" ? 3 : mode === "subset" ? 1 : 0);
      const snapshots: Array<{ hints: unknown; nodes: unknown }> = [];
      const individual = vi.fn();
      graph.on("nodeAttributesUpdated", individual);
      graph.on("eachNodeAttributesUpdated", ({ hints }) => snapshots.push({ hints, nodes: graph.mapNodes((id, attrs) => [id, { ...attrs }]) }));
      await act(async () => jobs.at(-1)!(positions));
      await waitFor(() => expect(screen.queryByText("Laying out graph...")).toBeNull());
      expect(individual).not.toHaveBeenCalled();
      expect(snapshots).toHaveLength(positions.length ? 1 : 0);
      const accepted = new Map(positions.map(position => [position.id, position]));
      const expected = graph.nodes().map(id => {
        const position = accepted.get(id);
        return [id, position ? { ...before.get(id), x: position.x, y: position.y, baseX: position.x, baseY: position.y } : before.get(id)];
      });
      expect(graph.mapNodes((id, attrs) => [id, attrs])).toEqual(expected);
      if (positions.length) expect(snapshots[0]).toEqual({ hints: { attributes: ["x", "y", "baseX", "baseY"] }, nodes: expected });
      expect(mockSigmaConstruct).toHaveBeenCalledOnce();
      expect(mockCameraAnimate).not.toHaveBeenCalled();
    } finally { view.unmount(); canvasContext.mockRestore(); }
  });

  it("resolves construction colors once per distinct token even for large graphs and unknown role names", async () => {
    const tokenRead = vi.fn((token: string) => ({
      "--requirement": "#112233", "--contract": "#223344", "--other": "#334455",
      "--edge-derive": "#445566", "--edge-satisfy": "#556677",
    })[token] ?? "");
    const originalStyle = window.getComputedStyle.bind(window);
    const computedStyle = vi.spyOn(window, "getComputedStyle").mockImplementation(element => element === document.documentElement
      ? { getPropertyValue: tokenRead } as unknown as CSSStyleDeclaration : originalStyle(element));
    const canvasContext = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const nodes = Array.from({ length: 1000 }, (_, i) => ({
      id: `n${i}`, identifier: `n${i}`, label: `Node ${i}`, file_path: "fixture.md",
      element_type: i % 3 === 0 ? "requirement" : i % 3 === 1 ? "behavior" : `unknown-role-${i}`,
    }));
    const edges = nodes.slice(1).map((node, i) => ({ source: "n0", target: node.id, label: i % 2 ? "derive" : "verify", kind: "relation" }));
    const view = renderWithStore(<KnowledgeGraphView frameTestId="model-graph" />, { ...devFixture, knowledge_graph: { nodes, edges } });
    try {
      await waitFor(() => expect(mockSigmaConstruct).toHaveBeenCalledOnce());
      const graph = mockSigmaConstruct.mock.calls[0][0] as Graph;
      for (const token of ["--requirement", "--contract", "--other", "--edge-derive", "--edge-satisfy"]) {
        expect(tokenRead.mock.calls.filter(([name]) => name === token)).toHaveLength(1);
      }
      expect(graph.getNodeAttribute("n0", "color")).toBe("#112233");
      expect(graph.getNodeAttribute("n1", "color")).toBe("#223344");
      expect(graph.getNodeAttribute("n2", "color")).toBe("#334455");
      expect(graph.getEdgeAttribute("e0", "color")).toBe("#556677");
      expect(graph.getEdgeAttribute("e1", "color")).toBe("#445566");
    } finally { view.unmount(); computedStyle.mockRestore(); canvasContext.mockRestore(); }
  });

  it("reads a new construction palette on context replacement and keeps live focus-edge resolution", async () => {
    let requirement = "#112233", derive = "#445566";
    const tokenRead = vi.fn((token: string) => token === "--requirement" ? requirement : token === "--edge-derive" ? derive : "");
    const originalStyle = window.getComputedStyle.bind(window);
    const computedStyle = vi.spyOn(window, "getComputedStyle").mockImplementation(element => element === document.documentElement
      ? { getPropertyValue: tokenRead } as unknown as CSSStyleDeclaration : originalStyle(element));
    const canvasContext = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const projection = {
      nodes: [{ id: "a", identifier: "a", label: "A", element_type: "requirement", file_path: "fixture.md" }, { id: "b", identifier: "b", label: "B", element_type: "requirement", file_path: "fixture.md" }],
      edges: [{ source: "a", target: "b", label: "derive", kind: "relation" }],
    };
    const initial = { ...devFixture, knowledge_graph: projection };
    const view = renderWithStore(<KnowledgeGraphView frameTestId="model-graph" />, initial);
    try {
      await waitFor(() => expect(mockSigmaConstruct).toHaveBeenCalledOnce());
      const graph = mockSigmaConstruct.mock.calls[0][0] as Graph;
      expect(graph.getNodeAttribute("a", "color")).toBe(requirement);
      requirement = "#778899"; derive = "#8899aa";
      act(() => mockSigmaHandlers.get("enterNode")!({ node: "a" }));
      const settings = mockSigmaConstruct.mock.calls[0][1] as { edgeReducer: (id: string, attributes: Record<string, unknown>) => Record<string, unknown> };
      expect(settings.edgeReducer("e0", graph.getEdgeAttributes("e0")).color).toBe(derive);
      expect(graph.getNodeAttribute("a", "color")).toBe("#112233");
      const next = { ...initial, project: { ...initial.project, worktree_id: "other" } };
      view.rerender(<StoreProvider store={next} schemaMismatch={null}><ExplorerUiStateProvider><KnowledgeGraphView frameTestId="model-graph" /></ExplorerUiStateProvider></StoreProvider>);
      await waitFor(() => expect(mockSigmaConstruct).toHaveBeenCalledTimes(2));
      expect((mockSigmaConstruct.mock.calls[1][0] as Graph).getNodeAttribute("a", "color")).toBe(requirement);
      expect(tokenRead.mock.calls.filter(([name]) => name === "--requirement")).toHaveLength(2);
    } finally { view.unmount(); computedStyle.mockRestore(); canvasContext.mockRestore(); }
  });

  it("retires a pending Model Graph job before dragging and ignores its late coordinates", async () => {
    const canvasContext = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    let resolve!: (positions: { id: string; x: number; y: number }[]) => void;
    const cancel = vi.fn();
    mockForceAtlasEngine.mockImplementationOnce(() => ({ result: new Promise(yes => { resolve = yes; }), cancel }));
    const view = renderWithStore(<KnowledgeGraphView frameTestId="model-graph" />);
    await waitFor(() => expect(mockSigmaConstruct).toHaveBeenCalledOnce());
    const graph = mockSigmaConstruct.mock.calls[0][0] as Graph;
    const id = mockForceAtlasEngine.mock.calls[0][0].nodes[0].id;
    act(() => mockSigmaHandlers.get("downNode")!({ node: id }));
    const move = mockSigmaHandlers.get("moveBody") as unknown as (input: { event: { x: number; y: number } }) => void;
    act(() => move({ event: { x: 111, y: 222 } }));
    expect(cancel).toHaveBeenCalledOnce();
    await act(async () => resolve([{ id, x: 9999, y: 9999 }]));
    expect(graph.getNodeAttribute(id, "x")).toBe(111);
    expect(graph.getNodeAttribute(id, "y")).toBe(222);
    expect(screen.queryByText("Laying out graph...")).toBeNull();
    view.unmount();
    canvasContext.mockRestore();
  });

  it("retires a pending Model Graph job on context replacement and ignores late failure", async () => {
    const canvasContext = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    let reject!: (error: Error) => void;
    const cancel = vi.fn();
    mockForceAtlasEngine.mockImplementationOnce(() => ({ result: new Promise((_yes, no) => { reject = no; }), cancel }));
    const view = renderWithStore(<KnowledgeGraphView frameTestId="model-graph" />);
    await waitFor(() => expect(mockSigmaConstruct).toHaveBeenCalledOnce());
    expect(screen.getByText("Laying out graph...")).toBeTruthy();
    const next = { ...devFixture, project: { ...devFixture.project, worktree_id: "other" } };
    view.rerender(<StoreProvider store={next} schemaMismatch={null}><ExplorerUiStateProvider><KnowledgeGraphView frameTestId="model-graph" /></ExplorerUiStateProvider></StoreProvider>);
    await waitFor(() => expect(mockSigmaConstruct).toHaveBeenCalledTimes(2));
    expect(cancel).toHaveBeenCalledOnce();
    act(() => reject(new Error("retired context")));
    await waitFor(() => expect(screen.queryByText("Laying out graph...")).toBeNull());
    expect(screen.queryByText("Graph layout failed.")).toBeNull();
    view.unmount();
    canvasContext.mockRestore();
  });

  it("shows a retry after Model Graph worker failure without reconstructing Sigma", async () => {
    mockForceAtlasEngine.mockImplementationOnce(() => ({ result: Promise.reject(new Error("failed")), cancel: vi.fn() }));
    const warning = vi.spyOn(console, "warn").mockImplementation(() => {});
    try {
      renderWithStore(<KnowledgeGraphView frameTestId="model-graph" />);
      await waitFor(() => expect(screen.getByRole("button", { name: "Retry layout" })).toBeTruthy());
      mockSigmaRefresh.mockClear();
      fireEvent.click(screen.getByRole("button", { name: "Retry layout" }));
      await waitFor(() => expect(screen.queryByText("Laying out graph...")).toBeNull());
      expect(mockSigmaConstruct).toHaveBeenCalledOnce();
      expect(mockSigmaRefresh).toHaveBeenCalledOnce();
    } finally { warning.mockRestore(); }
  });

  it("replaces a pending Model Graph job on filter changes without rebuilding the renderer", async () => {
    const cancel = vi.fn();
    mockForceAtlasEngine.mockImplementationOnce(() => ({ result: new Promise(() => {}), cancel }));
    function Filters() {
      const ui = useExplorerUiState();
      return <button onClick={() => ui.toggleModelType("requirement")}>Hide requirements</button>;
    }
    const view = renderWithStore(<><Filters /><KnowledgeGraphView frameTestId="model-graph" /></>);
    await waitFor(() => expect(mockSigmaConstruct).toHaveBeenCalledOnce());
    const requirementIds = new Set(devFixture.knowledge_graph.nodes!.filter(node => node.element_type === "requirement").map(node => node.id));
    expect(mockForceAtlasEngine.mock.calls[0][0].nodes.some((node: { id: string }) => requirementIds.has(node.id))).toBe(true);
    fireEvent.click(screen.getByRole("button", { name: "Hide requirements" }));
    await waitFor(() => expect(mockForceAtlasEngine).toHaveBeenCalledTimes(2));
    expect(cancel).toHaveBeenCalledOnce();
    expect(mockForceAtlasEngine.mock.calls[1][0].nodes.every((node: { id: string }) => !requirementIds.has(node.id))).toBe(true);
    expect(mockSigmaConstruct).toHaveBeenCalledOnce();
    const canvasContext = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    view.unmount(); canvasContext.mockRestore();
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

  it("retains the graph renderer and layout worker through selection URLs and details", async () => {
    const id = "system-model/Specifications.md#example-requirement";
    window.history.replaceState(null, "", `/${routeForSelection("model", null, { mode: "graph" })}`);
    renderWithStore(<KnowledgeGraphView frameTestId="model-graph" onOpenElement={vi.fn()} />);
    await waitFor(() => expect(mockSigmaConstruct).toHaveBeenCalledOnce());
    const selection = routeForSelection("model", id, { mode: "graph" });
    act(() => writeExplorerHash(selection));
    act(() => writeExplorerHash(routeForElement(id)));
    act(() => writeExplorerHash(selection));
    expect(mockSigmaConstruct).toHaveBeenCalledOnce();
    expect(mockSigmaKill).not.toHaveBeenCalled();
    expect(mockForceAtlasEngine).toHaveBeenCalledOnce();
    expect(mockCameraAnimate).toHaveBeenCalled();
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

  it("uses the canonical Thesaurus projection without reconstructing ontology graph concepts", () => {
    const store = { ...devFixture, ontology: { ...devFixture.ontology, graph_data: { nodes: [], edges: [] } } };
    renderWithStore(<ExplorerSidePane activeView="thesaurus" open onToggle={vi.fn()}
      onNavigate={vi.fn()} onOpenElement={vi.fn()} onOpenOntologyNode={vi.fn()} />, store);
    const tree = screen.getByRole("tree", { name: "Concept hierarchy" });
    expect(within(tree).getByText("Example Thesaurus")).toBeTruthy();
    expect(within(tree).getByText("Service Endpoint")).toBeTruthy();
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

  it("renders the selected trace through the native flow pattern", () => {
    window.history.replaceState(null, "", "/#/traces");
    const { container } = renderWithStore(
      <TracesView onOpenElement={vi.fn()} />,
    );

    expect(screen.getByRole("region", { name: "Verification trace flow" })).toBeTruthy();
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

  it("builds native roll-up topology from normalized trace graphs", () => {
    const flow = buildVerificationFlow(
      {
        id: "system-model/Traces.md#verify-api",
        name: "Verify API",
        file: "system-model/Traces.md",
        directCount: 1,
        totalCount: 2,
        requirementIds: ["system-model/Traces.md#api-response"],
        verificationType: "test-verification",
        traceGraph: {
          nodes: [
            { id: "system-model/Traces.md#api-response", name: "API Response", type: "requirement", is_directly_verified: true },
            { id: "system-model/Traces.md#api-root", name: "API Root", type: "requirement", is_directly_verified: false },
          ],
          edges: [{ source: "system-model/Traces.md#api-response", relation_type: "derivedFrom", target: "system-model/Traces.md#api-root" }],
        },
      },
      new Map(),
    );
    const graph = flow.graph!;

    expect(graph.nodes.map(node => node.element.name)).toEqual(["Verify API", "API Response", "API Root"]);
    expect(graph.edges.map(edge => edge.label)).toEqual(["verifies", "derivedFrom"]);
    expect(flow.requirements).toHaveLength(2);
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
