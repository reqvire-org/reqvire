import type { HTMLAttributes, KeyboardEvent, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { SegmentedControl } from "../../components/controls/SegmentedControl";
import { Card } from "../../components/core/Card";
import { Icon } from "../../components/core/Icon";
import { ElementIcon } from "../../components/data/ElementIcon";
import {
  Table,
  TableBody,
  TableCell,
  TableHeaderCell,
  TableHeaderGroup,
  TableRow,
  TableSortButton,
  TableViewport,
} from "../../components/data/Table";
import { TypeBadge } from "../../components/data/TypeBadge";
import { RouteLayout, RoutePanel } from "../shell";

export type FileBrowserLayout = "list" | "grid";
export type FileBrowserMode = FileBrowserLayout | "graph";
export type FileBrowserSortKey = "name" | "type" | "elements" | "path";
export type FileBrowserSortDirection = "asc" | "desc";
export type FileBrowserItemKind = "folder" | "file";

export interface FileBrowserBreadcrumb {
  path: string;
  label: string;
}

export interface FileBrowserItem {
  kind: FileBrowserItemKind;
  id: string;
  name: string;
  path: string;
  displayPath: string;
  elementCount: number;
  childCount: number;
  selected?: boolean;
  emptyFile?: boolean;
  href?: string;
  contentHref?: string;
}

export interface FileBrowserFrameProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  children?: ReactNode;
}

export interface FileBrowserToolbarProps {
  breadcrumbs: FileBrowserBreadcrumb[];
  selectedFile?: {
    name: string;
    title: string;
  };
  layout: FileBrowserLayout;
  resultCount: number;
  onOpenFolder: (path: string) => void;
  onLayoutChange: (layout: FileBrowserMode) => void;
}

export interface FileBrowserListProps {
  items: FileBrowserItem[];
  sortKey: FileBrowserSortKey;
  sortDirection: FileBrowserSortDirection;
  onSort: (key: FileBrowserSortKey) => void;
  onOpenFolder: (path: string) => void;
  onOpenFile?: (path: string) => void;
}

export interface FileBrowserGridProps {
  items: FileBrowserItem[];
  onOpenFolder: (path: string) => void;
  onOpenFile?: (path: string) => void;
}

export interface FileBrowserElementsPanelProps {
  children?: ReactNode;
}

export interface FileBrowserModeledElementsProps {
  layout: FileBrowserLayout;
  children?: ReactNode;
}

export interface FileBrowserModeledElementProps {
  layout: FileBrowserLayout;
  name: string;
  type?: string | null;
  family?: string | null;
  onOpen: () => void;
}

const frameUX = css`
  --ux-file-toolbar-actions-min-w: 280px;
  --ux-file-crumb-max-w: 190px;
  --ux-file-crumb-wide-max-w: 240px;
  --ux-file-table-min-w: 780px;
  --ux-file-path-max-w: 360px;
  --ux-file-tile-min-w: 230px;
  --ux-file-tile-min-h-compact: 112px;
  --ux-file-row-card-min-h: 78px;
  --ux-file-hover-lift: -1px;
  display: flex;
  flex-direction: column;
  gap: var(--space-7);
  overflow: visible;

  [data-product-pattern="app-shell"] & {
    overflow: auto;
  }
`;

const frameSkinX = css`
  color: var(--text-body);
`;

const toolbarUX = css`
  display: flex;
  min-height: var(--space-24);
  align-items: center;
  justify-content: space-between;
  gap: var(--space-6);
  padding: 0 var(--space-2) var(--space-7);

  .ux-file-browser__toolbar-actions {
    display: flex;
    min-width: min(100%, var(--ux-file-toolbar-actions-min-w));
    align-items: center;
    justify-content: flex-end;
    gap: var(--space-5);
    flex-wrap: wrap;
  }

  .ux-file-browser__breadcrumbs {
    display: flex;
    min-width: 0;
    flex: 1 1 auto;
    align-items: center;
    gap: var(--space-1);
    overflow: hidden;
    font-size: var(--text-sm);
  }

  .ux-file-browser__crumb {
    display: inline-flex;
    min-width: 0;
    align-items: center;
    gap: var(--space-1);
  }

  .ux-file-browser__crumb button {
    max-width: var(--ux-file-crumb-max-w);
    overflow: hidden;
    border: 0;
    background: transparent;
    text-overflow: ellipsis;
    white-space: nowrap;
    cursor: pointer;
  }

  .ux-file-browser__crumb-current span:last-child {
    display: inline-block;
    max-width: var(--ux-file-crumb-wide-max-w);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  @media (max-width: 900px) {
    align-items: stretch;
    flex-direction: column;

    .ux-file-browser__toolbar-actions {
      width: 100%;
      min-width: 0;
    }
  }
`;

