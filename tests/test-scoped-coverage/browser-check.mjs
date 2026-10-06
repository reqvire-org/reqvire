import { readFile } from "node:fs/promises";
import { withBrowser } from "../browser.mjs";

const [servedUrl, exportedUrl, profile, reportsPath, addedPath] = process.argv.slice(2);
const reports = JSON.parse(await readFile(reportsPath, "utf8"));
const pause = () => new Promise(resolve => setTimeout(resolve, 50));
console.error("Starting scoped-coverage browser");
await withBrowser(profile, async ({ rpc, evaluate, loadPage, reloadPage, on }) => {
  on("Runtime.exceptionThrown", params => console.error(JSON.stringify(params)));
  await rpc("Runtime.enable");
  const defaultViewport = { width: 780, height: 844, deviceScaleFactor: 1, mobile: false };
  await rpc("Emulation.setDeviceMetricsOverride", defaultViewport);

  async function waitFor(fn, ...args) {
    console.error(`Waiting: ${fn.toString()}`);
    const deadline = Date.now() + 15000;
    while (Date.now() < deadline) {
      if (await evaluate(fn, ...args)) return;
      await pause();
    }
    throw new Error(`Browser condition timed out: ${fn.toString()}`);
  }
  async function navigate(baseUrl) {
    console.error(`Navigating: ${baseUrl}/#/coverage`);
    // A fresh document resets route overlays and disclosure state, while
    // retaining persisted scope for the checks that explicitly exercise it.
    await loadPage("about:blank");
    await loadPage(`${baseUrl}/#/coverage?scope=`);
    await waitFor(() => !!document.querySelector('[aria-label="Coverage mode"]'));
    await evaluate(async () => { await document.fonts.ready; });
  }
  async function select(name) {
    const identifier = name ? reports[name].scope.capability_identifier : "";
    await evaluate(identifier => {
      const input = identifier ? [...document.querySelectorAll('[data-capability-id]')].find(row => row.dataset.capabilityId === identifier)
        : document.querySelector('[data-coverage-whole-model]');
      if (!input) throw new Error(`Missing scope: ${identifier}`);
      input.click();
    }, identifier);
    await waitFor(identifier => (document.querySelector('[data-capability-id][aria-selected="true"]')?.dataset.capabilityId ?? "") === identifier, identifier);
  }
  async function click(text) {
    await evaluate(text => {
      const button = [...document.querySelectorAll("button")].find(button => button.textContent.trim() === text || button.getAttribute("aria-label") === text);
      if (!button) throw new Error(`Missing button: ${text}`);
      button.click();
    }, text);
  }
  async function assertTerminalLinks() {
    await evaluate(() => {
      const hierarchy = document.querySelector(".ux-coverage-drilldown");
      if (hierarchy.querySelector('a[href^="#/resources/"], a[href^="#/content/"]')
        || hierarchy.textContent.includes("Via dependencies")) throw new Error("Coverage hierarchy repeats artifact evidence");
      for (const name of ["Alpha Implemented", "Alpha Gap"]) {
        const terminal = document.querySelector(`article[aria-label="${name}"]`);
        if (!terminal || terminal.querySelector("button") || !terminal.querySelector("a")) {
          throw new Error(`Terminal ${name} must retain its name link without an expansion action`);
        }
      }
    });
  }
  async function assertCoverageAlignment() {
    await evaluate(() => {
      const rows = [...document.querySelectorAll(".ux-coverage-drilldown__row")];
      const columns = row => [...row.querySelectorAll(":scope > .ux-coverage-drilldown__metric")].map(cell => cell.getBoundingClientRect());
      const reference = columns(rows[0]);
      const same = (a, b) => Math.abs(a - b) < 1;
      for (const row of rows) {
        if (columns(row).some((column, index) => !same(column.left, reference[index].left) || !same(column.right, reference[index].right))) {
          throw new Error(`Nested coverage columns drift horizontally for ${row.closest("article").getAttribute("aria-label")}: ${JSON.stringify(columns(row).map(({ left, right }) => ({ left, right })))}; expected ${JSON.stringify(reference.map(({ left, right }) => ({ left, right })))}`);
        }
      }
      const details = [...document.querySelectorAll(".ux-coverage-drilldown__dependency")];
      for (const detail of details) {
        const status = detail.lastElementChild.getBoundingClientRect();
        if (!same(status.right, reference[1].right)) throw new Error(`Binding consumer status does not align with the implementation column: ${detail.textContent.trim()} ends at ${status.right}; expected ${reference[1].right}`);
      }
      const names = ["Alpha Parent", "Alpha Middle", "Alpha Implemented"];
      const positions = names.map(name => document.querySelector(`article[aria-label="${name}"] > .ux-coverage-drilldown__row .ux-coverage-drilldown__identity a`).getBoundingClientRect().left);
      if (positions.some((left, index) => index > 0 && left <= positions[index - 1])) {
        throw new Error(`Requirement identity indentation is missing: ${JSON.stringify(Object.fromEntries(names.map((name, index) => [name, positions[index]])))} at viewport ${innerWidth}`);
      }
    });
  }
  async function assertCoverageColumns(count) {
    await waitFor(count => [...document.querySelectorAll(".ux-coverage-drilldown__row")]
      .every(row => getComputedStyle(row).gridTemplateColumns.split(" ").length === count), count);
  }
  async function assertReport(report) {
    await waitFor(report => {
      const s = report.summary;
      const text = document.querySelector('[data-view="coverage"]')?.textContent ?? "";
      return text.includes(`${s.covered_terminal_requirements} / ${s.total_terminal_requirements} terminal requirements covered`)
        && text.includes(`${s.verified_leaf_requirements} / ${s.total_leaf_requirements} verified`)
        && text.includes(`${s.satisfied_test_verifications} / ${s.total_test_verifications} satisfied`);
    }, report);
    const errors = await evaluate(report => {
      const failures = [];
      const summary = report.summary;
      const typeLabels = { test: "Test", formal_proof: "Formal proof", analysis: "Analysis", inspection: "Inspection", demonstration: "Demonstration" };
      for (const [kind, label] of Object.entries(typeLabels)) {
        const row = [...document.querySelectorAll(".coverage-legend-row")].find(row => row.querySelector("span:nth-child(2)")?.textContent === label);
        if (row?.querySelector("strong")?.textContent !== String(summary.verification_types[kind])) failures.push(`Verification type ${label}`);
      }
      const sourceLabels = { direct_satisfied: "Direct evidence", requirement_rollup: "Requirement rollup", contract_consumer_rollup: "Contract consumer rollup", combined_rollup: "Combined rollup" };
      for (const [source, label] of Object.entries(sourceLabels)) {
        const row = [...document.querySelectorAll(".coverage-source-row__head")].find(row => row.firstElementChild?.textContent === label);
        if (row?.lastElementChild?.textContent !== String(summary.coverage_sources[source])) failures.push(`Coverage source ${label}`);
      }
      const sections = { "unverified-requirements": "unverified_leaf_requirements", "unimplemented-requirements": "uncovered_requirements", "unsatisfied-verifications": "unsatisfied_test_verifications", "orphaned-verifications": "orphaned_verifications" };
      for (const [id, field] of Object.entries(sections)) {
        const section = document.getElementById(`coverage-section-${id}`);
        if (report.scope && id === "orphaned-verifications") {
          if (section) failures.push("Scoped orphan list must be absent");
          continue;
        }
        const wanted = Object.values(report[field].files).flat().map(row => row.name).sort();
        const names = [...(section?.querySelectorAll('.coverage-gap-row__title') ?? [])].map(row => row.textContent).sort();
        if (JSON.stringify(names) !== JSON.stringify(wanted)) failures.push(`${id}: ${names} != ${wanted}`);
      }
      for (const row of document.querySelectorAll('[data-kind="capability"][data-coverage-depth]')) {
        const name = row.querySelector("a span:last-child")?.textContent;
        const expected = report.capability_coverage.capabilities.find(capability => capability.name === name);
        if (!expected || !row.textContent.includes(`${expected.aggregate_verified_leaf_requirements} / ${expected.aggregate_leaf_requirements}`)
          || !row.textContent.includes(`${expected.aggregate_covered_terminal_requirements} / ${expected.aggregate_terminal_requirements} terminal`)) failures.push(`Capability aggregates ${name}`);
      }
      if (report.scope && document.querySelector('[data-view="coverage"] h1')?.textContent !== report.scope.capability_name) failures.push("Missing explicit scope");
      return failures;
    }, report);
    if (errors.length) throw new Error(errors.join("; "));
  }
  async function assertRows(names, depths) {
    await waitFor((names, depths) => {
      const rows = [...document.querySelectorAll('[data-kind="capability"][data-coverage-depth]')];
      return JSON.stringify(rows.map(row => row.querySelector("a span:last-child").textContent)) === JSON.stringify(names)
        && (!depths || JSON.stringify(rows.map(row => Number(row.dataset.coverageDepth))) === JSON.stringify(depths));
    }, names, depths ?? null);
  }
  let failed = false;
  async function run(label, check) {
    const started = Date.now();
    console.error(`START ${label}`);
    try {
      await check();
      console.log(`${label}: PASS`);
    } catch (error) {
      failed = true;
      console.log(`${label}: FAIL`);
      console.error(`${label}: ${error.message}`);
      console.error(await evaluate(() => JSON.stringify({
        url: location.href, width: innerWidth,
        dialogLinks: [...document.querySelectorAll('[role="dialog"] a')].map(link => ({ text: link.textContent.trim(), href: link.getAttribute("href") })),
      })));
      console.error(await evaluate(() => document.body.innerText.slice(0, 1800)));
    } finally {
      console.error(`END ${label}: ${Date.now() - started}ms`);
      await rpc("Emulation.setDeviceMetricsOverride", defaultViewport);
    }
  }
  for (const [mode, url] of [["served", servedUrl], ["exported", exportedUrl]]) {
    const scenario = (label, check) => run(`${mode} ${label}`, async () => {
      await navigate(url);
      if (await evaluate(() => !!document.querySelector('button[aria-label="Expand explorer"]'))) {
        await click("Expand explorer");
      }
      await waitFor(() => !!document.querySelector('button[aria-label="Collapse explorer"]'));
      await check();
    });
    await scenario("defaults", async () => {
      await assertReport(reports.whole);
      await assertRows(["Alpha Root", "Empty Branch", "Alpha Left", "Shared Branch", "Alpha Right", "Beta Root"], [0, 1, 1, 2, 1, 0]);
      if (await evaluate(() => !!document.querySelector('[aria-label="Display"]'))) throw new Error("Unexpected display selector");
      if (!await evaluate(() => document.querySelector('.coverage-header h1')?.textContent === "Whole Model"
        && document.querySelector('[data-coverage-whole-model]')?.getAttribute("aria-selected") === "true"
        && !document.querySelector('select[aria-label="Scope"]'))) throw new Error("Missing Whole Model navigation");
    });
    await scenario("scope and sidebar parity", async () => {
      for (const name of ["Alpha Root", "Beta Root", "Alpha Left", "Shared Branch", "Empty Branch"]) {
        await select(name);
        await assertReport(reports[name]);
        const children = {
          "Alpha Root": [["Alpha Root", "Empty Branch", "Alpha Left", "Shared Branch", "Alpha Right"], [0, 1, 1, 2, 1]],
          "Alpha Left": [["Alpha Left", "Shared Branch"], [0, 1]],
        };
        await assertRows(...(children[name] ?? [[name], [0]]));
      }
      await waitFor(() => document.body.textContent.includes("No requirements in this capability scope"));
    });
    await scenario("hierarchy and evidence navigation", async () => {
      await select("Alpha Root");
      await assertRows(["Alpha Root", "Empty Branch", "Alpha Left", "Shared Branch", "Alpha Right"], [0, 1, 1, 2, 1]);
      await assertReport(reports["Alpha Root"]);
      const layoutValid = await evaluate(() => [...document.querySelectorAll(".coverage-legend-row,.coverage-source-row__head")].every(row => getComputedStyle(row).display === "flex")
        && [...document.querySelectorAll('[data-kind="capability"][data-coverage-depth]')].every(row => row.getBoundingClientRect().right <= row.closest(".coverage-panel").getBoundingClientRect().right));
      if (!layoutValid) throw new Error("Coverage rows overflow or lose their shared layout");
      await click("Expand Alpha Left");
      await click("Expand Alpha Contract Owner");
      await waitFor(() => document.querySelector('[aria-label="Coverage details for Alpha Contract Owner"] a')?.textContent.trim() === "Beta Consumer");
      await evaluate(() => {
        const details = document.querySelector('[aria-label="Coverage details for Alpha Contract Owner"]');
        const link = [...details.querySelectorAll("a")].find(link => link.textContent.trim() === "Beta Consumer");
        if (!link || !details.textContent.includes("Outside scope")) throw new Error("Missing external evidence link");
        link.click();
      });
      await waitFor(() => document.querySelector('[role="dialog"]')?.textContent.includes("Beta Consumer"));
      await click("Close");
      await waitFor(() => !document.querySelector('[role="dialog"]'));
      await assertReport(reports["Alpha Root"]);
      await click("Expand Shared Branch");
      await click("Expand Alpha Parent");
      await evaluate(() => {
        const gap = document.querySelector('article[aria-label="Alpha Gap"]');
        if (!gap || gap.querySelector("button") || !gap.querySelector("a")) throw new Error("Terminal gap must retain its name link without an expansion action");
        if (!gap.textContent.includes("Verification") || !gap.textContent.includes("Implementation") || gap.querySelector(".ux-coverage-drilldown__metric-head strong")) throw new Error("Terminal gap must use labelled coverage bars without redundant status text");
      });
      await click("Expand Alpha Middle");
      await assertTerminalLinks();
      await evaluate(() => {
        for (const [name, implementation] of [["Alpha Implemented", 100], ["Alpha Gap", 0]]) {
          const row = document.querySelector(`article[aria-label="${name}"] > .ux-coverage-drilldown__row`);
          const bars = [...row.querySelectorAll(".ds-bar")];
          const values = bars.map(bar => [bar.dataset.colorToken, Number(bar.querySelector("rect").getAttribute("width"))]);
          if (JSON.stringify(values) !== JSON.stringify([["--verification", 100], ["--resource", implementation]])) {
            throw new Error(`Incorrect terminal bars for ${name}: ${JSON.stringify(values)}`);
          }
          if (!row.textContent.includes("100% · 1 / 1 leaves") || !row.textContent.includes(`${implementation}% · ${implementation ? 1 : 0} / 1 terminal`)) {
            throw new Error(`Missing terminal coverage counts for ${name}`);
          }
        }
      });
      await evaluate(() => {
        for (const [name, leaves, terminals, covered] of [["Alpha Parent", 2, 2, 1], ["Alpha Middle", 1, 1, 1], ["Alpha Contract Owner", 1, 1, 1]]) {
          const row = document.querySelector(`article[aria-label="${name}"] > .ux-coverage-drilldown__row`);
          const values = [...row.querySelectorAll(".ds-bar")].map(bar => Number(bar.querySelector("rect").getAttribute("width")));
          if (JSON.stringify(values) !== JSON.stringify([100, covered / terminals * 100])
            || !row.textContent.includes(`${leaves} / ${leaves} leaves`)
            || !row.textContent.includes(`${covered} / ${terminals} terminal`)) {
            throw new Error(`Missing parent aggregate metrics for ${name}: ${row.textContent}`);
          }
        }
      });
      await assertCoverageColumns(2);
      await assertCoverageAlignment();
      await click("Collapse explorer");
      await assertCoverageColumns(2);
      await assertCoverageAlignment();
      await click("Expand explorer");
      await rpc("Emulation.setDeviceMetricsOverride", { ...defaultViewport, width: 1440 });
      await assertCoverageColumns(3);
      await assertCoverageAlignment();
      await rpc("Emulation.setDeviceMetricsOverride", defaultViewport);
      await assertCoverageColumns(2);
      await evaluate(() => {
        const root = document.documentElement;
        const originalTheme = root.getAttribute("data-theme");
        const originallyDark = root.classList.contains("dark");
        try {
          for (const theme of ["light", "dark"]) {
            root.dataset.theme = theme;
            root.classList.toggle("dark", theme === "dark");
            const rows = [...document.querySelectorAll(".ux-coverage-drilldown__row")];
            for (const row of document.querySelectorAll(".ux-coverage-drilldown__branch, .ux-coverage-drilldown__list li")) {
              const style = getComputedStyle(row);
              if ([style.borderTopWidth, style.borderBottomWidth].some(width => parseFloat(width) > 0)) {
                throw new Error(`Coverage row divider remains in ${theme} theme`);
              }
            }
            for (const list of document.querySelectorAll(".ux-coverage-drilldown__list")) {
              const shades = [...list.children].map(row => getComputedStyle(row).backgroundColor);
              if (shades.some((shade, index) => index > 0 && shade === shades[index - 1])) {
                throw new Error(`Dependency rows must alternate in ${theme} theme`);
              }
            }
            const colors = rows.map(row => getComputedStyle(row).backgroundColor);
            if (colors.some((color, index) => index > 0 && color === colors[index - 1])) {
              throw new Error(`Consecutive capability and requirement rows must alternate in ${theme} theme: ${JSON.stringify(rows.map((row, index) => ({ name: row.closest("article").getAttribute("aria-label"), tone: row.dataset.coverageTone, color: colors[index] })))}`);
            }
          }
        } finally {
          if (originalTheme === null) root.removeAttribute("data-theme");
          else root.dataset.theme = originalTheme;
          root.classList.toggle("dark", originallyDark);
        }
      });
      await evaluate(() => {
        document.querySelector('article[aria-label="Alpha Implemented"] > .ux-coverage-drilldown__row a').click();
      });
      await waitFor(() => document.querySelector('[role="dialog"]')?.textContent.includes("Alpha Implemented"));
      await evaluate(() => {
        const dialog = document.querySelector('[role="dialog"]');
        const file = [...dialog.querySelectorAll("a")].find(link => link.getAttribute("href") === "#/content/evidence/alpha.txt");
        if (!file || !dialog.textContent.includes("satisfiedBy")) throw new Error("Missing implementation evidence in element details");
        file.click();
      });
      await waitFor(() => location.hash === "#/content/evidence/alpha.txt"
        && !document.querySelector('[role="dialog"]')
        && document.body.textContent.includes("Synthetic implementation artifact for the alpha fixture."));
      await click("Coverage");
      await waitFor(() => document.querySelector('[data-capability-id][aria-selected="true"]')?.getAttribute("aria-label") === "Alpha Root");
      await assertReport(reports["Alpha Root"]);
    });
    await scenario("reload and orphan navigation", async () => {
      await select("Alpha Root");
      await reloadPage();
      await waitFor(() => document.querySelector('[data-capability-id][aria-selected="true"]')?.getAttribute("aria-label") === "Alpha Root");
      await assertRows(["Alpha Root", "Empty Branch", "Alpha Left", "Shared Branch", "Alpha Right"], [0, 1, 1, 2, 1]);
      await assertReport(reports["Alpha Root"]);
      await click("Capabilities");
      await assertReport(reports.whole);
      await waitFor(() => document.querySelector("#coverage-section-orphaned-verifications")?.textContent.includes("Orphan Check"));
    });
    await scenario("narrow coverage controls", async () => {
      await rpc("Emulation.setDeviceMetricsOverride", { width: 390, height: 844, deviceScaleFactor: 1, mobile: false });
      await click("Collapse explorer");
      await select("Alpha Root");
      await click("Expand Shared Branch");
      await click("Expand Alpha Parent");
      await click("Expand Alpha Middle");
      await assertTerminalLinks();
      await assertCoverageAlignment();
      await waitFor(() => {
        return [...document.querySelectorAll(".ux-coverage-drilldown__list li")].every(item => item.scrollWidth <= item.clientWidth)
          && [...document.querySelectorAll(".ux-coverage-drilldown__row")].every(row => getComputedStyle(row).gridTemplateColumns.split(" ").length === 2);
      });
      await click("Expand explorer");
    });
  }
  async function tool(name, args) {
    const response = await fetch(`${servedUrl}/mcp`, {
      signal: AbortSignal.timeout(15000),
      method: "POST", headers: { "Content-Type": "application/json", "Accept": "application/json, text/event-stream", "Mcp-Protocol-Version": "2025-11-25" },
      body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "tools/call", params: { name, arguments: args } }),
    });
    const body = await response.json();
    if (!response.ok || body.error || body.result?.isError) throw new Error(JSON.stringify(body));
  }
  await run("live refresh preserves scope and hierarchy", async () => {
    await navigate(servedUrl);
    await select("Alpha Root");
    await tool("reqvire.add_element", { file: "specifications/Added.md", content: await readFile(addedPath, "utf8"), dry_run: false });
    await waitFor(() => document.body.textContent.includes("1 / 4 terminal requirements covered"));
    await waitFor(() => document.querySelector("#coverage-section-unimplemented-requirements")?.textContent.includes("Added Alpha Gap"));
    await waitFor(() => document.querySelector('[data-capability-id][aria-selected="true"]')?.getAttribute("aria-label") === "Alpha Root");
    await assertRows(["Alpha Root", "Empty Branch", "Alpha Right", "Alpha Left", "Shared Branch"], [0, 1, 1, 1, 2]);
  });
  await run("live refresh removes selected scope with explanation", async () => {
    await select("Empty Branch");
    await tool("reqvire.remove_element", { element_name: "Empty Branch", dry_run: false });
    await waitFor(() => document.querySelector('[data-coverage-whole-model]')?.getAttribute("aria-selected") === "true" && document.body.textContent.includes("selected capability is no longer available"));
    await waitFor(() => document.body.textContent.includes("2 / 6 terminal requirements covered"));
    await assertRows(["Alpha Root", "Alpha Right", "Alpha Left", "Shared Branch", "Beta Root"], [0, 1, 1, 2, 0]);
  });
  if (failed) process.exitCode = 1;
});
