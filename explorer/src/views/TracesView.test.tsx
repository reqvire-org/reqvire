import { act, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { ExplorerUiStateProvider } from "../state/ExplorerUiState";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import type { ExplorerProjectStore } from "../store/types";
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
      trace_tree: { requirements: [{
        id: `${file}#requirement`, name: label, type: "requirement", is_directly_verified: true,
        children: parent ? [{ id: `${file}#parent`, name: "Added Parent", type: "requirement", is_directly_verified: false, children: [] }] : [],
      }] },
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
afterEach(() => { vi.unstubAllGlobals(); });

describe("trace snapshot ownership", () => {
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
