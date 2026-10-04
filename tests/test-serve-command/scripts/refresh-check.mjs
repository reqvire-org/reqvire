import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { createHash } from "node:crypto";
import { createServer } from "node:http";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { promisify } from "node:util";
import { openBrowser, waitFor } from "./browser.mjs";

const [browser, plainUrl, mcpUrl, workspace, serverBin] = process.argv.slice(2);
const main = await openBrowser(browser, path.join(workspace, "refresh-main-profile"));
const plain = await openBrowser(browser, path.join(workspace, "refresh-plain-profile"));
const requirements = path.join(workspace, "specifications/Requirements.md");
const original = await readFile(requirements, "utf8");
const added = await readFile(path.join(workspace, "fixtures/live-refresh-requirement.md.txt"), "utf8");
const fileRoute = "#/files/specifications/Requirements.md";
const token = "issue-70-existing-document";
const protocol = "2025-11-25";
let nextId = 0;
let failed = false;
let staticServer;
let staticDir;

// Observe real HTTP traffic and inject failures at the transport boundary.
// The production store client and compiled UI remain responsible for recovery.
await main.rpc("Page.addScriptToEvaluateOnNewDocument", { source: `(${function instrument() {
  const original = window.fetch;
  window.reqvireManifestTraffic = [];
  window.fetch = async (...args) => {
    const url = String(args[0]);
    if (!url.startsWith("/api/project-store")) return original(...args);
    const request = args[1] ?? {};
    const entry = { url, method: request.method ?? "GET" };
    if (url.endsWith("/chunks")) Object.assign(entry, JSON.parse(request.body));
    window.reqvireManifestTraffic.push(entry);
    if (url.endsWith("/chunks") && window.reqvireHoldNextChunks) {
      window.reqvireHoldNextChunks = false;
      window.reqvireHeldChunks = true;
      await new Promise(resolve => {
        window.reqvireReleaseChunks = () => { window.reqvireHeldChunks = false; resolve(); };
      });
    }
    let response = await original(...args);
    entry.status = response.status;
    if (url.endsWith("/chunks") && response.ok && window.reqvireChunkFault) {
      const payload = await response.clone().json();
      const first = Object.keys(payload.chunks)[0];
      entry.fault = window.reqvireChunkFault;
      if (entry.fault === "missing") delete payload.chunks[first];
      else payload.chunks[first] += " ";
      window.reqvireChunkFault = null;
      response = new Response(JSON.stringify(payload), { status: 200, headers: response.headers });
    }
    return response;
  };
}.toString()})();` });

const referencedHashes = manifest => new Set(Object.values(manifest.sections)
  .flatMap(section => section.kind === "array" ? section.hashes : [section.hash]));
const wireHash = value => createHash("sha256").update(value).digest("hex");
const edit = marker => tool("reqvire.add_element", {
  file: "specifications/Requirements.md",
  content: added.replace("Live source marker alpha.", `Live source marker ${marker}.`),
  override_existing: true, dry_run: false,
});

async function browserMatchesPublished() {
  const published = await snapshot(mcpUrl);
  assert.deepEqual(await main.evaluate(() => window.reqvireProjectStore), published.store);
  assert.equal(await main.evaluate(() => window.reqvireLiveRefresh.revision), published.revision);
}

