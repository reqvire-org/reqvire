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
    expect(document.activeElement).toBe(screen.getByRole("alert"));
    fireEvent.keyDown(dialog, { key: "Tab" });
    expect(document.activeElement).toBe(screen.getByRole("button", { name: "Close" }));
    if (action === "outside") fireEvent.mouseDown(dialog.parentElement!);
    if (action === "escape") fireEvent.keyDown(dialog, { key: "Escape" });
    if (action === "close") fireEvent.click(screen.getByRole("button", { name: "Close" }));
    expect(onDismiss).toHaveBeenCalledOnce();
  });
  it("preserves complete diagnostics, labels branch context and traps forward/reverse focus", () => {
    const error = `Validation failed with 2 errors:\n${"system-model/Capabilities.md#dependency −[contract consumer]→ ".repeat(40)}`;
    const branch = "feature/long-branch-name";
    render(<WorktreeLoadDialog branch={branch} error={error} onDismiss={() => {}} />);
    const dialog = screen.getByRole("dialog", { name: "Couldn’t load worktree" });
    const context = screen.getByText("Branch").parentElement!;
    expect(context.textContent).toBe(`Branch${branch}`);
    expect(dialog.getAttribute("aria-describedby")?.split(" ")[0]).toBe(context.id);
    const details = screen.getByRole("alert", { name: "Error details" });
    expect(details.textContent).toBe(error);
    expect(screen.getByText(/Your current model is still displayed/)).toBeTruthy();
    const close = screen.getByRole("button", { name: "Close" });
    fireEvent.keyDown(dialog, { key: "Tab", shiftKey: true });
    expect(document.activeElement).toBe(close);
    fireEvent.keyDown(close, { key: "Tab" });
    expect(document.activeElement).toBe(details);
    fireEvent.keyDown(details, { key: "Tab", shiftKey: true });
    expect(document.activeElement).toBe(close);
  });
});
