import { useEffect, useMemo, useRef, useState, type FormEvent } from "react";
import { useStore } from "../store/StoreContext";
import { ViewFrame } from "./ViewFrame";
import { MarkdownContent } from "../rendering/MarkdownContent";
import type { ExplorerViewProps } from "./types/ExplorerViewProps";
import { routeForSearch } from "../router/routes";
import { useExplorerUiState } from "../state/ExplorerUiState";
import { useSearchIndex } from "../search/SearchIndexContext";
import { RouteLayout, RoutePanel, SearchPageToolbar, SearchResults, type SearchResultListItem } from "@ds";
import { displaySearchKind } from "../lib/searchIndex";
import type { ProjectStoreSearchDocument } from "../store/types";

/*
 * Search view. Filters the Project Store search documents, which carry enough
 * normalized ids to route to element/file/resource/ontology detail without
 * rebuilding view-local indexes from HTML text.
 */
export function SearchView({
  initialQuery,
  onOpenElement,
}: {
  initialQuery: string | null;
  onOpenElement: (id: string) => void;
} & Partial<ExplorerViewProps>) {
  const { store, elementById } = useStore();
  const { searchElementTypes, searchKinds } = useExplorerUiState();
  const searchIndex = useSearchIndex();
  const [query, setQuery] = useState(initialQuery ?? "");
  const [results, setResults] = useState<ProjectStoreSearchDocument[]>([]);
  const [searchError, setSearchError] = useState<string | null>(null);
  const searchRequestRef = useRef(0);

  useEffect(() => {
    setQuery(initialQuery ?? "");
  }, [initialQuery]);

  function submitSearch(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    window.location.hash = routeForSearch(query);
  }

  const resourceByTarget = useMemo(() => {
    const resources = new Map<string, typeof store.resources[number]>();
    for (const resource of store.resources) {
      resources.set(resource.target, resource);
      if (resource.file_path) {
        resources.set(resource.file_path, resource);
      }
    }
    return resources;
  }, [store.resources]);
  const filesWithElements = useMemo(
    () => new Set(store.elements.map((element) => element.file_path)),
    [store.elements],
  );

  useEffect(() => {
    if (searchIndex.status !== "ready") {
      searchRequestRef.current += 1;
      setResults([]);
      return;
    }

    const requestId = searchRequestRef.current + 1;
    searchRequestRef.current = requestId;
    setSearchError(null);

    searchIndex
      .search(query, searchKinds, searchElementTypes)
      .then((nextResults) => {
        if (searchRequestRef.current === requestId) {
          setResults(nextResults);
        }
      })
      .catch((error: unknown) => {
        if (searchRequestRef.current === requestId) {
          setResults([]);
          setSearchError(error instanceof Error ? error.message : String(error));
        }
      });
  }, [query, searchElementTypes, searchIndex, searchKinds]);

  const searchResultItems: SearchResultListItem[] = results.map((d) => {
    const element = d.kind === "element" ? elementById(d.id) : undefined;
    const resource = displayResourceForFile(d, resourceByTarget, filesWithElements);
    const displayKind = displaySearchKind(d, resourceByTarget, filesWithElements);
    const route = element?.source_anchor ?? (resource ? `#/resources/${resource.id}` : d.route);
    const displayRoute = displaySearchRoute(route);
    const displayPreview = element ? null : uniqueSearchPreview(d.text, [d.title, displayRoute, route]);

    return {
      key: `${d.kind}:${d.id}`,
      title: d.title,
      route: uniqueSearchRoute(displayRoute, d.title, displayKind) ?? undefined,
      preview: element ? (
        <MarkdownContent
          markdown={element.content}
          sourceFilePath={element.file_path}
          sourceAnchor={element.source_anchor}
          variant="preview"
        />
      ) : (
        displayPreview ?? undefined
      ),
      badge: element
        ? {
            label: element.element_type,
            type: element.element_type,
            family: element.type_family,
            tinted: true,
          }
        : {
            label: searchKindLabel(displayKind),
          },
      action: element ? { kind: "button", onClick: () => onOpenElement(d.id) } : { kind: "link", href: route },
    };
  });

  const emptyMessage =
    searchIndex.status !== "ready"
      ? "Building ranked search index..."
      : searchError
        ? searchError
        : results.length === 0
          ? "No matches."
          : undefined;

  return (
    <ViewFrame testId="search">
      <RouteLayout>
        <RoutePanel>
          <SearchPageToolbar
            query={query}
            resultSummary={searchIndex.status === "ready" ? `${results.length} results` : "Indexing"}
            statusText={searchStatusText(searchIndex.status, query, searchIndex.documentCount, searchIndex.error ?? searchError)}
            onQueryChange={setQuery}
            onSubmit={submitSearch}
          />
          <SearchResults results={searchResultItems} emptyMessage={emptyMessage} />
        </RoutePanel>
      </RouteLayout>
    </ViewFrame>
  );
}

function searchKindLabel(kind: string): string {
  return kind.replace(/[-_]/g, " ");
}

function displayResourceForFile<T>(
  document: { id: string; kind: string },
  resourceByTarget: Map<string, T>,
  filesWithElements: Set<string>,
): T | undefined {
  if (document.kind !== "file" || filesWithElements.has(document.id)) return undefined;
  return resourceByTarget.get(document.id);
}

function displaySearchRoute(route: string): string {
  return route
    .replace(/^#\/content\//, "")
    .replace(/^#\/elements\//, "")
    .replace(/^#\/resources\//, "resource:")
    .replace(/^#\//, "");
}

function uniqueSearchRoute(route: string, title: string, kind: string): string | null {
  if (kind === "resource") return null;
  return normalizeSearchLine(route) === normalizeSearchLine(title) ? null : route;
}

function uniqueSearchPreview(text: string, seen: string[]): string | null {
  const normalizedText = normalizeSearchLine(text);
  if (!normalizedText) return null;
  return seen.some((value) => normalizeSearchLine(value) === normalizedText) ? null : text;
}

function normalizeSearchLine(value: string): string {
  return value.trim().replace(/^#\/content\//, "").replace(/^#\//, "").toLowerCase();
}

function searchStatusText(
  status: "building" | "ready" | "error",
  query: string,
  documentCount: number,
  error: string | null,
): string {
  if (status === "building") return "Preparing ranked search index.";
  if (status === "error") return error ? `Search index failed: ${error}` : "Search index failed.";
  return query ? `Filtering by "${query}"` : `Search ${documentCount.toLocaleString()} indexed documents.`;
}
