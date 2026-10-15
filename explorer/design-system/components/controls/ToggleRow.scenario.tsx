import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds";
import { ToggleRow } from "@ds";
import { fixture } from "./ToggleRow.fixture";
export const scenario = {"title":"ToggleRow","covers":["ToggleRow"]};
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
        <ShowcaseRegion regions={["row"]}>
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>Default primitive</ShowcaseRegion>
            <ShowcaseRegion regions={["control-list"]}>
              <ToggleRow label="Primary option" colorToken="--accent" meta={4}/>
              <ToggleRow label="Successful option" colorToken="--success" meta={12}/>
              <ToggleRow label="Muted option" colorToken="--text-muted" on={false} meta={24}/>
            </ShowcaseRegion>
          </ShowcaseRegion>
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>Line primitive</ShowcaseRegion>
            <ShowcaseRegion regions={["control-list"]}>
              <ToggleRow line label="Primary line" colorToken="--accent" on/>
              <ToggleRow line label="Success line" colorToken="--success" on/>
              <ToggleRow line label="Muted line" colorToken="--text-muted" on={false}/>
              <ToggleRow line label="Neutral line" colorToken="--border-default" on/>
            </ShowcaseRegion>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
