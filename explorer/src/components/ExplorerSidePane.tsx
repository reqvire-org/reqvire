import { useEffect, useMemo, useState, type FormEvent } from "react";
import { css, cx } from "@linaria/atomic";
import {
  Badge,
  Button,
  ElementIcon,
  Icon,
  IconButton,
  SearchInput,
  SidebarSection,
  Stat,
  StatRow,
  ToggleRow,
  TreeItem,
  type DesignSystemColorToken,
} from "@ds";
import { useStore } from "../store/StoreContext";
import { VIEW_TITLES, type ViewId } from "../router/routes";
import type {
  ExplorerProjectStore,
  KnowledgeGraphNode,
  OntologyGraphNode,
  ProjectStoreElement,
  ProjectStoreFile,
} from "../store/types";
import {
  SEARCH_KINDS,
  useExplorerUiState,
  type SearchKind,
} from "./ExplorerUiState";
import { PaneChromeHeader, ReqvireRailMark, railMarkClass } from "./PaneChrome";
import { buildTraceFiles, type TraceFileNode } from "../lib/traces";

interface ExplorerSidePaneProps {
  activeView: ViewId;
  open: boolean;
  chrome?: "standalone" | "app";
  onToggle: () => void;
  onNavigate: (view: ViewId) => void;
  onOpenElement: (id: string) => void;
  onOpenOntologyNode: (id: string) => void;
}

interface TreeFolder {
  path: string;
  name: string;
  folders: TreeFolder[];
  files: ProjectStoreFile[];
}

const ROOT_PATH = "__root__";

const sideContentClass = css`
  box-sizing: border-box;
  display: flex;
  min-width: 0;
  min-height: 0;
  flex: 1 1 auto;
  flex-direction: column;
  width: calc(var(--ex-left-pane-width) - var(--ex-left-pane-collapsed-width));
`;

const sideContentAppClass = css`
  display: flex;
  flex: 1 1 auto;
  flex-direction: column;
  width: 100%;
  height: 100%;
  min-height: 0;
  overflow: hidden;
`;

const treeClass = css`
  box-sizing: border-box;
`;

const treeTabClass = css`
  box-sizing: border-box;
`;

const treeTabLabelClass = css`
  box-sizing: border-box;
`;

const treeTabToggleClass = css`
  box-sizing: border-box;
`;

const globalSearchClass = css`
  box-sizing: border-box;
`;

const globalSearchControlClass = css`
  box-sizing: border-box;
`;

const globalSearchResultsClass = css`
  box-sizing: border-box;
`;

const paneControlsClass = css`
  box-sizing: border-box;
`;

const paneControlsTitleClass = css`
  box-sizing: border-box;
`;

const paneSummaryClass = css`
  box-sizing: border-box;
`;

const summaryClass = css`
  box-sizing: border-box;
`;

const paneNavListClass = css`
  box-sizing: border-box;
`;

const paneNavRowClass = css`
  box-sizing: border-box;
`;

const paneNavRowIconClass = css`
  box-sizing: border-box;
`;

const paneNavRowLabelClass = css`
  box-sizing: border-box;
`;

const paneNavRowCountClass = css`
  box-sizing: border-box;
`;

const paneActionRowClass = css`
  box-sizing: border-box;
`;

const paneGhostLinkClass = css`
  box-sizing: border-box;
`;

const paneLegendClass = css`
  box-sizing: border-box;
`;

const paneLegendRowClass = css`
  box-sizing: border-box;
`;

const paneLegendTextClass = css`
  box-sizing: border-box;
`;

const paneSymbolClass = css`
  box-sizing: border-box;
`;

const graphControlSwatchClass = css`
  display: inline-flex;
  flex: 0 0 auto;
  width: var(--icon-xs);
  height: var(--icon-xs);
  box-sizing: border-box;
  border: var(--border-w) solid currentColor;
  border-radius: var(--radius-xs);
`;

const paneSelectedElementClass = css`
  box-sizing: border-box;
`;

const paneSelectedElementLinkClass = css`
  box-sizing: border-box;
`;

const paneSelectionRowClass = css`
  box-sizing: border-box;
`;

const paneSelectionNameClass = css`
  box-sizing: border-box;
`;

const paneSelectionKindClass = css`
  box-sizing: border-box;
`;

const paneSelectionOpenClass = css`
  box-sizing: border-box;
`;

const paneSelectionHintClass = css`
  box-sizing: border-box;
`;

const paneSectionLabelClass = css`
  box-sizing: border-box;
`;

const treeNodeClass = css`
  box-sizing: border-box;
`;

const emptyClass = css`
  box-sizing: border-box;
`;

