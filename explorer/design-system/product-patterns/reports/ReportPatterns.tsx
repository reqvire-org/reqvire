import type { ButtonHTMLAttributes, HTMLAttributes, ReactNode } from "react";
import { forwardRef } from "react";
import { css, cx } from "@linaria/atomic";
import { Icon } from "../../components/core/Icon";
import { BarMeterFill, ConicSwatch, DonutMeter } from "../../components/data/TokenVisual";
import { ElementIcon } from "../../components/data/ElementIcon";
import { TypeBadge } from "../../components/data/TypeBadge";
import type { DesignSystemColorToken } from "../../palette";
import { RouteLayout, RoutePanel, type RouteLayoutProps, type RoutePanelProps } from "../shell";

const tracePanelBaseUX = css`
  --ux-trace-meta-grid-min-w: 180px;
  --ux-trace-diagram-min-h: 520px;
  --ux-trace-rollup-diagram-h: min(82dvh, calc(var(--ux-trace-diagram-min-h) * 2));
  position: relative;
  box-sizing: border-box;
  display: flex;
  min-width: 0;
  min-height: 0;
  height: 100%;
  flex-direction: column;
  gap: var(--space-5);
  overflow: hidden;
  padding: var(--space-12) var(--space-16) var(--space-24);

  [data-view="traces"] & {
    height: 100%;
    min-height: 0;
    overflow: hidden;
  }

  .trace-content-scroll {
    flex: 1 1 auto;
    min-height: 0;
    overflow-x: hidden;
    overflow-y: auto;
    overscroll-behavior: contain;
  }

  .trace-content-scroll > .trace-report-view {
    min-height: 100%;
  }

  .trace-report-view {
    display: flex;
    flex-direction: column;
    gap: var(--space-14);
  }

  .trace-row-group {
    padding-top: var(--space-10);
  }

  .trace-row-group:first-child {
    padding-top: 0;
  }

  .trace-file-header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--space-6);
    margin-bottom: var(--space-6);
  }

  .trace-file-heading {
    margin: 0;
    font-size: var(--text-lg);
    font-weight: var(--weight-semibold);
    letter-spacing: 0;
  }

  .trace-file-count,
  .trace-tree-count-badge {
    flex: 0 0 auto;
    border-radius: var(--radius-pill);
    font-size: var(--text-caption);
    font-weight: var(--weight-semibold);
    line-height: 1;
  }

  .trace-file-count {
    padding: var(--space-2) var(--space-4);
  }

  .trace-tree-count-badge {
    display: inline-flex;
    align-items: center;
    padding: var(--space-1) var(--space-4);
  }

  .trace-verification-list {
    display: flex;
    flex-direction: column;
    gap: var(--stack-gap-compact);
  }

  .trace-verification-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
    outline: var(--border-w) solid transparent;
    outline-offset: calc(-1 * var(--border-w));
    padding: var(--space-7) var(--space-8) var(--space-8);
    transition:
      background-color var(--dur-fast) var(--ease-standard),
      border-color var(--dur-fast) var(--ease-standard),
      outline-color var(--dur-fast) var(--ease-standard),
      box-shadow var(--dur-fast) var(--ease-standard);
  }

  .trace-verification-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-6);
    flex-wrap: wrap;
  }

  .trace-verification-title {
    border: 0;
    background: transparent;
    padding: 0;
    cursor: pointer;
    font-size: var(--text-md);
    font-weight: var(--weight-semibold);
    text-align: left;
  }

  .trace-verification-meta {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(var(--ux-trace-meta-grid-min-w), 1fr));
    gap: var(--space-3) var(--space-8);
    margin: 0;
    font-size: var(--text-caption);
  }

  .trace-verification-meta div {
    display: flex;
    gap: var(--space-3);
  }

  .trace-verification-meta dd {
    margin: 0;
  }

  .trace-rollup-diagram {
    position: relative;
    box-sizing: border-box;
    display: flex;
    height: var(--ux-trace-rollup-diagram-h);
    min-height: 0;
    max-height: calc(100dvh - var(--space-24));
    flex-direction: column;
    overflow: hidden;
    padding: var(--space-6);
  }

  .trace-rollup-diagram .mermaid {
    flex: 1 1 auto;
    width: 100%;
    min-height: 0;
    height: 100%;
    max-height: 100%;
    text-align: center;
  }

  .trace-rollup-diagram .mermaid svg {
    max-height: 100%;
  }

  .trace-rollup-diagram .is-reqvire-clickable-node {
    cursor: pointer;
  }

  .trace-rollup-placeholder {
    display: grid;
    flex: 1 1 auto;
    min-height: 0;
    height: 100%;
    place-items: center;
    font-size: var(--text-sm);
  }
`;

