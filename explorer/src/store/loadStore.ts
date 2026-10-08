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
  "contract_bindings",
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
  "contract_bindings",
  "contract_references",
  "concept_refs",
  "search",
] as const;

const OBJECT_SECTIONS = [
  "project", "submodels", "traces", "coverage", "ontology",
  "knowledge_graph", "summaries", "routes", "thesaurus",
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
  if (typeof candidate !== "object" || candidate === null || Array.isArray(candidate)) {
    return ["seed is not an object"];
  }
  const seed = candidate as Record<string, unknown>;
  for (const section of REQUIRED_SECTIONS) {
    if (!(section in seed)) {
      problems.push(`missing required section "${section}"`);
    }
  }
  for (const section of ARRAY_SECTIONS) {
    if (!(section in seed)) continue;
    const records = seed[section];
    if (!Array.isArray(records)) {
      problems.push(`section "${section}" must be an array`);
      continue;
    }
    const identifier = section === "folders" || section === "files" ? "path" : "id";
    if (records.some(record => typeof record !== "object" || record === null || Array.isArray(record)
      || typeof record[identifier] !== "string")) {
      problems.push(`section "${section}" must contain records with string "${identifier}" identifiers`);
    }
  }
  for (const section of OBJECT_SECTIONS) {
    if (!(section in seed)) continue;
    const value = seed[section];
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
      problems.push(`section "${section}" must be an object`);
    }
  }
  if (isRecord(seed.coverage)) {
    const index = seed.coverage.scope_index;
    if (!isRecord(index)) problems.push("coverage.scope_index must be an object");
    else for (const [identifier, entry] of Object.entries(index)) {
      const scope = isRecord(entry) && isRecord(entry.scope) ? entry.scope : null;
      const summary = isRecord(entry) && isRecord(entry.summary) ? entry.summary : null;
      const membershipValid = scope && ["capability_ids", "requirement_ids", "verification_ids"].every(key => {
        const ids = scope[key];
        return Array.isArray(ids) && ids.every(id => typeof id === "string") && new Set(ids).size === ids.length;
      });
      if (!scope || scope.kind !== "capability" || scope.capability_identifier !== identifier
        || typeof scope.capability_name !== "string" || scope.orphaned_verifications_scope !== "whole_model_only"
        || !membershipValid || !(scope.capability_ids as string[]).includes(identifier)
        || !summary || typeof summary.total_requirements_in_scope !== "number"
        || Object.values(summary).some(value => typeof value === "number" && (!Number.isFinite(value) || value < 0))) {
        problems.push(`coverage.scope_index entry "${identifier}" is malformed`);
      }
    }
  }
  if (isRecord(seed.coverage)) {
    for (const sectionName of ["covered_requirements", "uncovered_requirements"]) {
      const section = seed.coverage[sectionName];
      if (!isRecord(section) || !isRecord(section.files)) {
        problems.push(`coverage.${sectionName}.files must be an object`);
        continue;
      }
      for (const rows of Object.values(section.files)) {
        if (!Array.isArray(rows) || rows.some(row => !isRecord(row)
          || typeof row.identifier !== "string" || typeof row.name !== "string"
          || typeof row.is_terminal !== "boolean" || !validCoverageAggregates(row)
          || !["direct_satisfied", "requirement_rollup", "contract_consumer_rollup", "combined_rollup", "uncovered"].includes(String(row.coverage_source))
          || ["direct_evidence", "evidence", "contributing_requirements", "blocking_requirements"].some(key =>
            !Array.isArray(row[key]) || !row[key].every(value => typeof value === "string")))) {
          problems.push(`coverage.${sectionName} contains a malformed requirement record`);
        }
      }
    }
    const capabilities = isRecord(seed.coverage.capability_coverage) ? seed.coverage.capability_coverage.capabilities : null;
    if (!Array.isArray(capabilities) || capabilities.some(row => !isRecord(row)
      || typeof row.identifier !== "string" || typeof row.name !== "string"
      || typeof row.implementation_covered !== "boolean" || !validCoverageAggregates(row))) {
      problems.push("coverage.capability_coverage must contain capability aggregate metrics");
    }
  }
  return problems;
}

function validCoverageAggregates(row: Record<string, unknown>): boolean {
  return [
    ["aggregate_verified_leaf_requirements", "aggregate_leaf_requirements"],
    ["aggregate_covered_terminal_requirements", "aggregate_terminal_requirements"],
  ].every(([coveredKey, totalKey]) => {
    const covered = row[coveredKey], total = row[totalKey];
    return typeof covered === "number" && Number.isInteger(covered) && covered >= 0
      && typeof total === "number" && Number.isInteger(total) && total >= covered;
  });
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
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

  return loadStoreCandidate(seed);
}

/** Validate an initial seed or a newly fetched snapshot using the same contract. */
export function loadStoreCandidate(seed: unknown): StoreLoadResult {
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
