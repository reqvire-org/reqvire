/**
 * ExplorerFixtureApp — renders the real App shell, router, and providers with fixture data.
 * Traces and Model use the production views with normalized fixture projections.
 * Fixture contexts replace HTTP loading while retaining the application's providers and navigation.
 */
import { useEffect, useState } from "preact/hooks";
import { ExplorerApplication } from "../../App";
import { devFixture } from "../../store/devFixture";
import type { ExplorerProjectStore } from "../../store/types";
import scopedCoverage from "../../store/fixtures/scopedCoverage.json";
import { TRACE_CAPABILITY, TRACE_EXAMPLES, traceExampleElements, traceExampleSource } from "../../../design-system/patterns/reports/fixtures/traces.ads.fixture";
import { SHOWCASE_WORKTREES } from "../../../design-system/composition/Workspace/Workspace.ads.fixture";
import { worktreeUrl } from "../../router/routes";
import { writeExplorerUrl } from "../../router/location";

/** Feed the same normalized records to the preview and the application's detail workflows. */
function withTraceElements(base: ExplorerProjectStore): ExplorerProjectStore {
  const elements = [...base.elements];
  const relations = [...base.relations];
  const files = new Map(base.files.map(file => [file.path, { ...file, element_ids: [...file.element_ids] }]));
  const folders = new Map(base.folders.map(folder => [folder.path, { ...folder, children: [...folder.children] }]));
  const ensureFolder = (path: string): void => {
    if (folders.has(path)) return;
    const parent = path.includes("/") ? path.slice(0, path.lastIndexOf("/")) : null;
    folders.set(path, { path, parent, children: [] });
    if (parent) { ensureFolder(parent); folders.get(parent)!.children.push(path); }
  };
  const capability = TRACE_CAPABILITY;
  const capabilityFolder = capability.file.slice(0, capability.file.lastIndexOf("/"));
  ensureFolder(capabilityFolder);
  folders.get(capabilityFolder)!.children.push(capability.file);
  files.set(capability.file, { path: capability.file, display_path: capability.file, parent_folder: capabilityFolder,
    element_ids: [capability.id], resource_ids: [],
    markdown_content: `# Capabilities\n\n### ${capability.name}\n\n${capability.description}\n\n#### Metadata\n  * type: capability\n`,
  });
  elements.push({ id: capability.id, name: capability.name, element_type: "capability", type_family: "capability",
    file_path: capability.file, line_number: 3, source_anchor: `#/content/${capability.file}#${capability.id}`,
    content: capability.description, metadata: { type: "capability" }, governance: {},
  });
  for (const example of TRACE_EXAMPLES) {
    for (const file of new Set(traceExampleElements(example).map(element => element.file))) {
      const parent = file.slice(0, file.lastIndexOf("/"));
      ensureFolder(parent);
      const previous = files.get(file);
      files.set(file, { path: file, display_path: file, parent_folder: parent, resource_ids: [],
        element_ids: previous?.element_ids ?? [],
        markdown_content: [previous?.markdown_content, traceExampleSource(example, file)].filter(Boolean).join("\n\n"),
      });
      if (!previous) folders.get(parent)!.children.push(file);
    }
    const addRelation = (source: string, target: string, type: string) => relations.push({
      id: JSON.stringify([source, type, target]), source_id: source, target_id: target, target_kind: "element",
      relation_type: type, canonical_relation_type: type, source_relation_types: [type],
      authored: true, generated_opposite: false, resource_id: null,
    });
    for (const element of traceExampleElements(example)) {
      const isVerification = element.id === example.trace.verification.id;
      const type = isVerification ? example.trace.verification.type : "requirement";
      const file = files.get(element.file)!;
      file.element_ids.push(element.id);
      elements.push({ id: element.id, name: element.name, element_type: type,
        type_family: isVerification ? "verification" : "requirement", file_path: element.file,
        line_number: file.markdown_content.slice(0, file.markdown_content.indexOf(`### ${element.name}\n`)).split("\n").length,
        source_anchor: `#/content/${element.file}#${element.id}`, content: example.descriptions[element.id],
        metadata: { type }, governance: isVerification ? {} : { status: "approved", priority: "medium", risk: "low" },
      });
    }
    for (const requirement of example.trace.requirements) {
      if (requirement.directlyVerified) addRelation(example.trace.verification.id, requirement.id, "verify");
      requirement.parentIds.forEach(parent => addRelation(requirement.id, parent, "derivedFrom"));
      if (!requirement.parentIds.length) addRelation(requirement.id, capability.id, "specify");
    }
  }
  const traceFiles: ExplorerProjectStore["traces"]["files"] = {};
  for (const { trace } of TRACE_EXAMPLES) {
    (traceFiles[trace.verification.file] ??= { verifications: [] }).verifications.push({
      identifier: trace.verification.id, name: trace.verification.name, file: trace.verification.file,
      type: trace.verification.type, directly_verified_count: trace.requirements.filter(node => node.directlyVerified).length,
      total_requirements_in_tree: trace.requirements.length,
      directly_verified_requirements: trace.requirements.filter(node => node.directlyVerified).map(node => node.id),
      trace_graph: {
        nodes: trace.requirements.map(node => ({ id: node.id, name: node.name, type: "requirement", is_directly_verified: node.directlyVerified })),
        edges: trace.requirements.flatMap(node => node.parentIds.map(parent => ({ source: node.id, target: parent, relation_type: "derivedFrom" }))),
      },
    });
  }
  return { ...base, traces: { files: traceFiles }, elements, relations, files: [...files.values()], folders: [...folders.values()],
    summaries: { ...base.summaries, elements: elements.length, relations: relations.length, files: files.size, folders: folders.size },
  };
}

