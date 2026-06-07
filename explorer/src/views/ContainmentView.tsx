import { useEffect, useMemo, useRef, useState } from "react";
import * as d3 from "d3";
import {
  Badge,
  Box,
  Code,
  Flex,
  Grid,
  Heading,
  Text,
  TextField,
} from "@radix-ui/themes";
import { CubeIcon, FileIcon, MagnifyingGlassIcon } from "@radix-ui/react-icons";
import { useStore } from "../store/StoreContext";
import { ViewFrame } from "./ViewFrame";
import { MarkdownContent } from "../components/MarkdownContent";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";
import { useExplorerUiState } from "../components/ExplorerUiState";
import type {
  ExplorerProjectStore,
  ProjectStoreElement,
  ProjectStoreFile,
} from "../store/types";

type ContainmentKind = "root" | "folder" | "file" | "element";

interface ContainmentNode {
  id: string;
  name: string;
  kind: ContainmentKind;
  path?: string;
  element?: ProjectStoreElement;
  file?: ProjectStoreFile;
  children: ContainmentNode[];
}

const KIND_COLORS: Record<ContainmentKind, string> = {
  root: "#172027",
  folder: "#52605b",
  file: "#00897b",
  element: "#673ab7",
};

function displayName(path: string): string {
  return path.split("/").filter(Boolean).at(-1) ?? path;
}

function buildContainmentTree(store: ExplorerProjectStore): ContainmentNode {
  const root: ContainmentNode = {
    id: "__root__",
    name: store.project.root_label || store.project.name || "Project",
    kind: "root",
    children: [],
  };
  const folderNodes = new Map<string, ContainmentNode>();

  for (const folder of store.folders) {
    folderNodes.set(folder.path, {
      id: `folder:${folder.path}`,
      name: displayName(folder.path),
      kind: "folder",
      path: folder.path,
      children: [],
    });
  }

  for (const folder of store.folders) {
    const node = folderNodes.get(folder.path);
    if (!node) continue;
    const parent = folder.parent ? folderNodes.get(folder.parent) : root;
    (parent ?? root).children.push(node);
  }

  const elementById = new Map(store.elements.map((element) => [element.id, element]));
  const attachedElements = new Set<string>();

  for (const file of store.files) {
    const fileNode: ContainmentNode = {
      id: `file:${file.path}`,
      name: file.display_path || displayName(file.path),
      kind: "file",
      path: file.path,
      file,
      children: [],
    };

    for (const id of file.element_ids) {
      const element = elementById.get(id);
      fileNode.children.push({
        id,
        name: element?.name ?? id,
        kind: "element",
        path: element?.file_path ?? file.path,
        element,
        children: [],
      });
      attachedElements.add(id);
    }

    const parent = folderNodes.get(file.parent_folder) ?? root;
    parent.children.push(fileNode);
  }

  for (const element of store.elements) {
    if (attachedElements.has(element.id)) continue;
    root.children.push({
      id: element.id,
      name: element.name,
      kind: "element",
      path: element.file_path,
      element,
      children: [],
    });
  }

  sortContainment(root);
  return root;
}

function sortContainment(node: ContainmentNode) {
  const order: Record<ContainmentKind, number> = {
    root: 0,
    folder: 1,
    file: 2,
    element: 3,
  };
  node.children.sort((a, b) => {
    const kind = order[a.kind] - order[b.kind];
    return kind === 0 ? a.name.localeCompare(b.name) : kind;
  });
  node.children.forEach(sortContainment);
}

function nodeCount(node: ContainmentNode): number {
  return 1 + node.children.reduce((sum, child) => sum + nodeCount(child), 0);
}

function elementCount(node: ContainmentNode): number {
  return (
    (node.kind === "element" ? 1 : 0) +
    node.children.reduce((sum, child) => sum + elementCount(child), 0)
  );
}

function breadcrumbs(node: ContainmentNode): string[] {
  const path = node.path?.split("/").filter(Boolean) ?? [];
  if (node.kind === "element") return [...path, node.name];
  if (path.length > 0) return path;
  return [node.name];
}

function labelForKind(kind: ContainmentKind): string {
  return kind === "root" ? "project" : kind;
}

