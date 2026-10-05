import ELK from "elkjs/lib/elk.bundled.js";
import type { FlowLayoutEngine } from "@ds";

// Only for jsdom interaction tests: browser worker lifetime is tested separately.
const engine = new ELK({ algorithms: ["layered"] });
export const testFlowLayoutEngine: FlowLayoutEngine = input => ({
  result: engine.layout(input), cancel: () => {},
});
