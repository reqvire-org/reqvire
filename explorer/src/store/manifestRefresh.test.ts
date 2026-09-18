import { afterEach, describe, expect, it, vi } from "vitest";
import { ManifestStoreClient } from "./manifestRefresh";
import { chunkResponse, manifestResponse, smallStore, wireHash, wireSnapshot } from "../test/liveStoreFixtures";

afterEach(() => vi.unstubAllGlobals());

function server(initial = smallStore()) {
  const seed = wireSnapshot(initial);
  let current = seed;
  const fetchMock = vi.fn(async (url: string, init: RequestInit = {}) => {
    if (url.endsWith("/manifest")) {
      const headers = new Headers(init.headers);
      return headers.get("If-None-Match") === `"${current.revision}"`
        ? new Response(null, { status: 304, headers: { ETag: `"${current.revision}"` } })
        : manifestResponse(current);
    }
    expect(url).toBe("/api/project-store/chunks");
    return chunkResponse(current, init);
  });
  vi.stubGlobal("fetch", fetchMock);
  const client = new ManifestStoreClient(initial, seed.seed);
  return { seed, client, fetchMock, update: (value: unknown) => { current = wireSnapshot(value); return current; } };
}
const signal = () => new AbortController().signal;

describe("manifest refresh transactions", () => {
  it("accepts a matching empty 304 without downloading chunks", async () => {
    const { client, fetchMock, seed } = server();
    expect(await client.prepare(signal())).toBeNull();
    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(new Headers(fetchMock.mock.calls[0][1]?.headers).get("If-None-Match")).toBe(`"${seed.revision}"`);
  });

  it("catches up from any old revision, preserving unchanged references and latest order", async () => {
    const initial = smallStore();
    const { client, fetchMock, update } = server(initial);
    update({ ...initial, elements: [] });
    update({ ...initial, elements: initial.elements.slice(0, 1) });
    const latest = { ...initial, elements: [initial.elements[1], { ...initial.elements[0], content: "Latest café 測定." }] };
    update(latest);
    const next = await client.prepare(signal());
    expect(next?.result.store).toEqual(latest);
    expect(next?.result.store.elements[0]).toBe(initial.elements[1]);
    expect(next?.result.store.files).toBe(initial.files);
    expect(initial.elements[0].content).toBe("First record.");
    const body = JSON.parse(String(fetchMock.mock.calls[1][1]?.body));
    expect(body.hashes).toEqual([wireHash(JSON.stringify(latest.elements[1]))]);
    client.commit(next!);
    expect(await client.prepare(signal())).toBeNull();
  });

  it("handles deletion-only manifests and prunes deleted chunks before re-addition", async () => {
    const initial = smallStore();
    const { client, fetchMock, update } = server(initial);
    update({ ...initial, elements: [] });
    const removed = await client.prepare(signal());
    expect(removed?.result.store.elements).toEqual([]);
    expect(fetchMock).toHaveBeenCalledTimes(1);
    client.commit(removed!);
    update(initial);
    const restored = await client.prepare(signal());
    expect(restored?.result.store).toEqual(initial);
    expect(JSON.parse(String(fetchMock.mock.calls[2][1]?.body)).hashes).toHaveLength(2);
  });

  it("deduplicates downloads but retains repeated references and unknown sections", async () => {
    const { client, fetchMock, update } = server();
    const latest = { ...smallStore(), labels: ["identical", "identical"] };
    update(latest);
    const next = await client.prepare(signal());
    expect(next?.result.store).toEqual(latest);
    expect(JSON.parse(String(fetchMock.mock.calls[1][1]?.body)).hashes).toHaveLength(1);
  });

  it("keeps published records and manifests immutable so shared cache entries remain valid", async () => {
    const initial = smallStore();
    const { client, seed, update } = server(initial);
    expect(() => { initial.elements[0].content = "Accidentally mutated"; }).toThrow(TypeError);
    expect(() => initial.elements.reverse()).toThrow(TypeError);
    expect(() => { seed.manifest.sections.elements = { kind: "array", hashes: [] }; }).toThrow(TypeError);
    const latest = update({ ...initial, elements: [{ ...initial.elements[0], content: "Published record" }] });
    const next = await client.prepare(signal());
    expect(next?.result.store).toEqual(latest.store);
    expect(() => { next!.result.store.elements[0].metadata.changed = "Mutation"; }).toThrow(TypeError);
  });

  it("does not advance its cursor when a prepared snapshot is abandoned", async () => {
    const { client, seed, update, fetchMock } = server();
    update({ ...smallStore(), elements: [] });
    await client.prepare(signal());
    await client.prepare(signal());
    expect(fetchMock.mock.calls.every(([, init]) => new Headers(init?.headers).get("If-None-Match") === `"${seed.revision}"`)).toBe(true);
  });

  it("recovers from an invalid seed manifest by fetching all required chunks", async () => {
    const latest = wireSnapshot(smallStore());
    const client = new ManifestStoreClient(smallStore(), { revision: latest.revision, manifest: {} });
    const fetchMock = vi.fn(async (url: string, init: RequestInit = {}) => url.endsWith("/manifest")
      ? manifestResponse(latest) : chunkResponse(latest, init));
    vi.stubGlobal("fetch", fetchMock);
    expect((await client.prepare(signal()))?.result.store).toEqual(latest.store);
    expect(new Headers(fetchMock.mock.calls[0][1]?.headers).has("If-None-Match")).toBe(false);
    expect(JSON.parse(String(fetchMock.mock.calls[1][1]?.body)).hashes.length).toBe(latest.chunks.size);
  });

  it.each(["project", "coverage", "ontology", "routes"])("rejects a hash-valid malformed %s section", async name => {
    const { client, update } = server();
    update({ ...smallStore(), [name]: [] });
    await expect(client.prepare(signal())).rejects.toThrow(`section "${name}" must be an object`);
  });

  it("rejects hash-valid element chunks that are not modeled records", async () => {
    const { client, update } = server();
    update({ ...smallStore(), elements: [null, "not a record", { id: 42 }] });
    await expect(client.prepare(signal())).rejects.toThrow(/elements.*record/);
  });

  it.each(["missing", "extra", "wrong revision", "corrupt"])("rejects %s chunk responses without advancing", async failure => {
    const { client, seed, update, fetchMock } = server();
    const latest = update({ ...smallStore(), elements: [{ ...smallStore().elements[0], content: "Changed." }] });
    fetchMock.mockImplementation(async (url, init = {}) => {
      if (url.endsWith("/manifest")) return manifestResponse(latest);
      const hashes: string[] = JSON.parse(String(init.body)).hashes;
      const chunks = Object.fromEntries(hashes.map(hash => [hash, latest.chunks.get(hash)]));
      if (failure === "missing") delete chunks[hashes[0]];
      if (failure === "extra") chunks["extra"] = "null";
      if (failure === "corrupt") chunks[hashes[0]] = "{}";
      return Response.json({ revision: failure === "wrong revision" ? seed.revision : latest.revision, chunks });
    });
    await expect(client.prepare(signal())).rejects.toThrow(/chunk/);
    fetchMock.mockImplementation(async (url, init = {}) => url.endsWith("/manifest")
      ? manifestResponse(latest) : chunkResponse(latest, init));
    expect((await client.prepare(signal()))?.result.store).toEqual(latest.store);
    const request = [...fetchMock.mock.calls].reverse().find(([url]) => url.endsWith("/manifest"))!;
    expect(new Headers(request[1]?.headers).get("If-None-Match")).toBe(`"${seed.revision}"`);
  });

  it.each(["hash", "protocol", "reference", "schema", "shape"])("fails closed on an invalid %s", async failure => {
    const { client, update, fetchMock } = server();
    const store = { ...smallStore(), elements: [] as unknown, schema_version: smallStore().schema_version };
    if (failure === "schema") store.schema_version = "unsupported";
    if (failure === "shape") store.elements = {};
    const latest = update(store);
    const manifest = structuredClone(latest.manifest);
    if (failure === "protocol") manifest.protocol = "unknown.v1";
    if (failure === "reference") manifest.sections.elements = { kind: "value", hash: "invalid" };
    const text = JSON.stringify(manifest);
    fetchMock.mockImplementation(async (url, init = {}) => url.endsWith("/manifest")
      ? new Response(text, { headers: { ETag: `"${failure === "hash" ? "0".repeat(64) : wireHash(text)}"` } })
      : chunkResponse(latest, init));
    await expect(client.prepare(signal())).rejects.toThrow();
  });

  it("retries a real revision conflict against the latest manifest", async () => {
    const { client, update, fetchMock } = server();
    const before = update({ ...smallStore(), elements: [{ ...smallStore().elements[0], content: "Before." }] });
    const latest = wireSnapshot({ ...smallStore(), elements: [{ ...smallStore().elements[0], content: "After." }] });
    let current = before;
    fetchMock.mockImplementation(async (url, init = {}) => {
      if (url.endsWith("/manifest")) return manifestResponse(current);
      current = latest;
      return chunkResponse(current, init);
    });
    const next = await client.prepare(signal());
    expect(next?.result.store).toEqual(latest.store);
    expect(fetchMock).toHaveBeenCalledTimes(4);
    expect(next?.revision).toBe(latest.revision);
  });

  it("bounds retries when every chunk request is superseded", async () => {
    const { client, update, fetchMock } = server();
    const latest = update({ ...smallStore(), elements: [{ ...smallStore().elements[0], content: "Changed." }] });
    fetchMock.mockImplementation(async url => url.endsWith("/manifest")
      ? manifestResponse(latest) : new Response(null, { status: 409 }));
    await expect(client.prepare(signal())).rejects.toThrow(/kept changing/);
    expect(fetchMock).toHaveBeenCalledTimes(6);
  });

  it("restarts a multi-batch download without mixing two published revisions", async () => {
    const { client, update, fetchMock } = server();
    const records = (marker: string) => Array.from({ length: 600 }, (_, index) => ({
      ...smallStore().elements[0], id: `record-${index}`, content: `${marker} ${index}`,
    }));
    const before = update({ ...smallStore(), elements: records("Before") });
    const latest = wireSnapshot({ ...smallStore(), elements: records("Latest") });
    let current = before;
    let batches = 0;
    fetchMock.mockImplementation(async (url, init = {}) => {
      if (url.endsWith("/manifest")) return manifestResponse(current);
      if (++batches === 2) current = latest;
      return chunkResponse(current, init);
    });
    const next = await client.prepare(signal());
    expect(next?.result.store).toEqual(latest.store);
    expect(next?.revision).toBe(latest.revision);
    const revisions = fetchMock.mock.calls.filter(([url]) => url.endsWith("/chunks"))
      .map(([, init]) => JSON.parse(String(init?.body)).revision);
    expect(revisions).toEqual([before.revision, before.revision, latest.revision, latest.revision]);
  });

  it("verifies downloaded chunks when Web Crypto is unavailable on an HTTP origin", async () => {
    vi.stubGlobal("crypto", undefined);
    const { client, update } = server();
    const latest = update({ ...smallStore(), elements: [{ ...smallStore().elements[0], content: "café 測定" }] });
    expect((await client.prepare(signal()))?.result.store).toEqual(latest.store);
  });

  it("discards partial batches after a network failure and succeeds on retry", async () => {
    const { client, update, fetchMock } = server();
    const latest = update({ ...smallStore(), elements: Array.from({ length: 600 }, (_, index) => ({
      ...smallStore().elements[0], id: `new-${index}`, name: `Record ${index}`,
    })) });
    let batches = 0;
    fetchMock.mockImplementation(async (url, init = {}) => {
      if (url.endsWith("/manifest")) return manifestResponse(latest);
      if (++batches === 2) throw new Error("Disconnected");
      return chunkResponse(latest, init);
    });
    await expect(client.prepare(signal())).rejects.toThrow("Disconnected");
    const next = await client.prepare(signal());
    expect(next?.result.store).toEqual(latest.store);
    const sizes = fetchMock.mock.calls.filter(([url]) => url.endsWith("/chunks"))
      .map(([, init]) => JSON.parse(String(init?.body)).hashes.length);
    expect(sizes).toEqual([512, 88, 512, 88]);
  });

  it("honors cancellation even when the fetch transport completes after abort", async () => {
    const { client, update, fetchMock } = server();
    const latest = update({ ...smallStore(), elements: [] });
    const controller = new AbortController();
    fetchMock.mockImplementation(async () => {
      controller.abort();
      return manifestResponse(latest);
    });
    await expect(client.prepare(controller.signal)).rejects.toThrow();
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });
});