const baseUX = css`
  position: fixed;
  inset: 0 auto 0 0;
  z-index: 45;
  display: flex;
  width: var(--ex-current-left-width);
  box-sizing: border-box;
  flex-direction: row;
  overflow: hidden;

  &.is-standalone {
    --ex-left-pane-width: 380px;
    --ex-left-pane-collapsed-width: 30px;
    --ex-current-left-width: var(--ex-left-pane-width);
  }

  &.is-collapsed .${sideContentClass} {
    display: none;
  }

  .ex-app & {
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

  .ex-app &.is-collapsed {
    display: none;
  }

  .${sideContentClass} {
    display: flex;
    min-width: 0;
    min-height: 0;
    flex: 1 1 auto;
    flex-direction: column;
    width: calc(var(--ex-left-pane-width) - var(--ex-left-pane-collapsed-width));
  }

  .ex-app & .${sideContentClass} {
    display: flex;
    flex: 1 1 auto;
    flex-direction: column;
    width: 100%;
    height: 100%;
    min-height: 0;
    overflow: hidden;
  }

  .${treeTabClass} {
    position: relative;
    display: flex;
    min-height: 0;
    flex: 0 0 var(--ex-left-pane-collapsed-width);
    width: var(--ex-left-pane-collapsed-width);
    align-items: flex-start;
    justify-content: center;
    padding-top: var(--space-5);
    border: 0;
    border-left: 0;
    cursor: pointer;
  }

  .ex-app & .${treeTabClass} {
    display: none;
  }

  &:not(.is-collapsed) .${treeTabLabelClass},
  &:not(.is-collapsed) .${treeTabClass} .${railMarkClass} {
    display: none;
  }

  .${treeTabLabelClass} {
    display: inline-block;
    margin-top: var(--space-14);
    writing-mode: vertical-rl;
    font-size: var(--text-micro);
    font-weight: var(--weight-bold);
    letter-spacing: 0.075em;
    text-transform: uppercase;
  }

  .${treeTabToggleClass} {
    position: absolute;
    top: 50%;
    left: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    width: var(--icon-lg);
    height: var(--icon-lg);
    transform: translate(-50%, -50%);
  }

  .${globalSearchClass} {
    flex: 0 0 auto;
    margin: var(--space-6) var(--space-7) 0;
  }

  .ex-app & .${globalSearchClass} {
    margin: var(--space-12) var(--space-10) 0;
  }

  .${globalSearchControlClass} {
    position: relative;
    --rq-search-input-h: var(--control-lg);
    --rq-search-input-p: 0 var(--space-8) 0 calc(var(--space-16) + var(--space-3));
    --rq-search-input-fs: var(--text-base);
    --rq-search-icon-left: var(--space-8);
    --rq-search-icon-sz: var(--icon-md);
  }

  .${globalSearchResultsClass} {
    --ex-pane-search-results-max-h: 260px;
    max-height: var(--ex-pane-search-results-max-h);
    overflow: auto;
    margin: var(--space-4) 0 0;
    padding: var(--space-2);
    list-style: none;
  }

  .${globalSearchResultsClass}:empty {
    display: none;
  }

  .${globalSearchResultsClass} button,
  .${globalSearchResultsClass} a {
    display: grid;
    width: 100%;
    box-sizing: border-box;
    gap: var(--space-1);
    padding: var(--space-3) var(--space-4);
    border: 0;
    border-radius: var(--radius-sm);
    text-align: left;
    text-decoration: none;
    cursor: pointer;
  }

  .${globalSearchResultsClass} small {
    font-size: var(--text-micro);
  }

  .${globalSearchResultsClass} .ontology-graph-result {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
    border-radius: var(--radius-sm);
    padding: var(--space-3) var(--space-4);
    cursor: pointer;
    font-size: var(--text-caption);
  }

  .${globalSearchResultsClass} .ontology-graph-badge {
    display: inline-block;
    flex-shrink: 0;
    border-radius: var(--radius-pill);
    padding: var(--space-1) var(--space-4);
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
  }

  .${paneControlsClass} {
    display: grid;
    flex: 0 0 auto;
    gap: var(--space-4);
    padding: var(--space-6) var(--space-7) var(--space-7);
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
    --rq-togglerow-static-cursor: default;
  }

  .ex-app & .${paneControlsClass} {
    display: flex;
    flex: 1 1 auto;
    min-height: 0;
    flex-direction: column;
    gap: 0;
    overflow-x: hidden;
    overflow-y: auto;
    padding: var(--space-12) var(--space-10) var(--space-16);
    scrollbar-gutter: stable;
  }

  .${paneControlsTitleClass} {
    margin: 0 0 var(--space-7);
    font-size: var(--text-lg);
    font-weight: var(--weight-semibold);
    letter-spacing: 0;
    line-height: var(--leading-tight);
  }

  .${paneSectionLabelClass} {
    display: block;
    margin: var(--space-6) 0 var(--space-2);
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .ex-app & .${paneSectionLabelClass} {
    margin: var(--space-12) 0 var(--space-5);
    padding: 0 var(--space-2);
    letter-spacing: var(--tracking-label);
    line-height: 1;
  }

  .${paneLegendClass} {
    display: grid;
    gap: var(--space-4);
  }

  .ex-app & .${paneLegendClass} {
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

  .${paneSummaryClass} {
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    gap: var(--space-4);
  }

  .ex-app & .${paneSummaryClass} {
    flex-direction: column;
    flex-wrap: nowrap;
    align-items: stretch;
    gap: var(--space-5);
    margin: 0 0 var(--space-6);
  }

  .${paneSummaryClass} .${paneSectionLabelClass} {
    margin: 0 var(--space-1) 0 0;
  }

  .ex-app & .${paneSummaryClass} .${paneSectionLabelClass} {
    margin: 0;
    padding: 0;
  }

  .${paneSummaryClass} .${summaryClass} {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-3) var(--space-8);
    --rq-stat-display: flex;
    --rq-stat-min-w: 0;
    --rq-stat-jc: space-between;
  }

  .${paneLegendRowClass} {
    display: flex;
    align-items: center;
    gap: var(--space-6);
    min-height: var(--control-sm);
    padding: 0 var(--space-7);
  }

  .${paneLegendTextClass} {
    font-size: var(--text-caption);
    line-height: 1.3;
  }

  .${paneSymbolClass} {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: var(--control-md);
    height: var(--icon-lg);
    border-radius: var(--radius-md);
    font-size: var(--text-micro);
    font-weight: var(--weight-bold);
    line-height: 1;
  }

  .${treeClass} {
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
    --rq-treeitem-icon-color: var(--text-secondary);
    min-height: 0;
    flex: 1 1 auto;
    overflow-x: hidden;
    overflow-y: auto;
    padding: var(--space-5) 0 var(--space-7);
    border-top: var(--border-w) solid;
    scrollbar-gutter: stable;
  }

  .${treeNodeClass} {
    min-width: 0;
    margin: 0;
  }

  .${treeNodeClass} > summary {
    list-style: none;
  }

  .${treeNodeClass} > summary::-webkit-details-marker {
    display: none;
  }

  .${paneSelectedElementClass} {
    display: grid;
    gap: var(--space-5);
    margin: 0;
  }

  .${paneSelectedElementClass} .${paneSectionLabelClass} {
    margin: 0;
  }

  .${paneSelectedElementLinkClass} {
    display: inline-flex;
    width: 100%;
    min-width: 0;
    align-items: center;
    justify-content: flex-start;
    gap: var(--space-3);
    padding: var(--space-2) var(--space-6);
    border-radius: var(--radius-sm);
    font-size: var(--text-sm);
    font-weight: var(--weight-medium);
    text-align: left;
    text-decoration: none;
    cursor: pointer;
    transition:
      background var(--dur-fast),
      border-color var(--dur-fast);
  }

  .${paneSelectedElementLinkClass} span:last-child {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .${paneSelectionRowClass} {
    display: flex;
    min-width: 0;
    align-items: center;
    gap: var(--space-2);
  }

  .${paneSelectionRowClass} .${paneSelectedElementLinkClass} {
    flex: 1 1 auto;
  }

  .${paneSelectionNameClass} {
    flex: 1 1 auto;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .${paneSelectionKindClass} {
    flex: 0 0 auto;
    overflow: visible;
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-pill);
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    line-height: 1.2;
  }

  .${paneSelectionOpenClass} {
    flex: 0 0 auto;
  }

  .${paneSelectionHintClass} {
    margin: 0;
    font-size: var(--text-caption);
  }

  .${paneNavListClass} {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .${paneNavRowClass} {
    display: grid;
    grid-template-columns: var(--icon-md) minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--space-5);
    width: 100%;
    min-height: var(--control-md);
    padding: 0 var(--space-5);
    border: 0;
    border-radius: var(--radius-md);
    cursor: pointer;
    font: inherit;
    text-align: left;
  }

  .${paneNavRowIconClass} {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .${paneNavRowLabelClass} {
    overflow: hidden;
    font-size: var(--text-sm);
    font-weight: var(--weight-medium);
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .${paneNavRowCountClass} {
    display: inline-flex;
    min-width: var(--control-xs);
    height: var(--control-xs);
    align-items: center;
    justify-content: center;
    padding: 0 var(--space-3);
    border-radius: var(--radius-pill);
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    line-height: 1;
  }

  .${paneActionRowClass} {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-5);
    margin: 0;
  }

  .${paneGhostLinkClass} {
    display: inline-flex;
    height: var(--control-sm);
    align-items: center;
    gap: var(--space-4);
    padding: 0 var(--space-6);
    border-radius: var(--radius-sm);
    font-size: var(--text-caption);
    font-weight: var(--weight-medium);
    text-decoration: none;
  }

  .${paneGhostLinkClass} svg {
    display: block;
    width: var(--icon-sm);
    height: var(--icon-sm);
    flex: 0 0 auto;
  }

  .${emptyClass} {
    font-size: var(--text-sm);
    font-style: italic;
    line-height: 1.45;
  }
`;