async function mcp(method, params) {
  const response = await fetch(`${mcpUrl}/mcp`, {
    method: "POST",
    signal: AbortSignal.timeout(30000),
    headers: {
      "Content-Type": "application/json", Accept: "application/json, text/event-stream",
      "Mcp-Protocol-Version": protocol,
    },
    body: JSON.stringify({ jsonrpc: "2.0", id: ++nextId, method, params }),
  });
  assert.equal(response.status, 200);
  const value = await response.json();
  assert.equal(value.error, undefined, JSON.stringify(value));
  assert.notEqual(value.result?.isError, true, JSON.stringify(value));
  return value.result;
}
const tool = (name, args = {}) => mcp("tools/call", { name, arguments: args });
async function live(base, etag) {
  return fetch(`${base}/api/project-store`, {
    signal: AbortSignal.timeout(15000),
    headers: etag ? { "If-None-Match": etag } : {},
  });
}
async function snapshot(base) {
  const response = await live(base);
  assert.equal(response.status, 200);
  assert.match(response.headers.get("content-type"), /^application\/json/);
  assert.equal(response.headers.get("cache-control"), "no-store");
  const value = await response.json();
  assert.ok(value.revision);
  assert.ok(Array.isArray(value.store.elements));
  return { ...value, etag: response.headers.get("etag") };
}
async function storesMatch(check) {
  await waitFor(async () => {
    try {
      return check((await snapshot(mcpUrl)).store);
    } catch { return false; }
  });
}
async function rendered(session, marker, present = true, selector = ".ux-file-browser__elements") {
  await waitFor(() => session.evaluate((marker, present, selector) => {
    const scope = document.querySelector(selector);
    return Boolean(scope) && scope.textContent.includes(marker) === present;
  }, marker, present, selector));
}
async function sameDocument(session) {
  assert.equal(await session.evaluate(() => window.reqvireRefreshE2E), token);
}
async function run(label, check) {
  try {
    await check();
    console.log(`${label}: PASS`);
  } catch (error) {
    failed = true;
    console.log(`${label}: FAIL`);
    console.error(`${label}: ${error.stack}`);
    console.error(await main.evaluate(() => document.body.innerText.slice(0, 1000)));
    console.error(JSON.stringify(await main.evaluate(() => ({
      revision: window.reqvireLiveRefresh?.revision,
      traffic: window.reqvireManifestTraffic.slice(-12),
    })), null, 2));
  }
}

