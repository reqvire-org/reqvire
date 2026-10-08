import { act, cleanup, renderHook } from "@testing-library/react";
import type { ReactNode } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { StoreProvider } from "../store/StoreContext";
import { devFixture } from "../store/devFixture";
import type { ExplorerProjectStore } from "../store/types";
import { createProjectSearchIndex, searchProjectDocuments, type ProjectSearchDocument, type ProjectSearchIndex } from "../lib/searchIndex";
import { SearchIndexProvider, useSearchIndex } from "./SearchIndexContext";
import { SEARCH_KINDS, type SearchKind } from "./searchKinds";

type Request = { type: "build"; documents: ProjectSearchDocument[] } |
  { type: "search"; requestId: number; query: string; enabledKinds: SearchKind[]; enabledElementTypes: string[] };

// Replace the browser transport only; builds and searches use the production ranked index.
class SearchWorker {
  static instances: SearchWorker[] = [];
  onmessage: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: ErrorEvent) => void) | null = null;
  onmessageerror: (() => void) | null = null;
  index: ProjectSearchIndex | null = null;
  holdSearches = false;
  terminate = vi.fn();
  constructor() { SearchWorker.instances.push(this); }
  postMessage = vi.fn((request: Request) => {
    if (request.type === "build") {
      this.index = createProjectSearchIndex(request.documents);
      this.emit({ type: "ready", documentCount: request.documents.length });
    } else if (!this.holdSearches) {
      this.emit({ type: "results", requestId: request.requestId, results: searchProjectDocuments(
        this.index!, request.query, new Set(request.enabledKinds), new Set(request.enabledElementTypes),
      ) });
    }
  });
  emit(data: unknown) { this.onmessage?.({ data } as MessageEvent); }
}

function fixture(): ExplorerProjectStore {
  return { ...devFixture,
    project: { ...devFixture.project, worktree_id: "original" },
    elements: [{ ...devFixture.elements[0], id: "model/A.md#alpha", name: "Alpha needle", file_path: "model/A.md", element_type: "requirement" }],
    resources: [{ ...devFixture.resources[0], id: "evidence", target: "evidence.txt", file_path: "evidence.txt" }],
    search: [
      { id: "model/A.md#alpha", kind: "element", title: "Alpha needle", route: "#/elements/model/A.md#alpha", text: "Original content" },
      { id: "evidence.txt", kind: "file", title: "Evidence needle", route: "#/content/evidence.txt", text: "Artifact content" },
    ],
  };
}

function mount(initial = fixture(), strict = false) {
  let store = initial;
  const hook = renderHook(useSearchIndex, { reactStrictMode: strict, wrapper: ({ children }: { children: ReactNode }) => {
    return <StoreProvider store={store} schemaMismatch={null}>
      <SearchIndexProvider>{children}</SearchIndexProvider>
    </StoreProvider>;
  } });
  return { ...hook, update(next: ExplorerProjectStore) { store = next; hook.rerender(); } };
}
const allKinds = new Set<SearchKind>(SEARCH_KINDS);
async function build() { await act(async () => { await vi.runAllTimersAsync(); }); }
beforeEach(() => { vi.useFakeTimers(); SearchWorker.instances = []; vi.stubGlobal("Worker", SearchWorker); });
afterEach(() => { cleanup(); vi.useRealTimers(); vi.unstubAllGlobals(); vi.restoreAllMocks(); });