const appRootClass = css`
  position: relative;
  inset: auto;
  z-index: auto;
  align-self: stretch;
  flex: 0 0 var(--ex-current-left-width);
  width: var(--ex-current-left-width);
  min-width: 0;
  min-height: 0;
  height: 100%;
`;

const skinX = css`
  border-right: var(--border-w) solid var(--border-subtle);
  background: var(--bg-surface);
  color: var(--text-body);

  .${treeTabClass} {
    background: var(--bg-surface);
    color: var(--text-secondary);
  }

  .${treeTabClass}:hover,
  .${treeTabClass}:focus-visible {
    background: var(--bg-hover);
    color: var(--text-secondary);
    outline: 0;
  }

  .${treeTabToggleClass} {
    color: var(--text-muted);
  }

  .${globalSearchControlClass} {
    --rq-search-input-border: var(--border-w) solid var(--border-subtle);
    --rq-search-input-bg: var(--bg-canvas);
    --rq-search-input-color: var(--text-body);
    --rq-search-input-placeholder-color: var(--text-muted);
    --rq-search-input-focus-border-color: color-mix(in srgb, var(--accent) 55%, transparent);
    --rq-search-input-focus-shadow: var(--ring-focus);
  }

  .${globalSearchResultsClass} {
    border: var(--border-w) solid var(--border-subtle);
    border-radius: var(--radius-md);
    background: var(--bg-surface);
    box-shadow: var(--shadow-lg);
  }

  .${globalSearchResultsClass} button,
  .${globalSearchResultsClass} a {
    background: transparent;
    color: var(--text-body);
  }

  .${globalSearchResultsClass} button:hover,
  .${globalSearchResultsClass} a:hover {
    background: color-mix(in srgb, var(--accent) 6%, transparent);
  }

  .${globalSearchResultsClass} small {
    color: var(--text-muted);
  }

  .${globalSearchResultsClass} .ontology-graph-result:hover {
    background: var(--bg-hover);
  }

  .${globalSearchResultsClass} .ontology-graph-result.text-gray-400 {
    color: var(--text-muted);
  }

  .${globalSearchResultsClass} .ontology-graph-badge {
    color: var(--slate-0);
  }

  .${paneControlsClass},
  .${treeClass} {
    border-color: var(--border-subtle);
  }

  .${paneControlsClass} {
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

  .${paneControlsTitleClass} {
    color: var(--text-strong);
  }

  .${paneSectionLabelClass},
  .${paneLegendTextClass},
  .${paneSelectionOpenClass},
  .${emptyClass} {
    color: var(--text-muted);
  }

  .${paneSymbolClass} {
    border: var(--border-w) solid var(--border-subtle);
    background: var(--bg-sunken);
    color: var(--text-link);
  }

  .${treeClass} {
    --rq-treeitem-twist-color: var(--text-muted);
  }

  .${paneSelectedElementLinkClass} {
    border: var(--border-w) solid var(--border-default);
    background: var(--bg-surface);
    color: var(--text-body);
  }

  .${paneSelectedElementLinkClass}:hover {
    border-color: var(--border-strong);
    background: var(--bg-hover);
  }

  .${paneSelectionKindClass} {
    background: var(--bg-sunken);
    color: var(--text-muted);
  }

  .${paneNavRowClass} {
    background: transparent;
    color: var(--text-body);
  }

  .${paneNavRowClass}:hover,
  .${paneNavRowClass}:focus-visible {
    background: color-mix(in srgb, var(--accent) 5%, transparent);
  }

  .${paneNavRowClass}:focus-visible {
    outline: var(--focus-ring-w) solid var(--focus-ring);
    outline-offset: var(--focus-ring-offset);
  }

  .${paneNavRowIconClass} {
    color: var(--text-muted);
  }

  .${paneNavRowLabelClass} {
    color: var(--text-body);
  }

  .${paneNavRowCountClass} {
    background: var(--bg-sunken);
    color: var(--text-secondary);
  }

  .${paneGhostLinkClass} {
    background: transparent;
    color: var(--text-secondary);
  }

  .${paneGhostLinkClass}:hover {
    background: var(--bg-hover);
    color: var(--text-strong);
  }
`;

interface TracePaneVerification {
  id: string;
  name: string;
  type?: string;
}

interface TracePaneFile {
  path: string;
  name: string;
  verifications: TracePaneVerification[];
}

interface TracePaneFolder {
  path: string;
  name: string;
  folders: TracePaneFolder[];
  files: TracePaneFile[];
}

type CoverageSectionId =
  | "overview"
  | "capability-coverage"
  | "unverified-requirements"
  | "unimplemented-requirements"
  | "unsatisfied-verifications"
  | "orphaned-verifications";

