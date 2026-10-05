import { describe, expect, it } from "vitest";
import { devFixture } from "../store/devFixture";
import type { ProjectStoreThesaurus } from "../store/types";
import { prepareThesaurus, filterThesaurusSchemes, thesaurusAncestorIds } from "./thesaurus";

function projection(parents: Array<string | null>, secondSchemeAt = parents.length): ProjectStoreThesaurus {
  const scheme = devFixture.thesaurus.schemes[0];
  const concept = devFixture.thesaurus.concepts[0];
  return {
    schemes: [{ ...scheme, id: "scheme", label: "First" }, { ...scheme, id: "other", label: "Other" }],
    concepts: parents.map((parent_id, index) => ({
      ...concept, id: String(index), label: `Concept ${index}`, parent_id,
      scheme_id: index < secondSchemeAt ? "scheme" : "other", scheme_label: "",
    })),
  };
}

describe("shared Thesaurus topology", () => {
  it("prepares parent fields once and shares an immutable projection's index", () => {
    const data = projection(Array.from({ length: 2000 }, (_, i) => i ? String(i - 1) : null));
    data.concepts.reverse();
    let reads = 0;
    for (const concept of data.concepts) {
      const parent = concept.parent_id;
      Object.defineProperty(concept, "parent_id", { get() { reads += 1; return parent; } });
    }
    const prepared = prepareThesaurus(data);
    expect(reads).toBe(data.concepts.length);
    expect(prepared.depthById.get("1999")).toBe(1999);
    expect(prepareThesaurus(data)).toBe(prepared);
    expect(reads).toBe(data.concepts.length);
    expect(thesaurusAncestorIds(prepared, "1999").size).toBe(2000);
    expect(prepareThesaurus({ ...data })).not.toBe(prepared);
  });

  it("retains ancestors and sorted children during search without rebuilding topology", () => {
    const data = projection([null, "0", "1", null]);
    data.concepts[2].definition = "matched definition";
    const prepared = prepareThesaurus(data);
    const filtered = filterThesaurusSchemes(prepared.schemes, "matched");
    expect(filtered).toHaveLength(1);
    expect(filtered[0].concepts.map(c => c.id)).toEqual(["0", "1", "2"]);
    expect(filtered[0].roots.map(c => c.id)).toEqual(["0"]);
    expect(filtered[0].childrenById.get("1")?.map(c => c.id)).toEqual(["2"]);
    expect(filterThesaurusSchemes(prepared.schemes, "")).toBe(prepared.schemes);
    expect(filterThesaurusSchemes(prepared.schemes, "no match")).toEqual([]);
  });

  it("preserves scheme and external source identity while rooting absent or cross-scheme parents", () => {
    const data = projection([null, "missing", "0"], 2);
    data.concepts[1].element_id = "";
    data.concepts[1].source_href = "https://example.org/concepts#external";
    const prepared = prepareThesaurus(data);
    expect(prepared.schemes.map(s => s.label)).toEqual(["First", "Other"]);
    expect(prepared.schemes[0].roots.map(c => c.id)).toEqual(["0", "1"]);
    expect(prepared.schemes[1].roots.map(c => c.id)).toEqual(["2"]);
    expect(prepared.concepts.find(c => c.id === "1")?.sourceHref).toBe(data.concepts[1].source_href);
    expect(prepared.concepts.find(c => c.id === "1")?.sourceElementId).toBe("");
  });

  it("breaks a malformed cycle deterministically without hiding its members or looping search", () => {
    const data = projection(["1", "2", "0", "2"]);
    const prepared = prepareThesaurus(data);
    expect(prepared.parentById.get("0")).toBeNull();
    expect(prepared.schemes[0].roots.map(c => c.id)).toEqual(["0"]);
    expect(thesaurusAncestorIds(prepared, "3")).toEqual(new Set(["3", "2", "0"]));
    expect(filterThesaurusSchemes(prepared.schemes, "Concept 3")[0].concepts.map(c => c.id)).toEqual(["0", "2", "3"]);
    expect(data.concepts[0].parent_id).toBe("1");
    const reordered = prepareThesaurus({ ...data, concepts: [...data.concepts].reverse() });
    expect(reordered.parentById.get("0")).toBeNull();
    expect(reordered.schemes[0].concepts.map(c => c.id)).toEqual(prepared.schemes[0].concepts.map(c => c.id));
  });
});