const toolbarSkinX = css`
  border-bottom: var(--border-w) solid var(--border-default);
  background: var(--bg-surface);

  .ux-file-browser__breadcrumbs {
    color: var(--text-muted);
  }

  .ux-file-browser__crumb button {
    color: var(--text-body);
  }

  .ux-file-browser__crumb button:hover {
    text-decoration: underline;
  }

  .ux-file-browser__crumb-current span:last-child {
    color: var(--text-strong);
    font-weight: var(--weight-medium);
  }

  .ux-file-browser__crumb-separator {
    color: var(--text-separator);
  }

  .ux-file-browser__count {
    color: var(--text-muted);
    font-size: var(--text-caption);
    line-height: 1.4;
  }
`;

const tableUX = css`
  min-height: 0;
  overflow: visible;
  box-shadow: none;
  --ds-tablewrap-border: 0;
  --ds-tablewrap-radius: 0;
  --ds-tablewrap-bg: transparent;
  --ds-table-min-w: var(--ux-file-table-min-w);
  --ds-table-td-p: var(--space-4) var(--space-6);

  th {
    font-weight: var(--weight-bold);
  }

  .ux-file-browser__name-cell {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    min-width: 0;
  }

  .ux-file-browser__path {
    max-width: var(--ux-file-path-max-w);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
`;

const tableSkinX = css`
  --ds-table-th-bg: transparent;
  --ds-table-th-border: transparent;
  --ds-table-th-fw: var(--weight-bold);
  --ds-table-td-border: transparent;

  th {
    background: transparent;
  }

  .ux-file-browser__path {
    color: var(--text-muted);
  }

  .ux-file-browser__name-cell .ux-file-browser__item-action:hover,
  .ux-file-browser__name-cell .ux-file-browser__item-action:focus-visible,
  .ux-file-browser__name-cell .ux-file-browser__item-action.is-selected {
    border-color: transparent;
    background: transparent;
    box-shadow: none;
    outline: none;
  }
`;

const gridUX = css`
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(var(--ux-file-tile-min-w), 1fr));
  gap: var(--space-8);

  .ux-file-browser__card {
    position: relative;
    display: flex;
    flex-direction: column;
    min-height: var(--ux-file-tile-min-h-compact);
    gap: var(--space-5);
    box-sizing: border-box;
    padding: var(--space-7);
  }

  .ux-file-browser__card[role="button"] {
    cursor: pointer;
  }

  .ux-file-browser__card.is-empty-file {
    min-height: var(--ux-file-tile-min-h-compact);
  }

  .ux-file-browser__card > .ux-file-browser__item-action {
    border: 0;
    padding: 0;
    font-size: var(--text-base);
    font-weight: var(--weight-semibold);
  }

  .ux-file-browser__card > .ux-file-browser__open-link {
    position: absolute;
    top: var(--space-5);
    right: var(--space-5);
  }

  .ux-file-browser__card-path {
    display: -webkit-box;
    min-width: 0;
    overflow: hidden;
    overflow-wrap: anywhere;
    font-size: var(--text-caption);
    line-height: 1.35;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
  }

  .ux-file-browser__count-badge {
    display: inline-flex;
    width: fit-content;
    align-items: center;
    padding: var(--space-1) var(--space-4);
    font-size: var(--text-caption);
    font-weight: var(--weight-bold);
    line-height: 1.2;
  }
`;

