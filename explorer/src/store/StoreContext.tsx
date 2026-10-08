import { createContext, useContext, useMemo, type ReactNode } from "react";
import { prepareModelFlow, type ModelFlowIndex } from "../lib/modelFlow";
import { buildTraceFiles, type TraceFileNode } from "../lib/traces";
import { buildFileManagerModel, buildProjectFileTree, projectWorktrees, type FileManagerModel, type ProjectWorktree, type TreeFolder } from "../lib/fileTrees";
import type {
  ExplorerProjectStore,
  ProjectStoreElement,
} from "./types";

export interface StoreContextValue {
  store: ExplorerProjectStore;
  /** Non-null when the seed schema version differs from the expected one. */
  schemaMismatch: string | null;
  /** O(1) element lookup by full identifier. */
  elementById: (id: string) => ProjectStoreElement | undefined;
  elementIndex: ReadonlyMap<string, ProjectStoreElement>;
  getTraceFiles: () => TraceFileNode[];
  getModelFlowIndex: () => ModelFlowIndex;
  getProjectFileTree: () => TreeFolder;
  getFileManagerModel: () => FileManagerModel;
}

const StoreContext = createContext<StoreContextValue | null>(null);
const NO_ELEMENTS: ExplorerProjectStore["elements"] = [];
const NO_RELATIONS: ExplorerProjectStore["relations"] = [];

/** Each getter owns one result for exactly the inputs captured by its memo. */
function onDemand<T>(build: () => T): () => T {
  let cached: { value: T } | undefined;
  return () => (cached ??= { value: build() }).value;
}

export function StoreProvider({
  store,
  schemaMismatch,
  children,
}: {
  store: ExplorerProjectStore;
  schemaMismatch: string | null;
  children: ReactNode;
}) {
  const contextId = JSON.stringify([store.project.workspace_root, store.project.worktree_id]);
  const { elements, resources, relations, contract_bindings, contract_references, concept_refs, traces } = store;
  const elementIndex = useMemo(() => {
    const index = new Map<string, ProjectStoreElement>();
    for (const element of elements) {
      index.set(element.id, element);
    }
    return index;
  }, [contextId, elements]);
  const elementById = useMemo(() => (id: string) => elementIndex.get(id), [elementIndex]);

  // The canonical trace report is independent of element/relationship record changes.
  // Only its empty-report fallback derives directly from those records.
  const hasTraceFiles = useMemo(() => Object.keys(traces.files).length > 0, [traces]);
  const traceElements = hasTraceFiles ? NO_ELEMENTS : elements;
  const traceRelations = hasTraceFiles ? NO_RELATIONS : relations;
  const getTraceFiles = useMemo(() => onDemand(() => buildTraceFiles({
    traces, elements: traceElements, relations: traceRelations,
  })), [contextId, traces, traceElements, traceRelations]);
  const getModelFlowIndex = useMemo(() => onDemand(() => prepareModelFlow({
    elements, resources, relations, contract_bindings, contract_references, concept_refs,
  })), [contextId, elements, resources, relations, contract_bindings, contract_references, concept_refs]);

  const { files, folders } = store;
  const rootLabel = store.project.root_label;
  // Only normalized labels and path prefixes affect grouping, not Git HEAD/dirty metadata.
  // This small value key also handles a new project section with identical grouping.
  const worktreeGroupsKey = JSON.stringify(projectWorktrees(store.project));
  const getProjectFileTree = useMemo(() => onDemand(() => buildProjectFileTree(
    files, resources, JSON.parse(worktreeGroupsKey) as ProjectWorktree[],
  )), [contextId, files, resources, worktreeGroupsKey]);
  const getFileManagerModel = useMemo(() => onDemand(() => buildFileManagerModel(files, folders, rootLabel)),
    [contextId, files, folders, rootLabel]);

  const value = useMemo<StoreContextValue>(() => ({
    store, schemaMismatch, elementById, elementIndex, getTraceFiles, getModelFlowIndex, getProjectFileTree, getFileManagerModel,
  }), [store, schemaMismatch, elementById, elementIndex, getTraceFiles, getModelFlowIndex, getProjectFileTree, getFileManagerModel]);

  return <StoreContext.Provider value={value}>{children}</StoreContext.Provider>;
}

export function useStore(): StoreContextValue {
  const ctx = useContext(StoreContext);
  if (!ctx) {
    throw new Error("useStore must be used within a StoreProvider");
  }
  return ctx;
}
