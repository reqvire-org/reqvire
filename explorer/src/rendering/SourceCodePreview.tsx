import { useEffect, useMemo, useState } from "react";
import { codeToHtml } from "shiki";
import {
  CODE_PREVIEW_CODE_CLASS,
  CODE_PREVIEW_FALLBACK_CLASS,
  CodeBody,
  CodePreviewFrame,
  CodeToolbar,
} from "@ds";
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
    <CodePreviewFrame wrapped={wrapped}>
      <CodeToolbar
        expanded={expanded}
        onToggleExpanded={() => setExpanded((current) => !current)}
        kind={kind}
        path={path}
        showPath={showPath}
        languageLabel={languageLabel(sourceKind)}
        lineCount={lineCount}
        relationTypes={relationTypes}
        wrapped={wrapped}
        copied={copied}
        onToggleWrap={() => setWrapped((current) => !current)}
        onCopy={copySource}
      />
      {expanded && (
        <CodeBody>
          {html ? (
            <SafeHtml html={html} className={CODE_PREVIEW_CODE_CLASS} aria-label="Source code" />
          ) : (
            <pre className={CODE_PREVIEW_FALLBACK_CLASS} aria-label="Source code">
              <code>{content}</code>
            </pre>
          )}
        </CodeBody>
      )}
      {!expanded && <CodeBody collapsed lineCount={lineCount} />}
    </CodePreviewFrame>
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
