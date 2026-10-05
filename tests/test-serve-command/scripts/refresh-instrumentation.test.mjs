import assert from 'node:assert/strict';
import { test } from 'node:test';
import vm from 'node:vm';
import { instrument } from './refresh-instrumentation.mjs';

function harness() {
  const calls = [];
  const window = {
    fetch: async (...args) => {
      calls.push(args);
      return Response.json({ chunks: { hash: 'original chunk' } });
    },
  };
  vm.runInNewContext(`(${instrument.toString()})();`, {
    window, location: new URL('http://example.test/?worktree_id=branch-a'),
    URL, Request, Response,
  });
  return { window, calls };
}

for (const endpoint of [
  '/api/project-store/chunks',
  '/api/project-store/chunks?worktree_id=branch-a',
  'http://example.test/api/project-store/chunks?worktree_id=branch-a',
]) {
  test(`records requested chunk hashes: ${endpoint}`, async () => {
    const { window } = harness();
    await window.fetch(endpoint, { method: 'POST', body: JSON.stringify({ revision: 'r', hashes: ['hash'] }) });
    const [entry] = window.reqvireManifestTraffic;
    assert.ok(entry);
    assert.equal(entry.url, endpoint);
    assert.equal(JSON.stringify(entry.hashes), '["hash"]');
    assert.equal(entry.status, 200);
  });
}

for (const fault of ['missing', 'corrupt']) {
  test(`injects ${fault} chunks for selected worktrees`, async () => {
    const { window } = harness();
    window.reqvireChunkFault = fault;
    const response = await window.fetch('/api/project-store/chunks?worktree_id=branch-a', {
      method: 'POST', body: JSON.stringify({ revision: 'r', hashes: ['hash'] }),
    });
    const chunks = (await response.json()).chunks;
    assert.equal(chunks.hash, fault === 'missing' ? undefined : 'original chunk ');
    assert.equal(window.reqvireManifestTraffic[0].fault, fault);
    assert.equal(window.reqvireChunkFault, null);
  });
}

test('selected worktree chunks can be held and released', async () => {
  const { window, calls } = harness();
  window.reqvireHoldNextChunks = true;
  const request = window.fetch('/api/project-store/chunks?worktree_id=branch-a', {
    method: 'POST', body: JSON.stringify({ hashes: ['hash'] }),
  });
  assert.equal(window.reqvireHeldChunks, true);
  assert.equal(calls.length, 0);
  window.reqvireReleaseChunks();
  await request;
  assert.equal(calls.length, 1);
  assert.equal(window.reqvireHeldChunks, false);
});

test('unrelated traffic is neither recorded nor faulted', async () => {
  const { window, calls } = harness();
  window.reqvireChunkFault = 'missing';
  await window.fetch('/assets/explorer.js');
  await window.fetch('https://other.example/api/project-store/chunks');
  assert.equal(window.reqvireManifestTraffic.length, 0);
  assert.equal(window.reqvireChunkFault, 'missing');
  assert.equal(calls.length, 2);
});
