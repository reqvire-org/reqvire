/**
 * MockShell — renders the real App component with the selected showcase store.
 * Zero differences from the running explorer: same components, same CSS, same providers.
 * Injecting devFixture into window.reqvireProjectStore makes loadStore work in any build mode.
 */
import { App } from "../../src/App";
import { devFixture } from "../../src/store/devFixture";
import type { ExplorerProjectStore } from "../../src/store/types";
import scopedCoverage from "../../src/store/fixtures/scopedCoverage.json";

export function MockShell({ example = "model" }: { example?: "model" | "coverage" }) {
  // The keyed mock mounts a fresh App; its normal loader reads this seed on mount.
  window.reqvireProjectStore = example === "coverage" ? scopedCoverage as ExplorerProjectStore : devFixture;
  return <App />;
}
