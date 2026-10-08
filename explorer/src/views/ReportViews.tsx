import {
  useMemo,
  useState,
} from "react";
import { useStore } from "../store/StoreContext";
import type { ExplorerViewProps } from "./types/ExplorerViewProps";
import { useExplorerUiState } from "../state/ExplorerUiState";
import type {
  ProjectStoreElement,
} from "../store/types";
import { ViewFrame } from "./ViewFrame";
import { routeForContent } from "../router/routes";
import { writeExplorerHash } from "../router/location";
import { buildVerificationFlow } from "../lib/traceFlow";
import { flowLayoutEngine } from "../workers/flowLayoutEngine";
import {
  CoverageDrilldown,
  CoverageBarFrame,
  CoverageBarList,
  CoverageBreakdownFrame,
  SegmentedMeter,
  CoverageDashboard,
  CoverageEmptyNote,
  CoverageEmptyState,
  CoverageGapGrid,
  CoverageGapListFrame,
  CoverageGapRowButton,
  CoverageGapRows,
  CoverageGrid,
  CoverageHeader,
  CoverageKpiCard,
  CoverageKpiGrid,
  CoverageLegendRow,
  CoverageMoreButton,
  CoveragePanel,
  CoverageSourceRow,
  ReportEmptyNote,
  ReportRouteLayout,
  TraceFileGroup,
  TraceFlow,
  TraceFileHeader,
  TraceReportContent,
  TraceReportPanel,
  TraceRowsFrame,
  TraceTreeCountBadge,
  TraceVerificationCard,
  TraceVerificationHeader,
  TraceVerificationList,
  TraceVerificationMeta,
  TraceVerificationTitleButton,
  type DesignSystemColorToken,
} from "@ds";
import { COVERAGE_ISSUES } from "../state/useCoverageNavigation";
import { coverageDrilldownItems, COVERAGE_SOURCE_LABELS } from "../lib/coverage";
import { type TraceFileNode } from "../lib/traces";

/*
 * Report-projection views (Traces and Coverage).
 *
 * Each view renders natively from its Project Store report projection — no
 * iframe-mounted standalone page content. These views
 * surface store-backed report data and route element rows to the in-shell
 * element-detail modal.
 */

export function TracesView({ onOpenElement }: {
  onOpenElement: (id: string) => void;
} & Partial<ExplorerViewProps>) {
  const { store, elementIndex, getTraceFiles } = useStore();
  const { traceFilePath, traceSelectionId, setTraceSelectionId } = useExplorerUiState();
  const traceFiles = getTraceFiles();
  const selectedFile = traceFiles.find(file => file.file === traceFilePath) ?? traceFiles[0];
  const selectedVerification = selectedFile?.verifications.find(item => item.id === traceSelectionId);
  // Only the active verification needs flow preparation or a layout worker.
  const trace = useMemo(() => selectedVerification
    ? buildVerificationFlow(selectedVerification, elementIndex) : null, [selectedVerification, elementIndex]);

  return <ViewFrame testId="traces">
    {trace ? <TraceFlow
      key={JSON.stringify([store.project.workspace_root, store.project.worktree_id, trace.verification.id])}
      trace={trace} layoutEngine={flowLayoutEngine} onOpenElement={onOpenElement}
      onOpenSource={element => writeExplorerHash(element.sourceHref ?? routeForContent(element.file))}
    /> : <TraceReportPanel><TraceReportContent>
      <TraceFileOverview file={selectedFile} onSelect={setTraceSelectionId} />
      {traceFiles.length === 0 && <ReportEmptyNote>No verification traces in store.</ReportEmptyNote>}
    </TraceReportContent></TraceReportPanel>}
  </ViewFrame>;
}

function TraceFileOverview({ file, onSelect }: {
  file: TraceFileNode | undefined;
  onSelect: (id: string) => void;
}) {
  return <TraceRowsFrame data-testid="trace-rows">
    {file && <TraceFileGroup>
      <TraceFileHeader file={file.file}
        countLabel={`${file.verifications.length} ${file.verifications.length === 1 ? "verification" : "verifications"}`} />
      <TraceVerificationList>{file.verifications.map(verification => <TraceVerificationCard key={verification.id}>
        <TraceVerificationHeader>
          <TraceVerificationTitleButton onClick={() => onSelect(verification.id)}>{verification.name}</TraceVerificationTitleButton>
          <TraceTreeCountBadge>{verification.totalCount} in tree</TraceTreeCountBadge>
        </TraceVerificationHeader>
        <TraceVerificationMeta rows={[
          { label: "Type", value: verification.verificationType ?? "verification" },
          { label: "Directly Verified", value: `${verification.directCount} requirements` },
          { label: "Total in Tree", value: `${verification.totalCount} requirements` },
        ]} />
      </TraceVerificationCard>)}</TraceVerificationList>
    </TraceFileGroup>}
  </TraceRowsFrame>;
}

