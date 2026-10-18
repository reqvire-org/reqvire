import { expect, test } from "@playwright/test";
import { openBehaviorScenario } from "@reqvire-org/ads/verification";

test("actions and persistent close controls dismiss notifications", { tag: "@ads:toast-actions" }, async ({ page }) => {
  const frame = await openBehaviorScenario(page, "toast-actions");
  const stack = frame.getByRole("list", { name: "Notifications (Alt+T)", exact: true });
  await frame.getByRole("button", { name: "Undo action", exact: true }).click();
  await expect(stack.getByRole("group")).toHaveCount(1);
  await stack.getByRole("button", { name: "Undo", exact: true }).click();
  await expect(frame.getByRole("status").filter({ hasText: "Archive undone." })).toBeVisible();
  await expect(stack.locator("li")).toHaveCount(0);
  await frame.getByRole("button", { name: "Persistent toast", exact: true }).click();
  await expect(stack).toContainText("This notification stays until you dismiss it.");
  await stack.getByRole("button", { name: "Dismiss notification", exact: true }).click();
  await expect(stack.locator("li")).toHaveCount(0);
});

test("promise updates a card and keyboard focus returns", { tag: "@ads:toast-promise-focus" }, async ({ page }) => {
  const frame = await openBehaviorScenario(page, "toast-promise-focus");
  const stack = frame.getByRole("list", { name: "Notifications (Alt+T)", exact: true });
  const start = frame.getByRole("button", { name: "Loading → success", exact: true });
  await start.click();
  await expect(stack.getByRole("group")).toHaveAttribute("data-variant", "loading");
  const card = await stack.getByRole("group").elementHandle();
  await page.keyboard.press("Alt+KeyT");
  await expect(stack).toBeFocused();
  await expect(stack).toHaveAttribute("data-expanded", "true");
  await expect(stack.getByRole("group")).toHaveAttribute("data-variant", "success");
  await expect(stack.getByRole("group")).toHaveCount(1);
  expect(await card!.evaluate(element => element.isConnected)).toBe(true);
  await expect(stack).toContainText("Your changes are now up to date.");
  await page.keyboard.press("Escape");
  await expect(start).toBeFocused();
});
