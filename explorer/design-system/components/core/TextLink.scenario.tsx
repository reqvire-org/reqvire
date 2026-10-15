import { ShowcaseRegion, TextLink } from "@ds";

import { fixture } from "./TextLink.fixture";
export const scenario = {"title": "TextLink", "covers": ["TextLink"]};
export default function Scenario() {

return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<TextLink href="#preview">Open model source</TextLink>
</ShowcaseRegion>;
}
