import { spawn } from 'node:child_process';
import { accessSync, constants, statSync } from 'node:fs';
import { mkdir, readFile, rm } from 'node:fs/promises';
import path from 'node:path';

export function resolveBrowser(env = process.env) {
  const executable = name => {
    const candidates = name.includes(path.sep) ? [path.resolve(name)]
      : (env.PATH ?? '').split(path.delimiter).filter(Boolean).map(dir => path.resolve(dir, name));
    return candidates.find(file => {
      try { accessSync(file, constants.X_OK); return statSync(file).isFile(); }
      catch { return false; }
    });
  };
  for (const key of ['REQVIRE_TEST_BROWSER', 'CHROME_BIN']) {
    if (!env[key]) continue;
    const file = executable(env[key]);
    if (!file) throw new Error(`${key} is not an executable browser: ${env[key]}`);
    return file;
  }
  for (const name of ['chromium', 'chromium-browser', 'google-chrome']) {
    const file = executable(name);
    if (file) return file;
  }
  throw new Error('Chrome/Chromium is required; set REQVIRE_TEST_BROWSER to its executable');
}

const pause = ms => new Promise(resolve => setTimeout(resolve, ms));
const exited = child => child.exitCode !== null || child.signalCode !== null;

// Reap the browser before any profile removal, even if its CDP connection died.
export async function stopBrowser(child, requestClose, timeout = 2000) {
  const waitForExit = () => {
    if (exited(child)) return Promise.resolve(true);
    return new Promise(resolve => {
      const done = () => { clearTimeout(timer); child.off('exit', done); resolve(exited(child)); };
      const timer = setTimeout(done, timeout);
      child.once('exit', done);
    });
  };
  if (exited(child)) return;
  let timer;
  try {
    await Promise.race([
      Promise.resolve().then(requestClose).catch(() => {}),
      new Promise(resolve => { timer = setTimeout(resolve, timeout); }),
    ]);
  } finally { clearTimeout(timer); }
  if (await waitForExit()) return;
  child.kill('SIGTERM');
  if (await waitForExit()) return;
  child.kill('SIGKILL');
  if (!await waitForExit()) throw new Error('Browser process did not exit during cleanup');
}

// Transport is separate so failures can be exercised without a listening socket.
export function browserConnection(socket, timeout = 15000) {
  let nextId = 0, failure;
  const pending = new Map(), events = new Map(), listeners = new Map();
  function settle(map, key, error, result) {
    const request = map.get(key);
    if (!request) return;
    map.delete(key);
    clearTimeout(request.timer);
    if (error) request.reject(error); else request.resolve(result);
  }
  function fail(error) {
    failure ??= error;
    for (const key of pending.keys()) settle(pending, key, failure);
    for (const key of events.keys()) settle(events, key, failure);
  }
  function wait(map, key, label, duration) {
    const promise = new Promise((resolve, reject) => {
      if (failure) { reject(failure); return; }
      const timer = setTimeout(() => settle(map, key, new Error(`Browser ${label} timed out`)), duration);
      map.set(key, { resolve, reject, timer });
    });
    return promise;
  }
  const message = event => {
    let value;
    try { value = JSON.parse(event.data); }
    catch { fail(new Error('Invalid browser debugging response')); return; }
    if (value.id) settle(pending, value.id, value.error && new Error(value.error.message), value.result);
    else {
      // Events from a secondary target must not satisfy primary-page navigation.
      if (!value.sessionId) settle(events, value.method, null, value.params);
      listeners.get(value.method)?.(value.params);
    }
  };
  const closed = () => fail(new Error('Browser debugging connection closed'));
  socket.addEventListener('message', message);
  socket.addEventListener('close', closed);
  socket.addEventListener('error', closed);
  function rpc(method, params = {}, sessionId, duration = timeout) {
    if (socket.readyState !== 1) closed();
    const id = ++nextId;
    const promise = wait(pending, id, `command ${method}`, duration);
    if (!failure) {
      try { socket.send(JSON.stringify({ id, method, params, ...(sessionId ? { sessionId } : {}) })); }
      catch (error) { settle(pending, id, error); }
    }
    return promise;
  }
  async function loadDocument(method, params = {}) {
    const loaded = wait(events, 'Page.loadEventFired', 'navigation', timeout);
    try {
      await Promise.all([rpc(method, params).then(result => {
        if (result.errorText) throw new Error(`Browser navigation failed: ${result.errorText}`);
      }), loaded]);
    } finally { settle(events, 'Page.loadEventFired', new Error('Navigation ended')); }
  }
  async function evaluate(fn, ...args) {
    const response = await rpc('Runtime.evaluate', {
      expression: `(${fn.toString()})(${args.map(arg => JSON.stringify(arg)).join(',')})`,
      awaitPromise: true, returnByValue: true,
    });
    if (response.exceptionDetails) {
      const { exception, text } = response.exceptionDetails;
      throw new Error(exception?.description ?? exception?.value ?? text);
    }
    return response.result.value;
  }
  return {
    rpc, evaluate,
    loadPage: url => loadDocument('Page.navigate', { url }),
    reloadPage: () => loadDocument('Page.reload'),
    on: (event, callback) => listeners.set(event, callback),
    close() {
      closed();
      socket.removeEventListener('message', message);
      socket.removeEventListener('close', closed);
      socket.removeEventListener('error', closed);
      listeners.clear();
      socket.close();
    },
  };
}

