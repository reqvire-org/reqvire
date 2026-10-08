import { withBrowser, waitFor } from "../../browser.mjs";

const [baseUrl, profile] = process.argv.slice(2);
await withBrowser(profile, async ({ evaluate, navigate: navigatePage, rpc }) => {
  await rpc("Page.addScriptToEvaluateOnNewDocument", { source: `(${(() => {
    const NativeWorker = window.Worker;
    window.__forceAtlasWorkers = { active: 0, maximum: 0, started: 0 };
    window.Worker = class extends NativeWorker {
      constructor(url, options) {
        super(url, options);
        this.tracked = String(url).includes("forceAtlasLayout.worker-");
        if (this.tracked) {
          window.__forceAtlasWorkers.started++;
          window.__forceAtlasWorkers.active++;
          window.__forceAtlasWorkers.maximum = Math.max(window.__forceAtlasWorkers.maximum, window.__forceAtlasWorkers.active);
        }
      }
      terminate() {
        if (this.tracked) { window.__forceAtlasWorkers.active--; this.tracked = false; }
        return super.terminate();
      }
    };
  }).toString()})();` });
  let navigationCount = 0;
  async function navigate(route) {
    await navigatePage(`${baseUrl}/?route-e2e=${++navigationCount}${route}`);
    await evaluate(() => new Promise(resolve => setTimeout(resolve, 100)));
  }
  let failed = false;
  async function run(label, check) {
    try {
      if (!await check()) throw new Error("Expected fixture record did not render");
      console.log(`${label}: PASS`);
    } catch (error) {
      failed = true;
      console.log(`${label}: FAIL`);
      console.error(`${label}: ${error.message}`);
      console.error(await evaluate(() => document.body.innerText.slice(0, 500)));
    }
  }
  const unicodeFile = "specifications/Résumé.md";
  const literalFile = "specifications/Literal%20Name.md";
  const cases = [
    ["elements", `${unicodeFile}#café`, "unicode-route-body"],
    ["files", unicodeFile, "Café"],
    ["content", `${unicodeFile}#café`, "unicode-route-body"],
    ["resources", "resource:scripts/Café Evidence.txt", "unicode-route-evidence"],
  ];
  await run("ASCII element", async () => {
    await navigate("#/elements/specifications/Requirements.md#test-requirement-one");
    return evaluate(() => document.querySelector('[role="dialog"]')?.textContent.includes("Test Requirement One"));
  });
  for (const encoded of [false, true]) {
    for (const [kind, parameter, marker] of cases) {
      await run(`${kind} Unicode ${encoded ? "encoded" : "raw"}`, async () => {
        await navigate(`#/${kind}/${encoded ? encodeURIComponent(parameter) : parameter}`);
        return evaluate((kind, marker) => {
          const scope = kind === "elements"
            ? document.querySelector('[role="dialog"]')
            : kind === "files"
              ? document.querySelector('[data-view="files"] .ux-file-browser__elements')
              : document.querySelector('[data-product-pattern="app-shell"]');
          return scope?.textContent.includes(marker);
        }, kind, marker);
      });
    }
  }
  for (const [kind, parameter, marker] of [
    ["elements", `${literalFile}#literal-percent-requirement`, "literal-percent-route-body"],
    ["content", literalFile, "literal-percent-route-body"],
    ["resources", "resource:scripts/Literal%20Evidence.txt", "literal-percent-route-evidence"],
  ]) {
    await run(`${kind} literal percent`, async () => {
      await navigate(`#/${kind}/${encodeURIComponent(parameter)}`);
      return evaluate(marker => document.body.textContent.includes(marker), marker);
    });
  }
  await run("literal percent generated navigation", async () => {
    await navigate(`#/files/${encodeURIComponent(literalFile)}`);
    return evaluate(async () => {
      const settle = () => new Promise(resolve => setTimeout(resolve, 100));
      const button = () => [...document.querySelectorAll("button.ux-file-browser__element-row")]
        .find(candidate => candidate.textContent.includes("Literal Percent Requirement"));
      if (!button()) return false;
      button().click();
      await settle();
      const dialog = () => document.querySelector('[role="dialog"]');
      if (!dialog()?.textContent.includes("literal-percent-route-body")) return false;
      dialog().querySelector('button[aria-label="Close"]').click();
      await settle();
      if (dialog() || !button()) return false;
      button().click();
      await settle();
      const source = [...dialog().querySelectorAll("a")]
        .find(link => link.textContent.includes("Open source page"));
      if (!source) return false;
      source.click();
      await settle();
      return !dialog() && document.body.textContent.includes("literal-percent-route-body");
    });
  });
  await run("malformed percent fallback", async () => {
    await navigate("#/content/specifications/broken%ZZ.md");
    return evaluate(() => document.body.textContent.includes("File not found: specifications/broken%ZZ.md"));
  });
  await run("Model Flow packaged worker", async () => {
    await navigate("#/model");
    await evaluate(() => {
      const button = [...document.querySelectorAll('[aria-label="Model layout"] button')]
        .find(button => button.textContent.trim() === "Flow");
      if (!button) throw new Error("Flow mode is missing");
      button.click();
    });
    const ready = () => evaluate(() => {
      const flow = document.querySelector('[aria-label="Model flow"]');
      if (flow?.textContent.includes("Retry layout")) throw new Error("Flow worker failed");
      return flow?.getAttribute("aria-busy") === "false" && flow.querySelectorAll(".react-flow__node").length > 0;
    });
    await waitFor(ready);
    const original = await evaluate(() => [...document.querySelectorAll(".react-flow__node")].map(node => node.dataset.id).sort());
    await evaluate(() => [...document.querySelectorAll('[aria-label="Flow direction"] button')]
      .find(button => button.textContent === "Left to right").click());
    await waitFor(ready);
    return evaluate(original => {
      const ids = [...document.querySelectorAll(".react-flow__node")].map(node => node.dataset.id).sort();
      const workers = performance.getEntriesByType("resource").filter(entry => entry.name.includes("flowLayout.worker-"));
      return JSON.stringify(ids) === JSON.stringify(original) && workers.length > 0
        && workers.every(entry => new URL(entry.name).origin === location.origin);
    }, original);
  });
  for (const kind of ["Model Graph", "Ontologies"]) {
    await run(`${kind} packaged ForceAtlas worker`, async () => {
      await navigate(kind === "Model Graph" ? "#/model" : "#/ontologies");
      if (kind === "Model Graph") await evaluate(() => [...document.querySelectorAll('[aria-label="Model layout"] button')]
        .find(button => button.textContent.trim() === "Graph").click());
      const ready = () => evaluate(() => {
        if (document.body.textContent.includes("layout failed.")) throw new Error("ForceAtlas worker failed");
        const surface = document.querySelector('[data-testid="kg-sigma-canvas"], #ontology-graph-container');
        return Boolean(surface?.querySelector("canvas")) && surface.closest('[aria-busy]')?.getAttribute("aria-busy") === "false";
      });
      await waitFor(ready);
      if (kind === "Ontologies") {
        await evaluate(() => { for (let i = 0; i < 10; i++) window.resetOntologyGraphLayout(); });
        await waitFor(ready);
      }
      const result = await evaluate(() => {
        const workers = performance.getEntriesByType("resource").filter(entry => entry.name.includes("forceAtlasLayout.worker-"));
        return workers.length > 0 && workers.every(entry => new URL(entry.name).origin === location.origin)
          && window.__forceAtlasWorkers.started > 0 && window.__forceAtlasWorkers.maximum === 1 && window.__forceAtlasWorkers.active === 0;
      });
      await evaluate(() => { location.hash = "#/files"; });
      await waitFor(() => evaluate(() => !document.querySelector("canvas")));
      return result && await evaluate(() => window.__forceAtlasWorkers.active === 0);
    });
  }
  console.log("completed");
  if (failed) process.exitCode = 1;
});