export function ContainmentView({
  frameTestId = "containment",
  modeOverride,
  onOpenElement,
}: {
  frameTestId?: string;
  modeOverride?: "sunburst" | "icicle";
  onOpenElement: (id: string) => void;
} & Partial<ExplorerViewProps>) {
  const { store } = useStore();
  const { containmentMode } = useExplorerUiState();
  const mode = modeOverride ?? containmentMode;
  const [selectedId, setSelectedId] = useState<string>("__root__");
  const [query, setQuery] = useState("");

  const tree = useMemo(() => buildContainmentTree(store), [store]);
  const selected = useMemo(() => findNode(tree, selectedId) ?? tree, [selectedId, tree]);
  const totals = useMemo(
    () => ({
      folders: store.folders.length,
      files: store.files.length,
      elements: store.elements.length,
      nodes: nodeCount(tree),
    }),
    [store.elements.length, store.files.length, store.folders.length, tree],
  );
  const searchResults = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return [];
    return flattenContainment(tree)
      .filter((node) =>
        [
          node.name,
          node.path ?? "",
          node.element?.element_type ?? "",
          node.element?.content ?? "",
        ]
          .join(" ")
          .toLowerCase()
          .includes(q),
      )
      .slice(0, 30);
  }, [query, tree]);

  return (
    <ViewFrame testId={frameTestId}>
      <Grid
        columns={{ initial: "1fr", lg: "minmax(0, 1fr) 390px" }}
        className="explorer-route"
      >
        <Box className="explorer-main-panel containment-canvas">
          {mode === "list" && (
            <ListMode
              root={tree}
              selectedId={selected.id}
              onSelect={setSelectedId}
              onOpenElement={onOpenElement}
            />
          )}
          {mode === "sunburst" && (
            <SunburstMode
              root={tree}
              selectedId={selected.id}
              onSelect={setSelectedId}
              onOpenElement={onOpenElement}
            />
          )}
          {mode === "icicle" && (
            <IcicleMode
              root={tree}
              selectedId={selected.id}
              onSelect={setSelectedId}
              onOpenElement={onOpenElement}
            />
          )}
          {store.files.length === 0 && store.elements.length === 0 && (
            <Text color="gray">No containment data in store.</Text>
          )}
        </Box>

        <Inspector
          selected={selected}
          totals={totals}
          query={query}
          searchResults={searchResults}
          onQueryChange={setQuery}
          onSelect={setSelectedId}
          onOpenElement={onOpenElement}
        />
      </Grid>
    </ViewFrame>
  );
}

function findNode(node: ContainmentNode, id: string): ContainmentNode | null {
  if (node.id === id) return node;
  for (const child of node.children) {
    const result = findNode(child, id);
    if (result) return result;
  }
  return null;
}

function ListMode({
  root,
  selectedId,
  onSelect,
  onOpenElement,
}: {
  root: ContainmentNode;
  selectedId: string;
  onSelect: (id: string) => void;
  onOpenElement: (id: string) => void;
}) {
  const files = collectFiles(root);
  return (
    <Flex data-testid="containment-list" direction="column" gap="3">
      {files.map((file) => (
        <Box key={file.id} className="containment-list-group">
          <Flex align="center" gap="2" mb="2">
            <FileIcon />
            <Heading as="h2" size="3" className="m-0">
              {file.file?.display_path ?? file.name}
            </Heading>
            <Badge color="gray">{file.children.length} elements</Badge>
          </Flex>
          <Flex direction="column" gap="1" pl="4">
            {file.children.map((child) => {
              const el = child.element;
              const selected = child.id === selectedId;
              return (
                <Box
                  key={child.id}
                  className={[
                    "rounded px-2 py-2",
                    selected
                      ? "bg-reqvire-primary text-reqvire-background"
                      : "hover:bg-reqvire-surface-muted",
                  ].join(" ")}
                >
                  <button
                    type="button"
                    onClick={() => {
                      onSelect(child.id);
                      onOpenElement(child.id);
                    }}
                    className="flex items-center gap-2 text-left"
                  >
                    <CubeIcon />
                    <Text size="2">{child.name}</Text>
                    {el && (
                      <Code className="ml-1" color="gray">
                        {el.element_type}
                      </Code>
                    )}
                  </button>
                  {el && (
                    <MarkdownContent
                      markdown={el.content}
                      sourceFilePath={el.file_path}
                      sourceAnchor={el.source_anchor}
                      variant="preview"
                    />
                  )}
                </Box>
              );
            })}
            {file.children.length === 0 && (
              <Text size="1" color="gray">
                No elements.
              </Text>
            )}
          </Flex>
        </Box>
      ))}
      {files.length === 0 && <Text color="gray">No files in store.</Text>}
    </Flex>
  );
}

function collectFiles(node: ContainmentNode): ContainmentNode[] {
  return [
    ...(node.kind === "file" ? [node] : []),
    ...node.children.flatMap(collectFiles),
  ];
}

