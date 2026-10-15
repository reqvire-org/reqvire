import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds";
import { CodeRef, Table, TableBody, TableCell, TableHeaderCell, TableHeaderGroup, TableRow, TableSortButton, TableViewport } from "@ds";
import { fixture, TABLE_ROWS } from "./Table.fixture";
export const scenario = {"title":"Table","covers":["TableViewport","Table","TableHeaderGroup","TableRow","TableHeaderCell","TableSortButton","TableBody","TableCell","CodeRef"]};
function Section({ title, desc, children }: {
    title: string;
    desc?: string;
    children: ComponentChildren;
}) {
    return (<ShowcaseRegion regions={["section"]} as="section">
      <ShowcaseRegion regions={["section__heading"]}>
        <ShowcaseRegion regions={["section__title"]}>{title}</ShowcaseRegion>
        {desc && <ShowcaseRegion regions={["section__desc"]}>{desc}</ShowcaseRegion>}
      </ShowcaseRegion>
      {children}
    </ShowcaseRegion>);
}
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
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
              {TABLE_ROWS.map((row) => (<TableRow key={row.id}>
                  <TableCell><CodeRef>{row.id}</CodeRef></TableCell>
                  <ShowcaseRegion regions={["table-strong"]} asChild><TableCell>{row.name}</TableCell></ShowcaseRegion>
                  <TableCell>{row.category}</TableCell>
                  <TableCell>{row.owner}</TableCell>
                  <TableCell>
                    <ShowcaseRegion regions={row.status === "Ready" ? ["status", "status--ready"] : ["status"]} as="span">
                      {row.status}
                    </ShowcaseRegion>
                  </TableCell>
                </TableRow>))}
            </TableBody>
          </Table>
        </TableViewport>
      </Section>
</ShowcaseRegion>;
}
