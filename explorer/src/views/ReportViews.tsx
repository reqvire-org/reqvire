import {
  memo,
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type MouseEvent as ReactMouseEvent,
  type ReactNode,
} from "react";
import { css, cx } from "@linaria/atomic";
import { useStore } from "../store/StoreContext";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";
import { useExplorerUiState } from "../components/ExplorerUiState";
import type {
  ProjectStoreElement,
  TraceRequirementNode,
} from "../store/types";
import { ViewFrame } from "./ViewFrame";
import { MermaidBlock } from "../components/MarkdownContent";
import { ElementIcon, elementRole, getMermaidClassDefs, Icon, Stat, StatRow, TypeBadge, type ElementRole } from "@ds";
import { buildTraceFiles, type TraceFileNode, type TraceVerificationNode } from "../lib/traces";

/*
 * Report-projection views (Traces and Coverage).
 *
 * Each view renders natively from its Project Store report projection — no
 * iframe-mounted standalone page content. These views
 * surface store-backed report data and route element rows to the in-shell
 * element-detail modal.
 */

const reportRouteBaseUX = css`
  box-sizing: border-box;
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1fr) !important;
  column-gap: 0;
  height: 100vh;
  min-height: 0;
  padding-left: var(--ex-current-left-width);
  padding-right: 0;

  &.ex-route-single {
    grid-template-columns: minmax(0, 1fr) !important;
    column-gap: 0;
  }

  [data-view="traces"] &,
  .ex-app & {
    height: 100%;
    min-height: 0;
    overflow: hidden;
  }

  .ex-app & {
    padding-left: 0;
    padding-right: 0;
  }
`;

const reportRouteSkinX = css`
  background: var(--bg-canvas);
  color: var(--text-body);

  .ex-app & {
    background: var(--bg-canvas);
  }
`;

const tracePanelBaseUX = css`
  --ex-trace-meta-grid-min-w: 180px;
  --ex-trace-diagram-min-h: 520px;
  --ex-trace-rollup-diagram-h: min(82dvh, calc(var(--ex-trace-diagram-min-h) * 2));
  position: relative;
  box-sizing: border-box;
  display: flex;
  min-width: 0;
  min-height: 0;
  height: 100%;
  flex-direction: column;
  gap: var(--space-5);
  overflow: hidden;
  padding: var(--space-10);

  [data-view="traces"] & {
    height: 100%;
    min-height: 0;
    overflow: hidden;
  }

  .ex-app & {
    padding: var(--space-12) var(--space-16) var(--space-24);
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
    gap: var(--space-8);
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
    grid-template-columns: repeat(auto-fit, minmax(var(--ex-trace-meta-grid-min-w), 1fr));
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
    height: var(--ex-trace-rollup-diagram-h);
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
  background: var(--bg-canvas);

  .ex-app & {
    background: var(--bg-surface);
  }

  .ex-empty-note {
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
    background: color-mix(in srgb, var(--accent) 10%, transparent);
    color: var(--accent);
  }

  .trace-verification-card {
    border-top: var(--border-w) solid var(--border-subtle);
    border-radius: var(--radius-md);
    background-color: transparent;
  }

  .trace-verification-card:hover {
    background-color: color-mix(in srgb, var(--accent) 5%, transparent);
  }

  .trace-verification-card.is-selected,
  .trace-verification-card.is-selected:hover {
    background-color: color-mix(in srgb, var(--accent) 10%, transparent);
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

  .ex-app & .trace-rollup-diagram {
    border-color: var(--border-subtle);
    border-radius: var(--radius-lg);
    background: var(--bg-surface);
    box-shadow: none;
  }

  .ex-app & .trace-rollup-diagram .mermaid {
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
  padding: var(--space-14) var(--space-16);

  .ex-app & {
    height: 100%;
    min-height: 0;
    overflow: auto;
    padding: var(--space-16);
  }

  .ex-app [data-view="coverage"] & {
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

  .ex-coverage-eyebrow {
    font-size: var(--text-micro);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-label);
    line-height: var(--leading-tight);
    text-transform: uppercase;
  }

  .coverage-header-stats {
    flex: none;
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
  }

  .coverage-donut,
  .coverage-breakdown__pie {
    display: grid;
    flex: none;
    place-items: center;
    border-radius: var(--radius-pill);
  }

  .coverage-donut {
    width: var(--space-28);
    height: var(--space-28);
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
    z-index: 1;
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
    grid-template-columns: auto minmax(0, 1fr);
    align-items: center;
    gap: var(--space-8);
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
    gap: var(--space-5);
  }

  .coverage-legend-row,
  .coverage-source-row__head,
  .coverage-labeled-bar__head,
  .coverage-gap-list__head {
    justify-content: space-between;
  }

  .coverage-legend-row__swatch {
    width: var(--space-4);
    height: var(--space-4);
    flex: none;
    border-radius: var(--radius-xs);
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
    width: var(--coverage-bar-fill);
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
  border-left: var(--border-w) solid color-mix(in srgb, var(--border-subtle) 65%, transparent);
  border-right: var(--border-w) solid color-mix(in srgb, var(--border-subtle) 65%, transparent);
  background: var(--bg-surface);

  .ex-app & {
    border-right: 0;
    border-left: 0;
    background: var(--bg-surface);
  }

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

  .coverage-header-stats,
  .coverage-bar,
  .coverage-mark,
  .coverage-gap-list__head span,
  .coverage-more {
    background: var(--bg-sunken);
  }

  .ex-coverage-eyebrow {
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

  .coverage-donut,
  .coverage-breakdown__pie {
    background:
      conic-gradient(
        var(--coverage-color) var(--coverage-fill-angle),
        var(--bg-sunken) var(--coverage-fill-angle)
      );
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

  .coverage-bar__fill {
    background: var(--coverage-color);
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
    background: color-mix(in srgb, var(--accent) 5%, transparent);
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
    outline: var(--focus-ring-w) solid var(--focus-ring);
    outline-offset: var(--focus-ring-offset);
  }

  .coverage-gap-row {
    border-radius: var(--radius-md);
  }

  .coverage-empty svg {
    color: var(--text-muted);
  }
`;

