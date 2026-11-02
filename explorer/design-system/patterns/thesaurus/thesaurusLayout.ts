/**
 * Concept map geometry. Node boxes have a fixed width and a bounded height (ThesaurusExplorer's node
 * styles and --color-role-thesaurus-map-*-node-min-width), so positions can be computed up front.
 * Positions are top-left corners. The scheme is centred on x = 0 and each side of it fills columns
 * that start past its edge; nodes in one column are at least a row apart.
 */
export const THESAURUS_SCHEME_NODE_ID = "__scheme__";
export const THESAURUS_SCHEME_NODE_WIDTH = 264;
export const THESAURUS_CONCEPT_NODE_WIDTH = 188;
/** Two label lines, the optional meta line and padding. */
export const THESAURUS_NODE_MAX_HEIGHT = 88;
const COLUMN_GAP = 52;
const MINDMAP_X_STEP = THESAURUS_CONCEPT_NODE_WIDTH + COLUMN_GAP;
const MINDMAP_Y_STEP = 112;
const MINDMAP_SIDE_OFFSET = 56;

export interface ThesaurusLayoutConcept {
  id: string;
  label: string;
  parentId?: string | null;
}

export type ThesaurusPosition = { x: number; y: number };

export const THESAURUS_SCHEME_POSITION: ThesaurusPosition = { x: -THESAURUS_SCHEME_NODE_WIDTH / 2, y: 0 };

function columnX(direction: number, depth: number) {
  const nearest = direction > 0
    ? THESAURUS_SCHEME_NODE_WIDTH / 2 + COLUMN_GAP
    : -THESAURUS_SCHEME_NODE_WIDTH / 2 - COLUMN_GAP - THESAURUS_CONCEPT_NODE_WIDTH;
  return nearest + direction * (depth - 1) * MINDMAP_X_STEP;
}

/** Related concepts in a column on each side of the focused one, whatever their number. */
export function mapFocusNeighborhoodPositions(focusId: string, visibleIds: ReadonlySet<string>) {
  const positionById = new Map<string, ThesaurusPosition>();
  positionById.set(focusId, { x: 0, y: 0 });
  const relatedIds = Array.from(visibleIds).filter((id) => id !== focusId);
  [1, -1].forEach((direction, side) => {
    const column = relatedIds.filter((_id, index) => index % 2 === side);
    column.forEach((id, index) => {
      positionById.set(id, {
        x: direction * MINDMAP_X_STEP,
        y: (index - (column.length - 1) / 2) * MINDMAP_Y_STEP,
      });
    });
  });
  return positionById;
}

export function mapMindMapPositions(
  concepts: readonly ThesaurusLayoutConcept[],
  topConceptIds: readonly string[],
  conceptIds: ReadonlySet<string>,
  branchDirectionByTopConcept: ReadonlyMap<string, number>,
) {
  const childrenByParent = new Map<string, ThesaurusLayoutConcept[]>();
  const conceptById = new Map(concepts.map((concept) => [concept.id, concept]));
  for (const concept of concepts) {
    const parentId = concept.parentId && conceptIds.has(concept.parentId) ? concept.parentId : THESAURUS_SCHEME_NODE_ID;
    const siblings = childrenByParent.get(parentId) ?? [];
    siblings.push(concept);
    childrenByParent.set(parentId, siblings);
  }
  for (const siblings of childrenByParent.values()) {
    siblings.sort((left, right) => left.label.localeCompare(right.label));
  }

  const positionById = new Map<string, ThesaurusPosition>();
  let rightCursor = 0;
  const placeSide = (rootIds: readonly string[], direction: number) => {
    const totalLeaves = rootIds.reduce((sum, id) => sum + countConceptLeaves(id, childrenByParent), 0);
    const sideOffset = direction > 0 ? -MINDMAP_SIDE_OFFSET : MINDMAP_SIDE_OFFSET;
    let cursor = -((Math.max(totalLeaves, 1) - 1) * MINDMAP_Y_STEP) / 2 + sideOffset;
    for (const rootId of rootIds) {
      const root = conceptById.get(rootId);
      if (!root) continue;
      cursor = placeConceptSubtree(root, direction, 1, cursor, childrenByParent, positionById);
    }
    if (direction > 0) rightCursor = cursor;
  };
  placeSide(topConceptIds.filter((id) => branchDirectionByTopConcept.get(id) === -1), -1);
  placeSide(topConceptIds.filter((id) => branchDirectionByTopConcept.get(id) !== -1), 1);
  // A broader cycle has no root to reach it from: list its concepts under the right side.
  for (const concept of concepts) {
    if (positionById.has(concept.id)) continue;
    positionById.set(concept.id, { x: columnX(1, 1), y: rightCursor });
    rightCursor += MINDMAP_Y_STEP;
  }
  return positionById;
}

function placeConceptSubtree(
  concept: ThesaurusLayoutConcept,
  direction: number,
  depth: number,
  cursor: number,
  childrenByParent: ReadonlyMap<string, readonly ThesaurusLayoutConcept[]>,
  positionById: Map<string, ThesaurusPosition>,
) {
  const children = childrenByParent.get(concept.id) ?? [];
  if (children.length === 0) {
    positionById.set(concept.id, { x: columnX(direction, depth), y: cursor });
    return cursor + MINDMAP_Y_STEP;
  }
  const start = cursor;
  let nextCursor = cursor;
  for (const child of children) {
    nextCursor = placeConceptSubtree(child, direction, depth + 1, nextCursor, childrenByParent, positionById);
  }
  const end = nextCursor - MINDMAP_Y_STEP;
  positionById.set(concept.id, { x: columnX(direction, depth), y: (start + end) / 2 });
  return nextCursor;
}

function countConceptLeaves(id: string, childrenByParent: ReadonlyMap<string, readonly ThesaurusLayoutConcept[]>): number {
  const children = childrenByParent.get(id) ?? [];
  if (children.length === 0) return 1;
  return children.reduce((sum, child) => sum + countConceptLeaves(child.id, childrenByParent), 0);
}