interface CoverageRequirementDetails {
  identifier: string;
  name: string;
  verified_by?: string[];
}

type CoverageFileItem<T> = T & { file: string };

export function CoverageView({ onOpenElement }: {
  onOpenElement?: (id: string) => void;
} & Partial<ExplorerViewProps> = {}) {
  const { store, elementById } = useStore();
  const ui = useExplorerUiState();
  const coverage = ui.coverageProjection;
  const summary = coverage.summary ?? {};
  const capabilityRows = useMemo(() => coverageDrilldownItems(store, coverage), [store, coverage]);
  const scoped = Boolean(coverage.scope);
  const hasCoverageData = Object.keys(summary).length > 0;

  return <ViewFrame testId="coverage">
    <ReportRouteLayout><CoverageDashboard>
      <CoverageHeader eyebrow="Coverage" title={coverage.scope?.capability_name ?? "Whole Model"} />
      {ui.coverageNotice && <CoverageEmptyNote>{ui.coverageNotice}</CoverageEmptyNote>}
      {!hasCoverageData ? <CoverageEmptyState title="No coverage report in this Explorer seed">
        Serve or open a Project Store generated by Reqvire to inspect requirement and verification coverage.
      </CoverageEmptyState> : <>
        {scoped && summary.total_requirements_in_scope === 0
          && <CoverageEmptyNote>No requirements in this capability scope.</CoverageEmptyNote>}
        <CoverageKpiGrid>
          <CoverageKpi label="Leaf verification" value={summary.leaf_requirements_coverage_percentage}
            detail={`${formatNumber(summary.verified_leaf_requirements)} / ${formatNumber(summary.total_leaf_requirements)} verified`} token="--requirement" />
          <CoverageKpi label="Implementation" value={summary.implementation_coverage_percentage}
            detail={summary.total_terminal_requirements === undefined
              ? `${formatNumber(summary.covered_requirements)} / ${formatNumber(summary.total_requirements_in_scope)} covered`
              : `${formatNumber(summary.covered_terminal_requirements)} / ${formatNumber(summary.total_terminal_requirements)} terminal requirements covered`} token="--resource" />
          <CoverageKpi label="Test evidence" value={summary.test_verifications_satisfaction_percentage}
            detail={`${formatNumber(summary.satisfied_test_verifications)} / ${formatNumber(summary.total_test_verifications)} satisfied`} token="--verification" />
          {!scoped && <CoverageKpi label="Orphaned verifications" value={summary.orphaned_verifications_percentage}
            detail={`${formatNumber(summary.orphaned_verifications)} / ${formatNumber(summary.total_verifications)} orphaned`} token="--contract" inverted />}
        </CoverageKpiGrid>
        <CoverageGrid compact>
          <CoveragePanel title="Verification types"><CoverageBreakdown values={summary.verification_types ?? {}} rows={[
            ["test", "Test", "test-verification"], ["formal_proof", "Formal proof", "formal-proof-verification"],
            ["analysis", "Analysis", "analysis-verification"], ["inspection", "Inspection", "inspection-verification"],
            ["demonstration", "Demonstration", "demonstration-verification"],
          ]} /></CoveragePanel>
          <CoveragePanel title="Implementation sources"><CoverageSourceBars values={summary.coverage_sources ?? {}} /></CoveragePanel>
        </CoverageGrid>
        <CoveragePanel id={coverageSectionDomId("capability-coverage")}
          title={capabilityRows.length ? undefined : "Capability coverage"} span="wide">
          {capabilityRows.length ? <CoverageDrilldown key={ui.coverageScopeId ?? "whole"} items={capabilityRows}
            onInspect={target => {
              if (target.kind === "element") onOpenElement?.(target.id);
              else if (target.href) window.location.hash = target.href;
            }} /> : <CoverageEmptyNote>No capability coverage rows were reported.</CoverageEmptyNote>}
        </CoveragePanel>
        <CoverageGapGrid key={JSON.stringify([store.project.worktree_id, ui.coverageScopeId])}>
          {COVERAGE_ISSUES.filter(item => !scoped || item.id !== "orphaned-verifications").map(item => <CoverageGapList
            key={item.id} id={coverageSectionDomId(item.id)} title={item.label}
            items={coverageFileItems<CoverageRequirementDetails>(coverage[item.field])} emptyLabel={item.emptyLabel}
            defaultType={item.id.includes("verifications") ? "test-verification" : "requirement"}
            elementById={elementById} onOpenElement={onOpenElement} />)}
        </CoverageGapGrid>
        {scoped && <CoverageEmptyNote>Orphaned verifications are whole-model diagnostics. Select Whole Model in the capability tree to see them.</CoverageEmptyNote>}
      </>}
    </CoverageDashboard></ReportRouteLayout>
  </ViewFrame>;
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
  token: DesignSystemColorToken;
  inverted?: boolean;
}) {
  const percent = clampPercent(value ?? 0);
  const shown = typeof value === "number" ? formatPercent(value) : "—";
  const ringPercent = inverted ? 100 - percent : percent;
  return (
    <CoverageKpiCard
      label={label}
      detail={detail}
      shown={shown}
      ringPercent={ringPercent}
      token={token}
    />
  );
}

