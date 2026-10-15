import { ShowcaseRegion, SourcePageElements, SourcePageElement } from "@ds";

import { fixture } from "./SourcePageElements.fixture";
export const scenario = {"title": "SourcePageElements", "covers": ["SourcePageElements", "SourcePageElement"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<SourcePageElements><SourcePageElement title="Release traceability" elementType="requirement" typeFamily="requirement"><p>Approved requirements retain their evidence links.</p></SourcePageElement></SourcePageElements>
</ShowcaseRegion>;
}
