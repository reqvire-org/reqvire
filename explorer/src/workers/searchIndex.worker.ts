import {
  createProjectSearchIndex,
  searchProjectDocuments,
  type ProjectSearchDocument,
  type ProjectSearchIndex,
} from "../lib/searchIndex";
import type { ProjectStoreSearchDocument } from "../store/types";
import type { SearchKind } from "../search/searchKinds";

type SearchWorkerRequest =
  | { type: "build"; documents: ProjectSearchDocument[] }
  | {
      type: "search";
      requestId: number;
      query: string;
      enabledKinds: SearchKind[];
      enabledElementTypes: string[];
    };

type SearchWorkerResponse =
  | { type: "ready"; documentCount: number }
  | { type: "results"; requestId: number; results: ProjectStoreSearchDocument[] }
  | { type: "error"; requestId?: number; message: string };

let searchIndex: ProjectSearchIndex | null = null;

self.onmessage = (event: MessageEvent<SearchWorkerRequest>) => {
  try {
    const message = event.data;
    if (message.type === "build") {
      searchIndex = createProjectSearchIndex(message.documents);
      post({ type: "ready", documentCount: message.documents.length });
      return;
    }

    if (!searchIndex) {
      post({ type: "error", requestId: message.requestId, message: "Search index is not ready." });
      return;
    }

    const results = searchProjectDocuments(
      searchIndex,
      message.query,
      new Set<SearchKind>(message.enabledKinds),
      new Set(message.enabledElementTypes),
    );
    post({ type: "results", requestId: message.requestId, results });
  } catch (error) {
    post({
      type: "error",
      requestId: event.data.type === "search" ? event.data.requestId : undefined,
      message: error instanceof Error ? error.message : String(error),
    });
  }
};

function post(message: SearchWorkerResponse) {
  self.postMessage(message);
}
