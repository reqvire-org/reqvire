import { Fragment, useState } from "react";
import {
  ElementIcon, Icon,
  PaneControlSection, PaneSearchForm, PaneSummary, PaneTree, RouteFrame,
  SidePaneFrame, TraceFileHeader, TraceFlow, TraceReportContent, TraceReportPanel,
  TraceVerificationCard, TraceVerificationHeader, TraceVerificationList, TraceVerificationMeta,
  TraceVerificationTitleButton, TreeItem,
} from "@ds";
import { TRACE_EXAMPLES } from "./fixtures/traces";

const files = [...new Set(TRACE_EXAMPLES.map(example => example.trace.verification.file))];

/** A preview harness: every visual comes from public design-system exports. */
export function useTraceFlowMock() {
  const [selectedId, setSelectedId] = useState<string | null>("classification-test");
  const [selectedFile, setSelectedFile] = useState(TRACE_EXAMPLES.find(item => item.trace.verification.id === "classification-test")!.trace.verification.file);
  const [closedFiles, setClosedFiles] = useState<Set<string>>(new Set());
  const [query, setQuery] = useState("");
  const example = TRACE_EXAMPLES.find(item => item.trace.verification.id === selectedId);
  const needle = query.trim().toLowerCase();
  const filtered = TRACE_EXAMPLES.filter(item => `${item.label} ${item.trace.verification.name} ${item.trace.verification.file}`.toLowerCase().includes(needle));
  const selectExample = (id: string) => {
    setSelectedId(id);
    setSelectedFile(TRACE_EXAMPLES.find(item => item.trace.verification.id === id)!.trace.verification.file);
  };
  return {
    sidePane: ({ open, onToggle }: { open: boolean; onToggle: () => void }) => <SidePaneFrame chrome="app" open={open} onToggle={onToggle}>
      <PaneSearchForm searchInputId="trace-preview-filter" inputLabel="Filter trace examples"
        placeholder="Filter trace examples…" value={query} onQueryChange={setQuery} onSubmit={event => event.preventDefault()} />
      <PaneControlSection title="Verification traces">
        <PaneTree aria-label="Trace files and verifications">
          {files.map(file => {
            const entries = filtered.filter(item => item.trace.verification.file === file);
            if (!entries.length) return null;
            const expanded = needle.length > 0 || !closedFiles.has(file);
            const toggle = () => setClosedFiles(previous => {
              const next = new Set(previous);
              if (next.has(file)) next.delete(file); else next.add(file);
              return next;
            });
            return <Fragment key={file}>
              <TreeItem role="treeitem" tabIndex={0} aria-expanded={expanded} aria-selected={selectedFile === file && !selectedId}
                title={file} kind="file" icon={<Icon name="file-text" />} label={file.split("/").pop()}
                count={entries.length} expandable open={expanded} selected={selectedFile === file && !selectedId}
                onSelect={() => { setSelectedFile(file); setSelectedId(null); }} onToggle={toggle}
                onKeyDown={event => {
                  if (event.key === "Enter" || event.key === " ") { event.preventDefault(); setSelectedFile(file); setSelectedId(null); }
                  if ((event.key === "ArrowRight" && !expanded) || (event.key === "ArrowLeft" && expanded)) { event.preventDefault(); toggle(); }
                }} />
              {expanded && <div role="group">{entries.map(item => <TreeItem key={item.trace.verification.id}
                role="treeitem" tabIndex={0} aria-selected={selectedId === item.trace.verification.id}
                label={item.trace.verification.name} title={`${item.label}: ${item.trace.verification.name}`}
                icon={<ElementIcon type={item.trace.verification.type} size="sm" />} kind="element" depth={1}
                selected={selectedId === item.trace.verification.id} onSelect={() => selectExample(item.trace.verification.id)}
                onKeyDown={event => {
                  if (event.key === "Enter" || event.key === " ") { event.preventDefault(); selectExample(item.trace.verification.id); }
                }} />)}</div>}
            </Fragment>;
          })}
        </PaneTree>
        {filtered.length === 0 && <p>No matching verifications.</p>}
      </PaneControlSection>
      <PaneSummary placement="footer" items={[{ label: "Files", value: files.length }, { label: "Verifications", value: TRACE_EXAMPLES.length }]} />
    </SidePaneFrame>,
    main: ({ onOpenElement, onOpenSource }: { onOpenElement: (id: string) => void; onOpenSource: (file: string) => void }) => <RouteFrame viewId="traces">
      {example ? <TraceFlow trace={example.trace} onOpenElement={onOpenElement} onOpenSource={element => onOpenSource(element.file)} />
        : <TraceReportPanel><TraceReportContent>
          <TraceFileHeader file={selectedFile} countLabel={`${TRACE_EXAMPLES.filter(item => item.trace.verification.file === selectedFile).length} verifications`} />
          <TraceVerificationList>{TRACE_EXAMPLES.filter(item => item.trace.verification.file === selectedFile).map(item =>
            <TraceVerificationCard key={item.trace.verification.id}>
              <TraceVerificationHeader><TraceVerificationTitleButton onClick={() => selectExample(item.trace.verification.id)}>
                {item.trace.verification.name}
              </TraceVerificationTitleButton></TraceVerificationHeader>
              <TraceVerificationMeta rows={[
                { label: "Type", value: "Test" },
                { label: "Directly verified", value: item.trace.requirements.filter(node => node.directlyVerified).length },
                { label: "Requirements in trace", value: item.trace.requirements.length },
              ]} />
            </TraceVerificationCard>)}</TraceVerificationList>
        </TraceReportContent></TraceReportPanel>}
    </RouteFrame>,
  };
}
