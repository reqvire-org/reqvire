import { useEffect, useMemo, useState } from "react";
import { css, cx } from "@linaria/atomic";
import { codeToHtml } from "shiki";
import { Button, Icon } from "@ds";
import { SafeHtml } from "./SafeHtml";

const toolbarClass = "ex-source-code-preview__toolbar";
const titleClass = "ex-source-code-preview__title";
const kindClass = "ex-source-code-preview__kind";
const actionsClass = "ex-source-code-preview__actions";
const buttonClass = "ex-source-code-preview__button";
const bodyClass = "ex-source-code-preview__body";
const codeClass = "ex-source-code-preview__code";
const fallbackClass = "ex-source-code-preview__fallback";
const collapsedClass = "ex-source-code-preview__collapsed";

const baseUX = css`
  display: grid;
  border-radius: var(--radius-lg);

  .ex-source-code-preview__toolbar {
    position: sticky;
    top: 0;
    z-index: 2;
    display: flex;
    min-width: 0;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-5);
    border-radius: var(--radius-lg) var(--radius-lg) 0 0;
    padding: var(--space-4) var(--space-5);
  }

  .ex-source-code-preview__title {
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

  .ex-source-code-preview__title svg,
  .ex-source-code-preview__button svg {
    width: var(--icon-sm);
    height: var(--icon-sm);
    flex: 0 0 auto;
  }

  .ex-source-code-preview__title strong {
    min-width: 0;
    overflow: hidden;
    font-family: var(--font-mono);
    font-size: var(--text-caption);
    font-weight: var(--weight-semibold);
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ex-source-code-preview__title span {
    flex: 0 0 auto;
  }

  .ex-source-code-preview__kind {
    border-radius: var(--radius-pill);
    padding: var(--space-1) var(--space-3);
    font-weight: var(--weight-bold);
  }

  .ex-source-code-preview__actions {
    display: inline-flex;
    flex: 0 0 auto;
    align-items: center;
    gap: var(--space-3);
  }

  .ex-source-code-preview__button {
    display: inline-flex;
    height: var(--row-h);
    align-items: center;
    gap: var(--space-3);
    border-radius: var(--radius-md);
    padding: 0 var(--space-4);
    cursor: pointer;
    font-size: var(--text-caption);
    font-weight: var(--weight-bold);
  }

  .ex-source-code-preview__body {
    min-width: 0;
    overflow: visible;
  }

  .ex-source-code-preview__code {
    max-width: 100%;
    overflow-x: auto;
    overflow-y: visible;
  }

  .ex-source-code-preview__code .shiki,
  .ex-source-code-preview__fallback {
    margin: 0;
    padding: var(--space-8);
    font-family: var(--font-mono);
    font-size: var(--text-caption);
    line-height: 1.55;
    tab-size: 2;
  }

  .ex-source-code-preview__code .shiki {
    min-width: max-content;
  }

  .ex-source-code-preview__fallback {
    max-width: 100%;
    overflow-x: auto;
    overflow-y: visible;
  }

  .ex-source-code-preview__code code,
  .ex-source-code-preview__fallback code {
    font-family: inherit;
  }

  .ex-source-code-preview__collapsed {
    padding: var(--space-5);
    font-size: var(--text-caption);
  }
`;

const skinX = css`
  border: var(--border-w) solid var(--border-subtle);
  background: var(--bg-surface);
  box-shadow: var(--shadow-xs);

  .ex-source-code-preview__toolbar {
    border-bottom: var(--border-w) solid var(--border-subtle);
    background: var(--bg-surface);
  }

  .ex-source-code-preview__title {
    background: transparent;
    color: var(--text-muted);
  }

  .ex-source-code-preview__title strong {
    color: var(--text-body);
  }

  .ex-source-code-preview__kind {
    background: var(--bg-sunken);
    color: var(--text-muted);
  }

  .ex-source-code-preview__button {
    border: var(--border-w) solid var(--border-subtle);
    background: var(--bg-canvas);
    color: var(--text-muted);
  }

  .ex-source-code-preview__button:hover,
  .ex-source-code-preview__button[aria-pressed="true"] {
    border-color: color-mix(in srgb, var(--accent) 28%, transparent);
    background: color-mix(in srgb, var(--accent) 8%, transparent);
    color: var(--text-body);
  }

  .ex-source-code-preview__body {
    background: var(--bg-sunken);
  }

  .ex-source-code-preview__code .shiki,
  .ex-source-code-preview__fallback {
    background: transparent !important;
    color: var(--text-body);
  }

  .ex-source-code-preview__collapsed {
    color: var(--text-muted);
  }

  .element-detail-dialog & .ex-source-code-preview__body {
    border: var(--border-w) solid var(--border-subtle);
    background: var(--bg-sunken);
  }
`;

const wrappedUX = css`
  .ex-source-code-preview__code .shiki,
  .ex-source-code-preview__fallback {
    min-width: 0;
    overflow-wrap: anywhere;
    white-space: pre-wrap;
  }
`;

interface SourceCodePreviewProps {
  path: string;
  content: string;
  kind?: string;
  relationTypes?: string[];
  defaultExpanded?: boolean;
  showPath?: boolean;
}

