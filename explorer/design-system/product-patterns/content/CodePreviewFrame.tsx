import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { Button } from "../../components/core/Button";
import { Icon } from "../../components/core/Icon";

const toolbarClass = "ux-source-code-preview__toolbar";
const titleClass = "ux-source-code-preview__title";
const kindClass = "ux-source-code-preview__kind";
const actionsClass = "ux-source-code-preview__actions";
const buttonClass = "ux-source-code-preview__button";
const bodyClass = "ux-source-code-preview__body";
const collapsedClass = "ux-source-code-preview__collapsed";

export const CODE_PREVIEW_CODE_CLASS = "ux-source-code-preview__code";
export const CODE_PREVIEW_FALLBACK_CLASS = "ux-source-code-preview__fallback";

export interface CodePreviewFrameProps extends Omit<HTMLAttributes<HTMLElement>, "style"> {
  wrapped?: boolean;
  children?: ReactNode;
}

export interface CodeToolbarProps extends Omit<HTMLAttributes<HTMLDivElement>, "onCopy" | "style"> {
  expanded: boolean;
  onToggleExpanded: () => void;
  kind: ReactNode;
  path?: ReactNode;
  showPath?: boolean;
  languageLabel: ReactNode;
  lineCount: number;
  relationTypes?: readonly string[];
  wrapped: boolean;
  copied: boolean;
  onToggleWrap: () => void;
  onCopy: () => void | Promise<void>;
}

export interface CodeBodyProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  collapsed?: boolean;
  lineCount?: number;
  children?: ReactNode;
}

const baseUX = css`
  display: grid;
  border-radius: var(--radius-lg);

  .ux-source-code-preview__toolbar {
    position: sticky;
    top: 0;
    z-index: var(--z-local-raised);
    display: flex;
    min-width: 0;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-5);
    border-radius: var(--radius-lg) var(--radius-lg) 0 0;
    padding: var(--space-4) var(--space-5);
  }

  .ux-source-code-preview__title {
    display: flex;
    min-width: 0;
    flex: 1 1 auto;
    align-items: center;
    gap: var(--space-4);
    border: 0;
    padding: 0;
    cursor: pointer;
    font-size: var(--text-caption);
    text-align: left;
  }

  .ux-source-code-preview__title svg,
  .ux-source-code-preview__button svg {
    width: var(--icon-sm);
    height: var(--icon-sm);
    flex: 0 0 auto;
  }

  .ux-source-code-preview__title strong {
    min-width: 0;
    overflow: hidden;
    font-family: var(--font-mono);
    font-size: var(--text-caption);
    font-weight: var(--weight-semibold);
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ux-source-code-preview__title span {
    flex: 0 0 auto;
  }

  .ux-source-code-preview__kind {
    border-radius: var(--radius-pill);
    padding: var(--space-1) var(--space-3);
    font-weight: var(--weight-bold);
  }

  .ux-source-code-preview__actions {
    display: inline-flex;
    flex: 0 0 auto;
    align-items: center;
    gap: var(--space-3);
  }

  .ux-source-code-preview__button {
    display: inline-flex;
    height: var(--row-height-compact);
    align-items: center;
    gap: var(--space-3);
    border-radius: var(--radius-md);
    padding: 0 var(--space-4);
    cursor: pointer;
    font-size: var(--text-caption);
    font-weight: var(--weight-bold);
  }

  .ux-source-code-preview__body {
    min-width: 0;
    overflow: visible;
  }

  .ux-source-code-preview__code {
    max-width: 100%;
    overflow-x: auto;
    overflow-y: visible;
  }

  .ux-source-code-preview__code .shiki,
  .ux-source-code-preview__fallback {
    margin: 0;
    padding: var(--space-8);
    font-family: var(--font-mono);
    font-size: var(--text-caption);
    line-height: 1.55;
    tab-size: 2;
  }

  .ux-source-code-preview__code .shiki {
    min-width: max-content;
  }

  .ux-source-code-preview__fallback {
    max-width: 100%;
    overflow-x: auto;
    overflow-y: visible;
  }

  .ux-source-code-preview__code code,
  .ux-source-code-preview__fallback code {
    font-family: inherit;
  }

  .ux-source-code-preview__collapsed {
    padding: var(--space-5);
    font-size: var(--text-caption);
  }
`;

