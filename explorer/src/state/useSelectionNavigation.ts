import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useStore } from "../store/StoreContext";
import { DEFAULT_VIEW, parseHash, routeForSelection } from "../router/routes";
import { useExplorerLocation, writeExplorerUrl } from "../router/location";
import type { ModelMode } from "./ExplorerUiState";

type SelectionView = "model" | "traces" | "thesaurus" | "ontologies";
interface Selection { id: string | null; mode?: ModelMode; file?: string | null; graphId?: string | null }
interface Preferences {
  model?: Selection;
  traces?: Selection;
  thesaurus?: Selection;
  ontologies?: Selection;
  notice?: string | null;
}
const modes = new Set<ModelMode>(["list", "grid", "graph", "flow"]);
function readPreferences(key: string): Preferences {
  try {
    const stored = JSON.parse(localStorage.getItem(key) ?? "{}");
    const result: Preferences = {};
    for (const view of ["model", "traces", "thesaurus", "ontologies"] as const) {
      const value = stored?.[view];
      if (value && (value.id === null || typeof value.id === "string")) result[view] = {
        id: value.id, graphId: typeof value.graphId === "string" ? value.graphId : null, file: typeof value.file === "string" ? value.file : null,
        mode: modes.has(value.mode) ? value.mode : undefined,
      };
    }
    return result;
  } catch { return {}; }
}

