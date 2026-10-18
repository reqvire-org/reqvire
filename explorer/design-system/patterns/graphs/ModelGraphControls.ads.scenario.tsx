import { ModelGraphControls } from "@ds";
import { useWorkspaceFixture } from "../../composition/Workspace/Workspace.ads.helpers";
import { fixture } from "./ModelGraphControls.ads.fixture";
export const scenario = { title: fixture.title, covers: ["ModelGraphControls"] };
export default function Scenario() {
 const {workspace,overlays} = useWorkspaceFixture({startView:fixture.view, initialMode:fixture.mode, theme: document.documentElement.dataset.theme === "dark" ? "dark" : "light", onThemeChange: theme => {document.documentElement.dataset.theme = theme;}});
 return <><ModelGraphControls {...workspace.modelGraphControls!} />{overlays}</>;
}
