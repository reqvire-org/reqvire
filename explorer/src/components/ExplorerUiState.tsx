import { createContext, useContext, useMemo, useState, type ReactNode } from "react";

export type ContainmentMode = "list" | "sunburst" | "icicle";
export type ModelMode = "list" | "grid" | "sunburst" | "icicle";
export type TraceMode = "flow" | "rows";
export type GraphOverlayKey = "cross" | "verification" | "trace";
export type SearchKind = "file" | "element" | "resource" | "ontology" | "trace" | "coverage";
export type Kn2LayoutMode = "structural" | "concentric" | "breadthfirst" | "circle" | "grid";
export type Kn2ClusterMode = "structural" | "modularity";
export type Kn2RelationCategory = "derive" | "specify" | "refine";

export const MODEL_ROLE_TYPES = [
  "capability",
  "requirement",
  "refinement",
  "verification",
  "ontology",
  "resource",
  "other",
] as const;

export const MODEL_DEFAULT_TYPES = [
  "capability",
  "requirement",
  "refinement",
  "verification",
  "ontology",
  "other",
] as const;

export const ONTOLOGY_NODE_ROLES = [
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

export const ONTOLOGY_SHOW_FILTERS = [
  ["role", "ontology-term", "Terms", "class"],
  ["relation", "datatype-properties", "Datatype property links", "datatype-property"],
  ["relation", "object-properties", "Object property links", "object-property"],
  ["relation", "class-membership", "Class membership", "class"],
  ["relation", "class-disjointness", "Class disjointness", "disjoint"],
  ["relation", "restrictions", "Restrictions", "forall"],
  ["relation", "class-expressions", "Class expressions", "and"],
  ["role", "shacl-shape", "SHACL shapes", "node-shape"],
  ["role", "resource", "Resources", "resource"],
  ["role", "external-reference", "External refs", "resource"],
] as const;

export const ONTOLOGY_DEFAULT_FILTERS = [
  "ontology-term",
  "datatype-properties",
  "object-properties",
  "class-membership",
] as const;

export const SEARCH_KINDS = [
  "file",
  "element",
  "resource",
  "ontology",
  "trace",
  "coverage",
] as const satisfies readonly SearchKind[];

export const KN2_RELATIONS = ["derive", "specify", "refine"] as const satisfies readonly Kn2RelationCategory[];

interface ExplorerUiState {
  containmentMode: ContainmentMode;
  setContainmentMode: (mode: ContainmentMode) => void;
  modelMode: ModelMode;
  setModelMode: (mode: ModelMode) => void;
  traceMode: TraceMode;
  setTraceMode: (mode: TraceMode) => void;
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
  searchKinds: Set<SearchKind>;
  toggleSearchKind: (kind: SearchKind) => void;
  resetSearchKinds: () => void;
  kn2LayoutMode: Kn2LayoutMode;
  setKn2LayoutMode: (mode: Kn2LayoutMode) => void;
  kn2ClusterMode: Kn2ClusterMode;
  setKn2ClusterMode: (mode: Kn2ClusterMode) => void;
  kn2FocusRadius: number;
  setKn2FocusRadius: (radius: number) => void;
  kn2FocusOnly: boolean;
  setKn2FocusOnly: (enabled: boolean) => void;
  kn2LabelsEnabled: boolean;
  setKn2LabelsEnabled: (enabled: boolean) => void;
  kn2Relations: Set<Kn2RelationCategory>;
  toggleKn2Relation: (relation: Kn2RelationCategory) => void;
  kn2Overlays: Set<GraphOverlayKey>;
  toggleKn2Overlay: (overlay: GraphOverlayKey) => void;
}

const ExplorerUiStateContext = createContext<ExplorerUiState | null>(null);

export function ExplorerUiStateProvider({ children }: { children: ReactNode }) {
  const [containmentMode, setContainmentMode] = useState<ContainmentMode>("sunburst");
  const [modelMode, setModelMode] = useState<ModelMode>("grid");
  const [traceMode, setTraceMode] = useState<TraceMode>("flow");
  const [modelTypes, setModelTypes] = useState(() => new Set<string>(MODEL_DEFAULT_TYPES));
  const [modelOverlays, setModelOverlays] = useState<Set<GraphOverlayKey>>(() => new Set());
  const [ontologyRoles, setOntologyRoles] = useState(() => new Set<string>(ONTOLOGY_NODE_ROLES));
  const [ontologyFilters, setOntologyFilters] = useState(
    () => new Set<string>(ONTOLOGY_DEFAULT_FILTERS),
  );
  const [searchKinds, setSearchKinds] = useState(() => new Set<SearchKind>(SEARCH_KINDS));
  const [kn2LayoutMode, setKn2LayoutMode] = useState<Kn2LayoutMode>("structural");
  const [kn2ClusterMode, setKn2ClusterMode] = useState<Kn2ClusterMode>("structural");
  const [kn2FocusRadius, setKn2FocusRadius] = useState(1);
  const [kn2FocusOnly, setKn2FocusOnly] = useState(false);
  const [kn2LabelsEnabled, setKn2LabelsEnabled] = useState(true);
  const [kn2Relations, setKn2Relations] = useState(() => new Set<Kn2RelationCategory>(KN2_RELATIONS));
  const [kn2Overlays, setKn2Overlays] = useState<Set<GraphOverlayKey>>(() => new Set());

  const value = useMemo<ExplorerUiState>(
    () => ({
      containmentMode,
      setContainmentMode,
      modelMode,
      setModelMode,
      traceMode,
      setTraceMode,
      modelTypes,
      toggleModelType: (type) =>
        setModelTypes((current) => toggleSetValue(current, type)),
      resetModelTypes: () => setModelTypes(new Set(MODEL_DEFAULT_TYPES)),
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
      searchKinds,
      toggleSearchKind: (kind) =>
        setSearchKinds((current) => toggleSetValue(current, kind)),
      resetSearchKinds: () => setSearchKinds(new Set(SEARCH_KINDS)),
      kn2LayoutMode,
      setKn2LayoutMode,
      kn2ClusterMode,
      setKn2ClusterMode,
      kn2FocusRadius,
      setKn2FocusRadius: (radius) =>
        setKn2FocusRadius(Number.isFinite(radius) ? Math.max(1, Math.min(4, radius)) : 1),
      kn2FocusOnly,
      setKn2FocusOnly,
      kn2LabelsEnabled,
      setKn2LabelsEnabled,
      kn2Relations,
      toggleKn2Relation: (relation) =>
        setKn2Relations((current) => toggleSetValue(current, relation)),
      kn2Overlays,
      toggleKn2Overlay: (overlay) =>
        setKn2Overlays((current) => toggleSetValue(current, overlay)),
    }),
    [
      containmentMode,
      kn2ClusterMode,
      kn2FocusOnly,
      kn2FocusRadius,
      kn2LabelsEnabled,
      kn2LayoutMode,
      kn2Overlays,
      kn2Relations,
      modelMode,
      modelOverlays,
      modelTypes,
      ontologyFilters,
      ontologyRoles,
      searchKinds,
      traceMode,
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

function toggleSetValue<T>(set: Set<T>, value: T) {
  const next = new Set(set);
  if (next.has(value)) next.delete(value);
  else next.add(value);
  return next;
}
