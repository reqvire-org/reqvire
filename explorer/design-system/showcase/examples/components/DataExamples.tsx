import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds";
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

function Section({ title, desc, children }: { title: string; desc?: string; children: ComponentChildren }) {
  return (
    <ShowcaseRegion regions={["section"]} as="section">
      <ShowcaseRegion regions={["section__heading"]}>
        <ShowcaseRegion regions={["section__title"]}>{title}</ShowcaseRegion>
        {desc && <ShowcaseRegion regions={["section__desc"]}>{desc}</ShowcaseRegion>}
      </ShowcaseRegion>
      {children}
    </ShowcaseRegion>
  );
}

const ELEMENT_ROLES = [
  "capability", "requirement", "contract", "source", "constraint",
  "behavior", "state", "input-output", "specification", "semantic-contract", "semantic-query",
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

export function DataExamples() {
  return (
    <ShowcaseRegion regions={["page"]}>

      <Section title="TypeBadge" desc="Element-type chip with the same marker shape and glyph contract as ElementIcon. Use tinted for header badges.">
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Plain</ShowcaseRegion>
          <ShowcaseRegion regions={["row", "row--center"]}>
            {ELEMENT_ROLES.map((role) => (
              <TypeBadge key={role} type={role} dot />
            ))}
          </ShowcaseRegion>
        </ShowcaseRegion>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Tinted</ShowcaseRegion>
          <ShowcaseRegion regions={["row", "row--center"]}>
            {ELEMENT_ROLES.map((role) => (
              <TypeBadge key={role} type={role} tinted dot />
            ))}
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>

      <Section title="ElementIcon" desc="Colored model-element marker. Contracts use diamond subtype glyphs; verification subtypes use verification glyphs.">
        {(["sm", "md", "lg"] as const).map((size) => (
          <ShowcaseRegion key={size} regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>{size}</ShowcaseRegion>
            <ShowcaseRegion regions={["row", "row--center"]}>
              {ELEMENT_ROLES.map((role) => (
                <ElementIcon key={role} type={role} size={size} />
              ))}
            </ShowcaseRegion>
          </ShowcaseRegion>
        ))}
      </Section>

      <Section title="Chip" desc="Filter chip with active toggle.">
        <ShowcaseRegion regions={["row", "row--center"]}>
          <Chip>All types</Chip>
          <Chip active>Requirement</Chip>
          <Chip>Verification</Chip>
          <Chip>Capability</Chip>
          <Chip>Contract</Chip>
          <Chip>12 results</Chip>
        </ShowcaseRegion>
      </Section>

      <Section title="Stat &amp; StatRow" desc="Key-value stat pairs. stacked=true for dashboard counts.">
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Inline (default)</ShowcaseRegion>
          <StatRow>
            <Stat label="Elements" value={640} />
            <Stat label="Relations" value={1090} />
            <Stat label="Submodels" value={13} />
            <Stat label="Coverage" value="86%" />
          </StatRow>
        </ShowcaseRegion>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Stacked (dashboard)</ShowcaseRegion>
          <ShowcaseRegion regions={["row"]}>
            <Stat label="Elements" value={640} stacked />
            <Stat label="Relations" value={1090} stacked />
            <Stat label="Submodels" value={13} stacked />
          </ShowcaseRegion>
        </ShowcaseRegion>
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
                  <ShowcaseRegion regions={["table-strong"]} asChild><TableCell >{row.name}</TableCell></ShowcaseRegion>
                  <TableCell>{row.category}</TableCell>
                  <TableCell>{row.owner}</TableCell>
                  <TableCell>
                    <ShowcaseRegion regions={row.status === "Ready" ? ["status", "status--ready"] : ["status"]} as="span">
                      {row.status}
                    </ShowcaseRegion>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </TableViewport>
      </Section>

    </ShowcaseRegion>
  );
}
