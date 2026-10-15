import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds";
import { Stat, StatRow } from "@ds";
import { fixture } from "./Stat.fixture";
export const scenario = {"title":"Stat & StatRow","covers":["StatRow","Stat"]};
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
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Inline (default)</ShowcaseRegion>
          <StatRow>
            <Stat label="Elements" value={640}/>
            <Stat label="Relations" value={1090}/>
            <Stat label="Submodels" value={13}/>
            <Stat label="Coverage" value="86%"/>
          </StatRow>
        </ShowcaseRegion>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Stacked (dashboard)</ShowcaseRegion>
          <ShowcaseRegion regions={["row"]}>
            <Stat label="Elements" value={640} stacked/>
            <Stat label="Relations" value={1090} stacked/>
            <Stat label="Submodels" value={13} stacked/>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
