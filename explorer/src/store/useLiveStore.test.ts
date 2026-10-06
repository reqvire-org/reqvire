import { act, cleanup, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useLiveStore } from "./useLiveStore";
import { ManifestStoreClient } from "./manifestRefresh";
import { chunkResponse, manifestResponse, smallStore, wireSnapshot } from "../test/liveStoreFixtures";

beforeEach(() => {
  vi.useFakeTimers();
  vi.spyOn(document, "visibilityState", "get").mockReturnValue("visible");
});
afterEach(() => {
  cleanup();
  vi.useRealTimers();
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
  delete window.reqvireProjectStore;
  delete window.reqvireLiveRefresh;
});

async function tick(ms = 0) {
  await act(async () => { await vi.advanceTimersByTimeAsync(ms); });
}

function backend() {
  const seed = wireSnapshot(smallStore());
  let current = seed;
  window.reqvireProjectStore = seed.store;
  window.reqvireLiveRefresh = seed.seed;
  const transport = async (url: string, init: RequestInit = {}) => {
    if (url.endsWith("/manifest")) {
      return new Headers(init.headers).get("If-None-Match") === `"${current.revision}"`
        ? new Response(null, { status: 304, headers: { ETag: `"${current.revision}"` } })
        : manifestResponse(current);
    }
    return chunkResponse(current, init);
  };
  const fetchMock = vi.fn(transport);
  vi.stubGlobal("fetch", fetchMock);
  return { seed, fetchMock, transport, update: (marker: string) => {
    current = wireSnapshot({ ...smallStore(), elements: [{ ...smallStore().elements[0], content: marker }] });
    return current;
  } };
}

