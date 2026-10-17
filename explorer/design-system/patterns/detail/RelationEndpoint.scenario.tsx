import { RelationEndpoint } from "@ds";
import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";

import { fixture } from "./RelationEndpoint.fixture";
export const scenario = {"title": "RelationEndpoint", covers: ["RelationEndpoint"]};
export default function Scenario() {
const [selected,setSelected] = useState("none");
return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<RelationEndpoint endpoint={fixture.endpoint} onOpenElement={setSelected} /><p>Selected: {selected}</p>
</ShowcaseRegion>;
}
