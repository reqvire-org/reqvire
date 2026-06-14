import {
  Children,
  isValidElement,
  useEffect,
  useId,
  useMemo,
  useRef,
  type ReactElement,
  type ReactNode,
} from "react";
import * as d3 from "d3";
import ReactMarkdown, {
  defaultUrlTransform,
  type Components,
} from "react-markdown";
import remarkGfm from "remark-gfm";
import { css, cx } from "@linaria/atomic";
import { cssVar, replaceCssVarsForMermaid } from "@ds";

type MarkdownContentVariant = "detail" | "preview";

interface MarkdownContentProps {
  markdown: string;
  sourceFilePath: string;
  sourceAnchor?: string;
  variant?: MarkdownContentVariant;
  scrollToAnchor?: string | null;
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

const markdownBaseUX = css`
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
  .ex-markdown-table-wrap {
    margin: 0.55em 0;
  }

  ul,
  ol {
    padding-left: var(--space-9);
  }

  li {
    margin: 0.2em 0;
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

  pre code {
    background: transparent;
  }

  blockquote {
    border-left: var(--border-w-heavy) solid var(--border-subtle);
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

const markdownTableWrapUX = css`
  overflow-x: auto;
`;

const markdownEmptySkinX = css`
  color: var(--text-muted);
`;

const diagramBlockUX = css`
  --ex-markdown-diagram-min-w: 520px;
  margin: var(--space-5) 0;
  overflow: auto;
  padding: var(--space-5);

