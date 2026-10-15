import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds";
import { Badge, Button } from "@ds";
import { fixture } from "./Badge.fixture";
export const scenario = {"title":"Badge","covers":["Badge","Button"]};
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
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>variants</ShowcaseRegion>
            <ShowcaseRegion regions={["row", "row--center"]}>
              <Badge>12</Badge>
              <Badge variant="accent">5</Badge>
              <Badge variant="solid">64</Badge>
              <Badge variant="dot"/>
            </ShowcaseRegion>
          </ShowcaseRegion>
          <ShowcaseRegion regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>in context</ShowcaseRegion>
            <ShowcaseRegion regions={["row", "row--center"]}>
              <Button tone="secondary" iconRight={<Badge>3</Badge>}>Pending</Button>
              <Button tone="ghost">Relations <Badge variant="solid">14</Badge></Button>
            </ShowcaseRegion>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
