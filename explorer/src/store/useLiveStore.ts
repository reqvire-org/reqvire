import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { loadStore } from "./loadStore";
import { devFixture } from "./devFixture";
import { ManifestStoreClient } from "./manifestRefresh";
import { worktreeUrl } from "../router/routes";
import { useExplorerLocation, writeExplorerUrl } from "../router/location";

const REFRESH_INTERVAL_MS = 5000;
export interface WorktreeChoice {
  worktree_id: string;
  branch: string;
  workspace_root: string;
  available: boolean;
  explorer_available: boolean;
  owned?: boolean;
  explorer_diagnostic?: string | null;
}
function requestedWorktree() { return new URL(window.location.href).searchParams.get("worktree_id") ?? undefined; }


export function useLiveStore() {
  const location = useExplorerLocation();
  const [result, setResult] = useState(() => loadStore(window.reqvireWorktreeRouting ? undefined : devFixture));
  const displayedRef = useRef(result.ok ? result.store.project.worktree_id : undefined);
  displayedRef.current = result.ok ? result.store.project.worktree_id : undefined;
  const retainedLocation = useRef(location);
  if (result.ok && (!requestedWorktree() || requestedWorktree() === displayedRef.current)) retainedLocation.current = location;
  const failedSelection = useRef<string | null>(null);
  const [live] = useState(() => window.reqvireLiveRefresh);
  const [worktreeRouting] = useState(() => Boolean(window.reqvireWorktreeRouting));
  const [selectedWorktree, setSelectedWorktree] = useState(() => worktreeRouting
    ? requestedWorktree() ?? (result.ok ? result.store.project.worktree_id : undefined) : undefined);
  const [selectionVersion, setSelectionVersion] = useState(0);
  const selectedWorktreeRef = useRef(selectedWorktree);
  selectedWorktreeRef.current = selectedWorktree;
  const [worktrees, setWorktrees] = useState<WorktreeChoice[]>([]);
  const [inventoryError, setInventoryError] = useState<string | null>(null);
  const [clients] = useState(() => new Map<string | undefined, ManifestStoreClient>([
    [result.ok ? result.store.project.worktree_id : undefined,
      new ManifestStoreClient(result.ok ? result.store : undefined, live, result.ok ? result.store.project.worktree_id : undefined)],
  ]));
  const client = useMemo(() => {
    let found = clients.get(selectedWorktree);
    if (!found) { found = new ManifestStoreClient(undefined, undefined, selectedWorktree); clients.set(selectedWorktree, found); }
    return found;
  }, [clients, selectedWorktree]);
  const [refreshError, setRefreshError] = useState<string | null>(null);
  const [recovery, setRecovery] = useState<{ context?: string; required: boolean }>({ required: false });
  const requestRef = useRef<AbortController | null>(null);
  const mountedRef = useRef(false);
  const inventoryRequestRef = useRef<AbortController | null>(null);
  const inventoryRef = useRef<{ ids: Set<string>; observedClients: WeakSet<ManifestStoreClient> } | null>(null);
  const refreshWorktrees = useCallback(async () => {
    if (!worktreeRouting) return;
    inventoryRequestRef.current?.abort();
    const controller = new AbortController(); inventoryRequestRef.current = controller;
    // An inventory cannot establish removal of a client created after it began.
    // Weak identities also let evicted clients and their snapshots be collected.
    const observedClients = new WeakSet(clients.values());
    try {
      const signal = AbortSignal.any([controller.signal, AbortSignal.timeout(15000)]);
      const response = await fetch("/api/worktrees", { cache: "no-store", signal });
      if (!response.ok) throw new Error("Branch choices are unavailable.");
      const data: unknown = await response.json();
      signal.throwIfAborted();
      if (typeof data === "object" && data !== null && "error" in data) {
        throw new Error(typeof data.error === "string" ? data.error : "Branch choices are unavailable.");
      }
      if (typeof data !== "object" || data === null || !("worktrees" in data) || !Array.isArray(data.worktrees)
        || !("original_worktree_id" in data) || typeof data.original_worktree_id !== "string" || !data.original_worktree_id
        || !data.worktrees.every((item: unknown) => typeof item === "object" && item !== null
          && "worktree_id" in item && typeof item.worktree_id === "string" && item.worktree_id
          && "branch" in item && typeof item.branch === "string")) {
        throw new Error("Invalid branch inventory.");
      }
      const ids = new Set((data.worktrees as WorktreeChoice[]).map(item => item.worktree_id));
      if (ids.size !== data.worktrees.length) throw new Error("Invalid branch inventory.");
      if (!mountedRef.current || controller.signal.aborted) return;
      inventoryRef.current = { ids, observedClients };
      setWorktrees(data.worktrees as WorktreeChoice[]); setInventoryError(null);
    } catch (error) {
      if (mountedRef.current && !controller.signal.aborted) setInventoryError(error instanceof Error ? error.message : String(error));
    }
  }, [clients, worktreeRouting]);
  const selectWorktree = useCallback((id: string) => {
    if (!worktreeRouting) return;
    requestRef.current?.abort(); requestRef.current = null;
    failedSelection.current = null;
    writeExplorerUrl(worktreeUrl(window.location.href, id));
    setRefreshError(null);
    setSelectedWorktree(id);
    setSelectionVersion(version => version + 1);
  }, [worktreeRouting]);

  const dismissWorktreeError = useCallback(() => {
    const retained = displayedRef.current;
    if (!retained) return;
    requestRef.current?.abort(); requestRef.current = null;
    failedSelection.current = null;
    const url = new URL(retainedLocation.current);
    url.searchParams.set("worktree_id", retained);
    writeExplorerUrl(url, true);
    setSelectedWorktree(retained);
    setRefreshError(null);
    setSelectionVersion(version => version + 1);
  }, []);


  const refresh = useCallback(async (loadSelected = false): Promise<void> => {
    if ((!live && !worktreeRouting) || requestRef.current || !mountedRef.current) return;
    if (selectedWorktree && failedSelection.current === selectedWorktree) return;
    const controller = new AbortController();
    requestRef.current = controller;
    try {
      const signal = AbortSignal.any([controller.signal, AbortSignal.timeout(15000)]);
      if (loadSelected && worktreeRouting && selectedWorktree) {
        const response = await fetch("/api/worktrees/load", { method: "POST", cache: "no-store", signal,
          headers: { "Content-Type": "application/json" }, body: JSON.stringify({ worktree_id: selectedWorktree }) });
        if (!response.ok) {
          const failure = await response.json().catch(() => null);
          throw new Error(failure?.error ?? "Selected branch could not be loaded.");
        }
      }
      const next = await client.prepare(signal, worktreeRouting && selectedWorktree !== displayedRef.current);
      if (!mountedRef.current || controller.signal.aborted) return;
      if (next) {
        client.commit(next);
        window.reqvireProjectStore = next.result.store;
        if (live) window.reqvireLiveRefresh = { revision: next.revision, manifest: next.manifest };
        setResult(next.result);
        if (worktreeRouting && !selectedWorktree) setSelectedWorktree(next.result.store.project.worktree_id);
      }
      setRecovery({ context: next?.result.store.project.worktree_id ?? selectedWorktree ?? displayedRef.current,
        required: client.recoveryRequired });
      setRefreshError(null);
    } catch (error: unknown) {
      if (mountedRef.current && !controller.signal.aborted) {
        if (selectedWorktree && selectedWorktree !== displayedRef.current) failedSelection.current = selectedWorktree;
        setRefreshError(error instanceof Error && error.name === "TimeoutError"
          ? "The server did not respond within 15 seconds."
          : error instanceof Error ? error.message : String(error));
        void refreshWorktrees();
      }
    } finally {
      if (requestRef.current === controller) {
        requestRef.current = null;
      }
    }
  }, [live, client, worktreeRouting, refreshWorktrees, selectedWorktree]);

  useEffect(() => {
    mountedRef.current = true;
    if (!live && !worktreeRouting) return () => { mountedRef.current = false; };
    const checkVisible = () => {
      if (document.visibilityState === "visible") void refresh(worktreeRouting && selectedWorktree !== displayedRef.current);
      else {
        requestRef.current?.abort();
        requestRef.current = null;
      }
    };
    if (document.visibilityState === "visible") void refresh(worktreeRouting);
    const timer = live ? window.setInterval(checkVisible, REFRESH_INTERVAL_MS) : undefined;
    document.addEventListener("visibilitychange", checkVisible);
    return () => {
      mountedRef.current = false;
      if (timer !== undefined) window.clearInterval(timer);
      document.removeEventListener("visibilitychange", checkVisible);
      requestRef.current?.abort();
      requestRef.current = null;
    };
  }, [live, refresh, worktreeRouting, selectionVersion]);

  useEffect(() => {
    if (!worktreeRouting) return;
    void refreshWorktrees();
    const onPopState = () => {
      if (requestedWorktree() === selectedWorktreeRef.current) return;
      requestRef.current?.abort(); requestRef.current = null;
      failedSelection.current = null;
      setRefreshError(null);
      setSelectedWorktree(requestedWorktree());
    };
    window.addEventListener("popstate", onPopState);
    return () => { window.removeEventListener("popstate", onPopState); inventoryRequestRef.current?.abort(); };
  }, [worktreeRouting, refreshWorktrees]);

  useEffect(() => {
    const inventory = inventoryRef.current;
    if (!inventory) return;
    for (const [id, cachedClient] of clients) {
      if (id === displayedRef.current || id === selectedWorktree) continue;
      if ((id === undefined || !inventory.ids.has(id)) && inventory.observedClients.has(cachedClient)) {
        clients.delete(id);
      }
    }
  }, [clients, result, selectedWorktree, worktrees]);

  return { result, automaticRefresh: Boolean(live), refreshError: refreshError ?? inventoryError, worktreeRouting, worktrees,
    recoveryWarning: recovery.required && recovery.context === displayedRef.current
      ? "MCP recovery required: showing the last accepted model; writes are disabled. Local file downloads are unavailable until the worktree is repaired and the server restarted."
      : null,
    selectedWorktree, selectWorktree, refreshWorktrees,
    dismissWorktreeError,
    worktreeSelectionError: worktreeRouting && selectedWorktree !== undefined && selectedWorktree !== displayedRef.current ? refreshError : null,
    switching: !refreshError && worktreeRouting && selectedWorktree !== undefined && (!result.ok || selectedWorktree !== result.store.project.worktree_id),
  };
}