const gridSkinX = css`
  .ux-file-browser__card {
    border-color: var(--border-default);
    border-radius: var(--radius-sm);
    background: var(--bg-raised);
    box-shadow: var(--shadow-xs);
  }

  .ux-file-browser__card:hover {
    border-color: var(--border-default);
    background: var(--bg-hover);
    box-shadow: var(--shadow-xs);
    transform: translateY(var(--ux-file-hover-lift));
  }

  .ux-file-browser__card.is-selected,
  .ux-file-browser__card.is-selected:hover {
    border-color: var(--border-default);
    background: var(--bg-selected);
    box-shadow: var(--shadow-xs);
  }

  .ux-file-browser__card:focus-visible {
    outline: none;
    box-shadow: var(--ring-focus);
  }

  .ux-file-browser__card > .ux-file-browser__item-action {
    border-radius: var(--radius-sm);
  }

  .ux-file-browser__card > .ux-file-browser__item-action:hover,
  .ux-file-browser__card > .ux-file-browser__item-action.is-selected {
    border-color: transparent;
    background: transparent;
  }

  .ux-file-browser__card-path {
    color: var(--text-muted);
  }

  .ux-file-browser__count-badge {
    border-radius: var(--radius-pill);
    background: var(--bg-sunken);
    color: var(--text-muted);
  }
`;

const itemActionUX = css`
  display: flex;
  width: 100%;
  min-width: 0;
  align-items: center;
  gap: var(--space-4);
  box-sizing: border-box;
  padding: var(--space-3) var(--space-5);
  text-align: left;
  text-decoration: none;
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  cursor: pointer;
`;

const itemActionSkinX = css`
  border: var(--border-w) solid transparent;
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--text-body);

  &:hover {
    border-color: var(--border-subtle);
    background: var(--bg-hover);
  }
`;

const itemNameUX = css`
  min-width: 0;
  flex: 1 1 auto;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
`;

const itemIconUX = css`
  width: var(--icon-sm);
  height: var(--icon-sm);
  flex: 0 0 auto;
`;

const openLinkUX = css`
  display: inline-flex;
  flex: 0 0 auto;
  width: var(--space-12);
  height: var(--space-12);
  align-items: center;
  justify-content: center;
  text-decoration: none;
  opacity: 0.64;
  transition:
    color var(--dur-fast) var(--ease-standard),
    opacity var(--dur-fast) var(--ease-standard),
    transform var(--dur-fast) var(--ease-standard);

  svg {
    display: block;
    flex: 0 0 auto;
    width: var(--space-7);
    height: var(--space-7);
  }
`;

const openLinkSkinX = css`
  border: 0;
  border-radius: var(--radius-xs);
  background: transparent;
  color: var(--text-muted);

  &:hover,
  &:focus-visible {
    background: transparent;
    color: var(--text-strong);
    opacity: 1;
    outline: none;
  }

  &:focus-visible {
    box-shadow: var(--ring-focus);
  }
`;

const elementsUX = css`
  padding-top: var(--space-10);

  .ux-file-browser__elements-list {
    display: flex;
    flex-direction: column;
    gap: var(--gap-list-stack);
    margin-top: var(--space-6);
  }

  .ux-file-browser__elements-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(var(--ux-file-tile-min-w), 1fr));
    gap: var(--space-7);
    margin-top: var(--space-7);
  }

  .ux-file-browser__element-card {
    display: grid;
    grid-template-columns: var(--type-icon-sm) minmax(0, 1fr);
    align-items: start;
    gap: var(--space-3);
    min-height: var(--ux-file-row-card-min-h);
    box-sizing: border-box;
    padding: var(--space-6);
    text-align: left;
    cursor: pointer;
    transition:
      border-color var(--dur-fast) var(--ease-standard),
      background var(--dur-fast) var(--ease-standard),
      box-shadow var(--dur-fast) var(--ease-standard),
      transform var(--dur-fast) var(--ease-standard);
  }

  .ux-file-browser__element-card-main {
    display: grid;
    min-width: 0;
    gap: var(--space-2);
  }

  .ux-file-browser__element-card-title {
    display: -webkit-box;
    overflow: hidden;
    font-size: var(--text-sm);
    font-weight: var(--weight-medium);
    line-height: 1.3;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 2;
  }

  .ux-file-browser__element-row {
    display: flex;
    width: 100%;
    min-width: 0;
    align-items: center;
    gap: var(--space-4);
    box-sizing: border-box;
    padding: var(--space-4) var(--space-5);
    text-align: left;
  }
`;

