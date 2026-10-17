import { KnowledgeGraphCanvas } from "@ds";
import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";

import { fixture } from "./KnowledgeGraphCanvas.fixture";
export const scenario = {"title": "KnowledgeGraphCanvas", covers: ["KnowledgeGraphCanvas"]};
export default function Scenario() {
const [selected,setSelected] = useState<string|null>(null);
return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<KnowledgeGraphCanvas graphData={fixture.graph} contextKey="graph-preview" activeTypes={new Set(fixture.types)} activeOverlays={new Set()} selectedId={selected} onSelectNode={setSelected} />
</ShowcaseRegion>;
}
