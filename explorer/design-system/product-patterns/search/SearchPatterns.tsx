import type { FormEventHandler, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { Badge } from "../../components/core/Badge";
import { Card } from "../../components/core/Card";
import { Icon } from "../../components/core/Icon";
import { SearchInput } from "../../components/controls/SearchInput";
import { TypeBadge } from "../../components/data/TypeBadge";
import { WorkspaceToolbar } from "../chrome/WorkspaceToolbar";

export interface SearchPageToolbarProps {
  query: string;
  resultSummary: ReactNode;
  statusText: ReactNode;
  onQueryChange: (value: string) => void;
  onSubmit: FormEventHandler<HTMLFormElement>;
}

export interface SearchResultBadge {
  label: ReactNode;
  type?: string | null;
  family?: string | null;
  tinted?: boolean;
}

export type SearchResultAction =
  | { kind: "button"; onClick: () => void }
  | { kind: "link"; href: string };

export interface SearchResultListItem {
  key: string;
  title: ReactNode;
  route?: ReactNode;
  preview?: ReactNode;
  badge: SearchResultBadge;
  action: SearchResultAction;
}

export interface SearchResultsProps {
  results: readonly SearchResultListItem[];
  emptyMessage?: ReactNode;
}

export interface SearchEmptyStateProps {
  children?: ReactNode;
}

const searchToolbarBaseUX = css`
  position: sticky;
  top: 0;
  z-index: var(--z-local-overlay);
  display: grid;
  gap: var(--space-6);
  padding: var(--space-7);

  .ux-search-active-controls {
    display: flex;
    align-items: baseline;
    gap: var(--space-4);
  }

  .ux-search-page-title {
    font-size: var(--text-sm);
    font-weight: var(--weight-bold);
  }

  .ux-search-result-count {
    font-size: var(--text-caption);
    font-weight: var(--weight-medium);
  }

  .ux-search-page-form {
    min-width: 0;
  }

  .ux-search-page-query {
    font-size: var(--text-sm);
  }
`;

const searchToolbarSkinX = css`
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
  box-shadow: var(--shadow-md);

  .ux-search-page-title {
    color: var(--text-strong);
  }

  .ux-search-result-count,
  .ux-search-page-query {
    color: var(--text-muted);
  }
`;

const searchResultsBaseUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--gap-list-stack);
  margin-top: var(--space-8);

  .ux-search-result-card {
    overflow: hidden;
  }

  .ux-search-result-action {
    display: grid;
    grid-template-columns: minmax(0, 1fr) var(--space-8);
    align-items: start;
    gap: var(--space-6);
    width: 100%;
    box-sizing: border-box;
    padding: var(--space-5) var(--space-6);
    cursor: pointer;
    text-align: left;
    text-decoration: none;
  }

  .ux-search-result-main {
    display: grid;
    min-width: 0;
    gap: var(--space-2);
  }

  .ux-search-result-heading {
    display: flex;
    align-items: center;
    min-width: 0;
    min-height: var(--row-h);
    gap: var(--space-4);
  }

  .ux-search-result-title {
    min-width: 0;
    overflow: hidden;
    font-size: var(--text-base);
    font-weight: var(--weight-bold);
    line-height: 1.35;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ux-search-result-route {
    display: block;
    min-width: 0;
    overflow: hidden;
    font-family: var(--font-mono);
    font-size: var(--text-caption);
    line-height: 1.35;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ux-search-result-preview {
    display: -webkit-box;
    overflow: hidden;
    font-size: var(--text-sm);
    line-height: 1.45;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
  }

  .ux-search-result-card [data-product-pattern="markdown-frame"][data-variant="preview"] {
    margin-top: var(--space-3);
  }

  .ux-search-result-arrow {
    align-self: start;
    width: var(--space-8);
    height: var(--space-8);
    margin-top: var(--space-3);
  }
`;

const searchResultsSkinX = css`
  .ux-search-result-card {
    border-radius: var(--radius-lg);
  }

  .ux-search-result-card:hover {
    background: var(--bg-hover);
  }

  .ux-search-result-action {
    border: 0;
    background: transparent;
    color: var(--text-body);
  }

  .ux-search-result-title {
    color: var(--text-body);
  }

  .ux-search-result-route,
  .ux-search-result-preview,
  .ux-search-result-card [data-product-pattern="markdown-frame"][data-variant="preview"],
  .ux-search-result-arrow {
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

export function SearchPageToolbar({
  query,
  resultSummary,
  statusText,
  onQueryChange,
  onSubmit,
}: SearchPageToolbarProps) {
  return (
    <WorkspaceToolbar
      aria-label="Search controls"
      className={cx("ux-search-page-toolbar", searchToolbarBaseUX, searchToolbarSkinX)}
    >
      <div className="ux-search-active-controls">
        <span className="ux-search-page-title">Search</span>
        <span className="ux-search-result-count">{resultSummary}</span>
      </div>
      <form className="ux-search-page-form" role="search" onSubmit={onSubmit}>
        <SearchInput
          size="lg"
          type="search"
          aria-label="Search project"
          placeholder="Search elements, files, resources, ontology terms..."
          value={query}
          onChange={(event) => onQueryChange(event.target.value)}
        />
      </form>
      <span className="ux-search-page-query">{statusText}</span>
    </WorkspaceToolbar>
  );
}

export function SearchResults({ results, emptyMessage }: SearchResultsProps) {
  return (
    <div className={cx("ux-search-results", searchResultsBaseUX, searchResultsSkinX)}>
      {results.map((result) => (
        <Card key={result.key} interactive padded={false} className="ux-search-result-card">
          <SearchResultActionFrame action={result.action}>
            <div className="ux-search-result-main">
              <div className="ux-search-result-heading">
                {result.badge.type ? (
                  <TypeBadge type={result.badge.type} family={result.badge.family} tinted={result.badge.tinted}>
                    {result.badge.label}
                  </TypeBadge>
                ) : (
                  <Badge className={cx("ux-search-kind-badge", searchKindBadgeBaseUX)}>
                    {result.badge.label}
                  </Badge>
                )}
                <span className="ux-search-result-title">{result.title}</span>
              </div>
              {result.route ? <span className="ux-search-result-route">{result.route}</span> : null}
              {typeof result.preview === "string" ? (
                <span className="ux-search-result-preview">{result.preview}</span>
              ) : (
                result.preview
              )}
            </div>
            <Icon name="arrow-up-right" className="ux-search-result-arrow" aria-hidden="true" />
          </SearchResultActionFrame>
        </Card>
      ))}
      {emptyMessage ? <SearchEmptyState>{emptyMessage}</SearchEmptyState> : null}
    </div>
  );
}

export function SearchEmptyState({ children }: SearchEmptyStateProps) {
  return <span className={cx("ux-search-empty", emptyBaseUX, emptySkinX)}>{children}</span>;
}

function SearchResultActionFrame({
  action,
  children,
}: {
  action: SearchResultAction;
  children: ReactNode;
}) {
  if (action.kind === "button") {
    return (
      <button type="button" className="ux-search-result-action" onClick={action.onClick}>
        {children}
      </button>
    );
  }

  return (
    <a className="ux-search-result-action" href={action.href}>
      {children}
    </a>
  );
}
