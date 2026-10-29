import { afterEach } from "vitest";
import { act, cleanup as baseCleanup, render as baseRender, renderHook as baseRenderHook } from "@testing-library/preact/pure";
import type { RenderOptions, RenderHookOptions } from "@testing-library/preact";
import type { ComponentChild } from "preact";

export * from "@testing-library/preact/pure";

// Preact 11 defers passive unmount cleanup until after paint. The current
// testing-library unmount helpers do not wrap that phase in act themselves.
// Flush it here so assertions and the next test see completed resource cleanup.
export function cleanup() { act(() => { baseCleanup(); }); }
export function render(ui: ComponentChild, options?: RenderOptions) {
  const view = baseRender(ui, options);
  return { ...view, unmount: () => { let result = false; act(() => { result = view.unmount(); }); return result; } };
}
export function renderHook<Result, Props>(callback: (props: Props) => Result, options?: RenderHookOptions<Props>) {
  const view = baseRenderHook(callback, options);
  return { ...view, unmount: () => { act(() => { view.unmount(); }); } };
}
afterEach(cleanup);
