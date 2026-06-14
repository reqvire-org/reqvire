import {
  useEffect,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type KeyboardEvent as ReactKeyboardEvent,
  type PointerEvent as ReactPointerEvent,
} from "react";
import { css, cx } from "@linaria/atomic";
import { loadStore } from "./store/loadStore";
import { devFixture } from "./store/devFixture";
import { StoreProvider } from "./store/StoreContext";
import { MissingStoreNotice } from "./components/MissingStoreNotice";
import { HelpModal } from "./components/HelpModal";
import { ElementDetailModal } from "./components/ElementDetailModal";
import { OntologyNodeDetailModal } from "./components/OntologyNodeDetailModal";
import { ExplorerSidePane } from "./components/ExplorerSidePane";
import { ExplorerUiStateProvider } from "./components/ExplorerUiState";
import { SearchIndexProvider } from "./components/SearchIndexContext";
import { useHashRoute } from "./router/useHashRoute";
import { VIEW_TITLES, type ViewId } from "./router/routes";
import { ResourcesView } from "./views/ResourcesView";
import { SearchView } from "./views/SearchView";
import { FilesView } from "./views/FilesView";
import { ModelView } from "./views/ModelView";
import {
  CoverageView,
  TracesView,
} from "./views/ReportViews";
import { OntologiesView } from "./views/OntologiesView";
import { ContentView } from "./components/ContentView";
import { useTheme } from "./hooks/useTheme";
import { ReqvireRailMark, railMarkClass } from "./components/PaneChrome";
import { Icon, IconButton, Tabs } from "@ds";
import type { TabItem } from "@ds";

const LEFT_PANE_WIDTH_DEFAULT = 380;
const LEFT_PANE_WIDTH_MIN = 300;
const LEFT_PANE_WIDTH_MAX = 720;
const LEFT_PANE_WIDTH_STORAGE_KEY = "reqvire:explorer:left-pane-width";

const paneResizerClass = css`
  position: absolute;
  top: 0;
  bottom: 0;
  left: calc(var(--ex-current-left-width) - var(--space-1));
  z-index: var(--z-sticky);
  width: var(--space-3);
  cursor: ew-resize;
  touch-action: none;
  transform: translateX(-50%);

  &::before {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 50%;
    width: var(--border-w);
    background: transparent;
    content: "";
    transform: translateX(-50%);
  }

  &:hover::before,
  &:focus-visible::before {
    background: var(--border-strong);
  }

  &:focus-visible {
    outline: none;
  }
`;

const schemaWarningClass = css`
  position: absolute;
  top: var(--space-8);
  right: var(--space-8);
  left: var(--space-8);
  z-index: var(--z-popover);
`;

