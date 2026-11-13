import { css, cx } from "@linaria/atomic";

// Stable pattern hooks for selectors that relate these elements; each exported
// class carries its hook, so the selectors match exactly where the class is applied.
const emptyHook = "ux-side-pane-empty";
const globalSearchHook = "ux-side-pane-global-search";
const globalSearchControlHook = "ux-side-pane-global-search-control";
const globalSearchResultsHook = "ux-side-pane-global-search-results";
const paneActionRowHook = "ux-side-pane-action-row";
const paneControlsHook = "ux-side-pane-controls";
const paneControlsTitleHook = "ux-side-pane-controls-title";
const paneFilterGridHook = "ux-side-pane-filter-grid";
const paneFilterGridTwoHook = "ux-side-pane-filter-grid-two";
const paneGhostLinkHook = "ux-side-pane-ghost-link";
const paneLegendHook = "ux-side-pane-legend";
const paneLegendRowHook = "ux-side-pane-legend-row";
const paneLegendTextHook = "ux-side-pane-legend-text";
const paneNavRowCountHook = "ux-side-pane-nav-row-count";
const paneNavRowIconHook = "ux-side-pane-nav-row-icon";
const paneNavRowLabelHook = "ux-side-pane-nav-row-label";
const paneSectionLabelHook = "ux-side-pane-section-label";
const paneSelectedElementHook = "ux-side-pane-selected-element";
const paneSelectedElementLinkHook = "ux-side-pane-selected-element-link";
const paneSelectionHintHook = "ux-side-pane-selection-hint";
const paneSelectionKindHook = "ux-side-pane-selection-kind";
const paneSelectionNameHook = "ux-side-pane-selection-name";
const paneSelectionOpenHook = "ux-side-pane-selection-open";
const paneSelectionRowHook = "ux-side-pane-selection-row";
const paneSummaryHook = "ux-side-pane-summary";
const paneSummaryFooterHook = "ux-side-pane-summary-footer";
const sideContentHook = "ux-side-pane-side-content";
const treeHook = "ux-side-pane-tree";
const treeNodeHook = "ux-side-pane-tree-node";
const treeTabHook = "ux-side-pane-tree-tab";
const treeTabLabelHook = "ux-side-pane-tree-tab-label";
const treeTabMarkHook = "ux-side-pane-tree-tab-mark";
const treeTabToggleHook = "ux-side-pane-tree-tab-toggle";

export const sideContentClass = cx(sideContentHook, css`
  box-sizing: border-box;
  display: flex;
  min-width: 0;
  min-height: 0;
  flex: 1 1 auto;
  flex-direction: column;
  width: calc(var(--ux-left-pane-width) - var(--ux-left-pane-collapsed-width));
`);

export const sideContentAppClass = css`
  display: flex;
  flex: 1 1 auto;
  flex-direction: column;
  width: 100%;
  height: 100%;
  min-height: 0;
  overflow-x: hidden;
  overflow-y: hidden;
`;

export const treeClass = cx(treeHook, css`
  box-sizing: border-box;
  --ds-treeitem-count-ml: var(--space-1);
  --ds-treeitem-h: var(--space-16);
  --ds-treeitem-label-flex: 0 1 auto;
  --ds-treeitem-lh: 1.2;
  --ds-treeitem-pl-base: var(--side-pane-content-inset-inline);
  --ds-treeitem-pr: var(--side-pane-content-inset-inline);
  --ds-treeitem-twist-w: var(--space-7);
  --ds-treeitem-icon-color: var(--text-secondary);
  min-height: 0;
  flex: 1 1 auto;
  overflow-x: hidden;
  overflow-y: auto;
  padding-top: 0;
  padding-right: 0;
  padding-bottom: var(--space-7);
  padding-left: 0;
  scrollbar-gutter: stable;
`);

export const treeTabClass = cx(treeTabHook, css`
  box-sizing: border-box;
`);

export const treeTabLabelClass = cx(treeTabLabelHook, css`
  box-sizing: border-box;
`);

export const treeTabToggleClass = cx(treeTabToggleHook, css`
  box-sizing: border-box;
`);

export const treeTabMarkClass = cx(treeTabMarkHook, css`
  box-sizing: border-box;
`);

export const globalSearchClass = cx(globalSearchHook, css`
  box-sizing: border-box;
  flex: 0 0 auto;
  margin-top: var(--space-12);
  margin-right: var(--side-pane-content-inset-inline);
  margin-bottom: var(--side-pane-search-content-gap);
  margin-left: var(--side-pane-content-inset-inline);
`);

export const globalSearchControlClass = cx(globalSearchControlHook, css`
  box-sizing: border-box;
`);