function flattenContainment(node: ContainmentNode): ContainmentNode[] {
  return [node, ...node.children.flatMap(flattenContainment)];
}

function SunburstMode({
  root,
  onSelect,
  onOpenElement,
}: {
  root: ContainmentNode;
  selectedId: string;
  onSelect: (id: string) => void;
  onOpenElement: (id: string) => void;
}) {
  const mountRef = useRef<HTMLDivElement | null>(null);
  const [size, setSize] = useState({ width: 0, height: 0 });

  useEffect(() => {
    const element = mountRef.current;
    if (!element) return;
    const update = () => {
      setSize((current) => {
        const next = {
          width: element.clientWidth,
          height: element.clientHeight,
        };
        return current.width === next.width && current.height === next.height
          ? current
          : next;
      });
    };
    update();
    const observer =
      typeof ResizeObserver === "undefined" ? null : new ResizeObserver(update);
    observer?.observe(element);
    window.addEventListener("resize", update);
    return () => {
      observer?.disconnect();
      window.removeEventListener("resize", update);
    };
  }, []);

  useEffect(() => {
    if (!mountRef.current || size.width === 0 || size.height === 0) return;
    renderD3Sunburst(mountRef.current, toD3Containment(root), (node) => {
      const source = node.source;
      onSelect(source.id);
      if (source.kind === "element") onOpenElement(source.id);
      return source.kind === "element";
    });
    return () => {
      if (mountRef.current) mountRef.current.innerHTML = "";
    };
  }, [onOpenElement, onSelect, root, size.height, size.width]);

  return (
    <Box
      data-testid="containment-sunburst"
      ref={mountRef}
      className="h-full min-h-[520px]"
      role="img"
      aria-label="Containment sunburst visualization"
    />
  );
}

function IcicleMode({
  root,
  onSelect,
  onOpenElement,
}: {
  root: ContainmentNode;
  selectedId: string;
  onSelect: (id: string) => void;
  onOpenElement: (id: string) => void;
}) {
  const mountRef = useRef<HTMLDivElement | null>(null);
  const [size, setSize] = useState({ width: 0, height: 0 });

  useEffect(() => {
    const element = mountRef.current;
    if (!element) return;
    const update = () => {
      setSize((current) => {
        const next = {
          width: element.clientWidth,
          height: element.clientHeight,
        };
        return current.width === next.width && current.height === next.height
          ? current
          : next;
      });
    };
    update();
    const observer =
      typeof ResizeObserver === "undefined" ? null : new ResizeObserver(update);
    observer?.observe(element);
    window.addEventListener("resize", update);
    return () => {
      observer?.disconnect();
      window.removeEventListener("resize", update);
    };
  }, []);

  useEffect(() => {
    if (!mountRef.current || size.width === 0 || size.height === 0) return;
    renderD3Icicle(mountRef.current, toD3Containment(root), (node) => {
      const source = node.source;
      onSelect(source.id);
      if (source.kind === "element") onOpenElement(source.id);
      return source.kind === "element";
    });
    return () => {
      if (mountRef.current) mountRef.current.innerHTML = "";
    };
  }, [onOpenElement, onSelect, root, size.height, size.width]);

  return (
    <Box
      data-testid="containment-icicle"
      ref={mountRef}
      className="h-full min-h-[520px]"
      role="img"
      aria-label="Containment icicle visualization"
    />
  );
}

interface D3ContainmentNode {
  id: string;
  name: string;
  type: string;
  link?: string;
  source: ContainmentNode;
  children?: D3ContainmentNode[];
}

type D3HierarchyNode = d3.HierarchyRectangularNode<D3ContainmentNode> & {
  current?: D3Rect;
  target?: D3Rect;
};

interface D3Rect {
  x0: number;
  x1: number;
  y0: number;
  y1: number;
}

const D3_CONTAINMENT_COLORS: Record<string, string> = {
  folder: "#9E9E9E",
  file: "#B8860B",
  "design-document": "#607D8B",
  capability: "#BBDEFB",
  ontology: "#B08A00",
  "system-requirement": "#673AB7",
  requirement: "#673AB7",
  verification: "#4CAF50",
  "test-verification": "#4CAF50",
  refinement: "#FF9800",
  element: "#424242",
  "attachment-element": "#8D6E63",
  "attachment-file": "#8D6E63",
};

