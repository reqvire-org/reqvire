import { useEffect, useState } from "preact/hooks";
import { WorkspaceMock } from "./Workspace.scenario-adapter";
import { fixture } from "./fixture";
export const scenario = { title: "Explorer workspace", covers: ["ExplorerWorkspace", "KnowledgeGraphCanvas", "ConceptElementDetailContent", "DetailDialog", "ElementDetailContent", "HelpDialog", "OntologyNodeDetailDialog", "DocumentPanel", "SourcePageElements", "SourcePageElement", "MarkdownFrame", "CodePreviewFrame", "CodeToolbar", "CodeBody", "CodeFallback"] };
export default function Scenario() {
 const [theme, setTheme] = useState<"light" | "dark">(() => document.documentElement.dataset.theme === "dark" ? "dark" : "light");
 useEffect(() => { document.documentElement.dataset.theme = theme; }, [theme]);
 return <WorkspaceMock {...fixture} theme={theme} onThemeChange={setTheme} />;
}