function CoverageBreakdown({
  values,
  rows,
}: {
  values: Record<string, number>;
  rows: [string, string, string][];
}) {
  const tokens: readonly DesignSystemColorToken[] = ["--verification-type-test", "--verification-type-formal-proof",
    "--verification-type-analysis", "--verification-type-inspection", "--verification-type-demonstration"];
  return (
    <CoverageBreakdownFrame bar={<SegmentedMeter aria-hidden="true" segments={rows.map(([key, label], index) => ({
      label, value: values[key] ?? 0, colorToken: tokens[index],
    }))} />}>
      {rows.map(([key, label, type], index) => (
        <CoverageLegendRow key={key} label={label} value={formatNumber(values[key] ?? 0)} type={type} colorToken={tokens[index]} />
      ))}
    </CoverageBreakdownFrame>
  );
}

function CoverageSourceBars({ values }: { values: Record<string, number> }) {
  const rows: [string, string, DesignSystemColorToken][] = [
    ["direct_satisfied", COVERAGE_SOURCE_LABELS.direct_satisfied, "--resource"],
    ["requirement_rollup", COVERAGE_SOURCE_LABELS.requirement_rollup, "--capability"],
    ["contract_consumer_rollup", COVERAGE_SOURCE_LABELS.contract_consumer_rollup, "--ontology"],
    ["combined_rollup", COVERAGE_SOURCE_LABELS.combined_rollup, "--requirement"],
  ];
  for (const key of ["contract_satisfied_via_contract_bindings", "contract_satisfied_via_child"]) {
    if (key in values) rows.push([key, COVERAGE_SOURCE_LABELS[key], "--ontology"]);
  }
  const max = Math.max(1, ...rows.map(([key]) => values[key] ?? 0));
  return (
    <CoverageBarList>
      {rows.map(([key, label, token]) => {
        const value = values[key] ?? 0;
        return (
          <CoverageSourceRow key={key} label={label} value={formatNumber(value)}>
            <CoverageBar value={(value / max) * 100} token={token} />
          </CoverageSourceRow>
        );
      })}
    </CoverageBarList>
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
    <CoverageGapListFrame id={id} title={title} count={formatNumber(items.length)}>
      {items.length === 0 ? (
        <CoverageEmptyNote>{emptyLabel}</CoverageEmptyNote>
      ) : (
        <CoverageGapRows>
          {visible.map((item) => {
            const element = elementById(item.identifier);
            const type = element?.element_type ?? defaultType;
            const family = element?.type_family ?? defaultType;
            return (
              <CoverageGapRowButton
                key={`${item.file}:${item.identifier}`}
                type={type}
                family={family}
                title={item.name || displayIdentifier(item.identifier)}
                file={item.file}
                typeLabel={humanizeType(type)}
                onClick={() => onOpenElement?.(item.identifier)}
              />
            );
          })}
          {items.length > visibleLimit ? (
            <CoverageMoreButton
              aria-expanded={expanded}
              onClick={() => setExpanded((current) => !current)}
            >
              {expanded ? "Show fewer" : `+ ${formatNumber(hiddenCount)} more`}
            </CoverageMoreButton>
          ) : null}
        </CoverageGapRows>
      )}
    </CoverageGapListFrame>
  );
}

function coverageSectionDomId(section: string) {
  return `coverage-section-${section}`;
}

function CoverageBar({ value, token }: { value: number; token: DesignSystemColorToken }) {
  return <CoverageBarFrame value={clampPercent(value)} token={token} />;
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