const D3_CONTAINMENT_ICONS: Record<string, string> = {
  folder: "Folder",
  file: "File",
  capability: "Capability",
  ontology: "Ontology",
  "system-requirement": "Requirement",
  requirement: "Requirement",
  verification: "Verification",
  refinement: "Refinement",
  "design-document": "Design",
  element: "Element",
  "attachment-element": "Attachment",
  "attachment-file": "Attachment",
};

function toD3Containment(node: ContainmentNode): D3ContainmentNode {
  const link =
    node.kind === "element" && node.element
      ? `${node.element.file_path.replace(/\.md$/, ".html")}#${node.element.source_anchor}`
      : node.path
        ? node.path.replace(/\.md$/, ".html")
        : undefined;
  return {
    id: node.id,
    name: node.name,
    type: d3ContainmentType(node),
    link,
    source: node,
    children: node.children.map(toD3Containment),
  };
}

function d3ContainmentType(node: ContainmentNode): string {
  if (node.kind === "root" || node.kind === "folder") return "folder";
  if (node.kind === "file") {
    return node.path?.includes("/DesignDocuments/") ? "design-document" : "file";
  }
  const type = node.element?.element_type.toLowerCase() ?? "element";
  if (type === "capability") return "capability";
  if (type === "ontology") return "ontology";
  if (type.includes("verification")) return type === "test-verification" ? type : "verification";
  if (
    type.includes("refinement") ||
    ["specification", "behavior", "constraint", "state", "input-output", "semantic-contract"].includes(
      type,
    )
  ) {
    return "refinement";
  }
  if (type === "requirement" || type === "system-requirement") return "system-requirement";
  return "element";
}

function d3ContainmentColor(type: string): string {
  return D3_CONTAINMENT_COLORS[type] || "#9E9E9E";
}

function d3ContainmentIcon(type: string): string {
  return D3_CONTAINMENT_ICONS[type] || "Element";
}