export function ExplorerSidePane({
  activeView,
  open,
  chrome = "standalone",
  onToggle,
  onNavigate,
  onOpenElement,
  onOpenOntologyNode,
}: ExplorerSidePaneProps) {
  const { store, elementById } = useStore();
  const ui = useExplorerUiState();
  const tree = useMemo(() => buildFileTree(store.files), [store.files]);
  const traceTree = useMemo(() => buildTraceFileTree(buildTraceFiles(store)), [store]);
  const graphModelActive = activeView === "model" && ui.modelMode === "graph";
  const showProjectTree = (activeView === "model" || activeView === "files") && !graphModelActive;
  const title = graphModelActive ? "Graph Explorer" : `${VIEW_TITLES[activeView]} Explorer`;
  const showStandaloneChrome = chrome === "standalone";
  const appChrome = chrome === "app";

  return (
    <aside
      className={cx(
        "ex-side-pane",
        baseUX,
        skinX,
        appChrome ? appRootClass : "is-standalone",
        !open && "is-collapsed",
      )}
      aria-label="Explorer navigation"
    >
      <div className={cx("ex-side-content", sideContentClass, appChrome && sideContentAppClass)}>
        {showStandaloneChrome && <PaneChromeHeader title={title} />}
        {activeView === "ontologies" && <OntologyGraphSearch />}
        <ExplorerViewControls
          activeView={activeView}
          onOpenElement={onOpenElement}
          onOpenOntologyNode={onOpenOntologyNode}
        />
        {activeView === "traces" && (
          <div className={cx("ex-tree", treeClass)} aria-label="Verification trace tree">
            <TraceTreeFolderNode folder={traceTree} depth={0} />
          </div>
        )}
        {showProjectTree && (
          <div className={cx("ex-tree", treeClass)} aria-label="Project tree">
            <TreeFolderNode
              folder={tree}
              activeView={activeView}
              elementById={elementById}
              onNavigate={onNavigate}
              onOpenElement={onOpenElement}
              depth={0}
            />
          </div>
        )}
      </div>
      {showStandaloneChrome && (
        <button
          type="button"
          className={cx("ex-tree-tab", treeTabClass)}
          aria-label={open ? "Collapse explorer pane" : "Expand explorer pane"}
          aria-expanded={open}
          onClick={onToggle}
        >
          <ReqvireRailMark />
          <span className={cx("ex-tree-tab-label", treeTabLabelClass)}>Explorer</span>
          <span className={cx("ex-tree-toggle", treeTabToggleClass)} aria-hidden="true">
            {open ? <Icon name="chevron-left" /> : <Icon name="chevron-right" />}
          </span>
        </button>
      )}
    </aside>
  );
}

function OntologyGraphSearch() {
  const [query, setQuery] = useState("");

  useEffect(() => {
    window.filterOntologyGraph?.(query);
  }, [query]);

  function submitSearch(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    window.filterOntologyGraph?.(query);
  }

  return (
    <form className={cx("ex-global-search", globalSearchClass)} role="search" onSubmit={submitSearch}>
      <SearchInput
        id="ontology-graph-search"
        className={cx("ex-global-search-control", globalSearchControlClass)}
        size="lg"
        aria-label="Search Explorer"
        type="search"
        placeholder="Search ontology graph..."
        value={query}
        onChange={(event) => setQuery(event.target.value)}
      />
      <ul id="ontology-graph-results" className={cx("ontology-graph-results", globalSearchResultsClass)} />
    </form>
  );
}

