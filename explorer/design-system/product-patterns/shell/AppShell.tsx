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
  z-index: var(--z-sticky);
  display: flex;
  flex: 0 0 var(--app-header-height);
  align-items: stretch;
  height: var(--app-header-height);

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
