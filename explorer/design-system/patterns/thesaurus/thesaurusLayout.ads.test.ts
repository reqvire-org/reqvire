import { describe, expect, it } from "vitest";
import { readCssToken } from "../../../src/test/cssTokens";
import {
  mapFocusNeighborhoodPositions,
  mapMindMapPositions,
  THESAURUS_CONCEPT_NODE_WIDTH,
  THESAURUS_NODE_MAX_HEIGHT,
  THESAURUS_SCHEME_NODE_ID,
  THESAURUS_SCHEME_NODE_WIDTH,
  THESAURUS_SCHEME_POSITION,
  type ThesaurusLayoutConcept,
  type ThesaurusPosition,
} from "./thesaurusLayout";

type Rect = ThesaurusPosition & { id: string; width: number };

function overlaps(rects: readonly Rect[]) {
  const found: string[] = [];
  rects.forEach((a, index) => rects.slice(index + 1).forEach((b) => {
    if (a.x < b.x + b.width && b.x < a.x + a.width && a.y < b.y + THESAURUS_NODE_MAX_HEIGHT && b.y < a.y + THESAURUS_NODE_MAX_HEIGHT) found.push(`${a.id} x ${b.id}`);
  }));
  return found;
}

function mapRects(concepts: readonly ThesaurusLayoutConcept[]) {
  const ids = new Set(concepts.map((concept) => concept.id));
  const tops = concepts.filter((concept) => !concept.parentId || !ids.has(concept.parentId)).map((concept) => concept.id);
  const directions = new Map(tops.map((id, index) => [id, index % 2 === 0 ? 1 : -1]));
  const positions = mapMindMapPositions(concepts, tops, ids, directions);
  expect(positions.size).toBe(concepts.length);
  return [
    { id: THESAURUS_SCHEME_NODE_ID, width: THESAURUS_SCHEME_NODE_WIDTH, ...THESAURUS_SCHEME_POSITION },
    ...[...positions].map(([id, position]) => ({ id, width: THESAURUS_CONCEPT_NODE_WIDTH, ...position })),
  ];
}

const concept = (id: string, parentId?: string): ThesaurusLayoutConcept => ({ id, label: id, parentId });

function tree(tops: number, children: number, depth: number) {
  const out: ThesaurusLayoutConcept[] = [];
  const add = (id: string, parent: string | undefined, left: number) => {
    out.push(concept(id, parent));
    if (left > 0) for (let index = 0; index < children; index++) add(`${id}.${index}`, id, left - 1);
  };
  for (let index = 0; index < tops; index++) add(`t${index}`, undefined, depth);
  return out;
}

describe("thesaurus concept map layout", () => {
  it("keeps the fixture shape clear of the scheme node", () => {
    expect(overlaps(mapRects([
      concept("Traceability"), concept("API Surface"), concept("Verification Coverage"), concept("Service Endpoint", "API Surface"),
    ]))).toEqual([]);
  });

  it.each([[1, 0, 0], [2, 0, 0], [3, 2, 1], [9, 3, 2], [40, 2, 3], [2, 1, 8]])("has no overlaps with %i top concepts, %i children, depth %i", (tops, children, depth) => {
    expect(overlaps(mapRects(tree(tops, children, depth)))).toEqual([]);
  });

  it("places every concept of a broader cycle without overlaps", () => {
    expect(overlaps(mapRects([concept("root"), concept("a", "b"), concept("b", "a")]))).toEqual([]);
  });

  it.each([1, 2, 5, 8, 30])("has no overlaps around a focused concept with %i related", (count) => {
    const related = Array.from({ length: count }, (_, index) => `r${index}`);
    const positions = mapFocusNeighborhoodPositions("focus", new Set(["focus", ...related]));
    expect(overlaps([...positions].map(([id, position]) => ({ id, width: THESAURUS_CONCEPT_NODE_WIDTH, ...position })))).toEqual([]);
  });

  it("uses the widths the node styles declare", () => {
    const px = (name: string) => {
      const expression = readCssToken(name).replace(/var\((--[\w-]+)\)/g, (_, token) => String(Number.parseFloat(readCssToken(token)))).replace(/^calc/, "");
      return Number(new Function(`return ${expression}`)());
    };
    expect(px("--color-role-thesaurus-map-scheme-node-min-width")).toBe(THESAURUS_SCHEME_NODE_WIDTH);
    expect(px("--color-role-thesaurus-map-concept-node-min-width")).toBe(THESAURUS_CONCEPT_NODE_WIDTH);
  });
});