export async function openBrowser(profile, { gpuBackend = 'swiftshader' } = {}) {
  if (!['swiftshader', 'native'].includes(gpuBackend)) throw new Error(`Unknown browser GPU backend: ${gpuBackend}`);
  const browser = resolveBrowser();
  // Only remove profiles we created; never reuse/delete a caller's existing one.
  await mkdir(profile);
  const env = { ...process.env };
  for (const key of ['DISPLAY', 'WAYLAND_DISPLAY', 'XAUTHORITY']) delete env[key];
  let child, socket, client, closing, spawnError;
  const close = ({ preserveProfile = false } = {}) => closing ??= (async () => {
    try {
      if (child?.pid) await stopBrowser(child, () => client?.rpc('Browser.close', {}, undefined, 2000));
    } finally { client ? client.close() : socket?.close(); }
    if (preserveProfile) console.error(`Browser profile retained: ${profile}`);
    else await rm(profile, { recursive: true, force: true, maxRetries: 10, retryDelay: 100 });
  })();
  try {
    child = spawn(browser, [
      '--headless', '--no-sandbox', '--disable-dev-shm-usage',
      '--use-gl=angle',
      ...(gpuBackend === 'swiftshader'
        ? ['--use-angle=swiftshader-webgl', '--enable-unsafe-swiftshader', '--disable-vulkan']
        : ['--enable-gpu', '--use-angle=vulkan', '--enable-features=Vulkan', '--disable-vulkan-surface']),
      '--ozone-platform=headless',
      '--no-first-run', '--no-default-browser-check', '--remote-debugging-port=0',
      '--disable-background-networking', '--disable-background-timer-throttling',
      '--disable-renderer-backgrounding', `--user-data-dir=${profile}`, 'about:blank',
    ], { stdio: ['ignore', 'ignore', 'inherit'], env });
    child.on('error', error => { spawnError = error; });
    let port;
    for (let attempt = 0; attempt < 200; attempt++) {
      if (spawnError) throw spawnError;
      if (exited(child)) throw new Error(`Browser exited: ${child.exitCode ?? child.signalCode}`);
      try {
        port = (await readFile(path.join(profile, 'DevToolsActivePort'), 'utf8')).split('\n')[0];
        if (/^[0-9]+$/.test(port)) break;
      } catch { /* Chromium has not published the port file yet. */ }
      port = undefined;
      await pause(50);
    }
    if (!port) throw new Error('Browser debugging endpoint did not start');
    const response = await fetch(`http://127.0.0.1:${port}/json/list`, { signal: AbortSignal.timeout(10000) });
    if (!response.ok) throw new Error(`Browser target discovery failed: HTTP ${response.status}`);
    const page = (await response.json()).find(target => target.type === 'page');
    if (!page) throw new Error('Browser has no page target');
    socket = new WebSocket(page.webSocketDebuggerUrl);
    client = browserConnection(socket);
    await new Promise((resolve, reject) => {
      const finish = error => {
        clearTimeout(timer);
        socket.removeEventListener('open', opened);
        socket.removeEventListener('error', failed);
        socket.removeEventListener('close', failed);
        if (error) reject(error); else resolve();
      };
      const opened = () => finish();
      const failed = () => finish(new Error('Browser debugging connection failed to open'));
      const timer = setTimeout(() => finish(new Error('Browser debugging connection timed out')), 10000);
      socket.addEventListener('open', opened);
      socket.addEventListener('error', failed);
      socket.addEventListener('close', failed);
    });
    await client.rpc('Page.enable');
    return {
      ...client, close,
      async navigate(url) {
        await client.loadPage(url);
        await waitFor(() => client.evaluate(() => Boolean(document.querySelector('[data-product-pattern="app-shell"]'))));
      },
    };
  } catch (error) {
    try { await close({ preserveProfile: true }); }
    catch (cleanupError) { throw new AggregateError([error, cleanupError], 'Browser startup and cleanup failed'); }
    throw error;
  }
}

export async function withBrowser(profile, check, options) {
  const browser = await openBrowser(profile, options);
  let failure;
  try { return await check(browser); }
  catch (error) { failure = error; throw error; }
  finally {
    try { await browser.close({ preserveProfile: Boolean(failure || process.exitCode) }); }
    catch (error) {
      if (failure) throw new AggregateError([failure, error], 'Browser check and cleanup failed');
      throw error;
    }
  }
}

export async function waitFor(check, timeout = 12000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    if (await check()) return;
    await pause(100);
  }
  throw new Error('Timed out waiting for the expected browser or server state');
}
