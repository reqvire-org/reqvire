import { toastRules } from "../design-system/components/core/toast/toast.oxlint.mjs";

// Tooling entry only: component-owned rules never enter the runtime public barrel.
export default {
  meta: { name: "design-system" },
  rules: { ...toastRules },
};
