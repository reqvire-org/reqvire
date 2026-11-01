import foundationCss from "../../design-system/styles.css?inline";
import postcss from "postcss";
import { beforeEach } from "vitest";

// jsdom 25 cannot parse cascade layers. Load the actual Vite-processed
// foundation, unwrapping its single base layer for this test environment.
// Browser builds keep the original layer and cascade unchanged.
const foundation = postcss.parse(foundationCss);
foundation.walkAtRules("layer", (layer) => {
  if (layer.params !== "base" || !layer.nodes) throw new Error("Test foundation adapter only supports the base layer.");
  layer.replaceWith(...layer.nodes);
});
const stylesheet = document.createElement("style");
stylesheet.textContent = foundation.toString();
document.head.append(stylesheet);

beforeEach(() => {
  document.documentElement.removeAttribute("data-theme");
});

if (!globalThis.ResizeObserver) {
  globalThis.ResizeObserver = class ResizeObserver {
    observe() {}
    unobserve() {}
    disconnect() {}
  };
}
