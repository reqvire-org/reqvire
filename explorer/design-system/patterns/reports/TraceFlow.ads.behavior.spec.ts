import { expect, test, type Frame } from "@playwright/test";
import { openBehaviorScenario } from "@reqvire-org/ads/verification";
import { fixture } from "./TraceFlow.ads.fixture";

async function ready(frame: Frame) {
  const flow = frame.getByRole("region", { name: "Verification trace flow", exact: true });
  await expect(flow).toHaveAttribute("aria-busy", "false");
  await expect(flow.locator(".react-flow__node")).toHaveCount(fixture.trace.requirements.length + 1);
  return flow;
}

test("real CSS dimensions and worker direction agree", { tag: "@ads:trace-flow-layout" }, async ({ page }) => {
  const frame = await openBehaviorScenario(page, "trace-flow-layout");
  const flow = await ready(frame);
  const dimensions = await flow.evaluate(element => {
    const probe = element.querySelector<HTMLElement>(".ux-trace-flow-node-size")!;
    return {
      width: probe.offsetWidth,
      height: probe.offsetHeight,
      nodes: [...element.querySelectorAll<HTMLElement>(".react-flow__node")].map(node => ({ width: node.offsetWidth, height: node.offsetHeight })),
    };
  });
  expect(dimensions.width).toBeGreaterThan(0);
  expect(dimensions.height).toBeGreaterThan(0);
  for (const node of dimensions.nodes) expect(node).toEqual({ width: dimensions.width, height: dimensions.height });
  const verification = flow.locator(`.react-flow__node[data-id=${JSON.stringify(fixture.trace.verification.id)}]`);
  const requirement = flow.locator(`.react-flow__node[data-id=${JSON.stringify(fixture.trace.requirements[0].id)}]`);
  const followsAxis = async (axis: "x" | "y") => {
    const [source, target] = await Promise.all([verification.boundingBox(), requirement.boundingBox()]);
    return !!source && !!target && source[axis] < target[axis];
  };
  await flow.getByRole("button", { name: "Left to right", exact: true }).click();
  await expect.poll(() => followsAxis("x")).toBe(true);
  const positions = () => flow.locator(".react-flow__node").evaluateAll(nodes => nodes.map(node => getComputedStyle(node).transform));
  const before = await positions();
  await flow.getByRole("button", { name: "Top to bottom", exact: true }).click();
  await expect(flow.getByRole("button", { name: "Top to bottom", exact: true })).toHaveAttribute("aria-pressed", "true");
  await ready(frame);
  await expect.poll(positions).not.toEqual(before);
  await expect.poll(() => followsAxis("y")).toBe(true);
});

test("expanded flow preserves its mounted renderer", { tag: "@ads:trace-flow-expand" }, async ({ page }) => {
  const frame = await openBehaviorScenario(page, "trace-flow-expand");
  const flow = await ready(frame);
  const node = await flow.locator(".react-flow__node").first().elementHandle();
  await flow.getByRole("button", { name: "Expand flow", exact: true }).click();
  const dialog = frame.getByRole("dialog", { name: "Expanded verification trace flow", exact: true });
  await expect(dialog).toBeVisible();
  expect(await node!.evaluate(element => element.isConnected)).toBe(true);
  await page.keyboard.press("Escape");
  await expect(dialog).toHaveCount(0);
  await expect(flow.getByRole("button", { name: "Expand flow", exact: true })).toBeFocused();
  expect(await node!.evaluate(element => element.isConnected)).toBe(true);
  await ready(frame);
});