try {
  assert.equal((await mcp("initialize", {
    protocolVersion: protocol, capabilities: {}, clientInfo: { name: "serve-refresh-e2e", version: "0" },
  })).protocolVersion, protocol);
  await Promise.all([main.navigate(`${mcpUrl}/${fileRoute}`), plain.navigate(`${plainUrl}/${fileRoute}`)]);
  await Promise.all([main, plain].map(session => session.evaluate(token => {
    window.reqvireRefreshE2E = token;
  }, token)));
  const initialManifest = await main.evaluate(() => window.reqvireLiveRefresh.manifest);
  const trafficStart = await main.evaluate(() => window.reqvireManifestTraffic.length);

  await run("embedded MCP updates open tabs", async () => {
    const result = await tool("reqvire.add_element", {
      file: "specifications/Requirements.md", content: added, dry_run: false,
    });
    assert.equal(result.structuredContent.dry_run, false);
    await rendered(main, "Live Refresh Requirement");
    await rendered(plain, "Live Refresh Requirement", false);
    assert.equal(await plain.evaluate(() => Boolean(window.reqvireLiveRefresh)), false);
    assert.equal((await live(plainUrl)).status, 200);
    for (const session of [main, plain]) {
      assert.equal(await session.evaluate(() => Boolean(document.querySelector('button[aria-label="Refresh"]'))), false);
    }
    await Promise.all([sameDocument(main), sameDocument(plain)]);
    assert.equal(await main.evaluate(() => location.hash), fileRoute);
  });

  await run("manifest transfers only changed chunks", async () => {
    const latest = await main.evaluate(() => window.reqvireLiveRefresh.manifest);
    const before = referencedHashes(initialManifest);
    const missing = [...referencedHashes(latest)].filter(hash => !before.has(hash)).sort();
    assert.ok(missing.length > 0);
    const traffic = await main.evaluate(start => window.reqvireManifestTraffic.slice(start), trafficStart);
    const transferred = traffic.filter(entry => entry.url.endsWith("/chunks") && entry.status === 200)
      .flatMap(entry => entry.hashes).sort();
    assert.deepEqual(transferred, missing);
    assert.equal(traffic.some(entry => entry.url === "/api/project-store"), false);
    await browserMatchesPublished();
  });

  await run("refreshed immutable store renders reports and graphs", async () => {
    assert.ok(await main.evaluate(() => Object.isFrozen(window.reqvireProjectStore.elements[0])));
    try {
      for (const view of ["coverage", "traces", "ontologies", "model"]) {
        await main.evaluate(view => { location.hash = `#/${view}`; }, view);
        if (view === "model") {
          await waitFor(() => main.evaluate(() => Boolean(document.querySelector('[aria-label="Model layout"]'))));
          await main.evaluate(() => {
            [...document.querySelectorAll('[aria-label="Model layout"] button')]
              .find(button => button.textContent.includes("Graph")).click();
          });
        }
        await waitFor(() => main.evaluate(view => {
          const frame = document.querySelector(`[data-view="${view}"]`);
          return Boolean(frame) && !frame.textContent.includes("Loading");
        }, view));
        assert.equal(await main.evaluate(view => document.querySelector(`[data-view="${view}"]`)
          .textContent.toLowerCase().includes("failed"), view), false);
        await browserMatchesPublished();
        await sameDocument(main);
      }
    } finally {
      await main.evaluate(route => { location.hash = route; }, fileRoute);
      await rendered(main, "Live Refresh Requirement");
    }
  });

  await run("unchanged conditional revision", async () => {
    const base = mcpUrl;
    const first = await snapshot(base);
    assert.ok(first.etag);
    const unchanged = await live(base, first.etag);
    assert.equal(unchanged.status, 304);
    assert.equal(await unchanged.text(), "");
    assert.equal(unchanged.headers.get("etag"), first.etag);
    assert.equal((await snapshot(base)).revision, first.revision);
    const head = await fetch(`${base}/api/project-store`, { method: "HEAD" });
    assert.equal(head.status, 200);
    assert.equal(await head.text(), "");
    assert.equal(head.headers.get("etag"), first.etag);
    assert.equal((await fetch(`${base}/api/missing`)).status, 404);
    assert.equal((await fetch(`${base}/api/project-store`, { method: "POST" })).status, 405);
    const seed = await (await fetch(`${base}/assets/project-store.js`)).text();
    const prefix = "window.reqvireProjectStore = ";
    assert.ok(seed.startsWith(prefix));
    assert.deepEqual(JSON.parse(seed.slice(prefix.length, seed.indexOf(";\n"))), first.store);
    assert.ok(seed.includes("Live Refresh Requirement"));
    assert.ok(seed.includes(first.revision));
    assert.equal((await fetch(`${base}/ontologies.ttl`)).status, 200);
    const manifestResponse = await fetch(`${base}/api/project-store/manifest`);
    const text = await manifestResponse.text();
    assert.equal(manifestResponse.headers.get("etag"), first.etag);
    assert.equal(wireHash(text), first.revision);
    const manifest = JSON.parse(text);
    assert.equal(manifest.protocol, "reqvire-manifest.v1");
    const conditional = await fetch(`${base}/api/project-store/manifest`, { headers: { "If-None-Match": first.etag } });
    assert.equal(conditional.status, 304);
    assert.equal(await conditional.text(), "");
    const hash = manifest.sections.elements.hashes[0];
    const chunksResponse = await fetch(`${base}/api/project-store/chunks`, {
      method: "POST", headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ revision: first.revision, hashes: [hash] }),
    });
    assert.equal(chunksResponse.status, 200);
    assert.equal(chunksResponse.headers.get("cache-control"), "no-store");
    const payload = await chunksResponse.json();
    assert.deepEqual(Object.keys(payload.chunks), [hash]);
    assert.equal(wireHash(payload.chunks[hash]), hash);
    assert.equal(payload.revision, first.revision);
    assert.deepEqual(JSON.parse(payload.chunks[hash]), first.store.elements[0]);
    assert.equal((await fetch(`${plainUrl}/api/project-store/manifest`)).status, 200);
    assert.equal((await fetch(`${plainUrl}/api/project-store/chunks`, { method: "POST" })).status, 415);
  });

  let context;
  await run("MCP edit preserves context", async () => {
    await main.evaluate(() => document.querySelector('input[placeholder="Filter model tree..."]').focus());
    await main.rpc("Input.insertText", { text: "Requirements" });
    assert.equal(await main.evaluate(() => document.querySelector('input[placeholder="Filter model tree..."]').value), "Requirements");
    await main.evaluate(() => {
      document.querySelector('button[aria-label="Collapse explorer"]').click();
      [...document.querySelectorAll("button.ux-file-browser__element-row")]
        .find(button => button.textContent.includes("Live Refresh Requirement")).click();
    });
    await rendered(main, "Live source marker alpha.", true, '[role="dialog"]');
    context = await main.evaluate(() => location.hash);
    const before = await snapshot(mcpUrl);
    await tool("reqvire.add_element", {
      file: "specifications/Requirements.md",
      content: added.replace("Live source marker alpha.", "Live source marker bravo."),
      override_existing: true, dry_run: false,
    });
    await storesMatch(store => store.elements.some(element => element.content.includes("Live source marker bravo.")));
    await rendered(main, "Live source marker bravo.", true, '[role="dialog"]');
    assert.notEqual((await snapshot(mcpUrl)).revision, before.revision);
    assert.equal(await main.evaluate(() => location.hash), context);
    assert.ok(await main.evaluate(() => Boolean(document.querySelector('button[aria-label="Expand explorer"]'))));
    await main.evaluate(() => document.querySelector('[role="dialog"] button[aria-label="Close"]').click());
    await main.evaluate(() => document.querySelector('button[aria-label="Expand explorer"]').click());
    assert.equal(await main.evaluate(() => document.querySelector('input[placeholder="Filter model tree..."]').value), "Requirements");
    await main.evaluate(() => {
      [...document.querySelectorAll("button.ux-file-browser__element-row")]
        .find(button => button.textContent.includes("Live Refresh Requirement")).click();
    });
    await rendered(main, "Live source marker bravo.", true, '[role="dialog"]');
    await sameDocument(main);
  });

  await run("refreshed source and search", async () => {
    await main.evaluate(() => { location.hash = "#/search/bravo"; });
    await rendered(main, "Live Refresh Requirement", true, '[data-view="search"]');
    await rendered(main, "Live source marker bravo.", true, '[data-view="search"]');
    await tool("reqvire.add_element", {
      file: "specifications/Requirements.md",
      content: added.replace("Live source marker alpha.", "Live source marker gamma."),
      override_existing: true, dry_run: false,
    });
    await storesMatch(store => store.elements.some(element => element.content.includes("Live source marker gamma.")));
    await rendered(main, "Live Refresh Requirement", false, '[data-view="search"]');
    await main.evaluate(() => { location.hash = "#/content/specifications/Requirements.md"; });
    await rendered(main, "Live source marker gamma.", true, '[data-product-pattern="app-shell"]');
    await sameDocument(main);
  });

  await main.evaluate(() => { location.hash = "#/files/specifications/Requirements.md"; });
  await rendered(main, "Live Refresh Requirement");
  await main.evaluate(() => {
    [...document.querySelectorAll("button.ux-file-browser__element-row")]
      .find(button => button.textContent.includes("Live Refresh Requirement")).click();
  });
  await rendered(main, "Live source marker gamma.", true, '[role="dialog"]');

  await run("mutation during refresh retries the latest manifest", async () => {
    const start = await main.evaluate(() => window.reqvireManifestTraffic.length);
    await main.evaluate(() => { window.reqvireHoldNextChunks = true; });
    await edit("delta");
    await waitFor(() => main.evaluate(() => window.reqvireHeldChunks === true), 20000);
    await rendered(main, "Live source marker gamma.", true, '[role="dialog"]');
    await edit("omega café 測定");
    await main.evaluate(() => window.reqvireReleaseChunks());
    await rendered(main, "Live source marker omega café 測定.", true, '[role="dialog"]');
    const traffic = await main.evaluate(start => window.reqvireManifestTraffic.slice(start), start);
    assert.ok(traffic.some(entry => entry.url.endsWith("/chunks") && entry.status === 409));
    assert.ok(traffic.some(entry => entry.url.endsWith("/chunks") && entry.status === 200));
    await browserMatchesPublished();
    await sameDocument(main);
  });

  for (const [fault, marker, previous] of [["missing", "sigma", "omega café 測定"], ["corrupt", "phi", "sigma"]]) {
    await run(`${fault} chunks retain the valid store and recover`, async () => {
      const before = await main.evaluate(() => window.reqvireLiveRefresh.revision);
      await main.evaluate(fault => { window.reqvireChunkFault = fault; }, fault);
      await edit(marker);
      await rendered(main, "Refresh failed", true, '[role="alert"]');
      assert.equal(await main.evaluate(() => window.reqvireLiveRefresh.revision), before);
      await rendered(main, `Live source marker ${previous}.`, true, '[role="dialog"]');
      await sameDocument(main);
      await rendered(main, `Live source marker ${marker}.`, true, '[role="dialog"]');
      await waitFor(() => main.evaluate(() => !document.querySelector('[role="alert"]')));
      await browserMatchesPublished();
    });
  }

  await tool("reqvire.add_element", {
    file: "specifications/Requirements.md",
    content: added.replace("Live Refresh Requirement", "Manifest Catchup Requirement"), dry_run: false,
  });
  await rendered(main, "Manifest Catchup Requirement");

  await run("visibility pauses and resumes refresh", async () => {
    await main.evaluate(() => {
      const originalFetch = window.fetch;
      window.reqvireRefreshRequestCount = 0;
      window.fetch = (...args) => {
        if (String(args[0]).includes("/api/project-store")) window.reqvireRefreshRequestCount++;
        return originalFetch(...args);
      };
    });
    // Freezing backgrounds the document. Resume execution while keeping it
    // hidden so timers run and the application must suspend its own requests.
    await main.rpc("Page.setWebLifecycleState", { state: "frozen" });
    await main.rpc("Page.setWebLifecycleState", { state: "active" });
    assert.equal(await main.evaluate(() => document.visibilityState), "hidden");
    const requests = await main.evaluate(() => window.reqvireRefreshRequestCount);
    await edit("eta");
    await edit("iota");
    await tool("reqvire.remove_element", { element_name: "Manifest Catchup Requirement", dry_run: false });
    await edit("theta");
    await snapshot(mcpUrl);
    await new Promise(resolve => setTimeout(resolve, 10500));
    assert.equal(await main.evaluate(() => window.reqvireRefreshRequestCount), requests);
    await rendered(main, "Live source marker phi.", true, '[role="dialog"]');
    // In headless shell, focus emulation produces the actual visibilitychange
    // back to visible; resuming the lifecycle alone leaves the page hidden.
    await main.rpc("Emulation.setFocusEmulationEnabled", { enabled: true });
    assert.equal(await main.evaluate(() => document.visibilityState), "visible");
    await rendered(main, "Live source marker theta.", true, '[role="dialog"]');
    await rendered(main, "Manifest Catchup Requirement", false);
    assert.ok(await main.evaluate(requests => window.reqvireRefreshRequestCount > requests, requests));
    await sameDocument(main);
    await browserMatchesPublished();
  });

  await run("static export has no live refresh", async () => {
    staticDir = await mkdtemp(path.join(tmpdir(), "reqvire-refresh-export-"));
    await promisify(execFile)(serverBin, ["export", "--output", staticDir], { cwd: workspace, timeout: 30000 });
    let liveRequests = 0;
    staticServer = createServer(async (request, response) => {
      const requestPath = new URL(request.url, "http://localhost").pathname;
      if (requestPath.startsWith("/api/")) liveRequests++;
      try {
        const content = await readFile(path.join(staticDir, requestPath === "/" ? "index.html" : requestPath));
        response.setHeader("Content-Type", requestPath.endsWith(".js") ? "application/javascript" : requestPath.endsWith(".css") ? "text/css" : "text/html");
        response.end(content);
      } catch { response.writeHead(404).end(); }
    });
    await new Promise(resolve => staticServer.listen(0, "127.0.0.1", resolve));
    await plain.navigate(`http://127.0.0.1:${staticServer.address().port}/${fileRoute}`);
    assert.equal(await plain.evaluate(() => Boolean(document.querySelector('button[aria-label="Refresh"]'))), false);
    await new Promise(resolve => setTimeout(resolve, 10500));
    assert.equal(liveRequests, 0);
    await rendered(plain, "Live Refresh Requirement");
  });

  await run("MCP remains available without reinitialization", async () => {
    assert.ok(JSON.stringify(await tool("reqvire.search")).includes("Live source marker theta."));
    assert.ok((await mcp("tools/list", {})).tools.some(tool => tool.name === "reqvire.add_element"));
  });
  console.log("completed");
  if (failed) process.exitCode = 1;
} finally {
  await writeFile(requirements, original);
  await Promise.all([main.close(), plain.close()]);
  if (staticServer) await new Promise(resolve => staticServer.close(resolve));
  if (staticDir) await rm(staticDir, { recursive: true, force: true });
}