const shellBaseUX = css`
  --ex-left-pane-width: 380px;
  --ex-left-pane-collapsed-width: 30px;
  --ex-graph-side-panel-w: 390px;
  --ex-graph-side-panel-max-h: 420px;
  --ex-current-left-width: var(--ex-left-pane-width);
  --ex-current-right-width: 0px;
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  font-family: var(--font-sans);

  &.is-left-collapsed {
    --ex-current-left-width: var(--ex-left-pane-collapsed-width);
  }

  &.is-right-collapsed {
    --ex-current-right-width: 0px;
  }

  &.is-left-resizing,
  &.is-left-resizing * {
    cursor: ew-resize !important;
    user-select: none;
  }

  &.is-left-resizing .${paneResizerClass}::before {
    background: var(--border-strong);
  }

  &.is-left-collapsed .${paneResizerClass} {
    display: none;
  }

  &:not(.is-right-collapsed) .ex-inspector-tab-label,
  &:not(.is-right-collapsed) .ex-inspector-tab .${railMarkClass} {
    display: none;
  }

  &.has-right-inspector .ex-inspector-tab {
    display: flex;
  }

  .ex-side-pane {
    position: relative;
    inset: auto;
    z-index: auto;
    align-self: stretch;
    flex: 0 0 var(--ex-current-left-width);
    width: var(--ex-current-left-width);
    min-width: 0;
    min-height: 0;
    height: 100%;
  }

  .ex-side-pane.is-collapsed {
    display: none;
  }

  .ex-side-content {
    display: flex;
    flex: 1 1 auto;
    flex-direction: column;
    width: 100%;
    height: 100%;
    min-height: 0;
    overflow: hidden;
  }

  .ex-tree {
    --rq-treeitem-count-ml: var(--space-1);
    --rq-treeitem-h: var(--space-16);
    --rq-treeitem-hover-bg: color-mix(in srgb, var(--accent) 5%, transparent);
    --rq-treeitem-label-flex: 0 1 auto;
    --rq-treeitem-lh: 1.2;
    --rq-treeitem-pr: var(--space-6);
    --rq-treeitem-border-l: var(--border-w-thick) solid transparent;
    --rq-treeitem-radius: 0;
    --rq-treeitem-sel-bg: color-mix(in srgb, var(--accent) 10%, transparent);
    --rq-treeitem-sel-border: transparent;
    --rq-treeitem-sel-color: var(--text-body);
    --rq-treeitem-sel-fw: var(--weight-semibold);
    --rq-treeitem-sel-icon-color: var(--accent);
    --rq-treeitem-twist-w: var(--space-7);
    --rq-treeitem-twist-color: var(--text-muted);
    --rq-treeitem-icon-color: var(--text-secondary);
    flex: 1 1 auto;
    min-height: 0;
    overflow-x: hidden;
    overflow-y: auto;
    padding: var(--space-5) 0 var(--space-7);
    border-top: var(--border-w) solid;
    scrollbar-gutter: stable;
  }

  .ex-tree-tab,
  .ex-pane-chrome-header {
    display: none;
  }

  .ex-route,
  .graph-route {
    height: 100%;
    min-height: 0;
    overflow: hidden;
    padding-left: 0;
    padding-right: 0;
  }

  .ex-main-panel,
  .trace-main-panel {
    padding: var(--space-12) var(--space-16) var(--space-24);
  }

  .ex-document-panel {
    height: 100%;
    min-height: 0;
    overflow: auto;
    padding: var(--space-16);
  }

  .graph-route > .graph-sidebar,
  .ex-route > .graph-sidebar,
  .ontology-graph-sidebar {
    top: var(--space-12);
    right: var(--space-12);
    width: min(var(--ex-graph-side-panel-w), calc(100% - var(--space-24)));
    max-height: calc(100% - var(--space-24));
  }

  .graph-selection-card {
    top: var(--space-12);
    right: var(--space-12);
  }

  .ex-global-search {
    margin: var(--space-12) var(--space-10) 0;
  }

  .ex-global-search-control {
    position: relative;
  }

  .ex-pane-controls {
    display: flex;
    flex: 1 1 auto;
    flex-direction: column;
    gap: 0;
    min-height: 0;
    overflow-x: hidden;
    overflow-y: auto;
    padding: var(--space-12) var(--space-10) var(--space-16);
    scrollbar-gutter: stable;
    --rq-togglerow-jc: flex-start;
    --rq-togglerow-min-h: var(--control-md);
    --rq-togglerow-border: 0;
    --rq-togglerow-radius: 0;
    --rq-togglerow-shadow: none;
    --rq-togglerow-label-min-w: 0;
    --rq-togglerow-label-of: hidden;
    --rq-togglerow-label-toe: ellipsis;
    --rq-togglerow-label-ws: nowrap;
    --rq-togglerow-meta-display: inline-flex;
    --rq-togglerow-meta-min-w: var(--control-xs);
    --rq-togglerow-meta-h: var(--control-xs);
    --rq-togglerow-meta-ai: center;
    --rq-togglerow-meta-jc: center;
    --rq-togglerow-meta-p: 0 var(--space-3);
    --rq-togglerow-meta-radius: var(--radius-pill);
    --rq-togglerow-meta-fw: var(--weight-semibold);
    --rq-togglerow-meta-lh: 1;
    --rq-togglerow-line-min-h: var(--control-sm);
    --rq-togglerow-line-swatch-w: calc(var(--space-8) + var(--space-1));
    --rq-togglerow-static-cursor: default;
  }

  .ex-pane-controls-title {
    margin: 0 0 var(--space-7);
    font-size: var(--text-lg);
    font-weight: var(--weight-semibold);
    letter-spacing: 0;
    line-height: var(--leading-tight);
  }

  .ex-pane-section-label {
    display: block;
    margin: var(--space-12) 0 var(--space-5);
    padding: 0 var(--space-2);
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-label);
    line-height: 1;
    text-transform: uppercase;
  }

  .ex-pane-summary {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: var(--space-5);
    margin: 0 0 var(--space-6);
  }

  .ex-pane-summary .ex-pane-section-label {
    margin: 0;
    padding: 0;
  }

  .ex-pane-summary .ex-summary {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-3) var(--space-8);
    --rq-stat-display: flex;
    --rq-stat-min-w: 0;
    --rq-stat-jc: space-between;
  }

  .ex-pane-action-row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-5);
    margin: 0;
  }

  .ex-pane-ghost-link {
    display: inline-flex;
    align-items: center;
    gap: var(--space-4);
    height: var(--control-sm);
    padding: 0 var(--space-6);
    border-radius: var(--radius-sm);
    font-size: var(--text-caption);
    font-weight: var(--weight-medium);
    text-decoration: none;
  }

  .ex-pane-ghost-link svg {
    display: block;
    width: var(--icon-sm);
    height: var(--icon-sm);
    flex: 0 0 auto;
  }

  .ex-pane-nav-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .ex-pane-nav-row {
    display: grid;
    grid-template-columns: var(--icon-md) minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--space-5);
    width: 100%;
    min-height: var(--control-md);
    padding: 0 var(--space-5);
    cursor: pointer;
    font: inherit;
    text-align: left;
  }

  .ex-pane-nav-row__icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .ex-pane-nav-row__label {
    overflow: hidden;
    font-size: var(--text-sm);
    font-weight: var(--weight-medium);
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ex-pane-nav-row__count {
    display: inline-flex;
    min-width: var(--control-xs);
    height: var(--control-xs);
    align-items: center;
    justify-content: center;
    padding: 0 var(--space-3);
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    line-height: 1;
  }

  .ex-pane-legend {
    --rq-togglerow-h: var(--control-sm);
    --rq-togglerow-min-h: var(--control-sm);
    --rq-togglerow-gap: var(--space-6);
    --rq-togglerow-p: 0 var(--space-7);
    --rq-togglerow-line-h: var(--control-sm);
    --rq-togglerow-line-min-h: var(--control-sm);
    --rq-togglerow-line-p: 0 var(--space-7);
    --rq-togglerow-line-swatch-w: var(--icon-xs);
    --rq-togglerow-line-color: var(--text-muted);
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .ex-pane-legend-row {
    display: flex;
    align-items: center;
    gap: var(--space-6);
    min-height: var(--control-sm);
    padding: 0 var(--space-7);
  }

  .ex-pane-legend-text {
    font-size: var(--text-caption);
    line-height: 1.3;
  }

  .graph-line-swatch {
    display: inline-flex;
    flex: 0 0 auto;
    width: var(--icon-xs);
    height: 0;
    border-top: var(--border-w-thick) solid;
  }

  .ex-pane-symbol {
    display: inline-flex;
    min-width: var(--control-md);
    height: var(--icon-lg);
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-md);
    font-size: var(--text-micro);
    font-weight: var(--weight-bold);
    line-height: 1;
  }

  .ex-pane-selected-element {
    display: grid;
    gap: var(--space-5);
    margin: 0;
  }

  .ex-pane-selected-element .ex-pane-section-label {
    margin: 0;
    padding: 0;
  }

  .ex-pane-selected-element-link {
    display: inline-flex;
    width: 100%;
    min-width: 0;
    align-items: center;
    justify-content: flex-start;
    gap: var(--space-5);
    padding: var(--space-2) var(--space-6);
    cursor: pointer;
    font: inherit;
    text-align: left;
    text-decoration: none;
  }

  .ex-pane-selected-element-link span:last-child {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ex-pane-selection-row {
    display: flex;
    min-width: 0;
    align-items: center;
    gap: var(--space-2);
  }

  .ex-pane-selection-row .ex-pane-selected-element-link {
    flex: 1 1 auto;
  }

  .ex-pane-selection-name {
    flex: 1 1 auto;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ex-pane-selection-kind,
  .ex-pane-selection-open {
    flex: 0 0 auto;
  }

  .ex-pane-selection-kind {
    overflow: visible;
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-pill);
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    line-height: 1.2;
  }

  .ex-pane-selection-hint {
    margin: 0;
    font-size: var(--text-caption);
  }

  @media (max-width: 900px) {
    --ex-current-left-width: min(var(--ex-left-pane-width), 82vw);

    &.is-left-collapsed {
      --ex-current-left-width: var(--ex-left-pane-collapsed-width);
    }

    .graph-route > .graph-sidebar,
    .ex-route > .graph-sidebar,
    .ontology-graph-sidebar {
      inset: auto var(--space-8) var(--space-8) var(--space-8);
      width: auto;
      max-height: min(52vh, var(--ex-graph-side-panel-max-h));
    }
  }
`;

