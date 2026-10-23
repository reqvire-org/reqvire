import { expect, test } from "@playwright/test";
import { openBehaviorScenario } from "@reqvire-org/ads/verification";

test("custom concept canvases retain node geometry, pointer targeting, pan and zoom", { tag: "@ads:thesaurus-flow-interaction" }, async ({ page }) => {
  const frame = await openBehaviorScenario(page, "thesaurus-flow-interaction");
  const flow = frame.locator(".react-flow");
  const nodes = flow.locator(".react-flow__node-concept");
  await expect.poll(() => nodes.count()).toBeGreaterThan(1);
  const node = nodes.first();
  await expect(node).toBeVisible();
  await expect(node).toHaveCSS("position", "absolute");
  await expect(node).toHaveCSS("pointer-events", "all");
  const start = await node.boundingBox();
  expect(start!.width).toBeGreaterThan(0);
  expect(start!.height).toBeGreaterThan(0);
  expect(await node.evaluate(element => {
    const bounds = element.getBoundingClientRect();
    const hit = element.ownerDocument.elementFromPoint(bounds.x + bounds.width / 2, bounds.y + bounds.height / 2);
    return hit !== null && element.contains(hit);
  })).toBe(true);
  const viewport = flow.locator(".react-flow__viewport");
  const transform = () => viewport.evaluate(element => element.style.transform);
  const beforePan = await transform();
  const bounds = await flow.boundingBox();
  await page.mouse.move(bounds!.x + 12, bounds!.y + 12);
  await page.mouse.down();
  await page.mouse.move(bounds!.x + 65, bounds!.y + 40, { steps: 8 });
  await page.mouse.up();
  await expect.poll(transform).not.toBe(beforePan);
  const beforeZoom = await transform();
  await page.mouse.wheel(0, -200);
  await expect.poll(transform).not.toBe(beforeZoom);
});
