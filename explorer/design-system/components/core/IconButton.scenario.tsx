import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds/review";
import { Icon, IconButton } from "@ds";
import { fixture } from "./IconButton.fixture";
export const scenario = {"title":"IconButton",covers: ["IconButton","Icon"]};
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
        <ShowcaseRegion regions={["row", "row--center"]}>
          {(["secondary", "ghost"] as const).map((tone) => (<ShowcaseRegion key={tone} regions={["col"]}>
              <ShowcaseRegion regions={["label"]}>{tone}</ShowcaseRegion>
              <ShowcaseRegion regions={["row", "row--center"]}>
                <IconButton tone={tone} size="sm" aria-label="settings sm"><Icon name="settings"/></IconButton>
                <IconButton tone={tone} size="md" aria-label="settings md"><Icon name="settings"/></IconButton>
              </ShowcaseRegion>
            </ShowcaseRegion>))}
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>active</ShowcaseRegion>
            <IconButton tone="ghost" aria-label="active" aria-pressed="true"><Icon name="filter"/></IconButton>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