function ExplorerViewControls({
  activeView,
  onOpenElement,
  onOpenOntologyNode,
}: {
  activeView: ViewId;
  onOpenElement: (id: string) => void;
  onOpenOntologyNode: (id: string) => void;
}) {
  const ui = useExplorerUiState();
  const { store, elementById } = useStore();

  const graphControlsActive = (activeView === "model" && ui.modelMode === "graph");
  const graphTypeOptions = useMemo(
    () => buildKnowledgeGraphTypeOptions(store.knowledge_graph.nodes ?? [], elementById),
    [elementById, store.knowledge_graph.nodes],
  );
  const searchElementTypeOptions = useMemo(
    () => buildSearchElementTypeOptions(store.elements),
    [store.elements],
  );
  const searchKindCounts = useMemo(
    () => buildSearchKindCounts(store),
    [store.elements.length, store.files.length, store.resources.length, store.ontology.graph_data?.nodes],
  );

  if ((activeView === "model" || activeView === "files") && !graphControlsActive) {
    return null;
  }

  if (graphControlsActive) {
    return (
      <section className={cx("ex-pane-controls", paneControlsClass)} aria-label="Graph controls">
        <SidebarSection title="Summary" className={cx("ex-pane-summary", paneSummaryClass)} aria-label="Summary">
          <StatRow className={cx("ex-summary", summaryClass)}>
            <Stat label="Submodels" value={formatSummaryValue(store.knowledge_graph.summary?.submodels ?? store.knowledge_graph.submodels?.length ?? 0)} />
            <Stat label="Elements" value={formatSummaryValue(store.knowledge_graph.summary?.elements ?? store.elements.length)} />
            <Stat label="Relations" value={formatSummaryValue(store.knowledge_graph.summary?.relations ?? store.relations.length)} />
            <Stat label="Attachments" value={formatSummaryValue(store.knowledge_graph.summary?.attachments ?? store.attachments.length)} />
          </StatRow>
        </SidebarSection>
        <KnowledgeGraphSelectedElementLink
          selectedNodeId={ui.knowledgeGraphSelectionId}
          nodes={store.knowledge_graph.nodes ?? []}
          elementById={elementById}
          onOpenElement={onOpenElement}
        />
        <PaneSectionLabel label="Show" />
        <Button size="sm" onClick={ui.resetModelTypes}>
          Reset filters
        </Button>
        {graphTypeOptions.map((option) => (
          <ToggleRow
            key={option.type}
            label={humanize(option.type)}
            on={ui.modelTypes.has(option.type)}
            icon={<ElementIcon type={option.type} family={option.family} size="sm" />}
            meta={formatCompactCount(option.count)}
            onToggle={() => ui.toggleModelType(option.type)}
          />
        ))}
      </section>
    );
  }

  if (activeView === "traces") return null;

  if (activeView === "coverage") {
    const coverageItems = buildCoveragePaneItems(store);
    return (
      <section className={cx("ex-pane-controls", paneControlsClass)} aria-label="Coverage explorer">
        <PaneSectionLabel label="Coverage" />
        <div className={cx("ex-pane-nav-list", paneNavListClass)}>
          {coverageItems.map((item) => (
            <button
              key={item.id}
              type="button"
              className={cx("ex-pane-nav-row", paneNavRowClass)}
              onClick={() => navigateCoverageSection(item.id)}
            >
              <span className={cx("ex-pane-nav-row__icon", paneNavRowIconClass)} aria-hidden="true">
                <Icon name={item.icon} />
              </span>
              <span className={cx("ex-pane-nav-row__label", paneNavRowLabelClass)}>{item.label}</span>
              <Badge className={cx("ex-pane-nav-row__count", paneNavRowCountClass)}>{formatCompactCount(item.count)}</Badge>
            </button>
          ))}
        </div>
      </section>
    );
  }

  if (activeView === "search") {
    return (
      <section className={cx("ex-pane-controls", paneControlsClass)} aria-label="Search controls">
        <h2 className={cx("ex-pane-controls-title", paneControlsTitleClass)}>Filter by</h2>
        <Button size="sm" onClick={ui.resetSearchKinds}>
          Reset filters
        </Button>
        <PaneSectionLabel label="Result types" />
        {SEARCH_KINDS.map((kind) => (
          <ToggleRow
            key={kind}
            label={searchKindLabel(kind)}
            on={ui.searchKinds.has(kind)}
            colorToken={searchKindColorToken(kind)}
            meta={formatCompactCount(searchKindCounts[kind] ?? 0)}
            onToggle={() => ui.toggleSearchKind(kind)}
          />
        ))}
        {searchElementTypeOptions.length > 0 ? (
          <>
            <PaneSectionLabel label="Element types" />
            {searchElementTypeOptions.map((option) => (
              <ToggleRow
                key={option.type}
                label={humanize(option.type)}
                on={ui.searchElementTypes.has(option.type)}
                icon={<ElementIcon type={option.type} family={option.family} size="sm" />}
                meta={formatCompactCount(option.count)}
                onToggle={() => ui.toggleSearchElementType(option.type)}
              />
            ))}
          </>
        ) : null}
      </section>
    );
  }

  if (activeView === "ontologies") {
    const summary = store.ontology.summary ?? {};
    return (
      <section className={cx("ex-pane-controls", paneControlsClass)} aria-label="Ontology controls">
        <SidebarSection title="Summary" className={cx("ex-pane-summary", paneSummaryClass)} aria-label="Summary">
          <StatRow className={cx("ex-summary", summaryClass)}>
            <Stat label="Ontologies" value={formatSummaryValue(summary.ontology_blocks ?? 0)} />
            <Stat label="Shapes" value={formatSummaryValue(summary.shape_blocks ?? 0)} />
            <Stat
              label="Quads"
              value={formatSummaryValue(summary.total_quads ?? 0)}
              title="RDF statements (subject-predicate-object, with graph context)"
            />
            <Stat
              label="Blocks"
              value={formatSummaryValue(summary.total_blocks ?? 0)}
              title="Ontology and shape source blocks discovered in the model"
            />
          </StatRow>
        </SidebarSection>
        <OntologySelectedNodeLink
          selectedNodeId={ui.ontologySelectionId}
          nodes={store.ontology.graph_data?.nodes ?? []}
          onOpenOntologyNode={onOpenOntologyNode}
          onClear={() => {
            ui.setOntologySelectionId(null);
            window.clearOntologySelection?.();
          }}
        />
        <PaneSectionLabel label="Graph" />
        <div className={cx("ex-pane-action-row", paneActionRowClass)}>
          {store.ontology.ttl_href && (
            <a
              href={store.ontology.ttl_href}
              className={cx("ex-pane-ghost-link", paneGhostLinkClass)}
              title="Download the exported ontology as Turtle (ontologies.ttl)"
            >
              <Icon name="download" />
              Download .ttl
            </a>
          )}
          <Button
            tone="ghost"
            size="sm"
            iconLeft={<Icon name="rotate-ccw" />}
            onClick={() =>
              (window as typeof window & { resetOntologyGraphLayout?: () => void })
                .resetOntologyGraphLayout?.()
            }
          >
            Reset layout
          </Button>
        </div>
        <PaneSectionLabel label="Types" />
        <PaneVisualLegend
          rows={[
            ["class", "Class"],
            ["named-individual", "Individual"],
            ["datatype", "Datatype"],
            ["class-expression", "Class expr."],
            ["node-shape", "Node shape"],
            ["property-shape", "Property shape"],
            ["resource", "Resource"],
          ]}
        />
        <div className={cx("ex-pane-legend", paneLegendClass)}>
          <ToggleRow
            label="Relation"
            colorToken="--text-muted"
            line
            static
          />
        </div>
        <PaneSectionLabel label="Notation" />
        <PaneNotationLegend
          rows={[
            ["D/R", "Domain/range"],
            ["⊆", "Subclass"],
            ["∈", "Membership"],
            ["⟂", "Disjoint"],
            ["⇔", "Equivalence"],
            ["⟲", "Inverse"],
            ["∘", "Property chain"],
            ["∩", "Class expr."],
            ["SH", "SHACL overlay"],
          ]}
        />
      </section>
    );
  }

  return null;
}

function KnowledgeGraphSelectedElementLink({
  selectedNodeId,
  nodes,
  elementById,
  onOpenElement,
}: {
  selectedNodeId: string | null;
  nodes: KnowledgeGraphNode[];
  elementById: (id: string) => ProjectStoreElement | undefined;
  onOpenElement: (id: string) => void;
}) {
  if (!selectedNodeId) return null;
  const node = nodes.find((candidate) => candidate.id === selectedNodeId);
  if (!node?.identifier) return null;
  const element = elementById(node.identifier);
  if (!element) return null;

  return (
    <section className={cx("ex-pane-selected-element", paneSelectedElementClass)} aria-label="Selected graph element">
      <PaneSectionLabel label="Element" />
      <button
        type="button"
        className={cx("ex-pane-selected-element-link", paneSelectedElementLinkClass)}
        onClick={() => onOpenElement(element.id)}
      >
        <ElementTypeGlyph element={element} />
        <span>{element.name}</span>
      </button>
    </section>
  );
}

