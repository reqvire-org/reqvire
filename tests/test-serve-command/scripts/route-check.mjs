import { spawn } from "node:child_process";
import { readFile } from "node:fs/promises";
import path from "node:path";

// Drive the real served bundle through Chrome's debugging protocol using
// Node's built-in WebSocket, without adding a separate test framework.
const [browser, baseUrl, profile] = process.argv.slice(2);
const child = spawn(browser, [
  "--headless", "--no-sandbox", "--disable-dev-shm-usage",
  "--use-angle=swiftshader", "--enable-unsafe-swiftshader",
  "--no-first-run", "--no-default-browser-check", "--remote-debugging-port=0",
  "--disable-background-networking", "--disable-background-timer-throttling",
  "--disable-renderer-backgrounding",
  `--user-data-dir=${profile}`, "about:blank",
], { stdio: ["ignore", "ignore", "inherit"] });
let socket;
const pause = () => new Promise(resolve => setTimeout(resolve, 50));

try {
  let port;
  for (let attempt = 0; attempt < 200; attempt++) {
    if (child.exitCode !== null) throw new Error(`Browser exited: ${child.exitCode}`);
    try {
      port = (await readFile(path.join(profile, "DevToolsActivePort"), "utf8")).split("\n")[0];
      break;
    } catch {
      await pause();
    }
  }
  if (!port) throw new Error("Browser debugging endpoint did not start");
  const pages = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
  const page = pages.find(target => target.type === "page");
  if (!page) throw new Error("Browser has no page target");
  socket = new WebSocket(page.webSocketDebuggerUrl);
  await new Promise((resolve, reject) => {
    socket.addEventListener("open", resolve, { once: true });
    socket.addEventListener("error", reject, { once: true });
  });
  let nextId = 0;
  const pending = new Map();
  const events = new Map();
  socket.addEventListener("message", event => {
    const message = JSON.parse(event.data);
    if (message.id) {
      const request = pending.get(message.id);
      pending.delete(message.id);
      if (message.error) request?.reject(new Error(message.error.message));
      else request?.resolve(message.result);
    } else if (events.has(message.method)) {
      events.get(message.method)(message.params);
      events.delete(message.method);
    }
  });
  function rpc(method, params = {}) {
    return new Promise((resolve, reject) => {
      const id = ++nextId;
      pending.set(id, { resolve, reject });
      socket.send(JSON.stringify({ id, method, params }));
    });
  }
  async function evaluate(fn, ...args) {
    const result = await rpc("Runtime.evaluate", {
      expression: `(${fn.toString()})(${args.map(arg => JSON.stringify(arg)).join(",")})`,
      awaitPromise: true,
      returnByValue: true,
    });
    if (result.exceptionDetails) throw new Error(result.exceptionDetails.text);
    return result.result.value;
  }
  await rpc("Page.enable");

  let navigationCount = 0;
  async function navigate(route) {
    const loaded = new Promise(resolve => events.set("Page.loadEventFired", resolve));
    await rpc("Page.navigate", { url: `${baseUrl}/?route-e2e=${++navigationCount}${route}` });
    await loaded;
    await evaluate(async () => {
      for (let attempt = 0; attempt < 100; attempt++) {
        if (document.querySelector('[data-product-pattern="app-shell"]')) {
          await new Promise(resolve => setTimeout(resolve, 100));
          return;
        }
        await new Promise(resolve => setTimeout(resolve, 50));
      }
      throw new Error("Explorer shell did not render");
    });
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
  console.log("completed");
  if (failed) process.exitCode = 1;
} finally {
  socket?.close();
  child.kill();
}
