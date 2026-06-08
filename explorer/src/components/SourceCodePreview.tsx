import { useEffect, useMemo, useState } from "react";
import { codeToHtml } from "shiki";
import { Icon } from "@ds";
import { SafeHtml } from "./SafeHtml";

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
      className={[
        "source-code-preview",
        "rq-card",
        expanded ? "is-expanded" : "is-collapsed",
        wrapped ? "is-wrapped" : "is-unwrapped",
      ].join(" ")}
    >
      <div className="source-code-preview-toolbar">
        <button
          type="button"
          className="source-code-preview-title"
          aria-expanded={expanded}
          onClick={() => setExpanded((current) => !current)}
        >
          {expanded ? <Icon name="chevron-down" /> : <Icon name="chevron-right" />}
          <span className="source-code-preview-kind">{kind}</span>
          {showPath && <strong>{path || "source"}</strong>}
          <span>{languageLabel(sourceKind)}</span>
          <span>{lineCount} lines</span>
          {relationTypes.length > 0 && <span>via {relationTypes.join(", ")}</span>}
        </button>
        <div className="source-code-preview-actions">
          <button
          type="button"
          className="source-code-preview-button rq-btn rq-btn--secondary rq-btn--sm"
          aria-pressed={wrapped}
          onClick={() => setWrapped((current) => !current)}
          title={wrapped ? "Disable line wrap" : "Enable line wrap"}
          >
            <Icon name="wrap-text" />
            <span>{wrapped ? "No wrap" : "Wrap"}</span>
          </button>
          <button
          type="button"
          className="source-code-preview-button rq-btn rq-btn--secondary rq-btn--sm"
          onClick={copySource}
          title="Copy source"
          >
            {copied ? <Icon name="check" /> : <Icon name="copy" />}
            <span>{copied ? "Copied" : "Copy"}</span>
          </button>
        </div>
      </div>
      {expanded && (
        <div className="source-code-preview-body">
          {html ? (
            <SafeHtml html={html} className="source-code-preview-code" aria-label="Source code" />
          ) : (
            <pre className="source-code-preview-fallback" aria-label="Source code">
              <code>{content}</code>
            </pre>
          )}
        </div>
      )}
      {!expanded && (
        <div className="source-code-preview-collapsed">
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
