import {
  forwardRef,
  type HTMLAttributes,
  type KeyboardEventHandler,
  type PointerEventHandler,
  type ReactNode,
} from "react";
import { css, cx } from "@linaria/atomic";
import { BrandMark } from "../../components/core/BrandMark";
import { Icon, type IconName } from "../../components/core/Icon";
import { IconButton } from "../../components/core/IconButton";
import { Tabs, type TabItem } from "../../components/controls/Tabs";
import { PaneResizer } from "./PaneResizer";
import { ShellMain } from "./ShellMain";
import { ShellPane } from "./ShellPane";

export interface ShellNavigationItem {
  value: string;
  label: ReactNode;
  icon?: IconName;
  badge?: ReactNode;
}

export interface ShellActionItem {
  id: string;
  label: string;
  icon: IconName;
  active?: boolean;
  disabled?: boolean;
  onClick: () => void;
}

export interface AppShellProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  brandLabel?: ReactNode;
  toolbar?: ReactNode;
  navigationItems?: ShellNavigationItem[];
  activeNavigationValue?: string;
  headerActions?: ShellActionItem[];
  sidePane?: ReactNode;
  main?: ReactNode;
  detailPane?: ReactNode;
  mainWarning?: ReactNode;
  leftPaneOpen?: boolean;
  leftPaneResizing?: boolean;
  leftPaneWidth?: number;
  leftPaneMinWidth?: number;
  leftPaneMaxWidth?: number;
  leftPaneCollapseLabel?: string;
  leftPaneExpandLabel?: string;
  leftPaneResizeLabel?: string;
  onNavigate?: (value: string) => void;
  onToggleLeftPane?: () => void;
  onLeftPaneResizePointerDown?: PointerEventHandler<HTMLDivElement>;
  onLeftPaneResizeKeyDown?: KeyboardEventHandler<HTMLDivElement>;
  children?: ReactNode;
}

