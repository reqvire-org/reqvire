import { withBrowser, waitFor } from '../browser.mjs';

const [base, original, secondary, profile, dirty] = process.argv.slice(2);
await withBrowser(profile, async browser => {
let stage = 'browser startup';
async function wait(check, description) {
  stage = description;
  await waitFor(check);
}
try {
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
    await browser.evaluate(() => document.querySelector('.ux-worktree-load-dialog').parentElement.dispatchEvent(new MouseEvent('mousedown', { bubbles: true })));
    await wait(() => browser.evaluate(id => new URLSearchParams(location.search).get('worktree_id') === id && !document.querySelector('.ux-worktree-load-dialog'), original), 'dismiss dirty-branch rejection');
  }
  assert(await browser.evaluate(() => {
    const navigation = document.querySelector('nav[aria-label="Explorer views"]');
    const selector = document.querySelector('.ux-worktree-selector [role="combobox"]');
    return Boolean(navigation && selector
      && navigation.querySelectorAll('[role="tab"]').length === 5
      && selector.closest('[data-product-pattern-slot="header-context"]')?.previousElementSibling?.getAttribute('data-product-pattern-slot') === 'brand'
      && selector.closest('[data-product-pattern="shell-header"]'));
  }), 'Worktree picker must follow the brand in the shared header and preserve Explorer navigation');
  await browser.evaluate(() => document.querySelector('button[aria-label="Collapse explorer"]').click());
  assert(await browser.evaluate(() => Boolean(document.querySelector('[data-product-pattern="shell-header"] [role="combobox"]'))), 'Collapsed pane hid the worktree picker');
  await browser.evaluate(() => document.querySelector('button[aria-label="Expand explorer"]').click());
  assert(!await browser.evaluate(() => window.reqvireProjectStore.elements.some(e => e.name === 'Other Subject')), 'Original tab contains secondary model');
  assert(await other(() => window.reqvireProjectStore.elements.some(e => e.name === 'Other Subject')), 'Secondary tab has wrong model');
  for (const view of ['model', 'ontologies', 'traces', 'coverage', 'search']) {
    await other(view => { location.hash = '#/' + view; }, view);
    await wait(() => other(view => location.hash.startsWith(view === 'coverage' ? '#/coverage?scope=' : '#/' + view)
      && Boolean(document.querySelector('main')), view), `second-tab ${view} view`);
    assert(await other(id => window.reqvireProjectStore.project.worktree_id === id, secondary), `${view} lost context`);
    assert(await browser.evaluate(id => window.reqvireProjectStore.project.worktree_id === id, original), `${view} changed the other tab`);
  }
  // Exercise failures over real accepted data, without replacing the server store.
  const retainedUrl = await browser.evaluate(() => location.href);
  await browser.evaluate(id => {
    const originalFetch = window.fetch;
    window.fetch = async (url, init) => {
      if (String(url) === '/api/worktrees/load' && JSON.parse(init.body).worktree_id === id) {
        if (window.simulateWorktreeError) return Response.json({ error: 'Injected worktree load failure' }, { status: 503 });
        if (window.holdWorktreeLoad) await new Promise(resolve => { window.releaseWorktreeLoad = resolve; });
      }
      return originalFetch(url, init);
    };
  }, secondary);
  for (const action of ['outside', 'escape', 'close']) {
    await browser.evaluate(() => { window.simulateWorktreeError = true; document.querySelector('.ux-worktree-selector [role="combobox"]').click(); });
    await wait(() => browser.evaluate(id => Boolean(document.querySelector(`[data-worktree-id="${id}"]`)), secondary), 'failure target choice');
    await browser.evaluate(id => document.querySelector(`[data-worktree-id="${id}"]`).click(), secondary);
    await wait(() => browser.evaluate(() => document.querySelector('.ux-worktree-load-dialog [role="alert"]')?.textContent === 'Injected worktree load failure'), 'worktree error dialog');
    assert(await browser.evaluate(id => window.reqvireProjectStore.project.worktree_id === id && document.querySelector('[data-product-pattern="app-shell"]').inert, original), 'Failure must retain and block the accepted model');
    if (action === 'escape') await browser.rpc('Input.dispatchKeyEvent', { type: 'keyDown', key: 'Escape', code: 'Escape' });
    else await browser.evaluate(action => action === 'outside'
      ? document.querySelector('.ux-worktree-load-dialog').parentElement.dispatchEvent(new MouseEvent('mousedown', { bubbles: true }))
      : [...document.querySelectorAll('.ux-worktree-load-dialog button')].find(button => button.textContent.trim() === 'Close').click(), action);
    await wait(() => browser.evaluate(url => location.href === url && !document.querySelector('.ux-worktree-load-dialog') && !document.querySelector('[data-product-pattern="app-shell"]').inert, retainedUrl), 'dismiss failed switch');
  }
  await browser.evaluate(() => { window.simulateWorktreeError = false; window.holdWorktreeLoad = true; });
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
  await wait(() => browser.evaluate(() => Boolean(document.querySelector('.ux-worktree-load-dialog [role="status"]'))), 'worktree loading spinner');
  assert(await browser.evaluate(id => window.reqvireProjectStore.project.worktree_id === id && document.querySelector('[data-product-pattern="app-shell"]').inert, original), 'Pending switch must retain and block the accepted model');
  assert(await browser.evaluate(() => {
    const overlay = getComputedStyle(document.querySelector('.ux-worktree-load-dialog').parentElement);
    return overlay.alignItems === 'center' && overlay.backdropFilter.includes('blur') && overlay.backgroundColor !== 'rgba(0, 0, 0, 0)';
  }), 'Loading dialog must use the centered blurred/darkened scrim');
  await browser.evaluate(() => document.querySelector('.ux-worktree-load-dialog').parentElement.dispatchEvent(new MouseEvent('mousedown', { bubbles: true })));
  await browser.rpc('Input.dispatchKeyEvent', { type: 'keyDown', key: 'Escape', code: 'Escape' });
  assert(await browser.evaluate(() => Boolean(document.querySelector('.ux-worktree-load-dialog [role="status"]'))), 'Pending loading dialog was dismissed');
  await browser.evaluate(() => { window.holdWorktreeLoad = false; window.releaseWorktreeLoad(); });
  await wait(() => browser.evaluate(id => window.reqvireProjectStore.project.worktree_id === id && !document.querySelector('[role="dialog"]'), secondary), 'secondary branch adoption');
  assert(await browser.evaluate(id => new URLSearchParams(location.search).get('worktree_id') === id, secondary), 'Selection absent from URL');
  const previousDocument = await browser.evaluate(() => performance.timeOrigin);
  await browser.reloadPage();
  await wait(() => browser.evaluate((id, previous) => performance.timeOrigin !== previous
    && window.reqvireProjectStore?.project.worktree_id === id
    && Boolean(document.querySelector('[data-product-pattern="app-shell"]')), secondary, previousDocument), 'reloaded selection');
  await browser.evaluate(() => history.back());
  await wait(() => browser.evaluate(id => window.reqvireProjectStore?.project.worktree_id === id, original), 'back to original branch');
  assert(await other(id => window.reqvireProjectStore.project.worktree_id === id, secondary), 'History affected the second tab');
} catch (error) {
  const failure = new Error(`Worktree browser check failed during ${stage}`, { cause: error });
  try {
    console.error('Browser state:', await browser?.evaluate(() => ({
      url: location.href,
      context: window.reqvireProjectStore?.project.worktree_id,
      text: document.body.innerText.slice(0, 4000),
    })));
  } catch { /* Keep the original failure when the debugging connection is gone. */ }
  throw failure;
}
});
console.log('PASS serve/browser-worktree-selection');