const shellSkinX = css`
  background: var(--bg-canvas);
  color: var(--text-body);

  .ex-side-pane {
    border-right: var(--border-w) solid var(--border-subtle);
    background: var(--bg-surface);
    color: var(--text-body);
  }

  .ex-mode-nav,
  .ex-pane-controls,
  .ex-tree {
    border-color: var(--border-subtle);
  }

  .ex-route,
  .graph-route,
  .ex-main-panel {
    background: var(--bg-canvas);
  }

  .trace-main-panel {
    background: var(--bg-surface);
  }

  .ex-document-panel {
    border-right: 0;
    border-left: 0;
    background: var(--bg-surface);
  }

  .graph-canvas-wrap,
  .ontology-graph-canvas {
    background: var(--bg-canvas);
  }

  .graph-route > .graph-sidebar,
  .ex-route > .graph-sidebar,
  .ontology-graph-sidebar {
    border: var(--border-w) solid var(--border-default);
    border-radius: var(--radius-lg);
    background: var(--bg-overlay);
    box-shadow: var(--shadow-lg);
  }

  .graph-selection-card {
    border-color: var(--border-default);
    border-radius: var(--radius-lg);
    background: var(--bg-overlay);
    box-shadow: var(--shadow-lg);
  }

  .graph-selection-card-header,
  .graph-inspector-header,
  .ontology-inspector-header {
    border-color: var(--border-subtle);
    background: var(--bg-surface);
  }

  .graph-inspector-body,
  .ontology-inspector-body {
    background: var(--bg-overlay);
  }

  .trace-rollup-diagram {
    border-color: var(--border-subtle);
    border-radius: var(--radius-lg);
    background: var(--bg-surface);
    box-shadow: none;
  }

  .trace-rollup-diagram .mermaid {
    background: var(--bg-surface);
  }

  .diagram-nav-buttons {
    border-color: var(--border-default);
    background: color-mix(in srgb, var(--bg-overlay) 94%, transparent);
    box-shadow: var(--shadow-md);
  }

  .diagram-nav-btn {
    border-color: var(--border-default);
    background: var(--bg-surface);
    color: var(--text-secondary);
  }

  .diagram-nav-btn:hover,
  .diagram-nav-btn:focus-visible {
    border-color: var(--border-strong);
    background: var(--bg-hover);
    color: var(--text-strong);
    outline: none;
  }

  .ex-pane-controls {
    --rq-togglerow-bg: transparent;
    --rq-togglerow-hover-bg: color-mix(in srgb, var(--accent) 5%, transparent);
    --rq-togglerow-hover-border: transparent;
    --rq-togglerow-off-bg: transparent;
    --rq-togglerow-off-color: var(--text-faint);
    --rq-togglerow-off-hover-color: var(--text-secondary);
    --rq-togglerow-off-opacity: 0.68;
    --rq-togglerow-off-hover-opacity: 0.9;
    --rq-togglerow-off-label-td: line-through;
    --rq-togglerow-off-label-td-color: color-mix(in srgb, var(--text-faint), transparent 35%);
    --rq-togglerow-off-label-td-w: var(--border-w);
    --rq-togglerow-off-swatch-bg: transparent;
    --rq-togglerow-off-swatch-border: var(--border-strong);
    --rq-togglerow-meta-bg: var(--bg-sunken);
    --rq-togglerow-meta-color: var(--text-secondary);
    --rq-togglerow-line-swatch-border: currentColor;
    --rq-togglerow-line-swatch-bg: transparent;
    --rq-togglerow-static-hover-border: var(--border-default);
    --rq-togglerow-static-hover-bg: transparent;
  }

  .ex-pane-controls-title {
    color: var(--text-strong);
  }

  .ex-pane-section-label {
    color: var(--text-muted);
  }

  .ex-pane-legend-text {
    color: var(--text-muted);
  }

  .graph-line-swatch {
    border-color: var(--text-muted);
  }

  .ex-pane-symbol {
    border: var(--border-w) solid var(--border-subtle);
    background: var(--bg-sunken);
    color: var(--text-link);
  }

  .ex-resource-link {
    color: var(--text-secondary);
    text-decoration: none;
    font-size: var(--text-sm);
  }

  .ex-resource-link:hover {
    color: var(--text-strong);
    text-decoration: underline;
  }

  .ex-pane-ghost-link {
    color: var(--text-secondary);
    background: transparent;
  }

  .ex-pane-ghost-link:hover {
    background: var(--bg-hover);
    color: var(--text-strong);
  }

  .ex-pane-selected-element-link {
    border: var(--border-w) solid var(--border-default);
    border-radius: var(--radius-sm);
    background: var(--bg-surface);
    color: var(--text-body);
  }

  .ex-pane-selected-element-link:hover {
    border-color: var(--border-strong);
    background: var(--bg-hover);
  }

  .ex-pane-selection-kind {
    background: var(--bg-sunken);
    color: var(--text-muted);
  }

  .ex-pane-selection-open {
    color: var(--text-muted);
  }

  .ex-pane-nav-row {
    border: 0;
    border-radius: var(--radius-md);
    background: transparent;
    color: var(--text-body);
  }

  .ex-pane-nav-row:hover,
  .ex-pane-nav-row:focus-visible {
    background: color-mix(in srgb, var(--accent) 5%, transparent);
  }

  .ex-pane-nav-row:focus-visible {
    outline: var(--focus-ring-w) solid var(--focus-ring);
    outline-offset: var(--focus-ring-offset);
  }

  .ex-pane-nav-row__icon {
    color: var(--text-muted);
  }

  .ex-pane-nav-row__label {
    color: var(--text-body);
  }

  .ex-pane-nav-row__count {
    border-radius: var(--radius-pill);
    background: var(--bg-sunken);
    color: var(--text-secondary);
  }
`;

