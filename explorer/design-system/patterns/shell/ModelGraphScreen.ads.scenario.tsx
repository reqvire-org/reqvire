import { ModelGraphScreen } from "@ds";
import { useWorkspaceFixture } from "../../composition/Workspace/Workspace.ads.helpers";
import { fixture } from "./ModelGraphScreen.ads.fixture";
export const scenario = { title: fixture.title, covers: ["ModelGraphScreen"] };
export default function Scenario() {
 const {workspace,overlays} = useWorkspaceFixture({startView:fixture.view, initialMode:fixture.mode, theme: document.documentElement.dataset.theme === "dark" ? "dark" : "light", onThemeChange: theme => {document.documentElement.dataset.theme = theme;}});
 const graph = workspace.modelGraph!;
 return <><ModelGraphScreen graph={graph.graph} mode={graph.mode}
   onModeChange={graph.onModeChange} onRootClick={graph.onRootClick} />{overlays}</>;
}