function OntologySelectedNodeLink({
  selectedNodeId,
  nodes,
  onOpenOntologyNode,
  onClear,
}: {
  selectedNodeId: string | null;
  nodes: OntologyGraphNode[];
  onOpenOntologyNode: (id: string) => void;
  onClear: () => void;
}) {
  const node = selectedNodeId
    ? nodes.find((candidate) => candidate.id === selectedNodeId)
    : undefined;
  const kind = node ? node.semantic_type || node.node_type || node.type || "resource" : "";
  const swatchColor = `var(${ontologyColorToken(kind)})`;

  return (
    <section className={cx("ex-pane-selected-element", paneSelectedElementClass)} aria-label="Selected ontology node">
      <PaneSectionLabel label="Selection" />
      {!node ? (
        <p className={cx(emptyClass, "ex-pane-selection-hint", paneSelectionHintClass)}>
          Select a graph node to inspect its details.
        </p>
      ) : (
        <div className={cx("ex-pane-selection-row", paneSelectionRowClass)}>
          <button
            type="button"
            className={cx("ex-pane-selected-element-link", paneSelectedElementLinkClass)}
            onClick={() => onOpenOntologyNode(node.id)}
            title="Open node details"
          >
            <span
              className={cx("ex-graph-control-swatch", graphControlSwatchClass)}
              style={{ backgroundColor: swatchColor, borderColor: swatchColor }}
            />
            <span className={cx("ex-pane-selection-name", paneSelectionNameClass)}>{node.label || node.id}</span>
            <span className={cx("ex-pane-selection-kind", paneSelectionKindClass)}>{kind}</span>
            <Icon name="arrow-up-right" size={13} className={cx("ex-pane-selection-open", paneSelectionOpenClass)} />
          </button>
          <IconButton size="sm" tone="ghost" aria-label="Clear selection" title="Clear selection" onClick={onClear}>
            <Icon name="x" />
          </IconButton>
        </div>
      )}
    </section>
  );
}

function TreeFolderNode({
  folder,
  activeView,
  elementById,
  onNavigate,
  onOpenElement,
  depth,
}: {
  folder: TreeFolder;
  activeView: ViewId;
  elementById: (id: string) => ProjectStoreElement | undefined;
  onNavigate: (view: ViewId) => void;
  onOpenElement: (id: string) => void;
  depth: number;
}) {
  const [open, setOpen] = useState(depth < 2);
  const ui = useExplorerUiState();
  const selectionId = folder.path === ROOT_PATH ? "__root__" : `folder:${folder.path}`;

  function selectFolder() {
    ui.setModelSelectionId(selectionId);
    if (activeView === "files") onNavigate("model");
  }

  return (
    <div className={cx(treeNodeClass)}>
      <TreeItem
        kind="folder"
        label={folder.name}
        icon={open ? <Icon name="folder-open" className="file-kind-folder" /> : <Icon name="folder" className="file-kind-folder" />}
        count={folder.files.length + folder.folders.length}
        depth={depth}
        open={open}
        expandable={folder.files.length + folder.folders.length > 0}
        selected={ui.modelSelectionId === selectionId}
        onToggle={() => setOpen((value) => !value)}
        onSelect={selectFolder}
      />
      {open && (
        <>
          {folder.folders.map((child) => (
            <TreeFolderNode
              key={child.path}
              folder={child}
              activeView={activeView}
              elementById={elementById}
              onNavigate={onNavigate}
              onOpenElement={onOpenElement}
              depth={depth + 1}
            />
          ))}
          {folder.files.map((file) => (
            <TreeFileNode
              key={file.path}
              file={file}
              activeView={activeView}
              elementById={elementById}
              onNavigate={onNavigate}
              onOpenElement={onOpenElement}
              depth={depth + 1}
            />
          ))}
        </>
      )}
    </div>
  );
}

function TraceTreeFolderNode({
  folder,
  depth,
}: {
  folder: TracePaneFolder;
  depth: number;
}) {
  const ui = useExplorerUiState();
  const selectedPath = ui.traceFilePath;
  const hasSelectedDescendant = selectedPath
    ? traceFolderContainsPath(folder, selectedPath)
    : false;
  const [open, setOpen] = useState(depth < 2 || hasSelectedDescendant);

  useEffect(() => {
    if (hasSelectedDescendant) setOpen(true);
  }, [hasSelectedDescendant]);

  return (
    <div className={cx(treeNodeClass)}>
      <TreeItem
        kind="folder"
        label={folder.name}
        icon={open ? <Icon name="folder-open" className="file-kind-folder" /> : <Icon name="folder" className="file-kind-folder" />}
        count={traceFolderVerificationCount(folder)}
        depth={depth}
        open={open}
        expandable={folder.files.length + folder.folders.length > 0}
        selected={folder.path === ROOT_PATH && !selectedPath}
        onToggle={() => setOpen((value) => !value)}
        onSelect={() => {
          ui.setTraceFilePath(null);
          ui.setTraceSelectionId(null);
        }}
      />
      {open && (
        <>
          {folder.folders.map((child) => (
            <TraceTreeFolderNode key={child.path} folder={child} depth={depth + 1} />
          ))}
          {folder.files.map((file) => (
            <TraceTreeFileNode key={file.path} file={file} depth={depth + 1} />
          ))}
        </>
      )}
    </div>
  );
}

function TraceTreeFileNode({
  file,
  depth,
}: {
  file: TracePaneFile;
  depth: number;
}) {
  const ui = useExplorerUiState();
  const selectedFile = ui.traceFilePath === file.path;
  const selectedVerification = selectedFile ? ui.traceSelectionId : null;
  const [open, setOpen] = useState(true);

  function selectFile() {
    ui.setTraceFilePath(file.path);
    ui.setTraceSelectionId(null);
  }

  function selectVerification(id: string) {
    ui.setTraceFilePath(file.path);
    ui.setTraceSelectionId(id);
  }

  return (
    <div className={cx(treeNodeClass)}>
      <TreeItem
        kind="file"
        label={file.name}
        icon={<Icon name="file" className="file-kind-file" />}
        count={file.verifications.length}
        depth={depth}
        open={open}
        expandable={file.verifications.length > 0}
        selected={selectedFile && !selectedVerification}
        onToggle={() => setOpen((value) => !value)}
        onSelect={selectFile}
      />
      {open && file.verifications.map((verification) => (
        <TreeItem
          key={verification.id}
          kind="element"
          label={verification.name}
          icon={<ElementIcon type={verification.type ?? "verification"} family="verification" size="sm" />}
          depth={depth + 1}
          selected={selectedVerification === verification.id}
          onSelect={() => selectVerification(verification.id)}
        />
      ))}
    </div>
  );
}