function renderD3Sunburst(
  mount: HTMLDivElement,
  data: D3ContainmentNode,
  onNodeClick: (node: D3ContainmentNode) => boolean,
) {
  mount.innerHTML = "";
  const width = Math.max(320, mount.clientWidth);
  const height = Math.max(520, mount.clientHeight);
  const breadcrumbReserve = 46;
  const edgeMargin = 14;
  const leftControlReserve = width >= 760 ? 244 : 0;
  const usableWidth = Math.max(
    280,
    width - leftControlReserve - edgeMargin * 2,
  );
  const usableHeight = Math.max(320, height - breadcrumbReserve - edgeMargin);
  const diameter = Math.max(280, Math.min(usableWidth, usableHeight) * 0.98);
  const radius = diameter / 2;
  const centerX = leftControlReserve + edgeMargin + usableWidth / 2;
  const centerY = usableHeight / 2;
  const container = d3
    .select(mount)
    .append("div")
    .attr("class", "d3-sunburst-container");
  const breadcrumb = container.append("div").attr("class", "d3-sunburst-breadcrumb");
  breadcrumb
    .style("margin-left", `${leftControlReserve + edgeMargin}px`)
    .style("width", `${usableWidth}px`);
  const wrapper = container.append("div").attr("class", "d3-sunburst-wrapper");
  const svg = wrapper.append("svg").attr("class", "d3-sunburst-svg");

  svg
    .attr("width", width)
    .attr("height", usableHeight)
    .attr("viewBox", `0 0 ${width} ${usableHeight}`)
    .style("font", "12px system-ui, sans-serif")
    .style("display", "block");

  const viewport = svg
    .append("g")
    .attr("transform", `translate(${centerX},${centerY})`);
  const root = d3
    .hierarchy(data)
    .sum((item) => (item.children?.length ? 0 : 1))
    .sort((a, b) => (b.value ?? 0) - (a.value ?? 0)) as D3HierarchyNode;

  d3.partition<D3ContainmentNode>().size([2 * Math.PI, radius])(root);

  const arc = d3
    .arc<D3Rect>()
    .startAngle((node) => node.x0)
    .endAngle((node) => node.x1)
    .padAngle((node) => Math.min((node.x1 - node.x0) / 2, 0.005))
    .padRadius(radius / 2)
    .innerRadius((node) => node.y0)
    .outerRadius((node) => node.y1 - 1);

  let currentFocus = root;
  let breadcrumbAncestors: D3HierarchyNode[] = [];

  function updateBreadcrumbDisplay(node: D3HierarchyNode, isHover = false) {
    const ancestors = node.ancestors().reverse() as D3HierarchyNode[];
    breadcrumbAncestors = ancestors;
    breadcrumb.html(
      ancestors
        .map((ancestor, index) => {
          const isLast = index === ancestors.length - 1;
          const canClick = !isHover && !isLast && node === currentFocus;
          const style = isLast
            ? `color: ${d3ContainmentColor(ancestor.data.type)}; font-weight: bold;`
            : canClick
              ? `color: ${d3ContainmentColor(ancestor.data.type)}; cursor: pointer; text-decoration: underline;`
              : `color: ${d3ContainmentColor(ancestor.data.type)};`;
          return `<span class="breadcrumb-item" data-index="${index}" style="${style}">${d3ContainmentIcon(ancestor.data.type)} ${escapeHtml(ancestor.data.name.replace(/\.html$/, ".md"))}</span>`;
        })
        .join(" -> "),
    );
    if (!isHover) {
      breadcrumb.selectAll<HTMLSpanElement, unknown>(".breadcrumb-item").on("click", function () {
        const index = Number(d3.select(this).attr("data-index"));
        const targetNode = breadcrumbAncestors[index];
        if (targetNode && targetNode !== currentFocus) clicked(null, targetNode);
      });
    }
  }

  const path = viewport
    .append("g")
    .selectAll<SVGPathElement, D3HierarchyNode>("path")
    .data((root.descendants() as D3HierarchyNode[]).filter((node) => node.depth > 0))
    .join("path")
    .attr("fill", (node) => d3ContainmentColor(node.data.type))
    .attr("fill-opacity", (node) => (node.children ? 0.8 : 0.6))
    .attr("d", (node) => arc(node.current ?? node) ?? "")
    .style("cursor", "pointer")
    .on("click", clicked)
    .on("mouseover", function (_event, node) {
      d3.select(this).attr("fill-opacity", 1);
      updateBreadcrumbDisplay(node, true);
    })
    .on("mouseout", function (_event, node) {
      d3.select(this).attr("fill-opacity", node.children ? 0.8 : 0.6);
      updateBreadcrumbDisplay(currentFocus, false);
    });

  path.append("title").text((node) => {
    const nodePath = node.ancestors().map((ancestor) => ancestor.data.name).reverse().join(" / ");
    return node.data.link ? `${nodePath}\n${node.data.link}` : nodePath;
  });

  const label = viewport
    .append("g")
    .attr("pointer-events", "none")
    .attr("text-anchor", "middle")
    .selectAll<SVGTextElement, D3HierarchyNode>("text")
    .data((root.descendants() as D3HierarchyNode[]).filter((node) => node.depth > 0 && (node.y0 + node.y1) / 2 * (node.x1 - node.x0) > 10))
    .join("text")
    .attr("transform", (node) => labelTransform(node))
    .attr("dy", "0.35em")
    .style("font-size", "10px")
    .style("fill", "#fff")
    .text((node) => (node.data.name.length > 12 ? `${node.data.name.substring(0, 10)}...` : node.data.name));

  const parent = viewport
    .append("circle")
    .datum(root)
    .attr("r", radius / 6)
    .attr("fill", "#FAFAFA")
    .attr("stroke", "#EEEEEE")
    .attr("stroke-width", 2)
    .attr("pointer-events", "all")
    .style("cursor", "pointer")
    .on("click", clicked);

  const centerText = viewport
    .append("text")
    .attr("text-anchor", "middle")
    .attr("dy", "0.35em")
    .style("font-size", "14px")
    .style("fill", "#424242")
    .style("cursor", "default")
    .text(data.name);

  function updateCenterLink(node: D3HierarchyNode) {
    currentFocus = node;
    centerText
      .text(node.data.name)
      .style("cursor", node.data.link ? "pointer" : "default")
      .style("fill", node.data.link ? "var(--color-link-hover, #3F51B5)" : "#424242")
      .style("text-decoration", node.data.link ? "underline" : "none");
    updateBreadcrumbDisplay(node, false);
  }

  updateBreadcrumbDisplay(root, false);

  centerText.on("click", (event) => {
    event.stopPropagation();
    onNodeClick(currentFocus.data);
  });

  function clicked(_event: MouseEvent | null, focus: D3HierarchyNode) {
    if (!focus.children && onNodeClick(focus.data)) return;
    parent.datum(focus.parent || root);
    updateCenterLink(focus);
    root.each((node) => {
      const rect = {
        x0: Math.max(0, Math.min(1, (node.x0 - focus.x0) / (focus.x1 - focus.x0))) * 2 * Math.PI,
        x1: Math.max(0, Math.min(1, (node.x1 - focus.x0) / (focus.x1 - focus.x0))) * 2 * Math.PI,
        y0: Math.max(0, node.y0 - focus.y0),
        y1: Math.max(0, node.y1 - focus.y0),
      };
      (node as D3HierarchyNode).target = rect;
    });
    const transitionName = `sunburst-${Date.now()}`;
    svg.transition(transitionName).duration(750);
    path
      .transition(transitionName)
      .duration(750)
      .tween("data", (node) => {
        const interpolation = d3.interpolate(node.current ?? node, node.target ?? node);
        return (t) => {
          node.current = interpolation(t);
        };
      })
      .filter(function (node) {
        return Boolean(Number(this.getAttribute("fill-opacity")) || arcVisible(node.target ?? node));
      })
      .attr("fill-opacity", (node) => (arcVisible(node.target ?? node) ? (node.children ? 0.8 : 0.6) : 0))
      .attr("pointer-events", (node) => (arcVisible(node.target ?? node) ? "auto" : "none"))
      .attrTween("d", (node) => () => arc(node.current ?? node) ?? "");
    label
      .filter(function (node) {
        return Boolean(Number(this.getAttribute("fill-opacity")) || labelVisible(node.target ?? node));
      })
      .transition(transitionName)
      .duration(750)
      .attr("fill-opacity", (node) => Number(labelVisible(node.target ?? node)))
      .attrTween("transform", (node) => () => labelTransform(node.current ?? node));
  }

  function arcVisible(rect: D3Rect) {
    return rect.y1 <= radius && rect.y0 >= 0 && rect.x1 > rect.x0;
  }

  function labelVisible(rect: D3Rect) {
    return rect.y1 <= radius && rect.y0 >= 0 && (rect.y0 + rect.y1) / 2 * (rect.x1 - rect.x0) > 10;
  }

  function labelTransform(rect: D3Rect) {
    const x = ((rect.x0 + rect.x1) / 2) * 180 / Math.PI;
    const y = (rect.y0 + rect.y1) / 2;
    return `rotate(${x - 90}) translate(${y},0) rotate(${x < 180 ? 0 : 180})`;
  }

  root.each((node) => {
    (node as D3HierarchyNode).current = node;
  });
}

