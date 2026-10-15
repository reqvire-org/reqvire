import { type ComponentChildren } from "preact";
import { RouteFrame } from "@ds";

/*
 * Native view modules fill the available viewport behind the persistent
 * Explorer side pane and tool rail.
 * A route change swaps the active view module entirely (no stale content,
 * no iframe-mounted standalone page content).
 */
export function ViewFrame({
  testId,
  children,
}: {
  testId: string;
  children: ComponentChildren;
}) {
  return <RouteFrame viewId={testId}>{children}</RouteFrame>;
}
