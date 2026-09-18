import { useCallback, useEffect, useRef, useState } from "react";
import { loadStore } from "./loadStore";
import { devFixture } from "./devFixture";
import { ManifestStoreClient } from "./manifestRefresh";

const REFRESH_INTERVAL_MS = 5000;

export function useLiveStore() {
  const [result, setResult] = useState(() => loadStore(devFixture));
  const [live] = useState(() => window.reqvireLiveRefresh);
  const [client] = useState(() => new ManifestStoreClient(result.ok ? result.store : undefined, live));
  const [refreshError, setRefreshError] = useState<string | null>(null);
  const requestRef = useRef<AbortController | null>(null);
  const mountedRef = useRef(false);

  const refresh = useCallback(async (): Promise<void> => {
    if (!live || requestRef.current || !mountedRef.current) return;
    const controller = new AbortController();
    requestRef.current = controller;
    try {
      const next = await client.prepare(AbortSignal.any([controller.signal, AbortSignal.timeout(15000)]));
      if (!mountedRef.current || controller.signal.aborted) return;
      if (next) {
        client.commit(next);
        window.reqvireProjectStore = next.result.store;
        window.reqvireLiveRefresh = { revision: next.revision, manifest: next.manifest };
        setResult(next.result);
      }
      setRefreshError(null);
    } catch (error: unknown) {
      if (mountedRef.current && !controller.signal.aborted) {
        setRefreshError(error instanceof Error && error.name === "TimeoutError"
          ? "The server did not respond within 15 seconds."
          : error instanceof Error ? error.message : String(error));
      }
    } finally {
      if (requestRef.current === controller) {
        requestRef.current = null;
      }
    }
  }, [live, client]);

  useEffect(() => {
    mountedRef.current = true;
    if (!live) return () => { mountedRef.current = false; };
    const checkVisible = () => {
      if (document.visibilityState === "visible") void refresh();
      else {
        requestRef.current?.abort();
        requestRef.current = null;
      }
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

  return { result, refreshError };
}
