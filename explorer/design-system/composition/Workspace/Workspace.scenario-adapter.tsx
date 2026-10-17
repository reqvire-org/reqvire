import { useMemo, useState } from "preact/hooks";
import {
  ExplorerWorkspace, type ExplorerWorkspaceProps, useExplorerPane, KnowledgeGraphCanvas, ONTOLOGY_DEFAULT_FILTERS,
  ConceptElementDetailContent, DetailDialog, ElementDetailContent, HelpDialog,
  OntologyNodeDetailDialog, DocumentPanel, SourcePageElements, SourcePageElement,
  MarkdownFrame, CodePreviewFrame, CodeToolbar, CodeBody, CodeFallback,
  type CoverageDrilldownItem, type CoverageReportProjection, type ElementFlowData,
  type FileBrowserItem, type FileBrowserSortKey, type FileNavigationFolder,
  type KnowledgeGraphProjection, type ModelScreenMode, type OntologyGraphData,
  type SearchResultKind, type TraceOverviewFile,
} from "@ds";
import fixture from "./fixture.json";
import { SHOWCASE_WORKTREES } from "./fixture";
import { TRACE_EXAMPLES, traceExampleElements, traceExampleSource } from "../../patterns/reports/fixtures/traces";
import { showcaseFlowLayoutEngine } from "../../patterns/reports/fixtures/layoutEngine";

const elements = new Map(fixture.elements.map(element => [element.id, element]));
const elementById = (id: string) => elements.get(id);
const folders = new Map(fixture.folders.map(folder => [folder.path, folder]));
const graph = fixture.knowledgeGraph as KnowledgeGraphProjection;
const graphNodes = graph.nodes ?? [];
const ontologyGraph = fixture.ontologyGraph as OntologyGraphData;
const coverageByScope = fixture.coverageByScope as Record<string, { projection: CoverageReportProjection; rows: CoverageDrilldownItem[] }>;
const flowBySelection = fixture.flowBySelection as Record<string, ElementFlowData>;
const traceFiles: TraceOverviewFile[] = [];
for (const example of TRACE_EXAMPLES) {
  let file = traceFiles.find(item => item.file === example.trace.verification.file);
  if (!file) { file = { file: example.trace.verification.file, verifications: [] }; traceFiles.push(file); }
  (file.verifications as Array<TraceOverviewFile["verifications"][number]>).push({
    ...example.trace.verification, verificationType: example.trace.verification.type,
    directCount: example.trace.requirements.filter(item => item.directlyVerified).length,
    totalCount: example.trace.requirements.length,
  });
}
traceFiles.sort((left, right) => left.file.localeCompare(right.file));
const typeOptions = [...new Set(fixture.elements.map(element => element.element_type))].map(type => ({
  type, family: fixture.elements.find(element => element.element_type === type)!.type_family,
  count: fixture.elements.filter(element => element.element_type === type).length,
}));
const modelTypeOptions = [...new Set(graphNodes.map(node => node.element_type || node.node_type || node.type || "other"))].map(type => ({
  type, family: type, count: graphNodes.filter(node => (node.element_type || node.node_type || node.type || "other") === type).length,
}));
const NO_OVERLAYS = new Set<never>();
const ALL_KINDS: SearchResultKind[] = ["file", "element", "resource", "ontology"];
const routeForContent = (path: string) => `#/content/${path}`;
const routeForResource = (id: string) => `#/resources/${encodeURIComponent(id)}`;
const initialView = (example: string) => {
  const requested = new URLSearchParams(window.location.search).get("view");
  return ["thesaurus", "model", "ontologies", "traces", "coverage", "search"].includes(requested ?? "") ? requested! : example;
};
const toggle = <T,>(values: Set<T>, value: T) => { const next = new Set(values); if (!next.delete(value)) next.add(value); return next; };

