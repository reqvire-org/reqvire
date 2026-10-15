import { TokenSamples } from "@ds";
import { fixture } from "./Elevation.fixture";
export const scenario = { title: fixture.title, covers: ["TokenSamples"], tokens: fixture.tokens };
export default function Scenario() { return <TokenSamples {...fixture} />; }