/** URL precedence, history and context ownership shared by persistent view selections. */
export function useSelectionNavigation(worktreeRouting: boolean) {
  const { store, elementIndex, getTraceFiles } = useStore();
  const location = useExplorerLocation();
  const url = new URL(location);
  const route = parseHash(url.hash, DEFAULT_VIEW);
  const view = ["model", "traces", "thesaurus", "ontologies"].includes(route.view) && !route.elementId ? route.view as SelectionView : null;
  const requested = url.searchParams.get("worktree_id");
  const displayed = !worktreeRouting || !requested || requested === store.project.worktree_id;
  const key = `reqvire:selection:${JSON.stringify([store.project.workspace_root, store.project.repository, store.project.name, store.project.worktree_id])}`;
  const urlContext = useRef({ key, location });
  const sameContext = urlContext.current.key === key || urlContext.current.location !== location;
  if (displayed && urlContext.current.location !== location) urlContext.current = { key, location };
  const initial = useMemo(() => readPreferences(key), [key]);
  const [contexts, setContexts] = useState<Record<string, Preferences>>({});
  const [lastMode, setLastMode] = useState<ModelMode>("grid");
  const preferences = contexts[key] ?? initial;
  const query = view && displayed && sameContext && route.param !== null ? new URLSearchParams(route.param) : null;
  const explicit = Boolean(query && (query.has("selected") || query.has("file") || query.has("mode")));
  const input = (owner: SelectionView): Selection | undefined => view === owner && explicit ? {
    id: owner === "model" && query!.get("mode") === "graph" ? preferences.model?.id ?? null : query!.get("selected") || null,
    graphId: owner === "model" && query!.get("mode") === "graph" ? query!.get("selected") || null : preferences[owner]?.graphId ?? null, file: query!.get("file"),
    mode: modes.has(query!.get("mode") as ModelMode) ? query!.get("mode") as ModelMode : undefined,
  } : preferences[owner];
  const modelInput = input("model");
  const modelMode = modelInput?.mode ?? lastMode;
  const modelId = modelInput?.id ?? "__root__";
  const validModel = modelId === "__root__" || elementIndex.has(modelId)
    || store.resources.some(node => node.id === modelId)
    || store.files.some(file => `file:${file.path}` === modelId)
    || store.folders.some(folder => `folder:${folder.path}` === modelId)
    || (modelMode === "graph" && store.knowledge_graph.nodes?.some(node => node.id === modelId));
  const validGraph = !modelInput?.graphId || Boolean(store.knowledge_graph.nodes?.some(node => node.id === modelInput.graphId));
  const model: Selection = { id: validModel ? modelId : "__root__", mode: modelMode, graphId: validGraph ? modelInput?.graphId ?? null : null };

  // The shared lazy trace grouping stays unprepared outside the Traces route.
  const traceInput = input("traces");
  let traces: Selection = traceInput ?? { id: null, file: null };
  let validTrace = true;
  if (view === "traces" && displayed) {
    const files = getTraceFiles();
    const owner = traceInput?.id ? files.find(file => file.verifications.some(node => node.id === traceInput.id)) : undefined;
    validTrace = !traceInput || Boolean(traceInput.id ? owner : !traceInput.file || files.some(file => file.file === traceInput.file));
    traces = !traceInput ? { id: files[0]?.verifications[0]?.id ?? null, file: files[0]?.file ?? null }
      : owner ? { id: traceInput.id, file: owner.file }
        : { id: null, file: validTrace ? traceInput.file ?? files[0]?.file ?? null : files[0]?.file ?? null };
  }
  const thesaurusInput = input("thesaurus");
  const concept = store.thesaurus.concepts.find(node => node.id === thesaurusInput?.id || node.element_id === thesaurusInput?.id);
  const validThesaurus = !thesaurusInput?.id || Boolean(concept);
  const thesaurus: Selection = { id: !thesaurusInput ? store.thesaurus.concepts[0]?.id ?? null : concept?.id ?? null };
  const ontologyInput = input("ontologies");
  const validOntology = !ontologyInput?.id || Boolean(store.ontology.graph_data?.nodes?.some(node => node.id === ontologyInput.id));
  const ontologies: Selection = { id: validOntology ? ontologyInput?.id ?? null : null };
  const selections = { model, traces, thesaurus, ontologies };
  const selectionRef = useRef(selections);
  selectionRef.current = selections;
  const missing = !validModel || !validGraph || !validTrace || !validThesaurus || !validOntology;
  const missingNotice = "The selected item is unavailable in this worktree. Showing an available overview.";
  const canonical = useCallback((owner: SelectionView, selection: Selection) => routeForSelection(owner,
    owner === "model" && selection.mode === "graph" ? selection.graphId ?? null : owner === "thesaurus" ? store.thesaurus.concepts.find(node => node.id === selection.id)?.element_id ?? selection.id : selection.id,
    { mode: selection.mode, file: selection.file }), [store.thesaurus.concepts]);
  const update = useCallback((owner: SelectionView, selection: Selection, notice: string | null = null) => {
    setContexts(current => {
      const before = current[key] ?? initial;
      if (JSON.stringify(before[owner]) === JSON.stringify(selection) && (before.notice ?? null) === notice) return current;
      const next = { ...before, [owner]: selection, notice };
      try { localStorage.setItem(key, JSON.stringify({ ...next, notice: undefined })); } catch { /* Session navigation remains usable. */ }
      return { ...current, [key]: next };
    });
  }, [key, initial]);
  const active = view ? selections[view] : null;
  const activeSerialized = JSON.stringify(active);
  useEffect(() => {
    if (!displayed || !view || !active) return;
    if (JSON.stringify(selectionRef.current[view]) !== activeSerialized) return;
    const unchanged = JSON.stringify(preferences[view]) === activeSerialized;
    update(view, active, missing ? missingNotice : unchanged ? preferences.notice ?? null : null);
    const next = new URL(location);
    next.hash = canonical(view, active);
    if (worktreeRouting && store.project.worktree_id) next.searchParams.set("worktree_id", store.project.worktree_id);
    writeExplorerUrl(next, true);
    if (view === "model") setLastMode(modelMode);
  // Serialized domain selection avoids rerunning on equivalent immutable snapshots.
  }, [view, displayed, activeSerialized, location, update, missing, preferences.notice, worktreeRouting, store.project.worktree_id, canonical]);
  const navigationRef = useRef({ displayed, view, canonical });
  navigationRef.current = { displayed, view, canonical };
  const select = useCallback((owner: SelectionView, value: Selection | ((previous: Selection) => Selection)) => {
    const selection = typeof value === "function" ? value(selectionRef.current[owner]) : value;
    selectionRef.current = { ...selectionRef.current, [owner]: selection };
    update(owner, selection);
    const navigation = navigationRef.current;
    if (navigation.displayed && navigation.view === owner) {
      const next = new URL(window.location.href);
      next.hash = navigation.canonical(owner, selection);
      writeExplorerUrl(next);
    }
  }, [update]);
  const setModelMode = useCallback((mode: ModelMode) => { setLastMode(mode); select("model", previous => ({ ...previous, mode })); }, [select]);
  const setModelSelectionId = useCallback((id: string) => select("model", previous => ({ ...previous, id })), [select]);
  const setKnowledgeGraphSelectionId = useCallback((id: string | null) => select("model", previous => ({ ...previous, graphId: id })), [select]);
  const setTraceFilePath = useCallback((file: string | null) => select("traces", { id: null, file }), [select]);
  const setTraceSelectionId = useCallback((id: string | null) => {
    const owner = id ? getTraceFiles().find(file => file.verifications.some(node => node.id === id)) : undefined;
    select("traces", previous => ({ id, file: owner?.file ?? previous.file ?? null }));
  }, [getTraceFiles, select]);
  const setThesaurusSelectionId = useCallback((id: string | null) => select("thesaurus", { id }), [select]);
  const setOntologySelectionId = useCallback((id: string | null) => select("ontologies", { id }), [select]);
  return useMemo(() => ({
    selectionNotice: missing ? missingNotice : preferences.notice ?? null,
    modelMode, setModelMode,
    modelSelectionId: model.id!, setModelSelectionId,
    knowledgeGraphSelectionId: model.graphId ?? null,
    setKnowledgeGraphSelectionId,
    traceFilePath: traces.file ?? null,
    setTraceFilePath,
    traceSelectionId: traces.id,
    setTraceSelectionId,
    thesaurusSelectionId: thesaurus.id, setThesaurusSelectionId,
    ontologySelectionId: ontologies.id, setOntologySelectionId,
  }), [missing, preferences.notice, model.id, model.graphId, modelMode, traces.id, traces.file, thesaurus.id, ontologies.id, setModelMode, setModelSelectionId, setKnowledgeGraphSelectionId, setTraceFilePath, setTraceSelectionId, setThesaurusSelectionId, setOntologySelectionId]);
}