const headerBaseUX = css`
  z-index: var(--z-sticky);
  display: flex;
  flex: 0 0 var(--header-h);
  align-items: stretch;
  height: var(--header-h);

  @media (max-width: 900px) {
    padding-right: 0;
  }
`;

const headerSkinX = css`
  border-bottom: var(--border-w) solid var(--border-subtle);
  background: var(--bg-surface);
`;

const brandClass = css`
  --ex-brand-min-w: 160px;
  display: flex;
  flex: 0 0 var(--ex-current-left-width);
  align-items: center;
  gap: var(--space-5);
  box-sizing: border-box;
  border-right: var(--border-w) solid var(--border-subtle);
  padding: 0 var(--space-10);

  @media (max-width: 900px) {
    flex-basis: auto;
    width: auto;
    min-width: var(--ex-brand-min-w);
  }
`;

const brandMarkClass = css`
  position: static;
  top: auto;
  left: auto;
  display: block;
  flex: 0 0 auto;
  width: var(--space-10);
  height: var(--space-10);
  transform: none;
`;

const brandNameClass = css`
  --ex-brand-name-nudge-y: 0.5px;
  display: inline-flex;
  align-items: center;
  color: var(--text-strong);
  font-size: var(--text-md);
  font-weight: var(--weight-semibold);
  letter-spacing: 0.14em;
  line-height: 1;
  transform: translateY(var(--ex-brand-name-nudge-y));
`;