const shellBaseUX = css`
  --ux-left-pane-width: 380px;
  --ux-left-pane-collapsed-width: 30px;
  --ux-graph-side-panel-w: 390px;
  --ux-graph-side-panel-max-h: 420px;
  --ux-current-left-width: var(--ux-left-pane-width);
  --ux-current-right-width: 0px;
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  font-family: var(--font-sans);

  &.is-left-collapsed {
    --ux-current-left-width: var(--ux-left-pane-collapsed-width);
  }

  &.is-right-collapsed {
    --ux-current-right-width: 0px;
  }

  &.is-left-resizing,
  &.is-left-resizing * {
    cursor: ew-resize !important;
    user-select: none;
  }

  &.is-left-collapsed [data-product-pattern="pane-resizer"] {
    display: none;
  }

  &.has-right-inspector .ux-inspector-tab {
    display: flex;
  }

  .ux-side-pane {
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

  .ux-side-pane.is-collapsed {
    display: none;
  }

  .ux-side-content {
    display: flex;
    flex: 1 1 auto;
    flex-direction: column;
    width: 100%;
    height: 100%;
    min-height: 0;
    overflow: hidden;
  }

  .ux-tree {
    --ds-treeitem-count-ml: var(--space-1);
    --ds-treeitem-h: var(--space-16);
    --ds-treeitem-label-flex: 0 1 auto;
    --ds-treeitem-lh: 1.2;
    --ds-treeitem-pr: var(--space-6);
    --ds-treeitem-twist-w: var(--space-7);
    --ds-treeitem-twist-color: var(--text-muted);
    --ds-treeitem-icon-color: var(--text-secondary);
    flex: 1 1 auto;
    min-height: 0;
    overflow-x: hidden;
    overflow-y: auto;
    padding: var(--space-5) 0 var(--space-7);
    border-top: var(--border-w) solid;
    scrollbar-gutter: stable;
  }

  .ux-tree-tab,
  .ux-pane-chrome-header {
    display: none;
  }

  [data-route-frame] {
    height: 100%;
    min-height: 0;
    overflow: hidden;
    padding-left: 0;
    padding-right: 0;
  }

  [data-panel="main"] {
    padding: var(--space-12) var(--space-16) var(--space-24);
  }

  [data-panel="document"] {
    height: 100%;
    min-height: 0;
    overflow: auto;
    padding: var(--space-16);
  }

  .ux-global-search {
    margin: var(--space-12) var(--space-10) 0;
  }

  .ux-global-search-control {
    position: relative;
  }

  .ux-pane-controls {
    display: flex;
    flex: 1 1 auto;
    flex-direction: column;
    gap: var(--gap-list-stack);
    min-height: 0;
    overflow-x: hidden;
    overflow-y: auto;
    padding: var(--space-12) var(--space-10) var(--space-16);
    scrollbar-gutter: stable;
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
    --ds-togglerow-line-swatch-w: calc(var(--space-8) + var(--space-1));
    --ds-togglerow-static-cursor: default;
  }

  .ux-pane-controls-title {
    margin: 0 0 var(--space-7);
    font-size: var(--text-lg);
    font-weight: var(--weight-semibold);
    letter-spacing: 0;
    line-height: var(--leading-tight);
  }

  .ux-pane-section-label {
    display: block;
    margin: var(--space-12) 0 var(--space-5);
    padding: 0 var(--space-2);
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-label);
    line-height: 1;
    text-transform: uppercase;
  }

  .ux-pane-summary {
    --ds-section-head-p: 0;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: var(--space-5);
    margin: 0 0 var(--space-10);
  }

  .ux-pane-summary .ux-pane-section-label {
    margin: 0;
    padding: 0;
  }

  .ux-pane-summary .ux-summary {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-3) var(--space-8);
    --ds-stat-display: flex;
    --ds-stat-min-w: 0;
    --ds-stat-jc: space-between;
  }

  .ux-pane-action-row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-5);
    margin: 0;
  }

  .ux-pane-ghost-link {
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

  .ux-pane-ghost-link svg {
    display: block;
    width: var(--icon-sm);
    height: var(--icon-sm);
    flex: 0 0 auto;
  }

  .ux-pane-nav-list {
    display: flex;
    flex-direction: column;
    gap: var(--gap-list-stack);
  }

  .ux-pane-nav-row {
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

  .ux-pane-nav-row__icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .ux-pane-nav-row__label {
    overflow: hidden;
    font-size: var(--text-sm);
    font-weight: var(--weight-medium);
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ux-pane-nav-row__count {
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

  .ux-pane-legend {
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

  .ux-pane-legend-row {
    display: flex;
    align-items: center;
    gap: var(--space-6);
    min-height: var(--control-sm);
    padding: 0 var(--space-7);
  }

  .ux-pane-legend-text {
    font-size: var(--text-caption);
    line-height: 1.3;
  }

  .ux-pane-symbol {
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

  .ux-pane-selected-element {
    display: grid;
    gap: var(--space-5);
    margin: 0;
  }

  .ux-pane-selected-element .ux-pane-section-label {
    margin: 0;
    padding: 0;
  }

  .ux-pane-selected-element-link {
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

  .ux-pane-selected-element-link span:last-child {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ux-pane-selection-row {
    display: flex;
    min-width: 0;
    align-items: center;
    gap: var(--space-2);
  }

  .ux-pane-selection-row .ux-pane-selected-element-link {
    flex: 1 1 auto;
  }

  .ux-pane-selection-name {
    flex: 1 1 auto;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ux-pane-selection-kind,
  .ux-pane-selection-open {
    flex: 0 0 auto;
  }

  .ux-pane-selection-kind {
    overflow: visible;
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-pill);
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    line-height: 1.2;
  }

  .ux-pane-selection-hint {
    margin: 0;
    font-size: var(--text-caption);
  }

  @media (max-width: 900px) {
    --ux-current-left-width: min(var(--ux-left-pane-width), 82vw);

    &.is-left-collapsed {
      --ux-current-left-width: var(--ux-left-pane-collapsed-width);
    }
  }
`;

