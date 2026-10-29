import { act, fireEvent, render, renderHook, screen } from "@/test/render";
import { afterEach, expect, it, vi } from "vitest";
import { useExplorerPane } from "@ds";

afterEach(() => { vi.restoreAllMocks(); });

it("keeps pane geometry usable when the host denies browser storage", () => {
  vi.spyOn(window, "localStorage", "get").mockImplementation(() => { throw new DOMException("Opaque origin", "SecurityError"); });
  const hook = renderHook(() => useExplorerPane());
  expect(hook.result.current.leftPaneWidth).toBe(380);
  act(() => hook.result.current.toggleLeftPane());
  expect(hook.result.current.leftPaneOpen).toBe(false);
  act(() => hook.result.current.toggleLeftPane());
  expect(hook.result.current.leftPaneOpen).toBe(true);
  hook.unmount();
});

it("releases drag listeners and document styles when a preview is unmounted mid-drag", () => {
  const removed = vi.spyOn(window, "removeEventListener");
  function Harness() {
    const pane = useExplorerPane();
    return <div ref={pane.shellRef}><div role="separator" onPointerDown={pane.handleLeftPaneResizePointerDown} /></div>;
  }
  const view = render(<Harness />);
  fireEvent(screen.getByRole("separator"), new MouseEvent("pointerdown", { bubbles: true, button: 0, clientX: 100 }));
  expect(document.body.style.cursor).toBe("ew-resize");
  view.unmount();
  expect(document.body.style.cursor).toBe("");
  expect(document.body.style.userSelect).toBe("");
  expect(removed.mock.calls.map(([type]) => type)).toEqual(expect.arrayContaining(["pointermove", "pointerup", "pointercancel"]));
});
