import MiniSearch from "minisearch";
import type { ProjectStoreSearchDocument } from "../store/types";
import type { SearchKind } from "../components/ExplorerUiState";

const EMPTY_RESULT_LIMIT = 50;
const QUERY_RESULT_LIMIT = 100;

export interface ProjectSearchDocument extends ProjectStoreSearchDocument {
  displayKind: string;
  elementType?: string;
}

interface IndexedSearchDocument extends ProjectSearchDocument {
  searchId: string;
  pathText: string;
  ordinal: number;
}

interface StoredSearchResult extends ProjectSearchDocument {
  searchId: string;
  pathText: string;
  ordinal: number;
  score?: number;
}

export interface ProjectSearchIndex {
  index: MiniSearch<IndexedSearchDocument>;
  documents: IndexedSearchDocument[];
}

export function createProjectSearchIndex(
  documents: ProjectSearchDocument[],
): ProjectSearchIndex {
  const indexedDocuments = documents.map((document, index) => ({
    ...document,
    searchId: `${document.displayKind}:${document.kind}:${document.id}:${index}`,
    pathText: searchPathText(document),
    ordinal: index,
  }));

  const index = new MiniSearch<IndexedSearchDocument>({
    idField: "searchId",
    fields: ["title", "pathText", "displayKind", "text"],
    storeFields: [
      "id",
      "kind",
      "title",
      "route",
      "text",
      "displayKind",
      "elementType",
      "pathText",
      "ordinal",
    ],
    searchOptions: {
      boost: {
        title: 7,
        pathText: 4,
        displayKind: 2,
        text: 1,
      },
      prefix: true,
      fuzzy: 0.18,
    },
    tokenize: tokenizeSearchText,
    processTerm: (term) => term.toLowerCase(),
  });

  index.addAll(indexedDocuments);
  return { index, documents: indexedDocuments };
}

export function searchProjectDocuments(
  searchIndex: ProjectSearchIndex,
  query: string,
  enabledKinds: ReadonlySet<SearchKind>,
  enabledElementTypes: ReadonlySet<string> = new Set(),
): ProjectStoreSearchDocument[] {
  const allowed = (document: ProjectSearchDocument) => {
    const kind = normalizeSearchKind(document.displayKind);
    if (kind && !enabledKinds.has(kind)) return false;
    if (kind === "element" && enabledElementTypes.size > 0) {
      return Boolean(document.elementType && enabledElementTypes.has(document.elementType));
    }
    return true;
  };

  const q = query.trim();
  if (!q) {
    return searchIndex.documents
      .filter(allowed)
      .slice(0, EMPTY_RESULT_LIMIT)
      .map(stripIndexFields);
  }

  const results = searchIndex.index.search(q, {
    filter: (result) => allowed(result as unknown as StoredSearchResult),
  }) as unknown as StoredSearchResult[];

  return results
    .sort((left, right) => compareSearchResults(left, right, q))
    .slice(0, QUERY_RESULT_LIMIT)
    .map(stripIndexFields);
}

export function tokenizeSearchText(value: string): string[] {
  return value
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1 $2")
    .replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .split(/[^A-Za-z0-9]+/)
    .map((token) => token.trim().toLowerCase())
    .filter(Boolean);
}

function searchPathText(document: ProjectStoreSearchDocument): string {
  return [
    document.id,
    document.route,
    document.route.replace(/^#\/(?:content|elements|resources)\//, ""),
  ].join(" ");
}

export function displaySearchKind(
  document: { id: string; kind: string },
  resourceByTarget: Map<string, unknown>,
  filesWithElements: Set<string>,
): string {
  if (document.kind === "file" && resourceByTarget.has(document.id) && !filesWithElements.has(document.id)) {
    return "resource";
  }
  return document.kind;
}

function compareSearchResults(left: StoredSearchResult, right: StoredSearchResult, query: string): number {
  const leftScore = resultScore(left, query);
  const rightScore = resultScore(right, query);
  if (leftScore !== rightScore) return rightScore - leftScore;
  return left.ordinal - right.ordinal;
}

function resultScore(result: StoredSearchResult, query: string): number {
  const normalizedQuery = query.toLowerCase();
  const title = result.title.toLowerCase();
  const path = result.pathText.toLowerCase();
  let score = Number(result.score ?? 0);

  if (title === normalizedQuery) score += 50;
  if (title.startsWith(normalizedQuery)) score += 25;
  if (title.includes(normalizedQuery)) score += 15;
  if (path.includes(normalizedQuery)) score += 6;

  return score;
}

function stripIndexFields(document: ProjectSearchDocument): ProjectStoreSearchDocument {
  return {
    id: document.id,
    kind: document.kind,
    title: document.title,
    route: document.route,
    text: document.text,
  };
}

function normalizeSearchKind(kind: string): SearchKind | null {
  const normalized = kind.toLowerCase();
  return normalized === "element" ||
    normalized === "file" ||
    normalized === "resource" ||
    normalized === "ontology"
    ? normalized
    : null;
}
