import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import * as d3 from "d3";
import * as d3Sankey from "d3-sankey";
import { App } from "./App";
import "../design-system/styles.css";

type MermaidApi = typeof import("mermaid").default;

declare global {
  interface Window {
    d3?: typeof d3 & typeof d3Sankey;
    mermaid?: MermaidApi;
  }
}

const bundledD3 = { ...d3, ...d3Sankey };
window.d3 = bundledD3;

const container = document.getElementById("root");
if (!container) {
  throw new Error("Reqvire Explorer: #root mount point not found");
}

function loadProjectStoreSeed() {
  return new Promise<void>((resolve) => {
    if (window.reqvireProjectStore !== undefined) {
      resolve();
      return;
    }

    const candidates = projectStoreSeedCandidates();
    let index = 0;

    function tryNext() {
      if (window.reqvireProjectStore !== undefined || index >= candidates.length) {
        resolve();
        return;
      }

      const script = document.createElement("script");
      script.src = candidates[index];
      script.async = false;
      script.onload = () => resolve();
      script.onerror = () => {
        script.remove();
        index += 1;
        tryNext();
      };
      document.head.appendChild(script);
    }

    tryNext();
  });
}

function projectStoreSeedCandidates() {
  const candidates = new Set<string>();
  const moduleUrl = new URL(import.meta.url);

  if (moduleUrl.pathname.includes("/assets/")) {
    candidates.add(new URL("project-store.js", moduleUrl).toString());
  }

  candidates.add(new URL("assets/project-store.js", document.baseURI).toString());
  candidates.add(new URL("assets/project-store.js", window.location.href).toString());
  candidates.add("assets/project-store.js");

  return [...candidates];
}

loadProjectStoreSeed().then(() => {
  createRoot(container).render(
    <StrictMode>
      <App />
    </StrictMode>,
  );
});
