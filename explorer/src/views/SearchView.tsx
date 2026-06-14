import { useEffect, useMemo, useRef, useState, type FormEvent } from "react";
import { css, cx } from "@linaria/atomic";
import { useStore } from "../store/StoreContext";
import { ViewFrame } from "./ViewFrame";
import { MarkdownContent } from "../components/MarkdownContent";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";
import { ExplorerWorkspaceToolbar } from "../components/ExplorerWorkspaceToolbar";
import { routeForSearch } from "../router/routes";
import { useExplorerUiState } from "../components/ExplorerUiState";
import { useSearchIndex } from "../components/SearchIndexContext";
import { Badge, Card, Icon, SearchInput, TypeBadge } from "@ds";
import { displaySearchKind } from "../lib/searchIndex";
import type { ProjectStoreSearchDocument } from "../store/types";

const routeBaseUX = css`
  box-sizing: border-box;
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1fr) !important;
  column-gap: 0;
  height: 100vh;
  min-height: 0;
  padding-left: var(--ex-current-left-width);
  padding-right: 0;

  .ex-app & {
    height: 100%;
    min-height: 0;
    overflow: hidden;
    padding-left: 0;
    padding-right: 0;
  }
`;

const routeSingleUX = css`
  grid-template-columns: minmax(0, 1fr) !important;
  column-gap: 0;
`;

const routeSkinX = css`
  background: var(--bg-canvas);
  color: var(--text-body);
`;

const documentPanelBaseUX = css`
  position: relative;
  box-sizing: border-box;
  min-width: 0;
  min-height: 0;
  overflow: auto;
  padding: var(--space-14) var(--space-16);

  .ex-app & {
    height: 100%;
    min-height: 0;
    overflow: auto;
    padding: var(--space-16);
  }
`;

const documentPanelSkinX = css`
  border-left: var(--border-w) solid color-mix(in srgb, var(--border-subtle) 65%, transparent);
  border-right: var(--border-w) solid color-mix(in srgb, var(--border-subtle) 65%, transparent);
  background: var(--bg-surface);

  .ex-app & {
    border-right: 0;
    border-left: 0;
    background: var(--bg-surface);
  }
`;

const searchToolbarBaseUX = css`
  position: sticky;
  top: 0;
  z-index: 3;
  display: grid;
  gap: var(--space-6);
  padding: var(--space-7);

  .ex-active-controls {
    display: flex;
    align-items: baseline;
    gap: var(--space-4);
  }

  .search-page-title {
    font-size: var(--text-sm);
    font-weight: var(--weight-bold);
  }

  .search-result-count {
    font-size: var(--text-caption);
    font-weight: var(--weight-medium);
  }

  .search-page-form {
    min-width: 0;
  }

  .search-page-query {
    font-size: var(--text-sm);
  }
`;

const searchToolbarSkinX = css`
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
  box-shadow: var(--shadow-md);

  .search-page-title {
    color: var(--text-strong);
  }

  .search-result-count,
  .search-page-query {
    color: var(--text-muted);
  }
`;

const searchResultsBaseUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  margin-top: var(--space-8);

  .search-result-card {
    overflow: hidden;
  }

  .search-result-action {
    display: grid;
    grid-template-columns: minmax(0, 1fr) var(--space-8);
    align-items: start;
    gap: var(--space-6);
    width: 100%;
    box-sizing: border-box;
    padding: var(--space-5) var(--space-6);
    text-align: left;
    text-decoration: none;
    cursor: pointer;
  }

  .search-result-main {
    display: grid;
    gap: var(--space-2);
    min-width: 0;
  }

  .search-result-heading {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    min-height: var(--row-h);
    min-width: 0;
  }

  .search-result-title {
    min-width: 0;
    overflow: hidden;
    font-size: var(--text-base);
    font-weight: var(--weight-bold);
    line-height: 1.35;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .search-result-route {
    display: block;
    min-width: 0;
    overflow: hidden;
    font-family: var(--font-mono);
    font-size: var(--text-caption);
    line-height: 1.35;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .search-result-preview {
    display: -webkit-box;
    overflow: hidden;
    font-size: var(--text-sm);
    line-height: 1.45;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
  }

  .search-result-card .ex-markdown-preview {
    margin-top: var(--space-3);
  }

  .search-result-arrow {
    align-self: start;
    width: var(--space-8);
    height: var(--space-8);
    margin-top: var(--space-3);
  }
`;

const searchResultsSkinX = css`
  .search-result-card {
    border-radius: var(--radius-lg);
  }

  .search-result-card:hover {
    background: var(--bg-hover);
  }

  .search-result-action {
    border: 0;
    background: transparent;
    color: var(--text-body);
  }

  .search-result-title {
    color: var(--text-body);
  }

  .search-result-route,
  .search-result-preview,
  .search-result-card .ex-markdown-preview,
  .search-result-arrow {
    color: var(--text-muted);
  }
`;

const searchKindBadgeBaseUX = css`
  text-transform: capitalize;
`;

const emptyBaseUX = css`
  font-size: var(--text-sm);
`;

const emptySkinX = css`
  color: var(--text-muted);
`;

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

  return (
    <ViewFrame testId="search">
      <div className={cx(routeBaseUX, routeSingleUX, routeSkinX)}>
        <div className={cx(documentPanelBaseUX, documentPanelSkinX)}>
          <ExplorerWorkspaceToolbar ariaLabel="Search controls" className={cx(searchToolbarBaseUX, searchToolbarSkinX)}>
            <div className={cx("ex-active-controls")}>
              <span className="search-page-title">
                Search
              </span>
              <span className="search-result-count">
              {searchIndex.status === "ready" ? `${results.length} results` : "Indexing"}
              </span>
            </div>
            <form className="search-page-form" role="search" onSubmit={submitSearch}>
              <SearchInput
                size="lg"
                type="search"
                aria-label="Search project"
                placeholder="Search elements, files, resources, ontology terms..."
                value={query}
                onChange={(event) => setQuery(event.target.value)}
              />
            </form>
            <span className="search-page-query">
              {searchStatusText(searchIndex.status, query, searchIndex.documentCount, searchIndex.error ?? searchError)}
            </span>
          </ExplorerWorkspaceToolbar>
          <div className={cx(searchResultsBaseUX, searchResultsSkinX)}>
            {results.map((d) => {
              const element = d.kind === "element" ? elementById(d.id) : undefined;
              const resource = displayResourceForFile(d, resourceByTarget, filesWithElements);
              const displayKind = displaySearchKind(d, resourceByTarget, filesWithElements);
              const route = element?.source_anchor ?? (resource ? `#/resources/${resource.id}` : d.route);
              const displayRoute = displaySearchRoute(route);
              const displayPreview = element
                ? null
                : uniqueSearchPreview(d.text, [d.title, displayRoute, route]);
              const resultBody = (
                <>
                  <div className="search-result-main">
                    <div className="search-result-heading">
                      {element ? (
                        <TypeBadge type={element.element_type} family={element.type_family} tinted>
                          {element.element_type}
                        </TypeBadge>
                      ) : (
                        <Badge className={cx(searchKindBadgeBaseUX)}>
                          {searchKindLabel(displayKind)}
                        </Badge>
                      )}
                      <span className="search-result-title">{d.title}</span>
                    </div>
                    {uniqueSearchRoute(displayRoute, d.title, displayKind) && (
                      <span className="search-result-route">{displayRoute}</span>
                    )}
                    {element ? (
                      <MarkdownContent
                        markdown={element.content}
                        sourceFilePath={element.file_path}
                        sourceAnchor={element.source_anchor}
                        variant="preview"
                      />
                    ) : displayPreview ? (
                      <span className="search-result-preview">{displayPreview}</span>
                    ) : null}
                  </div>
                  <Icon name="arrow-up-right" className="search-result-arrow" aria-hidden="true" />
                </>
              );

              return (
                <Card key={`${d.kind}:${d.id}`} interactive padded={false} className="search-result-card">
                  {d.kind === "element" ? (
                    <button
                      type="button"
                      className="search-result-action"
                      onClick={() => onOpenElement(d.id)}
                    >
                      {resultBody}
                    </button>
                  ) : (
                    <a className="search-result-action" href={route}>
                      {resultBody}
                    </a>
                  )}
                </Card>
              );
            })}
            {searchIndex.status !== "ready" ? (
              <span className={cx(emptyBaseUX, emptySkinX)}>Building ranked search index...</span>
            ) : searchError ? (
              <span className={cx(emptyBaseUX, emptySkinX)}>{searchError}</span>
            ) : results.length === 0 ? (
              <span className={cx(emptyBaseUX, emptySkinX)}>No matches.</span>
            ) : null}
          </div>
        </div>
      </div>
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
