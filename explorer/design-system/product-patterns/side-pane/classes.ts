import { css } from "@linaria/atomic";

export const sideContentClass = css`
  box-sizing: border-box;
  display: flex;
  min-width: 0;
  min-height: 0;
  flex: 1 1 auto;
  flex-direction: column;
  width: calc(var(--ux-left-pane-width) - var(--ux-left-pane-collapsed-width));
`;

export const sideContentAppClass = css`
  display: flex;
  flex: 1 1 auto;
  flex-direction: column;
  width: 100%;
  height: 100%;
  min-height: 0;
  overflow: hidden;
`;

export const treeClass = css`
  box-sizing: border-box;
`;

export const treeTabClass = css`
  box-sizing: border-box;
`;

export const treeTabLabelClass = css`
  box-sizing: border-box;
`;

export const treeTabToggleClass = css`
  box-sizing: border-box;
`;

export const treeTabMarkClass = css`
  box-sizing: border-box;
`;

export const globalSearchClass = css`
  box-sizing: border-box;
`;

export const globalSearchControlClass = css`
  box-sizing: border-box;
`;

export const globalSearchResultsClass = css`
  box-sizing: border-box;
`;

export const paneControlsClass = css`
  box-sizing: border-box;
`;

export const paneControlsTitleClass = css`
  box-sizing: border-box;
`;

export const paneSummaryClass = css`
  box-sizing: border-box;
`;

export const summaryClass = css`
  box-sizing: border-box;
`;

export const paneNavListClass = css`
  box-sizing: border-box;
`;

export const paneNavRowClass = css`
  box-sizing: border-box;
`;

export const paneNavRowIconClass = css`
  box-sizing: border-box;
`;

export const paneNavRowLabelClass = css`
  box-sizing: border-box;
`;

export const paneNavRowCountClass = css`
  box-sizing: border-box;
`;

export const paneActionRowClass = css`
  box-sizing: border-box;
`;

export const paneGhostLinkClass = css`
  box-sizing: border-box;
`;

export const paneLegendClass = css`
  box-sizing: border-box;
`;

export const paneLegendRowClass = css`
  box-sizing: border-box;
`;

export const paneLegendTextClass = css`
  box-sizing: border-box;
`;

export const paneSymbolClass = css`
  box-sizing: border-box;
`;

export const graphControlSwatchClass = css`
  display: inline-flex;
  flex: 0 0 auto;
  width: var(--icon-xs);
  height: var(--icon-xs);
  box-sizing: border-box;
  border: var(--border-w) solid currentColor;
  border-radius: var(--radius-xs);
`;

export const paneSelectedElementClass = css`
  box-sizing: border-box;
`;

export const paneSelectedElementLinkClass = css`
  box-sizing: border-box;
`;

export const paneSelectionRowClass = css`
  box-sizing: border-box;
`;

export const paneSelectionNameClass = css`
  box-sizing: border-box;
`;

export const paneSelectionKindClass = css`
  box-sizing: border-box;
`;

export const paneSelectionOpenClass = css`
  box-sizing: border-box;
`;

export const paneSelectionHintClass = css`
  box-sizing: border-box;
`;

export const paneSectionLabelClass = css`
  box-sizing: border-box;
`;

export const treeNodeClass = css`
  box-sizing: border-box;
`;

export const emptyClass = css`
  box-sizing: border-box;
`;

