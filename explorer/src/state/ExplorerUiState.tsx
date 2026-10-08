import { createContext, useContext, useMemo, useState, type ReactNode } from "react";
import { useSelectionNavigation } from "./useSelectionNavigation";
import { useStore } from "../store/StoreContext";
import { SEARCH_KINDS, type SearchKind } from "../search/searchKinds";
import { useCoverageNavigation } from "./useCoverageNavigation";
import type { CoverageProjection } from "../store/types";

export type ModelMode = "list" | "grid" | "graph" | "flow";
export type ModelSelectionId = "__root__" | `folder:${string}` | `file:${string}` | string;
export type GraphOverlayKey = "cross" | "verification" | "trace";
export const MODEL_DEFAULT_OVERLAYS = ["cross", "verification", "trace"] as const;

export const ONTOLOGY_NODE_ROLES = [
  "semantic-query",
  "class",
  "object-property",
  "datatype-property",
  "property",
  "named-individual",
  "datatype",
  "restriction",
  "class-expression",
  "node-shape",
  "property-shape",
  "resource",
] as const;

export const ONTOLOGY_SHOW_FILTERS = [] as const;

export const ONTOLOGY_CONSTRUCT_FILTERS = [
  ["domain-range", "Domain/range", "D/R"],
  ["subclass", "Subclass", "⊆"],
  ["membership", "Membership", "∈"],
  ["disjoint", "Disjoint", "⟂"],
  ["equivalence", "Equivalence", "⇔"],
  ["inverse", "Inverse", "⟲"],
  ["property-chain", "Property chain", "∘"],
  ["property-characteristic", "Property char.", "→"],
  ["restriction", "Restriction", "∀"],
  ["class-expression", "Class expr.", "∩"],
  ["shape-overlay", "SHACL overlay", "SH"],
] as const;

export const ONTOLOGY_ORIGIN_FILTERS = [
  ["authored", "Defined", "authored"],
  ["registry", "Registry", "registry"],
  ["construct", "Constructs", "construct"],
] as const;

export const ONTOLOGY_LAYER_FILTERS = [
  ["layer-authored", "Ontologies", "authored"],
  ["layer-concepts", "Concepts", "concepts"],
  ["layer-reqvire-context", "Semantic Context", "semantic"],
  ["layer-external-source", "External Sources", "external"],
] as const;

export const ONTOLOGY_DEFAULT_FILTERS = [
  "semantic-query",
  "layer-authored",
  "layer-concepts",
  "ontology-term",
  "shacl-shape",
  "resource",
  "external-reference",
  "class-membership",
  "class-disjointness",
  "class-expressions",
  "domain-range",
  "subclass",
  "membership",
  "disjoint",
  "equivalence",
  "inverse",
  "property-chain",
  "property-characteristic",
  "class-expression",
  "shape-overlay",
  "authored",
  "registry",
  "construct",
] as const;

interface ExplorerUiState {
  navigationNotice: string | null;
  modelMode: ModelMode;
  setModelMode: (mode: ModelMode) => void;
  modelSelectionId: ModelSelectionId;
  setModelSelectionId: (id: ModelSelectionId) => void;
  modelTreeQuery: string;
  setModelTreeQuery: (query: string) => void;
  modelTypes: Set<string>;
  toggleModelType: (type: string) => void;
  resetModelTypes: () => void;
  modelOverlays: Set<GraphOverlayKey>;
  toggleModelOverlay: (overlay: GraphOverlayKey) => void;
  ontologyRoles: Set<string>;
  toggleOntologyRole: (role: string) => void;
  resetOntologyRoles: () => void;
  ontologyFilters: Set<string>;
  toggleOntologyFilter: (filter: string) => void;
  resetOntologyFilters: () => void;
  ontologyLayoutNonce: number;
  resetOntologyLayout: () => void;
  searchKinds: Set<SearchKind>;
  toggleSearchKind: (kind: SearchKind) => void;
  searchElementTypes: Set<string>;
  toggleSearchElementType: (type: string) => void;
  resetSearchKinds: () => void;
  knowledgeGraphSelectionId: string | null;
  setKnowledgeGraphSelectionId: (id: string | null) => void;
  ontologySelectionId: string | null;
  setOntologySelectionId: (id: string | null) => void;
  thesaurusSelectionId: string | null;
  setThesaurusSelectionId: (id: string | null) => void;
  thesaurusQuery: string;
  setThesaurusQuery: (query: string) => void;
  coverageScopeId: string | null;
  setCoverageScopeId: (id: string | null) => void;
  coverageTreeQuery: string;
  setCoverageTreeQuery: (query: string) => void;
  coverageProjection: CoverageProjection;
  coverageNotice: string | null;
  traceFilePath: string | null;
  setTraceFilePath: (path: string | null) => void;
  traceSelectionId: string | null;
  setTraceSelectionId: (id: string | null) => void;
  traceTreeQuery: string;
  setTraceTreeQuery: (query: string) => void;
}

