import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

export interface RouteFrameProps extends Omit<HTMLAttributes<HTMLElement>, "style"> {
  viewId?: string;
  children?: ReactNode;
}

export interface RouteLayoutProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  layout?: "single";
  children?: ReactNode;
}

export interface RoutePanelProps extends Omit<HTMLAttributes<HTMLElement>, "style"> {
  children?: ReactNode;
}

const frameBaseUX = css`
  position: absolute;
  inset: 0;
  overflow: hidden;

  &[data-view="traces"] {
    overflow: hidden;
  }

  &[data-view="traces"] [data-route-frame],
  &[data-view="traces"] .trace-main-panel {
    height: 100%;
    min-height: 0;
    overflow: hidden;
  }

  &[data-view="traces"] .trace-content-scroll {
    min-height: 0;
    overflow-x: hidden;
    overflow-y: auto;
  }

  &[data-view="coverage"] .coverage-dashboard {
    width: 100%;
    margin-right: 0;
  }

  &[data-view="thesaurus"] [data-route-frame],
  &[data-view="thesaurus"] .ux-route-panel,
  &[data-view="thesaurus"] .ux-workspace-shell,
  &[data-view="thesaurus"] .ux-thesaurus {
    height: 100%;
    min-height: 0;
    overflow: hidden;
  }

  &[data-view="thesaurus"] .ux-route-panel {
    padding: 0;
  }
`;

const frameSkinX = css`
  background: var(--bg-canvas);
`;

const layoutBaseUX = css`
  box-sizing: border-box;
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1fr) !important;
  column-gap: 0;
  height: 100vh;
  min-height: 0;
  padding-left: var(--ux-current-left-width);
  padding-right: 0;

  &[data-route-layout="single"] {
    grid-template-columns: minmax(0, 1fr) !important;
    column-gap: 0;
  }

  [data-product-pattern="app-shell"] & {
    height: 100%;
    min-height: 0;
    overflow: hidden;
    padding-left: 0;
    padding-right: 0;
  }
`;

const layoutSkinX = css`
  background: var(--bg-canvas);
  color: var(--text-body);

  [data-product-pattern="app-shell"] & {
    background: var(--bg-canvas);
  }
`;

const panelBaseUX = css`
  position: relative;
  box-sizing: border-box;
  min-width: 0;
  min-height: 0;
  overflow: auto;
  padding: var(--space-14) var(--space-16);

  [data-product-pattern="app-shell"] & {
    height: 100%;
    min-height: 0;
    overflow: auto;
    padding: var(--space-16);
  }
`;

const panelSkinX = css`
  border-left: var(--border-w) solid var(--border-panel);
  border-right: var(--border-w) solid var(--border-panel);
  background: var(--bg-surface);

  [data-product-pattern="app-shell"] & {
    border-right: 0;
    border-left: 0;
    background: var(--bg-surface);
  }
`;

export function RouteFrame({
  viewId,
  children,
  className = "",
  ...props
}: RouteFrameProps) {
  return (
    <main
      data-product-pattern="route-frame"
      data-view={viewId}
      className={cx("ux-route-frame", frameBaseUX, frameSkinX, className)}
      {...props}
    >
      {children}
    </main>
  );
}

export function RouteLayout({
  layout = "single",
  children,
  className = "",
  ...props
}: RouteLayoutProps) {
  return (
    <div
      data-product-pattern="route-layout"
      data-route-frame
      data-route-layout={layout}
      className={cx("ux-route-layout", layoutBaseUX, layoutSkinX, className)}
      {...props}
    >
      {children}
    </div>
  );
}

export function RoutePanel({
  children,
  className = "",
  ...props
}: RoutePanelProps) {
  return (
    <section
      data-product-pattern="route-panel"
      className={cx("ux-route-panel", panelBaseUX, panelSkinX, className)}
      {...props}
    >
      {children}
    </section>
  );
}