export const baseUX = css`
  position: fixed;
  inset: 0 auto 0 0;
  z-index: var(--z-pane-resizer);
  display: flex;
  width: var(--ux-current-left-width);
  box-sizing: border-box;
  flex-direction: row;
  overflow: hidden;

  &.is-standalone {
    --ux-left-pane-width: 380px;
    --ux-left-pane-collapsed-width: 30px;
    --ux-current-left-width: var(--ux-left-pane-width);
  }

  &.is-collapsed .${sideContentClass} {
    display: none;
  }

  .ux-app & {
    position: relative;
    inset: auto;
    z-index: auto;
    align-self: stretch;
    flex: 0 0 var(--ux-current-left-width);
    width: var(--ux-current-left-width);
    min-width: 0;
    min-height: 0;
    height: 100%;
  }

  .ux-app &.is-collapsed {
    display: none;
  }

  .${sideContentClass} {
    display: flex;
    min-width: 0;
    min-height: 0;
    flex: 1 1 auto;
    flex-direction: column;
    width: calc(var(--ux-left-pane-width) - var(--ux-left-pane-collapsed-width));
  }

  .ux-app & .${sideContentClass} {
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
    flex: 0 0 var(--ux-left-pane-collapsed-width);
    width: var(--ux-left-pane-collapsed-width);
    align-items: flex-start;
    justify-content: center;
    padding-top: var(--space-5);
    border: 0;
    border-left: 0;
    cursor: pointer;
  }

  .ux-app & .${treeTabClass} {
    display: none;
  }

  &:not(.is-collapsed) .${treeTabLabelClass},
  &:not(.is-collapsed) .${treeTabMarkClass} {
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

  .ux-app & .${globalSearchClass} {
    margin: var(--space-12) var(--space-10) 0;
  }

  .${globalSearchControlClass} {
    position: relative;
    --ds-search-input-h: var(--control-lg);
    --ds-search-input-p: 0 var(--space-8) 0 calc(var(--space-16) + var(--space-3));
    --ds-search-input-fs: var(--text-base);
    --ds-search-icon-left: var(--space-8);
    --ds-search-icon-sz: var(--icon-md);
  }

  .${globalSearchResultsClass} {
    --ux-pane-search-results-max-h: 260px;
    max-height: var(--ux-pane-search-results-max-h);
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
    --ds-togglerow-jc: flex-start;
    --ds-togglerow-min-h: var(--control-md);
    --ds-togglerow-border: 0;
    --ds-togglerow-radius: 0;
    --ds-togglerow-shadow: none;
    --ds-togglerow-label-min-w: 0;
    --ds-togglerow-label-of: hidden;
    --ds-togglerow-label-toe: ellipsis;
    --ds-togglerow-label-ws: nowrap;
    --ds-togglerow-meta-display: inline-flex;
    --ds-togglerow-meta-min-w: var(--control-xs);
    --ds-togglerow-meta-h: var(--control-xs);
    --ds-togglerow-meta-ai: center;
    --ds-togglerow-meta-jc: center;
    --ds-togglerow-meta-p: 0 var(--space-3);
    --ds-togglerow-meta-radius: var(--radius-pill);
    --ds-togglerow-meta-fw: var(--weight-semibold);
    --ds-togglerow-meta-lh: 1;
    --ds-togglerow-line-min-h: var(--control-sm);
    --ds-togglerow-static-cursor: default;
  }

  .ux-app & .${paneControlsClass} {
    display: flex;
    flex: 1 1 auto;
    min-height: 0;
    flex-direction: column;
    gap: var(--gap-list-stack);
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

  .ux-app & .${paneSectionLabelClass} {
    margin: var(--space-12) 0 var(--space-5);
    padding: 0 var(--space-2);
    letter-spacing: var(--tracking-label);
    line-height: 1;
  }

  .${paneLegendClass} {
    display: grid;
    gap: var(--gap-list-stack);
  }

  .ux-app & .${paneLegendClass} {
    --ds-togglerow-h: var(--control-sm);
    --ds-togglerow-min-h: var(--control-sm);
    --ds-togglerow-gap: var(--space-6);
    --ds-togglerow-p: 0 var(--space-7);
    --ds-togglerow-line-h: var(--control-sm);
    --ds-togglerow-line-min-h: var(--control-sm);
    --ds-togglerow-line-p: 0 var(--space-7);
    --ds-togglerow-line-swatch-w: var(--icon-xs);
    --ds-togglerow-line-color: var(--text-muted);
    display: flex;
    flex-direction: column;
    gap: var(--gap-list-stack);
  }

  .${paneSummaryClass} {
    --ds-section-head-p: 0;
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    gap: var(--space-4);
  }

  .ux-app & .${paneSummaryClass} {
    flex-direction: column;
    flex-wrap: nowrap;
    align-items: stretch;
    gap: var(--space-5);
    margin: 0 0 var(--space-10);
  }

  .${paneSummaryClass} .${paneSectionLabelClass} {
    margin: 0 var(--space-1) 0 0;
  }

  .ux-app & .${paneSummaryClass} .${paneSectionLabelClass} {
    margin: 0;
    padding: 0;
  }

  .${paneSummaryClass} .${summaryClass} {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-3) var(--space-8);
    --ds-stat-display: flex;
    --ds-stat-min-w: 0;
    --ds-stat-jc: space-between;
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
    --ds-treeitem-count-ml: var(--space-1);
    --ds-treeitem-h: var(--space-16);
    --ds-treeitem-label-flex: 0 1 auto;
    --ds-treeitem-lh: 1.2;
    --ds-treeitem-pr: var(--space-6);
    --ds-treeitem-twist-w: var(--space-7);
    --ds-treeitem-icon-color: var(--text-secondary);
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
    gap: var(--gap-list-stack);
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

export const appRootClass = css`
  position: relative;
  inset: auto;
  z-index: auto;
  align-self: stretch;
  flex: 0 0 var(--ux-current-left-width);
  width: var(--ux-current-left-width);
  min-width: 0;
  min-height: 0;
  height: 100%;
`;

export const skinX = css`
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
    --ds-search-input-border: var(--border-w) solid var(--border-subtle);
    --ds-search-input-bg: var(--bg-canvas);
    --ds-search-input-color: var(--text-body);
    --ds-search-input-placeholder-color: var(--text-muted);
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
    background: var(--bg-hover);
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
    --ds-togglerow-bg: transparent;
    --ds-togglerow-meta-bg: var(--bg-sunken);
    --ds-togglerow-meta-color: var(--text-secondary);
    --ds-togglerow-line-swatch-border: currentColor;
    --ds-togglerow-line-swatch-bg: transparent;
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
    --ds-treeitem-twist-color: var(--text-muted);
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
    background: var(--bg-hover);
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
