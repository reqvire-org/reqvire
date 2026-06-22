import type { ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { RoutePanel } from "../shell";

export interface WorkspaceShellProps {
  rootLabel: ReactNode;
  currentLabel?: ReactNode;
  countLabel?: ReactNode;
  controls?: ReactNode;
  children?: ReactNode;
  breadcrumbLabel?: string;
  onRootClick?: () => void;
  tone?: "surface" | "canvas";
  showDivider?: boolean;
}

const shellUX = css`
  --ux-workspace-toolbar-actions-min-w: 280px;
  --ux-workspace-crumb-max-w: 190px;
  --ux-workspace-crumb-wide-max-w: 240px;
  display: flex;
  height: 100%;
  flex-direction: column;
  gap: var(--space-7);
  overflow: hidden;

  [data-product-pattern="app-shell"] & {
    overflow: auto;
  }
`;

const shellSkinX = css`
  color: var(--text-body);
`;

const toolbarUX = css`
  display: flex;
  min-height: var(--space-24);
  align-items: center;
  justify-content: space-between;
  gap: var(--space-6);
  padding: 0 var(--space-2) var(--space-7);

  @media (max-width: 900px) {
    align-items: stretch;
    flex-direction: column;
  }
`;

const toolbarSkinX = css`
  border-bottom: var(--border-w) solid var(--border-default);
  background: var(--bg-surface);

  &[data-workspace-divider="false"] {
    border-bottom: 0;
  }
`;

const breadcrumbsUX = css`
  display: flex;
  min-width: 0;
  flex: 1 1 auto;
  align-items: center;
  gap: var(--space-1);
  overflow: hidden;
  font-size: var(--text-sm);
`;

const breadcrumbsSkinX = css`
  color: var(--text-muted);
`;

const crumbUX = css`
  display: inline-flex;
  min-width: 0;
  align-items: center;
  gap: var(--space-1);

  button,
  [data-workspace-crumb-current] {
    max-width: var(--ux-workspace-crumb-max-w);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  button {
    border: 0;
    background: transparent;
    cursor: pointer;
  }

  [data-workspace-crumb-current] {
    display: inline-block;
    max-width: var(--ux-workspace-crumb-wide-max-w);
    font-weight: var(--weight-medium);
  }
`;

const crumbSkinX = css`
  button {
    color: var(--text-body);
  }

  button:hover {
    text-decoration: underline;
  }

  [data-workspace-crumb-current] {
    color: var(--text-strong);
  }

  [data-workspace-crumb-separator] {
    color: var(--text-separator);
  }
`;

const actionsUX = css`
  display: flex;
  min-width: min(100%, var(--ux-workspace-toolbar-actions-min-w));
  align-items: center;
  justify-content: flex-end;
  gap: var(--space-5);
  flex-wrap: wrap;

  @media (max-width: 900px) {
    width: 100%;
    min-width: 0;
  }
`;

const countUX = css`
  font-size: var(--text-caption);
  line-height: 1.4;
`;

const countSkinX = css`
  color: var(--text-muted);
`;

const panelToneSkinX = css`
  &[data-workspace-tone="canvas"] {
    background: var(--bg-canvas);
  }
`;

export function WorkspaceShell({
  rootLabel,
  currentLabel,
  countLabel,
  controls,
  children,
  breadcrumbLabel = "Workspace breadcrumbs",
  onRootClick,
  tone = "surface",
  showDivider = true,
}: WorkspaceShellProps) {
  const showBreadcrumbs = Boolean(onRootClick || currentLabel);
  const showToolbar = showBreadcrumbs || Boolean(countLabel) || Boolean(controls);

  return (
    <RoutePanel
      data-panel="document"
      data-workspace-shell
      data-workspace-tone={tone}
      className={cx("ux-workspace-shell", shellUX, shellSkinX, panelToneSkinX)}
    >
      {showToolbar ? (
        <div
          className={cx("ux-workspace-shell__toolbar", toolbarUX, toolbarSkinX)}
          data-workspace-divider={showDivider ? "true" : "false"}
        >
          {showBreadcrumbs ? (
            <div
              className={cx("ux-workspace-shell__breadcrumbs", breadcrumbsUX, breadcrumbsSkinX)}
              aria-label={breadcrumbLabel}
            >
              <span className={cx("ux-workspace-shell__crumb", crumbUX, crumbSkinX)}>
                {onRootClick ? (
                  <button type="button" onClick={onRootClick}>
                    {rootLabel}
                  </button>
                ) : (
                  <span data-workspace-crumb-current={!currentLabel ? true : undefined}>{rootLabel}</span>
                )}
              </span>
              {currentLabel ? (
                <span className={cx("ux-workspace-shell__crumb", crumbUX, crumbSkinX)}>
                  <span data-workspace-crumb-separator>/</span>
                  <span data-workspace-crumb-current>{currentLabel}</span>
                </span>
              ) : null}
            </div>
          ) : null}
          {!showBreadcrumbs ? <span aria-hidden="true" /> : null}
          <div className={cx("ux-workspace-shell__actions", actionsUX)}>
            {countLabel ? (
              <span className={cx("ux-workspace-shell__count", countUX, countSkinX)}>
                {countLabel}
              </span>
            ) : null}
            {controls}
          </div>
        </div>
      ) : null}
      {children}
    </RoutePanel>
  );
}
