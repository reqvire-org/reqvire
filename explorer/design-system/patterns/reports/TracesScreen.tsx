import { RouteFrame } from "../shell/RouteFrame";
import { TraceFlow, type TraceFlowProps } from "./TraceFlow";
import {
  TraceReportPanel, TraceReportContent, TraceRowsFrame, TraceFileGroup, TraceFileHeader,
  TraceVerificationList, TraceVerificationCard, TraceVerificationHeader,
  TraceVerificationTitleButton, TraceTreeCountBadge, TraceVerificationMeta, ReportEmptyNote,
} from "./ReportPatterns";

export interface TraceOverviewFile {
  file: string;
  verifications: readonly {
    id: string; name: string; verificationType?: string;
    directCount: number; totalCount: number;
  }[];
}
export interface TracesScreenProps extends Omit<TraceFlowProps, "trace"> {
  trace: TraceFlowProps["trace"] | null;
  traceFiles: readonly TraceOverviewFile[];
  selectedFile?: TraceOverviewFile;
  onSelect: (id: string) => void;
  contextKey?: string;
}

/** The same grouped overview and selected verification flow used by the application. */
export function TracesScreen({ trace, traceFiles, selectedFile, layoutEngine,
  onOpenElement, onOpenSource, onSelect, contextKey }: TracesScreenProps) {
  return <RouteFrame viewId="traces">
    {trace ? <TraceFlow
      key={contextKey ?? trace.verification.id}
      trace={trace} layoutEngine={layoutEngine} onOpenElement={onOpenElement}
      onOpenSource={onOpenSource}
    /> : <TraceReportPanel><TraceReportContent>
      <TraceFileOverview file={selectedFile} onSelect={onSelect} />
      {traceFiles.length === 0 && <ReportEmptyNote>No verification traces in store.</ReportEmptyNote>}
    </TraceReportContent></TraceReportPanel>}
  </RouteFrame>;
}

function TraceFileOverview({ file, onSelect }: {
  file: TraceOverviewFile | undefined;
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
