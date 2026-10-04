import assert from 'node:assert/strict';
import { EventEmitter } from 'node:events';
import { setImmediate } from 'node:timers/promises';
import { test } from 'node:test';
import { stopBrowser } from './browser.mjs';

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

test('already exited browser needs no protocol request or signal', async () => {
  const child = childProcess();
  child.finish();
  await stopBrowser(child, async () => { assert.fail('Must not contact exited browser'); });
  assert.deepEqual(child.signals, []);
});