function renderD3Icicle(
  mount: HTMLDivElement,
  data: D3ContainmentNode,
  onNodeClick: (node: D3ContainmentNode) => boolean,
) {
  mount.innerHTML = "";
  const width = Math.max(320, mount.clientWidth);
  const height = Math.max(520, mount.clientHeight);
  const breadcrumbReserve = 46;
  const edgeMargin = 14;
  const leftControlReserve = width >= 760 ? 244 : 0;
  const chartWidth = Math.max(
    320,
    width - leftControlReserve - edgeMargin * 2,
  );
  const chartHeight = Math.max(320, height - breadcrumbReserve - edgeMargin);
  const container = d3.select(mount).append("div").attr("class", "d3-icicle-container");
  const breadcrumb = container.append("div").attr("class", "d3-icicle-breadcrumb");
  breadcrumb
    .style("margin-left", `${leftControlReserve + edgeMargin}px`)
    .style("width", `${chartWidth}px`);
  const wrapper = container.append("div").attr("class", "d3-icicle-wrapper");
  const svg = wrapper.append("svg").attr("class", "d3-icicle-svg");

  svg
    .attr("width", width)
    .attr("height", chartHeight)
    .attr("viewBox", `0 0 ${width} ${chartHeight}`)
    .style("font", "11px system-ui, sans-serif")
    .style("display", "block");

  const viewport = svg
    .append("g")
    .attr("transform", `translate(${leftControlReserve + edgeMargin},0)`);
  const root = d3
    .hierarchy(data)
    .sum((item) => (item.children?.length ? 0 : 1))
    .sort((a, b) => b.height - a.height || a.data.name.localeCompare(b.data.name)) as D3HierarchyNode;

  d3.partition<D3ContainmentNode>().size([chartHeight, chartWidth]).padding(1)(root);

  let currentFocus = root;
  let breadcrumbAncestors: D3HierarchyNode[] = [];

  const cell = viewport
    .selectAll<SVGGElement, D3HierarchyNode>("g")
    .data(root.descendants() as D3HierarchyNode[])
    .join("g")
    .attr("transform", (node) => `translate(${node.y0},${node.x0})`);

  const rect = cell
    .append("rect")
    .attr("width", (node) => rectWidth(node))
    .attr("height", (node) => rectHeight(node))
    .attr("fill", (node) => d3ContainmentColor(node.data.type))
    .attr("fill-opacity", (node) => (node.children ? 0.8 : 0.6))
    .style("cursor", "pointer")
    .on("click", clicked)
    .on("mouseover", function (_event, node) {
      d3.select(this).attr("fill-opacity", 1);
      const ancestors = node.ancestors().reverse() as D3HierarchyNode[];
      breadcrumb.html(
        ancestors
          .map((ancestor) => `<span style="color: ${d3ContainmentColor(ancestor.data.type)}">${escapeHtml(ancestor.data.name.replace(/\.html$/, ".md"))}</span>`)
          .join(" -> "),
      );
    })
    .on("mouseout", function (_event, node) {
      d3.select(this).attr("fill-opacity", node.children ? 0.8 : 0.6);
      updateBreadcrumb(currentFocus);
    });

  rect.append("title").text((node) => (node.data.link ? `${node.data.name}\n${node.data.link}` : node.data.name));

  const text = cell
    .append("text")
    .attr("pointer-events", "none")
    .attr("x", 4)
    .attr("y", (node) => Math.min(rectHeight(node) / 2 + 4, 14))
    .attr("fill", "#fff")
    .attr("fill-opacity", (node) => (labelVisible(node) ? 1 : 0))
    .text((node) => node.data.name);

  const centerLink = viewport
    .append("text")
    .attr("class", "center-link")
    .attr("text-anchor", "start")
    .attr("x", 10)
    .attr("y", 20)
    .style("font-size", "14px")
    .style("font-weight", "bold")
    .style("fill", "var(--color-link-hover, #3F51B5)")
    .style("cursor", "pointer")
    .style("text-decoration", "underline")
    .style("display", "none")
    .on("click", () => {
      onNodeClick(currentFocus.data);
    });

  function updateBreadcrumb(node: D3HierarchyNode) {
    const ancestors = node.ancestors().reverse() as D3HierarchyNode[];
    breadcrumbAncestors = ancestors;
    breadcrumb.html(
      ancestors
        .map((ancestor, index) => {
          const isLast = index === ancestors.length - 1;
          const style = isLast
            ? `color: ${d3ContainmentColor(ancestor.data.type)}; font-weight: bold;`
            : `color: ${d3ContainmentColor(ancestor.data.type)}; cursor: pointer; text-decoration: underline;`;
          return `<span class="breadcrumb-item" data-index="${index}" style="${style}">${escapeHtml(ancestor.data.name.replace(/\.html$/, ".md"))}</span>`;
        })
        .join(" -> "),
    );
    breadcrumb.selectAll<HTMLSpanElement, unknown>(".breadcrumb-item").on("click", function () {
      const index = Number(d3.select(this).attr("data-index"));
      const targetNode = breadcrumbAncestors[index];
      if (targetNode && targetNode !== currentFocus) zoomTo(targetNode);
    });
    if (node.data.link && node !== root) {
      centerLink.text(`-> ${node.data.name.replace(/\.html$/, ".md")}`).style("display", "block");
    } else {
      centerLink.style("display", "none");
    }
  }

  function zoomTo(target: D3HierarchyNode) {
    currentFocus = target;
    updateBreadcrumb(currentFocus);
    root.each((node) => {
      node.target = {
        x0: ((node.x0 - currentFocus.x0) / (currentFocus.x1 - currentFocus.x0)) * chartHeight,
        x1: ((node.x1 - currentFocus.x0) / (currentFocus.x1 - currentFocus.x0)) * chartHeight,
        y0: node.y0 - currentFocus.y0,
        y1: node.y1 - currentFocus.y0,
      };
    });
    const transitionName = `icicle-${Date.now()}`;
    svg.transition(transitionName).duration(750);
    cell
      .transition(transitionName)
      .duration(750)
      .attr(
        "transform",
        (node) => `translate(${node.target?.y0 ?? node.y0},${node.target?.x0 ?? node.x0})`,
      );
    rect
      .transition(transitionName)
      .duration(750)
      .attr("width", (node) => rectWidth(node.target ?? node))
      .attr("height", (node) => rectHeight(node.target ?? node));
    text
      .transition(transitionName)
      .duration(750)
      .attr("fill-opacity", (node) => (labelVisible(node.target ?? node) ? 1 : 0))
      .attr("y", (node) => Math.min(rectHeight(node.target ?? node) / 2 + 4, 14));
  }

  function clicked(_event: MouseEvent, node: D3HierarchyNode) {
    if (!node.children && onNodeClick(node.data)) return;
    zoomTo(currentFocus === node ? node.parent || root : node);
  }

  function rectWidth(rectLike: D3Rect) {
    return Math.max(0, rectLike.y1 - rectLike.y0 - 1);
  }

  function rectHeight(rectLike: D3Rect) {
    return Math.max(0, rectLike.x1 - rectLike.x0 - 1);
  }

  function labelVisible(rectLike: D3Rect) {
    return rectLike.y1 <= chartWidth && rectLike.y0 >= 0 && rectLike.x1 - rectLike.x0 > 16;
  }

  root.each((node) => {
    node.target = { x0: node.x0, x1: node.x1, y0: node.y0, y1: node.y1 };
  });
  updateBreadcrumb(root);
}

