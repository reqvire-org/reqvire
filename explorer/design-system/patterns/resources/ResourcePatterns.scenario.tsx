import { ShowcaseRegion } from "@ds/review";
import { type ComponentChildren } from "preact";
import { ResourceDetailCard, ResourceEmptyState, ResourcePageHeader, ResourceList } from "@ds";
import { fixture } from "./ResourcePatterns.fixture";
export const scenario = {"title":"Resources",covers: ["ResourcePageHeader","ResourceList","ResourceDetailCard","ResourceEmptyState"]};
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
        <ResourcePageHeader count={2}/>
        <ResourceList resources={[
        { id: "fixture-source", kind: "file", display: "Coverage implementation", target: "src/lib/coverage.ts", relationTypes: ["satisfiedBy"] },
        { id: "fixture-reference", kind: "url", display: "Reqvire documentation", target: "https://reqvire.com", externalUrl: "https://reqvire.com" },
    ]}/>
        <ResourceDetailCard kind="file" target="src/lib/coverage.ts"/>
        <ResourceEmptyState>No resources in this scope.</ResourceEmptyState>
      </Section>
</ShowcaseRegion>;
}
