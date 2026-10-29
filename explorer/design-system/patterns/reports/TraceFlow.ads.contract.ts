import type { UnitContract } from "@reqvire-org/ads/contracts";
import { TraceFlow } from "./TraceFlow";

export default {
  schemaVersion: 1,
  behavior: {
    subjects: { flow: { api: TraceFlow } },
    guarantees: [
      {
        id: "measured-layout",
        description: "The real worker lays out all fixture nodes at the shared CSS dimensions, and changing direction changes their layout axis.",
        subjects: ["flow"],
        checks: ["trace-flow-layout"],
      },
      {
        id: "expanded-lifecycle",
        description: "Expanding and closing the flow preserves its mounted node elements and restores focus to the expand control.",
        subjects: ["flow"],
        checks: ["trace-flow-expand"],
      },
    ],
    checks: {
      "trace-flow-layout": { file: "./TraceFlow.ads.behavior.spec.ts", case: "trace-flow-layout", scenario: "patterns/reports/TraceFlow", profile: "widget" },
      "trace-flow-expand": { file: "./TraceFlow.ads.behavior.spec.ts", case: "trace-flow-expand", scenario: "patterns/reports/TraceFlow", profile: "widget" },
    },
  },
} satisfies UnitContract;
