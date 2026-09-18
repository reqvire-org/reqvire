import { createHash } from "node:crypto";
import { devFixture } from "../store/devFixture";
import type { ExplorerProjectStore } from "../store/types";

export function wireHash(content: string): string {
  return createHash("sha256").update(content).digest("hex");
}

export function wireSnapshot(store: unknown) {
  const chunks = new Map<string, string>();
  const section = (value: unknown): string => {
    const content = JSON.stringify(value);
    const hash = wireHash(content);
    chunks.set(hash, content);
    return hash;
  };
  const manifest = {
    protocol: "reqvire-manifest.v1",
    sections: Object.fromEntries(Object.entries(store as object).map(([name, value]) => [name,
      Array.isArray(value) ? { kind: "array", hashes: value.map(section) } : { kind: "value", hash: section(value) },
    ])),
    ontology_hash: wireHash("@prefix : <urn:manifest-test:> ."),
  };
  const text = JSON.stringify(manifest);
  const revision = wireHash(text);
  return { store, chunks, manifest, text, revision, seed: { revision, manifest } };
}

export function smallStore(): ExplorerProjectStore {
  return {
    ...devFixture,
    elements: [
      { ...devFixture.elements[0], id: "first", name: "Lärche 測定", content: "First record." },
      { ...devFixture.elements[0], id: "second", name: "Aster", content: "Unchanged record." },
    ],
  };
}

export function manifestResponse(snapshot: ReturnType<typeof wireSnapshot>): Response {
  return new Response(snapshot.text, { headers: { ETag: `"${snapshot.revision}"` } });
}

export function chunkResponse(snapshot: ReturnType<typeof wireSnapshot>, request: RequestInit): Response {
  const { revision, hashes } = JSON.parse(String(request.body)) as { revision: string; hashes: string[] };
  if (revision !== snapshot.revision) return new Response(null, { status: 409 });
  return Response.json({ revision, chunks: Object.fromEntries(hashes.map(hash => [hash, snapshot.chunks.get(hash)])) });
}
