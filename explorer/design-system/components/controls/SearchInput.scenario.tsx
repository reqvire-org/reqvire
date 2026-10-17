import type { ComponentChildren } from "preact";
import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";
import { SearchInput } from "@ds";
import { fixture } from "./SearchInput.fixture";
export const scenario = {"title":"SearchInput",covers: ["SearchInput"]};
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
const [query, setQuery] = useState("");
const [queryLg, setQueryLg] = useState("");
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Default (md)</ShowcaseRegion>
          <ShowcaseRegion regions={["input-demo", "input-demo--md"]}>
            <SearchInput value={query} onInput={(e) => setQuery(e.currentTarget.value)} placeholder="Search elements, files, relations…"/>
          </ShowcaseRegion>
        </ShowcaseRegion>
        <ShowcaseRegion regions={["col"]}>
          <ShowcaseRegion regions={["label"]}>Large (lg)</ShowcaseRegion>
          <ShowcaseRegion regions={["input-demo", "input-demo--lg"]}>
            <SearchInput size="lg" value={queryLg} onInput={(e) => setQueryLg(e.currentTarget.value)} placeholder="Search the project store…"/>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