  svg {
    display: block;
    min-width: var(--ex-markdown-diagram-min-w);
  }
`;

const diagramBlockSkinX = css`
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--bg-surface);
`;

export function MarkdownContent({
  markdown,
  sourceFilePath,
  sourceAnchor: _sourceAnchor,
  variant = "detail",
  scrollToAnchor,
}: MarkdownContentProps) {
  const sourceHtmlPath = spaRouteForFile(sourceFilePath);
  const normalizedMarkdown = normalizeReqvireMarkdown(stripYamlFrontmatter(markdown));
  const content =
    variant === "preview" ? markdownPreview(normalizedMarkdown) : normalizedMarkdown.trim();
  const components = useMemo(
    () => markdownComponents({ variant }),
    [variant],
  );

  useEffect(() => {
    if (!scrollToAnchor) return;
    const frame = window.requestAnimationFrame(() => {
      document.getElementById(scrollToAnchor)?.scrollIntoView({ behavior: "smooth" });
    });
    return () => window.cancelAnimationFrame(frame);
  }, [content, scrollToAnchor]);

  if (!content) {
    return <span className={cx("ex-markdown-empty", markdownEmptySkinX)}>-</span>;
  }

  return (
    <div
      className={cx(
        "ex-markdown",
        `ex-markdown-${variant}`,
        markdownBaseUX,
        markdownSkinX,
        variant === "preview" && markdownPreviewUX,
        variant === "preview" && markdownPreviewSkinX,
      )}
    >
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        skipHtml
        unwrapDisallowed={variant === "preview"}
        allowedElements={
          variant === "preview" ? PREVIEW_ALLOWED_ELEMENTS : undefined
        }
        urlTransform={(url) =>
          sourceUrlTransform(url, {
            sourceFilePath,
            sourceHtmlPath,
          })
        }
        components={components}
      >
        {content}
      </ReactMarkdown>
    </div>
  );
}

function stripYamlFrontmatter(markdown: string): string {
  const normalized = markdown.replace(/^\uFEFF/, "");
  const lines = normalized.split(/\r?\n/);
  if (lines[0]?.trim() !== "---") return markdown;

  for (let index = 1; index < lines.length; index += 1) {
    if (lines[index].trim() === "---") {
      const frontmatter = lines.slice(1, index).join("\n");
      if (!/^[A-Za-z0-9_-]+\s*:/m.test(frontmatter)) return markdown;
      return lines.slice(index + 1).join("\n").replace(/^\s+/, "");
    }
  }

  return markdown;
}

export function sourceUrlTransform(
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

  const sourceRoute = safeSpaRoute(
    context.sourceHtmlPath || spaRouteForFile(context.sourceFilePath),
  );

  if (safeUrl.startsWith("#") || safeUrl.startsWith("?")) {
    return sourceRoute ? `${sourceRoute}${safeUrl}` : safeUrl;
  }

  const { path, suffix } = splitUrlSuffix(safeUrl);
  if (!path) {
    return `${sourceRoute}${suffix}`;
  }

  const resolved = resolveRelativePath(dirname(context.sourceFilePath), path);
  if (resolved.toLowerCase().endsWith(".md")) {
    return `${spaRouteForFile(resolved)}${suffix}`;
  }
  return `${resolved}${suffix}`;
}

function markdownPreview(markdown: string): string {
  const normalized = markdown.trim().replace(/\s+/g, " ");
  return normalized.length > PREVIEW_LIMIT
    ? `${normalized.slice(0, PREVIEW_LIMIT).trimEnd()}...`
    : normalized;
}

function normalizeReqvireMarkdown(markdown: string): string {
  const source = stripReqvireDocumentHeading(markdown);
  if (!source.includes("REQVIRE-AUTOGENERATED-DIAGRAM")) return source;

  const lines = source.split(/\r?\n/);
  const output: string[] = [];
  let index = 0;
  let inFence = false;
  let changed = false;

  while (index < lines.length) {
    const line = lines[index];
    if (/^\s*```/.test(line)) {
      inFence = !inFence;
      output.push(line);
      index += 1;
      continue;
    }

    if (!inFence && /^\s*graph\s+(TD|LR|BT|RL)\b;?/i.test(line)) {
      let graphEnd = lines.length;
      for (let cursor = index + 1; cursor < lines.length; cursor += 1) {
        const candidate = lines[cursor];
        if (
          /^\s*```/.test(candidate) ||
          /^#{1,6}\s+\S/.test(candidate) ||
          /^---\s*$/.test(candidate)
        ) {
          graphEnd = cursor;
          break;
        }
      }

      const graphLines = lines.slice(index, graphEnd);
      if (graphLines.some((candidate) => candidate.includes("REQVIRE-AUTOGENERATED-DIAGRAM"))) {
        output.push("```mermaid", ...expandAutogeneratedMermaidLines(graphLines), "```");
        changed = true;
        index = graphEnd;
        continue;
      }
    }

    output.push(line);
    index += 1;
  }

  return changed ? output.join("\n") : markdown;
}

function stripReqvireDocumentHeading(markdown: string): string {
  return markdown.replace(/^\s*#\s+Elements?\s*(?:\r?\n)+/i, "");
}

function expandAutogeneratedMermaidLines(lines: string[]): string[] {
  return lines.flatMap((line) => {
    if (!line.includes("REQVIRE-AUTOGENERATED-DIAGRAM") && !/;\s*\S/.test(line)) {
      return [line];
    }
    return line
      .replace(
        /%%\s*REQVIRE-AUTOGENERATED-DIAGRAM\s*%%\s*(?:Graph styling\s*)?/i,
        "%% REQVIRE-AUTOGENERATED-DIAGRAM; ",
      )
      .split(/;\s*/)
      .map((part) => part.trim())
      .filter(Boolean);
  });
}

function spaRouteForFile(path: string): string {
  return `#/content/${path}`;
}

function isExternalOrAbsoluteUrl(url: string): boolean {
  return /^[a-z][a-z0-9+.-]*:/i.test(url) || url.startsWith("//");
}

function safeSpaRoute(path: string): string {
  if (isExternalOrAbsoluteUrl(path)) return "";
  return path;
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

function markdownComponents({ variant }: { variant: MarkdownContentVariant }): Components {
  const components: Components = {
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
      <div className={cx("ex-markdown-table-wrap", markdownTableWrapUX)}>
        <table {...props}>{children}</table>
      </div>
    );
  },
  img({ src, alt, node: _node, ...props }) {
    void _node;
    if (!src) return null;
    return <img {...props} src={src} alt={alt ?? ""} />;
  },
  h1({ children, node: _node, ...props }) {
    void _node;
    return <h1 id={slugFromChildren(children)} {...props}>{children}</h1>;
  },
  h2({ children, node: _node, ...props }) {
    void _node;
    return <h2 id={slugFromChildren(children)} {...props}>{children}</h2>;
  },
  h3({ children, node: _node, ...props }) {
    void _node;
    return <h3 id={slugFromChildren(children)} {...props}>{children}</h3>;
  },
  h4({ children, node: _node, ...props }) {
    void _node;
    return <h4 id={slugFromChildren(children)} {...props}>{children}</h4>;
  },
  pre({ children, node: _node, ...props }) {
    void _node;
    const block = codeBlockFromPreChildren(children);
    if (block && variant === "detail" && block.language === "mermaid") {
      return <MermaidBlock code={block.code} />;
    }
    if (
      block &&
      variant === "detail" &&
      ["d3-tree", "d3-sunburst", "d3-icicle"].includes(block.language)
    ) {
      return (
        <D3HierarchyBlock
          code={block.code}
          mode={block.language.replace("d3-", "") as D3HierarchyMode}
        />
      );
    }
    if (block && variant === "detail" && block.language === "d3-sankey") {
      return <D3JsonNotice code={block.code} label="D3 sankey" />;
    }
    return <pre {...props}>{children}</pre>;
  },
  code({ className, children, node: _node, ...props }) {
    void _node;
    return <code className={className} {...props}>{children}</code>;
  },
  };
  return components;
}

function codeBlockFromPreChildren(
  children: ReactNode,
): { language: string; code: string } | null {
  const child = Children.toArray(children)[0];
  if (!isValidElement(child)) return null;
  const element = child as ReactElement<{ className?: string; children?: ReactNode }>;
  const language = /language-([a-z0-9-]+)/i.exec(element.props.className ?? "")?.[1] ?? "";
  if (!language) return null;
  return {
    language,
    code: String(element.props.children ?? "").replace(/\n$/, ""),
  };
}

type MermaidApi = typeof import("mermaid").default;

let mermaidPromise: Promise<MermaidApi> | null = null;

function loadMermaid(): Promise<MermaidApi> {
  if (window.mermaid) {
    return Promise.resolve(window.mermaid);
  }

  if (!mermaidPromise) {
    mermaidPromise = import("mermaid").then(({ default: mermaid }) => {
      mermaid.initialize({
        startOnLoad: false,
        theme: "neutral",
        maxTextSize: 5000000,
        maxEdges: 50000,
        flowchart: {
          useMaxWidth: true,
          htmlLabels: true,
          curve: "basis",
        },
        securityLevel: "loose",
        logLevel: "error",
      });
      window.mermaid = mermaid;
      return mermaid;
    }).catch((error) => {
      mermaidPromise = null;
      throw error;
    });
  }

  return mermaidPromise;
}

export function MermaidBlock({
  code,
  nodeClickTargets,
  onNodeClick,
  onRenderSettled,
}: {
  code: string;
  nodeClickTargets?: ReadonlyMap<string, string>;
  onNodeClick?: (id: string) => void;
  onRenderSettled?: () => void;
}) {
  const ref = useRef<HTMLDivElement | null>(null);
  const renderId = useId().replace(/[^a-zA-Z0-9_-]/g, "");
  const renderCode = useMemo(() => normalizeMermaidForRender(code), [code]);

  useEffect(() => {
    if (!ref.current) return;
    let cancelled = false;
    const target = ref.current;
    let cleanupInteraction: (() => void) | undefined;
    let cleanupNodeClicks: (() => void) | undefined;
    let settled = false;
    const settle = () => {
      if (settled) return;
      settled = true;
      onRenderSettled?.();
    };
    target.textContent = "";
    loadMermaid().then((mermaid) => {
      if (cancelled) return null;
      return mermaid.render(`reqvire-mermaid-${renderId}`, renderCode);
    }).then((rendered) => {
      if (!rendered) return;
      if (cancelled) {
        settle();
        return;
      }
      const svg = typeof rendered === "string" ? rendered : rendered.svg;
      renderMermaidSvg(target, svg);
      if (typeof rendered !== "string") {
        rendered.bindFunctions?.(target);
      }
      cleanupNodeClicks = bindMermaidNodeClicks(target, nodeClickTargets, onNodeClick);
      try {
        cleanupInteraction = initializeMermaidInteraction(target);
      } catch (error) {
        console.warn("[Reqvire Explorer] Mermaid interaction setup failed", error);
      }
      settle();
    }).catch((error) => {
      if (cancelled) {
        settle();
        return;
      }
      console.warn("[Reqvire Explorer] Mermaid render failed", error);
      if (ref.current) {
        renderDiagramFallback(ref.current);
      }
      settle();
    });
    return () => {
      cancelled = true;
      cleanupInteraction?.();
      cleanupNodeClicks?.();
      settle();
    };
  }, [nodeClickTargets, onNodeClick, onRenderSettled, renderCode, renderId]);

  return <div ref={ref} className="mermaid" />;
}

function normalizeMermaidForRender(code: string): string {
  return replaceCssVarsForMermaid(code);
}

function renderMermaidSvg(target: HTMLElement, svg: string) {
  const template = document.createElement("template");
  template.innerHTML = svg.trim();
  const svgElement = template.content.querySelector("svg");
  if (!svgElement) {
    throw new Error("Mermaid did not return an SVG document");
  }
  target.replaceChildren(document.importNode(svgElement, true));
}

function renderDiagramFallback(target: HTMLElement) {
  const fallback = document.createElement("div");
  fallback.className = cx("ex-diagram-fallback", diagramBlockUX, diagramBlockSkinX);
  fallback.textContent = "Unable to render Mermaid diagram.";
  target.replaceChildren(fallback);
}

function bindMermaidNodeClicks(
  target: HTMLElement,
  nodeClickTargets?: ReadonlyMap<string, string>,
  onNodeClick?: (id: string) => void,
): (() => void) | undefined {
  if (!nodeClickTargets || nodeClickTargets.size === 0 || !onNodeClick) return undefined;
  const svg = target.querySelector("svg");
  if (!(svg instanceof SVGSVGElement)) return undefined;

  const graphNodes = [...svg.querySelectorAll<SVGGElement>(".node")];
  const boundNodes: SVGGElement[] = [];
  for (const [nodeId, elementId] of nodeClickTargets.entries()) {
    for (const node of graphNodes) {
      if (!mermaidSvgNodeMatches(node, nodeId)) continue;
      node.dataset.reqvireElementId = elementId;
      node.classList.add("is-reqvire-clickable-node");
      node.setAttribute("role", "link");
      node.setAttribute("tabindex", "0");
      boundNodes.push(node);
    }
  }

  const onClick = (event: MouseEvent) => {
    const targetElement = event.target;
    if (!(targetElement instanceof Element)) return;
    const node = targetElement.closest<SVGGElement>("[data-reqvire-element-id]");
    const elementId = node?.dataset.reqvireElementId;
    if (!elementId) return;
    event.preventDefault();
    event.stopPropagation();
    onNodeClick(elementId);
  };

  const onKeyDown = (event: KeyboardEvent) => {
    if (event.key !== "Enter" && event.key !== " ") return;
    const targetElement = event.target;
    if (!(targetElement instanceof Element)) return;
    const node = targetElement.closest<SVGGElement>("[data-reqvire-element-id]");
    const elementId = node?.dataset.reqvireElementId;
    if (!elementId) return;
    event.preventDefault();
    event.stopPropagation();
    onNodeClick(elementId);
  };

  svg.addEventListener("click", onClick);
  svg.addEventListener("keydown", onKeyDown);

  return () => {
    svg.removeEventListener("click", onClick);
    svg.removeEventListener("keydown", onKeyDown);
    for (const node of boundNodes) {
      delete node.dataset.reqvireElementId;
      node.classList.remove("is-reqvire-clickable-node");
      node.removeAttribute("role");
      node.removeAttribute("tabindex");
    }
  };
}

function mermaidSvgNodeMatches(node: Element, mermaidNodeId: string): boolean {
  const candidates = [
    node.id,
    node.getAttribute("data-id"),
    node.getAttribute("data-node-id"),
  ].filter(Boolean);
  return candidates.some((candidate) => (
    candidate === mermaidNodeId
    || candidate === `flowchart-${mermaidNodeId}`
    || candidate?.includes(`-${mermaidNodeId}-`)
    || candidate?.includes(mermaidNodeId)
  ));
}

function initializeMermaidInteraction(container: HTMLDivElement): () => void {
  const svg = container.querySelector("svg");
  if (!(svg instanceof SVGSVGElement)) return () => undefined;

  container.classList.add("mermaid-interactive");

  const naturalBox = readSvgBox(svg);
  if (!naturalBox) return () => undefined;

  const padding = 48;
  const baseViewBox = {
    x: naturalBox.x - padding / 2,
    y: naturalBox.y - padding / 2,
    width: Math.max(1, naturalBox.width + padding),
    height: Math.max(1, naturalBox.height + padding),
  };
  let initialViewBox = { ...baseViewBox };
  let viewBox = { ...baseViewBox };
  let dragging = false;
  let dragStart: { x: number; y: number; viewBox: typeof viewBox } | null = null;

  const applyViewBox = () => {
    svg.setAttribute(
      "viewBox",
      `${viewBox.x} ${viewBox.y} ${viewBox.width} ${viewBox.height}`,
    );
  };

  svg.removeAttribute("width");
  svg.removeAttribute("height");
  svg.style.maxWidth = "none";
  svg.style.width = "100%";
  svg.style.height = "100%";

  requestAnimationFrame(() => {
    const containerWidth = container.clientWidth || 720;
    const boundedHeight = readBoundedMermaidHeight(container);
    const fitInsideContainer = boundedHeight !== null;
    const minViewportHeight = boundedHeight === null ? 460 : Math.min(260, boundedHeight);
    const viewportHeight = boundedHeight ?? Math.max(460, window.innerHeight - 180);
    const naturalHeight = Math.ceil(baseViewBox.height + 24);
    const desiredHeight = Math.min(Math.max(minViewportHeight, naturalHeight), viewportHeight);
    const largeDiagram = baseViewBox.width > containerWidth || naturalHeight > desiredHeight;
    container.style.minHeight = "0";
    container.style.height = largeDiagram
      ? `${desiredHeight}px`
      : `${Math.min(Math.max(minViewportHeight, naturalHeight), viewportHeight)}px`;

    if (fitInsideContainer) {
      initialViewBox = { ...baseViewBox };
      viewBox = { ...initialViewBox };
      applyViewBox();
      return;
    }

    if (largeDiagram) {
      initialViewBox = {
        x: baseViewBox.x,
        y: baseViewBox.y,
        width: Math.min(baseViewBox.width, Math.max(760, containerWidth)),
        height: Math.min(baseViewBox.height, desiredHeight),
      };
      viewBox = { ...initialViewBox };
      applyViewBox();
      return;
    }

    initialViewBox = { ...baseViewBox };
    viewBox = { ...initialViewBox };
    applyViewBox();
  });

  const zoomAt = (factor: number, clientX?: number, clientY?: number) => {
    const rect = svg.getBoundingClientRect();
    const px = clientX === undefined || rect.width === 0 ? 0.5 : (clientX - rect.left) / rect.width;
    const py = clientY === undefined || rect.height === 0 ? 0.5 : (clientY - rect.top) / rect.height;
    const nextWidth = clamp(viewBox.width * factor, initialViewBox.width / 8, baseViewBox.width * 4);
    const nextHeight = clamp(viewBox.height * factor, initialViewBox.height / 8, baseViewBox.height * 4);
    const anchorX = viewBox.x + viewBox.width * px;
    const anchorY = viewBox.y + viewBox.height * py;
    viewBox = {
      x: anchorX - nextWidth * px,
      y: anchorY - nextHeight * py,
      width: nextWidth,
      height: nextHeight,
    };
    applyViewBox();
  };

  const panBy = (dx: number, dy: number) => {
    viewBox = {
      ...viewBox,
      x: viewBox.x + dx,
      y: viewBox.y + dy,
    };
    applyViewBox();
  };

  const reset = () => {
    viewBox = { ...initialViewBox };
    applyViewBox();
  };

  const controls = document.createElement("div");
  controls.className = "diagram-nav-buttons";
  [
    ["zoom-in", "+", "Zoom in"],
    ["zoom-out", "-", "Zoom out"],
    ["reset", "Reset", "Reset diagram view"],
    ["up", "Up", "Pan up"],
    ["down", "Down", "Pan down"],
    ["left", "Left", "Pan left"],
    ["right", "Right", "Pan right"],
  ].forEach(([action, label, ariaLabel]) => {
    const button = document.createElement("button");
    button.className = "diagram-nav-btn";
    button.type = "button";
    button.dataset.action = action;
    button.setAttribute("aria-label", ariaLabel);
    button.textContent = label;
    controls.appendChild(button);
  });
  container.appendChild(controls);

  const onControlsClick = (event: MouseEvent) => {
    const button = (event.target as HTMLElement).closest<HTMLButtonElement>("[data-action]");
    if (!button) return;
    event.preventDefault();
    const stepX = viewBox.width * 0.12;
    const stepY = viewBox.height * 0.12;
    switch (button.dataset.action) {
      case "zoom-in":
        zoomAt(0.8);
        break;
      case "zoom-out":
        zoomAt(1.25);
        break;
      case "reset":
        reset();
        break;
      case "up":
        panBy(0, -stepY);
        break;
      case "down":
        panBy(0, stepY);
        break;
      case "left":
        panBy(-stepX, 0);
        break;
      case "right":
        panBy(stepX, 0);
        break;
    }
  };

  const onWheel = (event: WheelEvent) => {
    if (!event.ctrlKey && !event.metaKey) return;
    event.preventDefault();
    zoomAt(event.deltaY < 0 ? 0.88 : 1.14, event.clientX, event.clientY);
  };

  const onPointerDown = (event: PointerEvent) => {
    if (event.button !== 0) return;
    const targetElement = event.target;
    if (
      targetElement instanceof Element
      && targetElement.closest("[data-reqvire-element-id], a[href]")
    ) {
      return;
    }
    dragging = true;
    dragStart = { x: event.clientX, y: event.clientY, viewBox: { ...viewBox } };
    svg.setPointerCapture(event.pointerId);
    container.classList.add("is-panning");
  };

  const onPointerMove = (event: PointerEvent) => {
    if (!dragging || !dragStart) return;
    const rect = svg.getBoundingClientRect();
    if (rect.width === 0 || rect.height === 0) return;
    const dx = ((event.clientX - dragStart.x) / rect.width) * dragStart.viewBox.width;
    const dy = ((event.clientY - dragStart.y) / rect.height) * dragStart.viewBox.height;
    viewBox = {
      ...dragStart.viewBox,
      x: dragStart.viewBox.x - dx,
      y: dragStart.viewBox.y - dy,
    };
    applyViewBox();
  };

  const onPointerUp = (event: PointerEvent) => {
    dragging = false;
    dragStart = null;
    container.classList.remove("is-panning");
    if (svg.hasPointerCapture(event.pointerId)) {
      svg.releasePointerCapture(event.pointerId);
    }
  };

  controls.addEventListener("click", onControlsClick);
  svg.addEventListener("wheel", onWheel, { passive: false });
  svg.addEventListener("pointerdown", onPointerDown);
  svg.addEventListener("pointermove", onPointerMove);
  svg.addEventListener("pointerup", onPointerUp);
  svg.addEventListener("pointercancel", onPointerUp);
  enableMermaidEdgeHighlight(svg);

  return () => {
    controls.removeEventListener("click", onControlsClick);
    controls.remove();
    svg.removeEventListener("wheel", onWheel);
    svg.removeEventListener("pointerdown", onPointerDown);
    svg.removeEventListener("pointermove", onPointerMove);
    svg.removeEventListener("pointerup", onPointerUp);
    svg.removeEventListener("pointercancel", onPointerUp);
  };
}

function readBoundedMermaidHeight(container: HTMLElement): number | null {
  const wrapper = container.closest(".trace-rollup-diagram");
  if (!(wrapper instanceof HTMLElement)) return null;
  const style = window.getComputedStyle(wrapper);
  const verticalPadding =
    Number.parseFloat(style.paddingTop || "0") + Number.parseFloat(style.paddingBottom || "0");
  return Math.max(1, wrapper.clientHeight - verticalPadding);
}

function readSvgBox(svg: SVGSVGElement) {
  try {
    const box = svg.getBBox();
    if (box.width > 0 && box.height > 0) return box;
  } catch {
    // jsdom and partially-rendered SVGs can throw here; fall through to viewBox.
  }
  const viewBox = svg.viewBox?.baseVal;
  if (!viewBox) return null;
  if (viewBox.width > 0 && viewBox.height > 0) return viewBox;
  return null;
}

function enableMermaidEdgeHighlight(svg: SVGSVGElement) {
  const nodes = [...svg.querySelectorAll<SVGGElement>(".node")];
  const edges = [...svg.querySelectorAll<SVGPathElement>("path.flowchart-link, .edges path")];
  const clear = () => {
    edges.forEach((edge) => edge.classList.remove("edge-highlighted"));
    nodes.forEach((node) => {
      node.style.filter = "";
    });
  };
  nodes.forEach((node) => {
    node.addEventListener("mouseenter", () => {
      clear();
      node.style.filter = `drop-shadow(0 0 var(--space-4) ${cssVar("--accent-ring")})`;
      edges.forEach((edge) => edge.classList.add("edge-highlighted"));
    });
    node.addEventListener("mouseleave", clear);
  });
}

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max);
}

