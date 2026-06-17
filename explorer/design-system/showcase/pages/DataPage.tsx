import {
  Chip,
  CodeRef,
  ElementIcon,
  Stat,
  StatRow,
  Table,
  TableBody,
  TableCell,
  TableHeaderCell,
  TableHeaderGroup,
  TableRow,
  TableSortButton,
  TableViewport,
  TypeBadge,
} from "@ds";

function Section({ title, desc, children }: { title: string; desc?: string; children: React.ReactNode }) {
  return (
    <section className="showcase-section">
      <div className="showcase-section__heading">
        <div className="showcase-section__title">{title}</div>
        {desc && <div className="showcase-section__desc">{desc}</div>}
      </div>
      {children}
    </section>
  );
}

const ELEMENT_ROLES = [
  "capability", "requirement", "refinement", "source", "constraint",
  "behavior", "state", "input-output", "specification", "semantic-contract",
  "verification-objective", "test-verification", "formal-proof-verification",
  "analysis-verification", "inspection-verification", "demonstration-verification",
  "verification", "ontology", "resource",
] as const;

const TABLE_ROWS = [
  { id: "ROW-001", name: "Workspace index", category: "Catalog", owner: "Platform", status: "Ready" },
  { id: "ROW-002", name: "Review queue", category: "Workflow", owner: "Modeling", status: "Draft" },
  { id: "ROW-003", name: "Coverage snapshot", category: "Report", owner: "Verification", status: "Ready" },
  { id: "ROW-004", name: "Import job", category: "Pipeline", owner: "Tools", status: "Running" },
  { id: "ROW-005", name: "Archive export", category: "Artifact", owner: "Release", status: "Ready" },
];

export function DataPage() {
  return (
    <div className="showcase-page">

      <Section title="TypeBadge" desc="Element-type chip with the same marker shape and glyph contract as ElementIcon. Use tinted for header badges.">
        <div className="showcase-col">
          <div className="showcase-label">Plain</div>
          <div className="showcase-row showcase-row--center">
            {ELEMENT_ROLES.map((role) => (
              <TypeBadge key={role} type={role} dot />
            ))}
          </div>
        </div>
        <div className="showcase-col">
          <div className="showcase-label">Tinted</div>
          <div className="showcase-row showcase-row--center">
            {ELEMENT_ROLES.map((role) => (
              <TypeBadge key={role} type={role} tinted dot />
            ))}
          </div>
        </div>
      </Section>

      <Section title="ElementIcon" desc="Colored model-element marker. Refinements use diamond subtype glyphs; verification subtypes use verification glyphs.">
        {(["sm", "md", "lg"] as const).map((size) => (
          <div key={size} className="showcase-col">
            <div className="showcase-label">{size}</div>
            <div className="showcase-row showcase-row--center">
              {ELEMENT_ROLES.map((role) => (
                <ElementIcon key={role} type={role} size={size} />
              ))}
            </div>
          </div>
        ))}
      </Section>

      <Section title="Chip" desc="Filter chip with active toggle.">
        <div className="showcase-row showcase-row--center">
          <Chip>All types</Chip>
          <Chip className="is-active">Requirement</Chip>
          <Chip>Verification</Chip>
          <Chip>Capability</Chip>
          <Chip>Refinement</Chip>
          <Chip>12 results</Chip>
        </div>
      </Section>

      <Section title="Stat &amp; StatRow" desc="Key-value stat pairs. stacked=true for dashboard counts.">
        <div className="showcase-col">
          <div className="showcase-label">Inline (default)</div>
          <StatRow>
            <Stat label="Elements" value={640} />
            <Stat label="Relations" value={1090} />
            <Stat label="Submodels" value={13} />
            <Stat label="Coverage" value="86%" />
          </StatRow>
        </div>
        <div className="showcase-col">
          <div className="showcase-label">Stacked (dashboard)</div>
          <div className="showcase-row">
            <Stat label="Elements" value={640} stacked />
            <Stat label="Relations" value={1090} stacked />
            <Stat label="Submodels" value={13} stacked />
          </div>
        </div>
      </Section>

      <Section title="Table" desc="Primitive sortable, scrollable table with sticky header. Explorer file/model tables are product patterns.">
        <TableViewport>
          <Table>
            <TableHeaderGroup>
              <TableRow>
                <TableHeaderCell><TableSortButton direction="asc">ID</TableSortButton></TableHeaderCell>
                <TableHeaderCell><TableSortButton>Name</TableSortButton></TableHeaderCell>
                <TableHeaderCell><TableSortButton>Category</TableSortButton></TableHeaderCell>
                <TableHeaderCell>Owner</TableHeaderCell>
                <TableHeaderCell>Status</TableHeaderCell>
              </TableRow>
            </TableHeaderGroup>
            <TableBody>
              {TABLE_ROWS.map((row) => (
                <TableRow key={row.id}>
                  <TableCell><CodeRef>{row.id}</CodeRef></TableCell>
                  <TableCell className="showcase-table-strong">{row.name}</TableCell>
                  <TableCell>{row.category}</TableCell>
                  <TableCell>{row.owner}</TableCell>
                  <TableCell>
                    <span className={row.status === "Ready" ? "showcase-status showcase-status--ready" : "showcase-status"}>
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
