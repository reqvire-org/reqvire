import { ShowcaseRegion, DetailSection } from "@ds";

import { fixture } from "./DetailSection.fixture";
export const scenario = {"title": "DetailSection", "covers": ["DetailSection"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<DetailSection title="Relations"><p>Verified by the release coverage check.</p></DetailSection>
</ShowcaseRegion>;
}
