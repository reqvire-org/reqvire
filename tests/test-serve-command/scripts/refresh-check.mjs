import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { createServer } from "node:http";
import { mkdtemp, readFile, rm, stat, unlink, utimes, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { promisify } from "node:util";
import { openBrowser, waitFor } from "./browser.mjs";

const [browser, plainUrl, mcpUrl, workspace, serverBin] = process.argv.slice(2);
const main = await openBrowser(browser, path.join(workspace, "refresh-main-profile"));
const plain = await openBrowser(browser, path.join(workspace, "refresh-plain-profile"));
const requirements = path.join(workspace, "specifications/Requirements.md");
const externalFile = path.join(workspace, "specifications/ExternalRefresh.md");
const original = await readFile(requirements, "utf8");
const added = await readFile(path.join(workspace, "fixtures/live-refresh-requirement.md.txt"), "utf8");
const external = await readFile(path.join(workspace, "fixtures/external-refresh-file.md.txt"), "utf8");
const fileRoute = "#/files/specifications/Requirements.md";
const token = "issue-70-existing-document";
const protocol = "2025-11-25";
let nextId = 0;
let failed = false;
let staticServer;
let staticDir;

async function mcp(method, params) {
  const response = await fetch(`${mcpUrl}/mcp`, {
    method: "POST",
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
async function live(base, force = false, etag) {
  return fetch(`${base}/api/project-store${force ? "?refresh=true" : ""}`, {
    headers: etag ? { "If-None-Match": etag } : {},
  });
}
async function snapshot(base, force = false) {
  const response = await live(base, force);
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
      const values = await Promise.all([snapshot(plainUrl), snapshot(mcpUrl)]);
      return values.every(value => check(value.store));
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

  await run("embedded MCP updates open tabs", async () => {
    const result = await tool("reqvire.add_element", {
      file: "specifications/Requirements.md", content: added, dry_run: false,
    });
    assert.equal(result.structuredContent.dry_run, false);
    await Promise.all([rendered(main, "Live Refresh Requirement"), rendered(plain, "Live Refresh Requirement")]);
    await Promise.all([sameDocument(main), sameDocument(plain)]);
    assert.equal(await main.evaluate(() => location.hash), fileRoute);
  });

  await run("unchanged conditional revision", async () => {
    for (const base of [plainUrl, mcpUrl]) {
      const first = await snapshot(base, true);
      assert.ok(first.etag);
      const unchanged = await live(base, true, first.etag);
      assert.equal(unchanged.status, 304);
      assert.equal(await unchanged.text(), "");
      assert.equal(unchanged.headers.get("etag"), first.etag);
      assert.equal((await snapshot(base, true)).revision, first.revision);
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
    }
  });

  await writeFile(externalFile, external);
  await run("external file addition", async () => {
    await storesMatch(store => store.elements.some(element => element.name === "External Refresh Requirement"));
    await waitFor(() => main.evaluate(() => document.body.textContent.includes("ExternalRefresh.md")));
    await waitFor(() => plain.evaluate(() => document.body.textContent.includes("ExternalRefresh.md")));
    await Promise.all([sameDocument(main), sameDocument(plain)]);
    assert.ok(JSON.stringify(await tool("reqvire.search")).includes("External Refresh Requirement"));
  });

  let context;
  await run("same length and timestamp edit preserves context", async () => {
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
    const before = await snapshot(mcpUrl, true);
    const timestamps = await stat(requirements);
    const content = await readFile(requirements, "utf8");
    const changed = content.replace("Live source marker alpha.", "Live source marker bravo.");
    assert.equal(changed.length, content.length);
    assert.notEqual(changed, content);
    await writeFile(requirements, changed);
    await utimes(requirements, timestamps.atime, timestamps.mtime);
    assert.equal((await stat(requirements)).size, timestamps.size);
    assert.ok(Math.abs((await stat(requirements)).mtimeMs - timestamps.mtimeMs) < 1);
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
    const content = await readFile(requirements, "utf8");
    await writeFile(requirements, content.replace("Live source marker bravo.", "Live source marker gamma."));
    await storesMatch(store => store.elements.some(element => element.content.includes("Live source marker gamma.")));
    await rendered(main, "Live Refresh Requirement", false, '[data-view="search"]');
    await main.evaluate(() => { location.hash = "#/content/specifications/Requirements.md"; });
    await rendered(main, "Live source marker gamma.", true, '[data-product-pattern="app-shell"]');
    await sameDocument(main);
  });

  await main.evaluate(() => { location.hash = "#/files/specifications/Requirements.md"; });
  await run("manual refresh without navigation", async () => {
    await main.evaluate(() => {
      const originalFetch = window.fetch;
      window.reqvireForcedRefreshRequests = 0;
      window.fetch = (...args) => {
        if (String(args[0]).includes("/api/project-store?refresh=true")) window.reqvireForcedRefreshRequests++;
        return originalFetch(...args);
      };
    });
    const content = await readFile(requirements, "utf8");
    await writeFile(requirements, content.replace(/Live source marker (alpha|bravo|gamma)\./, "Live source marker delta."));
    await waitFor(() => main.evaluate(() => {
      const button = document.querySelector('button[aria-label="Refresh"]');
      return button && !button.disabled;
    }));
    await main.evaluate(() => {
      const button = document.querySelector('button[aria-label="Refresh"]');
      if (!button) throw new Error("Refresh action missing");
      button.click();
    });
    await waitFor(() => main.evaluate(() => window.reqvireForcedRefreshRequests === 1));
    await main.evaluate(() => {
      [...document.querySelectorAll("button.ux-file-browser__element-row")]
        .find(button => button.textContent.includes("Live Refresh Requirement")).click();
    });
    await rendered(main, "Live source marker delta.", true, '[role="dialog"]');
    await sameDocument(main);
  });

  const lastValid = await readFile(requirements, "utf8");
  let lastRevision;
  try { lastRevision = (await snapshot(mcpUrl)).revision; } catch { /* Report missing live API in the named check below. */ }
  await main.evaluate(() => document.querySelector('[role="dialog"] button[aria-label="Close"]')?.click());
  await writeFile(requirements, lastValid.replace("### Live Refresh Requirement", "### Parent Requirement"));
  await run("invalid edit retains valid data", async () => {
    let errorResponse;
    await waitFor(async () => {
      errorResponse = await live(mcpUrl, true);
      return errorResponse.status === 503;
    });
    const diagnostic = await errorResponse.json();
    assert.ok(lastRevision);
    assert.equal(diagnostic.revision, lastRevision);
    assert.ok(diagnostic.error);
    const seed = await (await fetch(`${mcpUrl}/assets/project-store.js`)).text();
    assert.ok(seed.includes("Live source marker delta."));
    await rendered(main, "Refresh failed", true, '[role="alert"]');
    assert.equal(await main.evaluate(() => Boolean(document.querySelector('[role="dialog"]'))), false);
    assert.ok(await main.evaluate(() => {
      const alert = document.querySelector('[role="alert"]');
      return alert.getBoundingClientRect().height > 0 && getComputedStyle(alert).visibility === "visible";
    }));
    await main.evaluate(() => {
      [...document.querySelectorAll("button.ux-file-browser__element-row")]
        .find(button => button.textContent.includes("Live Refresh Requirement")).click();
    });
    await rendered(main, "Live source marker delta.", true, '[role="dialog"]');
    await sameDocument(main);
  });
  await writeFile(requirements, lastValid.replace("Live source marker delta.", "Live source marker omega."));
  await run("valid edit recovers", async () => {
    await storesMatch(store => store.elements.some(element => element.content.includes("Live source marker omega.")));
    await rendered(main, "Live source marker omega.", true, '[role="dialog"]');
    await waitFor(() => main.evaluate(() => !document.body.textContent.includes("Refresh failed")));
    await sameDocument(main);
  });

  await unlink(externalFile);
  await run("external file deletion", async () => {
    await storesMatch(store => !store.elements.some(element => element.name === "External Refresh Requirement"));
    await waitFor(() => plain.evaluate(() => !document.body.textContent.includes("ExternalRefresh.md")));
    await sameDocument(plain);
    assert.ok(!JSON.stringify(await tool("reqvire.search")).includes("External Refresh Requirement"));
  });

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
    const content = await readFile(requirements, "utf8");
    await writeFile(requirements, content.replace("Live source marker omega.", "Live source marker theta."));
    await snapshot(mcpUrl, true);
    await new Promise(resolve => setTimeout(resolve, 3500));
    assert.equal(await main.evaluate(() => window.reqvireRefreshRequestCount), requests);
    await rendered(main, "Live source marker omega.", true, '[role="dialog"]');
    // In headless shell, focus emulation produces the actual visibilitychange
    // back to visible; resuming the lifecycle alone leaves the page hidden.
    await main.rpc("Emulation.setFocusEmulationEnabled", { enabled: true });
    assert.equal(await main.evaluate(() => document.visibilityState), "visible");
    await rendered(main, "Live source marker theta.", true, '[role="dialog"]');
    assert.ok(await main.evaluate(requests => window.reqvireRefreshRequestCount > requests, requests));
    await sameDocument(main);
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
    await new Promise(resolve => setTimeout(resolve, 3500));
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
  await rm(externalFile, { force: true });
  await Promise.all([main.close(), plain.close()]);
  if (staticServer) await new Promise(resolve => staticServer.close(resolve));
  if (staticDir) await rm(staticDir, { recursive: true, force: true });
}
