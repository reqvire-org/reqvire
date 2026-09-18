import { act, cleanup, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useLiveStore } from "./useLiveStore";
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