const modelStore = withTraceElements(devFixture);
const coverageStore = withTraceElements(scopedCoverage as ExplorerProjectStore);

const fixtureStores = new Map(SHOWCASE_WORKTREES.filter(choice => choice.available).map(choice => {
  const base = choice.id === "showcase-coverage" ? coverageStore : modelStore;
  return [choice.branch, { ...base, project: { ...base.project, branch: choice.branch, worktree_id: choice.id } }];
}));

function requestedFixture(fallback: string) {
  const id = new URLSearchParams(window.location.search).get("branch");
  return id && fixtureStores.has(id) ? id : fallback;
}

export function ExplorerFixtureApp({ example = "model" }: { example?: "model" | "coverage" }) {
  const initialId = example === "coverage" ? "coverage-review" : "main";
  const [worktreeId, setWorktreeId] = useState(() => requestedFixture(initialId));
  useEffect(() => {
    const onPopState = () => setWorktreeId(requestedFixture(initialId));
    window.addEventListener("popstate", onPopState);
    return () => window.removeEventListener("popstate", onPopState);
  }, [initialId]);
  const selectWorktree = (id: string) => {
    if (!fixtureStores.has(id) || id === worktreeId) return;
    writeExplorerUrl(worktreeUrl(window.location.href, id));
    setWorktreeId(id);
  };
  return <ExplorerApplication live={{
    result: { ok: true, schemaMismatch: null, store: fixtureStores.get(worktreeId)! },
    refreshError: null,
    worktreeRouting: true,
    worktrees: SHOWCASE_WORKTREES.map(choice => ({ branch: choice.branch,
      workspace_root: choice.root, available: choice.available, explorer_available: choice.available })),
    selectedWorktree: worktreeId,
    selectWorktree,
    refreshWorktrees: async () => {},
    switching: false,
    worktreeSelectionError: null, dismissWorktreeError: () => {},
    automaticRefresh: false,
    refreshHead: undefined, refreshingHead: false,
    recoveryWarning: null,
  }} />;
}