export const globalSearchResultsClass = cx(globalSearchResultsHook, css`
  box-sizing: border-box;
`);

export const paneControlsClass = cx(paneControlsHook, css`
  box-sizing: border-box;
  display: flex;
  flex: 1 1 auto;
  min-height: 0;
  flex-direction: column;
  gap: var(--space-4);
  overflow-x: hidden;
  overflow-y: auto;
  padding-top: var(--space-6);
  padding-right: var(--side-pane-content-inset-inline);
  padding-bottom: var(--space-7);
  padding-left: var(--side-pane-content-inset-inline);
  scrollbar-gutter: stable;
  --ds-togglerow-jc: flex-start;
  --ds-togglerow-min-h: var(--control-height-md);
  --ds-togglerow-border: 0;
  --ds-togglerow-radius: 0;
  --ds-togglerow-shadow: none;
  --ds-togglerow-label-min-w: 0;
  --ds-togglerow-label-of: hidden;
  --ds-togglerow-label-toe: ellipsis;
  --ds-togglerow-label-ws: nowrap;
  --ds-togglerow-meta-display: inline-flex;
  --ds-togglerow-meta-min-w: var(--space-count-pill-height);
  --ds-togglerow-meta-h: var(--space-count-pill-height);
  --ds-togglerow-meta-ai: center;
  --ds-togglerow-meta-jc: center;
  --ds-togglerow-meta-p: 0 var(--space-3);
  --ds-togglerow-meta-radius: var(--radius-full);
  --ds-togglerow-meta-fw: var(--weight-semibold);
  --ds-togglerow-meta-lh: 1;
  --ds-togglerow-line-min-h: var(--control-height-sm);
  --ds-togglerow-static-cursor: default;
`);

export const paneControlsTitleClass = cx(paneControlsTitleHook, css`
  box-sizing: border-box;
  margin-top: 0;
  margin-right: 0;
  margin-bottom: var(--space-7);
  margin-left: 0;
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  letter-spacing: 0;
  line-height: var(--leading-tight);
`);

export const paneSummaryClass = cx(paneSummaryHook, css`
  box-sizing: border-box;
  --ds-section-head-p: 0;
  display: flex;
  flex-direction: column;
  flex-wrap: nowrap;
  align-items: stretch;
  gap: var(--space-5);
`);

export const paneSummaryFooterClass = cx(paneSummaryFooterHook, css`
  box-sizing: border-box;
  flex: 0 0 auto;
  margin-top: auto;
  margin-right: 0;
  margin-bottom: 0;
  margin-left: 0;
  padding-top: var(--space-12);
  padding-right: var(--side-pane-content-inset-inline);
  padding-bottom: var(--space-12);
  padding-left: var(--side-pane-content-inset-inline);
`);

export const summaryClass = css`
  box-sizing: border-box;
  display: grid;
  grid-template-columns: var(--side-pane-summary-columns);
  gap: var(--space-3) var(--space-8);
  width: 100%;

  .ux-summary__item {
    display: flex;
    min-width: 0;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--space-3);
  }

  .ux-summary__label {
    min-width: 0;
    overflow-x: hidden;
    overflow-y: hidden;
    color: var(--text-muted);
    font-size: var(--text-caption);
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ux-summary__value {
    color: var(--text-strong);
    font-size: var(--text-caption);
    font-variant-numeric: tabular-nums;
    font-weight: var(--weight-semibold);
  }
`;

export const paneNavListClass = css`
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: var(--stack-gap-compact);
`;

export const paneNavRowLabelClass = cx(paneNavRowLabelHook, css`
  box-sizing: border-box;
  overflow-x: hidden;
  overflow-y: hidden;
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  text-overflow: ellipsis;
  white-space: nowrap;
`);

export const paneNavRowClass = css`
  box-sizing: border-box;
  display: grid;
  grid-template-columns: var(--icon-md) minmax(0, 1fr) auto;
  align-items: center;
  gap: var(--space-5);
  width: 100%;
  min-height: var(--control-height-md);
  padding-top: 0;
  padding-right: var(--control-padding-md);
  padding-bottom: 0;
  padding-left: var(--control-padding-md);
  appearance: none;
  border: 0;
  border-radius: var(--radius-md);
  background: transparent;
  color: inherit;
  cursor: pointer;
  font: inherit;
  text-align: left;

  &:hover,
  &:focus-visible {
    background: var(--bg-hover);
  }

  &.is-selected {
    background: var(--bg-selected);
    color: var(--text-strong);
  }

  &.is-selected .${paneNavRowLabelHook} {
    color: var(--text-strong);
    font-weight: var(--weight-semibold);
  }

  &:focus-visible {
    outline: var(--focus-w) solid var(--focus-ring);
    outline-offset: var(--space-1);
  }
`;

