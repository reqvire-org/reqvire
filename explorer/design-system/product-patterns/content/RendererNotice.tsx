import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

export type RendererNoticeTone = "notice" | "empty";

type RendererNoticeBaseProps = {
  title?: ReactNode;
  tone?: RendererNoticeTone;
  children?: ReactNode;
  className?: string;
};

export type RendererNoticeProps =
  | (RendererNoticeBaseProps & { inline?: false } & Omit<HTMLAttributes<HTMLDivElement>, "title">)
  | (RendererNoticeBaseProps & { inline: true } & Omit<HTMLAttributes<HTMLSpanElement>, "title">);

const blockUX = css`
  font-size: var(--text-sm);
  line-height: 1.45;

  strong {
    display: block;
    margin-bottom: var(--space-3);
    color: var(--text-body);
    font-weight: var(--weight-semibold);
  }

  pre {
    max-width: 100%;
    overflow-x: auto;
    margin: var(--space-3) 0 0;
    border-radius: var(--radius-md);
    padding: var(--space-5);
  }

  code {
    font-family: var(--font-mono);
    font-size: var(--text-caption);
    line-height: 1.55;
  }
`;

const blockSkinX = css`
  color: var(--text-muted);

  &[data-tone="empty"] {
    font-style: italic;
  }

  pre {
    background: var(--bg-sunken);
    color: var(--text-body);
  }
`;

const inlineUX = css`
  font-size: var(--text-sm);
  line-height: 1.45;
`;

const inlineSkinX = css`
  color: var(--text-muted);

  &[data-tone="empty"] {
    font-style: italic;
  }
`;

export function RendererNotice({
  inline = false,
  title,
  tone = "notice",
  children,
  className = "",
  ...props
}: RendererNoticeProps) {
  if (inline) {
    return (
      <span
        className={cx("ux-renderer-notice", inlineUX, inlineSkinX, className)}
        data-product-pattern="renderer-notice"
        data-tone={tone}
        {...(props as HTMLAttributes<HTMLSpanElement>)}
      >
        {title ? <strong>{title}</strong> : null}
        {children}
      </span>
    );
  }

  return (
    <div
      className={cx("ux-renderer-notice", blockUX, blockSkinX, className)}
      data-product-pattern="renderer-notice"
      data-tone={tone}
      {...(props as HTMLAttributes<HTMLDivElement>)}
    >
      {title ? <strong>{title}</strong> : null}
      {children}
    </div>
  );
}
