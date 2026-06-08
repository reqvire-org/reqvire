import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import * as d3 from "d3";
import * as d3Sankey from "d3-sankey";
import { App } from "./App";
import "./design-system.css";
import "./styles.css";

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
    const script = document.createElement("script");
    script.src = "assets/project-store.js";
    script.onload = () => resolve();
    script.onerror = () => resolve();
    document.head.appendChild(script);
  });
}

loadProjectStoreSeed().then(() => {
  createRoot(container).render(
    <StrictMode>
      <App />
    </StrictMode>,
  );
});
