import { TokenSamples } from "@ds";
import { fixture } from "./Status.fixture";
export const scenario = { title: fixture.title, covers: ["TokenSamples"], samples: fixture.samples };
export default function Scenario() { return <TokenSamples {...fixture} />; }
