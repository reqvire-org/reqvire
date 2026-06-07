import ReactMarkdown, {
  defaultUrlTransform,
  type Components,
} from "react-markdown";
import remarkGfm from "remark-gfm";

type MarkdownContentVariant = "detail" | "preview";

interface MarkdownContentProps {
  markdown: string;
  sourceFilePath: string;
  sourceAnchor?: string;
  variant?: MarkdownContentVariant;
}

const PREVIEW_LIMIT = 420;
const PREVIEW_ALLOWED_ELEMENTS = [
  "p",
  "strong",
  "em",
  "a",
  "code",
  "br",
  "ul",
  "ol",
  "li",
];

export function MarkdownContent({
  markdown,
  sourceFilePath,
  sourceAnchor,
  variant = "detail",
}: MarkdownContentProps) {
  const sourceHtmlPath =
    sourceAnchor?.split("#")[0] || markdownPathToHtml(sourceFilePath);
  const content =
    variant === "preview" ? markdownPreview(markdown) : markdown.trim();

  if (!content) {
    return <span className="reqvire-markdown-empty">-</span>;
  }

  return (
    <div className={`reqvire-markdown reqvire-markdown-${variant}`}>
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        skipHtml
        unwrapDisallowed={variant === "preview"}
        allowedElements={
          variant === "preview" ? PREVIEW_ALLOWED_ELEMENTS : undefined
        }
        urlTransform={(url) =>
          staticExportUrlTransform(url, {
            sourceFilePath,
            sourceHtmlPath,
          })
        }
        components={markdownComponents}
      >
        {content}
      </ReactMarkdown>
    </div>
  );
}

export function staticExportUrlTransform(
  url: string,
  context: { sourceFilePath: string; sourceHtmlPath?: string },
): string {
  const safeUrl = defaultUrlTransform(url);
  if (!safeUrl) return "";

  if (isExternalOrAbsoluteUrl(safeUrl) || safeUrl.startsWith("/")) {
    return safeUrl;
  }

  if (safeUrl.startsWith("#/")) {
    return safeUrl;
  }

  const sourceHtmlPath = safeStaticSourcePath(
    context.sourceHtmlPath || markdownPathToHtml(context.sourceFilePath),
  );

  if (safeUrl.startsWith("#") || safeUrl.startsWith("?")) {
    return sourceHtmlPath ? `${sourceHtmlPath}${safeUrl}` : safeUrl;
  }

  const { path, suffix } = splitUrlSuffix(safeUrl);
  if (!path) {
    return `${sourceHtmlPath}${suffix}`;
  }

  return `${resolveRelativePath(dirname(context.sourceFilePath), markdownPathToHtml(path))}${suffix}`;
}

function markdownPreview(markdown: string): string {
  const normalized = markdown.trim().replace(/\s+/g, " ");
  return normalized.length > PREVIEW_LIMIT
    ? `${normalized.slice(0, PREVIEW_LIMIT).trimEnd()}...`
    : normalized;
}

function markdownPathToHtml(path: string): string {
  return path.replace(/\.md$/i, ".html");
}

function isExternalOrAbsoluteUrl(url: string): boolean {
  return /^[a-z][a-z0-9+.-]*:/i.test(url) || url.startsWith("//");
}

function safeStaticSourcePath(path: string): string {
  if (isExternalOrAbsoluteUrl(path)) return "";
  if (path.startsWith("/")) return path;
  return resolveRelativePath("", path);
}

function splitUrlSuffix(url: string): { path: string; suffix: string } {
  const hashIndex = url.indexOf("#");
  const queryIndex = url.indexOf("?");
  const suffixIndex =
    hashIndex === -1
      ? queryIndex
      : queryIndex === -1
        ? hashIndex
        : Math.min(hashIndex, queryIndex);

  if (suffixIndex === -1) {
    return { path: url, suffix: "" };
  }

  return {
    path: url.slice(0, suffixIndex),
    suffix: url.slice(suffixIndex),
  };
}

function dirname(path: string): string {
  const index = path.lastIndexOf("/");
  return index === -1 ? "" : path.slice(0, index);
}

function resolveRelativePath(baseDir: string, targetPath: string): string {
  const segments = `${baseDir ? `${baseDir}/` : ""}${targetPath}`.split("/");
  const resolved: string[] = [];

  for (const segment of segments) {
    if (!segment || segment === ".") continue;
    if (segment === "..") {
      if (resolved.length > 0) {
        resolved.pop();
      } else {
        resolved.push(segment);
      }
      continue;
    }
    resolved.push(segment);
  }

  return resolved.join("/");
}

const markdownComponents: Components = {
  a({ href, children, node: _node, ...props }) {
    void _node;
    const external = Boolean(href && /^[a-z][a-z0-9+.-]*:/i.test(href));
    return (
      <a
        {...props}
        href={href}
        target={external ? "_blank" : undefined}
        rel={external ? "noreferrer" : undefined}
      >
        {children}
      </a>
    );
  },
  table({ children, node: _node, ...props }) {
    void _node;
    return (
      <div className="reqvire-markdown-table-wrap">
        <table {...props}>{children}</table>
      </div>
    );
  },
};
