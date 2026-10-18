import type { UnitContract } from "@reqvire-org/ads/contracts";
import { WorktreeSelector } from "./WorktreeSelector";

export default {
  schemaVersion: 1,
  behavior: {
    subjects: { selector: { api: WorktreeSelector } },
    guarantees: [
      {
        id: "keyboard-selection",
        description: "Keyboard navigation selects an available branch, updates the controlled trigger and restores its focus.",
        subjects: ["selector"],
        checks: ["worktree-keyboard"],
      },
      {
        id: "pointer-and-dismissal",
        description: "Unavailable branches cannot be selected; pointer selection updates the value, and Escape and outside clicks dismiss the list.",
        subjects: ["selector"],
        checks: ["worktree-pointer"],
      },
    ],
    checks: {
      "worktree-keyboard": { file: "./WorktreeSelector.ads.behavior.spec.ts", case: "worktree-keyboard", scenario: "components/controls/WorktreeSelector", profile: "widget" },
      "worktree-pointer": { file: "./WorktreeSelector.ads.behavior.spec.ts", case: "worktree-pointer", scenario: "components/controls/WorktreeSelector", profile: "widget" },
    },
  },
} satisfies UnitContract;
