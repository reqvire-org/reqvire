import { ModelModeSelector } from "@ds";
import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";
import type { ModelScreenMode } from "@ds";
import { fixture } from "./ModelModeSelector.ads.fixture";
export const scenario = {"title": "ModelModeSelector", covers: ["ModelModeSelector"]};
export default function Scenario() {
const [mode,setMode] = useState<ModelScreenMode>("grid");
return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<ModelModeSelector value={mode} onChange={setMode} /><p>Selected layout: {mode}</p>
</ShowcaseRegion>;
}