type D3HierarchyMode = "tree" | "sunburst" | "icicle";

interface D3HierarchyDatum {
  name?: string;
  children?: D3HierarchyDatum[];
  value?: number;
}

function D3HierarchyBlock({
  code,
  mode,
}: {
  code: string;
  mode: D3HierarchyMode;
}) {
  const ref = useRef<SVGSVGElement | null>(null);
  const parseResult = useMemo(() => parseD3Hierarchy(code), [code]);

  useEffect(() => {
    const svgNode = ref.current;
    if (!svgNode || !parseResult.ok) return;
    renderD3Hierarchy(svgNode, parseResult.data, mode);
  }, [mode, parseResult]);

  if (!parseResult.ok) {
    return <D3JsonNotice code={code} label={`D3 ${mode}`} />;
  }

  return (
    <div className={cx("ex-d3-block", `ex-d3-${mode}`, diagramBlockUX, diagramBlockSkinX)}>
      <svg ref={ref} role="img" aria-label={`D3 ${mode} diagram`} />
    </div>
  );
}

function D3JsonNotice({ code, label }: { code: string; label: string }) {
  return (
    <div className={cx("ex-diagram-fallback", diagramBlockUX, diagramBlockSkinX)}>
      <strong>{label}</strong>
      <pre><code>{code}</code></pre>
    </div>
  );
}