const tracePanelSkinX = css`
  background: var(--bg-surface);

  .empty-note {
    color: var(--text-muted);
    font-size: var(--text-base);
    line-height: 1.4;
  }

  .trace-row-group {
    border-top: var(--border-w) solid var(--border-subtle);
    background: transparent;
  }

  .trace-row-group:first-child {
    border-top: 0;
  }

  .trace-file-heading,
  .trace-verification-title,
  .trace-verification-meta,
  .trace-verification-meta dt {
    color: var(--text-body);
  }

  .trace-file-count {
    background: var(--bg-sunken);
    color: var(--text-secondary);
  }

  .trace-tree-count-badge {
    background: var(--bg-selected);
    color: var(--accent);
  }

  .trace-verification-card {
    border-top: var(--border-w) solid var(--border-subtle);
    border-radius: var(--radius-md);
    background-color: transparent;
  }

  .trace-verification-card:hover {
    background-color: var(--bg-hover);
  }

  .trace-verification-card.is-selected,
  .trace-verification-card.is-selected:hover {
    background-color: var(--bg-selected);
    outline-color: transparent;
  }

  .trace-verification-card:first-child {
    border-top: 0;
  }

  .trace-verification-title:hover {
    color: var(--accent);
    text-decoration: underline;
    text-underline-offset: var(--space-1);
  }

  .trace-verification-meta dt {
    font-weight: var(--weight-bold);
  }

  .trace-verification-meta dd {
    color: var(--text-muted);
  }

  .trace-rollup-diagram {
    border: var(--border-w) solid var(--border-subtle);
    border-radius: var(--radius-lg);
    background: var(--bg-surface);
  }

  .trace-rollup-diagram .mermaid {
    background: var(--bg-surface);
  }

  .trace-rollup-placeholder {
    border: var(--border-w) dashed var(--border-subtle);
    border-radius: var(--radius-md);
    color: var(--text-muted);
  }
`;