/** Fixture data and controllers only; the production workspace owns all screen composition. */
export function useWorkspaceFixture({ example = "model", startView, initialMode = "grid", theme, onThemeChange }: {
  example?: "model" | "coverage"; startView?: string; initialMode?: ModelScreenMode; theme: "light" | "dark"; onThemeChange: (theme: "light" | "dark") => void;
}) {
  const [view, setView] = useState(() => startView ?? initialView(example));
  const pane = useExplorerPane();
  const [worktree, setWorktree] = useState(example === "coverage" ? "showcase-coverage" : "showcase-main");
  const [modelMode, setModelMode] = useState<ModelScreenMode>(initialMode);
  const [selection, setSelection] = useState("__root__");
  const [modelQuery, setModelQuery] = useState("");
  const [modelNode, setModelNode] = useState<string | null>(null);
  const [modelTypes, setModelTypes] = useState(() => new Set(modelTypeOptions.map(option => option.type)));
  const [concept, setConcept] = useState<string | null>(fixture.concepts[0]?.id ?? null);
  const [conceptQuery, setConceptQuery] = useState("");
  const [traceId, setTraceId] = useState<string | null>(traceFiles[0]?.verifications[0]?.id ?? null);
  const [traceFile, setTraceFile] = useState<string | null>(traceFiles[0]?.file ?? null);
  const [traceQuery, setTraceQuery] = useState("");
  const [scope, setScope] = useState<string | null>(null);
  const [coverageQuery, setCoverageQuery] = useState("");
  const [ontologyNode, setOntologyNode] = useState<string | null>(null);
  const [ontologyFilters, setOntologyFilters] = useState(() => new Set<string>(ONTOLOGY_DEFAULT_FILTERS));
  const [ontologyDetail, setOntologyDetail] = useState<string | null>(null);
  const [copiedUri, setCopiedUri] = useState(false);
  const [query, setQuery] = useState("");
  const [searchKinds, setSearchKinds] = useState(() => new Set(ALL_KINDS));
  const [searchTypes, setSearchTypes] = useState(() => new Set(typeOptions.map(option => option.type)));
  const [detailId, setDetailId] = useState<string | null>(null);
  const [sourcePath, setSourcePath] = useState<string | null>(null);
  const [help, setHelp] = useState(false);
  const [codeExpanded, setCodeExpanded] = useState(true);
  const [codeWrapped, setCodeWrapped] = useState(false);
  const [codeCopied, setCodeCopied] = useState(false);
  const openElement = (id: string) => setDetailId(id);
  const navigate = (next: string) => { setSourcePath(null); setView(next); };
  const openSource = (path: string) => { setDetailId(null); setOntologyDetail(null); setSourcePath(path); setView("source"); };
  const navigateFixture = (hash: string) => openSource(hash.replace(/^#\/content\//, "").split("#")[0]);
  const selectTrace = (id: string) => {
    const owner = traceFiles.find(file => file.verifications.some(item => item.id === id));
    setTraceFile(owner?.file ?? null); setTraceId(id);
  };
  const selectTraceFile = (path: string | null) => { setTraceFile(path); setTraceId(null); };
  const selectedTrace = TRACE_EXAMPLES.find(item => item.trace.verification.id === traceId);
  const selectedTraceFile = traceFiles.find(file => file.file === traceFile) ?? traceFiles[0];
  const report = coverageByScope[scope ?? "whole"] ?? coverageByScope.whole;
  const selectedElement = elementById(selection);
  const filePath = selection.startsWith("file:") ? selection.slice(5) : selectedElement?.file_path;
  const selectedFile = filePath ? fixture.folders.flatMap(folder => folder.files).find(file => file.path === filePath) : undefined;
  const folderPath = selection.startsWith("folder:") ? selection.slice(7) : selectedFile ? selectedFile.path.slice(0, selectedFile.path.lastIndexOf("/")) : "__root__";
  const folder = folders.get(folderPath) ?? fixture.folders[0];
  const [sortKey, setSortKey] = useState<FileBrowserSortKey>("name");
  const [sortDirection, setSortDirection] = useState<"asc" | "desc">("asc");
  const browserItems = useMemo<FileBrowserItem[]>(() => [
    ...folder.folders.map(item => ({kind: "folder" as const, id: `folder:${item.path}`, path: item.path,
      name: item.name, displayPath: item.path, elementCount: item.count, childCount: item.children})),
    ...folder.files.map(item => ({kind: "file" as const, id: `file:${item.path}`, path: item.path,
      name: item.name, displayPath: item.displayPath, elementCount: item.element_ids.length,
      childCount: item.element_ids.length, selected: item.path === selectedFile?.path,
      emptyFile: item.element_ids.length === 0, href: `#/files/${item.path}`, contentHref: routeForContent(item.path)})),
  ].sort((a, b) => {
    if (a.kind !== b.kind && sortKey !== "type") return a.kind === "folder" ? -1 : 1;
    const result = sortKey === "elements" ? a.elementCount - b.elementCount : sortKey === "type" ? a.kind.localeCompare(b.kind) || a.name.localeCompare(b.name)
      : sortKey === "path" ? a.displayPath.localeCompare(b.displayPath) : a.name.localeCompare(b.name);
    return sortDirection === "asc" ? result : -result;
  }), [folder, selectedFile, sortKey, sortDirection]);
  const crumbs = [{path:"__root__",label:fixture.rootLabel}];
  if (folder.path !== "__root__") folder.path.split("/").forEach((label, index, parts) => crumbs.push({path:parts.slice(0,index+1).join("/"),label}));
  const traceElement = detailId ? TRACE_EXAMPLES.flatMap(traceExampleElements).find(item => item.id === detailId) : undefined;
  const detailElement = detailId ? elementById(detailId) : undefined;
  const detailConcept = fixture.concepts.find(item => item.sourceElementId === detailId);
  const sourceElements = fixture.elements.filter(element => element.file_path === sourcePath);
  const sourceTrace = TRACE_EXAMPLES.find(item => traceExampleElements(item).some(element => element.file === sourcePath));
  const sourceText = sourceTrace ? traceExampleSource(sourceTrace, sourcePath!) : sourceElements.map(element => `## ${element.name}\n\n${element.content}`).join("\n\n");
  const searchResults = fixture.elements.filter(element => searchKinds.has("element") && searchTypes.has(element.element_type)
    && `${element.name} ${element.content}`.toLowerCase().includes(query.trim().toLowerCase())).map(element => ({
      key:element.id,title:element.name,route:element.file_path,preview:element.content,
      badge:{label:element.element_type,type:element.element_type,family:element.type_family},
      action:{kind:"button" as const,onClick:()=>openElement(element.id)},
    }));
const workspace: ExplorerWorkspaceProps = {
brandLabel: "REQVIRE",
activeNavigationValue: view,
onNavigate: navigate,
leftPaneOpen: pane.leftPaneOpen,
leftPaneResizing: pane.leftPaneResizing,
leftPaneWidth: pane.leftPaneWidth,
leftPaneMinWidth: pane.minWidth,
leftPaneMaxWidth: pane.maxWidth,
onToggleLeftPane: pane.toggleLeftPane,
onLeftPaneResizePointerDown: pane.handleLeftPaneResizePointerDown,
onLeftPaneResizeKeyDown: pane.handleLeftPaneResizeKeyDown,
isDark: theme === "dark",
onToggleTheme: () => onThemeChange(theme === "dark" ? "light" : "dark"),
onSearch: () => navigate("search"),
onHelp: () => setHelp(true),
worktree: {value:worktree,choices:SHOWCASE_WORKTREES,branch:SHOWCASE_WORKTREES.find(choice=>choice.id===worktree)?.branch,
        onOpen:()=>{},onChange:id=>{setWorktree(id);setDetailId(null);setSourcePath(null);}},
thesaurus: {concepts:fixture.concepts,selectedId:concept,onSelectConcept:setConcept,onOpenConcept:openElement,
        query:conceptQuery,onQueryChange:setConceptQuery},
files: {breadcrumbs:crumbs,layout:modelMode === "list" ? "list" : "grid",showResultCount:false,
        selectedFile:selectedFile?{name:selectedFile.name,title:selectedFile.displayPath}:undefined,
        onOpenFolder:path=>setSelection(path==="__root__"?path:`folder:${path}`),onLayoutChange:setModelMode,
        items:browserItems,sortKey,sortDirection,onSort:key=>{if(key===sortKey)setSortDirection(value=>value==="asc"?"desc":"asc");else{setSortKey(key);setSortDirection("asc");}},
        onOpenFile:path=>setSelection(`file:${path}`),onNavigateFile:path=>setSelection(`file:${path}`),onOpenElement:openElement,
        elements:selectedFile?.element_ids.map(id=>({id,name:elementById(id)?.name??id,type:elementById(id)?.element_type,family:elementById(id)?.type_family}))},
fileNavigation: {tree:fixture.tree as FileNavigationFolder,elementById,activeView:view,onNavigate:navigate,
        onOpenElement:openElement,modelMode,modelSelectionId:selection,setModelSelectionId:setSelection,
        modelTreeQuery:modelQuery,setModelTreeQuery:setModelQuery,routeForContent,routeForResource,navigateHash:navigateFixture,
        summaryItems:[{label:"Elements",value:graph.summary?.elements??fixture.elements.length},{label:"Relations",value:graph.summary?.relations??0},
          {label:"Files",value:fixture.folders.reduce((sum,item)=>sum+item.files.length,0)},{label:"Resources",value:fixture.resources.length}]},
modelFlow: modelMode==="flow"?{data:flowBySelection[selection]??flowBySelection.__root__,layoutEngine:showcaseFlowLayoutEngine,
        mode:modelMode,onModeChange:setModelMode,onRootClick:()=>setSelection("__root__"),onOpenElement:openElement,onOpenSource:element=>openSource(element.file)}:undefined,
modelGraph: modelMode==="graph"?{mode:modelMode,onModeChange:setModelMode,onRootClick:()=>setModelMode("grid"),
        graph:<KnowledgeGraphCanvas graphData={graph} contextKey={worktree} activeTypes={modelTypes} activeOverlays={NO_OVERLAYS}
          selectedId={modelNode} onSelectNode={setModelNode} onOpenElement={openElement} embedded frameTestId="model" />}:undefined,
modelGraphControls: {nodes:graphNodes,selectedNodeId:modelNode,elementById,onOpenElement:openElement,onClear:()=>setModelNode(null),
        options:modelTypeOptions,activeTypes:modelTypes,onToggleType:type=>setModelTypes(values=>toggle(values,type)),
        onResetFilters:()=>setModelTypes(new Set(modelTypeOptions.map(option=>option.type))),summaryItems:[{label:"Submodels",value:graph.summary?.submodels??graph.submodels?.length??0},
          {label:"Elements",value:graph.summary?.elements??fixture.elements.length},{label:"Relations",value:graph.summary?.relations??fixture.relationCount},
          {label:"Contract Binding",value:graph.summary?.contract_bindings??fixture.contractBindingCount}]},
traces: {trace:selectedTrace?.trace??null,traceFiles,selectedFile:selectedTraceFile,layoutEngine:showcaseFlowLayoutEngine,
        contextKey:`${worktree}:${traceId}`,onSelect:selectTrace,onOpenElement:openElement,onOpenSource:element=>openSource(element.file)},
traceNavigation: {files:traceFiles,rootLabel:fixture.rootLabel,traceTreeQuery:traceQuery,setTraceTreeQuery:setTraceQuery,
        traceFilePath:traceFile,setTraceFilePath:selectTraceFile,traceSelectionId:traceId,setTraceSelectionId:selectTrace},
coverage: {coverage:report.projection,capabilityRows:report.rows,scopeId:scope,contextKey:`${worktree}:${scope}`,
        elementById,onOpenElement:openElement,onInspect:target=>{if(target.kind==="element")openElement(target.id);else if(target.href)navigateFixture(target.href);}},
coverageNavigation: {scopes:fixture.coverageScopes,selectedId:scope,onSelect:setScope,query:coverageQuery,onQueryChange:setCoverageQuery},
ontology: {graphData:ontologyGraph,activeFilters:[...ontologyFilters],selectedId:ontologyNode,onSelectNode:setOntologyNode,contextKey:worktree},
ontologyControls: {nodes:ontologyGraph.nodes??[],selectedNodeId:ontologyNode,onOpenNode:setOntologyDetail,onClear:()=>setOntologyNode(null),
        filters:ontologyFilters,onToggleFilter:filter=>setOntologyFilters(values=>toggle(values,filter)),onResetLayout:()=>window.resetOntologyGraphLayout?.(),
        summaryItems:[{label:"Ontologies",value:fixture.ontologySummary.ontology_blocks??0},{label:"Shapes",value:fixture.ontologySummary.shape_blocks??0},
          {label:"Quads",value:fixture.ontologySummary.total_quads??0,title:"RDF statements (subject-predicate-object, with graph context)"},
          {label:"Blocks",value:fixture.ontologySummary.total_blocks??0,title:"Ontology and shape source blocks discovered in the model"}]},
search: {query,onQueryChange:setQuery,onSubmit:event=>event.preventDefault(),resultSummary:`${searchResults.length} results`,
        statusText:query?`Results for “${query}”`:"Enter a query to search the project.",results:query?searchResults:[],emptyMessage:"Enter a query to search elements, files, resources, and ontology terms."},
searchControls: {kinds:searchKinds,counts:{file:7,element:fixture.elements.length,resource:fixture.resources.length,ontology:ontologyGraph.nodes?.length??0},
        elementTypes:searchTypes,options:typeOptions,onToggleKind:kind=>setSearchKinds(values=>toggle(values,kind)),
        onToggleElementType:type=>setSearchTypes(values=>toggle(values,type)),onReset:()=>{setSearchKinds(new Set(ALL_KINDS));setSearchTypes(new Set(typeOptions.map(option=>option.type)));}},
main: sourcePath&&<DocumentPanel toolbar={{label:"Source page",title:sourcePath}}>
        {sourceElements.length?<SourcePageElements>{sourceElements.map(element=><SourcePageElement key={element.id}
          title={element.name} elementType={element.element_type} typeFamily={element.type_family}>
          <ElementDetailContent content={<MarkdownFrame><p>{element.content}</p></MarkdownFrame>} onOpenElement={openElement} />
        </SourcePageElement>)}</SourcePageElements>:<CodePreviewFrame wrapped={codeWrapped}><CodeToolbar expanded={codeExpanded} kind="source" path={sourcePath} lineCount={sourceText.split("\n").length}
            onToggleExpanded={()=>setCodeExpanded(value=>!value)} languageLabel="Markdown" wrapped={codeWrapped} copied={codeCopied} onToggleWrap={()=>setCodeWrapped(value=>!value)} onCopy={async()=>{await navigator.clipboard.writeText(sourceText);setCodeCopied(true);}} />
          <CodeBody collapsed={!codeExpanded}><CodeFallback>{sourceText}</CodeFallback></CodeBody></CodePreviewFrame>}
      </DocumentPanel>
};
return { workspace: workspace, shellRef: pane.shellRef, overlays: <>


{detailId&&<DetailDialog open title={detailElement?.name??traceElement?.name??detailId}
      elementType={detailElement?.element_type??"requirement"} typeFamily={detailElement?.type_family??"requirement"}
      onOpenChange={open=>{if(!open)setDetailId(null);}} onOpenSource={()=>openSource(detailElement?.file_path??traceElement?.file??"")}>
      {detailConcept?<ConceptElementDetailContent definition={<p>{detailConcept.definition}</p>} altLabels={detailConcept.altLabels}
        scopeNote={detailConcept.scopeNote} onOpenElement={openElement} />
        :<ElementDetailContent content={<MarkdownFrame><p>{detailElement?.content??selectedTrace?.descriptions[detailId]??""}</p></MarkdownFrame>} onOpenElement={openElement} />}
    </DetailDialog>}

<OntologyNodeDetailDialog open={ontologyDetail!==null} onOpenChange={open=>{if(!open)setOntologyDetail(null);}}
      node={ontologyGraph.nodes?.find(node=>node.id===ontologyDetail)} nodes={ontologyGraph.nodes??[]}
      missingNodeId={ontologyDetail} copiedUri={copiedUri} onCopyUri={uri=>{void navigator.clipboard.writeText(uri);setCopiedUri(true);}}
      onOpenSource={source=>openSource(source.file_path??"")} />

<HelpDialog open={help} onOpenChange={setHelp} />

</> };

}

export function WorkspaceMock(props: Parameters<typeof useWorkspaceFixture>[0]) {
 const {workspace, shellRef, overlays} = useWorkspaceFixture(props);
 return <><ExplorerWorkspace {...workspace} ref={shellRef} />{overlays}</>;
}
