import type {
  ExplorerProjectStore,
  ProjectStoreElement,
  TraceTree,
} from "../store/types";

export interface TraceVerificationNode {
  id: string;
  name: string;
  file: string;
  directCount: number;
  totalCount: number;
  requirementIds: string[];
  traceTree?: TraceTree;
  verificationType?: string;
}

export interface TraceFileNode {
  file: string;
  verifications: TraceVerificationNode[];
}

export function isVerification(element: ProjectStoreElement): boolean {
  return (
    element.type_family === "verification" ||
    element.element_type.toLowerCase().includes("verification")
  );
}

export function buildTraceFiles(store: ExplorerProjectStore): TraceFileNode[] {
  const files = store.traces?.files ?? {};
  const entries = Object.entries(files).sort((a, b) => a[0].localeCompare(b[0]));
  if (entries.length > 0) {
    return entries.map(([file, entry]) => ({
      file,
      verifications: (entry.verifications ?? []).map((verification) => ({
        id: verification.identifier,
        name: verification.name,
        file: verification.file || file,
        directCount: verification.directly_verified_count ?? 0,
        totalCount: verification.total_requirements_in_tree ?? 0,
        requirementIds: verification.directly_verified_requirements ?? [],
        traceTree: verification.trace_tree,
        verificationType: verification.type,
      })),
    }));
  }

  const elementById = new Map(store.elements.map((element) => [element.id, element]));
  const requirementIdsByVerification = new Map<string, string[]>();
  for (const relation of store.relations) {
    const relationType =
      relation.canonical_relation_type || relation.relation_type || "";
    if (!relationType.toLowerCase().includes("verify")) continue;
    const source = elementById.get(relation.source_id);
    const target = elementById.get(relation.target_id);
    if (!source || !target || !isVerification(source)) continue;
    const list = requirementIdsByVerification.get(source.id) ?? [];
    list.push(target.id);
    requirementIdsByVerification.set(source.id, list);
  }

  const byFile = new Map<string, TraceVerificationNode[]>();
  for (const element of store.elements.filter(isVerification)) {
    const requirementIds = requirementIdsByVerification.get(element.id) ?? [];
    const list = byFile.get(element.file_path) ?? [];
    list.push({
      id: element.id,
      name: element.name,
      file: element.file_path,
      directCount: requirementIds.length,
      totalCount: requirementIds.length,
      requirementIds,
      verificationType: element.element_type,
    });
    byFile.set(element.file_path, list);
  }

  return [...byFile.entries()]
    .sort((a, b) => a[0].localeCompare(b[0]))
    .map(([file, verifications]) => ({ file, verifications }));
}