const coverageDashboardBaseUX = css`
  position: relative;
  box-sizing: border-box;
  display: flex;
  min-width: 0;
  min-height: 0;
  flex-direction: column;
  gap: var(--space-12);
  overflow: auto;
  height: 100%;
  padding: var(--space-16);

  [data-product-pattern="app-shell"] [data-view="coverage"] & {
    width: 100%;
    margin-right: 0;
  }

  .coverage-header {
    display: flex;
    scroll-margin-top: var(--space-12);
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-10);
    padding-bottom: var(--space-10);
    transition:
      outline-color var(--dur-fast) var(--ease-standard),
      border-color var(--dur-fast) var(--ease-standard),
      box-shadow var(--dur-fast) var(--ease-standard);
  }

  .coverage-title-block {
    display: flex;
    min-width: 0;
    flex-direction: column;
    gap: var(--space-3);
  }

  .coverage-title-block h1 {
    margin: 0;
    font-size: var(--text-2xl);
    font-weight: var(--weight-semibold);
    line-height: var(--leading-tight);
  }

  .coverage-eyebrow {
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-label);
    line-height: var(--leading-tight);
    text-transform: uppercase;
  }

  .coverage-kpi-grid,
  .coverage-grid,
  .coverage-gap-grid {
    display: grid;
    gap: var(--space-8);
  }

  .coverage-kpi-grid {
    grid-template-columns: repeat(4, minmax(0, 1fr));
  }

  .coverage-grid,
  .coverage-gap-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .coverage-panel--wide {
    grid-column: 1 / -1;
  }

  .coverage-panel {
    display: flex;
    scroll-margin-top: var(--space-12);
    flex-direction: column;
    gap: var(--space-8);
    padding: var(--space-10);
    transition:
      background-color var(--dur-fast) var(--ease-standard),
      border-color var(--dur-fast) var(--ease-standard),
      box-shadow var(--dur-fast) var(--ease-standard);
  }

  .coverage-panel__head,
  .coverage-gap-list__head,
  .coverage-legend-row,
  .coverage-source-row__head,
  .coverage-labeled-bar__head,
  .coverage-capability-row__title {
    display: flex;
    align-items: center;
    gap: var(--space-5);
  }

  .coverage-panel__head {
    align-items: baseline;
    justify-content: space-between;
    gap: var(--space-6);
  }

  .coverage-panel__head h2,
  .coverage-gap-list__head h3 {
    margin: 0;
    font-weight: var(--weight-semibold);
    line-height: var(--leading-tight);
  }

  .coverage-panel__head h2 {
    font-size: var(--text-lg);
  }

  .coverage-kpi {
    display: flex;
    align-items: center;
    gap: var(--space-8);
    padding: var(--space-8);
    min-width: 0;
  }

  .coverage-donut,
  .coverage-breakdown__pie {
    display: grid;
    flex: none;
    place-items: center;
    border-radius: var(--radius-pill);
  }

  .coverage-donut {
    width: clamp(var(--space-24), 5vw, calc(var(--space-32) + var(--space-10)));
    height: clamp(var(--space-24), 5vw, calc(var(--space-32) + var(--space-10)));
  }

  .coverage-donut::after,
  .coverage-breakdown__pie::after {
    content: "";
    grid-area: 1 / 1;
    border-radius: inherit;
  }

  .coverage-donut::after {
    width: calc(100% - var(--space-8));
    height: calc(100% - var(--space-8));
  }

  .coverage-breakdown__pie::after {
    width: calc(100% - var(--space-10));
    height: calc(100% - var(--space-10));
  }

  .coverage-donut__center {
    z-index: var(--z-local-base);
    grid-area: 1 / 1;
    font-size: var(--text-sm);
    font-weight: var(--weight-bold);
  }

  .coverage-kpi__copy,
  .coverage-source-row,
  .coverage-labeled-bar,
  .coverage-gap-row__copy {
    display: flex;
    flex-direction: column;
  }

  .coverage-kpi__copy {
    min-width: 0;
    gap: var(--space-2);
  }

  .coverage-kpi__label {
    font-size: var(--text-sm);
    font-weight: var(--weight-semibold);
  }

  .coverage-kpi__detail,
  .coverage-empty-note,
  .coverage-gap-row__file,
  .coverage-labeled-bar__head,
  .coverage-source-row__head,
  .coverage-more {
    font-size: var(--text-caption);
    line-height: var(--leading-normal);
  }

  .coverage-breakdown {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    align-items: center;
    gap: var(--space-8);
  }

  .coverage-breakdown[data-has-pie="true"] {
    grid-template-columns: auto minmax(0, 1fr);
  }

  .coverage-breakdown__pie {
    width: var(--space-32);
    height: var(--space-32);
  }

  .coverage-breakdown__legend,
  .coverage-bar-list,
  .coverage-capability-list,
  .coverage-gap-list,
  .coverage-gap-list__rows {
    display: flex;
    flex-direction: column;
  }

  .coverage-breakdown__legend,
  .coverage-bar-list,
  .coverage-capability-list,
  .coverage-gap-list__rows {
    gap: var(--stack-gap-compact);
  }

  .coverage-legend-row,
  .coverage-source-row__head,
  .coverage-labeled-bar__head,
  .coverage-gap-list__head {
    justify-content: space-between;
  }

  .coverage-legend-row__icon {
    flex: none;
  }

  .coverage-legend-row span:nth-child(2),
  .coverage-gap-row__copy,
  .coverage-capability-row__title span:nth-child(2) {
    min-width: 0;
    flex: 1 1 auto;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .coverage-source-row,
  .coverage-labeled-bar {
    gap: var(--space-3);
  }

  .coverage-bar {
    display: block;
    overflow: hidden;
    width: 100%;
    height: var(--space-3);
    border-radius: var(--radius-pill);
  }

  .coverage-bar__fill {
    display: block;
    width: 100%;
    height: 100%;
    border-radius: inherit;
  }

  .coverage-capability-row,
  .coverage-gap-row {
    width: 100%;
    border: 0;
    cursor: pointer;
    font: inherit;
    text-align: left;
  }

  .coverage-capability-row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1.35fr);
    align-items: center;
    gap: var(--space-8);
    padding: var(--space-6);
  }

  .coverage-capability-row__title {
    min-width: 0;
    font-weight: var(--weight-semibold);
  }

  .coverage-capability-row__bars {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: var(--space-8);
  }

  .coverage-mark,
  .coverage-gap-list__head span,
  .coverage-more {
    flex: none;
    border-radius: var(--radius-pill);
    padding: var(--space-1) var(--space-4);
    font-size: var(--text-caption);
    font-weight: var(--weight-semibold);
    line-height: var(--leading-tight);
  }

  .coverage-more {
    align-self: flex-start;
    border: 0;
    cursor: pointer;
    font: inherit;
  }

  .coverage-gap-list {
    gap: var(--space-6);
    min-width: 0;
  }

  .coverage-gap-list__head h3 {
    font-size: var(--text-sm);
  }

  .coverage-gap-row {
    display: flex;
    align-items: center;
    gap: var(--space-6);
    padding: var(--space-5);
  }

  .coverage-gap-row__copy {
    gap: var(--space-1);
  }

  .coverage-gap-row__title {
    overflow: hidden;
    font-weight: var(--weight-semibold);
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .coverage-empty {
    display: flex;
    align-items: flex-start;
    gap: var(--space-8);
    padding: var(--space-12);
  }

  .coverage-empty svg {
    width: var(--icon-lg);
    height: var(--icon-lg);
    flex: none;
  }

  .coverage-empty h2 {
    margin: 0 0 var(--space-3);
    font-size: var(--text-lg);
    font-weight: var(--weight-semibold);
  }

  .coverage-empty p,
  .coverage-empty-note {
    margin: 0;
  }

  @media (max-width: 1200px) {
    .coverage-kpi-grid,
    .coverage-grid,
    .coverage-gap-grid,
    .coverage-capability-row,
    .coverage-capability-row__bars {
      grid-template-columns: minmax(0, 1fr);
    }
  }
`;

