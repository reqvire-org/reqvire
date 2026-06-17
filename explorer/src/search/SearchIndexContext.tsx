import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { useStore } from "../store/StoreContext";
import type { ProjectStoreSearchDocument } from "../store/types";
import type { SearchKind } from "./searchKinds";
import { displaySearchKind, type ProjectSearchDocument } from "../lib/searchIndex";

type SearchIndexStatus = "building" | "ready" | "error";

interface SearchIndexContextValue {
  status: SearchIndexStatus;
  error: string | null;
  documentCount: number;
  search: (
    query: string,
    enabledKinds: ReadonlySet<SearchKind>,
    enabledElementTypes: ReadonlySet<string>,
  ) => Promise<ProjectStoreSearchDocument[]>;
}

type WorkerResponse =
  | { type: "ready"; documentCount: number }
  | { type: "results"; requestId: number; results: ProjectStoreSearchDocument[] }
  | { type: "error"; requestId?: number; message: string };

const SearchIndexContext = createContext<SearchIndexContextValue | null>(null);

export function SearchIndexProvider({ children }: { children: ReactNode }) {
  const { store } = useStore();
  const [status, setStatus] = useState<SearchIndexStatus>("building");
  const [error, setError] = useState<string | null>(null);
  const [documentCount, setDocumentCount] = useState(0);
  const workerRef = useRef<Worker | null>(null);
  const requestIdRef = useRef(0);
  const pendingRef = useRef(
    new Map<number, {
      resolve: (results: ProjectStoreSearchDocument[]) => void;
      reject: (error: Error) => void;
    }>(),
  );

  const searchDocuments = useMemo<ProjectSearchDocument[]>(() => {
    const resourceByTarget = new Map<string, unknown>();
    for (const resource of store.resources) {
      resourceByTarget.set(resource.target, resource);
      if (resource.file_path) {
        resourceByTarget.set(resource.file_path, resource);
      }
    }
    const filesWithElements = new Set(store.elements.map((element) => element.file_path));
    const elementTypeById = new Map(store.elements.map((element) => [element.id, element.element_type]));

    return store.search.map((document) => ({
      ...document,
      displayKind: displaySearchKind(document, resourceByTarget, filesWithElements),
      elementType: document.kind === "element" ? elementTypeById.get(document.id) : undefined,
    }));
  }, [store.elements, store.resources, store.search]);

  useEffect(() => {
    setStatus("building");
    setError(null);
    setDocumentCount(0);

    const worker = new Worker(new URL("../workers/searchIndex.worker.ts", import.meta.url), {
      type: "module",
    });
    workerRef.current = worker;

    worker.onmessage = (event: MessageEvent<WorkerResponse>) => {
      const message = event.data;
      if (message.type === "ready") {
        setDocumentCount(message.documentCount);
        setStatus("ready");
        return;
      }

      if (message.type === "results") {
        const pending = pendingRef.current.get(message.requestId);
        if (pending) {
          pending.resolve(message.results);
          pendingRef.current.delete(message.requestId);
        }
        return;
      }

      if (message.type === "error") {
        if (message.requestId !== undefined) {
          const pending = pendingRef.current.get(message.requestId);
          if (pending) {
            pending.reject(new Error(message.message));
            pendingRef.current.delete(message.requestId);
          }
        } else {
          setError(message.message);
          setStatus("error");
        }
      }
    };

    worker.onerror = (event) => {
      setError(event.message);
      setStatus("error");
    };

    const buildHandle = window.setTimeout(() => {
      worker.postMessage({ type: "build", documents: searchDocuments });
    }, 0);

    return () => {
      window.clearTimeout(buildHandle);
      for (const pending of pendingRef.current.values()) {
        pending.reject(new Error("Search index worker was stopped."));
      }
      pendingRef.current.clear();
      worker.terminate();
      if (workerRef.current === worker) {
        workerRef.current = null;
      }
    };
  }, [searchDocuments]);

  const search = useCallback<SearchIndexContextValue["search"]>((query, enabledKinds, enabledElementTypes) => {
    if (status !== "ready" || !workerRef.current) {
      return Promise.resolve([]);
    }

    const requestId = requestIdRef.current + 1;
    requestIdRef.current = requestId;

    return new Promise((resolve, reject) => {
      pendingRef.current.set(requestId, { resolve, reject });
      workerRef.current?.postMessage({
        type: "search",
        requestId,
        query,
        enabledKinds: Array.from(enabledKinds),
        enabledElementTypes: Array.from(enabledElementTypes),
      });
    });
  }, [status]);

  const value = useMemo<SearchIndexContextValue>(
    () => ({ status, error, documentCount, search }),
    [documentCount, error, search, status],
  );

  return (
    <SearchIndexContext.Provider value={value}>
      {children}
    </SearchIndexContext.Provider>
  );
}

export function useSearchIndex() {
  const context = useContext(SearchIndexContext);
  if (!context) throw new Error("useSearchIndex must be used within SearchIndexProvider");
  return context;
}