const shellSkinX = css`
  background: var(--bg-canvas);
  color: var(--text-body);

  .ux-side-pane {
    border-right: var(--border-w) solid var(--border-subtle);
    background: var(--bg-surface);
    color: var(--text-body);
  }

  .ux-mode-nav,
  .ux-pane-controls,
  .ux-tree {
    border-color: var(--border-subtle);
  }

  [data-route-frame],
  [data-panel="main"] {
    background: var(--bg-canvas);
  }

  [data-panel="document"] {
    border-right: 0;
    border-left: 0;
    background: var(--bg-surface);
  }

  .ux-pane-controls {
    --ds-togglerow-bg: transparent;
    --ds-togglerow-meta-bg: var(--bg-sunken);
    --ds-togglerow-meta-color: var(--text-secondary);
    --ds-togglerow-line-swatch-border: currentColor;
    --ds-togglerow-line-swatch-bg: transparent;
  }

  .ux-pane-controls-title {
    color: var(--text-strong);
  }

  .ux-pane-section-label {
    color: var(--text-muted);
  }

  .ux-pane-legend-text {
    color: var(--text-muted);
  }

  .ux-pane-symbol {
    border: var(--border-w) solid var(--border-subtle);
    background: var(--bg-sunken);
    color: var(--text-link);
  }

  .ux-resource-link {
    color: var(--text-secondary);
    text-decoration: none;
    font-size: var(--text-sm);
  }

  .ux-resource-link:hover {
    color: var(--text-strong);
    text-decoration: underline;
  }

  .ux-pane-ghost-link {
    color: var(--text-secondary);
    background: transparent;
  }

  .ux-pane-ghost-link:hover {
    background: var(--bg-hover);
    color: var(--text-strong);
  }

  .ux-pane-selected-element-link {
    border: var(--border-w) solid var(--border-default);
    border-radius: var(--radius-sm);
    background: var(--bg-surface);
    color: var(--text-body);
  }

  .ux-pane-selected-element-link:hover {
    border-color: var(--border-strong);
    background: var(--bg-hover);
  }

  .ux-pane-selection-kind {
    background: var(--bg-sunken);
    color: var(--text-muted);
  }

  .ux-pane-selection-open {
    color: var(--text-muted);
  }

  .ux-pane-nav-row {
    border: 0;
    border-radius: var(--radius-md);
    background: transparent;
    color: var(--text-body);
  }

  .ux-pane-nav-row:hover,
  .ux-pane-nav-row:focus-visible {
    background: var(--bg-hover);
  }

  .ux-pane-nav-row:focus-visible {
    outline: var(--focus-ring-w) solid var(--focus-ring);
    outline-offset: var(--focus-ring-offset);
  }

  .ux-pane-nav-row__icon {
    color: var(--text-muted);
  }

  .ux-pane-nav-row__label {
    color: var(--text-body);
  }

  .ux-pane-nav-row__count {
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
  --ux-brand-min-w: 160px;
  display: flex;
  flex: 0 0 var(--ux-current-left-width);
  align-items: center;
  gap: var(--space-5);
  box-sizing: border-box;
  border-right: var(--border-w) solid var(--border-subtle);
  padding: 0 var(--space-10);

  @media (max-width: 900px) {
    flex-basis: auto;
    width: auto;
    min-width: var(--ux-brand-min-w);
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
  --ux-brand-name-nudge-y: 0.5px;
  display: inline-flex;
  align-items: center;
  color: var(--text-strong);
  font-size: var(--text-md);
  font-weight: var(--weight-semibold);
  letter-spacing: 0.14em;
  line-height: 1;
  transform: translateY(var(--ux-brand-name-nudge-y));
`;

const headerTabsClass = css`
  display: flex;
  flex: 1 1 auto;
  min-width: 0;
  align-items: stretch;
  overflow-x: auto;
  overflow-y: hidden;
  padding-left: calc(var(--space-16) - var(--space-7));
  --ds-tabs-h: 100%;
  --ds-tabs-border-bottom: 0;
  --ds-tab-h: 100%;
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

const collapseBaseUX = css`
  position: absolute;
  top: 50%;
  left: calc(var(--ux-current-left-width) - var(--space-6));
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

