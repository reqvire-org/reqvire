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
  /** Persistent context control after the brand, within the pane-aligned header segment. */
  headerContext?: ReactNode;
  /** Context controls below the persistent navigation header. */
  toolbar?: ReactNode;
  navigationItems?: ShellNavigationItem[];
  activeNavigationValue?: string;
  headerActions?: ShellActionItem[];
  sidePane?: ReactNode;
  sidePaneHeader?: ReactNode;
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

  .ux-mode-nav {
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

`;

const headerBaseUX = css`
  --ux-header-leading-min-width: calc(var(--space-32) * 2 + var(--space-16));
  --ux-header-leading-width: var(--ux-header-leading-min-width);
  z-index: var(--z-sticky);
  display: flex;
  flex: 0 0 var(--app-header-height);
  align-items: stretch;
  height: var(--app-header-height);

  &[data-align-pane="true"] {
    --ux-header-leading-width: max(var(--ux-current-left-width), var(--ux-header-leading-min-width));
  }

  &[data-has-context="true"] {
    --ux-header-leading-min-width: calc(var(--space-32) * 4 + var(--space-16));
    @media (max-width: 600px) {
      --ux-header-leading-min-width: calc(var(--space-32) * 3);
    }
  }

  @media (max-width: 1100px) {
    display: grid;
    flex-basis: auto;
    grid-template-columns: var(--ux-header-leading-width) minmax(0, 1fr);
    grid-template-rows: var(--app-header-height) var(--control-lg);
    height: auto;
  }

  @media (max-width: 640px) {
    grid-template-columns: minmax(0, 1fr) auto;
  }
`;

const sidePaneHeaderClass = css`
  flex: 0 0 auto;
  min-width: 0;
  padding: var(--space-6) var(--side-pane-content-inset-inline) 0;
  border-right: var(--border-w) solid var(--border-subtle);
  background: var(--bg-surface);
`;

const headerSkinX = css`
  border-bottom: var(--border-w) solid var(--border-subtle);
  background: var(--bg-surface);
`;

const headerLeadingClass = css`
  display: flex;
  flex: 0 0 var(--ux-header-leading-width);
  align-items: flex-start;
  width: var(--ux-header-leading-width);
  min-width: var(--ux-header-leading-min-width);
  box-sizing: border-box;
  border-right: var(--border-w) solid var(--border-subtle);

  @media (max-width: 1100px) {
    grid-column: 1;
    grid-row: 1 / -1;
  }

  @media (max-width: 640px) {
    grid-row: 1;
    width: min(100%, var(--ux-header-leading-width));
  }
`;

const brandClass = css`
  display: flex;
  flex: 0 0 auto;
  align-items: center;
  gap: var(--space-5);
  box-sizing: border-box;
  height: var(--app-header-height);
  padding: 0 var(--space-10);

  &[data-has-context="true"] {
    padding-right: var(--space-6);

    @media (max-width: 600px) {
      padding-inline: var(--space-6);
      > span { display: none; }
    }
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

const headerContextClass = css`
  display: flex;
  flex: 1 1 auto;
  align-items: center;
  min-width: 0;
  margin-left: auto;
  max-width: calc(var(--space-32) * 4 + var(--space-6));
  height: var(--app-header-height);
  box-sizing: border-box;
  padding-right: var(--space-6);
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

  @media (max-width: 1100px) {
    grid-column: 2;
    grid-row: 2;
  }

  @media (max-width: 640px) {
    grid-column: 1 / -1;
    padding-inline: var(--space-6);
  }
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

  @media (max-width: 1100px) {
    grid-column: 2;
    grid-row: 1;
    justify-content: flex-end;
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
    headerContext,
    toolbar,
    navigationItems = [],
    activeNavigationValue,
    headerActions = [],
    sidePane,
    sidePaneHeader,
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
      <ShellHeader
        brandLabel={brandLabel}
        context={headerContext}
        alignPane={sidePane != null}
        navigationItems={navigationItems}
        activeNavigationValue={activeNavigationValue}
        headerActions={headerActions}
        onNavigate={onNavigate}
      />
      {toolbar}
      <div data-product-pattern-slot="body" className={cx(mainClass)}>
        {sidePane != null ? (
          <ShellPane placement="start" collapsed={!leftPaneOpen}>
            {sidePaneHeader != null ? <div className={cx(sidePaneHeaderClass)} data-product-pattern-slot="side-pane-header">{sidePaneHeader}</div> : null}
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
  context,
  alignPane,
  navigationItems,
  activeNavigationValue,
  headerActions,
  onNavigate,
}: {
  brandLabel: ReactNode;
  context?: ReactNode;
  alignPane: boolean;
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
    <header data-product-pattern="shell-header" data-has-context={context != null || undefined}
      data-align-pane={alignPane || undefined} className={cx(headerBaseUX, headerSkinX)}>
      <div data-product-pattern-slot="header-leading" className={cx(headerLeadingClass)}>
        <div data-product-pattern-slot="brand" data-has-context={context != null || undefined} className={cx(brandClass)}>
          <BrandMark className={cx(brandMarkClass)} decorative />
          {brandLabel != null ? <span className={cx(brandNameClass)}>{brandLabel}</span> : null}
        </div>
        {context != null ? <div data-product-pattern-slot="header-context" className={cx(headerContextClass)}>{context}</div> : null}
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
        <div data-product-pattern-slot="header-actions" className={cx(headerActionsClass)}>
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
