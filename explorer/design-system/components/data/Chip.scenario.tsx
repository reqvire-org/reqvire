import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds";
import { Chip } from "@ds";
import { fixture } from "./Chip.fixture";
export const scenario = {"title":"Chip","covers":["Chip"]};
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
          <Chip>All types</Chip>
          <Chip active>Requirement</Chip>
          <Chip>Verification</Chip>
          <Chip>Capability</Chip>
          <Chip>Contract</Chip>
          <Chip>12 results</Chip>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
