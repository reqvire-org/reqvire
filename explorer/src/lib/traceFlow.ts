import type { TraceFlowData, TraceFlowElement } from "@ds";
import type { ProjectStoreElement } from "../store/types";
import type { TraceVerificationNode } from "./traces";
import { routeForContent, routeForElement } from "../router/routes";

/** Adapt evaluated trace facts; ancestry and capability membership stay in core. */
export function buildVerificationFlow(
  verification: TraceVerificationNode,
  elements: ReadonlyMap<string, ProjectStoreElement>,
): TraceFlowData {
  const element = (id: string, name: string, fallbackFile = id.split("#")[0]): TraceFlowElement => {
    const file = elements.get(id)?.file_path ?? fallbackFile;
    return { id, name, file, href: routeForElement(id), sourceHref: routeForContent(file) };
  };
  const root = { ...element(verification.id, verification.name, verification.file),
    type: verification.verificationType ?? "verification" };
  const publishedNodes = verification.traceGraph?.nodes ?? verification.requirementIds.map(id => ({
    id, name: elements.get(id)?.name ?? id,
    type: elements.get(id)?.element_type ?? "requirement", is_directly_verified: true,
  }));
  const nodes = new Map<string, NonNullable<TraceFlowData["graph"]>["nodes"][number]>([
    [root.id, { element: root, type: root.type, context: "Verification" }],
  ]);
  const requirements: TraceFlowData["requirements"][number][] = [];
  const edges = new Map<string, NonNullable<TraceFlowData["graph"]>["edges"][number]>();
  const addEdge = (source: string, target: string, label: string) => {
    const id = JSON.stringify([source, target, label]);
    edges.set(id, { id, source, target, label });
  };
  for (const node of publishedNodes) {
    const value = element(node.id, node.name);
    const requirement = node.type === "requirement" || node.type.endsWith("-requirement");
    nodes.set(node.id, { element: value, type: node.type, context: node.is_directly_verified
      ? "Directly verified" : requirement ? "Requirement ancestor" : "Capability context" });
    if (requirement) requirements.push({ ...value, directlyVerified: node.is_directly_verified, parentIds: [] });
    if (node.is_directly_verified) addEdge(root.id, node.id, "verifies");
  }
  for (const edge of verification.traceGraph?.edges ?? []) {
    if (nodes.has(edge.source) && nodes.has(edge.target)) addEdge(edge.source, edge.target, edge.relation_type);
  }
  return { verification: root, requirements, graph: { nodes: [...nodes.values()], edges: [...edges.values()] } };
}
