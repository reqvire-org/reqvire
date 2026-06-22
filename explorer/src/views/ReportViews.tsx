import {
  memo,
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
  type MouseEvent as ReactMouseEvent,
} from "react";
import { useStore } from "../store/StoreContext";
import type { ExplorerViewProps } from "./types/ExplorerViewProps";
import { useExplorerUiState } from "../state/ExplorerUiState";
import type {
  ProjectStoreElement,
  TraceRequirementNode,
} from "../store/types";
import { ViewFrame } from "./ViewFrame";
import { MermaidBlock } from "../rendering/MarkdownContent";
import {
  CoverageBarFrame,
  CoverageBarList,
  CoverageBreakdownFrame,
  CoverageCapabilityList,
  CoverageCapabilityRow,
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
  LabeledCoverageBarFrame,
  ReportEmptyNote,
  ReportRouteLayout,
  TraceFileGroup,
  TraceFileHeader,
  TraceReportContent,
  TraceReportPanel,
  TraceRollupDiagramShell,
  TraceRollupPlaceholder,
  TraceRowsFrame,
  TraceTreeCountBadge,
  TraceVerificationCard,
  TraceVerificationHeader,
  TraceVerificationList,
  TraceVerificationMeta,
  TraceVerificationTitleButton,
  elementRole,
  getMermaidClassDefs,
  type DesignSystemColorToken,
  type ElementRole,
} from "@ds";
import { buildTraceFiles, type TraceFileNode, type TraceVerificationNode } from "../lib/traces";

