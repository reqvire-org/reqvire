import { ShowcaseRegion, ShowcaseHeader, ShowcaseNavigation } from "@ds";
import { useState } from "preact/hooks";

import { fixture } from "./Showcase.fixture";
export const scenario = {"title": "Showcase", "covers": ["ShowcaseHeader", "ShowcaseNavigation"]};
export default function Scenario() {
const [tab,setTab] = useState("components"); const [theme,setTheme] = useState<"light"|"dark">("light");
return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<ShowcaseHeader title="Review frame" navigation={<ShowcaseNavigation items={fixture.items} value={tab} onChange={setTab} />} theme={theme} onThemeChange={setTheme} /><p>Selected review section: {tab}</p>
</ShowcaseRegion>;
}
