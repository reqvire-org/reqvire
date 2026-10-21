import assert from 'node:assert/strict';
import { withBrowser, waitFor } from '../browser.mjs';

const [base, profile] = process.argv.slice(2);
await withBrowser(profile, async browser => {
  await browser.navigate(`${base}/?branch=main#/model`);
  await waitFor(() => browser.evaluate(() => window.reqvireProjectStore?.project.branch === 'main'
    && Boolean(document.querySelector('[role="combobox"][aria-label="Branch"]'))));
  assert.ok(await browser.evaluate(() => window.reqvireProjectStore.elements
    .some(e => e.name === 'Component Specification' && e.id === 'repositories/component/Model.md#component-specification')),
  'The selected Central branch omitted its declared component');
  await browser.evaluate(() => document.querySelector('[role="combobox"][aria-label="Branch"]').click());
  await waitFor(() => browser.evaluate(() => Boolean(document.querySelector('[data-worktree-id="topic"]'))));
  await browser.evaluate(() => document.querySelector('[data-worktree-id="topic"]').click());
  await waitFor(() => browser.evaluate(() => window.reqvireProjectStore?.project.branch === 'topic'));
  assert.ok(await browser.evaluate(() => window.reqvireProjectStore.elements
    .some(e => e.name === 'Component Specification' && JSON.stringify(e).includes('omega'))),
  'The picker did not adopt the component revision selected by the topic manifest');
  assert.equal(await browser.evaluate(() => new URLSearchParams(location.search).get('branch')), 'topic');
});
