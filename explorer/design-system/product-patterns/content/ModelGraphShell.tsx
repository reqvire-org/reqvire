import type { ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { RoutePanel } from "../shell";

export interface ModelGraphShellProps {
  rootLabel: ReactNode;
  currentLabel: ReactNode;
  countLabel?: ReactNode;
  controls?: ReactNode;
  children?: ReactNode;
  breadcrumbLabel?: string;
  onRootClick?: () => void;
}

const shellUX = css`
  --ux-model-toolbar-actions-min-w: 280px;
  --ux-model-crumb-max-w: 190px;
  --ux-model-crumb-wide-max-w: 240px;
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
  [data-model-graph-crumb-current] {
    max-width: var(--ux-model-crumb-max-w);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  button {
    border: 0;
    background: transparent;
    cursor: pointer;
  }

  [data-model-graph-crumb-current] {
    display: inline-block;
    max-width: var(--ux-model-crumb-wide-max-w);
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

  [data-model-graph-crumb-current] {
    color: var(--text-strong);
  }

  [data-model-graph-crumb-separator] {
    color: var(--text-separator);
  }
`;

const actionsUX = css`
  display: flex;
  min-width: min(100%, var(--ux-model-toolbar-actions-min-w));
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

export function ModelGraphShell({
  rootLabel,
  currentLabel,
  countLabel,
  controls,
  children,
  breadcrumbLabel = "Model graph breadcrumbs",
  onRootClick,
}: ModelGraphShellProps) {
  return (
    <RoutePanel
      data-panel="document"
      data-model-graph-shell
      className={cx("ux-model-graph-shell", shellUX, shellSkinX)}
    >
      <div className={cx("ux-model-graph-shell__toolbar", toolbarUX, toolbarSkinX)}>
        <div
          className={cx("ux-model-graph-shell__breadcrumbs", breadcrumbsUX, breadcrumbsSkinX)}
          aria-label={breadcrumbLabel}
        >
          <span className={cx("ux-model-graph-shell__crumb", crumbUX, crumbSkinX)}>
            <button type="button" onClick={onRootClick}>
              {rootLabel}
            </button>
          </span>
          <span className={cx("ux-model-graph-shell__crumb", crumbUX, crumbSkinX)}>
            <span data-model-graph-crumb-separator>/</span>
            <span data-model-graph-crumb-current>{currentLabel}</span>
          </span>
        </div>
        <div className={cx("ux-model-graph-shell__actions", actionsUX)}>
          {countLabel ? (
            <span className={cx("ux-model-graph-shell__count", countUX, countSkinX)}>
              {countLabel}
            </span>
          ) : null}
          {controls}
        </div>
      </div>
      {children}
    </RoutePanel>
  );
}
