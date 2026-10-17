import { RelationPill } from "@ds";
import { ShowcaseRegion } from "@ds/review";

import { fixture } from "./RelationPill.fixture";
export const scenario = {"title": "RelationPill", covers: ["RelationPill"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<RelationPill label="derivedFrom" /><RelationPill label="verifiedBy" />
</ShowcaseRegion>;
}
