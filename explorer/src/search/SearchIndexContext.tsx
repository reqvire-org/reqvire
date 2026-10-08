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

interface SearchWorkerSession {
  contextId: string;
  documents: ProjectSearchDocument[];
  worker: Worker;
  buildHandle?: number;
  ready: boolean;
  pending: Map<number, {
    resolve: (results: ProjectStoreSearchDocument[]) => void;
    reject: (error: Error) => void;
  }>;
}

// Compare only fields used by ranking, filtering, and returned results. Order also
// matters: it breaks ranking ties and determines the initial unfiltered results.
function sameSearchDocuments(left: ProjectSearchDocument[], right: ProjectSearchDocument[]): boolean {
  return left === right || (left.length === right.length && left.every((document, index) => {
    const other = right[index];
    return document.id === other.id && document.kind === other.kind && document.title === other.title &&
      document.route === other.route && document.text === other.text &&
      document.displayKind === other.displayKind && document.elementType === other.elementType;
  }));
}

function stopWorker(session: SearchWorkerSession, reason = new Error("Search index worker was stopped.")) {
  session.ready = false;
  window.clearTimeout(session.buildHandle);
  session.worker.onmessage = null;
  session.worker.onerror = null;
  session.worker.onmessageerror = null;
  session.worker.terminate();
  for (const pending of session.pending.values()) pending.reject(reason);
  session.pending.clear();
}

export function SearchIndexProvider({ children }: { children: ReactNode }) {
  const { store } = useStore();
  const contextId = JSON.stringify([store.project.workspace_root, store.project.worktree_id]);
  const [status, setStatus] = useState<SearchIndexStatus>("building");
  const [error, setError] = useState<string | null>(null);
  const [documentCount, setDocumentCount] = useState(0);
  const sessionRef = useRef<SearchWorkerSession | null>(null);
  const requestIdRef = useRef(0);

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
      id: document.id,
      kind: document.kind,
      title: document.title,
      route: document.route,
      text: document.text,
      displayKind: displaySearchKind(document, resourceByTarget, filesWithElements),
      elementType: document.kind === "element" ? elementTypeById.get(document.id) : undefined,
    }));
  }, [store.elements, store.resources, store.search]);

  useEffect(() => {
    const previous = sessionRef.current;
    if (previous?.contextId === contextId && sameSearchDocuments(previous.documents, searchDocuments)) {
      // Release the old projection while keeping its already-built worker index.
      previous.documents = searchDocuments;
      return;
    }
    sessionRef.current = null;
    if (previous) stopWorker(previous);
    setStatus("building");
    setError(null);
    setDocumentCount(0);

    let worker: Worker;
    try {
      worker = new Worker(new URL("../workers/searchIndex.worker.ts", import.meta.url), { type: "module" });
    } catch (error) {
      setError(error instanceof Error ? error.message : String(error));
      setStatus("error");
      return;
    }
    const session: SearchWorkerSession = {
      contextId, documents: searchDocuments, worker, ready: false, pending: new Map(),
    };
    sessionRef.current = session;
    const fail = (message: string) => {
      if (sessionRef.current !== session) return;
      sessionRef.current = null;
      stopWorker(session, new Error(message));
      setError(message);
      setStatus("error");
    };

    worker.onmessage = (event: MessageEvent<WorkerResponse>) => {
      if (sessionRef.current !== session) return;
      const message = event.data;
      if (message.type === "ready") {
        session.ready = true;
        setDocumentCount(message.documentCount);
        setStatus("ready");
        return;
      }

      if (message.type === "results") {
        const pending = session.pending.get(message.requestId);
        if (pending) {
          pending.resolve(message.results);
          session.pending.delete(message.requestId);
        }
        return;
      }

      if (message.type === "error") {
        if (message.requestId !== undefined) {
          const pending = session.pending.get(message.requestId);
          if (pending) {
            pending.reject(new Error(message.message));
            session.pending.delete(message.requestId);
          }
        } else {
          fail(message.message);
        }
      }
    };

    worker.onerror = (event) => fail(event.message);
    worker.onmessageerror = () => fail("Search index worker response could not be decoded.");

    session.buildHandle = window.setTimeout(() => {
      try {
        worker.postMessage({ type: "build", documents: session.documents });
      } catch (error) {
        fail(error instanceof Error ? error.message : String(error));
      }
    }, 0);
  }, [contextId, searchDocuments]);

  // Input replacement retires workers above; this cleanup belongs to the
  // provider lifetime so equivalent immutable input replacements keep the worker.
  useEffect(() => () => {
    const session = sessionRef.current;
    sessionRef.current = null;
    if (session) stopWorker(session);
  }, []);

  const search = useCallback<SearchIndexContextValue["search"]>((query, enabledKinds, enabledElementTypes) => {
    const session = sessionRef.current;
    if (status !== "ready" || !session?.ready || session.contextId !== contextId) {
      return Promise.resolve([]);
    }

    const requestId = requestIdRef.current + 1;
    requestIdRef.current = requestId;

    return new Promise((resolve, reject) => {
      session.pending.set(requestId, { resolve, reject });
      try {
        session.worker.postMessage({
          type: "search", requestId, query,
          enabledKinds: Array.from(enabledKinds),
          enabledElementTypes: Array.from(enabledElementTypes),
        });
      } catch (error) {
        session.pending.delete(requestId);
        reject(error instanceof Error ? error : new Error(String(error)));
      }
    });
  }, [contextId, status]);

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
