import { useMemo, useState } from "react";
import {
  Badge,
  Box,
  Card,
  Code,
  Flex,
  Grid,
  Heading,
  Text,
  TextField,
} from "@radix-ui/themes";
import { MagnifyingGlassIcon } from "@radix-ui/react-icons";
import { useStore } from "../store/StoreContext";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";
import { useExplorerUiState } from "../components/ExplorerUiState";
import type { ProjectStoreElement } from "../store/types";
import { REQVIRE_SURFACE_BASE } from "../theme";
import { ViewFrame } from "./ViewFrame";

/*
 * Report-projection views (Traces and Coverage).
 *
 * Each view renders natively from its Project Store report projection — no
 * iframe-mounted standalone page content. These views
 * surface store-backed report data and route element rows to the in-shell
 * element-detail modal.
 */

function pct(value: number | undefined): string {
  return typeof value === "number" ? `${value}%` : "—";
}

function Metric({ label, value }: { label: string; value: string | number }) {
  return (
    <Box className="explorer-metric">
      <Flex direction="column" gap="1">
        <Text size="6" weight="bold">
          {value}
        </Text>
        <Text size="1" color="gray">
          {label}
        </Text>
      </Flex>
    </Box>
  );
}

interface TraceVerificationNode {
  id: string;
  name: string;
  file: string;
  directCount: number;
  totalCount: number;
  requirementIds: string[];
}

interface TraceFileNode {
  file: string;
  verifications: TraceVerificationNode[];
}

function isVerification(element: ProjectStoreElement): boolean {
  return (
    element.type_family === "verification" ||
    element.element_type.toLowerCase().includes("verification")
  );
}

