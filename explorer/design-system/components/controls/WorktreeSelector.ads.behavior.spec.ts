import { expect, test } from "@playwright/test";
import { openBehaviorScenario } from "@reqvire-org/ads/verification";
import { fixture, SHOWCASE_WORKTREES } from "./WorktreeSelector.ads.fixture";

const available = SHOWCASE_WORKTREES.filter(choice => choice.available);
const unavailable = SHOWCASE_WORKTREES.find(choice => !choice.available)!;

test("keyboard selects an available branch", { tag: "@ads:worktree-keyboard" }, async ({ page }) => {
  const frame = await openBehaviorScenario(page, "worktree-keyboard");
  const trigger = frame.getByRole("combobox", { name: "Branch", exact: true });
  await expect(trigger).toContainText(available[0].branch);
  await trigger.focus();
  await trigger.press("ArrowDown");
  await trigger.press("End");
  const target = frame.getByRole("option").filter({ hasText: available.at(-1)!.branch });
  await expect(trigger).toHaveAttribute("aria-activedescendant", (await target.getAttribute("id"))!);
  await trigger.press("Enter");
  await expect(trigger).toContainText(available.at(-1)!.branch);
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(trigger).toBeFocused();
  await expect(frame.getByRole("listbox", { name: "Branches", exact: true })).toHaveCount(0);
  await trigger.press("ArrowUp");
  await trigger.press("Home");
  await trigger.press("Enter");
  await expect(trigger).toContainText(available[0].branch);
});

test("pointer selection respects unavailable branches and dismissal", { tag: "@ads:worktree-pointer" }, async ({ page }) => {
  const frame = await openBehaviorScenario(page, "worktree-pointer");
  const trigger = frame.getByRole("combobox", { name: "Branch", exact: true });
  await trigger.click();
  const blocked = frame.getByRole("option").filter({ hasText: unavailable.branch });
  await expect(blocked).toHaveAttribute("aria-disabled", "true");
  await blocked.click({ force: true });
  await expect(trigger).toContainText(available[0].branch);
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await frame.getByRole("option").filter({ hasText: available[1].branch }).click();
  await expect(trigger).toContainText(available[1].branch);
  await expect(trigger).toBeFocused();
  await trigger.click();
  await trigger.press("Escape");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await trigger.click();
  await frame.getByText(fixture.title, { exact: true }).click();
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
});