type TraceMermaidQueueTask = (release: () => void) => void;

const traceMermaidRenderQueue: TraceMermaidQueueTask[] = [];
let traceMermaidRenderActive = false;

function enqueueTraceMermaidRender(task: TraceMermaidQueueTask): () => void {
  let cancelled = false;
  const queuedTask: TraceMermaidQueueTask = (release) => {
    if (cancelled) {
      release();
      return;
    }
    task(release);
  };

  traceMermaidRenderQueue.push(queuedTask);
  scheduleTraceMermaidRenderQueue();

  return () => {
    cancelled = true;
    const index = traceMermaidRenderQueue.indexOf(queuedTask);
    if (index >= 0) {
      traceMermaidRenderQueue.splice(index, 1);
    }
  };
}

function scheduleTraceMermaidRenderQueue() {
  if (traceMermaidRenderActive || traceMermaidRenderQueue.length === 0) return;

  const run = () => {
    if (traceMermaidRenderActive) return;
    const task = traceMermaidRenderQueue.shift();
    if (!task) return;

    traceMermaidRenderActive = true;
    let released = false;
    const release = () => {
      if (released) return;
      released = true;
      traceMermaidRenderActive = false;
      scheduleTraceMermaidRenderQueue();
    };

    task(release);
  };

  if (typeof window !== "undefined" && "requestIdleCallback" in window) {
    window.requestIdleCallback(run, { timeout: 600 });
  } else {
    globalThis.setTimeout(run, 16);
  }
}

export function TracesView({
  onOpenElement,
}: {
  onOpenElement: (id: string) => void;
} & Partial<ExplorerViewProps>) {
  const { store } = useStore();
  const {
    traceFilePath,
    setTraceFilePath,
    traceSelectionId,
    setTraceSelectionId: setSelectedId,
  } = useExplorerUiState();
  const elementById = useMemo(
    () => new Map(store.elements.map((element) => [element.id, element])),
    [store.elements],
  );
  const traceFiles = useMemo(() => {
    return buildTraceFiles(store);
  }, [store]);
  const selectedFile = useMemo(
    () => traceFiles.find((file) => file.file === traceFilePath) ?? traceFiles[0],
    [traceFilePath, traceFiles],
  );

  useEffect(() => {
    if (traceFiles.length === 0) {
      if (traceFilePath !== null) setTraceFilePath(null);
      if (traceSelectionId !== null) setSelectedId(null);
      return;
    }

    const selectedExists = traceFilePath
      ? traceFiles.some((file) => file.file === traceFilePath)
      : false;
    if (!selectedExists) {
      setTraceFilePath(traceFiles[0].file);
      setSelectedId(null);
    }
  }, [setSelectedId, setTraceFilePath, traceFilePath, traceFiles, traceSelectionId]);

  return (
    <ViewFrame testId="traces">
      <div className={cx("ex-route", "ex-route-single", reportRouteBaseUX, reportRouteSkinX)}>
        <div className={cx("ex-main-panel", "trace-main-panel", tracePanelBaseUX, tracePanelSkinX)}>
          <div className="trace-content-scroll">
            <TraceRows
              file={selectedFile}
              elementById={elementById}
              onOpenElement={onOpenElement}
              onSelect={setSelectedId}
              selectedVerificationId={traceSelectionId}
            />
            {traceFiles.length === 0 && (
              <span className={cx("ex-empty-note")}>No verification traces in store.</span>
            )}
          </div>
        </div>
      </div>
    </ViewFrame>
  );
}

