import { TokenSamples } from "@ds";
import { fixture } from "./Typography.fixture";
export const scenario = { title: fixture.title, covers: ["TokenSamples"], tokens: fixture.tokens };
export default function Scenario() { return <TokenSamples {...fixture} />; }