const coverageDashboardSkinX = css`
  background: var(--bg-surface);

  .coverage-header {
    border-bottom: var(--border-w) solid var(--border-subtle);
    border-radius: var(--radius-lg);
    outline: var(--border-w) solid transparent;
    outline-offset: calc(-1 * var(--border-w));
  }

  .coverage-title-block h1,
  .coverage-panel__head h2,
  .coverage-gap-list__head h3,
  .coverage-donut__center,
  .coverage-kpi__label,
  .coverage-capability-row__title,
  .coverage-gap-row__title,
  .coverage-empty h2 {
    color: var(--text-strong);
  }

  .coverage-bar,
  .coverage-mark,
  .coverage-gap-list__head span,
  .coverage-more {
    background: var(--bg-sunken);
  }

  .coverage-eyebrow {
    color: var(--text-muted);
  }

  .coverage-panel,
  .coverage-kpi,
  .coverage-empty {
    border: var(--border-w) solid var(--border-subtle);
    border-radius: var(--radius-lg);
    background: var(--bg-surface);
  }

  .coverage-panel {
    box-shadow: var(--shadow-xs);
  }

  .coverage-donut::after,
  .coverage-breakdown__pie::after {
    background: var(--bg-surface);
  }

  .coverage-kpi__detail,
  .coverage-empty-note,
  .coverage-gap-row__file,
  .coverage-labeled-bar__head,
  .coverage-source-row__head,
  .coverage-more {
    color: var(--text-muted);
  }

  .coverage-capability-row,
  .coverage-gap-row {
    background: transparent;
    color: var(--text-body);
  }

  .coverage-capability-row {
    border-radius: var(--radius-md);
  }

  .coverage-capability-row:hover,
  .coverage-gap-row:hover {
    background: var(--bg-hover);
  }

  .coverage-mark,
  .coverage-gap-list__head span,
  .coverage-more {
    color: var(--text-secondary);
  }

  .coverage-more:hover {
    background: var(--bg-hover);
    color: var(--text-strong);
  }

  .coverage-more:focus-visible {
    outline: var(--focus-w) solid var(--focus-ring);
    outline-offset: var(--space-1);
  }

  .coverage-gap-row {
    border-radius: var(--radius-md);
  }

  .coverage-empty svg {
    color: var(--text-muted);
  }
`;

