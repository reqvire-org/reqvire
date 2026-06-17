export type SearchKind = "file" | "element" | "resource" | "ontology";

export const SEARCH_KINDS = ["file", "element", "resource", "ontology"] as const satisfies readonly SearchKind[];