function TraceRows({
  file,
  elementById,
  onOpenElement,
  onSelect,
  selectedVerificationId,
}: {
  file: TraceFileNode | undefined;
  elementById: Map<string, ProjectStoreElement>;
  onOpenElement: (id: string) => void;
  onSelect: (id: string) => void;
  selectedVerificationId: string | null;
}) {
  const containerRef = useRef<HTMLDivElement | null>(null);
  useEffect(() => {
    if (!selectedVerificationId) return;
    const target = containerRef.current?.querySelector<HTMLElement>(
      `#${traceVerificationDomId(selectedVerificationId)}`,
    );
    target?.scrollIntoView({ block: "start", behavior: "smooth" });
  }, [file?.file, selectedVerificationId]);

  if (!file) {
    return <div data-testid="trace-rows" className="trace-report-view" />;
  }

  return (
    <div ref={containerRef} data-testid="trace-rows" className="trace-report-view">
      <section className="trace-row-group">
        <div className="trace-file-header">
          <h2 className="trace-file-heading">{file.file}</h2>
          <span className="trace-file-count">
            {file.verifications.length} {file.verifications.length === 1 ? "verification" : "verifications"}
          </span>
        </div>
        <div className="trace-verification-list">
          {file.verifications.map((verification) => (
            <article
              key={verification.id}
              id={traceVerificationDomId(verification.id)}
              className={cx(
                "trace-verification-card",
                selectedVerificationId === verification.id ? "is-selected" : undefined,
              )}
            >
              <div className="trace-verification-header">
                <button
                  type="button"
                  onClick={() => {
                    onSelect(verification.id);
                  }}
                  className="trace-verification-title"
                >
                  {verification.name}
                </button>
                <span className="trace-tree-count-badge">
                  {verification.totalCount} in tree
                </span>
              </div>
              <dl className="trace-verification-meta">
                <div>
                  <dt>Type</dt>
                  <dd>{verification.verificationType ?? "verification"}</dd>
                </div>
                <div>
                  <dt>Directly Verified</dt>
                  <dd>{verification.directCount} requirements</dd>
                </div>
                <div>
                  <dt>Total in Tree</dt>
                  <dd>{verification.totalCount} requirements</dd>
                </div>
              </dl>
              <TraceRollupDiagram
                verification={verification}
                elementById={elementById}
                onOpenElement={onOpenElement}
              />
            </article>
          ))}
        </div>
      </section>
    </div>
  );
}

const TraceRollupDiagram = memo(function TraceRollupDiagram({
  verification,
  elementById,
  onOpenElement,
}: {
  verification: TraceVerificationNode;
  elementById: Map<string, ProjectStoreElement>;
  onOpenElement: (id: string) => void;
}) {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const cancelQueuedRenderRef = useRef<(() => void) | null>(null);
  const releaseRenderSlotRef = useRef<(() => void) | null>(null);
  const [shouldRender, setShouldRender] = useState(false);
  const [model, setModel] = useState<TraceRollupMermaidModel | null>(null);
  const startQueuedRender = useCallback(() => {
    if (shouldRender || model || cancelQueuedRenderRef.current || releaseRenderSlotRef.current) return;
    cancelQueuedRenderRef.current = enqueueTraceMermaidRender((release) => {
      cancelQueuedRenderRef.current = null;
      releaseRenderSlotRef.current = release;
      setModel(buildTraceRollupMermaidModel(verification, elementById));
      setShouldRender(true);
    });
  }, [elementById, model, shouldRender, verification]);

  const releaseRenderSlot = useCallback(() => {
    releaseRenderSlotRef.current?.();
    releaseRenderSlotRef.current = null;
  }, []);
  const handleDiagramClick = useCallback((event: ReactMouseEvent<HTMLDivElement>) => {
    const target = event.target;
    if (!(target instanceof Element)) return;
    const elementTarget = target.closest<HTMLElement>("[data-reqvire-element-id]");
    const elementId = elementTarget?.dataset.reqvireElementId ?? elementIdFromMermaidAnchor(target);
    if (!elementId || !elementById.has(elementId)) return;
    event.preventDefault();
    event.stopPropagation();
    onOpenElement(elementId);
  }, [elementById, onOpenElement]);

  useEffect(
    () => () => {
      cancelQueuedRenderRef.current?.();
      cancelQueuedRenderRef.current = null;
      releaseRenderSlotRef.current?.();
      releaseRenderSlotRef.current = null;
    },
    [],
  );

  useEffect(() => {
    const node = containerRef.current;
    if (!node || shouldRender) return;

    let timeout: ReturnType<typeof globalThis.setTimeout> | undefined;
    let idleCallback: ReturnType<typeof window.requestIdleCallback> | undefined;
    if (!("IntersectionObserver" in window)) {
      timeout = globalThis.setTimeout(startQueuedRender, 0);
      return () => globalThis.clearTimeout(timeout);
    }

    const observer = new IntersectionObserver(
      (entries) => {
        if (!entries.some((entry) => entry.isIntersecting)) return;
        observer.disconnect();
        if ("requestIdleCallback" in window) {
          idleCallback = window.requestIdleCallback(startQueuedRender, { timeout: 250 });
        } else {
          timeout = globalThis.setTimeout(startQueuedRender, 0);
        }
      },
      { rootMargin: "320px 0px" },
    );
    observer.observe(node);

    return () => {
      observer.disconnect();
      if (idleCallback !== undefined && "cancelIdleCallback" in window) {
        window.cancelIdleCallback(idleCallback);
      }
      if (timeout !== undefined) globalThis.clearTimeout(timeout);
    };
  }, [shouldRender, startQueuedRender]);

  return (
    <div ref={containerRef} className={cx("trace-rollup-diagram")} onClickCapture={handleDiagramClick}>
      {shouldRender && model ? (
        <MermaidBlock
          code={model.code}
          nodeClickTargets={model.nodeClickTargets}
          onNodeClick={onOpenElement}
          onRenderSettled={releaseRenderSlot}
        />
      ) : (
        <div className="trace-rollup-placeholder">
          Diagram queued. Rows remain interactive while rendering continues.
        </div>
      )}
    </div>
  );
});