const headerTabsClass = css`
  display: flex;
  flex: 1 1 auto;
  min-width: 0;
  align-items: stretch;
  overflow-x: auto;
  overflow-y: hidden;
  padding-left: calc(var(--space-16) - var(--space-7));
  --rq-tabs-h: 100%;
  --rq-tabs-border-bottom: 0;
  --rq-tab-h: 100%;
`;

const headerActionsClass = css`
  display: flex;
  flex: 0 0 auto;
  align-items: center;
  gap: var(--space-2);
  padding: 0 var(--space-10) 0 var(--space-4);

  @media (max-width: 900px) {
    padding-right: var(--space-6);
  }
`;

const mainClass = css`
  position: relative;
  display: flex;
  flex: 1 1 auto;
  min-height: 0;
  background: var(--bg-canvas);
`;

const contentClass = css`
  position: relative;
  flex: 1 1 auto;
  min-width: 0;
  min-height: 0;
  overflow: hidden;
  background: var(--bg-canvas);
`;

const collapseBaseUX = css`
  position: absolute;
  top: 50%;
  left: calc(var(--ex-current-left-width) - var(--space-6));
  z-index: calc(var(--z-sticky) + 1);
  display: inline-flex;
  width: var(--space-12);
  height: var(--space-16);
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transform: translateY(-50%);

  &.is-collapsed {
    left: var(--space-6);
  }

  &:focus-visible {
    outline: none;
  }

  svg {
    display: block;
    width: var(--icon-sm);
    height: var(--icon-sm);
  }
`;

