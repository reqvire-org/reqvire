import {
  Chip,
  ElementIcon,
  RelationPill,
  Stat,
  StatRow,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  TableSortButton,
  TableViewport,
  TypeBadge,
} from "@ds";

function Section({ title, desc, children }: { title: string; desc?: string; children: React.ReactNode }) {
  return (
    <section className="sc-section">
      <div className="sc-section__heading">
        <div className="sc-section__title">{title}</div>
        {desc && <div className="sc-section__desc">{desc}</div>}
      </div>
      {children}
    </section>
  );
}

const ELEMENT_ROLES = [
  "capability", "requirement", "refinement", "source", "constraint",
  "behavior", "state", "input-output", "specification", "semantic-contract",
  "verification", "ontology", "resource",
] as const;

const TABLE_ROWS = [
  { id: "CAP-001", name: "System Modeling Capability", type: "capability", file: "Capabilities.md", status: "active" },
  { id: "REQ-001", name: "Model Structure Specification", type: "requirement", file: "SystemRequirements.md", status: "draft" },
  { id: "REQ-002", name: "Traceability Coverage Requirement", type: "requirement", file: "SystemRequirements.md", status: "active" },
  { id: "VER-001", name: "Model Structure Test", type: "test-verification", file: "Verifications/ModelStructure.md", status: "active" },
  { id: "SPEC-001", name: "Containment Specification", type: "specification", file: "Specifications.md", status: "active" },
];

export function DataPage() {
  return (
    <div className="sc-page">

      <Section title="TypeBadge" desc="Element-type chip with colored dot. Use tinted for header badges.">
        <div className="sc-col">
          <div className="sc-label">Plain</div>
          <div className="sc-row sc-row--center">
            {ELEMENT_ROLES.map((role) => (
              <TypeBadge key={role} type={role} dot />
            ))}
          </div>
        </div>
        <div className="sc-col">
          <div className="sc-label">Tinted</div>
          <div className="sc-row sc-row--center">
            {ELEMENT_ROLES.map((role) => (
              <TypeBadge key={role} type={role} tinted dot />
            ))}
          </div>
        </div>
      </Section>

      <Section title="ElementIcon" desc="Colored model-element glyph. Capability = hub, refinements = diamond, rest = square.">
        {(["sm", "md", "lg"] as const).map((size) => (
          <div key={size} className="sc-col">
            <div className="sc-label">{size}</div>
            <div className="sc-row sc-row--center">
              {ELEMENT_ROLES.map((role) => (
                <ElementIcon key={role} type={role} size={size} />
              ))}
            </div>
          </div>
        ))}
      </Section>

      <Section title="RelationPill" desc="Relation row: kind label + colored pip + target.">
        <div className="sc-col">
          <div className="sc-label">Link (href)</div>
          <div className="sc-col">
            <RelationPill kind="specifiedBy" label="System Modeling Capability" pipColorToken="--capability" href="#" />
            <RelationPill kind="verifiedBy" label="Model Structure Test" pipColorToken="--verification" href="#" />
            <RelationPill kind="derivedFrom" label="Traceability Coverage Requirement" pipColorToken="--requirement" href="#" />
            <RelationPill kind="refinedBy" label="Containment Specification" pipColorToken="--refinement" href="#" />
          </div>
        </div>
        <div className="sc-col">
          <div className="sc-label">Button (no href)</div>
          <RelationPill kind="attachedTo" label="Architecture.md" pipColorToken="--resource" />
          <RelationPill kind="unknown" label="Unresolved element ID" disabled />
        </div>
      </Section>

      <Section title="Chip" desc="Filter chip with active toggle.">
        <div className="sc-row sc-row--center">
          <Chip>All types</Chip>
          <Chip className="is-active">Requirement</Chip>
          <Chip>Verification</Chip>
          <Chip>Capability</Chip>
          <Chip>Refinement</Chip>
          <Chip>12 results</Chip>
        </div>
      </Section>

      <Section title="Stat &amp; StatRow" desc="Key-value stat pairs. stacked=true for dashboard counts.">
        <div className="sc-col">
          <div className="sc-label">Inline (default)</div>
          <StatRow>
            <Stat label="Elements" value={640} />
            <Stat label="Relations" value={1090} />
            <Stat label="Submodels" value={13} />
            <Stat label="Coverage" value="86%" />
          </StatRow>
        </div>
        <div className="sc-col">
          <div className="sc-label">Stacked (dashboard)</div>
          <div className="sc-row">
            <Stat label="Elements" value={640} stacked />
            <Stat label="Relations" value={1090} stacked />
            <Stat label="Submodels" value={13} stacked />
          </div>
        </div>
      </Section>

      <Section title="Table" desc="Sortable, scrollable data table with sticky header.">
        <TableViewport>
          <Table>
            <TableHead>
              <TableRow>
                <TableHeader><TableSortButton direction="asc">ID</TableSortButton></TableHeader>
                <TableHeader><TableSortButton>Name</TableSortButton></TableHeader>
                <TableHeader><TableSortButton>Type</TableSortButton></TableHeader>
                <TableHeader>File</TableHeader>
                <TableHeader>Status</TableHeader>
              </TableRow>
            </TableHead>
            <TableBody>
              {TABLE_ROWS.map((row) => (
                <TableRow key={row.id}>
                  <TableCell><code className="rq-coderef">{row.id}</code></TableCell>
                  <TableCell style={{ fontWeight: "var(--weight-medium)", color: "var(--text-strong)" }}>{row.name}</TableCell>
                  <TableCell><TypeBadge type={row.type} tinted dot /></TableCell>
                  <TableCell><span style={{ fontFamily: "var(--font-mono)", fontSize: "var(--text-caption)", color: "var(--text-muted)" }}>{row.file}</span></TableCell>
                  <TableCell>
                    <span style={{ color: row.status === "active" ? "var(--success)" : "var(--text-muted)", fontSize: "var(--text-caption)" }}>
                      {row.status}
                    </span>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </TableViewport>
      </Section>

    </div>
  );
}
