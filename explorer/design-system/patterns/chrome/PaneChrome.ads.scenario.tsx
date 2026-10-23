import { ShowcaseRegion } from "@ds/review";
import { type ComponentChildren } from "preact";
import { Icon, PaneChromeHeader, PaneTree, PaneTreeNode, ReqvireRailMark, TreeItem } from "@ds";
import { fixture, MODEL_TREE_ROWS } from "./PaneChrome.ads.fixture";
export const scenario = {"title":"Pane Chrome And Trees",covers: ["PaneChromeHeader","ReqvireRailMark","PaneTree","PaneTreeNode"]};
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
        <ShowcaseRegion regions={["pattern-grid", "pattern-grid--two"]}>
          <ShowcaseRegion regions={["pattern-card"]}>
            <PaneChromeHeader title="Model"/>
            <ShowcaseRegion regions={["pattern-card__body"]}>
              <ReqvireRailMark placement="inline"/>
              <ShowcaseRegion regions={["pattern-muted"]} as="span">Rail mark and pane header keep Explorer chrome consistent.</ShowcaseRegion>
            </ShowcaseRegion>
          </ShowcaseRegion>
          <ShowcaseRegion regions={["pattern-card"]}>
            <PaneTree aria-label="Example file tree">
              {MODEL_TREE_ROWS.map((row) => (<PaneTreeNode key={row.id}>
                  <TreeItem label={row.label} icon={<Icon name={row.icon} size={14}/>} count={row.count} depth={row.depth} kind={row.kind} expandable={row.expandable} open={row.open} selected={row.selected}/>
                </PaneTreeNode>))}
            </PaneTree>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
