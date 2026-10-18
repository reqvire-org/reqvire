import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds/review";
import { Card } from "@ds";
import { fixture } from "./Card.ads.fixture";
export const scenario = {"title":"Card",covers: ["Card"]};
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
          <ShowcaseRegion regions={["card-demo"]} asChild><Card>
            <ShowcaseRegion regions={["card-demo__title"]}>Default card</ShowcaseRegion>
            <ShowcaseRegion regions={["card-demo__desc"]}>Static surface</ShowcaseRegion>
          </Card></ShowcaseRegion>
          <ShowcaseRegion regions={["card-demo"]} asChild><Card interactive>
            <ShowcaseRegion regions={["card-demo__title"]}>Interactive</ShowcaseRegion>
            <ShowcaseRegion regions={["card-demo__desc"]}>Hover to see lift</ShowcaseRegion>
          </Card></ShowcaseRegion>
          <ShowcaseRegion regions={["card-demo"]} asChild><Card interactive selected>
            <ShowcaseRegion regions={["card-demo__title"]}>Selected</ShowcaseRegion>
            <ShowcaseRegion regions={["card-demo__desc"]}>Accent ring</ShowcaseRegion>
          </Card></ShowcaseRegion>
          <ShowcaseRegion regions={["card-demo"]} asChild><Card accentColorToken="--color-role-requirement">
            <ShowcaseRegion regions={["card-demo__title"]}>Accent bar</ShowcaseRegion>
            <ShowcaseRegion regions={["card-demo__desc"]}>Left color rail</ShowcaseRegion>
          </Card></ShowcaseRegion>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