describe("visible Explorer manifest polling", () => {
  it("checks immediately, updates at five seconds, and publishes seed and revision together", async () => {
    const { seed, fetchMock, update } = backend();
    const hook = renderHook(() => useLiveStore());
    await tick();
    const latest = update("Fresh record.");
    await tick(4999);
    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(hook.result.current.result.ok && hook.result.current.result.store).toBe(seed.store);
    await tick(1);
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(latest.store);
    expect(window.reqvireProjectStore).toEqual(latest.store);
    expect(window.reqvireLiveRefresh).toEqual(latest.seed);
    expect(hook.result.current.refreshError).toBeNull();
  });

  it("keeps the displayed snapshot after failure and retries its old revision", async () => {
    const { seed, fetchMock, transport, update } = backend();
    const hook = renderHook(() => useLiveStore());
    await tick();
    const latest = update("Recovered.");
    fetchMock.mockImplementation(async (url, init) => url.endsWith("/chunks")
      ? Response.json({ error: "Unavailable" }, { status: 503 }) : transport(url, init));
    await tick(5000);
    expect(hook.result.current.refreshError).toBe("Unavailable");
    expect(window.reqvireProjectStore).toBe(seed.store);
    expect(window.reqvireLiveRefresh?.revision).toBe(seed.revision);
    fetchMock.mockImplementation(transport);
    await tick(5000);
    expect(window.reqvireProjectStore).toEqual(latest.store);
    expect(hook.result.current.refreshError).toBeNull();
  });

  it("does not overlap periodic checks while a manifest download is pending", async () => {
    const { fetchMock, transport, update } = backend();
    renderHook(() => useLiveStore());
    await tick();
    const latest = update("After pending request.");
    let release!: (response: Response) => void;
    fetchMock.mockImplementationOnce(() => new Promise(resolve => { release = resolve; }));
    await tick(5000);
    await tick(5000);
    expect(fetchMock).toHaveBeenCalledTimes(2);
    fetchMock.mockImplementation(transport);
    await act(async () => { release(manifestResponse(latest)); });
    await tick();
    expect(window.reqvireProjectStore).toEqual(latest.store);
  });

  it("retains the valid snapshot after timeout and releases the request for automatic retry", async () => {
    const { seed, fetchMock, update } = backend();
    const timeout = new AbortController();
    const timeoutSpy = vi.spyOn(AbortSignal, "timeout").mockReturnValueOnce(timeout.signal);
    fetchMock.mockImplementationOnce((_url, init) => new Promise((_resolve, reject) => {
      init!.signal!.addEventListener("abort", () => reject(init!.signal!.reason), { once: true });
    }));
    const hook = renderHook(() => useLiveStore());
    await tick();
    expect(timeoutSpy).toHaveBeenCalledWith(15000);
    await act(async () => {
      timeout.abort(Object.assign(new Error("Timed out"), { name: "TimeoutError" }));
    });
    expect(hook.result.current.refreshError).toBe("The server did not respond within 15 seconds.");
    expect(window.reqvireProjectStore).toBe(seed.store);
    expect(window.reqvireLiveRefresh?.revision).toBe(seed.revision);
    const latest = update("After the timeout.");
    await tick(5000);
    expect(window.reqvireProjectStore).toEqual(latest.store);
    expect(hook.result.current.refreshError).toBeNull();
  });

  it("aborts when hidden, resumes immediately, and discards the late previous response", async () => {
    const { fetchMock, transport, update } = backend();
    renderHook(() => useLiveStore());
    await tick();
    const before = update("Before hiding.");
    let release!: (response: Response) => void;
    let pendingSignal!: AbortSignal;
    fetchMock.mockImplementationOnce((_url, init) => {
      pendingSignal = init!.signal!;
      return new Promise(resolve => { release = resolve; });
    });
    await tick(5000);
    vi.spyOn(document, "visibilityState", "get").mockReturnValue("hidden");
    act(() => document.dispatchEvent(new Event("visibilitychange")));
    expect(pendingSignal.aborted).toBe(true);
    await tick(5000);
    expect(fetchMock).toHaveBeenCalledTimes(2);
    const latest = update("After returning.");
    fetchMock.mockImplementation(transport);
    vi.spyOn(document, "visibilityState", "get").mockReturnValue("visible");
    act(() => document.dispatchEvent(new Event("visibilitychange")));
    await tick();
    expect(window.reqvireProjectStore).toEqual(latest.store);
    await act(async () => { release(manifestResponse(before)); });
    await tick();
    expect(window.reqvireProjectStore).toEqual(latest.store);
    expect(window.reqvireLiveRefresh?.revision).toBe(latest.revision);
  });

  it("cancels pending work on unmount without changing the injected store", async () => {
    const { seed, fetchMock, update } = backend();
    let release!: (response: Response) => void;
    let pendingSignal!: AbortSignal;
    const latest = update("Unmounted.");
    fetchMock.mockImplementationOnce((_url, init) => {
      pendingSignal = init!.signal!;
      return new Promise(resolve => { release = resolve; });
    });
    const hook = renderHook(() => useLiveStore());
    hook.unmount();
    expect(pendingSignal.aborted).toBe(true);
    await act(async () => { release(manifestResponse(latest)); });
    await tick(30000);
    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(window.reqvireProjectStore).toBe(seed.store);
  });

  it("survives StrictMode effect cleanup without committing the abandoned request", async () => {
    const { fetchMock, update } = backend();
    renderHook(() => useLiveStore(), { reactStrictMode: true });
    await tick();
    expect(fetchMock.mock.calls[0][1]?.signal?.aborted).toBe(true);
    const latest = update("StrictMode update.");
    await tick(5000);
    expect(window.reqvireProjectStore).toEqual(latest.store);
  });

  it("does not poll when the server has not advertised mutation-enabled live refresh", async () => {
    const { fetchMock } = backend();
    delete window.reqvireLiveRefresh;
    renderHook(() => useLiveStore());
    await tick(30000);
    expect(fetchMock).not.toHaveBeenCalled();
  });
});