export const AppShell = forwardRef<HTMLDivElement, AppShellProps>(function AppShell(
  {
    brandLabel = "REQVIRE",
    toolbar,
    navigationItems = [],
    activeNavigationValue,
    headerActions = [],
    sidePane,
    main,
    detailPane,
    mainWarning,
    leftPaneOpen = true,
    leftPaneResizing = false,
    leftPaneWidth,
    leftPaneMinWidth,
    leftPaneMaxWidth,
    leftPaneCollapseLabel = "Collapse pane",
    leftPaneExpandLabel = "Expand pane",
    leftPaneResizeLabel = "Resize pane",
    onNavigate,
    onToggleLeftPane,
    onLeftPaneResizePointerDown,
    onLeftPaneResizeKeyDown,
    className = "",
    children,
    ...props
  },
  ref,
) {
  const canResizeLeftPane = Boolean(onLeftPaneResizePointerDown || onLeftPaneResizeKeyDown);
  const currentToggleLabel = leftPaneOpen ? leftPaneCollapseLabel : leftPaneExpandLabel;

  return (
    <div
      ref={ref}
      data-product-pattern="app-shell"
      className={cx(
        "ux-app",
        shellBaseUX,
        shellSkinX,
        !leftPaneOpen && "is-left-collapsed",
        leftPaneResizing && "is-left-resizing",
        className,
      )}
      {...props}
    >
      {toolbar ?? (
        <ShellHeader
          brandLabel={brandLabel}
          navigationItems={navigationItems}
          activeNavigationValue={activeNavigationValue}
          headerActions={headerActions}
          onNavigate={onNavigate}
        />
      )}
      <div data-product-pattern-slot="body" className={cx(mainClass)}>
        {sidePane != null ? (
          <ShellPane placement="start" collapsed={!leftPaneOpen}>
            {sidePane}
          </ShellPane>
        ) : null}
        {sidePane != null && onToggleLeftPane != null ? (
          <button
            type="button"
            className={cx(collapseBaseUX, collapseSkinX, !leftPaneOpen && "is-collapsed")}
            aria-label={currentToggleLabel}
            aria-expanded={leftPaneOpen}
            title={currentToggleLabel}
            onClick={onToggleLeftPane}
          >
            {leftPaneOpen ? <Icon name="chevron-left" /> : <Icon name="chevron-right" />}
          </button>
        ) : null}
        {sidePane != null && canResizeLeftPane ? (
          <PaneResizer
            active={leftPaneResizing}
            orientation="vertical"
            aria-label={leftPaneResizeLabel}
            aria-orientation="vertical"
            aria-valuemin={leftPaneMinWidth}
            aria-valuemax={leftPaneMaxWidth}
            aria-valuenow={leftPaneWidth}
            tabIndex={leftPaneOpen ? 0 : -1}
            onPointerDown={onLeftPaneResizePointerDown}
            onKeyDown={onLeftPaneResizeKeyDown}
          />
        ) : null}
        <ShellMain warning={mainWarning}>{main}</ShellMain>
        {detailPane != null ? <ShellPane placement="end">{detailPane}</ShellPane> : null}
      </div>
      {children}
    </div>
  );
});

function ShellHeader({
  brandLabel,
  navigationItems,
  activeNavigationValue,
  headerActions,
  onNavigate,
}: {
  brandLabel: ReactNode;
  navigationItems: ShellNavigationItem[];
  activeNavigationValue?: string;
  headerActions: ShellActionItem[];
  onNavigate?: (value: string) => void;
}) {
  const tabItems: TabItem<string>[] = navigationItems.map((item) => ({
    value: item.value,
    label: item.label,
    icon: item.icon != null ? <Icon name={item.icon} /> : undefined,
    badge: item.badge,
  }));

  return (
    <header data-product-pattern="shell-header" className={cx(headerBaseUX, headerSkinX)}>
      <div className={cx(brandClass)}>
        <BrandMark className={cx(brandMarkClass)} decorative />
        {brandLabel != null ? <span className={cx(brandNameClass)}>{brandLabel}</span> : null}
      </div>
      <nav className={cx(headerTabsClass)} aria-label="Explorer views">
        <Tabs
          items={tabItems}
          value={activeNavigationValue}
          onChange={onNavigate}
          variant="underline"
        />
      </nav>
      {headerActions.length > 0 ? (
        <div className={cx(headerActionsClass)}>
          {headerActions.map((action) => (
            <IconButton
              key={action.id}
              aria-label={action.label}
              title={action.label}
              active={action.active ?? false}
              disabled={action.disabled}
              onClick={action.onClick}
            >
              <Icon name={action.icon} />
            </IconButton>
          ))}
        </div>
      ) : null}
    </header>
  );
}
