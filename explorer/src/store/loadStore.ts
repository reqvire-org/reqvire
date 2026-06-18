/*
 * Project Store loader.
 *
 * Resolution order:
 *   1. `window.reqvireProjectStore` (global set by assets/project-store.js).
 *   2. Dev fixture (only in `import.meta.env.DEV`) so `npm run dev` works
 *      without `reqvire serve`.
 *
 * The loader FAILS CLOSED: a missing/malformed/incompatible seed yields a
 * diagnostic `LoadFailure` rather than a partially-rendered Explorer, per the
 * Explorer Store Seed Data Output Specification.
 */
import {
  EXPECTED_SCHEMA_VERSION,
  type ExplorerProjectStore,
} from "./types";

export type StoreLoadResult =
  | { ok: true; store: ExplorerProjectStore; schemaMismatch: string | null }
  | { ok: false; reason: string; detail?: string };

/** Top-level sections required by the store contract (store.rs / spec). */
const REQUIRED_SECTIONS = [
  "project",
  "folders",
  "files",
  "resources",
  "elements",
  "relations",
  "reused_contract_context",
  "concept_refs",
  "submodels",
  "traces",
  "coverage",
  "ontology",
  "knowledge_graph",
  "search",
  "summaries",
  "routes",
] as const;

const ARRAY_SECTIONS = [
  "folders",
  "files",
  "resources",
  "elements",
  "relations",
  "reused_contract_context",
  "concept_refs",
  "search",
] as const;

function readInjectedSeed(): unknown {
  if (typeof window !== "undefined" && window.reqvireProjectStore !== undefined) {
    return window.reqvireProjectStore;
  }
  return undefined;
}

/**
 * Validate that a candidate seed has the required shape. Returns a list of
 * problems; empty means structurally valid (forward-compatible: extra fields
 * are tolerated).
 */
export function validateStore(candidate: unknown): string[] {
  const problems: string[] = [];
  if (typeof candidate !== "object" || candidate === null) {
    return ["seed is not an object"];
  }
  const seed = candidate as Record<string, unknown>;
  for (const section of REQUIRED_SECTIONS) {
    if (!(section in seed)) {
      problems.push(`missing required section "${section}"`);
    }
  }
  for (const section of ARRAY_SECTIONS) {
    if (section in seed && !Array.isArray(seed[section])) {
      problems.push(`section "${section}" must be an array`);
    }
  }
  if ("project" in seed) {
    const project = seed.project;
    if (typeof project !== "object" || project === null) {
      problems.push(`section "project" must be an object`);
    }
  }
  return problems;
}

export function loadStore(devFixture?: ExplorerProjectStore): StoreLoadResult {
  let seed = readInjectedSeed();

  if (seed === undefined) {
    if (import.meta.env.DEV && devFixture) {
      seed = devFixture;
    } else {
      return {
        ok: false,
        reason: "No Reqvire Project Store seed found.",
        detail:
          "Expected window.reqvireProjectStore from assets/project-store.js. " +
          "Open this Explorer from reqvire serve, a reqvire export output directory, or npm run dev.",
      };
    }
  }

  const problems = validateStore(seed);
  if (problems.length > 0) {
    return {
      ok: false,
      reason: "Reqvire Project Store seed is malformed.",
      detail: problems.join("; "),
    };
  }

  const store = seed as ExplorerProjectStore;
  const schemaMismatch =
    store.schema_version === EXPECTED_SCHEMA_VERSION
      ? null
      : `seed schema "${store.schema_version ?? "unknown"}" != expected "${EXPECTED_SCHEMA_VERSION}"`;

  return { ok: true, store, schemaMismatch };
}