const ExplorerUiStateContext = createContext<ExplorerUiState | null>(null);

export function ExplorerUiStateProvider({ children, worktreeRouting = false }: { children: ReactNode; worktreeRouting?: boolean }) {
  const { store } = useStore();
  const coverageState = useCoverageNavigation(store, worktreeRouting);
  const selectionState = useSelectionNavigation(worktreeRouting);
  const searchElementTypeKeys = useMemo(
    () => Array.from(new Set(store.elements.map((element) => element.element_type).filter(Boolean))).sort(),
    [store.elements],
  );
  const modelTypeKeys = useMemo(
    () =>
      Array.from(
        new Set(
          (store.knowledge_graph.nodes ?? [])
            .map((node) => node.element_type || node.node_type || node.type || "other")
            .filter(Boolean),
        ),
      ).sort(),
    [store.knowledge_graph.nodes],
  );
  const [modelTreeQuery, setModelTreeQuery] = useState("");
  const [modelTypes, setModelTypes] = useState(() => new Set<string>(modelTypeKeys));
  const [modelOverlays, setModelOverlays] = useState<Set<GraphOverlayKey>>(
    () => new Set(MODEL_DEFAULT_OVERLAYS),
  );
  const [ontologyRoles, setOntologyRoles] = useState(() => new Set<string>(ONTOLOGY_NODE_ROLES));
  const [ontologyFilters, setOntologyFilters] = useState(
    () => new Set<string>(ONTOLOGY_DEFAULT_FILTERS),
  );
  const [ontologyLayoutNonce, setOntologyLayoutNonce] = useState(0);
  const [searchKinds, setSearchKinds] = useState(() => new Set<SearchKind>(SEARCH_KINDS));
  const [searchElementTypes, setSearchElementTypes] = useState(() => new Set<string>(searchElementTypeKeys));
  const [thesaurusQuery, setThesaurusQuery] = useState("");
  const [traceTreeQuery, setTraceTreeQuery] = useState("");
  const [coverageTreeQuery, setCoverageTreeQuery] = useState("");

  const value = useMemo<ExplorerUiState>(
    () => ({
      ...coverageState,
      ...selectionState,
      navigationNotice: selectionState.selectionNotice,
      modelTreeQuery,
      setModelTreeQuery,
      modelTypes,
      toggleModelType: (type) =>
        setModelTypes((current) => toggleSetValue(current, type)),
      resetModelTypes: () => setModelTypes(new Set(modelTypeKeys)),
      modelOverlays,
      toggleModelOverlay: (overlay) =>
        setModelOverlays((current) => toggleSetValue(current, overlay)),
      ontologyRoles,
      toggleOntologyRole: (role) =>
        setOntologyRoles((current) => toggleSetValue(current, role)),
      resetOntologyRoles: () => setOntologyRoles(new Set(ONTOLOGY_NODE_ROLES)),
      ontologyFilters,
      toggleOntologyFilter: (filter) =>
        setOntologyFilters((current) => toggleSetValue(current, filter)),
      resetOntologyFilters: () => setOntologyFilters(new Set(ONTOLOGY_DEFAULT_FILTERS)),
      ontologyLayoutNonce,
      resetOntologyLayout: () => setOntologyLayoutNonce((value) => value + 1),
      searchKinds,
      toggleSearchKind: (kind) =>
        setSearchKinds((current) => toggleSetValue(current, kind)),
      searchElementTypes,
      toggleSearchElementType: (type) =>
        setSearchElementTypes((current) => toggleSetValue(current, type)),
      resetSearchKinds: () => {
        setSearchKinds(new Set(SEARCH_KINDS));
        setSearchElementTypes(new Set(searchElementTypeKeys));
      },
      thesaurusQuery,
      setThesaurusQuery,
      traceTreeQuery,
      setTraceTreeQuery,
      coverageTreeQuery,
      setCoverageTreeQuery,
    }),
    [
      coverageState,
      selectionState,
      thesaurusQuery,
      traceTreeQuery,
      coverageTreeQuery,
      modelTreeQuery,
      modelOverlays,
      modelTypeKeys,
      modelTypes,
      ontologyFilters,
      ontologyLayoutNonce,
      ontologyRoles,
      searchKinds,
      searchElementTypes,
      searchElementTypeKeys,
    ],
  );

  return (
    <ExplorerUiStateContext.Provider value={value}>
      {children}
    </ExplorerUiStateContext.Provider>
  );
}

export function useExplorerUiState() {
  const state = useContext(ExplorerUiStateContext);
  if (!state) throw new Error("Explorer UI state is missing");
  return state;
}

export function useOptionalExplorerUiState() {
  return useContext(ExplorerUiStateContext);
}

function toggleSetValue<T>(set: Set<T>, value: T) {
  const next = new Set(set);
  if (next.has(value)) next.delete(value);
  else next.add(value);
  return next;
}
