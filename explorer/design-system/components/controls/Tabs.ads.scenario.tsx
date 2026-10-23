import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";
import { Icon, Tabs } from "@ds";
import { fixture } from "./Tabs.ads.fixture";
export const scenario = {"title":"Tabs",covers: ["Tabs","Icon"]};
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
const [tabUnderline, setTabUnderline] = useState("model");
const [tabPill, setTabPill] = useState("grid");
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Underline</ShowcaseRegion>
          <Tabs items={[
        { value: "model", label: "Model", icon: <Icon name="folder" size={14}/> },
        { value: "graph", label: "Knowledge Graph", icon: <Icon name="network" size={14}/> },
        { value: "ontologies", label: "Ontologies", icon: <Icon name="globe" size={14}/> },
        { value: "search", label: "Search", icon: <Icon name="search" size={14}/> },
        { value: "traces", label: "Traces", icon: <Icon name="activity" size={14}/>, badge: 145 },
    ]} value={tabUnderline} onChange={setTabUnderline}/>
        </ShowcaseRegion>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Pill</ShowcaseRegion>
          <Tabs variant="pill" items={[
        { value: "grid", label: "Grid" },
        { value: "list", label: "List" },
        { value: "sunburst", label: "Sunburst" },
    ]} value={tabPill} onChange={setTabPill}/>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
