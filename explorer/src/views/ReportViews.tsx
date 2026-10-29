import { useMemo } from "preact/hooks";
import { useStore } from "../store/StoreContext";
import type { ExplorerViewProps } from "./types/ExplorerViewProps";
import { useExplorerUiState } from "../state/ExplorerUiState";
import { routeForContent } from "../router/routes";
import { writeExplorerHash } from "../router/location";
import { buildVerificationFlow } from "../lib/traceFlow";
import { flowLayoutEngine } from "../workers/flowLayoutEngine";
import { CoverageReportScreen, TracesScreen } from "@ds";
import { coverageDrilldownItems } from "../lib/coverage";

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

  return <TracesScreen trace={trace} traceFiles={traceFiles} selectedFile={selectedFile}
    contextKey={JSON.stringify([store.project.workspace_root, store.project.worktree_id, trace?.verification.id])}
    layoutEngine={flowLayoutEngine} onOpenElement={onOpenElement} onSelect={setTraceSelectionId}
    onOpenSource={element => writeExplorerHash(element.sourceHref ?? routeForContent(element.file))} />;
}

export function CoverageView({ onOpenElement }: {
  onOpenElement?: (id: string) => void;
} & Partial<ExplorerViewProps> = {}) {
  const { store, elementById } = useStore();
  const ui = useExplorerUiState();
  const coverage = ui.coverageProjection;
  const capabilityRows = useMemo(() => coverageDrilldownItems(store, coverage), [store, coverage]);
  return <CoverageReportScreen coverage={coverage} capabilityRows={capabilityRows}
    scopeId={ui.coverageScopeId} contextKey={JSON.stringify([store.project.worktree_id, ui.coverageScopeId])}
    notice={ui.coverageNotice} elementById={elementById} onOpenElement={onOpenElement}
    onInspect={target => {
      if (target.kind === "element") onOpenElement?.(target.id);
      else if (target.href) window.location.hash = target.href;
    }} />;
}