function elementIdFromMermaidAnchor(target: Element): string | null {
  const anchor = target.closest<HTMLAnchorElement>("a[href]");
  if (!anchor) return null;
  return elementIdFromMermaidHref(anchor.getAttribute("href") ?? anchor.href);
}

function elementIdFromMermaidHref(href: string): string | null {
  const contentPrefix = "#/content/";
  let hash = href;
  if (!hash.startsWith("#")) {
    try {
      hash = new URL(href, window.location.href).hash;
    } catch {
      return null;
    }
  }
  if (!hash.startsWith(contentPrefix)) return null;
  const rawId = hash.slice(contentPrefix.length);
  try {
    return decodeURIComponent(rawId);
  } catch {
    return rawId;
  }
}

interface TraceDiagramElement {
  id: string;
  name: string;
  type: string;
}

interface TraceRollupMermaidModel {
  code: string;
  nodeClickTargets: ReadonlyMap<string, string>;
}

function buildTraceRollupMermaidModel(
  verification: TraceVerificationNode,
  elementById: Map<string, ProjectStoreElement>,
): TraceRollupMermaidModel {
  const nodeClickTargets = new Map<string, string>();
  const code = buildTraceRollupMermaid(verification, elementById, nodeClickTargets);
  return { code, nodeClickTargets };
}

function buildTraceRollupMermaid(
  verification: TraceVerificationNode,
  elementById: Map<string, ProjectStoreElement>,
  nodeClickTargets = new Map<string, string>(),
): string {
  const elements = new Map<string, TraceDiagramElement>();
  const edges = new Set<string>();
  const edgeLines: string[] = [];

  const addElement = (element: TraceDiagramElement) => {
    elements.set(element.id, element);
  };
  const addEdge = (source: string, label: string, target: string) => {
    const key = `${source}\0${label}\0${target}`;
    if (edges.has(key)) return;
    edges.add(key);
    edgeLines.push(`  ${mermaidNodeId(source)} -->|${label}| ${mermaidNodeId(target)};`);
  };

  addElement({
    id: verification.id,
    name: verification.name,
    type: "verification",
  });

  const addRequirementNode = (node: TraceRequirementNode) => {
    addElement({
      id: node.id,
      name: node.name,
      type: node.type,
    });
    if (node.is_directly_verified) {
      addEdge(verification.id, "verifies", node.id);
    }
    for (const child of node.children ?? []) {
      addRequirementNode(child);
      addEdge(node.id, "derivedFrom", child.id);
    }
  };

  if (verification.traceTree?.requirements.length) {
    for (const requirement of verification.traceTree.requirements) {
      addRequirementNode(requirement);
    }
  } else {
    for (const requirementId of verification.requirementIds) {
      const element = elementById.get(requirementId);
      addElement({
        id: requirementId,
        name: element?.name ?? requirementId,
        type: element?.element_type ?? "requirement",
      });
      addEdge(verification.id, "verifies", requirementId);
    }
  }

  const grouped = groupTraceDiagramElements([...elements.values()]);
  const lines = [
    "graph TD",
    ...getMermaidClassDefs(),
    "",
  ];

  for (const [folder, files] of grouped) {
    const folderId = mermaidNodeId(`folder:${folder}`);
    lines.push(`  subgraph ${folderId}["${escapeMermaidLabel(folder || "root")}"]`);
    for (const [file, fileElements] of files) {
      const fileId = mermaidNodeId(`file:${folder}:${file}`);
      lines.push(`    subgraph ${fileId}["${escapeMermaidLabel(file)}"]`);
      for (const element of fileElements) {
        const nodeId = mermaidNodeId(element.id);
        nodeClickTargets.set(nodeId, element.id);
        lines.push(
          `      ${nodeId}["${escapeMermaidLabel(element.name)}"]:::${mermaidClassForType(element.type)}`,
        );
        lines.push(`      click ${nodeId} "${spaRouteForElement(element.id)}";`);
      }
      lines.push("    end");
    }
    lines.push("  end");
  }

  lines.push(...edgeLines);
  return lines.join("\n");
}

function groupTraceDiagramElements(elements: TraceDiagramElement[]) {
  const folders = new Map<string, Map<string, TraceDiagramElement[]>>();
  for (const element of elements) {
    const path = element.id.split("#")[0] || element.id;
    const slash = path.lastIndexOf("/");
    const folder = slash >= 0 ? path.slice(0, slash) : "";
    const file = slash >= 0 ? path.slice(slash + 1) : path;
    const files = folders.get(folder) ?? new Map<string, TraceDiagramElement[]>();
    const fileElements = files.get(file) ?? [];
    fileElements.push(element);
    files.set(file, fileElements);
    folders.set(folder, files);
  }

  return [...folders.entries()]
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([folder, files]) => [
      folder,
      new Map(
        [...files.entries()]
          .sort(([a], [b]) => a.localeCompare(b))
          .map(([file, fileElements]) => [
            file,
            fileElements.sort((a, b) => a.id.localeCompare(b.id)),
          ]),
      ),
    ] as const);
}