const collapseSkinX = css`
  border: var(--border-w) solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-surface);
  box-shadow: var(--shadow-xs);
  color: var(--text-muted);

  &:hover,
  &:focus-visible {
    border-color: var(--border-strong);
    color: var(--text-strong);
  }
`;

const schemaAlertBaseUX = css`
  display: flex;
  align-items: center;
  gap: var(--space-5);
  margin: var(--space-4);
  padding: var(--space-6) var(--space-8);

  svg {
    flex: none;
  }
`;

const schemaAlertSkinX = css`
  border: var(--border-w) solid color-mix(in srgb, var(--danger) 36%, var(--border-default));
  border-radius: var(--radius-md);
  background: var(--danger-tint);
  color: var(--text-strong);
  box-shadow: var(--shadow-xs);

  svg {
    color: var(--danger);
  }
`;

const iconSmClass = css`
  width: var(--space-8);
  height: var(--space-8);
  flex: none;
`;

export function App() {
  // Load once: the seed is an immutable generated snapshot for the served workspace.
  const result = useMemo(() => loadStore(devFixture), []);

  if (!result.ok) {
    return <MissingStoreNotice reason={result.reason} detail={result.detail} />;
  }

  return (
    <StoreProvider store={result.store} schemaMismatch={result.schemaMismatch}>
      <SearchIndexProvider>
        <ExplorerUiStateProvider>
          <ExplorerShell schemaMismatch={result.schemaMismatch} />
        </ExplorerUiStateProvider>
      </SearchIndexProvider>
    </StoreProvider>
  );
}

