import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { withBrowser, waitFor } from '../browser.mjs';

const [base, original, secondary, root, secondRoot, profile] = process.argv.slice(2);
const git = (folder, ...args) => execFileSync('git', args, { cwd: folder, encoding: 'utf8', env: { ...process.env, GIT_OPTIONAL_LOCKS: '0' } }).trim();
function commit(folder) {
  git(folder, 'add', '.');
  git(folder, '-c', 'commit.gpgSign=false', 'commit', '-qm', 'External browser update');
  return git(folder, 'rev-parse', 'HEAD');
}

await withBrowser(profile, async browser => {
  await browser.navigate(`${base}/?branch=${encodeURIComponent(original)}#/model`);
  await waitFor(() => browser.evaluate(id => window.reqvireProjectStore?.project.branch === id
    && Boolean(document.querySelector('[data-product-pattern="app-shell"]')), original));
  const hasRefresh = await browser.evaluate(() => Boolean(document.querySelector('button[aria-label="Refresh HEAD"]')));
  assert.ok(hasRefresh, 'Served Explorer is missing the Refresh HEAD control');
  await browser.evaluate(() => {
    window.refreshCalls = [];
    const fetch = window.fetch;
    window.fetch = async (url, init) => {
      if (init?.body && typeof init.body === 'string') {
        try {
          const call = JSON.parse(init.body);
          if (call.method === 'tools/call' && call.params?.name === 'reqvire.worktree.refresh_head') window.refreshCalls.push(call.params.arguments);
        } catch { /* Asset requests do not have JSON-RPC bodies. */ }
      }
      return fetch(url, init);
    };
  });
  writeFileSync(join(root, 'impl.rs'), '// refreshed implementation for browser\n');
  const head = commit(root);
  await browser.evaluate(() => document.querySelector('button[aria-label="Refresh HEAD"]').click());
  await waitFor(() => browser.evaluate(head => window.reqvireProjectStore?.project.eligible_git_worktrees?.some(w => w.head === head), head));
  assert.ok(await browser.evaluate(() => window.refreshCalls.some(c => c.branch === window.reqvireProjectStore.project.branch && !('worktree_id' in c))), 'Refresh button bypassed the shared MCP command');
  assert.ok(await browser.evaluate(() => window.reqvireProjectStore.resources.some(r => r.source_text?.includes('refreshed implementation for browser'))), 'Refresh retained stale evidence source bytes');

  // Existing secondary context must adopt its branch tip on selection, without
  // forcing an explicit refresh first or replacing its opaque context identity.
  writeFileSync(join(secondRoot, 'impl.rs'), '// selected branch HEAD\n');
  const secondaryHead = commit(secondRoot);
  await browser.evaluate(() => document.querySelector('[role="combobox"][aria-label="Branch"]').click());
  await waitFor(() => browser.evaluate(id => Boolean(document.querySelector(`[data-worktree-id="${id}"]`)), secondary));
  await browser.evaluate(id => document.querySelector(`[data-worktree-id="${id}"]`).click(), secondary);
  await waitFor(() => browser.evaluate((id, head) => window.reqvireProjectStore?.project.branch === id
    && window.reqvireProjectStore.project.eligible_git_worktrees?.some(w => w.head === head), secondary, secondaryHead));
  assert.ok(await browser.evaluate(id => new URLSearchParams(location.search).get('branch') === id, secondary), 'HEAD selection replaced context identity');
  const retained = await browser.evaluate(() => JSON.stringify(window.reqvireProjectStore));
  const model = join(secondRoot, 'Model.md');
  writeFileSync(model, readFileSync(model, 'utf8') + '\n');
  commit(secondRoot);
  await browser.evaluate(() => document.querySelector('button[aria-label="Refresh HEAD"]').click());
  await waitFor(() => browser.evaluate(() => document.body.textContent.includes('Model.md')
    && /blocked|protected/i.test(document.body.textContent)));
  assert.equal(await browser.evaluate(() => JSON.stringify(window.reqvireProjectStore)), retained, 'Blocked refresh replaced the accepted view');
  assert.ok(await browser.evaluate(id => new URLSearchParams(location.search).get('branch') === id, secondary), 'Blocked refresh changed selection');
});
