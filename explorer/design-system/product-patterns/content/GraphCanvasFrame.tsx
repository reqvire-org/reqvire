import { forwardRef, type HTMLAttributes, type ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

export interface GraphRouteProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  embedded?: boolean;
  children?: ReactNode;
}

export interface GraphCanvasFrameProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  children?: ReactNode;
}

export interface GraphCanvasSurfaceProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  variant?: "knowledge" | "ontology";
}

export interface GraphCanvasNoticeProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  children?: ReactNode;
}

const routeUX = css`
  box-sizing: border-box;
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1fr) !important;
  column-gap: 0;
  height: 100vh;
  min-height: 0;
  padding-left: var(--ux-current-left-width);
  padding-right: 0;

  &[data-embedded="true"] {
    flex: 1 1 auto;
    height: auto;
  }

  [data-product-pattern="app-shell"] & {
    height: 100%;
    min-height: 0;
    overflow: hidden;
    padding-left: 0;
    padding-right: 0;
  }
`;

const routeSkinX = css`
  background: var(--bg-surface);
  color: var(--text-body);
`;

const frameUX = css`
  position: relative;
  min-width: 0;
  min-height: 0;
  overflow: hidden;
`;

const frameSkinX = css`
  background: var(--bg-canvas);
`;

const surfaceUX = css`
  display: block;
  width: 100%;
  height: 100%;
  min-height: 0;

  &[data-graph-surface="knowledge"] {
    --ux-graph-diagram-min-h: 520px;
    min-height: var(--ux-graph-diagram-min-h);
  }
`;

const surfaceSkinX = css`
  background: var(--bg-canvas);
`;

const noticeUX = css`
  position: absolute;
  top: 50%;
  left: 50%;
  font-size: var(--text-base);
  font-style: italic;
  transform: translate(-50%, -50%);
`;

const noticeSkinX = css`
  color: var(--text-muted);
`;

export function GraphRoute({
  embedded = false,
  children,
  className = "",
  ...props
}: GraphRouteProps) {
  return (
    <div
      data-product-pattern="graph-route"
      data-embedded={embedded ? "true" : undefined}
      className={cx("ux-graph-route", routeUX, routeSkinX, className)}
      {...props}
    >
      {children}
    </div>
  );
}

export function GraphCanvasFrame({
  children,
  className = "",
  ...props
}: GraphCanvasFrameProps) {
  return (
    <div
      data-product-pattern="graph-canvas-frame"
      className={cx("ux-graph-canvas-frame", frameUX, frameSkinX, className)}
      {...props}
    >
      {children}
    </div>
  );
}

export const GraphCanvasSurface = forwardRef<HTMLDivElement, GraphCanvasSurfaceProps>(
  function GraphCanvasSurface(
    { variant = "knowledge", className = "", ...props },
    ref,
  ) {
    return (
      <div
        ref={ref}
        data-product-pattern="graph-canvas-surface"
        data-graph-surface={variant}
        className={cx("ux-graph-canvas-surface", surfaceUX, surfaceSkinX, className)}
        {...props}
      />
    );
  },
);

export function GraphCanvasNotice({
  children,
  className = "",
  ...props
}: GraphCanvasNoticeProps) {
  return (
    <div
      data-product-pattern="graph-canvas-notice"
      className={cx("ux-graph-canvas-notice", noticeUX, noticeSkinX, className)}
      {...props}
    >
      {children}
    </div>
  );
}
