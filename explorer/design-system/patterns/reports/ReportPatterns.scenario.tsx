import { ShowcaseRegion } from "@ds";
import { useState } from "preact/hooks";
import { type ComponentChildren } from "preact";
import { DetailDialog, ElementDetailContent, CoverageDashboard, CoverageDrilldown, CoveragePanel } from "@ds";
import { fixture, COVERAGE_PATTERN_ITEMS, SHOWCASE_DETAIL_ELEMENTS } from "./ReportPatterns.fixture";
export const scenario = {"title":"Coverage Drill-down","covers":["ShowcaseRegion","CoverageDashboard","CoveragePanel","CoverageDrilldown"]};
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
const [elementDialogOpen, setElementDialogOpen] = useState(false);
const [elementDialogStack, setElementDialogStack] = useState<string[]>([]);
const pushElementDialog = (id: string) => {
    if (!SHOWCASE_DETAIL_ELEMENTS[id])
        return;
    setElementDialogStack((stack) => {
        const currentId = stack.at(-1);
        return currentId === id ? stack : [...stack, id];
    });
    setElementDialogOpen(true);
};
const currentElement = SHOWCASE_DETAIL_ELEMENTS[elementDialogStack.at(-1) ?? "REQ-DET-042"];
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <CoverageDashboard>
          <CoveragePanel>
            <CoverageDrilldown items={COVERAGE_PATTERN_ITEMS} onInspect={target => pushElementDialog(target.id)}/>
          </CoveragePanel>
        </CoverageDashboard>
      </Section>
<DetailDialog open={elementDialogOpen} onOpenChange={setElementDialogOpen} title={currentElement.title} typeFamily={currentElement.typeFamily} elementType={currentElement.elementType} sourceHref={currentElement.sourceHref} onOpenSource={() => {}}>
<ElementDetailContent metaBadges={currentElement.metaBadges} content={currentElement.content} relations={currentElement.relations} contract_bindings={currentElement.contract_bindings} onOpenElement={pushElementDialog} onOpenResource={() => {}} />
</DetailDialog>
</ShowcaseRegion>;
}