function ExplorerShell({ schemaMismatch }: { schemaMismatch: string | null }) {
  const { route, navigateView, openElement, closeElement } = useHashRoute();
  const [helpOpen, setHelpOpen] = useState(false);
  const [leftPaneOpen, setLeftPaneOpen] = useState(true);
  const [leftPaneWidth, setLeftPaneWidth] = useState(readStoredLeftPaneWidth);
  const [ontologyNodeId, setOntologyNodeId] = useState<string | null>(null);
  const shellRef = useRef<HTMLDivElement | null>(null);
  const leftPaneWidthRef = useRef(leftPaneWidth);
  const { isDark, toggleTheme } = useTheme();
  const sidePaneView =
    route.view === "content" || (route.view === "resources" && route.param)
      ? "model"
      : route.view;

  // Route changes update the document title to match the active Explorer view.
  useEffect(() => {
    document.title = `Reqvire Explorer — ${VIEW_TITLES[route.view]}`;
  }, [route.view]);

  useEffect(() => {
    leftPaneWidthRef.current = leftPaneWidth;
    shellRef.current?.style.setProperty("--ex-left-pane-width", `${leftPaneWidth}px`);
    window.localStorage.setItem(LEFT_PANE_WIDTH_STORAGE_KEY, String(leftPaneWidth));
  }, [leftPaneWidth]);

  useEffect(() => {
    function handleResize() {
      setLeftPaneWidth((width) => clampLeftPaneWidth(width));
    }

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  function toggleLeftPane() {
    setLeftPaneOpen((open) => !open);
  }

  function handleLeftPaneResizePointerDown(event: ReactPointerEvent<HTMLDivElement>) {
    if (!leftPaneOpen || event.button !== 0) return;

    const startX = event.clientX;
    const startWidth = leftPaneWidthRef.current;
    let nextWidth = startWidth;
    shellRef.current?.classList.add("is-left-resizing");
    document.body.style.cursor = "ew-resize";
    document.body.style.userSelect = "none";

    function handlePointerMove(moveEvent: PointerEvent) {
      const delta = moveEvent.clientX - startX;
      nextWidth = clampLeftPaneWidth(startWidth + delta);
      shellRef.current?.style.setProperty("--ex-left-pane-width", `${nextWidth}px`);
    }

    function finishPointerDrag() {
      window.removeEventListener("pointermove", handlePointerMove);
      window.removeEventListener("pointerup", finishPointerDrag);
      window.removeEventListener("pointercancel", finishPointerDrag);
      document.body.style.cursor = "";
      document.body.style.userSelect = "";
      shellRef.current?.classList.remove("is-left-resizing");
      leftPaneWidthRef.current = nextWidth;
      setLeftPaneWidth(nextWidth);
    }

    window.addEventListener("pointermove", handlePointerMove);
    window.addEventListener("pointerup", finishPointerDrag);
    window.addEventListener("pointercancel", finishPointerDrag);
  }

  function handleLeftPaneResizeKeyDown(event: ReactKeyboardEvent<HTMLDivElement>) {
    if (!leftPaneOpen) return;

    if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
      event.preventDefault();
      const direction = event.key === "ArrowLeft" ? -1 : 1;
      const step = event.shiftKey ? 40 : 16;
      setLeftPaneWidth((width) => clampLeftPaneWidth(width + direction * step));
    }
  }

  const shellStyle = {
    "--ex-left-pane-width": `${leftPaneWidth}px`,
  } as CSSProperties;

  return (
    <div
      ref={shellRef}
      className={cx(
        "ex-app",
        shellBaseUX,
        shellSkinX,
        !leftPaneOpen && "is-left-collapsed",
      )}
      style={shellStyle}
    >
      <ExplorerHeader
        activeView={route.view}
        isDark={isDark}
        onNavigate={navigateView}
        onOpenHelp={() => setHelpOpen(true)}
        onToggleTheme={toggleTheme}
      />
      <div className={cx(mainClass)}>
        <ExplorerSidePane
          activeView={sidePaneView}
          open={leftPaneOpen}
          chrome="app"
          onToggle={toggleLeftPane}
          onNavigate={navigateView}
          onOpenElement={openElement}
          onOpenOntologyNode={setOntologyNodeId}
        />
        <button
          type="button"
          className={cx(collapseBaseUX, collapseSkinX, !leftPaneOpen && "is-collapsed")}
          aria-label={leftPaneOpen ? "Collapse explorer" : "Expand explorer"}
          aria-expanded={leftPaneOpen}
          title={leftPaneOpen ? "Collapse explorer" : "Expand explorer"}
          onClick={toggleLeftPane}
        >
          {leftPaneOpen ? <Icon name="chevron-left" /> : <Icon name="chevron-right" />}
        </button>
        <div
          className={cx(paneResizerClass)}
          role="separator"
          aria-label="Resize explorer pane"
          aria-orientation="vertical"
          aria-valuemin={LEFT_PANE_WIDTH_MIN}
          aria-valuemax={LEFT_PANE_WIDTH_MAX}
          aria-valuenow={leftPaneWidth}
          tabIndex={leftPaneOpen ? 0 : -1}
          onPointerDown={handleLeftPaneResizePointerDown}
          onKeyDown={handleLeftPaneResizeKeyDown}
        />
        <div className={cx(contentClass)}>
          {schemaMismatch && (
            <div className={cx(schemaWarningClass)}>
              <div role="alert" className={cx(schemaAlertBaseUX, schemaAlertSkinX)}>
                <Icon name="alert-triangle" className={cx(iconSmClass)} />
                <span>Store schema mismatch: {schemaMismatch}</span>
              </div>
            </div>
          )}

          <ActiveView
            view={route.view}
            param={route.param}
            onNavigate={navigateView}
            onOpenElement={openElement}
          />
        </div>
      </div>

      <HelpModal open={helpOpen} onOpenChange={setHelpOpen} />

      <ElementDetailModal
        identifier={route.elementId}
        onClose={closeElement}
        onOpenElement={openElement}
      />
      <OntologyNodeDetailModal
        nodeId={ontologyNodeId}
        onClose={() => setOntologyNodeId(null)}
      />
    </div>
  );
}

function readStoredLeftPaneWidth() {
  if (typeof window === "undefined") return LEFT_PANE_WIDTH_DEFAULT;

  const stored = Number(window.localStorage.getItem(LEFT_PANE_WIDTH_STORAGE_KEY));
  return clampLeftPaneWidth(Number.isFinite(stored) ? stored : LEFT_PANE_WIDTH_DEFAULT);
}

function clampLeftPaneWidth(width: number) {
  const viewportMax =
    typeof window === "undefined"
      ? LEFT_PANE_WIDTH_MAX
      : Math.max(
          LEFT_PANE_WIDTH_MIN,
          Math.min(LEFT_PANE_WIDTH_MAX, window.innerWidth - 420),
        );

  return Math.round(
    Math.min(Math.max(width, LEFT_PANE_WIDTH_MIN), viewportMax),
  );
}

function ExplorerHeader({
  activeView,
  isDark,
  onNavigate,
  onOpenHelp,
  onToggleTheme,
}: {
  activeView: ViewId;
  isDark: boolean;
  onNavigate: (view: ViewId) => void;
  onOpenHelp: () => void;
  onToggleTheme: () => void;
}) {
  const tabItems: TabItem<ViewId>[] = [
    { value: "model", label: "Model", icon: <Icon name="folder" /> },
    { value: "ontologies", label: "Ontologies", icon: <Icon name="globe" /> },
    { value: "traces", label: "Traces", icon: <Icon name="activity" /> },
    { value: "coverage", label: "Coverage", icon: <Icon name="pie-chart" /> },
  ];
  const effectiveView: ViewId = activeView === "files" || activeView === "content" || activeView === "resources"
    ? "model"
    : activeView;

  return (
    <header className={cx(headerBaseUX, headerSkinX)}>
      <div className={cx(brandClass)}>
        <ReqvireRailMark className={cx(brandMarkClass)} />
        <span className={cx(brandNameClass)}>REQVIRE</span>
      </div>
      <nav className={cx(headerTabsClass)} aria-label="Explorer views">
        <Tabs
          items={tabItems}
          value={effectiveView}
          onChange={onNavigate}
          variant="underline"
        />
      </nav>
      <div className={cx(headerActionsClass)}>
        <IconButton aria-label="Search" onClick={() => onNavigate("search")}>
          <Icon name="search" />
        </IconButton>
        <IconButton
          aria-label={isDark ? "Switch to light mode" : "Switch to dark mode"}
          onClick={onToggleTheme}
        >
          {isDark ? <Icon name="sun" /> : <Icon name="moon" />}
        </IconButton>
        <IconButton aria-label="Help" onClick={onOpenHelp}>
          <Icon name="help-circle" />
        </IconButton>
      </div>
    </header>
  );
}

function ActiveView({
  view,
  param,
  onNavigate,
  onOpenElement,
}: {
  view: ReturnType<typeof useHashRoute>["route"]["view"];
  param: string | null;
  onNavigate: (view: ReturnType<typeof useHashRoute>["route"]["view"]) => void;
  onOpenElement: (id: string) => void;
}) {
  switch (view) {
    case "model":
      return <ModelView onOpenElement={onOpenElement} />;
    case "traces":
      return <TracesView activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    case "ontologies":
      return <OntologiesView activeView={view} onNavigate={onNavigate} />;
    case "coverage":
      return <CoverageView activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    case "resources":
      return <ResourcesView resourceId={param} activeView={view} onNavigate={onNavigate} />;
    case "files":
      return <FilesView path={param} activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    case "content":
      return <ContentView path={param ?? ""} />;
    case "search":
      return <SearchView initialQuery={param} activeView={view} onNavigate={onNavigate} onOpenElement={onOpenElement} />;
    default:
      return <ModelView onOpenElement={onOpenElement} />;
  }
}
