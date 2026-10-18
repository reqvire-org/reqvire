import type { UnitContract } from "@reqvire-org/ads/contracts";
import { Toaster } from "./Toaster";

export default {
  schemaVersion: 1,
  behavior: {
    subjects: { toaster: { api: Toaster } },
    guarantees: [
      {
        id: "actions-and-dismissal",
        description: "Toast actions call their callback and dismiss; the close control dismisses a persistent notification.",
        subjects: ["toaster"],
        checks: ["toast-actions"],
      },
      {
        id: "promise-and-focus",
        description: "A loading notification is updated in place on success; Alt+T focuses the stack and Escape returns focus to the initiating control.",
        subjects: ["toaster"],
        checks: ["toast-promise-focus"],
      },
    ],
    checks: {
      "toast-actions": { file: "./Toaster.ads.behavior.spec.ts", case: "toast-actions", scenario: "components/core/Toast", profile: "widget" },
      "toast-promise-focus": { file: "./Toaster.ads.behavior.spec.ts", case: "toast-promise-focus", scenario: "components/core/Toast", profile: "widget" },
    },
  },
} satisfies UnitContract;
