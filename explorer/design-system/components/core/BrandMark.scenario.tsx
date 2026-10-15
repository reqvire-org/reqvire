import { ShowcaseRegion, BrandMark } from "@ds";

import { fixture } from "./BrandMark.fixture";
export const scenario = {"title": "BrandMark", "covers": ["BrandMark"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<BrandMark alt="Reqvire brand mark" />
</ShowcaseRegion>;
}
