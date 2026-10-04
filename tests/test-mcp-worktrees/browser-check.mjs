import { mkdtemp, rm } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { openBrowser, waitFor } from '../test-serve-command/scripts/browser.mjs';

const [base, original, secondary, chromium, dirty] = process.argv.slice(2);
const profile = await mkdtemp(path.join(os.tmpdir(), 'reqvire-worktree-browser-'));
let browser, failure;
let stage = 'browser startup';
async function wait(check, description) {
  stage = description;
  await waitFor(check);
}
try {
  browser = await openBrowser(chromium, profile);
  await browser.navigate(`${base}/?worktree_id=${original}#/model`);
  await wait(() => browser.evaluate(id => window.reqvireProjectStore?.project.worktree_id === id, original), 'original model');
  // A second tab in the same browser shares storage, but not its model selection.
  const { targetId } = await browser.rpc('Target.createTarget', { url: `${base}/?worktree_id=${secondary}#/model` });
  const { sessionId } = await browser.rpc('Target.attachToTarget', { targetId, flatten: true });
  const other = async (fn, ...args) => {
    const response = await browser.rpc('Runtime.evaluate', {
      expression: `(${fn.toString()})(${args.map(JSON.stringify).join(',')})`, returnByValue: true, awaitPromise: true,
    }, sessionId);
    if (response.exceptionDetails) throw new Error(response.exceptionDetails.text);
    return response.result.value;
  };
  await wait(() => other(id => window.reqvireProjectStore?.project.worktree_id === id, secondary), 'second-tab model');
  // A real user activates the tab before switching its branch. Background tabs
  // deliberately defer model loads until visible.
  await browser.rpc('Page.bringToFront');
  await wait(() => browser.evaluate(() => document.visibilityState === 'visible'), 'original tab activation');
  const assert = (ok, message) => { if (!ok) throw new Error(message); };
  if (dirty) {
    await browser.evaluate(() => document.querySelector('.ux-worktree-selector [role="combobox"]').click());
    await wait(() => browser.evaluate(id => Boolean(document.querySelector(`[data-worktree-id="${id}"]`)), dirty), 'dirty branch choice');
    await browser.evaluate(id => document.querySelector(`[data-worktree-id="${id}"]`).click(), dirty);
    await wait(() => browser.evaluate(() => document.body.textContent.includes('clean')), 'dirty branch rejection');
    assert(await browser.evaluate(id => window.reqvireProjectStore.project.worktree_id === id, original), 'Dirty admission replaced the displayed branch');
    assert(await browser.evaluate(() => document.querySelector('.ux-worktree-selector [role="combobox"]').getAttribute('aria-busy') !== 'true'), 'Failed admission is still shown as loading');
    await browser.evaluate(() => history.back());
    await wait(() => browser.evaluate(id => new URLSearchParams(location.search).get('worktree_id') === id, original), 'history after rejection');
  }
  assert(await browser.evaluate(() => {
    const navigation = document.querySelector('nav[aria-label="Explorer views"]');
    const selector = document.querySelector('.ux-worktree-selector [role="combobox"]');
    return Boolean(navigation && selector
      && navigation.querySelectorAll('[role="tab"]').length === 5
      && selector.closest('[data-product-pattern-slot="start-pane"]'));
  }), 'Worktree picker must be inside the left pane and preserve Explorer navigation');
  assert(!await browser.evaluate(() => window.reqvireProjectStore.elements.some(e => e.name === 'Other Subject')), 'Original tab contains secondary model');
  assert(await other(() => window.reqvireProjectStore.elements.some(e => e.name === 'Other Subject')), 'Secondary tab has wrong model');
  for (const view of ['model', 'ontologies', 'traces', 'coverage', 'search']) {
    await other(view => { location.hash = '#/' + view; }, view);
    await wait(() => other(view => location.hash === '#/' + view && Boolean(document.querySelector('main')), view), `second-tab ${view} view`);
    assert(await other(id => window.reqvireProjectStore.project.worktree_id === id, secondary), `${view} lost context`);
    assert(await browser.evaluate(id => window.reqvireProjectStore.project.worktree_id === id, original), `${view} changed the other tab`);
  }
  // Show a detail overlay before changing contexts: it must close on adoption.
  await browser.evaluate(() => { location.hash = '#/elements/' + encodeURIComponent(window.reqvireProjectStore.elements[0].id); });
  await wait(() => browser.evaluate(() => Boolean(document.querySelector('[role="dialog"]'))), 'element detail dialog');
  await browser.evaluate(id => {
    const select = document.querySelector('.ux-worktree-selector [role="combobox"]');
    if (!select) throw new Error('Missing worktree selector');
    select.click();
  }, secondary);
  await wait(() => browser.evaluate(id => Boolean([...document.querySelectorAll('[role="option"]')].find(option => option.dataset.worktreeId === id)), secondary), 'secondary branch choice');
  await browser.evaluate(id => [...document.querySelectorAll('[role="option"]')].find(option => option.dataset.worktreeId === id).click(), secondary);
  await wait(() => browser.evaluate(id => window.reqvireProjectStore.project.worktree_id === id && !document.querySelector('[role="dialog"]'), secondary), 'secondary branch adoption');
  assert(await browser.evaluate(id => new URLSearchParams(location.search).get('worktree_id') === id, secondary), 'Selection absent from URL');
  const previousDocument = await browser.evaluate(() => performance.timeOrigin);
  await browser.rpc('Page.reload');
  await wait(() => browser.evaluate((id, previous) => performance.timeOrigin !== previous
    && window.reqvireProjectStore?.project.worktree_id === id
    && Boolean(document.querySelector('[data-product-pattern="app-shell"]')), secondary, previousDocument), 'reloaded selection');
  await browser.evaluate(() => history.back());
  await wait(() => browser.evaluate(id => window.reqvireProjectStore?.project.worktree_id === id, original), 'back to original branch');
  assert(await other(id => window.reqvireProjectStore.project.worktree_id === id, secondary), 'History affected the second tab');
} catch (error) {
  failure = new Error(`Worktree browser check failed during ${stage}`, { cause: error });
  try {
    console.error('Browser state:', await browser?.evaluate(() => ({
      url: location.href,
      context: window.reqvireProjectStore?.project.worktree_id,
      text: document.body.innerText.slice(0, 4000),
    })));
  } catch { /* Keep the original failure when the debugging connection is gone. */ }
} finally {
  const errors = failure ? [failure] : [];
  try { await browser?.close(); } catch (error) { errors.push(error); }
  try { await rm(profile, { recursive: true, force: true, maxRetries: 10, retryDelay: 100 }); }
  catch (error) { errors.push(error); }
  if (errors.length === 1) throw errors[0];
  if (errors.length > 1) throw new AggregateError(errors, 'Worktree browser check and cleanup failed');
}
console.log('PASS serve/browser-worktree-selection');
