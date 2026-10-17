import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds/review";
import { Button, Icon } from "@ds";
import { fixture } from "./Button.fixture";
export const scenario = {"title":"Button",covers: ["Button","Icon"]};
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
        {(["primary", "accent", "secondary", "ghost", "danger", "link"] as const).map((tone) => (<ShowcaseRegion key={tone} regions={["col"]}>
            <ShowcaseRegion regions={["label"]}>{tone}</ShowcaseRegion>
            <ShowcaseRegion regions={["row", "row--center"]}>
              {(["sm", "md", "lg"] as const).map((size) => (<Button key={size} tone={tone} size={size}>{size}</Button>))}
              <Button tone={tone} disabled>disabled</Button>
              <Button tone={tone} iconLeft={<Icon name="plus"/>}>with icon</Button>
            </ShowcaseRegion>
          </ShowcaseRegion>))}
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>block</ShowcaseRegion>
          <Button tone="secondary" block iconLeft={<Icon name="download"/>}>Full-width button</Button>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
