import { act, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { ExplorerUiStateProvider } from "../state/ExplorerUiState";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import type { ExplorerProjectStore } from "../store/types";
import { ExplorerSidePane } from "../components/ExplorerSidePane";
import * as traceProjection from "../lib/traces";
import { TracesView } from "./ReportViews";

const onOpenElement = vi.fn();
function fixture(label: string, context = "original", parent = false): ExplorerProjectStore {
  const file = "system-model/Specifications.md";
  return {
    ...devFixture,
    project: { ...devFixture.project, worktree_id: context },
    traces: { files: { [file]: { verifications: [{
      identifier: `${file}#verification`, name: "Shared Verification", file,
      directly_verified_count: 1, total_requirements_in_tree: parent ? 2 : 1,
      trace_graph: {
        nodes: [
          { id: `${file}#requirement`, name: label, type: "requirement", is_directly_verified: true },
          ...(parent ? [{ id: `${file}#parent`, name: "Added Parent", type: "requirement", is_directly_verified: false }] : []),
        ],
        edges: parent ? [{ source: `${file}#requirement`, relation_type: "derivedFrom", target: `${file}#parent` }] : [],
      },
    }] } } },
  };
}
function withSecondTrace(store: ExplorerProjectStore): ExplorerProjectStore {
  const file = "system-model/Specifications.md";
  const first = fixture("Second Requirement").traces.files[file].verifications[0];
  return { ...store, traces: { files: { [file]: { verifications: [
    ...store.traces.files[file].verifications,
    { ...first, identifier: `${file}#second-verification`, name: "Second Verification" },
  ] } } } };
}

function view(store: ExplorerProjectStore) {
  return <StoreProvider store={store} schemaMismatch={null}>
    <ExplorerUiStateProvider><TracesView onOpenElement={onOpenElement} /></ExplorerUiStateProvider>
  </StoreProvider>;
}
function renderer() {
  const renderMermaid = vi.fn(async (_id: string, code: string) => ({
    svg: `<svg data-testid="trace-svg"><text>${code.includes("New Requirement") ? "new diagram" : "old diagram"}</text></svg>`,
  }));
  window.mermaid = { render: renderMermaid } as unknown as typeof window.mermaid;
  return renderMermaid;
}
afterEach(() => { vi.unstubAllGlobals(); vi.restoreAllMocks(); });

describe("trace snapshot ownership", () => {
  it("renders each shared ancestor once with all split/merge and capability edges", async () => {
    const renderMermaid = renderer();
    const store = fixture("Leaf");
    const file = "system-model/Specifications.md";
    const id = (name: string) => `${file}#${name}`;
    const trace = store.traces.files[file].verifications[0];
    const topology = [
      ["leaf", "derivedFrom", "left"], ["leaf", "derivedFrom", "right"],
      ["left", "derivedFrom", "root"], ["right", "derivedFrom", "root"],
      ["root", "specify", "capability"], ["capability", "derivedFrom", "capability-root"],
    ];
    trace.directly_verified_count = 2;
    trace.total_requirements_in_tree = 4;
    trace.directly_verified_requirements = [id("leaf"), id("root")];
    trace.trace_graph = {
      nodes: ["leaf", "left", "right", "root", "capability", "capability-root"].map(name => ({
        id: id(name), name, type: name.startsWith("capability") ? "capability" : "requirement",
        is_directly_verified: name === "leaf" || name === "root",
      })),
      edges: topology.map(([source, relation_type, target]) => ({ source: id(source), relation_type, target: id(target) })),
    };
    render(view(store));
    await waitFor(() => expect(renderMermaid).toHaveBeenCalled());
    const code = renderMermaid.mock.calls.at(-1)![1];
    const identifiers = new Map<string, string>();
    for (const name of ["Shared Verification", ...trace.trace_graph.nodes.map(node => node.name)]) {
      const lines = code.split("\n").filter(line => line.includes(`["${name}"]:::`));
      expect(lines).toHaveLength(1);
      identifiers.set(name, lines[0].trim().split("[")[0]);
    }
    const expectedEdges = [
      ...topology, ["Shared Verification", "verifies", "leaf"], ["Shared Verification", "verifies", "root"],
    ].map(([source, relation, target]) => `${identifiers.get(source)} -->|${relation}| ${identifiers.get(target)};`);
    expect(code.split("\n").filter(line => line.includes("-->|" )).map(line => line.trim()).sort()).toEqual(expectedEdges.sort());
    expect(screen.getByText("4 in tree")).toBeTruthy();
    expect(screen.getByText("2 requirements")).toBeTruthy();
  });

  it("shares lazy grouping between the sidebar and report across coverage refreshes", async () => {
    renderer();
    const build = vi.spyOn(traceProjection, "buildTraceFiles");
    const initial = fixture("Old Requirement");
    const scene = (store: ExplorerProjectStore, opened: boolean) =>
      <StoreProvider store={store} schemaMismatch={null}>
        <ExplorerUiStateProvider>
          <ExplorerSidePane activeView={opened ? "traces" : "model"} open onToggle={vi.fn()}
            onNavigate={vi.fn()} onOpenElement={onOpenElement} onOpenOntologyNode={vi.fn()} />
          {opened && <TracesView onOpenElement={onOpenElement} />}
        </ExplorerUiStateProvider>
      </StoreProvider>;
    const { rerender } = render(scene(initial, false));
    expect(build).not.toHaveBeenCalled();
    rerender(scene(initial, true));
    await screen.findByText("old diagram");
    expect(screen.getByLabelText("Verification trace tree")).toBeTruthy();
    expect(build).toHaveBeenCalledTimes(1);
    rerender(scene({ ...initial, coverage: { ...initial.coverage } }, true));
    expect(build).toHaveBeenCalledTimes(1);
    rerender(scene(fixture("New Requirement"), true));
    await screen.findByText("new diagram");
    expect(build).toHaveBeenCalledTimes(2);
  });

  for (const context of ["original", "other-branch"]) {
    it(`updates an existing verification for ${context}`, async () => {
      const renderMermaid = renderer();
      const initial = fixture("Old Requirement");
      const { rerender } = render(view(initial));
      await screen.findByText("old diagram");
      rerender(view(fixture("New Requirement", context, true)));
      await screen.findByText("new diagram");
      expect(screen.queryByText("old diagram")).toBeNull();
      const code = renderMermaid.mock.calls.at(-1)?.[1];
      expect(code).toContain("Added Parent");
      expect(code).toContain("derivedFrom");
      expect(code).not.toContain("Old Requirement");
    });
  }

  it("ignores an old asynchronous render completed after a newer snapshot", async () => {
    const renderMermaid = renderer();
    let finishOld!: (value: { svg: string }) => void;
    renderMermaid.mockImplementationOnce(() => new Promise(resolve => { finishOld = resolve; }));
    const { rerender } = render(view(fixture("Old Requirement")));
    await waitFor(() => expect(renderMermaid).toHaveBeenCalledTimes(1));
    rerender(view(fixture("New Requirement", "other-branch")));
    await screen.findByText("new diagram");
    await act(async () => { finishOld({ svg: '<svg><text>obsolete delayed diagram</text></svg>' }); });
    expect(screen.queryByText("obsolete delayed diagram")).toBeNull();
    expect(screen.getByText("new diagram")).toBeTruthy();
  });

  it("an obsolete render cannot release the replacement's active queue slot", async () => {
    const renderMermaid = renderer();
    let finishOld!: (value: { svg: string }) => void;
    let finishNew!: (value: { svg: string }) => void;
    renderMermaid.mockImplementationOnce(() => new Promise(resolve => { finishOld = resolve; }));
    renderMermaid.mockImplementationOnce(() => new Promise(resolve => { finishNew = resolve; }));
    const { rerender } = render(view(withSecondTrace(fixture("Old Requirement"))));
    await waitFor(() => expect(renderMermaid).toHaveBeenCalledTimes(1));
    rerender(view(withSecondTrace(fixture("New Requirement"))));
    await waitFor(() => expect(renderMermaid).toHaveBeenCalledTimes(2));
    expect(renderMermaid.mock.calls[1][1]).toContain("New Requirement");
    await act(async () => {
      finishOld({ svg: '<svg><text>obsolete delayed diagram</text></svg>' });
      await new Promise(resolve => setTimeout(resolve, 50));
    });
    expect(renderMermaid).toHaveBeenCalledTimes(2);
    await act(async () => { finishNew({ svg: '<svg><text>new diagram</text></svg>' }); });
    await waitFor(() => expect(renderMermaid).toHaveBeenCalledTimes(3));
    expect(renderMermaid.mock.calls[2][1]).toContain("Second Verification");
    expect(screen.queryByText("obsolete delayed diagram")).toBeNull();
  });

  it("only prepares the latest input when an offscreen trace becomes visible", async () => {
    const callbacks: IntersectionObserverCallback[] = [];
    vi.stubGlobal("IntersectionObserver", class {
      constructor(callback: IntersectionObserverCallback) { callbacks.push(callback); }
      observe() {}
      disconnect() {}
    });
    const renderMermaid = renderer();
    const { rerender } = render(view(fixture("Old Requirement")));
    rerender(view(fixture("New Requirement")));
    expect(renderMermaid).not.toHaveBeenCalled();
    act(() => callbacks.at(-1)!([{ isIntersecting: true } as IntersectionObserverEntry], {} as IntersectionObserver));
    await screen.findByText("new diagram");
    expect(renderMermaid).toHaveBeenCalledTimes(1);
    expect(renderMermaid.mock.calls[0][1]).not.toContain("Old Requirement");
  });
});
