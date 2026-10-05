import type { ThesaurusConceptItem } from "@ds";
import type { ProjectStoreThesaurus } from "../store/types";

export interface ThesaurusSchemeTree {
  id: string;
  label: string;
  concepts: ThesaurusConceptItem[];
  byId: ReadonlyMap<string, ThesaurusConceptItem>;
  roots: ThesaurusConceptItem[];
  childrenById: ReadonlyMap<string, ThesaurusConceptItem[]>;
}

export interface ThesaurusIndex {
  concepts: ThesaurusConceptItem[];
  schemes: ThesaurusSchemeTree[];
  parentById: ReadonlyMap<string, string | null>;
  depthById: ReadonlyMap<string, number>;
}

// Project Store projections are immutable. Weak keys release indexes when the
// owning snapshot is no longer retained; both panes share the same preparation.
const prepared = new WeakMap<ProjectStoreThesaurus, ThesaurusIndex>();

export function prepareThesaurus(thesaurus: ProjectStoreThesaurus): ThesaurusIndex {
  const cached = prepared.get(thesaurus);
  if (cached) return cached;
  const schemeById = new Map(thesaurus.schemes.map(scheme => [scheme.id, scheme]));
  const conceptById = new Map(thesaurus.concepts.map(concept => [concept.id, concept]));
  const parentById = new Map<string, string | null>();
  for (const concept of thesaurus.concepts) {
    const parent = concept.parent_id;
    parentById.set(concept.id, parent && conceptById.get(parent)?.scheme_id === concept.scheme_id ? parent : null);
  }
  const depthById = new Map<string, number>();
  for (const id of parentById.keys()) {
    if (depthById.has(id)) continue;
    let path: string[] = [];
    const positions = new Map<string, number>();
    let current: string | null = id;
    while (current !== null && !depthById.has(current) && !positions.has(current)) {
      positions.set(current, path.length);
      path.push(current);
      current = parentById.get(current) ?? null;
    }
    if (current !== null && positions.has(current)) {
      // Defensive display forest for malformed external input; never mutate the
      // canonical projection. Choose a stable cycle root independent of ordering.
      const cycle = path.slice(positions.get(current));
      const cycleRoot = cycle.reduce((smallest, node) => node < smallest ? node : smallest);
      parentById.set(cycleRoot, null);
      path = [];
      current = id;
      while (current !== null && !depthById.has(current)) {
        path.push(current);
        current = parentById.get(current) ?? null;
      }
    }
    let depth = current === null ? -1 : depthById.get(current)!;
    for (let i = path.length - 1; i >= 0; i -= 1) depthById.set(path[i], ++depth);
  }
  const concepts: ThesaurusConceptItem[] = thesaurus.concepts.map(concept => {
    const scheme = schemeById.get(concept.scheme_id);
    return {
      id: concept.id, label: concept.label, schemeId: concept.scheme_id,
      schemeLabel: concept.scheme_label || scheme?.label || "Thesaurus",
      schemeSourceElementId: scheme?.element_id || concept.scheme_element_id || null,
      parentId: parentById.get(concept.id) ?? null,
      depth: Math.min(depthById.get(concept.id) ?? 0, 2),
      definition: concept.definition, altLabels: concept.alt_labels, scopeNote: concept.scope_note,
      relatedIds: concept.related_ids, usedBy: concept.used_by, mapsTo: concept.maps_to,
      sourceElementId: concept.element_id, sourceHref: concept.source_href, sourceLabel: concept.source_label,
    };
  }).sort((left, right) => left.depth - right.depth || left.label.localeCompare(right.label));
  const groups = new Map<string, ThesaurusConceptItem[]>();
  for (const concept of concepts) {
    let group = groups.get(concept.schemeId);
    if (!group) { group = []; groups.set(concept.schemeId, group); }
    group.push(concept);
  }
  const schemes = [...groups].map(([id, group]) => schemeTree(id, group[0].schemeLabel,
    group.sort((left, right) => depthById.get(left.id)! - depthById.get(right.id)! || left.label.localeCompare(right.label)),
  )).sort((left, right) => left.label.localeCompare(right.label));
  const index = { concepts, schemes, parentById, depthById };
  prepared.set(thesaurus, index);
  return index;
}

function schemeTree(id: string, label: string, concepts: ThesaurusConceptItem[]): ThesaurusSchemeTree {
  const byId = new Map(concepts.map(concept => [concept.id, concept]));
  const childrenById = new Map<string, ThesaurusConceptItem[]>();
  const roots: ThesaurusConceptItem[] = [];
  for (const concept of concepts) {
    if (!concept.parentId || !byId.has(concept.parentId)) { roots.push(concept); continue; }
    let children = childrenById.get(concept.parentId);
    if (!children) { children = []; childrenById.set(concept.parentId, children); }
    children.push(concept);
  }
  return { id, label, concepts, byId, childrenById, roots };
}

export function thesaurusAncestorIds(index: ThesaurusIndex, selectedId: string | null): ReadonlySet<string> {
  const path = new Set<string>();
  let current = selectedId;
  while (current !== null && index.parentById.has(current) && !path.has(current)) {
    path.add(current);
    current = index.parentById.get(current) ?? null;
  }
  return path;
}

export function filterThesaurusSchemes(schemes: ThesaurusSchemeTree[], query: string): ThesaurusSchemeTree[] {
  const normalized = query.trim().toLowerCase();
  if (!normalized) return schemes;
  return schemes.flatMap(scheme => {
    const included = new Set<string>();
    for (const concept of scheme.concepts) {
      if (!concept.label.toLowerCase().includes(normalized)
          && !(concept.definition || concept.scopeNote).toLowerCase().includes(normalized)) continue;
      let current: ThesaurusConceptItem | undefined = concept;
      while (current && !included.has(current.id)) {
        included.add(current.id);
        current = current.parentId ? scheme.byId.get(current.parentId) : undefined;
      }
    }
    return included.size ? [schemeTree(scheme.id, scheme.label, scheme.concepts.filter(concept => included.has(concept.id)))] : [];
  });
}
