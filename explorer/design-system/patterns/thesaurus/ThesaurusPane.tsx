import { useEffect, useRef, useState } from "preact/hooks";
import { ElementIcon } from "../../components/data/ElementIcon";
import { PaneSearchForm } from "../side-pane/PaneControls";
import { PaneTree, PaneTreeNode } from "../side-pane/PaneTree";
import { PaneSummary } from "../side-pane/PaneSummary";
import { TreeItem } from "../../components/navigation/TreeItem";
import type { ThesaurusConceptItem } from "./ThesaurusExplorer";

export interface ThesaurusSchemeTree {
  id: string;
  label: string;
  concepts: ThesaurusConceptItem[];
  byId: ReadonlyMap<string, ThesaurusConceptItem>;
  roots: ThesaurusConceptItem[];
  childrenById: ReadonlyMap<string, ThesaurusConceptItem[]>;
}

export interface ThesaurusPaneProps {
  concepts: readonly ThesaurusConceptItem[];
  selectedId: string | null;
  query: string;
  onQueryChange: (query: string) => void;
  onSelectConcept: (id: string | null) => void;
}

function schemesFor(concepts: readonly ThesaurusConceptItem[]): ThesaurusSchemeTree[] {
  const groups = new Map<string, ThesaurusConceptItem[]>();
  for (const concept of concepts) {
    const group = groups.get(concept.schemeId) ?? [];
    group.push(concept); groups.set(concept.schemeId, group);
  }
  return [...groups].map(([id, group]) => {
    const byId = new Map(group.map(concept => [concept.id, concept]));
    const childrenById = new Map<string, ThesaurusConceptItem[]>();
    const roots: ThesaurusConceptItem[] = [];
    for (const concept of group) {
      if (!concept.parentId || !byId.has(concept.parentId)) roots.push(concept);
      else {
        const children = childrenById.get(concept.parentId) ?? [];
        children.push(concept); childrenById.set(concept.parentId, children);
      }
    }
    return { id, label: group[0].schemeLabel, concepts: group, byId, roots, childrenById };
  }).sort((a, b) => a.label.localeCompare(b.label));
}

/** The production concept filter, hierarchy, selection, and footer. */
export function ThesaurusPane({ concepts, selectedId, query, onQueryChange, onSelectConcept }: ThesaurusPaneProps) {
  const allSchemes = schemesFor(concepts);
  const byId = new Map(concepts.map(concept => [concept.id, concept]));
  const selectedPath = new Set<string>();
  let selected = selectedId;
  while (selected && byId.has(selected) && !selectedPath.has(selected)) {
    selectedPath.add(selected); selected = byId.get(selected)?.parentId ?? null;
  }
  const normalized = query.trim().toLowerCase();
  const included = new Set<string>();
  if (normalized) {
    for (const candidate of concepts) {
      if (!candidate.label.toLowerCase().includes(normalized)
          && !(candidate.definition || candidate.scopeNote).toLowerCase().includes(normalized)) continue;
      let current: ThesaurusConceptItem | undefined = candidate;
      while (current && !included.has(current.id)) {
        included.add(current.id); current = current.parentId ? byId.get(current.parentId) : undefined;
      }
    }
  }
  const filtered = normalized ? schemesFor(concepts.filter(concept => included.has(concept.id))) : allSchemes;
  return <>
    <PaneSearchForm searchInputId="thesaurus-search" inputLabel="Filter concepts"
      placeholder="Filter concepts..." value={query} resultsId="thesaurus-tree"
      onQueryChange={onQueryChange} onSubmit={event => event.preventDefault()} />
    <PaneTree aria-label="Concept hierarchy" id="thesaurus-tree">
      {filtered.map(scheme => <ThesaurusSchemeTreeNode key={scheme.id} scheme={scheme}
        selectedPath={selectedPath} selectedId={selectedId} query={query} onSelectConcept={onSelectConcept} />)}
    </PaneTree>
    <PaneSummary placement="footer" items={[
      { label: "Schemes", value: allSchemes.length.toLocaleString() },
      { label: "Concepts", value: concepts.length.toLocaleString() },
    ]} />
  </>;
}