const elementsSkinX = css`
  border-top: var(--border-w) solid var(--border-default);

  .ux-file-browser__element-card {
    border: var(--border-w) solid var(--border-default);
    border-radius: var(--radius-sm);
    background: var(--bg-surface);
    color: var(--text-body);
  }

  .ux-file-browser__element-card:hover {
    border-color: var(--border-default);
    background: var(--bg-hover);
    transform: translateY(var(--ux-file-hover-lift));
  }

  .ux-file-browser__element-card-title {
    color: var(--text-body);
  }

  .ux-file-browser__element-row {
    border: var(--border-w) solid transparent;
    border-radius: var(--radius-md);
    background: transparent;
    color: var(--text-body);
  }

  .ux-file-browser__element-row:hover {
    border-color: transparent;
    background: var(--bg-hover);
  }

  .ux-file-browser__element-row.is-selected {
    border-color: transparent;
    background: var(--bg-selected);
    color: var(--text-body);
  }
`;

const emptyUX = css`
  font-size: var(--text-sm);
  font-style: italic;
  line-height: 1.45;
`;

const emptySkinX = css`
  color: var(--text-muted);
`;

const missingMessageUX = css`
  font-size: var(--text-sm);
`;

const missingMessageSkinX = css`
  color: var(--text-muted);
`;

const missingPathUX = css`
  padding: var(--space-1) var(--space-2);
  font-size: var(--text-caption);
`;

const missingPathSkinX = css`
  border-radius: var(--radius-xs);
  background: var(--bg-sunken);
`;

export function FileBrowserFrame({
  children,
  className = "",
  ...props
}: FileBrowserFrameProps) {
  return (
    <RouteLayout layout="single">
      <RoutePanel>
        <div
          data-product-pattern="file-browser"
          className={cx("ux-file-browser", frameUX, frameSkinX, className)}
          {...props}
        >
          {children}
        </div>
      </RoutePanel>
    </RouteLayout>
  );
}

export function FileBrowserToolbar({
  breadcrumbs,
  selectedFile,
  layout,
  resultCount,
  onOpenFolder,
  onLayoutChange,
}: FileBrowserToolbarProps) {
  return (
    <div className={cx("ux-file-browser__toolbar", toolbarUX, toolbarSkinX)}>
      <div className="ux-file-browser__breadcrumbs" aria-label="File breadcrumbs">
        {breadcrumbs.map((crumb, index) => (
          <span key={crumb.path} className="ux-file-browser__crumb">
            {index > 0 && <span className="ux-file-browser__crumb-separator">/</span>}
            <button type="button" onClick={() => onOpenFolder(crumb.path)}>
              {crumb.label}
            </button>
          </span>
        ))}
        {selectedFile ? (
          <span className={cx("ux-file-browser__crumb", "ux-file-browser__crumb-current")}>
            <span className="ux-file-browser__crumb-separator">/</span>
            <span title={selectedFile.title}>{selectedFile.name}</span>
          </span>
        ) : null}
      </div>
      <div className="ux-file-browser__toolbar-actions">
        <span className="ux-file-browser__count">{resultCount} items</span>
        <SegmentedControl<FileBrowserMode>
          ariaLabel="Model layout"
          value={layout}
          onChange={onLayoutChange}
          items={[
            { value: "list", label: "List", icon: <Icon name="list" /> },
            { value: "grid", label: "Grid", icon: <Icon name="layout-grid" /> },
            { value: "graph", label: "Graph", icon: <Icon name="git-branch" /> },
          ]}
        />
      </div>
    </div>
  );
}

export function FileBrowserMissingFile({ path }: { path: string }) {
  return (
    <span className={cx("ux-file-browser__missing", missingMessageUX, missingMessageSkinX)}>
      No file container for{" "}
      <code className={cx("ux-file-browser__missing-path", missingPathUX, missingPathSkinX)}>{path}</code>.
    </span>
  );
}