export const paneNavRowIconClass = cx(paneNavRowIconHook, css`
  box-sizing: border-box;
  display: inline-flex;
  align-items: center;
  justify-content: center;
`);

export const paneNavRowCountClass = cx(paneNavRowCountHook, css`
  box-sizing: border-box;
  display: inline-flex;
  min-width: min(100%, var(--space-count-pill-height));
  height: var(--space-count-pill-height);
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-full);
  font-size: var(--text-micro);
  font-weight: var(--weight-semibold);
  line-height: 1;
`);

export const paneActionRowClass = cx(paneActionRowHook, css`
  box-sizing: border-box;
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-5);
  margin-top: 0;
  margin-right: 0;
  margin-bottom: 0;
  margin-left: 0;
`);

export const paneGhostLinkClass = cx(paneGhostLinkHook, css`
  box-sizing: border-box;
  display: inline-flex;
  height: var(--control-height-sm);
  align-items: center;
  gap: var(--space-4);
  padding-top: 0;
  padding-right: var(--control-padding-sm);
  padding-bottom: 0;
  padding-left: var(--control-padding-sm);
  border-radius: var(--radius-sm);
  font-size: var(--text-caption);
  font-weight: var(--weight-medium);
  text-decoration: none;
`);

export const paneLegendClass = cx(paneLegendHook, css`
  box-sizing: border-box;
  display: grid;
  gap: var(--stack-gap-compact);
`);

export const paneFilterGridClass = cx(paneFilterGridHook, css`
  box-sizing: border-box;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(var(--compact-column-min), 1fr));
  align-items: start;
  gap: var(--space-4) var(--space-8);
`);

export const paneFilterGridTwoClass = cx(paneFilterGridTwoHook, css`
  box-sizing: border-box;
  grid-template-columns: repeat(2, minmax(0, 1fr));
`);

export const paneLegendRowClass = cx(paneLegendRowHook, css`
  box-sizing: border-box;
  display: flex;
  align-items: center;
  gap: var(--space-6);
  min-height: var(--side-pane-legend-row-height);
  padding-top: 0;
  padding-right: var(--space-7);
  padding-bottom: 0;
  padding-left: var(--space-7);
`);

export const paneLegendTextClass = cx(paneLegendTextHook, css`
  box-sizing: border-box;
  font-size: var(--text-caption);
  line-height: 1.3;
`);

export const paneSymbolClass = css`
  box-sizing: border-box;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: var(--control-height-md);
  height: var(--icon-lg);
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--bg-sunken);
  color: var(--accent-text);
  font-size: var(--text-micro);
  font-weight: var(--weight-bold);
  line-height: 1;
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

export const paneSelectedElementClass = cx(paneSelectedElementHook, css`
  box-sizing: border-box;
`);

export const paneSelectedElementLinkClass = cx(paneSelectedElementLinkHook, css`
  box-sizing: border-box;
  appearance: none;
  display: inline-flex;
  width: 100%;
  min-width: 0;
  align-items: center;
  justify-content: flex-start;
  gap: var(--space-3);
  border: var(--border-w) solid var(--border-default);
  border-radius: var(--radius-sm);
  padding-top: var(--space-2);
  padding-right: var(--control-padding-sm);
  padding-bottom: var(--space-2);
  padding-left: var(--control-padding-sm);
  background: var(--bg-surface);
  color: var(--text-body);
  font-family: inherit;
  font-style: inherit;
  font-variant: inherit;
  font-stretch: inherit;
  line-height: inherit;
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  text-align: left;
  text-decoration: none;
  cursor: pointer;
  transition:
    background var(--dur-fast),
    border-color var(--dur-fast);

  &:hover {
    border-color: var(--border-strong);
    background: var(--bg-hover);
  }
`);

export const paneSelectionRowClass = cx(paneSelectionRowHook, css`
  box-sizing: border-box;
  display: flex;
  min-width: 0;
  align-items: center;
  gap: var(--space-2);
`);

export const paneSelectionNameClass = cx(paneSelectionNameHook, css`
  box-sizing: border-box;
  flex: 1 1 auto;
  min-width: 0;
  overflow-x: hidden;
  overflow-y: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
`);

export const paneSelectionKindClass = cx(paneSelectionKindHook, css`
  box-sizing: border-box;
  flex: 0 0 auto;
  overflow-x: visible;
  overflow-y: visible;
  border-radius: var(--radius-full);
  padding-top: var(--space-1);
  padding-right: var(--control-padding-sm);
  padding-bottom: var(--space-1);
  padding-left: var(--control-padding-sm);
  background: var(--bg-sunken);
  color: var(--text-muted);
  font-size: var(--text-micro);
  font-weight: var(--weight-semibold);
  line-height: 1.2;