function parseD3Hierarchy(code: string):
  | { ok: true; data: D3HierarchyDatum }
  | { ok: false } {
  try {
    const data = JSON.parse(code) as D3HierarchyDatum;
    if (!data || typeof data !== "object") return { ok: false };
    return { ok: true, data };
  } catch {
    return { ok: false };
  }
}

function renderD3Hierarchy(
  svgNode: SVGSVGElement,
  data: D3HierarchyDatum,
  mode: D3HierarchyMode,
) {
  const width = Math.max(680, svgNode.parentElement?.clientWidth ?? 680);
  const height = mode === "tree" ? 460 : 560;
  const svg = d3.select(svgNode);
  svg.selectAll("*").remove();
  svg.attr("viewBox", `0 0 ${width} ${height}`).attr("width", "100%").attr("height", height);

  const root = d3.hierarchy(data).sum((datum) => datum.value ?? 1);
  if (mode === "tree") {
    const tree = d3.tree<D3HierarchyDatum>().size([height - 60, width - 180]);
    const laidOut = tree(root);
    const group = svg.append("g").attr("transform", "translate(80,30)");
    group.selectAll("path")
      .data(laidOut.links())
      .join("path")
      .attr("d", d3.linkHorizontal<d3.HierarchyPointLink<D3HierarchyDatum>, d3.HierarchyPointNode<D3HierarchyDatum>>()
        .x((node) => node.y)
        .y((node) => node.x))
      .attr("fill", "none")
      .attr("stroke", cssVar("--edge-default"));
    const nodes = group.selectAll("g")
      .data(laidOut.descendants())
      .join("g")
      .attr("transform", (node) => `translate(${node.y},${node.x})`);
    nodes.append("circle").attr("r", 5).attr("fill", cssVar("--accent"));
    nodes.append("text")
      .attr("x", 9)
      .attr("dy", "0.32em")
      .attr("font-size", 12)
      .text((node) => truncateText(node.data.name ?? "node", 34));
    return;
  }

  const radius = Math.min(width, height) / 2 - 18;
  const partition = d3.partition<D3HierarchyDatum>().size([2 * Math.PI, radius]);
  const laidOut = partition(root);
  const group = svg.append("g").attr("transform", `translate(${width / 2},${height / 2})`);
  const color = d3.scaleOrdinal(d3.schemeTableau10);
  const arc = d3.arc<d3.HierarchyRectangularNode<D3HierarchyDatum>>()
    .startAngle((node) => node.x0)
    .endAngle((node) => node.x1)
    .innerRadius((node) => mode === "sunburst" ? node.y0 : node.y0 * 0.92)
    .outerRadius((node) => mode === "sunburst" ? node.y1 : node.y1 * 0.92);
  group.selectAll("path")
    .data(laidOut.descendants().filter((node) => node.depth > 0))
    .join("path")
    .attr("d", arc)
    .attr("fill", (node) => color(String(node.depth)))
    .attr("stroke", cssVar("--bg-surface"))
    .append("title")
    .text((node) => node.ancestors().reverse().map((ancestor) => ancestor.data.name ?? "node").join(" / "));
  group.append("text")
    .attr("text-anchor", "middle")
    .attr("font-size", 13)
    .attr("font-weight", 700)
    .text(data.name ?? "root");
}

function slugFromChildren(children: unknown): string {
  return String(children)
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9\s-]/g, "")
    .replace(/\s+/g, "-");
}

function truncateText(value: string, max: number) {
  return value.length > max ? `${value.slice(0, Math.max(1, max - 1))}...` : value;
}
