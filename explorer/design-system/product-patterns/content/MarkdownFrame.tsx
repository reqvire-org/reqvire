import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";

export type MarkdownFrameVariant = "detail" | "preview";

export const MARKDOWN_TABLE_WRAP_CLASS = "ux-markdown-table-wrap";

export interface MarkdownFrameProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  variant?: MarkdownFrameVariant;
  children?: ReactNode;
}

const markdownBaseUX = css`
  --ux-markdown-list-marker-col: var(--space-8);
  --ux-markdown-list-marker-size: var(--space-2);
  font-size: var(--text-md);
  line-height: 1.65;

  > :first-child {
    margin-top: 0;
  }

  > :last-child {
    margin-bottom: 0;
  }

  h1,
  h2,
  h3,
  h4 {
    margin: 0.9em 0 0.35em;
    font-weight: var(--weight-semibold);
    line-height: 1.25;
  }

  h1 {
    font-size: var(--text-2xl);
  }

  h2 {
    font-size: var(--text-xl);
  }

  h3 {
    font-size: var(--text-lg);
  }

  h4 {
    font-size: var(--text-base);
  }

  p,
  ul,
  ol,
  blockquote,
  pre,
  .mermaid,
  .ux-markdown-table-wrap {
    margin: 0.55em 0;
  }

  .mermaid {
    position: relative;
    min-width: 0;
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .mermaid + h1,
  .mermaid + h2,
  .mermaid + h3,
  .mermaid + h4 {
    margin-top: var(--space-8);
  }

  .diagram-nav-buttons {
    position: absolute;
    left: var(--space-4);
    bottom: var(--space-4);
    display: inline-flex;
    flex-wrap: wrap;
    gap: var(--space-1);
    align-items: center;
    padding: var(--space-1);
    border-radius: var(--radius-md);
  }

  .diagram-nav-btn {
    display: inline-flex;
    min-width: var(--control-sm);
    min-height: var(--control-sm);
    align-items: center;
    justify-content: center;
    border: var(--border-w) solid transparent;
    border-radius: var(--radius-xs);
    padding: 0 var(--space-3);
    font: inherit;
    font-size: var(--text-sm);
    font-weight: var(--weight-medium);
    line-height: var(--leading-tight);
    cursor: pointer;
  }

  ul,
  ol {
    padding-left: 0;
    list-style: none;
  }

  li {
    position: relative;
    margin: var(--space-1) 0;
    padding-left: var(--ux-markdown-list-marker-col);
  }

  li > p {
    margin: var(--space-1) 0;
  }

  li > ul,
  li > ol {
    margin: var(--space-2) 0 0;
  }

  ul > li::before {
    content: "";
    position: absolute;
    left: var(--space-2);
    top: 0.72em;
    width: var(--ux-markdown-list-marker-size);
    height: var(--ux-markdown-list-marker-size);
    border-radius: var(--radius-pill);
  }

  ol {
    counter-reset: ux-markdown-list-item;
  }

  ol > li {
    counter-increment: ux-markdown-list-item;
  }

  ol > li::before {
    content: counter(ux-markdown-list-item) ".";
    position: absolute;
    left: 0;
    top: 0;
    min-width: var(--space-6);
    text-align: right;
    font-size: var(--text-sm);
    font-weight: var(--weight-medium);
    line-height: inherit;
  }

  a {
    text-decoration: underline;
    text-underline-offset: var(--space-1);
  }

  img {
    display: block;
    max-width: 100%;
    height: auto;
    margin: var(--space-5) 0;
  }

  code {
    border-radius: var(--radius-md);
    padding: var(--space-1) var(--space-2);
    font-size: 0.92em;
  }

  pre {
    overflow-x: auto;
    border-radius: var(--radius-md);
    padding: var(--space-5);
  }

  pre code {
    padding: 0;
  }

  blockquote {
    padding-left: var(--space-5);
  }

  .ux-markdown-table-wrap {
    overflow-x: auto;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.95em;
  }

  th,
  td {
    padding: var(--space-3) var(--space-4);
    text-align: left;
    vertical-align: top;
  }
`;

const markdownSkinX = css`
  color: var(--text-body);

  h1,
  h2,
  h3,
  h4 {
    color: var(--text-body);
  }

  a {
    color: var(--text-link);
  }

  a:hover {
    color: var(--accent-hover);
  }

  img {
    border: var(--border-w) solid var(--border-subtle);
  }

  code,
  pre {
    background: var(--bg-sunken);
  }

  .mermaid {
    background: var(--bg-surface);
  }

  .diagram-nav-buttons {
    border: var(--border-w) solid var(--border-subtle);
    background: var(--bg-surface);
    box-shadow: var(--shadow-sm);
  }

  .diagram-nav-btn {
    border-color: var(--border-subtle);
    background: var(--bg-overlay);
    color: var(--text-body);
  }

  .diagram-nav-btn:hover {
    border-color: var(--accent);
    background: var(--bg-hover);
    color: var(--text-strong);
  }

  .diagram-nav-btn:focus-visible {
    outline: var(--border-w) solid var(--text-link);
    outline-offset: var(--space-1);
  }

  pre code {
    background: transparent;
  }

  blockquote {
    border-left: var(--border-w-heavy) solid var(--border-subtle);
    color: var(--text-muted);
  }

  ul > li::before {
    background: var(--text-muted);
  }

  ol > li::before {
    color: var(--text-muted);
  }

  th,
  td {
    border: var(--border-w) solid var(--border-subtle);
  }

  th {
    background: var(--bg-sunken);
    font-weight: var(--weight-semibold);
  }
`;

const markdownPreviewUX = css`
  margin-top: var(--space-2);
  display: -webkit-box;
  overflow: hidden;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;

  p,
  ul,
  ol {
    margin: 0;
  }
`;

const markdownPreviewSkinX = css`
  color: var(--text-muted);
`;

export function MarkdownFrame({
  variant = "detail",
  children,
  className = "",
  ...props
}: MarkdownFrameProps) {
  return (
    <div
      className={cx(
        "ux-markdown",
        `ux-markdown-${variant}`,
        markdownBaseUX,
        markdownSkinX,
        variant === "preview" && markdownPreviewUX,
        variant === "preview" && markdownPreviewSkinX,
        className,
      )}
      data-product-pattern="markdown-frame"
      data-variant={variant}
      {...props}
    >
      {children}
    </div>
  );
}