export function TracesView({
  onOpenElement,
}: {
  onOpenElement: (id: string) => void;
} & Partial<ExplorerViewProps>) {
  const { store } = useStore();
  const { traceMode: mode } = useExplorerUiState();
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [query, setQuery] = useState("");
  const elementById = useMemo(
    () => new Map(store.elements.map((element) => [element.id, element])),
    [store.elements],
  );
  const traceFiles = useMemo(() => {
    const files = store.traces?.files ?? {};
    const entries = Object.entries(files).sort((a, b) => a[0].localeCompare(b[0]));
    if (entries.length > 0) {
      return entries.map(([file, entry]) => ({
        file,
        verifications: (entry.verifications ?? []).map((verification) => ({
          id: verification.identifier,
          name: verification.name,
          file: verification.file || file,
          directCount: verification.directly_verified_count ?? 0,
          totalCount: verification.total_requirements_in_tree ?? 0,
          requirementIds: verification.directly_verified_requirements ?? [],
        })),
      }));
    }

    const requirementIdsByVerification = new Map<string, string[]>();
    for (const relation of store.relations) {
      const relationType =
        relation.canonical_relation_type || relation.relation_type || "";
      if (!relationType.toLowerCase().includes("verify")) continue;
      const source = elementById.get(relation.source_id);
      const target = elementById.get(relation.target_id);
      if (!source || !target || !isVerification(source)) continue;
      const list = requirementIdsByVerification.get(source.id) ?? [];
      list.push(target.id);
      requirementIdsByVerification.set(source.id, list);
    }

    const byFile = new Map<string, TraceVerificationNode[]>();
    for (const element of store.elements.filter(isVerification)) {
      const requirementIds = requirementIdsByVerification.get(element.id) ?? [];
      const list = byFile.get(element.file_path) ?? [];
      list.push({
        id: element.id,
        name: element.name,
        file: element.file_path,
        directCount: requirementIds.length,
        totalCount: requirementIds.length,
        requirementIds,
      });
      byFile.set(element.file_path, list);
    }

    return [...byFile.entries()]
      .sort((a, b) => a[0].localeCompare(b[0]))
      .map(([file, verifications]) => ({ file, verifications }));
  }, [elementById, store.elements, store.relations, store.traces?.files]);
  const totalVerifications = traceFiles.reduce(
    (sum, file) => sum + file.verifications.length,
    0,
  );
  const directRequirementCount = traceFiles.reduce(
    (sum, file) =>
      sum +
      file.verifications.reduce(
        (inner, verification) => inner + verification.requirementIds.length,
        0,
      ),
    0,
  );
  const selected = selectedId ? elementById.get(selectedId) ?? null : null;
  const traceSearchResults = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return [];
    const verificationMatches = traceFiles
      .flatMap((file) =>
        file.verifications.map((verification) => ({
          id: verification.id,
          label: verification.name,
          kind: "verification",
          file: file.file,
        })),
      )
      .filter((item) => `${item.label} ${item.id} ${item.file}`.toLowerCase().includes(q));
    const requirementMatches = Array.from(
      new Set(traceFiles.flatMap((file) => file.verifications.flatMap((v) => v.requirementIds))),
    )
      .map((id) => {
        const element = elementById.get(id);
        return {
          id,
          label: element?.name ?? id,
          kind: "requirement",
          file: element?.file_path ?? "",
        };
      })
      .filter((item) => `${item.label} ${item.id} ${item.file}`.toLowerCase().includes(q));
    return [...verificationMatches, ...requirementMatches].slice(0, 30);
  }, [elementById, query, traceFiles]);

  return (
    <ViewFrame testId="traces">
      <Grid
        columns={{ initial: "1fr", lg: "minmax(0, 1fr) 390px" }}
        className="explorer-route"
      >
        <Box className="explorer-main-panel trace-main-panel">
          {mode === "flow" ? (
            <TraceFlow
              files={traceFiles}
              elementById={elementById}
              selectedId={selectedId}
              onSelect={setSelectedId}
              onOpenElement={onOpenElement}
            />
          ) : (
            <TraceRows
              files={traceFiles}
              onSelect={setSelectedId}
              onOpenElement={onOpenElement}
            />
          )}
          {traceFiles.length === 0 && (
            <Text color="gray">No verification traces in store.</Text>
          )}
        </Box>

        <Box className="graph-sidebar">
          <div className="graph-search-panel">
            <TextField.Root
              aria-label="Search traces"
              placeholder="Search verifications, requirements, files"
              value={query}
              onChange={(event) => setQuery(event.target.value)}
            >
              <TextField.Slot>
                <MagnifyingGlassIcon />
              </TextField.Slot>
            </TextField.Root>
            {traceSearchResults.length > 0 && (
              <ul className="graph-results">
                {traceSearchResults.map((item) => (
                  <li key={`${item.kind}:${item.id}`}>
                    <button
                      type="button"
                      onClick={() => {
                        setSelectedId(item.id);
                        onOpenElement(item.id);
                      }}
                    >
                      <span
                        className="graph-result-swatch"
                        style={{ backgroundColor: item.kind === "verification" ? "#4caf50" : "#673ab7" }}
                      />
                      <span>{item.label}</span>
                    </button>
                  </li>
                ))}
              </ul>
            )}
          </div>
          <div className="graph-inspector-header">
            <Heading as="h2" size="3">
              Trace Inspector
            </Heading>
          </div>
          <div className="graph-inspector-body">
          <Flex direction="column" gap="3">
            {selected ? (
              <>
                <button
                  type="button"
                  onClick={() => onOpenElement(selected.id)}
                  className="explorer-command"
                >
                  Open element detail
                </button>
                <Box>
                  <Heading as="h2" size="3" mb="2">
                    {selected.name}
                  </Heading>
                  <Flex gap="2" wrap="wrap">
                    <Badge>{selected.type_family}</Badge>
                    <Code>{selected.element_type}</Code>
                  </Flex>
                </Box>
                <Text size="1" color="gray">
                  {selected.file_path}:{selected.line_number}
                </Text>
              </>
            ) : (
              <Text size="2" color="gray">
                Select a verification or requirement in the flow to inspect it.
              </Text>
            )}
          </Flex>
          </div>
          <div className="graph-summary-strip">
            <span>
              Files <strong>{traceFiles.length}</strong>
            </span>
            <span>
              Verifications <strong>{totalVerifications}</strong>
            </span>
            <span>
              Direct reqs <strong>{directRequirementCount}</strong>
            </span>
            <span>
              Elements <strong>{store.elements.length}</strong>
            </span>
          </div>
        </Box>
      </Grid>
    </ViewFrame>
  );
}