`);

export const paneSelectionOpenClass = cx(paneSelectionOpenHook, css`
  box-sizing: border-box;
  flex: 0 0 auto;
  color: var(--text-muted);
`);

export const paneSelectionHintClass = cx(paneSelectionHintHook, css`
  box-sizing: border-box;
  margin-top: 0;
  margin-right: 0;
  margin-bottom: 0;
  margin-left: 0;
  color: var(--text-muted);
  font-size: var(--text-caption);
`);

export const paneSectionLabelClass = cx(paneSectionLabelHook, css`
  box-sizing: border-box;
  display: block;
  margin-top: var(--space-6);
  margin-right: 0;
  margin-bottom: var(--space-2);
  margin-left: 0;
  font-size: var(--text-micro);
  font-weight: var(--weight-semibold);
  letter-spacing: var(--tracking-pane-chrome-letter-spacing);
  line-height: 1;
  text-transform: uppercase;
`);

export const treeNodeClass = cx(treeNodeHook, css`
  box-sizing: border-box;
`);

export const emptyClass = cx(emptyHook, css`
  box-sizing: border-box;
  font-size: var(--text-sm);
  font-style: italic;
  line-height: 1.45;
`);

export const baseUX = css`
  position: fixed;
  inset: 0 auto 0 0;
  z-index: var(--z-pane-resizer);
  display: flex;
  width: var(--ux-current-left-width);
  box-sizing: border-box;
  flex-direction: row;
  overflow-x: hidden;
  overflow-y: hidden;

  &.is-standalone {
    --ux-left-pane-width: var(--side-pane-width);
    --ux-left-pane-collapsed-width: var(--side-pane-collapsed-width);
    --ux-current-left-width: var(--ux-left-pane-width);
  }

  &.is-collapsed .${sideContentHook} {
    display: none;
  }

  .${treeTabHook} {
    position: relative;
    display: flex;
    min-height: 0;
    flex: 0 0 var(--ux-left-pane-collapsed-width);
    width: var(--ux-left-pane-collapsed-width);
    align-items: flex-start;
    justify-content: center;
    padding-top: var(--space-5);
    border: 0;
    cursor: pointer;
  }

  &:not(.is-collapsed) .${treeTabLabelHook},
  &:not(.is-collapsed) .${treeTabMarkHook} {
    display: none;
  }

  .${treeTabLabelHook} {
    display: inline-block;
    margin-top: var(--space-14);
    writing-mode: vertical-rl;
    font-size: var(--text-micro);
    font-weight: var(--weight-bold);
    letter-spacing: var(--tracking-pane-tab);
    text-transform: uppercase;
  }

  .${treeTabToggleHook} {
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

  .${globalSearchControlHook} {
    position: relative;
    --ds-search-input-h: var(--control-height-lg);
    --ds-search-input-p: 0 var(--space-8) 0 calc(var(--space-16) + var(--space-3));
    --ds-search-input-fs: var(--text-base);
    --ds-search-icon-left: var(--space-8);
    --ds-search-icon-sz: var(--icon-md);
  }

  .${globalSearchResultsHook},
  .ontology-graph-results {
    --ux-pane-search-results-max-h: min(var(--space-pane-search-results-limit), calc(100vh - var(--space-32)));
    max-height: var(--ux-pane-search-results-max-h);
    overflow-x: hidden;
    overflow-y: auto;
    overscroll-behavior: contain;
    margin-top: var(--space-4);
    margin-right: 0;
    margin-bottom: 0;
    margin-left: 0;
    padding-top: var(--space-2);
    padding-right: var(--space-2);
    padding-bottom: var(--space-2);
    padding-left: var(--space-2);
    list-style: none;
    scrollbar-gutter: stable;
  }

  .${globalSearchResultsHook}:empty,
  .ontology-graph-results:empty {
    display: none;
  }

  .${globalSearchResultsHook} button,
  .${globalSearchResultsHook} a,
  .ontology-graph-results button,
  .ontology-graph-results a {
    appearance: none;
    display: grid;
    width: 100%;
    box-sizing: border-box;
    gap: var(--space-1);
    padding-top: var(--space-3);
    padding-right: var(--control-padding-sm);
    padding-bottom: var(--space-3);
    padding-left: var(--control-padding-sm);
    border: 0;
    border-radius: var(--radius-sm);
    text-align: left;
    text-decoration: none;
    cursor: pointer;
  }

  .${globalSearchResultsHook} small,
  .ontology-graph-results small {
    font-size: var(--text-micro);
  }

  .${globalSearchResultsHook} .ontology-graph-result,
  .ontology-graph-results .ontology-graph-result {
    display: grid;
    grid-template-columns: var(--icon-md) minmax(0, 1fr) var(--icon-lg);
    align-items: center;
    min-width: 0;
    gap: var(--space-3);
    border-radius: var(--radius-sm);
    padding-top: var(--space-3);
    padding-right: var(--control-padding-sm);
    padding-bottom: var(--space-3);
    padding-left: var(--control-padding-sm);
    cursor: pointer;
    font-size: var(--text-caption);
    list-style: none;
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph,
  .ontology-graph-results .ontology-graph-result-glyph {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--icon-sm);
    height: var(--icon-sm);
    border-radius: var(--radius-xs);
    border: var(--border-w) solid var(--border-subtle);
    background: var(--color-role-rdf-resource);
  }

  .${globalSearchResultsHook} .ontology-graph-result-label,
  .ontology-graph-results .ontology-graph-result-label {
    min-width: 0;
    overflow-x: hidden;
    overflow-y: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .${globalSearchResultsHook} .ontology-graph-badge,
  .ontology-graph-results .ontology-graph-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    min-width: var(--space-10);
    height: var(--space-count-pill-height);
    border-radius: var(--radius-full);
    border: var(--border-w) solid var(--border-subtle);
    padding-top: 0;
    padding-right: var(--control-padding-sm);
    padding-bottom: 0;
    padding-left: var(--control-padding-sm);
    font-size: var(--text-micro);
    font-weight: var(--weight-bold);
    line-height: 1;
  }

  .${paneControlsHook} {
    display: grid;
    flex: 0 0 auto;
    gap: var(--space-4);
    padding-top: var(--space-6);
    padding-right: var(--side-pane-content-inset-inline);
    padding-bottom: var(--space-7);
    padding-left: var(--side-pane-content-inset-inline);
    --ds-togglerow-jc: flex-start;
    --ds-togglerow-min-h: var(--control-height-md);
    --ds-togglerow-border: 0;
    --ds-togglerow-radius: 0;
    --ds-togglerow-shadow: none;
    --ds-togglerow-label-min-w: 0;
    --ds-togglerow-label-of: hidden;
    --ds-togglerow-label-toe: ellipsis;
    --ds-togglerow-label-ws: nowrap;
    --ds-togglerow-meta-display: inline-flex;
    --ds-togglerow-meta-min-w: var(--space-count-pill-height);
    --ds-togglerow-meta-h: var(--space-count-pill-height);
    --ds-togglerow-meta-ai: center;
    --ds-togglerow-meta-jc: center;
    --ds-togglerow-meta-p: 0 var(--space-3);
    --ds-togglerow-meta-radius: var(--radius-full);
    --ds-togglerow-meta-fw: var(--weight-semibold);
    --ds-togglerow-meta-lh: 1;
    --ds-togglerow-line-min-h: var(--control-height-sm);
    --ds-togglerow-static-cursor: default;
  }

  .ux-app & .${paneControlsHook} {
    gap: var(--stack-gap-compact);
    padding-top: var(--space-8);
    padding-right: var(--side-pane-content-inset-inline);
    padding-bottom: var(--space-16);
    padding-left: var(--side-pane-content-inset-inline);
  }

  .${paneControlsTitleHook} {
    margin-top: 0;
    margin-right: 0;
    margin-bottom: var(--space-7);
    margin-left: 0;
    font-size: var(--text-lg);
    font-weight: var(--weight-semibold);
    letter-spacing: 0;
    line-height: var(--leading-tight);
  }

  .${paneSectionLabelHook} {
    display: block;
    margin-top: var(--space-6);
    margin-right: 0;
    margin-bottom: var(--space-2);
    margin-left: 0;
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-pane-chrome-letter-spacing);
    text-transform: uppercase;
  }

  .ux-app & .${paneSectionLabelHook} {
    margin-top: var(--space-12);
    margin-right: 0;
    margin-bottom: var(--space-5);
    margin-left: 0;
    padding-top: 0;
    padding-right: var(--space-2);
    padding-bottom: 0;
    padding-left: var(--space-2);
    letter-spacing: var(--tracking-label);
    line-height: 1;
  }

  .${paneLegendHook} {
    display: grid;
    gap: var(--stack-gap-compact);
  }

  .${paneFilterGridHook} {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(var(--compact-column-min), 1fr));
    align-items: start;
    gap: var(--space-4) var(--space-8);
  }

  .${paneFilterGridHook}.${paneFilterGridTwoHook} {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .${paneFilterGridHook} .${paneSectionLabelHook}:first-child {
    margin-top: 0;
  }

  .ux-app & .${paneLegendHook} {
    --ds-togglerow-h: var(--control-height-sm);
    --ds-togglerow-min-h: var(--control-height-sm);
    --ds-togglerow-gap: var(--space-6);
    --ds-togglerow-p: 0 var(--side-pane-content-inset-inline);
    --ds-togglerow-line-h: var(--control-height-sm);
    --ds-togglerow-line-min-h: var(--control-height-sm);
    --ds-togglerow-line-p: 0 var(--side-pane-content-inset-inline);
    --ds-togglerow-line-swatch-w: var(--space-6);
    --ds-togglerow-line-color: var(--text-muted);
    display: flex;
    flex-direction: column;
    gap: var(--stack-gap-compact);
  }

  .${paneSummaryHook} {
    --ds-section-head-p: 0;
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    gap: var(--space-4);
  }

  .ux-app & .${paneSummaryHook} {
    flex-direction: column;
    flex-wrap: nowrap;
    align-items: stretch;
    gap: var(--space-5);
    margin-top: 0;
    margin-right: 0;
    margin-bottom: var(--space-10);
    margin-left: 0;
  }

  .ux-app & .${paneSummaryHook}.${paneSummaryFooterHook} {
    margin-top: auto;
    margin-right: 0;
    margin-bottom: 0;
    margin-left: 0;
    padding-top: var(--space-12);
    padding-right: var(--side-pane-content-inset-inline);
    padding-bottom: var(--space-12);
    padding-left: var(--side-pane-content-inset-inline);
  }

  .ux-app & .ux-pane-summary--footer {
    margin-top: auto;
    margin-right: 0;
    margin-bottom: 0;
    margin-left: 0;
    padding-top: var(--space-12);
    padding-right: var(--side-pane-content-inset-inline);
    padding-bottom: var(--space-12);
    padding-left: var(--side-pane-content-inset-inline);
  }

  .${paneSummaryHook} .${paneSectionLabelHook} {
    margin-top: 0;
    margin-right: var(--space-1);
    margin-bottom: 0;
    margin-left: 0;
  }

  .ux-app & .${paneSummaryHook} .${paneSectionLabelHook} {
    margin-top: 0;
    margin-right: 0;
    margin-bottom: 0;
    margin-left: 0;
    padding-top: 0;
    padding-right: 0;
    padding-bottom: 0;
    padding-left: 0;
  }

  .${paneLegendRowHook} {
    display: flex;
    align-items: center;
    gap: var(--space-6);
    min-height: var(--side-pane-legend-row-height);
    padding-top: 0;
    padding-right: var(--space-7);
    padding-bottom: 0;
    padding-left: var(--space-7);
  }

  .${paneLegendTextHook} {
    font-size: var(--text-caption);
    line-height: 1.3;
  }

  .${treeHook} {
    --ds-treeitem-count-ml: var(--space-1);
    --ds-treeitem-h: var(--space-16);
    --ds-treeitem-label-flex: 0 1 auto;
    --ds-treeitem-lh: 1.2;
    --ds-treeitem-pl-base: var(--side-pane-content-inset-inline);
  --ds-treeitem-pr: var(--side-pane-content-inset-inline);
    --ds-treeitem-twist-w: var(--space-7);
    --ds-treeitem-icon-color: var(--text-secondary);
    min-height: 0;
    flex: 1 1 auto;
    overflow-x: hidden;
    overflow-y: auto;
    padding-top: 0;
    padding-right: 0;
    padding-bottom: var(--space-7);
    padding-left: 0;
    scrollbar-gutter: stable;
  }

  .${globalSearchHook} + .${paneControlsHook},
  .ux-app & .${globalSearchHook} + .${paneControlsHook} {
    padding-top: 0;
  }

  .${globalSearchHook} + .${treeHook},
  .ux-app & .${globalSearchHook} + .${treeHook} {
    padding-top: 0;
  }

  .${globalSearchHook} + .${paneControlsHook} > .${paneSectionLabelHook}:first-child,
  .ux-app & .${globalSearchHook} + .${paneControlsHook} > .${paneSectionLabelHook}:first-child {
    margin-top: 0;
  }

  .${paneControlsHook} > .${treeHook}:first-child,
  .ux-app & .${paneControlsHook} > .${treeHook}:first-child {
    padding-top: 0;
  }

  .${treeNodeHook} {
    min-width: 0;
    margin-top: 0;
    margin-right: 0;
    margin-bottom: 0;
    margin-left: 0;
  }

  .${treeNodeHook} > summary {
    list-style: none;
  }

  .${treeNodeHook} > summary::-webkit-details-marker {
    display: none;
  }

  .${paneSelectedElementHook} {
    display: grid;
    gap: var(--space-5);
    margin-top: 0;
    margin-right: 0;
    margin-bottom: 0;
    margin-left: 0;
  }

  .${paneSelectedElementHook} .${paneSectionLabelHook} {
    margin-top: 0;
    margin-right: 0;
    margin-bottom: 0;
    margin-left: 0;
  }

  .${paneSelectedElementLinkHook} {
    display: inline-flex;
    width: 100%;
    min-width: 0;
    align-items: center;
    justify-content: flex-start;
    gap: var(--space-3);
    padding-top: var(--space-2);
    padding-right: var(--control-padding-sm);
    padding-bottom: var(--space-2);
    padding-left: var(--control-padding-sm);
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

  .${paneSelectedElementLinkHook} span:last-child {
    min-width: 0;
    overflow-x: hidden;
    overflow-y: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .${paneSelectionRowHook} {
    display: flex;
    min-width: 0;
    align-items: center;
    gap: var(--space-2);
  }

  .${paneSelectionRowHook} .${paneSelectedElementLinkHook} {
    flex: 1 1 auto;
  }

  .${paneSelectionNameHook} {
    flex: 1 1 auto;
    min-width: 0;
    overflow-x: hidden;
    overflow-y: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .${paneSelectionKindHook} {
    flex: 0 0 auto;
    overflow-x: visible;
    overflow-y: visible;
    padding-top: var(--space-1);
    padding-right: var(--control-padding-sm);
    padding-bottom: var(--space-1);
    padding-left: var(--control-padding-sm);
    border-radius: var(--radius-full);
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    line-height: 1.2;
  }

  .${paneSelectionOpenHook} {
    flex: 0 0 auto;
  }

  .${paneSelectionHintHook} {
    margin-top: 0;
    margin-right: 0;
    margin-bottom: 0;
    margin-left: 0;
    font-size: var(--text-caption);
  }

  .${paneActionRowHook} {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-5);
    margin-top: 0;
    margin-right: 0;
    margin-bottom: 0;
    margin-left: 0;
  }

  .${paneGhostLinkHook} {
    display: inline-flex;
    height: var(--control-height-sm);
    align-items: center;
    gap: var(--space-4);
    padding-top: 0;
    padding-right: var(--control-padding-sm);
    padding-bottom: 0;
    padding-left: var(--control-padding-sm);
    border-radius: var(--radius-sm);
    font-size: var(--text-caption);
    font-weight: var(--weight-medium);
    text-decoration: none;
  }

  .${paneGhostLinkHook} svg {
    display: block;
    width: var(--icon-sm);
    height: var(--icon-sm);
    flex: 0 0 auto;
  }

  .${emptyHook} {
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
  flex: 1 1 auto;
  width: 100%;
  min-width: 0;
  min-height: 0;
  height: auto;

  &.is-collapsed {
    display: none;
  }
`;

export const skinX = css`
  border-right: var(--border-w) solid var(--border-subtle);
  background: var(--bg-surface);
  color: var(--text-body);

  .${treeTabHook} {
    background: var(--bg-surface);
    color: var(--text-secondary);
  }

  .${treeTabHook}:hover,
  .${treeTabHook}:focus-visible {
    background: var(--bg-hover);
    color: var(--text-secondary);
    outline: 0;
  }

  .${treeTabToggleHook} {
    color: var(--text-muted);
  }

  .${globalSearchControlHook} {
    --ds-search-input-border: var(--border-w) solid var(--border-strong);
    --ds-search-input-bg: var(--bg-canvas);
    --ds-search-input-color: var(--text-body);
    --ds-search-input-placeholder-color: var(--text-muted);
  }

  .${globalSearchResultsHook},
  .ontology-graph-results {
    border: var(--border-w) solid var(--border-subtle);
    border-radius: var(--radius-md);
    background: var(--bg-surface);
    box-shadow: var(--shadow-lg);
  }

  .${globalSearchResultsHook} button,
  .${globalSearchResultsHook} a,
  .ontology-graph-results button,
  .ontology-graph-results a {
    background: transparent;
    color: var(--text-body);
  }

  .${globalSearchResultsHook} button:hover,
  .${globalSearchResultsHook} a:hover,
  .ontology-graph-results button:hover,
  .ontology-graph-results a:hover {
    background: var(--bg-hover);
  }

  .${globalSearchResultsHook} small,
  .ontology-graph-results small {
    color: var(--text-muted);
  }

  .${globalSearchResultsHook} .ontology-graph-result:hover,
  .${globalSearchResultsHook} .ontology-graph-result:focus-visible,
  .ontology-graph-results .ontology-graph-result:hover,
  .ontology-graph-results .ontology-graph-result:focus-visible {
    background: var(--bg-hover);
  }

  .${globalSearchResultsHook} .ontology-graph-result.text-gray-400,
  .ontology-graph-results .ontology-graph-result.text-gray-400 {
    color: var(--text-muted);
  }

  .${globalSearchResultsHook} .ontology-graph-badge,
  .ontology-graph-results .ontology-graph-badge {
    background: var(--bg-sunken);
    color: var(--danger-text);
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph,
  .ontology-graph-results .ontology-graph-result-glyph {
    background: var(--color-role-rdf-resource);
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph[data-semantic-type="semantic-query"],
  .ontology-graph-results .ontology-graph-result-glyph[data-semantic-type="semantic-query"] {
    background: var(--color-role-ontology);
    color: var(--color-ink);
    font-family: var(--font-mono);
    font-size: var(--text-micro);
    font-weight: var(--weight-bold);
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph[data-semantic-type="class"],
  .ontology-graph-results .ontology-graph-result-glyph[data-semantic-type="class"] {
    background: var(--color-role-rdf-class);
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph[data-semantic-type="skos-concept"],
  .ontology-graph-results .ontology-graph-result-glyph[data-semantic-type="skos-concept"] {
    background: var(--color-role-rdf-concept);
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph[data-semantic-type="skos-concept-scheme"],
  .ontology-graph-results .ontology-graph-result-glyph[data-semantic-type="skos-concept-scheme"] {
    background: var(--color-role-rdf-concept-scheme);
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph[data-semantic-type="named-individual"],
  .ontology-graph-results .ontology-graph-result-glyph[data-semantic-type="named-individual"] {
    background: var(--color-role-rdf-individual);
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph[data-semantic-type="datatype"],
  .ontology-graph-results .ontology-graph-result-glyph[data-semantic-type="datatype"] {
    background: var(--color-role-rdf-datatype);
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph[data-semantic-type="class-expression"],
  .ontology-graph-results .ontology-graph-result-glyph[data-semantic-type="class-expression"] {
    background: var(--color-role-rdf-classexpr);
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph[data-semantic-type="node-shape"],
  .ontology-graph-results .ontology-graph-result-glyph[data-semantic-type="node-shape"] {
    background: var(--color-role-rdf-nodeshape);
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph[data-semantic-type="property-shape"],
  .ontology-graph-results .ontology-graph-result-glyph[data-semantic-type="property-shape"] {
    background: var(--color-role-rdf-propshape);
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph[data-semantic-type="object-property"],
  .${globalSearchResultsHook} .ontology-graph-result-glyph[data-semantic-type="property"],
  .ontology-graph-results .ontology-graph-result-glyph[data-semantic-type="object-property"],
  .ontology-graph-results .ontology-graph-result-glyph[data-semantic-type="property"] {
    background: var(--color-role-rdf-objprop);
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph[data-semantic-type="datatype-property"],
  .ontology-graph-results .ontology-graph-result-glyph[data-semantic-type="datatype-property"] {
    background: var(--color-role-rdf-dtprop);
  }

  .${globalSearchResultsHook} .ontology-graph-result-glyph[data-semantic-type="rdf-property"],
  .ontology-graph-results .ontology-graph-result-glyph[data-semantic-type="rdf-property"] {
    background: var(--color-role-rdf-rdfprop);
  }

  .${paneControlsHook} {
    --ds-togglerow-bg: transparent;
    --ds-togglerow-meta-bg: var(--bg-sunken);
    --ds-togglerow-meta-color: var(--text-secondary);
    --ds-togglerow-line-swatch-border: currentColor;
    --ds-togglerow-line-swatch-bg: transparent;
  }

  .${paneControlsTitleHook} {
    color: var(--text-strong);
  }

  .${paneSectionLabelHook},
  .${paneLegendTextHook},
  .${paneSelectionOpenHook},
  .${emptyHook} {
    color: var(--text-muted);
  }

  .${treeHook} {
    --ds-treeitem-twist-color: var(--text-muted);
  }

  .${paneSelectedElementLinkHook} {
    border: var(--border-w) solid var(--border-default);
    background: var(--bg-surface);
    color: var(--text-body);
  }

  .${paneSelectedElementLinkHook}:hover {
    border-color: var(--border-strong);
    background: var(--bg-hover);
  }

  .${paneSelectionKindHook} {
    background: var(--bg-sunken);
    color: var(--text-muted);
  }

  .${paneNavRowIconHook} {
    color: var(--text-muted);
  }

  .${paneNavRowLabelHook} {
    color: var(--text-body);
  }

  .${paneNavRowCountHook} {
    background: var(--bg-sunken);
    color: var(--text-secondary);
  }

  .${paneGhostLinkHook} {
    background: transparent;
    color: var(--text-secondary);
  }

  .${paneGhostLinkHook}:hover {
    background: var(--bg-hover);
    color: var(--text-strong);
  }
`;
