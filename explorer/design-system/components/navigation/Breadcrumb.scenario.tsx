import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds";
import { Breadcrumb } from "@ds";
import { fixture } from "./Breadcrumb.fixture";
export const scenario = {"title":"Breadcrumb","covers":["Breadcrumb"]};
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
          <ShowcaseRegion regions={["label"]}>File path</ShowcaseRegion>
          <Breadcrumb items={[
        { label: "requirements", onClick: () => { } },
        { label: "SystemRequirements.md", onClick: () => { } },
        { label: "Model Structure Specification" },
    ]}/>
        </ShowcaseRegion>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Short path</ShowcaseRegion>
          <Breadcrumb items={[
        { label: "requirements", onClick: () => { } },
        { label: "Capabilities.md" },
    ]}/>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