function TreeFileNode({
  file,
  activeView,
  elementById,
  onNavigate,
  onOpenElement,
  depth,
}: {
  file: ProjectStoreFile;
  activeView: ViewId;
  elementById: (id: string) => ProjectStoreElement | undefined;
  onNavigate: (view: ViewId) => void;
  onOpenElement: (id: string) => void;
  depth: number;
}) {
  const ui = useExplorerUiState();
  const elements = file.element_ids.map(elementById).filter(Boolean) as ProjectStoreElement[];
  const showElementChildren = elements.length > 0;
  const [open, setOpen] = useState(showElementChildren);
  const selectionId = `file:${file.path}`;

  function selectFile() {
    ui.setModelSelectionId(selectionId);
    if (activeView === "files") onNavigate("model");
  }

  function selectElement(elementId: string) {
    ui.setModelSelectionId(elementId);
    if (activeView === "files") onNavigate("model");
    onOpenElement(elementId);
  }

  return (
    <div className={cx(treeNodeClass)}>
      <TreeItem
        kind="file"
        label={displayName(file.display_path || file.path)}
        icon={<Icon name="file" className="file-kind-file" />}
        count={elements.length > 0 ? elements.length : undefined}
        depth={depth}
        open={open}
        expandable={showElementChildren}
        selected={ui.modelSelectionId === selectionId}
        onToggle={() => setOpen((value) => !value)}
        onSelect={selectFile}
      />
      {open && showElementChildren && elements.map((element) => (
        <TreeItem
          key={element.id}
          kind="element"
          label={element.name}
          icon={<ElementTypeGlyph element={element} />}
          depth={depth + 1}
          selected={ui.modelSelectionId === element.id}
          onSelect={() => selectElement(element.id)}
        />
      ))}
    </div>
  );
}

export function ElementTypeGlyph({ element }: { element: ProjectStoreElement }) {
  return (
    <ElementIcon
      type={element.element_type}
      family={element.type_family}
      title={element.element_type}
      size="sm"
    />
  );
}

function PaneSectionLabel({ label }: { label: string }) {
  return (
    <span className={cx("ex-pane-section-label", paneSectionLabelClass)}>
      {label}
    </span>
  );
}

function PaneVisualLegend({ rows }: { rows: [string, string][] }) {
  return (
    <div className={cx("ex-pane-legend", paneLegendClass)}>
      {rows.map(([kind, label]) => (
        <ToggleRow
          key={kind}
          label={label}
          colorToken={ontologyColorToken(kind)}
          static
        />
      ))}
    </div>
  );
}

function PaneNotationLegend({ rows }: { rows: [string, string][] }) {
  return (
    <div className={cx("ex-pane-legend", paneLegendClass)}>
      {rows.map(([symbol, label]) => (
        <div key={symbol} className={cx("ex-pane-legend-row", paneLegendRowClass)}>
          <span className={cx("ex-pane-symbol", paneSymbolClass)}>{symbol}</span>
          <span className={cx("ex-pane-legend-text", paneLegendTextClass)}>{label}</span>
        </div>
      ))}
    </div>
  );
}

function formatSummaryValue(value: string | number) {
  return typeof value === "number" ? value.toLocaleString() : value;
}

function buildFileTree(files: ProjectStoreFile[]): TreeFolder {
  const root: TreeFolder = { path: ROOT_PATH, name: "Project", folders: [], files: [] };
  const byPath = new Map<string, TreeFolder>([[ROOT_PATH, root]]);

  for (const file of files) {
    const folderPath = file.parent_folder || ROOT_PATH;
    ensureFolder(folderPath, byPath, root);
    byPath.get(folderPath)?.files.push(file);
  }

  for (const folder of byPath.values()) {
    folder.folders.sort((a, b) => a.name.localeCompare(b.name));
    folder.files.sort((a, b) =>
      displayName(a.display_path || a.path).localeCompare(displayName(b.display_path || b.path)),
    );
  }

  return root;
}

function buildTraceFileTree(files: TraceFileNode[]): TracePaneFolder {
  const root: TracePaneFolder = { path: ROOT_PATH, name: "Project", folders: [], files: [] };
  const byPath = new Map<string, TracePaneFolder>([[ROOT_PATH, root]]);

  for (const entry of files) {
    const path = entry.file;
    const folderPath = dirname(path) || ROOT_PATH;
    const folder = ensureTraceFolder(folderPath, byPath, root);
    folder.files.push({
      path,
      name: displayName(path),
      verifications: (entry.verifications ?? [])
        .map((verification) => ({
          id: verification.id,
          name: verification.name,
          type: verification.verificationType,
        }))
        .sort((a, b) => a.name.localeCompare(b.name)),
    });
  }

  for (const folder of byPath.values()) {
    folder.folders.sort((a, b) => a.name.localeCompare(b.name));
    folder.files.sort((a, b) => a.name.localeCompare(b.name));
  }

  return root;
}

function ensureTraceFolder(
  path: string,
  byPath: Map<string, TracePaneFolder>,
  root: TracePaneFolder,
): TracePaneFolder {
  if (path === ROOT_PATH || path === "") return root;
  const existing = byPath.get(path);
  if (existing) return existing;
  const parentPath = dirname(path) || ROOT_PATH;
  const parent = ensureTraceFolder(parentPath, byPath, root);
  const folder: TracePaneFolder = {
    path,
    name: displayName(path),
    folders: [],
    files: [],
  };
  byPath.set(path, folder);
  parent.folders.push(folder);
  return folder;
}

function traceFolderVerificationCount(folder: TracePaneFolder): number {
  return folder.files.reduce((sum, file) => sum + file.verifications.length, 0) +
    folder.folders.reduce((sum, child) => sum + traceFolderVerificationCount(child), 0);
}

function traceFolderContainsPath(folder: TracePaneFolder, path: string): boolean {
  return folder.files.some((file) => file.path === path) ||
    folder.folders.some((child) => traceFolderContainsPath(child, path));
}

function ensureFolder(path: string, byPath: Map<string, TreeFolder>, root: TreeFolder): TreeFolder {
  if (path === ROOT_PATH || path === "") return root;
  const existing = byPath.get(path);
  if (existing) return existing;
  const parentPath = dirname(path) || ROOT_PATH;
  const parent = ensureFolder(parentPath, byPath, root);
  const folder: TreeFolder = {
    path,
    name: displayName(path),
    folders: [],
    files: [],
  };
  byPath.set(path, folder);
  parent.folders.push(folder);
  return folder;
}

function displayName(path: string) {
  const normalized = path.replace(/\\/g, "/").replace(/\/$/, "");
  return normalized.split("/").pop() || normalized || "Project";
}

function dirname(path: string) {
  const normalized = path.replace(/\\/g, "/").replace(/\/$/, "");
  const index = normalized.lastIndexOf("/");
  return index > 0 ? normalized.slice(0, index) : "";
}

function humanize(value: string) {
  return value.replace(/-/g, " ").replace(/\b\w/g, (match) => match.toUpperCase());
}