const skinX = css`
  border: var(--border-w) solid var(--border-subtle);
  background: var(--bg-surface);
  box-shadow: var(--shadow-xs);

  .ux-source-code-preview__toolbar {
    border-bottom: var(--border-w) solid var(--border-subtle);
    background: var(--bg-surface);
  }

  .ux-source-code-preview__title {
    background: transparent;
    color: var(--text-muted);
  }

  .ux-source-code-preview__title strong {
    color: var(--text-body);
  }

  .ux-source-code-preview__kind {
    background: var(--bg-sunken);
    color: var(--text-muted);
  }

  .ux-source-code-preview__button {
    border: var(--border-w) solid var(--border-subtle);
    background: var(--bg-canvas);
    color: var(--text-muted);
  }

  .ux-source-code-preview__button:hover {
    border-color: var(--border-strong);
    background: var(--bg-hover);
    color: var(--text-body);
  }

  .ux-source-code-preview__button[aria-pressed="true"] {
    border-color: var(--border-selected);
    background: var(--bg-selected);
    color: var(--text-body);
  }

  .ux-source-code-preview__body {
    background: var(--bg-sunken);
  }

  .ux-source-code-preview__code .shiki,
  .ux-source-code-preview__fallback {
    background: transparent !important;
    color: var(--text-body);
  }

  .ux-source-code-preview__collapsed {
    color: var(--text-muted);
  }

  .element-detail-dialog & .ux-source-code-preview__body {
    border: var(--border-w) solid var(--border-subtle);
    background: var(--bg-sunken);
  }
`;

const wrappedUX = css`
  .ux-source-code-preview__code .shiki,
  .ux-source-code-preview__fallback {
    min-width: 0;
    overflow-wrap: anywhere;
    white-space: pre-wrap;
  }
`;

export function CodePreviewFrame({
  wrapped = false,
  children,
  className = "",
  ...props
}: CodePreviewFrameProps) {
  return (
    <section
      className={cx("ux-source-code-preview", baseUX, skinX, wrapped ? wrappedUX : "", className)}
      data-product-pattern="code-preview-frame"
      {...props}
    >
      {children}
    </section>
  );
}

export function CodeToolbar({
  expanded,
  onToggleExpanded,
  kind,
  path,
  showPath = true,
  languageLabel,
  lineCount,
  relationTypes = [],
  wrapped,
  copied,
  onToggleWrap,
  onCopy,
  className = "",
  ...props
}: CodeToolbarProps) {
  return (
    <div className={cx(toolbarClass, className)} data-product-pattern-slot="toolbar" {...props}>
      <button
        type="button"
        className={titleClass}
        aria-expanded={expanded}
        onClick={onToggleExpanded}
      >
        {expanded ? <Icon name="chevron-down" /> : <Icon name="chevron-right" />}
        <span className={kindClass}>{kind}</span>
        {showPath ? <strong>{path || "source"}</strong> : null}
        <span>{languageLabel}</span>
        <span>{lineCount} lines</span>
        {relationTypes.length > 0 ? <span>via {relationTypes.join(", ")}</span> : null}
      </button>
      <div className={actionsClass}>
        <Button
          tone="secondary"
          size="sm"
          className={buttonClass}
          aria-label={wrapped ? "Disable line wrap" : "Enable line wrap"}
          aria-pressed={wrapped}
          onClick={onToggleWrap}
          title={wrapped ? "Disable line wrap" : "Enable line wrap"}
        >
          <Icon name="wrap-text" />
        </Button>
        <Button
          tone="secondary"
          size="sm"
          className={buttonClass}
          aria-label={copied ? "Copied" : "Copy source"}
          onClick={onCopy}
          title={copied ? "Copied!" : "Copy source"}
        >
          {copied ? <Icon name="check" /> : <Icon name="copy" />}
        </Button>
      </div>
    </div>
  );
}

export function CodeBody({
  collapsed = false,
  lineCount = 0,
  children,
  className = "",
  ...props
}: CodeBodyProps) {
  if (collapsed) {
    return (
      <div className={cx(collapsedClass, className)} data-product-pattern-slot="collapsed" {...props}>
        {children ?? `${lineCount} lines hidden`}
      </div>
    );
  }

  return (
    <div className={cx(bodyClass, className)} data-product-pattern-slot="body" {...props}>
      {children}
    </div>
  );
}
