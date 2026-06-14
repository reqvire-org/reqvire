import type { ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

const baseUX = css`
  position: absolute;
  inset: 0;
  overflow: hidden;

  &[data-view="traces"] {
    overflow: hidden;
  }

  &[data-view="traces"] .ex-route,
  &[data-view="traces"] .trace-main-panel {
    height: 100%;
    min-height: 0;
    overflow: hidden;
  }

  &[data-view="traces"] .trace-content-scroll {
    min-height: 0;
    overflow-x: hidden;
    overflow-y: auto;
  }

  &[data-view="coverage"] .coverage-dashboard {
    width: 100%;
    margin-right: 0;
  }
`;

const skinX = css`
  background: var(--bg-canvas);
`;

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
      className={cx(baseUX, skinX)}
    >
      {children}
    </main>
  );
}
