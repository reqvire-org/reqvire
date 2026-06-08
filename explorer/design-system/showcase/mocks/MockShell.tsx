/**
 * MockShell — renders the real App component with devFixture data.
 * Zero differences from the running explorer: same components, same CSS, same providers.
 * Injecting devFixture into window.reqvireProjectStore makes loadStore work in any build mode.
 */
import { App } from "../../../src/App";
import { devFixture } from "../../../src/store/devFixture";

if (typeof window !== "undefined") {
  window.reqvireProjectStore ??= devFixture;
}

export function MockShell() {
  return <App />;
}