export type ReportRouteLayoutProps = Omit<RouteLayoutProps, "layout">;

export function ReportRouteLayout({
  children,
  className = "",
  ...props
}: ReportRouteLayoutProps) {
  return (
    <RouteLayout className={cx("ux-report-route", className)} layout="single" {...props}>
      {children}
    </RouteLayout>
  );
}

export type TraceReportPanelProps = RoutePanelProps;

export function TraceReportPanel({
  children,
  className = "",
  ...props
}: TraceReportPanelProps) {
  return (
    <RoutePanel
      data-panel="main"
      className={cx("ux-report-trace-panel", "trace-main-panel", tracePanelBaseUX, tracePanelSkinX, className)}
      {...props}
    >
      {children}
    </RoutePanel>
  );
}

export function TraceReportContent({
  children,
  className = "",
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <div className={cx("trace-content-scroll", className)} {...props}>
      {children}
    </div>
  );
}

export const TraceRowsFrame = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(function TraceRowsFrame({
  children,
  className = "",
  ...props
}, ref) {
  return (
    <div ref={ref} className={cx("trace-report-view", className)} {...props}>
      {children}
    </div>
  );
});

export function TraceFileGroup({
  children,
  className = "",
  ...props
}: HTMLAttributes<HTMLElement>) {
  return (
    <section className={cx("trace-row-group", className)} {...props}>
      {children}
    </section>
  );
}

export function TraceFileHeader({
  file,
  countLabel,
}: {
  file: ReactNode;
  countLabel: ReactNode;
}) {
  return (
    <div className="trace-file-header">
      <h2 className="trace-file-heading">{file}</h2>
      <span className="trace-file-count">{countLabel}</span>
    </div>
  );
}

export function TraceVerificationList({ children }: { children: ReactNode }) {
  return <div className="trace-verification-list">{children}</div>;
}

export interface TraceVerificationCardProps extends Omit<HTMLAttributes<HTMLElement>, "style"> {
  selected?: boolean;
}

export function TraceVerificationCard({
  selected = false,
  children,
  className = "",
  ...props
}: TraceVerificationCardProps) {
  return (
    <article
      className={cx("trace-verification-card", selected ? "is-selected" : undefined, className)}
      {...props}
    >
      {children}
    </article>
  );
}

export function TraceVerificationHeader({
  children,
}: {
  children: ReactNode;
}) {
  return <div className="trace-verification-header">{children}</div>;
}

export function TraceVerificationTitleButton({
  children,
  className = "",
  ...props
}: ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button type="button" className={cx("trace-verification-title", className)} {...props}>
      {children}
    </button>
  );
}

export function TraceTreeCountBadge({ children }: { children: ReactNode }) {
  return <span className="trace-tree-count-badge">{children}</span>;
}

export function TraceVerificationMeta({
  rows,
}: {
  rows: readonly { label: ReactNode; value: ReactNode }[];
}) {
  return (
    <dl className="trace-verification-meta">
      {rows.map((row, index) => (
        <div key={index}>
          <dt>{row.label}</dt>
          <dd>{row.value}</dd>
        </div>
      ))}
    </dl>
  );
}

export const TraceRollupDiagramShell = forwardRef<HTMLDivElement, HTMLAttributes<HTMLDivElement>>(
  function TraceRollupDiagramShell({
    children,
    className = "",
    ...props
  }, ref) {
    return (
      <div ref={ref} className={cx("trace-rollup-diagram", className)} {...props}>
        {children}
      </div>
    );
  },
);

