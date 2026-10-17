import { TokenSamples } from "@ds/review";
import { fixture } from "./BordersAccent.fixture";
export const scenario = { title: fixture.title, covers: ["TokenSamples"], samples: fixture.samples };
export default function Scenario() { return <TokenSamples {...fixture} />; }
