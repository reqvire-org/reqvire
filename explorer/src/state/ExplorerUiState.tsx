import { createContext, useCallback, useContext, useEffect, useMemo, useState, type ReactNode, type Dispatch, type SetStateAction } from "react";
import { useStore } from "../store/StoreContext";
import { SEARCH_KINDS, type SearchKind } from "../search/searchKinds";
import { projectCoverage } from "../lib/coverage";
import type { CoverageProjection, ExplorerProjectStore } from "../store/types";

export type ModelMode = "list" | "grid" | "graph" | "flow";
export type ModelSelectionId = "__root__" | `folder:${string}` | `file:${string}` | string;
export type GraphOverlayKey = "cross" | "verification" | "trace";
export type CoverageSectionId =
  | "overview"
  | "capability-coverage"
  | "unverified-requirements"
  | "unimplemented-requirements"
  | "unsatisfied-verifications"
  | "orphaned-verifications";

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
  coverageSectionId: CoverageSectionId;
  setCoverageSectionId: (id: CoverageSectionId) => void;
  coverageScopeId: string | null;
  setCoverageScopeId: (id: string | null) => void;
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

export function ExplorerUiStateProvider({ children }: { children: ReactNode }) {
  const { store } = useStore();
  const coverageState = useCoverageState(store);
  const contextKey = store.project.worktree_id ?? store.project.workspace_root;
  const [navigationNotice, setNavigationNotice] = useContextSelection<string | null>(contextKey, null);
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
  const [modelMode, setModelMode] = useState<ModelMode>("grid");
  const [modelSelectionId, setModelSelectionId] = useContextSelection<ModelSelectionId>(contextKey, "__root__");
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
  const [knowledgeGraphSelectionId, setKnowledgeGraphSelectionId] = useContextSelection<string | null>(contextKey, null);
  const [ontologySelectionId, setOntologySelectionId] = useContextSelection<string | null>(contextKey, null);
  const [thesaurusSelectionId, setThesaurusSelectionId] = useContextSelection<string | null>(contextKey, null);
  const [thesaurusQuery, setThesaurusQuery] = useState("");
  const [coverageSectionId, setCoverageSectionId] = useState<CoverageSectionId>("overview");
  const [traceFilePath, setTraceFilePath] = useContextSelection<string | null>(contextKey, null);
  const [traceSelectionId, setTraceSelectionId] = useContextSelection<string | null>(contextKey, null);
  const [traceTreeQuery, setTraceTreeQuery] = useState("");

  useEffect(() => {
    if (!store.project.worktree_id) return;
    let cleared = false;
    const exists = (id: string) => store.elements.some(element => element.id === id);
    if (modelSelectionId !== "__root__" && !exists(modelSelectionId)
      && !store.resources.some(resource => resource.id === modelSelectionId)
      && !store.files.some(file => `file:${file.path}` === modelSelectionId)
      && !store.folders.some(folder => `folder:${folder.path}` === modelSelectionId)) {
      setModelSelectionId("__root__"); cleared = true;
    }
    if (traceSelectionId && !exists(traceSelectionId)) { setTraceSelectionId(null); cleared = true; }
    if (traceFilePath && !store.files.some(file => file.path === traceFilePath)) { setTraceFilePath(null); cleared = true; }
    if (thesaurusSelectionId && !store.thesaurus.concepts.some(concept => concept.id === thesaurusSelectionId)) { setThesaurusSelectionId(null); cleared = true; }
    if (knowledgeGraphSelectionId && !store.knowledge_graph.nodes?.some(node => node.id === knowledgeGraphSelectionId)) { setKnowledgeGraphSelectionId(null); cleared = true; }
    if (ontologySelectionId && !store.ontology.graph_data?.nodes?.some(node => node.id === ontologySelectionId)) { setOntologySelectionId(null); cleared = true; }
    if (cleared) setNavigationNotice("The previous selection is unavailable in this worktree. Select an available item to continue.");
  }, [store, modelSelectionId, setModelSelectionId, traceSelectionId, setTraceSelectionId, traceFilePath, setTraceFilePath, thesaurusSelectionId, setThesaurusSelectionId, knowledgeGraphSelectionId, setKnowledgeGraphSelectionId, ontologySelectionId, setOntologySelectionId, setNavigationNotice]);

  const value = useMemo<ExplorerUiState>(
    () => ({
      ...coverageState,
      navigationNotice,
      modelMode,
      setModelMode,
      modelSelectionId,
      setModelSelectionId,
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
      knowledgeGraphSelectionId,
      setKnowledgeGraphSelectionId,
      ontologySelectionId,
      setOntologySelectionId,
      thesaurusSelectionId,
      setThesaurusSelectionId,
      thesaurusQuery,
      setThesaurusQuery,
      coverageSectionId,
      setCoverageSectionId,
      traceFilePath,
      setTraceFilePath,
      traceSelectionId,
      setTraceSelectionId,
      traceTreeQuery,
      setTraceTreeQuery,
    }),
    [
      coverageState,
      navigationNotice,
      knowledgeGraphSelectionId,
      ontologySelectionId,
      thesaurusSelectionId,
      thesaurusQuery,
      coverageSectionId,
      traceFilePath,
      traceSelectionId,
      traceTreeQuery,
      modelMode,
      modelSelectionId,
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

function useContextSelection<T>(key: string, initial: T): [T, Dispatch<SetStateAction<T>>] {
  const [selections, setSelections] = useState<Record<string, T>>({});
  const set = useCallback((value: SetStateAction<T>) => setSelections(current => {
    const previous = current[key] ?? initial;
    const next = typeof value === "function" ? (value as (previous: T) => T)(previous) : value;
    return Object.is(previous, next) ? current : { ...current, [key]: next };
  }), [key, initial]);
  return [selections[key] ?? initial, set];
}

interface CoveragePreferences {
  scopeId: string | null;
  notice: string | null;
}

function readCoveragePreferences(key: string): CoveragePreferences {
  try {
    const stored = JSON.parse(localStorage.getItem(key) ?? "null");
    return {
      scopeId: typeof stored?.scopeId === "string" ? stored.scopeId : null,
      notice: null,
    };
  } catch { return { scopeId: null, notice: null }; }
}

function useCoverageState(store: ExplorerProjectStore) {
  const identity = [store.project.workspace_root, store.project.repository, store.project.name];
  if (store.project.worktree_id) identity.push(store.project.worktree_id);
  const projectKey = `reqvire:coverage:${JSON.stringify(identity)}`;
  const initial = useMemo(() => readCoveragePreferences(projectKey), [projectKey]);
  const [selections, setSelections] = useState<Record<string, CoveragePreferences>>({});
  const preferences = selections[projectKey] ?? initial;
  const missing = preferences.scopeId !== null && (!store.coverage.scope_index[preferences.scopeId]
    || !store.elements.some(element => element.id === preferences.scopeId && element.element_type === "capability"));
  const coverageScopeId = missing ? null : preferences.scopeId;
  const update = useCallback((patch: Partial<CoveragePreferences>) => {
    setSelections(current => {
      const next = { ...(current[projectKey] ?? initial), ...patch };
      try { localStorage.setItem(projectKey, JSON.stringify({ scopeId: next.scopeId })); } catch { /* Session state remains usable without storage. */ }
      return { ...current, [projectKey]: next };
    });
  }, [projectKey, initial]);
  const missingNotice = "The selected capability is no longer available. Showing Whole model.";
  useEffect(() => {
    if (missing) update({ scopeId: null, notice: missingNotice });
  }, [missing, update]);
  const setCoverageScopeId = useCallback((scopeId: string | null) => update({ scopeId, notice: null }), [update]);
  const coverageProjection = useMemo(() => projectCoverage(store.coverage, coverageScopeId), [store.coverage, coverageScopeId]);
  return useMemo(() => ({
    coverageScopeId, setCoverageScopeId, coverageProjection,
    coverageNotice: missing ? missingNotice : preferences.notice,
  }), [coverageScopeId, setCoverageScopeId, preferences.notice, coverageProjection, store.coverage.scope_index, missing]);
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
