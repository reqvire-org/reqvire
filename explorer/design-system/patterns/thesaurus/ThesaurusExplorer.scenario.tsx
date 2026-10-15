import { ThesaurusExplorer } from "@ds";
import { useWorkspaceFixture } from "../../composition/Workspace/Workspace.scenario-adapter";
import { fixture } from "./ThesaurusExplorer.fixture";
export const scenario = { title: fixture.title, covers: ["ThesaurusExplorer"] };
export default function Scenario() {
 const {workspace,overlays} = useWorkspaceFixture({startView:fixture.view, initialMode:fixture.mode, theme: document.documentElement.dataset.theme === "dark" ? "dark" : "light", onThemeChange: theme => {document.documentElement.dataset.theme = theme;}});
 return <><ThesaurusExplorer {...workspace.thesaurus!} />{overlays}</>;
}
