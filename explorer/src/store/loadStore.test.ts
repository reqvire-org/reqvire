import { afterEach, describe, expect, it } from "vitest";
import { loadStore, validateStore } from "./loadStore";
import { EXPECTED_SCHEMA_VERSION } from "./types";
import { devFixture } from "./devFixture";

afterEach(() => {
  delete window.reqvireProjectStore;
});

describe("validateStore", () => {
  it("accepts a complete fixture seed", () => {
    expect(validateStore(devFixture)).toEqual([]);
  });

  it("rejects non-objects", () => {
    expect(validateStore(null)).toContain("seed is not an object");
    expect(validateStore(42)).toContain("seed is not an object");
  });

  it("reports each missing required section", () => {
    const problems = validateStore({ project: {} });
    expect(problems.some((p) => p.includes('missing required section "elements"'))).toBe(true);
    expect(problems.some((p) => p.includes('missing required section "routes"'))).toBe(true);
  });

  it("rejects wrong-typed array sections", () => {
    const seed = { ...devFixture, elements: {} as unknown };
    expect(validateStore(seed)).toContain('section "elements" must be an array');
  });

  it("accepts optional contract references and rejects malformed reference collections", () => {
    expect(validateStore({ ...devFixture, contract_references: [] })).toEqual([]);
    expect(validateStore({ ...devFixture, contract_references: {} }))
      .toContain('section "contract_references" must be an array');
  });

  it("rejects missing or malformed scope indexes", () => {
    expect(validateStore(devFixture)).toEqual([]);
    for (const scope_index of [undefined, null, [], { broken: { scope: {}, summary: {} } }]) {
      const problems = validateStore({ ...devFixture, coverage: { ...devFixture.coverage, scope_index } });
      expect(problems.some(problem => problem.includes("coverage.scope_index"))).toBe(true);
    }
  });
  it("rejects missing or invalid requirement aggregate metrics", () => {
    for (const section of ["covered_requirements", "uncovered_requirements"] as const) {
      for (const key of ["aggregate_leaf_requirements", "aggregate_verified_leaf_requirements",
        "aggregate_terminal_requirements", "aggregate_covered_terminal_requirements"]) {
        for (const value of [undefined, -1, 0.5, "1"]) {
          const seed = structuredClone(devFixture);
          const row = Object.values(seed.coverage[section].files)[0][0];
          Object.assign(row, { [key]: value });
          expect(validateStore(seed).some(problem => problem.includes(`coverage.${section}`))).toBe(true);
        }
      }
    }
  });

});

describe("loadStore", () => {
  it("fails closed when no seed is present and no fixture is given", () => {
    const result = loadStore();
    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.reason).toMatch(/No Reqvire Project Store seed/);
    }
  });

  it("loads an injected window seed", () => {
    window.reqvireProjectStore = devFixture;
    const result = loadStore();
    expect(result.ok).toBe(true);
    if (result.ok) {
      expect(result.schemaMismatch).toBeNull();
      expect(result.store.summaries.elements).toBe(devFixture.summaries.elements);
    }
  });

  it("flags a schema version mismatch but still loads", () => {
    window.reqvireProjectStore = { ...devFixture, schema_version: "old.v0" };
    const result = loadStore();
    expect(result.ok).toBe(true);
    if (result.ok) {
      expect(result.schemaMismatch).toContain(EXPECTED_SCHEMA_VERSION);
    }
  });

  it("fails closed on a malformed injected seed", () => {
    window.reqvireProjectStore = { project: {} };
    const result = loadStore();
    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.reason).toMatch(/malformed/);
    }
  });
});
