import type { ReactNode } from "react";

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
  children: ReactNode;
}) {
  return (
    <main
      data-view={testId}
      className="absolute inset-0 overflow-auto bg-reqvire-background"
    >
      {children}
    </main>
  );
}