export function TraceRollupPlaceholder({ children }: { children: ReactNode }) {
  return <div className="trace-rollup-placeholder">{children}</div>;
}

export function ReportEmptyNote({ children }: { children: ReactNode }) {
  return <span className="empty-note">{children}</span>;
}

export type CoverageDashboardProps = RoutePanelProps;

export function CoverageDashboard({
  children,
  className = "",
  ...props
}: CoverageDashboardProps) {
  return (
    <RoutePanel
      data-panel="document"
      className={cx("ux-report-coverage-dashboard", "coverage-dashboard", coverageDashboardBaseUX, coverageDashboardSkinX, className)}
      {...props}
    >
      {children}
    </RoutePanel>
  );
}

export function CoverageHeader({
  id,
  title,
  eyebrow,
}: {
  id?: string;
  title: ReactNode;
  eyebrow: ReactNode;
}) {
  return (
    <header id={id} className="coverage-header">
      <div className="coverage-title-block">
        <span className="coverage-eyebrow">{eyebrow}</span>
        <h1>{title}</h1>
      </div>
    </header>
  );
}

export function CoverageEmptyState({
  title,
  children,
}: {
  title: ReactNode;
  children: ReactNode;
}) {
  return (
    <div className="coverage-empty">
      <Icon name="pie-chart" />
      <div>
        <h2>{title}</h2>
        <p>{children}</p>
      </div>
    </div>
  );
}

export function CoverageKpiGrid({ children }: { children: ReactNode }) {
  return (
    <section className="coverage-kpi-grid" aria-label="Coverage summary">
      {children}
    </section>
  );
}

export function CoverageGrid({ children }: { children: ReactNode }) {
  return (
    <section className="coverage-grid" aria-label="Coverage breakdown">
      {children}
    </section>
  );
}

export function CoverageGapGrid({ children }: { children: ReactNode }) {
  return (
    <section className="coverage-gap-grid" aria-label="Coverage gaps">
      {children}
    </section>
  );
}

export function CoverageKpiCard({
  label,
  detail,
  shown,
  ringPercent,
  token,
}: {
  label: ReactNode;
  detail: ReactNode;
  shown: ReactNode;
  ringPercent: number;
  token: DesignSystemColorToken;
}) {
  return (
    <div className="coverage-kpi">
      <CoverageDonut percent={ringPercent} token={token}>
        {shown}
      </CoverageDonut>
      <div className="coverage-kpi__copy">
        <span className="coverage-kpi__label">{label}</span>
        <span className="coverage-kpi__detail">{detail}</span>
      </div>
    </div>
  );
}

function CoverageDonut({
  percent,
  token,
  children,
}: {
  percent: number;
  token: DesignSystemColorToken;
  children: ReactNode;
}) {
  return (
    <DonutMeter
      className="coverage-donut"
      percent={percent}
      colorToken={token}
      aria-hidden="true"
    >
      <span className="coverage-donut__center">{children}</span>
    </DonutMeter>
  );
}

export interface CoveragePanelProps extends Omit<HTMLAttributes<HTMLElement>, "title" | "style"> {
  title: ReactNode;
  span?: "default" | "wide";
}

export function CoveragePanel({
  title,
  span = "default",
  className = "",
  children,
  ...props
}: CoveragePanelProps) {
  return (
    <section
      className={cx("coverage-panel", span === "wide" ? "coverage-panel--wide" : undefined, className)}
      {...props}
    >
      <header className="coverage-panel__head">
        <h2>{title}</h2>
      </header>
      {children}
    </section>
  );
}

export function CoverageBreakdownFrame({
  pie,
  children,
}: {
  pie?: ReactNode;
  children: ReactNode;
}) {
  return (
    <div className="coverage-breakdown" data-has-pie={pie ? "true" : undefined}>
      {pie}
      <div className="coverage-breakdown__legend">{children}</div>
    </div>
  );
}

export function CoverageBreakdownPie({
  segments,
}: {
  segments: readonly { value: number; colorToken: DesignSystemColorToken }[];
}) {
  return (
    <ConicSwatch
      className="coverage-breakdown__pie"
      segments={segments}
      aria-hidden="true"
    />
  );
}

