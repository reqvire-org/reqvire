import { ThesaurusPane } from "@ds";
import { useWorkspaceFixture } from "../../composition/Workspace/Workspace.ads.helpers";
import { fixture } from "./ThesaurusPane.ads.fixture";
export const scenario = { title: fixture.title, covers: ["ThesaurusPane"] };
export default function Scenario() {
 const {workspace,overlays} = useWorkspaceFixture({startView:fixture.view, initialMode:fixture.mode, theme: document.documentElement.dataset.theme === "dark" ? "dark" : "light", onThemeChange: theme => {document.documentElement.dataset.theme = theme;}});
 return <><ThesaurusPane {...workspace.thesaurus!} query={workspace.thesaurus!.query} onQueryChange={workspace.thesaurus!.onQueryChange} onSelectConcept={id => { if(id) workspace.thesaurus!.onSelectConcept(id); }} />{overlays}</>;
}