/*
 * Report-projection views (Traces and Coverage).
 *
 * Each view renders natively from its Project Store report projection — no
 * iframe-mounted standalone page content. These views
 * surface store-backed report data and route element rows to the in-shell
 * element-detail modal.
 */

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
      <ReportRouteLayout>
        <TraceReportPanel>
          <TraceReportContent>
            <TraceRows
              file={selectedFile}
              elementById={elementById}
              onOpenElement={onOpenElement}
              onSelect={setSelectedId}
              selectedVerificationId={traceSelectionId}
            />
            {traceFiles.length === 0 && (
              <ReportEmptyNote>No verification traces in store.</ReportEmptyNote>
            )}
          </TraceReportContent>
        </TraceReportPanel>
      </ReportRouteLayout>
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
    return <TraceRowsFrame data-testid="trace-rows" />;
  }

  return (
    <TraceRowsFrame ref={containerRef} data-testid="trace-rows">
      <TraceFileGroup>
        <TraceFileHeader
          file={file.file}
          countLabel={`${file.verifications.length} ${file.verifications.length === 1 ? "verification" : "verifications"}`}
        />
        <TraceVerificationList>
          {file.verifications.map((verification) => (
            <TraceVerificationCard
              key={verification.id}
              id={traceVerificationDomId(verification.id)}
              selected={selectedVerificationId === verification.id}
            >
              <TraceVerificationHeader>
                <TraceVerificationTitleButton
                  onClick={() => {
                    onSelect(verification.id);
                  }}
                >
                  {verification.name}
                </TraceVerificationTitleButton>
                <TraceTreeCountBadge>
                  {verification.totalCount} in tree
                </TraceTreeCountBadge>
              </TraceVerificationHeader>
              <TraceVerificationMeta
                rows={[
                  { label: "Type", value: verification.verificationType ?? "verification" },
                  { label: "Directly Verified", value: `${verification.directCount} requirements` },
                  { label: "Total in Tree", value: `${verification.totalCount} requirements` },
                ]}
              />
              <TraceRollupDiagram
                verification={verification}
                elementById={elementById}
                onOpenElement={onOpenElement}
              />
            </TraceVerificationCard>
          ))}
        </TraceVerificationList>
      </TraceFileGroup>
    </TraceRowsFrame>
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
    <TraceRollupDiagramShell ref={containerRef} onClickCapture={handleDiagramClick}>
      {shouldRender && model ? (
        <MermaidBlock
          code={model.code}
          nodeClickTargets={model.nodeClickTargets}
          onNodeClick={onOpenElement}
          onRenderSettled={releaseRenderSlot}
        />
      ) : (
        <TraceRollupPlaceholder>
          Diagram queued. Rows remain interactive while rendering continues.
        </TraceRollupPlaceholder>
      )}
    </TraceRollupDiagramShell>
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
    case "verification-objective":
      return "verificationObjective";
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
      <ReportRouteLayout>
        <CoverageDashboard>
          <CoverageHeader
            id={coverageSectionDomId("overview")}
            eyebrow="Coverage"
            title="Verification Coverage"
          />

          {!hasCoverageData ? (
            <CoverageEmptyState title="No coverage report in this Explorer seed">
              Serve or open a Project Store generated by Reqvire to inspect requirement and verification coverage.
            </CoverageEmptyState>
          ) : (
            <>
              <CoverageKpiGrid>
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
                  token="--contract"
                  inverted
                />
              </CoverageKpiGrid>

              <CoverageGrid>
                <CoveragePanel title="Verification types">
                  <CoverageBreakdown
                    values={summary.verification_types ?? {}}
                    rows={[
                      ["test", "Test", "test-verification"],
                      ["formal_proof", "Formal proof", "formal-proof-verification"],
                      ["analysis", "Analysis", "analysis-verification"],
                      ["inspection", "Inspection", "inspection-verification"],
                      ["demonstration", "Demonstration", "demonstration-verification"],
                    ]}
                  />
                </CoveragePanel>
                <CoveragePanel title="Coverage sources">
                  <CoverageSourceBars values={summary.coverage_sources ?? {}} />
                </CoveragePanel>
                <CoveragePanel
                  id={coverageSectionDomId("capability-coverage")}
                  title="Capability coverage"
                  span="wide"
                >
                  <CapabilityCoverageList capabilities={capabilityRows} onOpenElement={onOpenElement} />
                </CoveragePanel>
              </CoverageGrid>

              <CoverageGapGrid>
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
              </CoverageGapGrid>
            </>
          )}
        </CoverageDashboard>
      </ReportRouteLayout>
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
  return (
    <CoverageBreakdownFrame>
      {rows.map(([key, label, type]) => (
        <CoverageLegendRow key={key} label={label} value={formatNumber(values[key] ?? 0)} type={type} />
      ))}
    </CoverageBreakdownFrame>
  );
}

function CoverageSourceBars({ values }: { values: Record<string, number> }) {
  const rows: [string, string, DesignSystemColorToken][] = [
    ["direct_satisfied", "Direct evidence", "--resource"],
    ["contract_satisfied_via_reused_contract_context", "Reused contract", "--ontology"],
    ["contract_satisfied_via_child", "Child contract", "--capability"],
  ];
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

function CapabilityCoverageList({
  capabilities,
  onOpenElement,
}: {
  capabilities: CapabilityCoverageDetails[];
  onOpenElement?: (id: string) => void;
}) {
  if (capabilities.length === 0) {
    return <CoverageEmptyNote>No capability coverage rows were reported.</CoverageEmptyNote>;
  }

  return (
    <CoverageCapabilityList>
      {capabilities.map((capability) => (
        <CoverageCapabilityRow
          key={capability.identifier}
          name={capability.name || displayIdentifier(capability.identifier)}
          mark={capability.mark}
          onClick={() => onOpenElement?.(capability.identifier)}
        >
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
        </CoverageCapabilityRow>
      ))}
    </CoverageCapabilityList>
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
  token: DesignSystemColorToken;
}) {
  return (
    <LabeledCoverageBarFrame label={label} value={`${formatPercent(value)} · ${count}`}>
      <CoverageBar value={value ?? 0} token={token} />
    </LabeledCoverageBarFrame>
  );
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
