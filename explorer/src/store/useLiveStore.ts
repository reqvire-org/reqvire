import { useCallback, useEffect, useRef, useState } from "react";
import { loadStore, loadStoreCandidate } from "./loadStore";
import { devFixture } from "./devFixture";

const REFRESH_INTERVAL_MS = 3000;

export function useLiveStore() {
  const [result, setResult] = useState(() => loadStore(devFixture));
  const [live] = useState(() => window.reqvireLiveRefresh);
  const [refreshError, setRefreshError] = useState<string | null>(null);
  const [refreshing, setRefreshing] = useState(false);
  const revisionRef = useRef(live?.revision);
  const requestRef = useRef<AbortController | null>(null);
  const mountedRef = useRef(false);

  const refresh = useCallback(async (force = false) => {
    if (!live || requestRef.current || !mountedRef.current) return;
    const controller = new AbortController();
    requestRef.current = controller;
    setRefreshing(true);
    try {
      const response = await fetch(`/api/project-store${force ? "?refresh=true" : ""}`, {
        cache: "no-store",
        headers: revisionRef.current ? { "If-None-Match": `"${revisionRef.current}"` } : {},
        signal: AbortSignal.any([controller.signal, AbortSignal.timeout(15000)]),
      });
      if (response.status === 304) {
        if (mountedRef.current && !controller.signal.aborted) setRefreshError(null);
        return;
      }
      const payload: unknown = await response.json();
      if (typeof payload !== "object" || payload === null) throw new Error("Invalid live store response.");
      const value = payload as { revision?: unknown; store?: unknown; error?: unknown };
      if (!response.ok) {
        throw new Error(typeof value.error === "string" ? value.error : `Server returned HTTP ${response.status}.`);
      }
      if (typeof value.revision !== "string" || !value.revision) throw new Error("Missing live store revision.");
      const next = loadStoreCandidate(value.store);
      if (!next.ok) throw new Error(next.detail ?? next.reason);
      if (next.schemaMismatch) throw new Error(next.schemaMismatch);
      if (!mountedRef.current || controller.signal.aborted) return;
      if (revisionRef.current !== value.revision) {
        revisionRef.current = value.revision;
        window.reqvireProjectStore = next.store;
        window.reqvireLiveRefresh = { revision: value.revision };
        setResult(next);
      }
      setRefreshError(null);
    } catch (error: unknown) {
      if (mountedRef.current && !controller.signal.aborted) {
        setRefreshError(error instanceof Error ? error.message : String(error));
      }
    } finally {
      if (requestRef.current === controller) {
        requestRef.current = null;
        if (mountedRef.current) setRefreshing(false);
      }
    }
  }, [live]);

  useEffect(() => {
    mountedRef.current = true;
    if (!live) return () => { mountedRef.current = false; };
    const checkVisible = () => {
      if (document.visibilityState === "visible") void refresh();
      else requestRef.current?.abort();
    };
    checkVisible();
    const timer = window.setInterval(checkVisible, REFRESH_INTERVAL_MS);
    document.addEventListener("visibilitychange", checkVisible);
    return () => {
      mountedRef.current = false;
      window.clearInterval(timer);
      document.removeEventListener("visibilitychange", checkVisible);
      requestRef.current?.abort();
      requestRef.current = null;
    };
  }, [live, refresh]);

  return { result, live: Boolean(live), refresh, refreshing, refreshError };
}
