import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { withBrowser } from "../browser.mjs";

const [servedUrl, exportedUrl, profile, reportsPath, contextsPath] = process.argv.slice(2);
const reports = JSON.parse(await readFile(reportsPath, "utf8"));
const contexts = JSON.parse(await readFile(contextsPath, "utf8"));
const categories = {
  "unverified-requirements": "unverified_leaf_requirements",
  "unimplemented-requirements": "uncovered_requirements",
  "unsatisfied-verifications": "unsatisfied_test_verifications",
  "orphaned-verifications": "orphaned_verifications",
};
const hash = name => `#/coverage?scope=${encodeURIComponent(name ? reports[name].scope.capability_identifier : "")}`;
const pause = () => new Promise(resolve => setTimeout(resolve, 50));

// Separate browser sessions keep history assertions below Chromium's 50-entry limit.
for (const [kind, base] of [["served", servedUrl], ["exported", exportedUrl]]) {
  await withBrowser(`${profile}-${kind}`, async ({ rpc, evaluate, loadPage, reloadPage }) => {
    async function waitFor(check, ...args) {
      const deadline = Date.now() + 15000;
      while (Date.now() < deadline) {
        if (await evaluate(check, ...args)) return;
        await pause();
      }
      console.error(await evaluate(() => JSON.stringify({ url: location.href, title: document.querySelector('[data-view="coverage"] h1')?.textContent, focus: document.activeElement?.outerHTML?.slice(0, 600) })));
      throw new Error(`Navigation timed out: ${check}`);
    }
    async function open(url) {
      await loadPage("about:blank"); await loadPage(url);
      await waitFor(() => Boolean(document.querySelector('[aria-label="Coverage capabilities"]')));
    }
    async function select(name) {
      await evaluate(id => {
        const button = id ? [...document.querySelectorAll('[data-capability-id]')].find(row => row.dataset.capabilityId === id)
          : document.querySelector('[data-coverage-whole-model]');
        if (!button) throw new Error(`Missing coverage selection: ${id}`);
        button.click();
      }, name ? reports[name].scope.capability_identifier : null);
      await scope(name);
    }
    async function scope(name, worktree, report = reports[name || "whole"]) {
      await waitFor((report, expectedHash, worktree) => {
        const view = document.querySelector('[data-view="coverage"]');
        return location.hash === expectedHash && (!worktree || new URL(location.href).searchParams.get("worktree_id") === worktree)
          && view?.querySelector("h1")?.textContent === (report.scope?.capability_name ?? "Whole Model")
          && view.textContent.includes(`${report.summary.covered_terminal_requirements} / ${report.summary.total_terminal_requirements} terminal requirements covered`)
          && view.textContent.includes(`${report.summary.verified_leaf_requirements} / ${report.summary.total_leaf_requirements} verified`);
      }, report, hash(name), worktree ?? null);
      // Issue disclosure is transient; expanding it must not add URL history.
      const historySize = await evaluate(() => history.length);
      await evaluate(() => document.querySelectorAll('.coverage-gap-list button[aria-expanded="false"]').forEach(button => button.click()));
      assert.equal(await evaluate(() => history.length), historySize);
      const actual = await evaluate(() => ({
        title: document.querySelector('[data-view="coverage"] h1')?.textContent,
        selected: document.querySelector('[data-capability-id][aria-selected="true"]')?.dataset.capabilityId ?? null,
        global: document.querySelector('[data-coverage-whole-model]')?.getAttribute("aria-selected"),
        mode: Boolean(document.querySelector('[aria-label="Coverage mode"]')),
        issuesAction: [...document.querySelectorAll('button')].some(button => button.textContent.trim() === "View issues"),
        prompt: [...document.querySelectorAll('h2')].some(heading => heading.textContent === "Select a capability"),
        pane: Boolean(document.querySelector('[aria-label="Explorer navigation"]')),
        kpis: document.querySelectorAll('.coverage-kpi').length,
        breakdownBefore: Boolean(document.querySelector('.coverage-grid').compareDocumentPosition(document.querySelector('#coverage-section-capability-coverage')) & Node.DOCUMENT_POSITION_FOLLOWING),
        typesBar: document.querySelectorAll('.ds-segmented-meter').length,
        rows: [...document.querySelectorAll('[data-kind="capability"][data-coverage-depth] a span:last-child')].map(node => node.textContent).sort(),
        gaps: Object.fromEntries([...document.querySelectorAll('.coverage-gap-list')].map(section => [section.id, {
          head: section.querySelector('.coverage-gap-list__head')?.textContent,
          names: [...section.querySelectorAll('.coverage-gap-row__title')].map(node => node.textContent).sort(),
          below: Boolean(document.querySelector('#coverage-section-capability-coverage').compareDocumentPosition(section) & Node.DOCUMENT_POSITION_FOLLOWING),
        }])),
        types: Object.fromEntries([...document.querySelectorAll('.coverage-legend-row')].map(row => [row.querySelector('span:nth-child(2)')?.textContent, row.querySelector('strong')?.textContent])),
        sources: Object.fromEntries([...document.querySelectorAll('.coverage-source-row__head')].map(row => [row.firstElementChild?.textContent, row.lastElementChild?.textContent])),
      }));
      assert.equal(actual.selected, report.scope?.capability_identifier ?? null);
      assert.equal(actual.global, name ? "false" : "true");
      assert.equal(actual.pane, true); assert.equal(actual.mode, false);
      assert.equal(actual.issuesAction, false); assert.equal(actual.prompt, false);
      assert.equal(actual.kpis, name ? 3 : 4);
      assert.equal(actual.breakdownBefore, true); assert.equal(actual.typesBar, 1);
      assert.deepEqual(actual.rows, report.capability_coverage.capabilities.map(row => row.name).sort());
      const fields = Object.entries(categories).filter(([id]) => !name || id !== "orphaned-verifications");
      assert.equal(Object.keys(actual.gaps).length, fields.length);
      for (const [id, field] of fields) {
        const expected = Object.values(report[field].files).flat().map(row => row.name).sort();
        const gap = actual.gaps[`coverage-section-${id}`];
        assert.ok(gap && gap.below); assert.ok(gap.head.endsWith(String(expected.length)));
        assert.deepEqual(gap.names, expected);
      }
      const typeLabels = { test: "Test", formal_proof: "Formal proof", analysis: "Analysis", inspection: "Inspection", demonstration: "Demonstration" };
      for (const [key, label] of Object.entries(typeLabels)) assert.equal(actual.types[label], String(report.summary.verification_types[key]));
      const sources = { direct_satisfied: "Direct evidence", requirement_rollup: "Requirement rollup", contract_consumer_rollup: "Contract consumer rollup", combined_rollup: "Combined rollup" };
      for (const [key, label] of Object.entries(sources)) assert.equal(actual.sources[label], String(report.summary.coverage_sources[key]));
    }
    async function click(label) {
      await evaluate(label => {
        const button = [...document.querySelectorAll("button")].find(button => button.getAttribute("aria-label") === label || button.textContent.trim() === label);
        if (!button) throw new Error(`Missing action: ${label}`); button.click();
      }, label);
    }
    async function run(label, check) { console.error(`START ${label}`); await check(); console.log(`${label}: PASS`); }

    await run(`${kind} URL precedence and explicit whole-model dashboard`, async () => {
      await open(`${base}/?theme=dark${hash("")}`); await evaluate(() => localStorage.clear()); await reloadPage(); await scope("");
      assert.deepEqual(await evaluate(() => [...document.querySelectorAll('[data-capability-id]')].map(row => [row.getAttribute("aria-label"), Number(row.getAttribute("aria-level"))])),
        [["Alpha Root", 2], ["Alpha Left", 3], ["Shared Branch", 4], ["Alpha Right", 3], ["Empty Branch", 3], ["Beta Root", 2]]);
      const before = await evaluate(() => history.length);
      await evaluate(() => { const root = document.querySelector('[data-coverage-whole-model]'); root.focus(); root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowLeft", bubbles: true })); });
      await waitFor(() => document.querySelectorAll('[aria-label="Coverage capabilities"] [role="treeitem"]').length === 1);
      assert.equal(await evaluate(() => history.length), before);
      assert.equal(await evaluate(() => document.querySelector('[data-coverage-whole-model]').getAttribute("aria-selected")), "true");
      await evaluate(() => document.activeElement.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowRight", bubbles: true })));
      await waitFor(() => Boolean(document.querySelector('[data-capability-id][aria-label="Alpha Root"]')));
      await evaluate(() => { const root = document.querySelector('[data-capability-id][aria-label="Alpha Root"]'); root.focus(); root.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowLeft", bubbles: true })); });
      await waitFor(() => !document.querySelector('[data-capability-id][aria-label="Shared Branch"]'));
      assert.equal(await evaluate(() => history.length), before);
      await evaluate(() => document.activeElement.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowRight", bubbles: true })));
      await select("Alpha Root");
      await open(`${base}/?theme=dark${hash("Shared Branch")}`); await scope("Shared Branch");
      assert.equal(await evaluate(() => new URL(location.href).searchParams.get("theme")), "dark");
      await open(`${base}/${hash("")}`); await scope("");
    });
    await run(`${kind} scope history and copied-link reload`, async () => {
      const before = await evaluate(() => history.length);
      await select("Alpha Root"); await select("Shared Branch");
      assert.equal(await evaluate(() => history.length), before + 2);
      await select("Shared Branch"); assert.equal(await evaluate(() => history.length), before + 2);
      await evaluate(() => history.back()); await scope("Alpha Root");
      await evaluate(() => history.forward()); await scope("Shared Branch");
      const copied = await evaluate(() => location.href);
      await evaluate(() => localStorage.clear()); await open(copied); await scope("Shared Branch");
      await reloadPage(); await scope("Shared Branch");
      await open(`${base}/#/coverage`); await scope("Shared Branch");
    });
    await run(`${kind} detail and view return retain the complete scoped dashboard`, async () => {
      await select("Alpha Root"); await click("Expand Shared Branch");
      await evaluate(() => document.querySelector('article[aria-label="Alpha Parent"] a').click());
      await waitFor(() => Boolean(document.querySelector('[role="dialog"]'))); await click("Close"); await scope("Alpha Root");
      await evaluate(() => document.querySelector('#coverage-section-unimplemented-requirements .coverage-gap-row').click());
      await waitFor(() => Boolean(document.querySelector('[role="dialog"]'))); await click("Close"); await scope("Alpha Root");
      await click("Model"); await waitFor(() => location.hash === "#/model?selected=&mode=grid");
      await click("Coverage"); await scope("Alpha Root");
    });
    await run(`${kind} scoped summaries issues and keyboard root reset`, async () => {
      for (const name of ["Alpha Root", "Shared Branch", "Empty Branch"]) await select(name);
      await rpc("Page.bringToFront");
      await evaluate(() => document.querySelector('[data-coverage-whole-model]').focus());
      assert.equal(await evaluate(() => document.activeElement.matches('[data-coverage-whole-model]')), true);
      await rpc("Input.dispatchKeyEvent", { type: "keyDown", key: "Enter", code: "Enter", text: "\r", unmodifiedText: "\r", windowsVirtualKeyCode: 13 });
      await rpc("Input.dispatchKeyEvent", { type: "keyUp", key: "Enter", code: "Enter", windowsVirtualKeyCode: 13 });
      await scope(""); const before = await evaluate(() => history.length);
      await select(""); assert.equal(await evaluate(() => history.length), before);
      await select("Alpha Root");
      await rpc("Page.bringToFront");
      await evaluate(() => document.querySelector('[data-coverage-whole-model]').focus());
      assert.equal(await evaluate(() => document.activeElement.matches('[data-coverage-whole-model]')), true);
      await rpc("Input.dispatchKeyEvent", { type: "keyDown", key: " ", code: "Space", text: " ", unmodifiedText: " ", windowsVirtualKeyCode: 32 });
      await rpc("Input.dispatchKeyEvent", { type: "keyUp", key: " ", code: "Space", windowsVirtualKeyCode: 32 });
      await scope("");
    });
    await run(`${kind} retired view links migrate to scope-only dashboards`, async () => {
      const root = hash("Alpha Root");
      for (const [suffix, expected] of [["&mode=capabilities", "Alpha Root"], ["&mode=issues&issue=unimplemented-requirements", "Alpha Root"], ["&mode=summary&section=implementation", ""], ["&mode=issues&issue=orphaned-verifications", ""]]) {
        await open(`${base}/${root}${suffix}`); await scope(expected);
      }
      await open(`${base}/#/coverage?mode=capabilities`); await scope("");
    });
    await run(`${kind} invalid and non-capability scope fallback`, async () => {
      for (const id of ["specifications/Capabilities.md#missing", "specifications/Alpha.md#alpha-parent"]) {
        await open(`${base}/#/coverage?scope=${encodeURIComponent(id)}`); await scope("");
        assert.equal(await evaluate(() => document.body.textContent.includes("selected capability is no longer available")), true);
      }
    });
    if (kind === "exported") return;
    await run("served worktree scopes are isolated and traversable", async () => {
      await open(`${servedUrl}/?worktree_id=${contexts.original}${hash("Alpha Root")}`); await scope("Alpha Root", contexts.original);
      async function branch(id) {
        await click("Branch"); await evaluate(root => {
          const choice = [...document.querySelectorAll('[role="option"]')].find(option => option.querySelector("small")?.textContent === root);
          if (!choice) throw new Error(`Missing worktree: ${root}`); choice.click();
        }, contexts.roots[id]);
      }
      await branch(contexts.other); await scope("", contexts.other); await select("Shared Branch");
      await branch(contexts.original); await scope("Alpha Root", contexts.original);
      await evaluate(() => history.back()); await scope("Shared Branch", contexts.other);
      await evaluate(() => history.forward()); await scope("Alpha Root", contexts.original);
    });
    await run("served refreshed scope removal restores the full whole-model dashboard", async () => {
      await select("Empty Branch");
      const response = await fetch(`${servedUrl}/mcp`, { signal: AbortSignal.timeout(15000), method: "POST",
        headers: { "Content-Type": "application/json", Accept: "application/json, text/event-stream", "Mcp-Protocol-Version": "2025-11-25" },
        body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "tools/call", params: { name: "reqvire.remove_element", arguments: { element_name: "Empty Branch", dry_run: false, worktree_id: contexts.original } } }),
      });
      const body = await response.json(); assert.ok(response.ok && !body.error && !body.result?.isError, JSON.stringify(body));
      await waitFor(() => document.body.textContent.includes("selected capability is no longer available") && location.hash === "#/coverage?scope=");
      const refreshed = structuredClone(reports.whole);
      refreshed.capability_coverage.capabilities = refreshed.capability_coverage.capabilities.filter(row => row.name !== "Empty Branch");
      await scope("", contexts.original, refreshed);
      assert.equal(await evaluate(() => Boolean(document.querySelector('[data-capability-id][aria-label="Empty Branch"]'))), false);
    });
  });
}