function TraceRows({
  files,
  onSelect,
  onOpenElement,
}: {
  files: TraceFileNode[];
  onSelect: (id: string) => void;
  onOpenElement: (id: string) => void;
}) {
  return (
    <Flex data-testid="trace-rows" direction="column" gap="3">
      {files.map((file) => (
        <Box key={file.file} className="trace-row-group">
          <Heading as="h2" size="2" mb="2">
            <Code>{file.file}</Code>
          </Heading>
          <Flex direction="column" gap="1">
            {file.verifications.map((verification) => (
              <button
                key={verification.id}
                type="button"
                onClick={() => {
                  onSelect(verification.id);
                  onOpenElement(verification.id);
                }}
                className="explorer-list-row"
              >
                <Text size="2">{verification.name}</Text>
                <Badge color="green">{verification.directCount} direct</Badge>
                <Text size="1" color="gray">
                  {verification.totalCount} in tree
                </Text>
              </button>
            ))}
          </Flex>
        </Box>
      ))}
    </Flex>
  );
}

function TraceFlow({
  files,
  elementById,
  selectedId,
  onSelect,
  onOpenElement,
}: {
  files: TraceFileNode[];
  elementById: Map<string, ProjectStoreElement>;
  selectedId: string | null;
  onSelect: (id: string) => void;
  onOpenElement: (id: string) => void;
}) {
  const width = 1050;
  const rowHeight = 58;
  const verifications = files.flatMap((file) => file.verifications);
  const requirementIds = Array.from(
    new Set(verifications.flatMap((verification) => verification.requirementIds)),
  );
  const rows = Math.max(files.length, verifications.length, requirementIds.length, 1);
  const height = Math.max(520, rows * rowHeight + 80);
  const fileY = new Map(
    files.map((file, index) => [file.file, 60 + index * rowHeight]),
  );
  const verificationY = new Map(
    verifications.map((verification, index) => [
      verification.id,
      60 + index * rowHeight,
    ]),
  );
  const requirementY = new Map(
    requirementIds.map((id, index) => [id, 60 + index * rowHeight]),
  );

  return (
    <Box data-testid="trace-flow" className="trace-canvas h-full min-h-[520px]">
      <svg
        viewBox={`0 0 ${width} ${height}`}
        role="img"
        aria-label="Verification trace flow"
        className="h-full min-h-[520px] w-full"
      >
        <text x="80" y="28" fontSize="12" fontWeight="700" fill="#4b4d4a">
          Files
        </text>
        <text x="440" y="28" fontSize="12" fontWeight="700" fill="#4b4d4a">
          Verifications
        </text>
        <text x="790" y="28" fontSize="12" fontWeight="700" fill="#4b4d4a">
          Requirements
        </text>
        {files.flatMap((file) =>
          file.verifications.map((verification) => {
            const y1 = fileY.get(file.file) ?? 60;
            const y2 = verificationY.get(verification.id) ?? 60;
            return (
              <path
                key={`${file.file}-${verification.id}`}
                d={`M 260 ${y1} C 330 ${y1}, 330 ${y2}, 400 ${y2}`}
                fill="none"
                stroke="#8a8a86"
                strokeWidth="2"
                opacity="0.45"
              />
            );
          }),
        )}
        {verifications.flatMap((verification) =>
          verification.requirementIds.map((id) => {
            const y1 = verificationY.get(verification.id) ?? 60;
            const y2 = requirementY.get(id) ?? 60;
            return (
              <path
                key={`${verification.id}-${id}`}
                d={`M 620 ${y1} C 700 ${y1}, 700 ${y2}, 760 ${y2}`}
                fill="none"
                stroke="#4caf50"
                strokeWidth="2.5"
                opacity="0.5"
              />
            );
          }),
        )}
        {files.map((file) => (
          <TracePill
            key={file.file}
            x={40}
            y={(fileY.get(file.file) ?? 60) - 18}
            width={220}
            label={file.file}
            color="#00897b"
          />
        ))}
        {verifications.map((verification) => (
          <TracePill
            key={verification.id}
            x={400}
            y={(verificationY.get(verification.id) ?? 60) - 18}
            width={220}
            label={verification.name}
            color="#4caf50"
            selected={verification.id === selectedId}
            onClick={() => onSelect(verification.id)}
            onDoubleClick={() => onOpenElement(verification.id)}
          />
        ))}
        {requirementIds.map((id) => {
          const requirement = elementById.get(id);
          return (
            <TracePill
              key={id}
              x={760}
              y={(requirementY.get(id) ?? 60) - 18}
              width={240}
              label={requirement?.name ?? id}
              color="#673ab7"
              selected={id === selectedId}
              onClick={() => onSelect(id)}
              onDoubleClick={() => onOpenElement(id)}
            />
          );
        })}
      </svg>
    </Box>
  );
}

