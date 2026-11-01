import { type HTMLAttributes, type ComponentChildren } from "preact";
import { css, cx } from "@linaria/atomic";

export interface DiagramBlockFrameProps extends Omit<(HTMLAttributes<HTMLDivElement> & { className?: string }), "style"> {
  children?: ComponentChildren;
}

const diagramBlockUX = css`
  margin: var(--space-5) 0;
  overflow: auto;
  padding: var(--space-5);

  svg {
    display: block;
    width: 100%;
    min-width: var(--space-markdown-diagram-measure);
  }
`;

const diagramBlockSkinX = css`
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--bg-surface);
`;

export function DiagramBlockFrame({
  children,
  className = "",
  ...props
}: DiagramBlockFrameProps) {
  return (
    // A wide diagram scrolls inside the frame, so the frame is a focusable, labelled region.
    <div
      className={cx("ux-diagram-block-frame", diagramBlockUX, diagramBlockSkinX, className)}
      data-product-pattern="diagram-block-frame"
      role="region"
      aria-label="Diagram"
      tabIndex={0}
      {...props}
    >
      {children}
    </div>
  );
}
