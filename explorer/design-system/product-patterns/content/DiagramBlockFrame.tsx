import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

export interface DiagramBlockFrameProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  children?: ReactNode;
}

const diagramBlockUX = css`
  --ux-markdown-diagram-min-w: 520px;
  margin: var(--space-5) 0;
  overflow: auto;
  padding: var(--space-5);

  svg {
    display: block;
    width: 100%;
    min-width: var(--ux-markdown-diagram-min-w);
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
    <div
      className={cx("ux-diagram-block-frame", diagramBlockUX, diagramBlockSkinX, className)}
      data-product-pattern="diagram-block-frame"
      {...props}
    >
      {children}
    </div>
  );
}
