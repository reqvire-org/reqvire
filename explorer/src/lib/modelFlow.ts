import type { ElementFlowData } from "@ds";
import type { ExplorerProjectStore } from "../store/types";
import { routeForContent, routeForElement, routeForResource } from "../router/routes";

type FlowInputs = Pick<ExplorerProjectStore,
  "elements" | "resources" | "relations" | "contract_bindings" | "contract_references" | "concept_refs">;

export type ModelFlowScope = "downstream" | "upstream";

export interface ModelFlowIndex {
  readonly nodes: ReadonlyMap<string, ElementFlowData["nodes"][number]>;
  readonly edges: readonly ElementFlowData["edges"][number][];
  readonly outgoing: ReadonlyMap<string, readonly string[]>;
  readonly incoming: ReadonlyMap<string, readonly string[]>;
  readonly inverseLabels: ReadonlyMap<string, string>;
}

/** Prepare normalized facts once; selection never changes this topology. */
export function prepareModelFlow(store: FlowInputs): ModelFlowIndex {
  const nodes = new Map<string, ElementFlowData["nodes"][number]>();
  for (const element of store.elements) {
    nodes.set(element.id, { type: element.element_type, context: element.element_type.replaceAll("-", " "),
      element: { id: element.id, name: element.name, file: element.file_path,
        href: routeForElement(element.id),
        sourceHref: element.source_anchor.startsWith("#/content/") ? element.source_anchor : routeForContent(element.file_path),
      },
    });
  }
  for (const resource of store.resources) {
    nodes.set(resource.id, { type: "resource", context: resource.kind.replaceAll("-", " "),
      element: { id: resource.id, name: resource.display, file: resource.file_path ?? resource.target,
        href: routeForResource(resource.id), sourceHref: routeForResource(resource.id) },
    });
  }
  const edges = new Map<string, ElementFlowData["edges"][number]>();
  const inverseLabels = new Map<string, string>();
  const addEdge = (source: string, target: string, label: string, inverse?: string | null) => {
    if (!nodes.has(source) || !nodes.has(target)) return;
    const id = JSON.stringify([source, target, label]);
    edges.set(id, { id, source, target, label });
    if (inverse) inverseLabels.set(id, inverse);
  };
  for (const relation of store.relations) {
    // The store has already canonicalized direction and merged inverse provenance.
    const target = nodes.has(relation.target_id) ? relation.target_id : relation.resource_id ?? relation.target_id;
    const type = relation.canonical_relation_type || relation.relation_type;
    addEdge(relation.source_id, target, type, relation.inverse_relation_type);
  }
  for (const binding of store.contract_bindings) addEdge(binding.source_id, binding.resource_id ?? binding.target, "Contract binding");
  for (const reference of store.contract_references ?? []) addEdge(reference.source_id, reference.resource_id ?? reference.target, "Contract reference");
  for (const reference of store.concept_refs) addEdge(reference.source_id, reference.target_element_id, "Concept reference");

  const outgoing = new Map<string, string[]>();
  const incoming = new Map<string, string[]>();
  const append = (adjacency: Map<string, string[]>, from: string, to: string) => {
    const targets = adjacency.get(from) ?? [];
    targets.push(to);
    adjacency.set(from, targets);
  };
  for (const edge of edges.values()) {
    append(outgoing, edge.source, edge.target);
    append(incoming, edge.target, edge.source);
  }
  return { nodes, edges: [...edges.values()], outgoing, incoming, inverseLabels };
}

/** Follow one horizon from every selected element without crossing into sibling paths. */
export function selectModelFlow(index: ModelFlowIndex, selection: string, rootLabel: string, scope: ModelFlowScope = "downstream"): ElementFlowData {
  const { nodes, edges, outgoing, incoming, inverseLabels } = index;
  let seeds: string[];
  let title = rootLabel;
  if (selection === "__root__") {
    // The project overview grows from capabilities, never from verification leaves.
    seeds = [...nodes].filter(([, node]) => node.type === "capability").map(([id]) => id);
  } else if (selection.startsWith("folder:") || selection.startsWith("file:") || selection.startsWith("resource-folder:") || selection === "resource-root") {
    const folder = selection.startsWith("folder:");
    const path = selection.slice(selection.indexOf(":") + 1);
    const resources = selection.startsWith("resource-");
    title = selection === "resource-root" ? "Resources" : path;
    seeds = [...nodes].filter(([, node]) => folder
      ? node.element.file.startsWith(`${path}/`) : resources
        ? node.type === "resource" && (selection === "resource-root" || node.element.file.startsWith(`${path}/`))
        : node.element.file === path).map(([id]) => id);
  } else {
    seeds = nodes.has(selection) ? [selection] : [];
    title = nodes.get(selection)?.element.name ?? "Selected element";
  }
  const adjacency = selection === "__root__" || scope === "downstream" ? outgoing : incoming;
  const visible = new Set<string>();
  const queue = [...seeds];
  for (let index = 0; index < queue.length; index++) {
    const id = queue[index];
    if (visible.has(id)) continue;
    visible.add(id);
    queue.push(...adjacency.get(id) ?? []);
  }
  const visibleEdges = edges.filter(edge => visible.has(edge.source) && visible.has(edge.target));
  const childCapabilities = new Set(visibleEdges.filter(edge => nodes.get(edge.source)?.type === "capability" && nodes.get(edge.target)?.type === "capability").map(edge => edge.target));
  return { id: selection, title,
    nodes: [...nodes].filter(([id]) => visible.has(id)).map(([id, node]) => ({ ...node, root: node.type === "capability" && !childCapabilities.has(id) })),
    edges: selection === "__root__" || scope === "downstream" ? visibleEdges : visibleEdges.map(edge => {
      const inverse = inverseLabels.get(edge.id);
      return inverse ? { ...edge, label: inverse, reversed: true } : edge;
    }),
  };
}

/** Uncached adapter for callers that do not retain a snapshot's prepared topology. */
export function buildModelFlow(store: ExplorerProjectStore, selection: string, scope: ModelFlowScope = "downstream"): ElementFlowData {
  return selectModelFlow(prepareModelFlow(store), selection, store.project.root_label, scope);
}