function TracePill({
  x,
  y,
  width,
  label,
  color,
  selected = false,
  onClick,
  onDoubleClick,
}: {
  x: number;
  y: number;
  width: number;
  label: string;
  color: string;
  selected?: boolean;
  onClick?: () => void;
  onDoubleClick?: () => void;
}) {
  return (
    <g
      className={onClick ? "cursor-pointer" : undefined}
      onClick={onClick}
      onDoubleClick={onDoubleClick}
    >
      <rect
        x={x}
        y={y}
        width={width}
        height="36"
        rx="6"
        fill={color}
        stroke={selected ? "#172027" : REQVIRE_SURFACE_BASE}
        strokeWidth={selected ? 3 : 1}
        opacity="0.92"
      />
      <text
        x={x + 10}
        y={y + 23}
        fontSize="12"
        fill="#ffffff"
        pointerEvents="none"
      >
        {label.length > 28 ? `${label.slice(0, 25)}...` : label}
      </text>
      <title>{label}</title>
    </g>
  );
}

export function CoverageView(_: Partial<ExplorerViewProps> = {}) {
  const { store } = useStore();
  const summary = store.coverage?.summary ?? {};
  return (
    <ViewFrame testId="coverage">
      <Grid columns={{ initial: "1fr", lg: "minmax(0, 1fr) 390px" }} className="explorer-route">
        <Box className="explorer-document-panel">
        <Grid columns={{ initial: "2", md: "4" }} gap="3" mb="4">
          <Metric
            label="Leaf requirement coverage"
            value={pct(summary.leaf_requirements_coverage_percentage)}
          />
          <Metric
            label="Implementation coverage"
            value={pct(summary.implementation_coverage_percentage)}
          />
          <Metric
            label="Test verification satisfaction"
            value={pct(summary.test_verifications_satisfaction_percentage)}
          />
          <Metric
            label="Orphaned verifications"
            value={summary.orphaned_verifications ?? 0}
          />
        </Grid>
        <Card variant="surface" className="explorer-card">
          <Flex direction="column" gap="1">
            <Text size="2">
              Requirements in scope:{" "}
              <strong>{summary.total_requirements_in_scope ?? 0}</strong>
            </Text>
            <Text size="2">
              Covered requirements:{" "}
              <strong>{summary.covered_requirements ?? 0}</strong> / uncovered{" "}
              <strong>{summary.uncovered_requirements ?? 0}</strong>
            </Text>
            <Text size="2">
              Leaf requirements:{" "}
              <strong>{summary.total_leaf_requirements ?? 0}</strong> (verified{" "}
              {summary.verified_leaf_requirements ?? 0}, unverified{" "}
              {summary.unverified_leaf_requirements ?? 0})
            </Text>
            <Text size="2">
              Verifications: <strong>{summary.total_verifications ?? 0}</strong>{" "}
              (test {summary.total_test_verifications ?? 0}, satisfied{" "}
              {summary.satisfied_test_verifications ?? 0})
            </Text>
          </Flex>
        </Card>
        </Box>
        <Box className="graph-sidebar">
          <div className="graph-inspector-header">
            <Heading as="h2" size="3">Coverage Inspector</Heading>
          </div>
          <div className="graph-inspector-body">
            <Text size="2" color="gray">
              Coverage records are generated from the same Project Store relations used by Model and Traces.
            </Text>
          </div>
        </Box>
      </Grid>
    </ViewFrame>
  );
}
