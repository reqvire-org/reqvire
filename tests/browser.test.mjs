import assert from 'node:assert/strict';
import { EventEmitter } from 'node:events';
import { setImmediate } from 'node:timers/promises';
import { test } from 'node:test';
import { stopBrowser } from './browser.mjs';
import * as support from './browser.mjs';
import { chmod, mkdir, mkdtemp, readFile, rm, stat, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';

async function directory(t) {
  const root = await mkdtemp(path.join(tmpdir(), 'reqvire-browser-support-'));
  t.after(() => rm(root, { recursive: true, force: true }));
  return root;
}

test('one executable resolver honors overrides, PATH, spaces, and invalid explicit values', async t => {
  const root = await directory(t);
  const configured = path.join(root, 'configured browser');
  const fallback = path.join(root, 'chromium');
  for (const file of [configured, fallback]) { await writeFile(file, '#!/bin/sh\n'); await chmod(file, 0o755); }
  assert.equal(typeof support.resolveBrowser, 'function');
  assert.equal(support.resolveBrowser({ REQVIRE_TEST_BROWSER: configured, CHROME_BIN: fallback, PATH: '' }), configured);
  assert.equal(support.resolveBrowser({ CHROME_BIN: configured, PATH: '' }), configured);
  assert.equal(support.resolveBrowser({ PATH: root }), fallback);
  assert.equal(support.resolveBrowser({ REQVIRE_TEST_BROWSER: 'chromium', PATH: root }), fallback);
  assert.throws(() => support.resolveBrowser({ REQVIRE_TEST_BROWSER: '/missing/browser', PATH: root }), /REQVIRE_TEST_BROWSER/);
  assert.throws(() => support.resolveBrowser({ REQVIRE_TEST_BROWSER: root, PATH: root }), /executable/);
  assert.throws(() => support.resolveBrowser({ PATH: '' }), /REQVIRE_TEST_BROWSER/);
});

class Socket extends EventTarget {
  readyState = 1;
  sent = [];
  send(text) { this.sent.push(JSON.parse(text)); }
  reply(message) { this.dispatchEvent(new MessageEvent('message', { data: JSON.stringify(message) })); }
  close() { this.readyState = 3; this.dispatchEvent(new Event('close')); }
}

test('debugging commands correlate responses, preserve sessions, and reject protocol errors', async () => {
  const socket = new Socket();
  const client = support.browserConnection(socket);
  const first = client.rpc('A', {}, 'secondary');
  const second = client.rpc('B');
  assert.equal(socket.sent[0].sessionId, 'secondary');
  const rejected = assert.rejects(second, /rejected command/);
  socket.reply({ id: socket.sent[1].id, error: { message: 'rejected command' } });
  socket.reply({ id: socket.sent[0].id, result: { value: 5 } });
  assert.deepEqual(await first, { value: 5 });
  await rejected;
  client.close();
});

test('connection loss rejects both pending commands and navigation', async () => {
  const socket = new Socket();
  const client = support.browserConnection(socket);
  const command = assert.rejects(client.rpc('Runtime.evaluate'), /closed/);
  const navigation = assert.rejects(client.loadPage('about:blank'), /closed/);
  socket.close();
  await Promise.all([command, navigation]);
  await assert.rejects(client.rpc('Page.enable'), /closed/);
});

test('debugging commands and navigation time out without leaving waiters', async () => {
  const socket = new Socket();
  const client = support.browserConnection(socket, 10);
  await assert.rejects(client.rpc('NeverReturns'), /timed out/);
  const loading = client.loadPage('about:blank');
  const rejected = assert.rejects(loading, /timed out/);
  socket.reply({ id: socket.sent.at(-1).id, result: {} });
  await rejected;
  client.close();
});

test('navigation handles a load event arriving before its command response', async () => {
  const socket = new Socket();
  const client = support.browserConnection(socket);
  const loading = client.loadPage('about:blank');
  socket.reply({ method: 'Page.loadEventFired', params: {} });
  socket.reply({ id: socket.sent.at(-1).id, result: {} });
  await loading;
  client.close();
});

test('rejected navigation does not wait for a load event that will never arrive', async () => {
  const socket = new Socket();
  const client = support.browserConnection(socket, 10);
  const rejected = assert.rejects(client.loadPage('invalid:'), /navigation failed: invalid URL/);
  socket.reply({ id: socket.sent.at(-1).id, result: { errorText: 'invalid URL' } });
  await rejected;
  client.close();
});

test('reload waits for the primary load event, handles early events and fails on timeout or disconnect', async () => {
  for (const outcome of ['loaded', 'timeout', 'closed']) {
    const socket = new Socket();
    const client = support.browserConnection(socket, 10);
    const loading = client.reloadPage();
    assert.equal(socket.sent.at(-1).method, 'Page.reload');
    const result = outcome === 'loaded' ? loading : assert.rejects(loading, /timed out|closed/);
    if (outcome === 'loaded') socket.reply({ method: 'Page.loadEventFired', params: {} });
    socket.reply({ id: socket.sent.at(-1).id, result: {} });
    if (outcome === 'closed') socket.close();
    await result;
    client.close();
  }
});

async function fakeBrowser(t) {
  const root = await directory(t);
  const executable = path.join(root, 'test browser');
  await writeFile(executable, `#!${process.execPath}
const fs = require('node:fs');
const path = require('node:path');
const profile = process.argv.find(arg => arg.startsWith('--user-data-dir=')).split('=').slice(1).join('=');
fs.writeFileSync(path.join(profile, 'pid'), String(process.pid));
fs.writeFileSync(path.join(profile, 'DevToolsActivePort'), '1234\\n');
setInterval(() => {}, 1000);
`);
  await chmod(executable, 0o755);
  const previous = process.env.REQVIRE_TEST_BROWSER;
  process.env.REQVIRE_TEST_BROWSER = executable;
  t.after(() => {
    if (previous === undefined) delete process.env.REQVIRE_TEST_BROWSER;
    else process.env.REQVIRE_TEST_BROWSER = previous;
  });
  return { root, executable };
}

function fakeTransport(t, profile) {
  const previousSocket = globalThis.WebSocket;
  t.after(() => { globalThis.WebSocket = previousSocket; });
  t.mock.method(globalThis, 'fetch', async () => Response.json([{ type: 'page', webSocketDebuggerUrl: 'ws://fake.invalid' }]));
  globalThis.WebSocket = class extends Socket {
    constructor() { super(); queueMicrotask(() => this.dispatchEvent(new Event('open'))); }
    send(text) {
      super.send(text);
      const request = this.sent.at(-1);
      queueMicrotask(() => {
        this.reply({ id: request.id, result: {} });
        if (request.method === 'Browser.close') readFile(path.join(profile, 'pid'), 'utf8')
          .then(pid => process.kill(Number(pid), 'SIGTERM'));
      });
    }
  };
}

for (const fails of [false, true]) {
  test(`real child exits before ${fails ? 'retaining failed' : 'removing successful'} profile`, async t => {
    const { root } = await fakeBrowser(t);
    const profile = path.join(root, 'profile');
    fakeTransport(t, profile);
    let pid;
    const check = support.withBrowser(profile, async () => {
      pid = Number(await readFile(path.join(profile, 'pid'), 'utf8'));
      if (fails) throw new Error('assertion failed');
    });
    if (fails) await assert.rejects(check, /assertion failed/); else await check;
    assert.throws(() => process.kill(pid, 0), { code: 'ESRCH' });
    if (fails) assert.ok((await stat(profile)).isDirectory());
    else await assert.rejects(stat(profile), { code: 'ENOENT' });
  });
}

test('second browser startup failure cleans the first and preserves diagnostics', async t => {
  const { root } = await fakeBrowser(t);
  const first = path.join(root, 'first');
  const second = path.join(root, 'second');
  fakeTransport(t, first);
  const exits = path.join(root, 'exits');
  await writeFile(exits, '#!/bin/sh\nexit 17\n');
  await chmod(exits, 0o755);
  let pid;
  await assert.rejects(support.withBrowser(first, async () => {
    pid = Number(await readFile(path.join(first, 'pid'), 'utf8'));
    process.env.REQVIRE_TEST_BROWSER = exits;
    await support.withBrowser(second, () => assert.fail('must not enter after failed startup'));
  }), /Browser exited: 17/);
  assert.throws(() => process.kill(pid, 0), { code: 'ESRCH' });
  assert.ok((await stat(first)).isDirectory());
  assert.ok((await stat(second)).isDirectory());
});

test('spawn errors are reported and caller-owned profiles are never deleted', async t => {
  const { root, executable } = await fakeBrowser(t);
  await writeFile(executable, '#!/does-not-exist\n');
  const profile = path.join(root, 'spawn-failure');
  await assert.rejects(support.openBrowser(profile), /ENOENT/);
  assert.ok((await stat(profile)).isDirectory());
  const existing = path.join(root, 'existing');
  await mkdir(existing);
  await writeFile(path.join(existing, 'evidence'), 'keep');
  await assert.rejects(support.openBrowser(existing), /EEXIST/);
  assert.equal(await readFile(path.join(existing, 'evidence'), 'utf8'), 'keep');
});

function childProcess() {
  const child = new EventEmitter();
  child.exitCode = null;
  child.signalCode = null;
  child.signals = [];
  child.kill = signal => { child.signals.push(signal); return true; };
  child.finish = signal => {
    child.exitCode = signal ? null : 0;
    child.signalCode = signal ?? null;
    child.emit('exit', child.exitCode, child.signalCode);
  };
  return child;
}

test('graceful browser close waits for process exit before profile cleanup', async () => {
  const child = childProcess();
  let requestSent = false, finished = false;
  const closing = stopBrowser(child, async () => { requestSent = true; })
    .then(() => { finished = true; });
  await setImmediate();
  assert.equal(requestSent, true);
  assert.equal(finished, false);
  assert.deepEqual(child.signals, []);
  child.finish();
  await closing;
  assert.equal(finished, true);
  assert.equal(child.listenerCount('exit'), 0);
});

test('lost debugging connection falls back to termination and waits for exit', async () => {
  const child = childProcess();
  child.kill = signal => {
    child.signals.push(signal);
    setImmediate().then(() => child.finish(signal));
    return true;
  };
  await stopBrowser(child, async () => { throw new Error('Debugging connection closed'); }, 10);
  assert.deepEqual(child.signals, ['SIGTERM']);
  assert.equal(child.signalCode, 'SIGTERM');
  assert.equal(child.listenerCount('exit'), 0);
});

test('unresponsive browser is killed after bounded graceful and termination waits', async () => {
  const child = childProcess();
  child.kill = signal => {
    child.signals.push(signal);
    if (signal === 'SIGKILL') child.finish(signal);
    return true;
  };
  await stopBrowser(child, async () => {}, 10);
  assert.deepEqual(child.signals, ['SIGTERM', 'SIGKILL']);
  assert.equal(child.listenerCount('exit'), 0);
});

test('a close request that never settles cannot block process termination', async () => {
  const child = childProcess();
  child.kill = signal => { child.signals.push(signal); child.finish(signal); return true; };
  await stopBrowser(child, () => new Promise(() => {}), 10);
  assert.deepEqual(child.signals, ['SIGTERM']);
});

test('already exited browser needs no protocol request or signal', async () => {
  const child = childProcess();
  child.finish();
  await stopBrowser(child, async () => { assert.fail('Must not contact exited browser'); });
  assert.deepEqual(child.signals, []);
});