function escapeHtml(value: string) {
  return value.replace(/[&<>"']/g, (char) => {
    switch (char) {
      case "&":
        return "&amp;";
      case "<":
        return "&lt;";
      case ">":
        return "&gt;";
      case '"':
        return "&quot;";
      default:
        return "&#39;";
    }
  });
}

function Inspector({
  selected,
  totals,
  query,
  searchResults,
  onQueryChange,
  onSelect,
  onOpenElement,
}: {
  selected: ContainmentNode;
  totals: { folders: number; files: number; elements: number; nodes: number };
  query: string;
  searchResults: ContainmentNode[];
  onQueryChange: (value: string) => void;
  onSelect: (id: string) => void;
  onOpenElement: (id: string) => void;
}) {
  const path = breadcrumbs(selected);
  return (
    <Box className="graph-sidebar">
      <div className="graph-search-panel">
        <TextField.Root
          aria-label="Search containment"
          placeholder="Search folders, files, elements"
          value={query}
          onChange={(event) => onQueryChange(event.target.value)}
        >
          <TextField.Slot>
            <MagnifyingGlassIcon />
          </TextField.Slot>
        </TextField.Root>
        {searchResults.length > 0 && (
          <ul className="graph-results">
            {searchResults.map((node) => (
              <li key={node.id}>
                <button
                  type="button"
                  onClick={() => {
                    onSelect(node.id);
                    if (node.kind === "element") onOpenElement(node.id);
                  }}
                >
                  <span
                    className="graph-result-swatch"
                    style={{ backgroundColor: KIND_COLORS[node.kind] }}
                  />
                  <span>{node.name}</span>
                </button>
              </li>
            ))}
          </ul>
        )}
      </div>
      <div className="graph-inspector-header">
        <Heading as="h2" size="3">
          Selection
        </Heading>
      </div>
      <div className="graph-inspector-body">
        <Flex direction="column" gap="3">
          <Box>
            <Heading as="h2" size="3" mb="2">
              {selected.name}
            </Heading>
            <Flex gap="2" wrap="wrap">
              <Badge color="gray">{labelForKind(selected.kind)}</Badge>
              {selected.element && <Code>{selected.element.element_type}</Code>}
              {selected.kind !== "element" && (
                <Badge color="gray">{elementCount(selected)} elements</Badge>
              )}
            </Flex>
          </Box>
          <Flex direction="column" gap="1">
            {path.map((part, index) => (
              <Text key={`${part}-${index}`} size="1" color="gray">
                {part}
              </Text>
            ))}
          </Flex>
          {selected.element && (
            <>
              <button
                type="button"
                onClick={() => onOpenElement(selected.id)}
                className="explorer-command"
              >
                Open element detail
              </button>
              <MarkdownContent
                markdown={selected.element.content}
                sourceFilePath={selected.element.file_path}
                sourceAnchor={selected.element.source_anchor}
                variant="preview"
              />
            </>
          )}
          {!selected.element && (
            <Text size="2" color="gray">
              Contains {selected.children.length} direct children and{" "}
              {elementCount(selected)} elements.
            </Text>
          )}
          <Flex direction="column" gap="1">
            {Object.entries(selected.element?.governance ?? {}).map(([key, value]) => (
              <Text key={key} size="1">
                <strong>{key}</strong> {value}
              </Text>
            ))}
          </Flex>
        </Flex>
      </div>
      <div className="graph-summary-strip">
        <span>
          Folders <strong>{totals.folders}</strong>
        </span>
        <span>
          Files <strong>{totals.files}</strong>
        </span>
        <span>
          Elements <strong>{totals.elements}</strong>
        </span>
        <span>
          Nodes <strong>{totals.nodes}</strong>
        </span>
      </div>
    </Box>
  );
}
