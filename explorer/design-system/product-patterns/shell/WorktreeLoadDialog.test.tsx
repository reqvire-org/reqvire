import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { WorktreeLoadDialog } from "./WorktreeLoadDialog";

afterEach(cleanup);
describe("WorktreeLoadDialog", () => {
  it("shares dismissible error presentation for refresh failures and explains automatic retry", () => {
    const onDismiss = vi.fn();
    render(<WorktreeLoadDialog branch="main" operation="refresh" error="Connection lost"
      retryAutomatically onDismiss={onDismiss} />);
    const dialog = screen.getByRole("dialog", { name: "Couldn’t refresh model" });
    expect(screen.getByRole("alert").textContent).toBe("Connection lost");
    expect(screen.getByText("Showing the last valid model. Refresh will retry automatically.")).toBeTruthy();
    fireEvent.keyDown(dialog, { key: "Escape" });
    expect(onDismiss).toHaveBeenCalledOnce();
  });

  it("focuses a centered loading modal and prevents pending dismissal", () => {
    const onDismiss = vi.fn();
    const trigger = document.createElement("button"); document.body.append(trigger); trigger.focus();
    const view = render(<WorktreeLoadDialog branch="feature/model" onDismiss={onDismiss} />);
    const dialog = screen.getByRole("dialog", { name: "Loading worktree" });
    expect(dialog.getAttribute("aria-busy")).toBe("true");
    expect(document.activeElement).toBe(dialog);
    expect(screen.getByRole("status", { name: "Loading feature/model" })).toBeTruthy();
    fireEvent.mouseDown(dialog.parentElement!); fireEvent.keyDown(dialog, { key: "Escape" });
    expect(onDismiss).not.toHaveBeenCalled();
    view.unmount(); expect(document.activeElement).toBe(trigger); trigger.remove();
  });
  it.each(["outside", "escape", "close"])("shows the error and allows %s dismissal", action => {
    const onDismiss = vi.fn();
    render(<WorktreeLoadDialog branch="feature" error="Invalid model in feature" onDismiss={onDismiss} />);
    const dialog = screen.getByRole("dialog", { name: "Couldn’t load worktree" });
    expect(screen.getByRole("alert").textContent).toBe("Invalid model in feature");
    expect(screen.queryByRole("status")).toBeNull();
    fireEvent.keyDown(dialog, { key: "Tab" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Close" }));
    if (action === "outside") fireEvent.mouseDown(dialog.parentElement!);
    if (action === "escape") fireEvent.keyDown(dialog, { key: "Escape" });
    if (action === "close") fireEvent.click(screen.getByRole("button", { name: "Close" }));
    expect(onDismiss).toHaveBeenCalledOnce();
  });
});
