import type { Plugin } from "vite";

// Keep the app, standalone DS builds and DOM tests on one Preact runtime.
// These aliases support React-only dependencies such as ReactFlow and Markdown;
// project-owned components import Preact and its hooks directly.
export const preactAliases = [
  { find: /^react-dom\/test-utils$/, replacement: "preact/test-utils" },
  { find: /^react-dom(?:\/client)?$/, replacement: "preact/compat" },
  { find: /^react\/jsx-dev-runtime$/, replacement: "preact/jsx-dev-runtime" },
  { find: /^react\/jsx-runtime$/, replacement: "preact/jsx-runtime" },
  { find: /^react$/, replacement: "preact/compat" },
];

export function preactRuntimeGuard(): Plugin {
  return {
    name: "reqvire-preact-runtime",
    generateBundle() {
      const modules = [...this.getModuleIds()];
      const react = modules.filter(id => /\/node_modules\/(?:react|react-dom)\/cjs\//.test(id.replaceAll("\\", "/")));
      if (react.length) this.error(`React renderer entered the Preact build: ${react.join(", ")}`);
      const cores = modules.filter(id => /\/preact\/dist\/preact\.(?:m?js)$/.test(id.replaceAll("\\", "/")));
      if (cores.length > 1) this.error(`Multiple Preact runtimes entered the build: ${cores.join(", ")}`);
    },
  };
}
