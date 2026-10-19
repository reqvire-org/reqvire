import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";
import { Icon, SegmentedControl } from "@ds";
import { fixture } from "./SegmentedControl.ads.fixture";
export const scenario = {"title":"SegmentedControl",covers: ["SegmentedControl","Icon"]};
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
const [view, setView] = useState<"list" | "grid">("list");
const [mode, setMode] = useState<"model" | "graph" | "traces">("model");
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Text only</ShowcaseRegion>
          <SegmentedControl items={[
        { value: "model" as const, label: "Model" },
        { value: "graph" as const, label: "Graph" },
        { value: "traces" as const, label: "Traces" },
    ]} value={mode} onChange={setMode} ariaLabel="View mode"/>
        </ShowcaseRegion>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>With icons</ShowcaseRegion>
          <SegmentedControl items={[
        { value: "list" as const, label: "List", icon: <Icon name="list" size={14}/> },
        { value: "grid" as const, label: "Grid", icon: <Icon name="grid" size={14}/> },
    ]} value={view} onChange={setView} ariaLabel="Layout mode"/>
        </ShowcaseRegion>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Icons with tooltips</ShowcaseRegion>
          <SegmentedControl items={[
        { value: "list" as const, icon: <Icon name="list" />, tooltip: "List" },
        { value: "grid" as const, icon: <Icon name="grid" />, tooltip: "Grid" },
    ]} value={view} onChange={setView} ariaLabel="Icon layout mode"/>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