export function CoverageLegendRow({
  label,
  value,
  type,
}: {
  label: ReactNode;
  value: ReactNode;
  type: string;
}) {
  return (
    <div className="coverage-legend-row">
      <ElementIcon className="coverage-legend-row__icon" type={type} size="sm" />
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

export function CoverageBarList({ children }: { children: ReactNode }) {
  return <div className="coverage-bar-list">{children}</div>;
}

export function CoverageSourceRow({
  label,
  value,
  children,
}: {
  label: ReactNode;
  value: ReactNode;
  children: ReactNode;
}) {
  return (
    <div className="coverage-source-row">
      <div className="coverage-source-row__head">
        <span>{label}</span>
        <span>{value}</span>
      </div>
      {children}
    </div>
  );
}

export function CoverageCapabilityList({ children }: { children: ReactNode }) {
  return <div className="coverage-capability-list">{children}</div>;
}

export interface CoverageCapabilityRowProps extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, "name" | "style"> {
  name: ReactNode;
  mark?: ReactNode;
  children: ReactNode;
}

export function CoverageCapabilityRow({
  name,
  mark,
  children,
  className = "",
  ...props
}: CoverageCapabilityRowProps) {
  return (
    <button type="button" className={cx("coverage-capability-row", className)} {...props}>
      <div className="coverage-capability-row__title">
        <ElementIcon type="capability" size="sm" />
        <span>{name}</span>
        {mark ? <CoverageMark>{mark}</CoverageMark> : null}
      </div>
      <div className="coverage-capability-row__bars">{children}</div>
    </button>
  );
}

export function CoverageMark({ children }: { children: ReactNode }) {
  return <span className="coverage-mark">{children}</span>;
}

export function CoverageGapListFrame({
  id,
  title,
  count,
  children,
}: {
  id?: string;
  title: ReactNode;
  count: ReactNode;
  children: ReactNode;
}) {
  return (
    <section id={id} className="coverage-panel coverage-gap-list">
      <header className="coverage-gap-list__head">
        <h3>{title}</h3>
        <span>{count}</span>
      </header>
      {children}
    </section>
  );
}

export function CoverageEmptyNote({ children }: { children: ReactNode }) {
  return <p className="coverage-empty-note">{children}</p>;
}

export function CoverageGapRows({ children }: { children: ReactNode }) {
  return <div className="coverage-gap-list__rows">{children}</div>;
}

export interface CoverageGapRowButtonProps extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, "title" | "type" | "style"> {
  type: string;
  family: string;
  title: ReactNode;
  file: ReactNode;
  typeLabel: ReactNode;
}

export function CoverageGapRowButton({
  type,
  family,
  title,
  file,
  typeLabel,
  className = "",
  ...props
}: CoverageGapRowButtonProps) {
  return (
    <button type="button" className={cx("coverage-gap-row", className)} {...props}>
      <ElementIcon type={type} family={family} size="sm" />
      <span className="coverage-gap-row__copy">
        <span className="coverage-gap-row__title">{title}</span>
        <span className="coverage-gap-row__file">{file}</span>
      </span>
      <TypeBadge type={type} family={family} tinted>
        {typeLabel}
      </TypeBadge>
    </button>
  );
}

export function CoverageMoreButton({
  children,
  className = "",
  ...props
}: ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button type="button" className={cx("coverage-more", className)} {...props}>
      {children}
    </button>
  );
}

export function LabeledCoverageBarFrame({
  label,
  value,
  children,
}: {
  label: ReactNode;
  value: ReactNode;
  children: ReactNode;
}) {
  return (
    <div className="coverage-labeled-bar">
      <div className="coverage-labeled-bar__head">
        <span>{label}</span>
        <span>{value}</span>
      </div>
      {children}
    </div>
  );
}

export function CoverageBarFrame({
  value,
  token,
}: {
  value: number;
  token: DesignSystemColorToken;
}) {
  return (
    <span className="coverage-bar" aria-hidden="true">
      <BarMeterFill
        className="coverage-bar__fill"
        value={value}
        colorToken={token}
      />
    </span>
  );
}
