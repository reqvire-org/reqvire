import { ModelFlowScreen } from "@ds";
import { useWorkspaceFixture } from "../../composition/Workspace/Workspace.scenario-adapter";
import { fixture } from "./ModelScreen.fixture";
export const scenario = { title: fixture.title, covers: ["ModelFlowScreen"] };
export default function Scenario() {
 const {workspace,overlays} = useWorkspaceFixture({startView:fixture.view, initialMode:fixture.mode, theme: document.documentElement.dataset.theme === "dark" ? "dark" : "light", onThemeChange: theme => {document.documentElement.dataset.theme = theme;}});
 const flow = workspace.modelFlow!;
 return <><ModelFlowScreen data={flow.data} layoutEngine={flow.layoutEngine} mode={flow.mode}
   onModeChange={flow.onModeChange} onRootClick={flow.onRootClick}
   onOpenElement={flow.onOpenElement} onOpenSource={flow.onOpenSource} />{overlays}</>;
}