describe("worktree selection", () => {
  afterEach(() => {
    delete window.reqvireWorktreeRouting;
    window.history.replaceState(null, "", "/");
  });
  function contexts() {
    const store = smallStore();
    const a = wireSnapshot({ ...store, project: { ...store.project, worktree_id: "a", branch: "main" } });
    const b = wireSnapshot({ ...store, project: { ...store.project, worktree_id: "b", branch: "feature" }, elements: [{ ...store.elements[0], content: "Branch B" }] });
    window.reqvireWorktreeRouting = true;
    window.reqvireProjectStore = a.store;
    window.reqvireLiveRefresh = a.seed;
    window.history.replaceState(null, "", "/?worktree_id=a#/model");
    const transport = async (url: string, init: RequestInit = {}) => {
      const parsed = new URL(url, window.location.origin);
      if (parsed.pathname === "/api/worktrees/load") return Response.json({});
      if (parsed.pathname === "/api/worktrees") return Response.json({ original_worktree_id: "a", worktrees: [
        { worktree_id: "a", branch: "main", workspace_root: "/a", available: true, explorer_available: true },
        { worktree_id: "b", branch: "feature", workspace_root: "/b", available: true, explorer_available: true },
      ] });
      const id = parsed.searchParams.get("worktree_id");
      if (id !== "a" && id !== "b") return Response.json({ error: "Unknown worktree" }, { status: 503 });
      const snapshot = id === "a" ? a : b;
      return parsed.pathname.endsWith("/manifest")
        ? new Headers(init.headers).get("If-None-Match") === `"${snapshot.revision}"`
          ? new Response(null, { status: 304, headers: { ETag: `"${snapshot.revision}"` } }) : manifestResponse(snapshot)
        : chunkResponse(snapshot, init);
    };
    const fetchMock = vi.fn(transport);
    vi.stubGlobal("fetch", fetchMock);
    return { a, b, fetchMock, transport };
  }
  it("keeps recovery warnings with the displayed context across failed and successful switches", async () => {
    const { a, b, fetchMock, transport } = contexts();
    fetchMock.mockImplementation(async (url, init) => {
      const response = await transport(url, init);
      if (url.includes('/manifest') && url.includes('worktree_id=a')) response.headers.set('X-Reqvire-Recovery-Required', 'true');
      return response;
    });
    const hook = renderHook(() => useLiveStore());
    await tick();
    expect(hook.result.current.recoveryWarning).toMatch(/recovery/i);
    fetchMock.mockImplementation(async (url, init) => url.includes('worktree_id=b')
      ? Response.json({ error: 'unavailable' }, { status: 503 }) : transport(url, init));
    act(() => hook.result.current.selectWorktree('b'));
    await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(a.store);
    expect(hook.result.current.recoveryWarning).toMatch(/recovery/i);
    fetchMock.mockImplementation(transport);
    act(() => hook.result.current.selectWorktree('b'));
    await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(b.store);
    expect(hook.result.current.recoveryWarning).toBeNull();
  });
  it("loads read-only branch selections without enabling periodic refresh", async () => {
    const { a, b, fetchMock } = contexts();
    delete window.reqvireLiveRefresh;
    const hook = renderHook(() => useLiveStore());
    await tick();
    expect(hook.result.current.worktrees).toHaveLength(2);
    fetchMock.mockClear();
    await tick(30000);
    expect(fetchMock).not.toHaveBeenCalled();
    act(() => hook.result.current.selectWorktree("b"));
    await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(b.store);
    expect(window.reqvireLiveRefresh).toBeUndefined();
    fetchMock.mockClear();
    await tick(30000);
    expect(fetchMock).not.toHaveBeenCalled();
    act(() => {
      window.history.replaceState(null, "", "/?worktree_id=a#/model");
      window.dispatchEvent(new PopStateEvent("popstate"));
    });
    await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(a.store);
  });
  it("retries the same read-only selection after a failed download", async () => {
    const { a, b, fetchMock, transport } = contexts();
    delete window.reqvireLiveRefresh;
    const hook = renderHook(() => useLiveStore());
    await tick();
    fetchMock.mockImplementation(async (url, init) => url.includes("worktree_id=b")
      ? Response.json({ error: "Temporarily unavailable" }, { status: 503 }) : transport(url, init));
    act(() => hook.result.current.selectWorktree("b"));
    await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(a.store);
    expect(hook.result.current.refreshError).toContain("Temporarily unavailable");
    fetchMock.mockImplementation(transport);
    act(() => hook.result.current.selectWorktree("b"));
    await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(b.store);
    expect(hook.result.current.refreshError).toBeNull();
    expect(hook.result.current.automaticRefresh).toBe(false);
  });
  it("loads only the selected context and retains the previous branch when admission fails", async () => {
    const { a, b, fetchMock, transport } = contexts();
    const hook = renderHook(() => useLiveStore()); await tick();
    fetchMock.mockClear();
    await tick(10000);
    expect(fetchMock.mock.calls.some(([url]) => url === "/api/worktrees/load")).toBe(false);
    fetchMock.mockImplementation(async (url, init) => url === "/api/worktrees/load"
      ? Response.json({ error: "Mutation-enabled MCP requires a clean worktree" }, { status: 503 }) : transport(url, init));
    fetchMock.mockClear();
    act(() => hook.result.current.selectWorktree("b")); await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(a.store);
    expect(hook.result.current.refreshError).toContain("clean worktree");
    expect(fetchMock.mock.calls.filter(([url]) => url === "/api/worktrees/load")).toHaveLength(1);
    expect(fetchMock.mock.calls.some(([url]) => url.includes("worktree_id=b"))).toBe(false);
    fetchMock.mockImplementation(transport);
    act(() => hook.result.current.selectWorktree("b")); await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(b.store);
  });
  it("adopts branch, model, manifest and routing together and ignores late former-context responses", async () => {
    const { a, b, fetchMock, transport } = contexts();
    const hook = renderHook(() => useLiveStore());
    await tick();
    let release!: (response: Response) => void;
    fetchMock.mockImplementationOnce(() => new Promise(resolve => { release = resolve; }));
    await tick(5000);
    act(() => hook.result.current.selectWorktree("b"));
    expect(hook.result.current.result.ok && hook.result.current.result.store.project.branch).toBe("main");
    fetchMock.mockImplementation(transport);
    await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(b.store);
    expect(window.location.search).toBe("?worktree_id=b");
    const late = manifestResponse(a);
    late.headers.set('X-Reqvire-Recovery-Required', 'true');
    await act(async () => { release(late); });
    await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(b.store);
    expect(hook.result.current.recoveryWarning).toBeNull();
    expect(fetchMock.mock.calls.filter(([url]) => url.includes("/chunks")).every(([url]) => url.includes("worktree_id="))).toBe(true);
  });
  it("retains the labelled valid branch when a selected ID disappears, and recovers explicitly", async () => {
    const { a, b } = contexts();
    const hook = renderHook(() => useLiveStore()); await tick();
    act(() => hook.result.current.selectWorktree("removed")); await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(a.store);
    expect(hook.result.current.refreshError).toContain("Unknown worktree");
    act(() => hook.result.current.selectWorktree("b")); await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(b.store);
  });
  it("follows popstate without changing other contexts or shared storage", async () => {
    const { a, b } = contexts();
    const storage = vi.spyOn(Storage.prototype, "setItem");
    const hook = renderHook(() => useLiveStore()); await tick();
    act(() => hook.result.current.selectWorktree("b")); await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(b.store);
    act(() => { window.history.replaceState(null, "", "/?worktree_id=a#/model"); window.dispatchEvent(new PopStateEvent("popstate")); });
    await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(a.store);
    expect(storage).not.toHaveBeenCalled();
  });

  it.each([false, true])("releases removed contexts after switching away (live refresh: %s)", async live => {
    const { a, b, fetchMock, transport } = contexts();
    if (!live) delete window.reqvireLiveRefresh;
    const prepare = vi.spyOn(ManifestStoreClient.prototype, "prepare");
    let listed = ["a", "b"];
    fetchMock.mockImplementation(async (url, init) => url === "/api/worktrees"
      ? Response.json({ original_worktree_id: "a", worktrees: listed.map(worktree_id => ({ worktree_id, branch: worktree_id })) })
      : transport(url, init));
    const hook = renderHook(() => useLiveStore()); await tick();
    const initialClient = prepare.mock.contexts.at(-1);
    act(() => hook.result.current.selectWorktree("b")); await tick();
    const cachedB = prepare.mock.contexts.at(-1);
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(b.store);

    listed = ["b"];
    await act(async () => { await hook.result.current.refreshWorktrees(); });
    fetchMock.mockClear();
    act(() => hook.result.current.selectWorktree("a")); await tick();
    expect(prepare.mock.contexts.at(-1)).not.toBe(initialClient);
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(a.store);
    const downloaded = fetchMock.mock.calls.filter(([url]) => url.includes("/chunks"))
      .flatMap(([, init]) => JSON.parse(String(init?.body)).hashes);
    expect(new Set(downloaded)).toEqual(new Set(a.chunks.keys()));
    act(() => hook.result.current.selectWorktree("b")); await tick();
    expect(prepare.mock.contexts.at(-1)).toBe(cachedB);
  });

  it("keeps an absent displayed client through a failed switch, then releases it after success", async () => {
    const { a, b, fetchMock, transport } = contexts();
    const prepare = vi.spyOn(ManifestStoreClient.prototype, "prepare");
    const hook = renderHook(() => useLiveStore()); await tick();
    const initialClient = prepare.mock.contexts.at(-1);
    let rejectLoad = true;
    fetchMock.mockImplementation(async (url, init) => {
      if (url === "/api/worktrees") return Response.json({ original_worktree_id: "a", worktrees: [{ worktree_id: "b", branch: "feature" }] });
      if (url === "/api/worktrees/load" && rejectLoad) return Response.json({ error: "Unavailable" }, { status: 503 });
      return transport(url, init);
    });
    act(() => hook.result.current.selectWorktree("b")); await tick();
    expect(hook.result.current.refreshError).toBe("Unavailable");
    expect(hook.result.current.result.ok && hook.result.current.result.store).toBe(a.store);
    rejectLoad = false;
    act(() => hook.result.current.selectWorktree("a")); await tick();
    expect(prepare.mock.contexts.at(-1)).toBe(initialClient);
    act(() => hook.result.current.selectWorktree("b")); await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(b.store);
    act(() => hook.result.current.selectWorktree("a")); await tick();
    expect(prepare.mock.contexts.at(-1)).not.toBe(initialClient);
  });

  it.each(["http error", "error payload", "malformed", "missing identity", "duplicate identity"])("keeps inactive caches after inventory %s", async failure => {
    const { fetchMock, transport } = contexts();
    const prepare = vi.spyOn(ManifestStoreClient.prototype, "prepare");
    const hook = renderHook(() => useLiveStore()); await tick();
    const originalChoices = hook.result.current.worktrees;
    const initialClient = prepare.mock.contexts.at(-1);
    act(() => hook.result.current.selectWorktree("b")); await tick();
    fetchMock.mockImplementation(async (url, init) => {
      if (url !== "/api/worktrees") return transport(url, init);
      if (failure === "http error") return Response.json({ error: "Git failed", worktrees: [] }, { status: 503 });
      if (failure === "error payload") return Response.json({ error: "Git failed", worktrees: [] });
      if (failure === "missing identity") return Response.json({ worktrees: [] });
      if (failure === "duplicate identity") return Response.json({ original_worktree_id: "a", worktrees: [
        { worktree_id: "b", branch: "first" }, { worktree_id: "b", branch: "second" },
      ] });
      return Response.json({ original_worktree_id: "a", worktrees: [{ branch: "broken" }] });
    });
    await act(async () => { await hook.result.current.refreshWorktrees(); });
    expect(hook.result.current.refreshError).toBeTruthy();
    expect(hook.result.current.worktrees).toBe(originalChoices);
    act(() => hook.result.current.selectWorktree("a")); await tick();
    expect(prepare.mock.contexts.at(-1)).toBe(initialClient);
  });

  it("keeps clients created after a delayed inventory began until a newer inventory confirms removal", async () => {
    const { fetchMock, transport } = contexts();
    const prepare = vi.spyOn(ManifestStoreClient.prototype, "prepare");
    const hook = renderHook(() => useLiveStore()); await tick();
    let release!: (response: Response) => void;
    fetchMock.mockImplementationOnce(() => new Promise(resolve => { release = resolve; }));
    let pending!: Promise<void>;
    act(() => { pending = hook.result.current.refreshWorktrees(); });
    act(() => hook.result.current.selectWorktree("b")); await tick();
    const cachedB = prepare.mock.contexts.at(-1);
    act(() => hook.result.current.selectWorktree("a")); await tick();
    const onlyA = () => Response.json({ original_worktree_id: "a", worktrees: [{ worktree_id: "a", branch: "main" }] });
    await act(async () => { release(onlyA()); await pending; });
    act(() => hook.result.current.selectWorktree("b")); await tick();
    expect(prepare.mock.contexts.at(-1)).toBe(cachedB);

    act(() => hook.result.current.selectWorktree("a")); await tick();
    fetchMock.mockImplementation(async (url, init) => url === "/api/worktrees" ? onlyA() : transport(url, init));
    await act(async () => { await hook.result.current.refreshWorktrees(); });
    act(() => hook.result.current.selectWorktree("b")); await tick();
    expect(prepare.mock.contexts.at(-1)).not.toBe(cachedB);
  });

  it.each(["superseded", "timed out"])("discards a late %s inventory without pruning", async failure => {
    const { fetchMock } = contexts();
    const prepare = vi.spyOn(ManifestStoreClient.prototype, "prepare");
    const hook = renderHook(() => useLiveStore()); await tick();
    const cachedA = prepare.mock.contexts.at(-1);
    act(() => hook.result.current.selectWorktree("b")); await tick();
    const timeout = new AbortController();
    vi.spyOn(AbortSignal, "timeout").mockReturnValueOnce(timeout.signal);
    let release!: (response: Response) => void;
    fetchMock.mockImplementationOnce(() => new Promise(resolve => { release = resolve; }));
    let pending!: Promise<void>;
    act(() => { pending = hook.result.current.refreshWorktrees(); });
    if (failure === "superseded") await act(async () => { await hook.result.current.refreshWorktrees(); });
    else timeout.abort(new DOMException("Inventory timed out", "TimeoutError"));
    const choices = hook.result.current.worktrees;
    await act(async () => {
      release(Response.json({ original_worktree_id: "a", worktrees: [] }));
      await pending;
    });
    expect(hook.result.current.worktrees).toBe(choices);
    act(() => hook.result.current.selectWorktree("a")); await tick();
    expect(prepare.mock.contexts.at(-1)).toBe(cachedA);
  });

  it("retains a selected client during a pending request and a listed unavailable client", async () => {
    const { b, fetchMock, transport } = contexts();
    const prepare = vi.spyOn(ManifestStoreClient.prototype, "prepare");
    const hook = renderHook(() => useLiveStore()); await tick();
    act(() => hook.result.current.selectWorktree("b")); await tick();
    const cachedB = prepare.mock.contexts.at(-1);
    act(() => hook.result.current.selectWorktree("a")); await tick();
    let release!: (response: Response) => void;
    let hold = true;
    let listed = ["a"];
    fetchMock.mockImplementation(async (url, init) => {
      if (url === "/api/worktrees") return Response.json({ original_worktree_id: "a", worktrees: listed.map(worktree_id => ({
        worktree_id, branch: worktree_id, available: worktree_id !== "b", explorer_available: worktree_id !== "b",
      })) });
      if (hold && url.includes("/manifest") && url.includes("worktree_id=b")) {
        hold = false;
        return new Promise(resolve => { release = resolve; });
      }
      return transport(url, init);
    });
    act(() => hook.result.current.selectWorktree("b")); await tick();
    expect(hook.result.current.switching).toBe(true);
    await act(async () => { await hook.result.current.refreshWorktrees(); });
    listed = ["a", "b"];
    await act(async () => { await hook.result.current.refreshWorktrees(); });
    await act(async () => { release(manifestResponse(b)); }); await tick();
    expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(b.store);
    act(() => hook.result.current.selectWorktree("a")); await tick();
    act(() => hook.result.current.selectWorktree("b")); await tick();
    expect(prepare.mock.contexts.at(-1)).toBe(cachedB);
  });

  it("retires successive removed context caches without loading unselected branches", async () => {
    const { a, fetchMock } = contexts();
    delete window.reqvireLiveRefresh;
    const prepare = vi.spyOn(ManifestStoreClient.prototype, "prepare");
    const snapshots = new Map([["a", a]]);
    fetchMock.mockImplementation(async (url, init = {}) => {
      const parsed = new URL(url, window.location.origin);
      if (parsed.pathname === "/api/worktrees/load") return Response.json({});
      if (parsed.pathname === "/api/worktrees") return Response.json({ original_worktree_id: "a", worktrees: [...snapshots.keys()]
        .map(worktree_id => ({ worktree_id, branch: "shared-branch-label" })) });
      const snapshot = snapshots.get(parsed.searchParams.get("worktree_id")!)!;
      return parsed.pathname.endsWith("/manifest") ? manifestResponse(snapshot) : chunkResponse(snapshot, init);
    });
    const hook = renderHook(() => useLiveStore()); await tick();
    const cachedA = prepare.mock.contexts.at(-1);
    const retired = new Map<string, unknown>();
    // Fixture size exercises repeated retire/recreate cycles; it is not a cache limit.
    for (let index = 0; index < 20; index++) {
      const id = `context-${index}`;
      for (const recreated of [false, true]) {
        const store = smallStore();
        const snapshot = wireSnapshot({ ...store, project: { ...store.project, worktree_id: id },
          elements: [{ ...store.elements[0], content: recreated ? "Recreated" : "Original" }] });
        snapshots.set(id, snapshot);
        act(() => hook.result.current.selectWorktree(id)); await tick();
        expect(hook.result.current.result.ok && hook.result.current.result.store).toEqual(snapshot.store);
        const client = prepare.mock.contexts.at(-1);
        if (recreated) expect(client).not.toBe(retired.get(id));
        else retired.set(id, client);
        act(() => hook.result.current.selectWorktree("a")); await tick();
        expect(prepare.mock.contexts.at(-1)).toBe(cachedA);
        snapshots.delete(id);
        fetchMock.mockClear();
        await act(async () => { await hook.result.current.refreshWorktrees(); });
        expect(fetchMock.mock.calls.map(([url]) => url)).toEqual(["/api/worktrees"]);
      }
    }
  });
});