describe("search index input and worker ownership", () => {
  it("retains ready workers and pending searches across unrelated and equivalent replacements", async () => {
    const initial = fixture();
    const { result, update } = mount(initial);
    await build();
    const [worker] = SearchWorker.instances;
    const search = result.current.search;
    worker.holdSearches = true;
    const pending = result.current.search("needle", allKinds, new Set()).catch(error => error);
    const request = worker.postMessage.mock.calls.at(-1)![0];
    if (request.type !== "search") throw new Error("Expected search request");
    update({ ...initial, coverage: { ...initial.coverage } });
    update({ ...initial,
      elements: initial.elements.map(element => ({ ...element, metadata: { ignored: "changed" } })),
      resources: initial.resources.map(resource => ({ ...resource, referring_element_ids: ["another"] })),
      search: initial.search.map(document => ({ ...document })),
    });
    expect(SearchWorker.instances).toHaveLength(1);
    expect(worker.terminate).not.toHaveBeenCalled();
    expect(worker.postMessage.mock.calls.filter(([message]) => message.type === "build")).toHaveLength(1);
    expect(result.current.status).toBe("ready");
    expect(result.current.search).toBe(search);
    act(() => worker.emit({ type: "results", requestId: request.requestId, results: initial.search }));
    expect(await pending).toEqual(initial.search);
  });

  it.each(["workspace_root", "worktree_id"] as const)("rebuilds for a changed %s even with identical input objects", async key => {
    const initial = fixture();
    const { result, update } = mount(initial);
    await build();
    const [worker] = SearchWorker.instances;
    update({ ...initial, project: { ...initial.project, [key]: "another-context" } });
    expect(worker.terminate).toHaveBeenCalledOnce();
    expect(SearchWorker.instances).toHaveLength(2);
    expect(result.current.status).toBe("building");
    await build();
    expect(await result.current.search("needle", allKinds, new Set())).toEqual(initial.search);
  });

  it("ignores late replies from replaced workers and rejects their outstanding requests", async () => {
    const initial = fixture();
    const { result, update } = mount(initial);
    await build();
    const [old] = SearchWorker.instances;
    const reply = old.onmessage!;
    const error = old.onerror!;
    old.holdSearches = true;
    const pending = result.current.search("needle", allKinds, new Set()).catch(value => value);
    update({ ...initial, search: [{ ...initial.search[0], title: "Replacement needle" }] });
    expect(await pending).toBeInstanceOf(Error);
    await build();
    act(() => {
      reply({ data: { type: "ready", documentCount: 999 } } as MessageEvent);
      reply({ data: { type: "error", message: "Obsolete error" } } as MessageEvent);
      error(new ErrorEvent("error", { message: "Obsolete load error" }));
    });
    expect(result.current.status).toBe("ready");
    expect(result.current.error).toBeNull();
    expect(result.current.documentCount).toBe(1);
    expect((await result.current.search("replacement", allKinds, new Set()))[0].title).toBe("Replacement needle");

    const current = SearchWorker.instances.at(-1)!;
    current.holdSearches = true;
    const fresh = result.current.search("replacement", allKinds, new Set());
    const settled = vi.fn();
    void fresh.then(settled);
    const request = current.postMessage.mock.calls.at(-1)![0];
    if (request.type !== "search") throw new Error("Expected search request");
    // Even a queued old reply carrying the current request id belongs to the old worker.
    act(() => reply({ data: { type: "results", requestId: request.requestId, results: initial.search } } as MessageEvent));
    await act(async () => { await Promise.resolve(); });
    expect(settled).not.toHaveBeenCalled();
    act(() => current.emit({ type: "results", requestId: request.requestId, results: [] }));
    expect(await fresh).toEqual([]);
  });

  it.each([
    ["id", "model/B.md#replacement"],
    ["kind", "ontology"],
    ["title", "Replacement title"],
    ["route", "#/elements/replacement"],
    ["text", "Replacement content"],
  ])("rebuilds for changed searchable %s and returns the new document", async (key, value) => {
    const initial = fixture();
    const { result, update } = mount(initial);
    await build();
    const search = [{ ...initial.search[0], [key]: value }, initial.search[1]];
    update({ ...initial, search });
    expect(SearchWorker.instances).toHaveLength(2);
    await build();
    expect(await result.current.search("", allKinds, new Set())).toEqual(search);
    expect(await result.current.search(key === "kind" ? "ontology" : "replacement", allKinds, new Set())).toEqual([search[0]]);
  });

  it.each(["reorder", "add", "remove"])("rebuilds for document membership/order: %s", async change => {
    const initial = fixture();
    const { result, update } = mount(initial);
    await build();
    const search = change === "reorder" ? [...initial.search].reverse() : change === "remove"
      ? initial.search.slice(1) : [...initial.search, { ...initial.search[0], title: "Another entry for the same id" }];
    update({ ...initial, search });
    await build();
    expect(result.current.documentCount).toBe(search.length);
    expect(await result.current.search("", allKinds, new Set())).toEqual(search);
  });

  it.each(["resource target", "resource file alias", "modeled file"])("updates file/resource filtering when %s changes", async change => {
    const initial = fixture();
    initial.resources[0] = { ...initial.resources[0],
      target: change === "resource file alias" ? "alias" : "evidence.txt",
      file_path: change === "resource target" ? "alias" : "evidence.txt",
    };
    const { result, update } = mount(initial);
    await build();
    expect(await result.current.search("", new Set(["resource"]), new Set())).toEqual([initial.search[1]]);
    const next = change === "modeled file"
      ? { ...initial, elements: [{ ...initial.elements[0], file_path: "evidence.txt" }] }
      : { ...initial, resources: [{ ...initial.resources[0],
        [change === "resource target" ? "target" : "file_path"]: "another.txt",
      }] };
    update(next);
    await build();
    expect(await result.current.search("", new Set(["resource"]), new Set())).toEqual([]);
    expect(await result.current.search("", new Set(["file"]), new Set())).toEqual([initial.search[1]]);
  });

  it("rebuilds element-type filtering and reuses it when only the enabled filters change", async () => {
    const initial = fixture();
    const { result, update } = mount(initial);
    await build();
    expect(await result.current.search("", allKinds, new Set(["requirement"]))).toEqual(initial.search);
    update({ ...initial, elements: [{ ...initial.elements[0], element_type: "capability" }] });
    await build();
    expect(await result.current.search("", allKinds, new Set(["requirement"]))).toEqual([initial.search[1]]);
    expect(await result.current.search("", allKinds, new Set(["capability"]))).toEqual(initial.search);
    expect(SearchWorker.instances).toHaveLength(2);
  });

  it("cancels scheduled builds on replacement and unmount", async () => {
    const initial = fixture();
    const { update, unmount } = mount(initial);
    update({ ...initial, search: [] });
    expect(SearchWorker.instances).toHaveLength(2);
    unmount();
    await build();
    for (const worker of SearchWorker.instances) {
      expect(worker.terminate).toHaveBeenCalledOnce();
      expect(worker.postMessage).not.toHaveBeenCalled();
    }
  });

  it("keeps one queued build through equivalent replacements before it starts", async () => {
    const initial = fixture();
    const { result, update } = mount(initial);
    update({ ...initial, search: initial.search.map(document => ({ ...document })) });
    expect(SearchWorker.instances).toHaveLength(1);
    await build();
    expect(SearchWorker.instances[0].postMessage).toHaveBeenCalledOnce();
    expect(result.current.status).toBe("ready");
    expect(await result.current.search("needle", allKinds, new Set())).toEqual(initial.search);
  });

  it.each(["error", "messageerror", "response"])("rejects pending work on worker %s and recovers with new inputs", async failure => {
    const initial = fixture();
    const { result, update } = mount(initial);
    await build();
    const [worker] = SearchWorker.instances;
    worker.holdSearches = true;
    const pending = result.current.search("needle", allKinds, new Set()).catch(error => error);
    act(() => {
      if (failure === "error") worker.onerror!(new ErrorEvent("error", { message: "Worker failed" }));
      else if (failure === "messageerror") worker.onmessageerror!();
      else worker.emit({ type: "error", message: "Index failed" });
    });
    expect(await pending).toBeInstanceOf(Error);
    expect(result.current.status).toBe("error");
    expect(result.current.error).toBeTruthy();
    expect(worker.terminate).toHaveBeenCalledOnce();
    update({ ...initial, search: [...initial.search] });
    await build();
    expect(result.current.status).toBe("ready");
    expect(result.current.error).toBeNull();
    expect(await result.current.search("needle", allKinds, new Set())).toEqual(initial.search);
  });

  it.each(["construction", "build message"])("exposes worker %s failures without leaking pending builds", async failure => {
    if (failure === "construction") vi.stubGlobal("Worker", class {
      constructor() { throw new Error("Worker failed"); }
    });
    const { result, unmount } = mount();
    if (failure === "build message") SearchWorker.instances[0].postMessage.mockImplementationOnce(() => {
      throw new Error("Worker failed");
    });
    await build();
    expect(result.current.status).toBe("error");
    expect(result.current.error).toBe("Worker failed");
    unmount();
    for (const worker of SearchWorker.instances) expect(worker.terminate).toHaveBeenCalledOnce();
  });

  it.each(["post", "reply"])("rejects only the affected search for a request %s error", async failure => {
    const initial = fixture();
    const { result } = mount(initial);
    await build();
    const [worker] = SearchWorker.instances;
    worker.holdSearches = true;
    if (failure === "post") worker.postMessage.mockImplementationOnce(() => { throw new Error("Query failed"); });
    const pending = result.current.search("needle", allKinds, new Set()).catch(error => error);
    if (failure === "reply") {
      const request = worker.postMessage.mock.calls.at(-1)![0];
      if (request.type !== "search") throw new Error("Expected search request");
      act(() => worker.emit({ type: "error", requestId: request.requestId, message: "Query failed" }));
    }
    expect((await pending).message).toBe("Query failed");
    expect(result.current.status).toBe("ready");
    expect(worker.terminate).not.toHaveBeenCalled();
    worker.holdSearches = false;
    expect(await result.current.search("needle", allKinds, new Set())).toEqual(initial.search);
  });

  it("survives StrictMode replay and rejects pending searches on unmount", async () => {
    const { result, unmount } = mount(fixture(), true);
    await build();
    const [retired, current] = SearchWorker.instances;
    expect(SearchWorker.instances).toHaveLength(2);
    expect(retired.terminate).toHaveBeenCalledOnce();
    expect(retired.postMessage).not.toHaveBeenCalled();
    expect(current.postMessage).toHaveBeenCalledOnce();
    expect(result.current.status).toBe("ready");
    current.holdSearches = true;
    const pending = result.current.search("needle", allKinds, new Set()).catch(error => error);
    unmount();
    expect(await pending).toBeInstanceOf(Error);
    expect(current.terminate).toHaveBeenCalledOnce();
  });
});
