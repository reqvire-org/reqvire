import { useEffect, useMemo } from "react";
import { ThesaurusExplorer, type ThesaurusConceptItem, type ThesaurusConceptUsage } from "@ds";
import { useStore } from "../store/StoreContext";
import { useExplorerUiState } from "../state/ExplorerUiState";
import type { ProjectStoreThesaurus, ProjectStoreThesaurusConcept } from "../store/types";

export function ThesaurusView({ onOpenElement }: { onOpenElement?: (id: string) => void }) {
  const { store } = useStore();
  const ui = useExplorerUiState();
  const { thesaurusSelectionId, setThesaurusSelectionId } = ui;
  const concepts = useMemo(() => buildThesaurusConcepts(store.thesaurus), [store.thesaurus]);
  const selectedId = thesaurusSelectionId;

  useEffect(() => {
    if (thesaurusSelectionId && concepts.some((concept) => concept.id === thesaurusSelectionId)) return;
    setThesaurusSelectionId(concepts[0]?.id ?? null);
  }, [concepts, setThesaurusSelectionId, thesaurusSelectionId]);

  return (
    <ThesaurusExplorer
      concepts={concepts}
      selectedId={selectedId}
      onSelectConcept={setThesaurusSelectionId}
      onOpenConcept={(id) => onOpenElement?.(id)}
    />
  );
}

function buildThesaurusConcepts(thesaurus: ProjectStoreThesaurus): ThesaurusConceptItem[] {
  if (!thesaurus) return [];
  const schemeById = new Map(thesaurus.schemes.map((scheme) => [scheme.id, scheme]));
  const conceptById = new Map(thesaurus.concepts.map((concept) => [concept.id, concept]));

  return thesaurus.concepts
    .map((concept) => {
      const scheme = schemeById.get(concept.scheme_id);
      return {
        id: concept.id,
        label: concept.label,
        schemeId: concept.scheme_id,
        schemeLabel: concept.scheme_label || scheme?.label || "Thesaurus",
        schemeSourceElementId: scheme?.element_id || concept.scheme_element_id || null,
        parentId: concept.parent_id,
        depth: conceptDepth(concept, conceptById),
        definition: concept.definition,
        altLabels: concept.alt_labels,
        scopeNote: concept.scope_note,
        relatedIds: concept.related_ids,
        usedBy: concept.used_by.map(thesaurusUsage),
        mapsTo: concept.maps_to.map(thesaurusUsage),
        sourceElementId: concept.element_id,
        sourceHref: concept.source_href,
        sourceLabel: concept.source_label,
      };
    })
    .sort((left, right) => left.depth - right.depth || left.label.localeCompare(right.label));
}

function thesaurusUsage(usage: { id: string; label: string; type: string }): ThesaurusConceptUsage {
  return {
    id: usage.id,
    label: usage.label,
    type: usage.type,
  };
}

function conceptDepth(
  concept: ProjectStoreThesaurusConcept,
  conceptById: ReadonlyMap<string, ProjectStoreThesaurusConcept>,
) {
  let depth = 0;
  let current = concept.parent_id;
  const seen = new Set<string>([concept.id]);
  while (current && !seen.has(current)) {
    seen.add(current);
    depth += 1;
    current = conceptById.get(current)?.parent_id ?? null;
  }
  return Math.min(depth, 2);
}
