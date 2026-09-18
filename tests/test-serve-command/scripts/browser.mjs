import { spawn } from "node:child_process";
import { readFile } from "node:fs/promises";
import path from "node:path";

// The serve E2Es use the real compiled bundle and Chromium's debugging
// protocol, with Node's built-in WebSocket and no separate test framework.
export async function openBrowser(browser, profile) {
  const child = spawn(browser, [
    "--headless", "--no-sandbox", "--disable-dev-shm-usage",
    "--use-angle=swiftshader", "--enable-unsafe-swiftshader",
    "--no-first-run", "--no-default-browser-check", "--remote-debugging-port=0",
    "--disable-background-networking", "--disable-background-timer-throttling",
    "--disable-renderer-backgrounding",
    `--user-data-dir=${profile}`, "about:blank",
  ], { stdio: ["ignore", "ignore", "inherit"] });
  let socket;
  const close = async () => {
    socket?.close();
    child.kill();
    if (child.exitCode === null && child.signalCode === null) {
      await new Promise(resolve => child.once("exit", resolve));
    }
  };
  try {
    let port;
    for (let attempt = 0; attempt < 200; attempt++) {
      if (child.exitCode !== null) throw new Error(`Browser exited: ${child.exitCode}`);
      try {
        port = (await readFile(path.join(profile, "DevToolsActivePort"), "utf8")).split("\n")[0];
        break;
      } catch {
        await new Promise(resolve => setTimeout(resolve, 50));
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
        clearTimeout(request?.timer);
        if (message.error) request?.reject(new Error(message.error.message));
        else request?.resolve(message.result);
      } else if (events.has(message.method)) {
        events.get(message.method)(message.params);
        events.delete(message.method);
      }
    });
    socket.addEventListener("close", () => {
      for (const request of pending.values()) {
        clearTimeout(request.timer);
        request.reject(new Error("Browser debugging connection closed"));
      }
      pending.clear();
    });
    function rpc(method, params = {}) {
      return new Promise((resolve, reject) => {
        if (socket.readyState !== WebSocket.OPEN) {
          reject(new Error("Browser debugging connection is not open"));
          return;
        }
        const id = ++nextId;
        const timer = setTimeout(() => {
          pending.delete(id);
          reject(new Error(`Browser command timed out: ${method}`));
        }, 15000);
        pending.set(id, { resolve, reject, timer });
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
    async function navigate(url) {
      let timer;
      const loaded = new Promise((resolve, reject) => {
        timer = setTimeout(() => reject(new Error("Browser navigation timed out")), 20000);
        events.set("Page.loadEventFired", () => { clearTimeout(timer); resolve(); });
      });
      try {
        await rpc("Page.navigate", { url });
        await loaded;
      } finally {
        clearTimeout(timer);
        events.delete("Page.loadEventFired");
      }
      await waitFor(async () => evaluate(() => Boolean(document.querySelector('[data-product-pattern="app-shell"]'))));
    }
    return { rpc, evaluate, navigate, close };
  } catch (error) {
    await close();
    throw error;
  }
}

export async function waitFor(check, timeout = 12000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    if (await check()) return;
    await new Promise(resolve => setTimeout(resolve, 100));
  }
  throw new Error("Timed out waiting for the expected browser or server state");
}