type SourceKind =
  | "bash"
  | "css"
  | "html"
  | "json"
  | "markdown"
  | "python"
  | "rust"
  | "toml"
  | "typescript"
  | "yaml"
  | "text";

export function SourceCodePreview({
  path,
  content,
  kind = "source file",
  relationTypes = [],
  defaultExpanded = true,
  showPath = true,
}: SourceCodePreviewProps) {
  const sourceKind = useMemo(() => inferSourceKind(path, content), [content, path]);
  const lineCount = useMemo(() => countLines(content), [content]);
  const [expanded, setExpanded] = useState(defaultExpanded);
  const [wrapped, setWrapped] = useState(false);
  const [copied, setCopied] = useState(false);
  const [html, setHtml] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    setHtml(null);
    const lang = shikiLanguage(sourceKind);
    if (!lang) {
      setHtml(escapeHtml(content));
      return;
    }
    codeToHtml(content, {
      lang,
      themes: {
        light: "github-light",
        dark: "github-dark",
      },
      defaultColor: false,
    })
      .then((rendered) => {
        if (!cancelled) setHtml(rendered);
      })
      .catch(() => {
        if (!cancelled) setHtml(escapeHtml(content));
      });
    return () => {
      cancelled = true;
    };
  }, [content, sourceKind]);

  async function copySource() {
    if (!navigator.clipboard?.writeText) return;
    await navigator.clipboard.writeText(content);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1400);
  }

  return (
    <section
      className={cx("ex-source-code-preview", baseUX, skinX, wrapped ? wrappedUX : "")}
    >
      <div className={cx(toolbarClass)}>
        <button
          type="button"
          className={cx(titleClass)}
          aria-expanded={expanded}
          onClick={() => setExpanded((current) => !current)}
        >
          {expanded ? <Icon name="chevron-down" /> : <Icon name="chevron-right" />}
          <span className={cx(kindClass)}>{kind}</span>
          {showPath && <strong>{path || "source"}</strong>}
          <span>{languageLabel(sourceKind)}</span>
          <span>{lineCount} lines</span>
          {relationTypes.length > 0 && <span>via {relationTypes.join(", ")}</span>}
        </button>
        <div className={cx(actionsClass)}>
          <Button
            tone="secondary"
            size="sm"
            className={cx(buttonClass)}
            aria-pressed={wrapped}
            onClick={() => setWrapped((current) => !current)}
            title={wrapped ? "Disable line wrap" : "Enable line wrap"}
          >
            <Icon name="wrap-text" />
          </Button>
          <Button
            tone="secondary"
            size="sm"
            className={cx(buttonClass)}
            onClick={copySource}
            title={copied ? "Copied!" : "Copy source"}
          >
            {copied ? <Icon name="check" /> : <Icon name="copy" />}
          </Button>
        </div>
      </div>
      {expanded && (
        <div className={cx(bodyClass)}>
          {html ? (
            <SafeHtml html={html} className={cx(codeClass)} aria-label="Source code" />
          ) : (
            <pre className={cx(fallbackClass)} aria-label="Source code">
              <code>{content}</code>
            </pre>
          )}
        </div>
      )}
      {!expanded && (
        <div className={cx(collapsedClass)}>
          {lineCount} lines hidden
        </div>
      )}
    </section>
  );
}

function inferSourceKind(path: string, content: string): SourceKind {
  const lower = path.toLowerCase();
  if (lower.endsWith(".rs")) return "rust";
  if (/\.(ts|tsx|mts|cts|js|jsx)$/.test(lower)) return "typescript";
  if (lower.endsWith(".py")) return "python";
  if (/\.(md|markdown)$/.test(lower)) return "markdown";
  if (/\.(yaml|yml)$/.test(lower)) return "yaml";
  if (lower.endsWith(".toml")) return "toml";
  if (lower.endsWith(".json")) return "json";
  if (lower.endsWith(".css")) return "css";
  if (/\.(html|htm)$/.test(lower)) return "html";
  if (/\.(sh|bash|zsh|ksh)$/.test(lower)) return "bash";

  const trimmed = content.trim();
  if (trimmed.startsWith("{") || trimmed.startsWith("[")) return "json";
  if (/^#!\/.*\b(?:bash|sh|zsh|ksh)\b/m.test(trimmed)) return "bash";
  if (/\b(fn|impl|struct|enum|trait|pub|let mut|use|match)\b/.test(trimmed)) return "rust";
  if (/\b(import|export|const|let|interface|type)\b/.test(trimmed)) return "typescript";
  return "text";
}

function shikiLanguage(kind: SourceKind): string | null {
  if (kind === "text") return null;
  return kind;
}

function languageLabel(kind: SourceKind): string {
  switch (kind) {
    case "typescript":
      return "TypeScript";
    case "markdown":
      return "Markdown";
    case "yaml":
      return "YAML";
    case "json":
      return "JSON";
    case "html":
      return "HTML";
    case "css":
      return "CSS";
    case "toml":
      return "TOML";
    case "bash":
      return "Shell";
    case "python":
      return "Python";
    case "rust":
      return "Rust";
    default:
      return "Text";
  }
}

function countLines(content: string): number {
  if (!content) return 0;
  return content.split(/\r?\n/).length;
}

function escapeHtml(content: string): string {
  return `<pre class="shiki"><code>${content
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")}</code></pre>`;
}
