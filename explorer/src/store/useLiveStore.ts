import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { loadStore } from "./loadStore";
import { devFixture } from "./devFixture";
import { ManifestStoreClient } from "./manifestRefresh";

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
  const [result, setResult] = useState(() => loadStore(window.reqvireWorktreeRouting ? undefined : devFixture));
  const displayedRef = useRef(result.ok ? result.store.project.worktree_id : undefined);
  displayedRef.current = result.ok ? result.store.project.worktree_id : undefined;
  const [live] = useState(() => window.reqvireLiveRefresh);
  const [worktreeRouting] = useState(() => Boolean(window.reqvireWorktreeRouting));
  const [selectedWorktree, setSelectedWorktree] = useState(() => worktreeRouting
    ? requestedWorktree() ?? (result.ok ? result.store.project.worktree_id : undefined) : undefined);
  const [selectionVersion, setSelectionVersion] = useState(0);
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
  const refreshWorktrees = useCallback(async () => {
    if (!worktreeRouting) return;
    inventoryRequestRef.current?.abort();
    const controller = new AbortController(); inventoryRequestRef.current = controller;
    try {
      const response = await fetch("/api/worktrees", { cache: "no-store", signal: AbortSignal.any([controller.signal, AbortSignal.timeout(15000)]) });
      if (!response.ok) throw new Error("Branch choices are unavailable.");
      const data: unknown = await response.json();
      if (typeof data !== "object" || data === null || !("worktrees" in data) || !Array.isArray(data.worktrees)
        || !data.worktrees.every((item: unknown) => typeof item === "object" && item !== null
          && "worktree_id" in item && typeof item.worktree_id === "string" && "branch" in item && typeof item.branch === "string")) {
        throw new Error("Invalid branch inventory.");
      }
      if (!mountedRef.current || controller.signal.aborted) return;
      setWorktrees(data.worktrees as WorktreeChoice[]); setInventoryError(null);
    } catch (error) {
      if (mountedRef.current && !controller.signal.aborted) setInventoryError(error instanceof Error ? error.message : String(error));
    }
  }, [worktreeRouting]);
  const selectWorktree = useCallback((id: string) => {
    if (!worktreeRouting) return;
    requestRef.current?.abort(); requestRef.current = null;
    const url = new URL(window.location.href); url.searchParams.set("worktree_id", id);
    window.history.pushState(null, "", url);
    setSelectedWorktree(id);
    setSelectionVersion(version => version + 1);
  }, [worktreeRouting]);


  const refresh = useCallback(async (loadSelected = false): Promise<void> => {
    if ((!live && !worktreeRouting) || requestRef.current || !mountedRef.current) return;
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
      if (document.visibilityState === "visible") void refresh();
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
      requestRef.current?.abort(); requestRef.current = null;
      setSelectedWorktree(requestedWorktree());
    };
    window.addEventListener("popstate", onPopState);
    return () => { window.removeEventListener("popstate", onPopState); inventoryRequestRef.current?.abort(); };
  }, [worktreeRouting, refreshWorktrees]);

  return { result, automaticRefresh: Boolean(live), refreshError: refreshError ?? inventoryError, worktreeRouting, worktrees,
    recoveryWarning: recovery.required && recovery.context === displayedRef.current
      ? "MCP recovery required: showing the last accepted model; writes are disabled. Local file downloads are unavailable until the worktree is repaired and the server restarted."
      : null,
    selectedWorktree, selectWorktree, refreshWorktrees,
    switching: !refreshError && worktreeRouting && selectedWorktree !== undefined && (!result.ok || selectedWorktree !== result.store.project.worktree_id),
  };
}