function mermaidClassForType(type: string): string {
  const normalized = type.toLowerCase();
  if (normalized === "system-requirement") return "systemRequirement";
  return mermaidClassForRole(elementRole(type));
}

function mermaidClassForRole(role: ElementRole): string {
  switch (role) {
    case "input-output":
      return "inputOutput";
    case "semantic-contract":
      return "semanticContract";
    default:
      return role === "other" ? "default" : role;
  }
}

function mermaidNodeId(value: string): string {
  let hash = 2166136261;
  for (let index = 0; index < value.length; index += 1) {
    hash ^= value.charCodeAt(index);
    hash = Math.imul(hash, 16777619);
  }
  return `n${(hash >>> 0).toString(16)}`;
}

function traceVerificationDomId(id: string): string {
  return `trace-verification-${mermaidNodeId(id).slice(1)}`;
}

function escapeMermaidLabel(label: string): string {
  return label.replace(/\\/g, "\\\\").replace(/"/g, '\\"').replace(/\n/g, " ");
}

function spaRouteForElement(id: string): string {
  const [file, anchor] = id.split("#");
  return anchor ? `#/content/${file}#${anchor}` : `#/content/${file}`;
}

export function __testBuildTraceRollupMermaid(
  verification: TraceVerificationNode,
  elementById: Map<string, ProjectStoreElement>,
) {
  return buildTraceRollupMermaid(verification, elementById);
}

interface CoverageSummaryLike {
  total_leaf_requirements?: number;
  verified_leaf_requirements?: number;
  unverified_leaf_requirements?: number;
  leaf_requirements_coverage_percentage?: number;
  total_test_verifications?: number;
  satisfied_test_verifications?: number;
  unsatisfied_test_verifications?: number;
  test_verifications_satisfaction_percentage?: number;
  total_verifications?: number;
  orphaned_verifications?: number;
  orphaned_verifications_percentage?: number;
  total_requirements_in_scope?: number;
  covered_requirements?: number;
  uncovered_requirements?: number;
  implementation_coverage_percentage?: number;
  verification_types?: Record<string, number>;
  coverage_sources?: Record<string, number>;
}

interface CoverageProjectionLike {
  summary?: CoverageSummaryLike;
  unverified_leaf_requirements?: unknown;
  unsatisfied_test_verifications?: unknown;
  orphaned_verifications?: unknown;
  covered_requirements?: unknown;
  uncovered_requirements?: unknown;
  satisfied_test_verifications?: unknown;
  capability_coverage?: {
    capabilities?: CapabilityCoverageDetails[];
  };
}

interface CapabilityCoverageDetails {
  identifier: string;
  name: string;
  aggregate_leaf_requirements?: number;
  aggregate_verified_leaf_requirements?: number;
  verification_coverage_percentage?: number;
  aggregate_requirements?: number;
  aggregate_covered_requirements?: number;
  implementation_coverage_percentage?: number;
  mark?: string;
}

interface CoverageRequirementDetails {
  identifier: string;
  name: string;
  verified_by?: string[];
}

interface CoverageVerificationDetails {
  identifier: string;
  name: string;
  verification_type?: string;
  satisfied_by?: string[];
}

interface CoveredRequirementDetails {
  identifier: string;
  name: string;
  coverage_source?: string;
  evidence?: string[];
}

type CoverageFileItem<T> = T & { file: string };
type CoverageSectionId =
  | "overview"
  | "capability-coverage"
  | "unverified-requirements"
  | "unimplemented-requirements"
  | "unsatisfied-verifications"
  | "orphaned-verifications";

export function CoverageView({
  onOpenElement,
}: {
  onOpenElement?: (id: string) => void;
} & Partial<ExplorerViewProps> = {}) {
  const { store, elementById } = useStore();
  const coverage = (store.coverage ?? {}) as CoverageProjectionLike;
  const summary = coverage.summary ?? {};
  const capabilityRows = [...(coverage.capability_coverage?.capabilities ?? [])].sort(
    (left, right) =>
      (right.implementation_coverage_percentage ?? 0) -
        (left.implementation_coverage_percentage ?? 0) ||
      left.name.localeCompare(right.name),
  );
  const unverifiedLeaf = coverageFileItems<CoverageRequirementDetails>(coverage.unverified_leaf_requirements);
  const uncoveredRequirements = coverageFileItems<CoverageRequirementDetails>(coverage.uncovered_requirements);
  const unsatisfiedTests = coverageFileItems<CoverageVerificationDetails>(coverage.unsatisfied_test_verifications);
  const orphanedVerifications = coverageFileItems<CoverageVerificationDetails>(coverage.orphaned_verifications);
  const coveredRequirements = coverageFileItems<CoveredRequirementDetails>(coverage.covered_requirements);
  const satisfiedTests = coverageFileItems<CoverageVerificationDetails>(coverage.satisfied_test_verifications);
  const hasCoverageData =
    Object.keys(summary).length > 0 ||
    capabilityRows.length > 0 ||
    unverifiedLeaf.length > 0 ||
    uncoveredRequirements.length > 0 ||
    unsatisfiedTests.length > 0 ||
    orphanedVerifications.length > 0 ||
    coveredRequirements.length > 0 ||
    satisfiedTests.length > 0;

  useEffect(() => {
    function navigateToCoverageSection(event: Event) {
      const section = (event as CustomEvent<{ section?: CoverageSectionId }>).detail?.section;
      if (!section) return;
      const target = document.getElementById(coverageSectionDomId(section));
      if (!target) return;
      target.scrollIntoView({ block: "start", behavior: "smooth" });
    }

    window.addEventListener("reqvire:coverage-navigate", navigateToCoverageSection);
    return () => window.removeEventListener("reqvire:coverage-navigate", navigateToCoverageSection);
  }, []);

  return (
    <ViewFrame testId="coverage">
      <div className={cx("ex-route", "ex-route-single", reportRouteBaseUX, reportRouteSkinX)}>
        <div
          className={cx(
            "ex-document-panel",
            "coverage-dashboard",
            coverageDashboardBaseUX,
            coverageDashboardSkinX,
          )}
        >
          <header
            id={coverageSectionDomId("overview")}
            className="coverage-header"
          >
            <div className="coverage-title-block">
              <span className="ex-coverage-eyebrow">Coverage</span>
              <h1>Verification Coverage</h1>
            </div>
            <StatRow className="coverage-header-stats">
              <Stat label="Requirements" value={formatNumber(summary.total_requirements_in_scope)} />
              <Stat label="Leaf reqs" value={formatNumber(summary.total_leaf_requirements)} />
              <Stat label="Verifications" value={formatNumber(summary.total_verifications)} />
            </StatRow>
          </header>

          {!hasCoverageData ? (
            <div className="coverage-empty">
              <Icon name="pie-chart" />
              <div>
                <h2>No coverage report in this Explorer seed</h2>
                <p>Serve or open a Project Store generated by Reqvire to inspect requirement and verification coverage.</p>
              </div>
            </div>
          ) : (
            <>
              <section className="coverage-kpi-grid" aria-label="Coverage summary">
                <CoverageKpi
                  label="Leaf verification"
                  value={summary.leaf_requirements_coverage_percentage}
                  detail={`${formatNumber(summary.verified_leaf_requirements)} / ${formatNumber(summary.total_leaf_requirements)} verified`}
                  token="--requirement"
                />
                <CoverageKpi
                  label="Implementation"
                  value={summary.implementation_coverage_percentage}
                  detail={`${formatNumber(summary.covered_requirements)} / ${formatNumber(summary.total_requirements_in_scope)} covered`}
                  token="--resource"
                />
                <CoverageKpi
                  label="Test evidence"
                  value={summary.test_verifications_satisfaction_percentage}
                  detail={`${formatNumber(summary.satisfied_test_verifications)} / ${formatNumber(summary.total_test_verifications)} satisfied`}
                  token="--verification"
                />
                <CoverageKpi
                  label="Orphaned verifications"
                  value={summary.orphaned_verifications_percentage}
                  detail={`${formatNumber(summary.orphaned_verifications)} / ${formatNumber(summary.total_verifications)} orphaned`}
                  token="--refinement"
                  inverted
                />
              </section>

              <section className="coverage-grid" aria-label="Coverage breakdown">
                <CoveragePanel title="Verification types" className="coverage-panel--compact">
                  <CoverageBreakdown
                    values={summary.verification_types ?? {}}
                    rows={[
                      ["test", "Test", "--verification"],
                      ["formal_proof", "Formal proof", "--verification"],
                      ["analysis", "Analysis", "--capability"],
                      ["inspection", "Inspection", "--ontology"],
                      ["demonstration", "Demonstration", "--refinement"],
                    ]}
                  />
                </CoveragePanel>
                <CoveragePanel title="Coverage sources" className="coverage-panel--compact">
                  <CoverageSourceBars values={summary.coverage_sources ?? {}} />
                </CoveragePanel>
                <CoveragePanel
                  id={coverageSectionDomId("capability-coverage")}
                  title="Capability coverage"
                  className="coverage-panel--wide"
                >
                  <CapabilityCoverageList capabilities={capabilityRows} onOpenElement={onOpenElement} />
                </CoveragePanel>
              </section>

              <section className="coverage-gap-grid" aria-label="Coverage gaps">
                <CoverageGapList
                  id={coverageSectionDomId("unverified-requirements")}
                  title="Unverified requirements"
                  items={unverifiedLeaf}
                  emptyLabel="All leaf requirements have verification."
                  defaultType="requirement"
                  elementById={elementById}
                  onOpenElement={onOpenElement}
                />
                <CoverageGapList
                  id={coverageSectionDomId("unimplemented-requirements")}
                  title="Unimplemented requirements"
                  items={uncoveredRequirements}
                  emptyLabel="All requirements in scope have implementation evidence."
                  defaultType="requirement"
                  elementById={elementById}
                  onOpenElement={onOpenElement}
                />
                <CoverageGapList
                  id={coverageSectionDomId("unsatisfied-verifications")}
                  title="Unsatisfied verifications"
                  items={unsatisfiedTests}
                  emptyLabel="All test verifications have evidence."
                  defaultType="test-verification"
                  elementById={elementById}
                  onOpenElement={onOpenElement}
                />
                <CoverageGapList
                  id={coverageSectionDomId("orphaned-verifications")}
                  title="Orphaned verifications"
                  items={orphanedVerifications}
                  emptyLabel="Every verification links to a requirement or capability."
                  defaultType="test-verification"
                  elementById={elementById}
                  onOpenElement={onOpenElement}
                />
              </section>
            </>
          )}
        </div>
      </div>
    </ViewFrame>
  );
}

function CoverageKpi({
  label,
  value,
  detail,
  token,
  inverted = false,
}: {
  label: string;
  value?: number;
  detail: string;
  token: `--${string}`;
  inverted?: boolean;
}) {
  const percent = clampPercent(value ?? 0);
  const shown = typeof value === "number" ? formatPercent(value) : "—";
  const ringPercent = inverted ? 100 - percent : percent;
  return (
    <div className="coverage-kpi">
      <div
        className="coverage-donut"
        style={{
          "--coverage-fill-angle": `${ringPercent * 3.6}deg`,
          "--coverage-color": `var(${token})`,
        } as CSSProperties}
        aria-hidden="true"
      >
        <span className="coverage-donut__center">{shown}</span>
      </div>
      <div className="coverage-kpi__copy">
        <span className="coverage-kpi__label">{label}</span>
        <span className="coverage-kpi__detail">{detail}</span>
      </div>
    </div>
  );
}

function CoveragePanel({
  id,
  title,
  className = "",
  children,
}: {
  id?: string;
  title: string;
  className?: string;
  children: ReactNode;
}) {
  return (
    <section
      id={id}
      className={cx("coverage-panel", className)}
    >
      <header className="coverage-panel__head">
        <h2>{title}</h2>
      </header>
      {children}
    </section>
  );
}

function CoverageBreakdown({
  values,
  rows,
}: {
  values: Record<string, number>;
  rows: [string, string, `--${string}`][];
}) {
  const total = rows.reduce((sum, [key]) => sum + (values[key] ?? 0), 0);
  return (
    <div className="coverage-breakdown">
      <div
        className="coverage-breakdown__pie"
        style={{ background: buildConicGradient(rows, values, total) }}
        aria-hidden="true"
      />
      <div className="coverage-breakdown__legend">
        {rows.map(([key, label, token]) => (
          <CoverageLegendRow key={key} label={label} value={values[key] ?? 0} token={token} />
        ))}
      </div>
    </div>
  );
}

function CoverageSourceBars({ values }: { values: Record<string, number> }) {
  const rows: [string, string, `--${string}`][] = [
    ["direct_satisfied", "Direct evidence", "--resource"],
    ["refinement_contract_satisfied_via_attachment", "Attached contract", "--ontology"],
    ["refinement_contract_satisfied_via_child", "Child contract", "--capability"],
  ];
  const max = Math.max(1, ...rows.map(([key]) => values[key] ?? 0));
  return (
    <div className="coverage-bar-list">
      {rows.map(([key, label, token]) => {
        const value = values[key] ?? 0;
        return (
          <div key={key} className="coverage-source-row">
            <div className="coverage-source-row__head">
              <span>{label}</span>
              <span>{formatNumber(value)}</span>
            </div>
            <CoverageBar value={(value / max) * 100} token={token} />
          </div>
        );
      })}
    </div>
  );
}

function CapabilityCoverageList({
  capabilities,
  onOpenElement,
}: {
  capabilities: CapabilityCoverageDetails[];
  onOpenElement?: (id: string) => void;
}) {
  if (capabilities.length === 0) {
    return <p className="coverage-empty-note">No capability coverage rows were reported.</p>;
  }

  return (
    <div className="coverage-capability-list">
      {capabilities.map((capability) => (
        <button
          key={capability.identifier}
          type="button"
          className="coverage-capability-row"
          onClick={() => onOpenElement?.(capability.identifier)}
        >
          <div className="coverage-capability-row__title">
            <ElementIcon type="capability" size="sm" />
            <span>{capability.name || displayIdentifier(capability.identifier)}</span>
            {capability.mark ? <span className="coverage-mark">{capability.mark}</span> : null}
          </div>
          <div className="coverage-capability-row__bars">
            <LabeledCoverageBar
              label="Verification"
              value={capability.verification_coverage_percentage}
              count={`${formatNumber(capability.aggregate_verified_leaf_requirements)} / ${formatNumber(capability.aggregate_leaf_requirements)}`}
              token="--requirement"
            />
            <LabeledCoverageBar
              label="Implementation"
              value={capability.implementation_coverage_percentage}
              count={`${formatNumber(capability.aggregate_covered_requirements)} / ${formatNumber(capability.aggregate_requirements)}`}
              token="--resource"
            />
          </div>
        </button>
      ))}
    </div>
  );
}

function CoverageGapList<T extends { identifier: string; name: string; file: string }>({
  id,
  title,
  items,
  emptyLabel,
  defaultType,
  elementById,
  onOpenElement,
}: {
  id?: string;
  title: string;
  items: T[];
  emptyLabel: string;
  defaultType: string;
  elementById: (id: string) => ProjectStoreElement | undefined;
  onOpenElement?: (id: string) => void;
}) {
  const [expanded, setExpanded] = useState(false);
  const visibleLimit = 8;
  const visible = expanded ? items : items.slice(0, visibleLimit);
  const hiddenCount = Math.max(0, items.length - visible.length);
  return (
    <section
      id={id}
      className="coverage-panel coverage-gap-list"
    >
      <header className="coverage-gap-list__head">
        <h3>{title}</h3>
        <span>{formatNumber(items.length)}</span>
      </header>
      {items.length === 0 ? (
        <p className="coverage-empty-note">{emptyLabel}</p>
      ) : (
        <div className="coverage-gap-list__rows">
          {visible.map((item) => {
            const element = elementById(item.identifier);
            const type = element?.element_type ?? defaultType;
            const family = element?.type_family ?? defaultType;
            return (
              <button
                key={`${item.file}:${item.identifier}`}
                type="button"
                className="coverage-gap-row"
                onClick={() => onOpenElement?.(item.identifier)}
              >
                <ElementIcon type={type} family={family} size="sm" />
                <span className="coverage-gap-row__copy">
                  <span className="coverage-gap-row__title">{item.name || displayIdentifier(item.identifier)}</span>
                  <span className="coverage-gap-row__file">{item.file}</span>
                </span>
                <TypeBadge type={type} family={family} tinted>
                  {humanizeType(type)}
                </TypeBadge>
              </button>
            );
          })}
          {items.length > visibleLimit ? (
            <button
              type="button"
              className="coverage-more"
              aria-expanded={expanded}
              onClick={() => setExpanded((current) => !current)}
            >
              {expanded ? "Show fewer" : `+ ${formatNumber(hiddenCount)} more`}
            </button>
          ) : null}
        </div>
      )}
    </section>
  );
}

function coverageSectionDomId(section: CoverageSectionId) {
  return `coverage-section-${section}`;
}

function LabeledCoverageBar({
  label,
  value,
  count,
  token,
}: {
  label: string;
  value?: number;
  count: string;
  token: `--${string}`;
}) {
  return (
    <div className="coverage-labeled-bar">
      <div className="coverage-labeled-bar__head">
        <span>{label}</span>
        <span>
          {formatPercent(value)} · {count}
        </span>
      </div>
      <CoverageBar value={value ?? 0} token={token} />
    </div>
  );
}

function CoverageBar({ value, token }: { value: number; token: `--${string}` }) {
  return (
    <span className="coverage-bar" aria-hidden="true">
      <span
        className="coverage-bar__fill"
        style={{
          "--coverage-bar-fill": `${clampPercent(value)}%`,
          "--coverage-color": `var(${token})`,
        } as CSSProperties}
      />
    </span>
  );
}

function CoverageLegendRow({
  label,
  value,
  token,
}: {
  label: string;
  value: number;
  token: `--${string}`;
}) {
  return (
    <div className="coverage-legend-row">
      <span className="coverage-legend-row__swatch" style={{ background: `var(${token})` }} />
      <span>{label}</span>
      <strong>{formatNumber(value)}</strong>
    </div>
  );
}

function coverageFileItems<T>(section: unknown): Array<CoverageFileItem<T>> {
  if (!isRecord(section) || !isRecord(section.files)) return [];
  const rows: Array<CoverageFileItem<T>> = [];
  for (const [file, value] of Object.entries(section.files)) {
    if (!Array.isArray(value)) continue;
    for (const item of value) {
      if (isRecord(item)) {
        rows.push({ file, ...(item as T) });
      }
    }
  }
  return rows.sort((left, right) => {
    const leftName = String((left as { name?: unknown }).name ?? "");
    const rightName = String((right as { name?: unknown }).name ?? "");
    return left.file.localeCompare(right.file) || leftName.localeCompare(rightName);
  });
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function buildConicGradient(
  rows: [string, string, `--${string}`][],
  values: Record<string, number>,
  total: number,
) {
  if (total <= 0) return "var(--bg-sunken)";
  let cursor = 0;
  const segments = rows
    .map(([key, , token]) => {
      const size = ((values[key] ?? 0) / total) * 360;
      const start = cursor;
      const end = cursor + size;
      cursor = end;
      return `var(${token}) ${start}deg ${end}deg`;
    })
    .filter((segment) => !segment.includes(" 0deg 0deg"));
  return `conic-gradient(${segments.join(", ")})`;
}

function clampPercent(value: number) {
  if (!Number.isFinite(value)) return 0;
  return Math.max(0, Math.min(100, value));
}

function formatPercent(value: number | undefined) {
  if (typeof value !== "number" || !Number.isFinite(value)) return "—";
  return `${roundOne(value)}%`;
}

function roundOne(value: number) {
  return Number.isInteger(value) ? value.toString() : value.toFixed(1).replace(/\.0$/, "");
}

function formatNumber(value: number | undefined) {
  return typeof value === "number" && Number.isFinite(value) ? value.toLocaleString() : "0";
}

function displayIdentifier(identifier: string) {
  const fragment = identifier.split("#").pop();
  return fragment ? fragment.replace(/-/g, " ") : identifier;
}

function humanizeType(value: string) {
  return value.replace(/-/g, " ");
}