it("keeps a recovery warning on unchanged accepted snapshots and clears it with a healthy response", async () => {
  const { seed, fetchMock, transport } = backend();
  let recovery = true;
  fetchMock.mockImplementation(async (url, init) => {
    const response = await transport(url, init);
    if (recovery && url.endsWith('/manifest')) response.headers.set('X-Reqvire-Recovery-Required', 'true');
    return response;
  });
  const hook = renderHook(() => useLiveStore());
  await tick();
  expect(hook.result.current.recoveryWarning).toMatch(/last accepted model.*writes are disabled/i);
  expect(hook.result.current.refreshError).toBeNull();
  expect(hook.result.current.result.ok && hook.result.current.result.store).toBe(seed.store);
  await tick(5000);
  expect(hook.result.current.recoveryWarning).toMatch(/recovery/i);
  expect(fetchMock.mock.calls.every(([url]) => url.endsWith('/manifest'))).toBe(true);
  recovery = false;
  await tick(5000);
  expect(hook.result.current.recoveryWarning).toBeNull();
});

it("adopts a fresh recovery snapshot after reload and labels it without a refresh failure", async () => {
  const seed = wireSnapshot(smallStore());
  delete window.reqvireProjectStore;
  window.reqvireLiveRefresh = seed.seed;
  vi.stubGlobal('fetch', vi.fn(async (url: string, init: RequestInit = {}) => {
    if (!url.endsWith('/manifest')) return chunkResponse(seed, init);
    const response = manifestResponse(seed);
    response.headers.set('X-Reqvire-Recovery-Required', 'true');
    return response;
  }));
  const hook = renderHook(() => useLiveStore());
  await tick();
  expect(hook.result.current.result.ok).toBe(true);
  expect(hook.result.current.recoveryWarning).toMatch(/writes are disabled/i);
  expect(hook.result.current.refreshError).toBeNull();
});
