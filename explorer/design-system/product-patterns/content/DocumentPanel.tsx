import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

export interface DocumentPanelToolbarProps {
  label?: ReactNode;
  title?: ReactNode;
  actionHref?: string;
  actionLabel?: ReactNode;
}

export interface DocumentPanelProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  toolbar?: DocumentPanelToolbarProps;
  children?: ReactNode;
  layout?: "page" | "embedded";
}

const routeBaseUX = css`
  position: relative;
  display: grid;
  box-sizing: border-box;
  grid-template-columns: minmax(0, 1fr) !important;
  column-gap: 0;
  height: 100vh;
  min-height: 0;
  padding-right: 0;
  padding-left: var(--ux-current-left-width);

  .ux-app & {
    height: 100%;
    min-height: 0;
    overflow: hidden;
    padding-right: 0;
    padding-left: 0;
  }
`;

const routeSkinX = css`
  background: var(--bg-canvas);
  color: var(--text-body);

  .ux-app & {
    background: var(--bg-canvas);
  }
`;

const routeEmbeddedUX = css`
  flex: 1 1 auto;
  height: 100%;
  min-height: 0;
  padding-left: 0;
`;

const panelBaseUX = css`
  position: relative;
  box-sizing: border-box;
  min-width: 0;
  min-height: 0;
  overflow: auto;
  padding: var(--space-14) var(--space-16);

  .ux-app & {
    height: 100%;
    min-height: 0;
    overflow: auto;
    padding: var(--space-16);
  }
`;

const panelSkinX = css`
  border-right: var(--border-w) solid var(--border-panel);
  border-left: var(--border-w) solid var(--border-panel);
  background: var(--bg-surface);

  .ux-app & {
    border-right: 0;
    border-left: 0;
    background: var(--bg-surface);
  }
`;

const toolbarBaseUX = css`
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-6);
  margin-bottom: var(--space-8);
  padding: 0 0 var(--space-6);

  .ux-content-page__title {
    display: grid;
    min-width: 0;
    gap: var(--space-1);
  }

  .ux-content-page__title span {
    font-size: var(--text-micro);
    font-weight: var(--weight-bold);
    letter-spacing: 0;
    text-transform: uppercase;
  }

  .ux-content-page__title strong {
    min-width: 0;
    overflow: hidden;
    font-size: var(--text-base);
    text-overflow: ellipsis;
    white-space: nowrap;
  }
`;

const toolbarSkinX = css`
  border-bottom: var(--border-w) solid var(--border-subtle);

  .ux-content-page__title span {
    color: var(--text-muted);
  }

  .ux-content-page__title strong {
    color: var(--text-body);
  }
`;

const commandBaseUX = css`
  border: 0;
  border-radius: var(--radius-md);
  padding: var(--space-3) var(--space-4);
  cursor: pointer;
  font-size: var(--text-sm);
  text-align: left;
  text-decoration: none;
`;

const commandSkinX = css`
  background: var(--accent);
  color: var(--accent-fg);
`;

export function DocumentPanel({
  toolbar,
  children,
  layout = "page",
  className = "",
  ...props
}: DocumentPanelProps) {
  return (
    <div
      className={cx("ux-content-route", routeBaseUX, layout === "embedded" && routeEmbeddedUX, routeSkinX, className)}
      data-product-pattern="document-panel"
      data-layout={layout}
      {...props}
    >
      <section className={cx("ux-content-document-panel", panelBaseUX, panelSkinX)}>
        {toolbar ? <DocumentPanelToolbar {...toolbar} /> : null}
        {children}
      </section>
    </div>
  );
}

export function DocumentPanelToolbar({
  label = "Source page",
  title,
  actionHref,
  actionLabel,
}: DocumentPanelToolbarProps) {
  const hasAction = actionHref && actionLabel;

  return (
    <div className={cx("ux-content-page__toolbar", toolbarBaseUX, toolbarSkinX)}>
      <div className="ux-content-page__title">
        <span>{label}</span>
        <strong>{title || "Unknown file"}</strong>
      </div>
      {hasAction ? (
        <a className={cx("ux-content-page__command", commandBaseUX, commandSkinX)} href={actionHref}>
          {actionLabel}
        </a>
      ) : null}
    </div>
  );
}
