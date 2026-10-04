import { worktreeUrl } from "./worktreeUrls";
import { sha256 } from "@noble/hashes/sha2.js";
import { bytesToHex } from "@noble/hashes/utils.js";
import { loadStoreCandidate, type StoreLoadResult } from "./loadStore";
import type { ExplorerProjectStore } from "./types";

const PROTOCOL = "reqvire-manifest.v1";
const HASH = /^[a-f0-9]{64}$/;
const CHUNK_BATCH_SIZE = 512;
const MAX_ATTEMPTS = 3;

type ManifestSection = { kind: "value"; hash: string } | { kind: "array"; hashes: string[] };
export interface StoreManifest {
  protocol: typeof PROTOCOL;
  sections: Record<string, ManifestSection>;
  ontology_hash: string;
}

export interface PreparedStore {
  revision: string;
  manifest: StoreManifest;
  result: Extract<StoreLoadResult, { ok: true }>;
  chunks: ReadonlyMap<string, unknown>;
}

function contentHash(text: string): string {
  // Works on ordinary HTTP hosts as well as localhost/HTTPS, without requiring
  // the secure-context-only Web Crypto API.
  return bytesToHex(sha256(new TextEncoder().encode(text)));
}

function object(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function freezeJson(value: unknown): void {
  const pending: unknown[] = [value];
  const visited = new WeakSet<object>();
  while (pending.length) {
    const current = pending.pop();
    if (typeof current !== "object" || current === null || visited.has(current)) continue;
    visited.add(current);
    for (const child of Object.values(current)) pending.push(child);
    Object.freeze(current);
  }
}

function validateManifest(value: unknown): StoreManifest {
  if (!object(value) || value.protocol !== PROTOCOL || !object(value.sections)
    || typeof value.ontology_hash !== "string" || !HASH.test(value.ontology_hash)) {
    throw new Error("Invalid or unsupported refresh manifest.");
  }
  for (const [name, section] of Object.entries(value.sections)) {
    if (!/^[a-z][a-z0-9_]*$/.test(name) || !object(section)) {
      throw new Error("Invalid refresh manifest section.");
    }
    if (section.kind === "value" && typeof section.hash === "string" && HASH.test(section.hash)) continue;
    if (section.kind === "array" && Array.isArray(section.hashes)
      && section.hashes.every(hash => typeof hash === "string" && HASH.test(hash))) continue;
    throw new Error("Invalid refresh manifest chunk reference.");
  }
  return value as unknown as StoreManifest;
}

function hashes(section: ManifestSection): string[] {
  return section.kind === "value" ? [section.hash] : section.hashes;
}

function sameSection(left: ManifestSection | undefined, right: ManifestSection): boolean {
  if (!left || left.kind !== right.kind) return false;
  const before = hashes(left);
  const after = hashes(right);
  return before.length === after.length && before.every((hash, index) => hash === after[index]);
}

async function httpError(response: Response): Promise<Error> {
  const value: unknown = await response.json().catch(() => null);
  return new Error(object(value) && typeof value.error === "string"
    ? value.error : `Server returned HTTP ${response.status}.`);
}

/** Preparation never changes the published revision or cache. The hook commits
 * only if its mounted, visible consumer still owns the request. */
export class ManifestStoreClient {
  private revision?: string;
  private manifest?: StoreManifest;
  private store?: ExplorerProjectStore;
  private chunks: ReadonlyMap<string, unknown> = new Map();

  constructor(store?: ExplorerProjectStore, seed?: { revision: string; manifest: unknown }, private readonly worktreeId?: string) {
    if (worktreeId && store?.project.worktree_id !== worktreeId) return;
    if (!store || !seed) return;
    try {
      const manifest = validateManifest(seed.manifest);
      if (contentHash(JSON.stringify(seed.manifest)) !== seed.revision) return;
      const chunks = new Map<string, unknown>();
      const entries = Object.entries(manifest.sections);
      const values = store as unknown as Record<string, unknown>;
      if (entries.length !== Object.keys(values).length) return;
      for (const [name, section] of entries) {
        if (!Object.hasOwn(values, name)) return;
        const value = values[name];
        if (section.kind === "array") {
          if (!Array.isArray(value) || value.length !== section.hashes.length) return;
          section.hashes.forEach((hash, index) => chunks.set(hash, value[index]));
        } else {
          if (Array.isArray(value)) return;
          chunks.set(section.hash, value);
        }
      }
      // The seed and its manifest arrive together in one generated script.
      // Associate its already-loaded values with the server's wire hashes;
      // reserializing parsed floats would not preserve the original JSON bytes.
      freezeJson(store);
      freezeJson(manifest);
      this.revision = seed.revision;
      this.manifest = manifest;
      this.store = store;
      this.chunks = chunks;
    } catch {
      // A bad advertisement cannot make us accept a 304 with an incomplete
      // cache. Fetch and validate the complete current manifest instead.
    }
  }

  async prepare(signal: AbortSignal, force = false): Promise<PreparedStore | null> {
    for (let attempt = 0; attempt < MAX_ATTEMPTS; attempt++) {
      signal.throwIfAborted();
      const response = await fetch(worktreeUrl("/api/project-store/manifest", this.worktreeId), {
        cache: "no-store",
        headers: this.revision && !force ? { "If-None-Match": `"${this.revision}"` } : {},
        signal,
      });
      signal.throwIfAborted();
      if (response.status === 304) {
        if (!this.revision || response.headers.get("etag") !== `"${this.revision}"`) {
          throw new Error("Invalid unchanged refresh response.");
        }
        return null;
      }
      if (!response.ok) throw await httpError(response);
      const text = await response.text();
      signal.throwIfAborted();
      const revision = response.headers.get("etag")?.match(/^"([a-f0-9]{64})"$/)?.[1];
      if (!revision || contentHash(text) !== revision) throw new Error("Refresh manifest hash mismatch.");
      const manifest = validateManifest(JSON.parse(text));
      const required = new Set(Object.values(manifest.sections).flatMap(hashes));
      const staged = new Map<string, unknown>();
      for (const hash of required) {
        if (this.chunks.has(hash)) staged.set(hash, this.chunks.get(hash));
      }
      const missing = [...required].filter(hash => !staged.has(hash));
      let superseded = false;
      for (let start = 0; start < missing.length; start += CHUNK_BATCH_SIZE) {
        const batch = missing.slice(start, start + CHUNK_BATCH_SIZE);
        const chunksResponse = await fetch(worktreeUrl("/api/project-store/chunks", this.worktreeId), {
          method: "POST", cache: "no-store", signal,
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ revision, hashes: batch }),
        });
        signal.throwIfAborted();
        if (chunksResponse.status === 409) {
          await chunksResponse.body?.cancel();
          superseded = true;
          break;
        }
        if (!chunksResponse.ok) throw await httpError(chunksResponse);
        const payload: unknown = await chunksResponse.json();
        signal.throwIfAborted();
        if (!object(payload) || payload.revision !== revision || !object(payload.chunks)
          || Object.keys(payload.chunks).length !== batch.length) {
          throw new Error("Invalid refresh chunk response.");
        }
        for (const hash of batch) {
          const content = payload.chunks[hash];
          if (typeof content !== "string" || contentHash(content) !== hash) {
            throw new Error("Refresh chunk hash mismatch.");
          }
          staged.set(hash, JSON.parse(content));
        }
      }
      if (superseded) continue;
      const entries = Object.entries(manifest.sections).map(([name, section]) => {
        const old = this.store as unknown as Record<string, unknown> | undefined;
        const value = old && sameSection(this.manifest?.sections[name], section)
          ? old[name]
          : section.kind === "value" ? staged.get(section.hash) : section.hashes.map(hash => staged.get(hash));
        return [name, value];
      });
      const result = loadStoreCandidate(Object.fromEntries(entries));
      if (!result.ok) throw new Error(result.detail ?? result.reason);
      if (result.schemaMismatch) throw new Error(result.schemaMismatch);
      if (this.worktreeId && result.store.project.worktree_id !== this.worktreeId) throw new Error("The server returned a different worktree context.");
      signal.throwIfAborted();
      // UI consumers share unchanged records with the content-addressed cache.
      // Protect that identity from incidental renderer mutations.
      freezeJson(result.store);
      freezeJson(manifest);
      return { revision, manifest, result, chunks: staged };
    }
    throw new Error("The model kept changing during refresh; will retry automatically.");
  }

  commit(next: PreparedStore): void {
    this.revision = next.revision;
    this.manifest = next.manifest;
    this.store = next.result.store;
    // Retain only chunks referenced by the published manifest, including after
    // deletions or retries. Old model revisions cannot accumulate in memory.
    this.chunks = next.chunks;
  }
}