function ThesaurusSchemeTreeNode({
  scheme,
  selectedPath,
  selectedId,
  query,
  onSelectConcept,
}: {
  scheme: ThesaurusSchemeTree;
  selectedPath: ReadonlySet<string>;
  selectedId: string | null;
  query: string;
  onSelectConcept: (id: string | null) => void;
}) {
  const hasSelectedDescendant = selectedId ? scheme.byId.has(selectedId) : false;
  const [open, setOpen] = useState(hasSelectedDescendant || scheme.concepts.length <= 8);

  useOpenWhenSelectionEnters(hasSelectedDescendant, setOpen);

  const children = scheme.roots;
  const expanded = Boolean(query.trim()) || open;

  return (
    <PaneTreeNode>
      <TreeItem
        kind="element"
        label={scheme.label}
        icon={<ElementIcon type="concept-scheme" size="sm" />}
        count={scheme.concepts.length}
        depth={0}
        open={expanded}
        expandable={children.length > 0}
        selected={false}
        onToggle={() => setOpen((value) => !value)}
        onSelect={() => {
          if (children.length > 0) {
            setOpen((value) => !value);
          }
          const first = children[0] ?? scheme.concepts[0];
          if (first) onSelectConcept(first.id);
        }}
      />
      {expanded && children.map((concept) => (
        <ThesaurusConceptTreeNode
          key={concept.id}
          concept={concept}
          childrenById={scheme.childrenById}
          selectedPath={selectedPath}
          selectedId={selectedId}
          query={query}
          onSelectConcept={onSelectConcept}
          depth={1}
        />
      ))}
    </PaneTreeNode>
  );
}

function ThesaurusConceptTreeNode({
  concept,
  childrenById,
  selectedPath,
  selectedId,
  query,
  onSelectConcept,
  depth,
}: {
  concept: ThesaurusConceptItem;
  childrenById: ReadonlyMap<string, ThesaurusConceptItem[]>;
  selectedPath: ReadonlySet<string>;
  selectedId: string | null;
  query: string;
  onSelectConcept: (id: string | null) => void;
  depth: number;
}) {
  const children = childrenById.get(concept.id) ?? [];
  const hasSelectedDescendant = selectedPath.has(concept.id);
  const [open, setOpen] = useState(depth < 2 || hasSelectedDescendant);
  const expanded = Boolean(query.trim()) || open;

  useOpenWhenSelectionEnters(hasSelectedDescendant, setOpen);

  return (
    <PaneTreeNode>
      <TreeItem
        kind="element"
        label={concept.label}
        icon={<ElementIcon type="concept" size="sm" />}
        count={children.length > 0 ? children.length : undefined}
        depth={depth}
        open={expanded}
        expandable={children.length > 0}
        selected={selectedId === concept.id}
        onToggle={() => setOpen((value) => !value)}
        onSelect={() => {
          if (children.length > 0) {
            setOpen((value) => !value);
          }
          onSelectConcept(concept.id);
        }}
        title={concept.definition || concept.scopeNote || concept.label}
      />
      {expanded && children.map((child) => (
        <ThesaurusConceptTreeNode
          key={child.id}
          concept={child}
          childrenById={childrenById}
          selectedPath={selectedPath}
          selectedId={selectedId}
          query={query}
          onSelectConcept={onSelectConcept}
          depth={depth + 1}
        />
      ))}
    </PaneTreeNode>
  );
}

function useOpenWhenSelectionEnters(
  hasSelectedDescendant: boolean,
  setOpen: (update: boolean) => void,
) {
  const previouslyHadSelectedDescendant = useRef(hasSelectedDescendant);

  useEffect(() => {
    if (hasSelectedDescendant && !previouslyHadSelectedDescendant.current) {
      setOpen(true);
    }
    previouslyHadSelectedDescendant.current = hasSelectedDescendant;
  }, [hasSelectedDescendant, setOpen]);
}
