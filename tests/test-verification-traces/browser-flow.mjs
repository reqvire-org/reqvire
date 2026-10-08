import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { withBrowser, waitFor } from "../browser.mjs";

const [served, exported, profile, reportPath] = process.argv.slice(2);
const report = JSON.parse(await readFile(reportPath, "utf8"));
const trace = Object.values(report.files)[0].verifications[0];
const expectedNodes = [trace.identifier, ...trace.trace_graph.nodes.map(node => node.id)].sort();
const expectedEdges = [...new Set([
  ...trace.trace_graph.edges.map(edge => JSON.stringify([edge.source, edge.target, edge.relation_type])),
  ...trace.trace_graph.nodes.filter(node => node.is_directly_verified).map(node => JSON.stringify([trace.identifier, node.id, "verifies"])),
])].sort();
await withBrowser(profile, async ({ evaluate, loadPage: loadDocument, reloadPage, rpc }) => {
  // Copied links open a fresh document, including when only the hash differs.
  const loadPage = async url => { await loadDocument("about:blank"); await loadDocument(url); };
  await rpc("Page.addScriptToEvaluateOnNewDocument", { source: `
    window.traceWorkerUrls = [];
    const OriginalWorker = window.Worker;
    window.Worker = class extends OriginalWorker {
      constructor(url, options) { super(url, options); window.traceWorkerUrls.push(String(url)); }
    };
    window.traceDocument = Math.random();
  ` });
  async function ready() {
    await waitFor(() => evaluate(() => document.querySelector('[aria-label="Verification trace flow"]')?.getAttribute("aria-busy") === "false"));
  }
  async function click(label) {
    await evaluate(label => {
      const button = [...document.querySelectorAll("button")].find(button => button.getAttribute("aria-label") === label || button.textContent.trim() === label);
      if (!button) throw new Error(`Missing button ${label}`);
      button.click();
    }, label);
  }
  async function graphMatches() {
    const actual = await evaluate(() => {
      const region = document.querySelector('[aria-label="Verification trace flow"]');
      return { nodes: [...region.querySelectorAll(".react-flow__node")].map(node => node.dataset.id).sort(),
        edges: [...region.querySelectorAll(".react-flow__edge")].map(edge => edge.dataset.id).sort(),
        labels: [...region.querySelectorAll(".react-flow__edge text")].map(edge => edge.textContent),
        summary: region.querySelector(".ux-trace-flow-summary").textContent,
        mermaid: Boolean(region.querySelector(".mermaid")),
      };
    });
    assert.deepEqual(actual.nodes, expectedNodes);
    assert.deepEqual(actual.edges, expectedEdges);
    assert.equal(actual.labels.length, expectedEdges.length);
    assert(actual.labels.includes("specify"));
    assert(actual.summary.includes(`${trace.directly_verified_count} directly verified`));
    assert(actual.summary.includes(`${trace.total_requirements_in_tree} requirements in trace`));
    assert.equal(actual.mermaid, false);
  }
  for (const [kind, base] of [["served", served], ["exported", exported]]) {
    await loadPage(`${base}/#/traces`); await ready();
    const documentId = await evaluate(() => window.traceDocument);
    const selectionHash = await evaluate(() => location.hash);
    assert.equal(new URLSearchParams(selectionHash.split("?")[1]).get("selected"), trace.identifier);
    assert.deepEqual(await evaluate(() => window.reqvireProjectStore.traces), report);
    await graphMatches();
    const urls = await evaluate(() => window.traceWorkerUrls.filter(url => url.includes("flowLayout.worker")));
    assert(urls.length > 0, "production trace must start the packaged worker");
    for (const url of urls) {
      assert.equal(new URL(url, base).origin, new URL(base).origin);
      assert.equal((await fetch(new URL(url, base))).status, 200);
    }
    await click("Top to bottom"); await ready(); await graphMatches();
    await click("Left to right"); await ready(); await graphMatches();
    console.log(`PASS ${kind}/native-trace-projection-and-worker`);

    const id = trace.trace_graph.nodes.find(node => node.type === "capability").id;
    await evaluate(id => {
      const name = [...document.querySelectorAll(".ux-trace-flow-node-identity")].find(link => link.getAttribute("href") === `#/elements/${encodeURI(id)}`);
      if (!name) throw new Error("Missing canonical capability link");
      name.click();
    }, id);
    await waitFor(() => evaluate(id => location.hash === `#/elements/${encodeURI(id)}` && Boolean(document.querySelector('[role="dialog"]')), id));
    await click("Close"); await ready();
    assert.equal(await evaluate(() => location.hash), selectionHash);
    await evaluate(id => [...document.querySelectorAll(".ux-trace-flow-node-identity")].find(link => link.getAttribute("href") === `#/elements/${encodeURI(id)}`).click(), id);
    await waitFor(() => evaluate(() => Boolean(document.querySelector('[role="dialog"]'))));
    await evaluate(() => history.back());
    await waitFor(() => evaluate(hash => location.hash === hash && !document.querySelector('[role="dialog"]'), selectionHash));
    await evaluate(() => document.querySelector(".ux-trace-flow-node-source").click());
    await waitFor(() => evaluate(file => location.hash === `#/content/${encodeURI(file)}` && document.body.textContent.includes("Fixture cap-root."), trace.file));
    await click("Traces"); await ready(); await graphMatches();
    assert.equal(await evaluate(() => window.traceDocument), documentId);
    console.log(`PASS ${kind}/trace-detail-source-and-return-routes`);

    const overviewWorkers = await evaluate(() => window.traceWorkerUrls.length);
    await evaluate(file => {
      const label = [...document.querySelectorAll('[aria-label="Verification trace tree"] .ds-treeitem__label')].find(node => node.textContent === file);
      if (!label) throw new Error("Missing trace file selection"); label.click();
    }, trace.file);
    await waitFor(() => evaluate(() => Boolean(document.querySelector('[data-testid="trace-rows"]'))));
    assert.equal(await evaluate(() => Boolean(document.querySelector(".react-flow"))), false);
    const workersBefore = await evaluate(() => window.traceWorkerUrls.length);
    assert.equal(workersBefore, overviewWorkers, "file overview must not start a layout worker");
    await click(trace.name); await ready(); await graphMatches();
    assert((await evaluate(() => window.traceWorkerUrls.length)) > workersBefore);
    console.log(`PASS ${kind}/file-overview-and-verification-selection`);
    const selectedUrl = await evaluate(() => location.href);
    await evaluate(() => history.back());
    await waitFor(() => evaluate(() => Boolean(document.querySelector('[data-testid="trace-rows"]'))));
    assert.equal(await evaluate(() => new URLSearchParams(location.hash.split("?")[1]).get("file")), trace.file);
    await evaluate(() => history.forward()); await ready(); await graphMatches();
    await loadPage(selectedUrl); await ready(); await graphMatches();
    await reloadPage(); await ready(); await graphMatches();
    await loadPage(`${base}/#/traces?selected=missing-verification`);
    await waitFor(() => evaluate(() => document.body.textContent.includes("unavailable") && Boolean(document.querySelector('[data-testid="trace-rows"]'))));
    await loadPage(selectedUrl); await ready();
    console.log(`PASS ${kind}/trace-selection-copy-reload-history-fallback`);

    await click("Model"); await click("Flow");
    await waitFor(() => evaluate(() => document.querySelector('[aria-label="Model flow"]')?.getAttribute("aria-busy") === "false"));
    const modelId = trace.directly_verified_requirements[0];
    await evaluate(id => {
      const name = window.reqvireProjectStore.elements.find(node => node.id === id).name;
      const label = [...document.querySelectorAll('[aria-label="Project tree"] .ds-treeitem__label')].find(node => node.textContent === name);
      if (!label) throw new Error(`Missing Model selection ${name}`); label.click();
    }, modelId);
    await waitFor(() => evaluate(id => new URLSearchParams(location.hash.split("?")[1]).get("selected") === id, modelId));
    const modelUrl = await evaluate(() => location.href);
    assert.equal(new URLSearchParams(new URL(modelUrl).hash.split("?")[1]).get("mode"), "flow");
    await click("Traces"); await ready();
    assert.equal(await evaluate(() => location.hash), selectionHash);
    await click("Model");
    await waitFor(() => evaluate(id => new URLSearchParams(location.hash.split("?")[1]).get("selected") === id, modelId));
    await loadPage(modelUrl);
    await waitFor(() => evaluate(id => new URLSearchParams(location.hash.split("?")[1]).get("selected") === id && document.querySelector('[aria-label="Model flow"]')?.getAttribute("aria-busy") === "false", modelId));
    await evaluate(() => { location.hash = "#/model?selected=missing-model&mode=flow"; });
    await waitFor(() => evaluate(() => document.body.textContent.includes("unavailable") && new URLSearchParams(location.hash.split("?")[1]).get("selected") === ""));
    await evaluate(() => history.back());
    await waitFor(() => evaluate(id => new URLSearchParams(location.hash.split("?")[1]).get("selected") === id, modelId));
    console.log(`PASS ${kind}/model-selection-mode-copy-history-fallback`);

    for (const view of ["thesaurus", "ontologies"]) {
      const selected = await evaluate(view => view === "thesaurus" ? window.reqvireProjectStore.thesaurus.concepts[0].element_id : window.reqvireProjectStore.ontology.graph_data.nodes.find(node => node.id === "https://example.test/ontology#ServiceEndpoint").id, view);
      const selectedRoute = `#/${view}?selected=${encodeURIComponent(selected)}`;
      await loadPage(`${base}/${selectedRoute}`);
      await waitFor(() => evaluate(view => Boolean(document.querySelector(view === "thesaurus" ? '[data-view="thesaurus"]' : '[data-view="ontologies"]')), view));
      assert.equal(await evaluate(() => new URLSearchParams(location.hash.split("?")[1]).get("selected")), selected);
      if (view === "thesaurus") await waitFor(() => evaluate(id => {
        const concept = window.reqvireProjectStore.thesaurus.concepts.find(node => node.element_id === id);
        return [...document.querySelectorAll('.react-flow__node')].some(node => node.dataset.id === concept.id && Boolean(node.querySelector('.is-selected')));
      }, selected));
      else await waitFor(() => evaluate(() => document.querySelector('[aria-label="Selected ontology node"]')?.textContent.includes("Service Endpoint Ontology Class") && !document.body.textContent.includes("renderer failed")));
      await evaluate(view => { location.hash = `#/${view}?selected=`; }, view);
      await waitFor(() => evaluate(() => new URLSearchParams(location.hash.split("?")[1]).get("selected") === ""));
      await evaluate(() => history.back());
      await waitFor(() => evaluate(id => new URLSearchParams(location.hash.split("?")[1]).get("selected") === id, selected));
      await reloadPage();
      await waitFor(() => evaluate(view => Boolean(document.querySelector(view === "thesaurus" ? '[data-view="thesaurus"]' : '[data-view="ontologies"]')), view));
      assert.equal(await evaluate(() => new URLSearchParams(location.hash.split("?")[1]).get("selected")), selected);
    }
    console.log(`PASS ${kind}/semantic-selection-copy-reload-history`);

  }
});
