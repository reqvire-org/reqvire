import { render } from "preact";
import { App } from "./App";
import { documentClasses } from "@ds";
import "../design-system/styles.css";

type MermaidApi = typeof import("mermaid").default;

declare global {
  interface Window {
    mermaid?: MermaidApi;
  }
}

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
  if (window.reqvireWorktreeRouting) {
    const seed = new URL("/assets/project-store.js", window.location.href);
    const branch = new URL(window.location.href).searchParams.get("branch");
    if (branch !== null) seed.searchParams.set("branch", branch);
    return [seed.toString()];
  }
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
  document.body.classList.add(...documentClasses().split(" "));
  render(<App />, container);
});