const ELEMENT_TYPE_ORDER = [
  "capability",
  "requirement",
  "refinement",
  "verification",
  "test-verification",
  "analysis-verification",
  "inspection-verification",
  "demonstration-verification",
  "specification",
  "semantic-contract",
  "semantic-query-contract",
  "ontology",
  "resource",
  "other",
];

function buildKnowledgeGraphTypeOptions(
  nodes: readonly KnowledgeGraphNode[],
  elementById: (id: string) => ProjectStoreElement | undefined,
) {
  const byType = new Map<string, { type: string; family: string; count: number }>();
  for (const node of nodes) {
    const type = knowledgeGraphNodeKind(node);
    const element = node.identifier ? elementById(node.identifier) : undefined;
    const family = element?.type_family || node.node_type || node.type || type;
    const existing = byType.get(type);
    if (existing) {
      existing.count += 1;
    } else {
      byType.set(type, { type, family, count: 1 });
    }
  }

  return Array.from(byType.values()).sort((left, right) => {
    const leftRank = elementTypeRank(left.type, left.family);
    const rightRank = elementTypeRank(right.type, right.family);
    return leftRank - rightRank || left.type.localeCompare(right.type);
  });
}

function knowledgeGraphNodeKind(node: KnowledgeGraphNode) {
  return node.element_type || node.node_type || node.type || "other";
}

function buildSearchKindCounts(store: ExplorerProjectStore): Record<SearchKind, number> {
  return {
    file: store.files.length,
    element: store.elements.length,
    resource: store.resources.length,
    ontology: store.ontology.graph_data?.nodes?.length ?? 0,
  };
}

function buildCoveragePaneItems(store: ExplorerProjectStore): Array<{
  id: CoverageSectionId;
  label: string;
  count: number;
  icon: "pie-chart" | "box" | "file" | "activity" | "x" | "help-circle";
}> {
  const coverage = isPlainRecord(store.coverage) ? store.coverage : {};
  const summary = isPlainRecord(coverage.summary) ? coverage.summary : {};
  return [
    {
      id: "overview",
      label: "Overview",
      count: readNumber(summary.total_requirements_in_scope, store.elements.length),
      icon: "pie-chart",
    },
    {
      id: "capability-coverage",
      label: "Capability coverage",
      count: coverageCapabilityCount(coverage.capability_coverage),
      icon: "box",
    },
    {
      id: "unverified-requirements",
      label: "Unverified requirements",
      count: coverageSectionCount(coverage.unverified_leaf_requirements),
      icon: "file",
    },
    {
      id: "unimplemented-requirements",
      label: "Unimplemented requirements",
      count: coverageSectionCount(coverage.uncovered_requirements),
      icon: "activity",
    },
    {
      id: "unsatisfied-verifications",
      label: "Unsatisfied verifications",
      count: coverageSectionCount(coverage.unsatisfied_test_verifications),
      icon: "x",
    },
    {
      id: "orphaned-verifications",
      label: "Orphaned verifications",
      count: coverageSectionCount(coverage.orphaned_verifications),
      icon: "help-circle",
    },
  ];
}

function navigateCoverageSection(section: CoverageSectionId) {
  window.dispatchEvent(new CustomEvent("reqvire:coverage-navigate", { detail: { section } }));
}

function coverageSectionCount(section: unknown): number {
  if (!isPlainRecord(section) || !isPlainRecord(section.files)) return 0;
  return Object.values(section.files).reduce<number>((count, value) => {
    return count + (Array.isArray(value) ? value.length : 0);
  }, 0);
}

function coverageCapabilityCount(section: unknown): number {
  if (!isPlainRecord(section) || !Array.isArray(section.capabilities)) return 0;
  return section.capabilities.length;
}

function readNumber(value: unknown, fallback = 0): number {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

function isPlainRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function buildSearchElementTypeOptions(elements: readonly ProjectStoreElement[]) {
  const byType = new Map<string, { type: string; family: string; count: number }>();
  for (const element of elements) {
    if (!element.element_type) continue;
    const existing = byType.get(element.element_type);
    if (existing) {
      existing.count += 1;
    } else {
      byType.set(element.element_type, {
        type: element.element_type,
        family: element.type_family || element.element_type,
        count: 1,
      });
    }
  }

  return Array.from(byType.values()).sort((left, right) => {
    const leftRank = elementTypeRank(left.type, left.family);
    const rightRank = elementTypeRank(right.type, right.family);
    return leftRank - rightRank || left.type.localeCompare(right.type);
  });
}

function elementTypeRank(type: string, family: string) {
  const direct = ELEMENT_TYPE_ORDER.indexOf(type);
  if (direct >= 0) return direct;
  const familyRank = ELEMENT_TYPE_ORDER.indexOf(family);
  return familyRank >= 0 ? familyRank + 0.5 : ELEMENT_TYPE_ORDER.length;
}

function formatCompactCount(value: number): string {
  if (value >= 1_000_000) return `${trimCompactNumber(value / 1_000_000)}M`;
  if (value >= 1_000) return `${trimCompactNumber(value / 1_000)}K`;
  return value.toLocaleString();
}

function trimCompactNumber(value: number): string {
  return value >= 10 ? Math.round(value).toString() : value.toFixed(1).replace(/\.0$/, "");
}

function searchKindLabel(kind: SearchKind) {
  const labels: Record<SearchKind, string> = {
    file: "Files",
    element: "Elements",
    resource: "Resources",
    ontology: "Ontology terms",
  };
  return labels[kind];
}

function searchKindColorToken(kind: SearchKind): DesignSystemColorToken {
  const colors: Record<SearchKind, DesignSystemColorToken> = {
    file: "--resource",
    element: "--requirement",
    resource: "--ontology",
    ontology: "--rdf-resource",
  };
  return colors[kind];
}

function ontologyColorToken(value: string): DesignSystemColorToken {
  const colors: Record<string, DesignSystemColorToken> = {
    class: "--rdf-class",
    "object-property": "--rdf-objprop",
    "datatype-property": "--rdf-dtprop",
    "rdf-property": "--rdf-rdfprop",
    property: "--rdf-objprop",
    "named-individual": "--rdf-individual",
    datatype: "--rdf-datatype",
    restriction: "--rdf-restriction",
    "class-expression": "--rdf-classexpr",
    "node-shape": "--rdf-nodeshape",
    "property-shape": "--rdf-propshape",
    resource: "--rdf-resource",
    relation: "--edge-default",
  };
  return colors[value] ?? colors.resource;
}
