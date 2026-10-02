import type { ElementFlowData } from "@ds";
import type { ExplorerProjectStore } from "../store/types";
import { routeForContent, routeForElement, routeForResource } from "../router/routes";

const DOWNWARD_RELATIONS: Record<string, string> = {
  derivedFrom: "derive", specify: "specifiedBy", verify: "verifiedBy", define: "definedBy",
};

/** Adapt normalized store facts without evaluating or inventing model relations. */
export function buildModelFlow(store: ExplorerProjectStore, selection: string): ElementFlowData {
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
  const addEdge = (source: string, target: string, label: string) => {
    if (!nodes.has(source) || !nodes.has(target)) return;
    const id = JSON.stringify([source, target, label]);
    edges.set(id, { id, source, target, label });
  };
  for (const relation of store.relations) {
    // The store has already canonicalized direction and merged inverse provenance.
    const target = nodes.has(relation.target_id) ? relation.target_id : relation.resource_id ?? relation.target_id;
    const type = relation.canonical_relation_type || relation.relation_type;
    if (DOWNWARD_RELATIONS[type]) addEdge(target, relation.source_id, DOWNWARD_RELATIONS[type]);
    else addEdge(relation.source_id, target, type);
  }
  for (const binding of store.contract_bindings) addEdge(binding.source_id, binding.resource_id ?? binding.target, "Contract binding");
  for (const reference of store.contract_references ?? []) addEdge(reference.source_id, reference.resource_id ?? reference.target, "Contract reference");
  for (const reference of store.concept_refs) addEdge(reference.source_id, reference.target_element_id, "Concept reference");

  let visible = new Set<string>();
  let title = store.project.root_label;
  if (selection === "__root__") {
    // The project overview grows from capabilities, never from verification leaves.
    const queue = [...nodes].filter(([, node]) => node.type === "capability").map(([id]) => id);
    const outgoing = new Map<string, string[]>();
    for (const edge of edges.values()) {
      const targets = outgoing.get(edge.source) ?? [];
      targets.push(edge.target);
      outgoing.set(edge.source, targets);
    }
    for (let index = 0; index < queue.length; index++) {
      const id = queue[index];
      if (visible.has(id)) continue;
      visible.add(id);
      queue.push(...outgoing.get(id) ?? []);
    }
  } else if (selection.startsWith("folder:") || selection.startsWith("file:") || selection.startsWith("resource-folder:") || selection === "resource-root") {
    const folder = selection.startsWith("folder:");
    const path = selection.slice(selection.indexOf(":") + 1);
    const resources = selection.startsWith("resource-");
    title = selection === "resource-root" ? "Resources" : path;
    const contained = new Set([...nodes].filter(([, node]) => folder
      ? node.element.file.startsWith(`${path}/`) : resources
        ? node.type === "resource" && (selection === "resource-root" || node.element.file.startsWith(`${path}/`))
        : node.element.file === path).map(([id]) => id));
    visible = new Set(contained);
    for (const edge of edges.values()) {
      if (contained.has(edge.source) || contained.has(edge.target)) {
        visible.add(edge.source);
        visible.add(edge.target);
      }
    }
    const parents = new Map<string, string[]>();
    for (const edge of edges.values()) {
      if (edge.label !== "derive" && edge.label !== "specifiedBy") continue;
      const ids = parents.get(edge.target) ?? [];
      ids.push(edge.source);
      parents.set(edge.target, ids);
    }
    const ancestors = [...visible];
    for (let index = 0; index < ancestors.length; index++) {
      for (const parent of parents.get(ancestors[index]) ?? []) {
        if (visible.has(parent)) continue;
        visible.add(parent);
        ancestors.push(parent);
      }
    }
  } else {
    visible = new Set(nodes.has(selection) ? [selection] : []);
    title = nodes.get(selection)?.element.name ?? "Selected element";
    // Traverse each direction separately so sibling-only branches stay outside this scope.
    for (const reverse of [false, true]) {
      const adjacency = new Map<string, string[]>();
      for (const edge of edges.values()) {
        const from = reverse ? edge.target : edge.source;
        const to = reverse ? edge.source : edge.target;
        const targets = adjacency.get(from) ?? [];
        targets.push(to);
        adjacency.set(from, targets);
      }
      const visited = new Set<string>();
      const queue = [selection];
      for (let index = 0; index < queue.length; index++) {
        const id = queue[index];
        if (visited.has(id)) continue;
        visited.add(id);
        for (const next of adjacency.get(id) ?? []) { visible.add(next); queue.push(next); }
      }
    }
  }
  const visibleEdges = [...edges.values()].filter(edge => visible.has(edge.source) && visible.has(edge.target));
  const childCapabilities = new Set(visibleEdges.filter(edge => edge.label === "derive" && nodes.get(edge.source)?.type === "capability").map(edge => edge.target));
  return { id: selection, title,
    nodes: [...nodes].filter(([id]) => visible.has(id)).map(([id, node]) => ({ ...node, root: node.type === "capability" && !childCapabilities.has(id) })),
    edges: visibleEdges,
  };
}