export function FileBrowserList({
  items,
  sortKey,
  sortDirection,
  onSort,
  onOpenFolder,
  onOpenFile,
}: FileBrowserListProps) {
  return (
    <TableViewport className={cx("ux-file-browser__table", tableUX, tableSkinX)}>
      <Table>
        <TableHeaderGroup>
          <TableRow>
            <SortableHeader label="Name" sortKey="name" activeKey={sortKey} direction={sortDirection} onSort={onSort} />
            <SortableHeader label="Type" sortKey="type" activeKey={sortKey} direction={sortDirection} onSort={onSort} />
            <SortableHeader label="Elements" sortKey="elements" activeKey={sortKey} direction={sortDirection} onSort={onSort} />
            <SortableHeader label="Path" sortKey="path" activeKey={sortKey} direction={sortDirection} onSort={onSort} />
          </TableRow>
        </TableHeaderGroup>
        <TableBody>
          {items.map((item) => (
            <TableRow key={item.id} selected={Boolean(item.selected)}>
              <TableCell>
                <div className="ux-file-browser__name-cell">
                  <FileBrowserItemAction item={item} onOpenFolder={onOpenFolder} onOpenFile={onOpenFile} />
                  <FileBrowserContentLink item={item} />
                </div>
              </TableCell>
              <TableCell>
                <TypeBadge type={item.kind} family={item.kind}>{item.kind}</TypeBadge>
              </TableCell>
              <TableCell>{item.elementCount}</TableCell>
              <TableCell className="ux-file-browser__path">{item.displayPath}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      {items.length === 0 ? <FileBrowserEmptyState>No files or folders match the current filter.</FileBrowserEmptyState> : null}
    </TableViewport>
  );
}

export function FileBrowserGrid({
  items,
  onOpenFolder,
  onOpenFile,
}: FileBrowserGridProps) {
  return (
    <div className={cx("ux-file-browser__grid", gridUX, gridSkinX)}>
      {items.map((item) => (
        <Card
          key={item.id}
          interactive
          selected={Boolean(item.selected)}
          role="button"
          tabIndex={0}
          aria-label={`Open ${item.name}`}
          onClick={() => openFileBrowserItem(item, onOpenFolder, onOpenFile)}
          onKeyDown={(event) => handleFileBrowserCardKeyDown(event, item, onOpenFolder, onOpenFile)}
          className={cx(
            "ux-file-browser__card",
            item.selected ? "is-selected" : "",
            item.emptyFile ? "is-empty-file" : "",
          )}
        >
          <FileBrowserItemIdentity item={item} selected={Boolean(item.selected)} />
          {(item.kind === "folder" || item.elementCount > 0) && (
            <span className="ux-file-browser__count-badge">
              {item.kind === "folder" ? `${item.childCount} children` : `${item.elementCount} elements`}
            </span>
          )}
          <span className="ux-file-browser__card-path">{item.displayPath}</span>
        </Card>
      ))}
      {items.length === 0 ? <FileBrowserEmptyState>No files or folders match the current filter.</FileBrowserEmptyState> : null}
    </div>
  );
}

export function FileBrowserElementsPanel({
  children,
}: FileBrowserElementsPanelProps) {
  return <div className={cx("ux-file-browser__elements", elementsUX, elementsSkinX)}>{children}</div>;
}

export function FileBrowserModeledElements({
  layout,
  children,
}: FileBrowserModeledElementsProps) {
  return (
    <div className={layout === "grid" ? "ux-file-browser__elements-grid" : "ux-file-browser__elements-list"}>
      {children}
    </div>
  );
}

export function FileBrowserModeledElement({
  layout,
  name,
  type,
  family,
  onOpen,
}: FileBrowserModeledElementProps) {
  if (layout === "grid") {
    return (
      <button type="button" onClick={onOpen} className="ux-file-browser__element-card">
        {type ? (
          <ElementIcon type={type} family={family} title={type} size="sm" />
        ) : (
          <ElementIcon type="other" size="sm" />
        )}
        <span className="ux-file-browser__element-card-main">
          <span className="ux-file-browser__element-card-title">{name}</span>
          {type ? (
            <TypeBadge type={type} family={family} tinted dot={false}>
              {type}
            </TypeBadge>
          ) : null}
        </span>
      </button>
    );
  }

  return (
    <button type="button" onClick={onOpen} className="ux-file-browser__element-row">
      {type ? (
        <ElementIcon type={type} family={family} title={type} size="sm" />
      ) : (
        <ElementIcon type="other" size="sm" />
      )}
      <span>{name}</span>
    </button>
  );
}

export function FileBrowserEmptyState({
  children,
}: {
  children: ReactNode;
}) {
  return <span className={cx("ux-file-browser__empty", emptyUX, emptySkinX)}>{children}</span>;
}

function SortableHeader({
  label,
  sortKey,
  activeKey,
  direction,
  onSort,
}: {
  label: string;
  sortKey: FileBrowserSortKey;
  activeKey: FileBrowserSortKey;
  direction: FileBrowserSortDirection;
  onSort: (key: FileBrowserSortKey) => void;
}) {
  const active = sortKey === activeKey;
  return (
    <TableHeaderCell>
      <TableSortButton direction={active ? direction : undefined} onClick={() => onSort(sortKey)}>
        {label}
      </TableSortButton>
    </TableHeaderCell>
  );
}

function FileBrowserContentLink({
  item,
}: {
  item: FileBrowserItem;
}) {
  if (!item.contentHref) return null;
  return (
    <a
      href={item.contentHref}
      className={cx("ux-file-browser__open-link", openLinkUX, openLinkSkinX)}
      aria-label={`Open content for ${item.name}`}
      title="Open content"
      onClick={(event) => event.stopPropagation()}
    >
      <Icon name="external-link" />
    </a>
  );
}

function openFileBrowserItem(
  item: FileBrowserItem,
  onOpenFolder: (path: string) => void,
  onOpenFile?: (path: string) => void,
) {
  if (item.kind === "folder") {
    onOpenFolder(item.path);
    return;
  }
  if (onOpenFile) {
    onOpenFile(item.path);
    return;
  }
  if (item.href) {
    window.location.hash = item.href.startsWith("#") ? item.href.slice(1) : item.href;
  }
}

function handleFileBrowserCardKeyDown(
  event: KeyboardEvent<HTMLDivElement>,
  item: FileBrowserItem,
  onOpenFolder: (path: string) => void,
  onOpenFile?: (path: string) => void,
) {
  if (event.key !== "Enter" && event.key !== " ") return;
  event.preventDefault();
  openFileBrowserItem(item, onOpenFolder, onOpenFile);
}

function FileBrowserItemIdentity({
  item,
  selected = false,
}: {
  item: FileBrowserItem;
  selected?: boolean;
}) {
  return (
    <span
      className={cx(
        "ux-file-browser__item-action",
        itemActionUX,
        itemActionSkinX,
        selected ? "is-selected" : "",
      )}
    >
      <FileBrowserItemContents item={item} />
    </span>
  );
}

function FileBrowserItemContents({ item }: { item: FileBrowserItem }) {
  return (
    <>
      {item.kind === "folder" ? (
        <Icon name="folder" className={cx("ux-file-browser__kind-folder", "ux-file-browser__item-icon", itemIconUX)} />
      ) : (
        <Icon name="file" className={cx("ux-file-browser__kind-file", "ux-file-browser__item-icon", itemIconUX)} />
      )}
      <span className={cx("ux-file-browser__item-name", itemNameUX)}>{item.name}</span>
    </>
  );
}

function FileBrowserItemAction({
  item,
  onOpenFolder,
  onOpenFile,
}: {
  item: FileBrowserItem;
  onOpenFolder: (path: string) => void;
  onOpenFile?: (path: string) => void;
}) {
  const className = cx(
    "ux-file-browser__item-action",
    itemActionUX,
    itemActionSkinX,
    item.selected ? "is-selected" : "",
  );

  if (item.kind === "folder") {
    return (
      <button type="button" className={className} onClick={() => onOpenFolder(item.path)}>
        <FileBrowserItemContents item={item} />
      </button>
    );
  }
  if (onOpenFile) {
    return (
      <button type="button" className={className} onClick={() => onOpenFile(item.path)}>
        <FileBrowserItemContents item={item} />
      </button>
    );
  }
  return (
    <a href={item.href} className={className}>
      <FileBrowserItemContents item={item} />
    </a>
  );
}
