import { TracesScreen } from "@ds";
import { useWorkspaceFixture } from "../../composition/Workspace/Workspace.ads.helpers";
import { fixture } from "./TracesScreen.ads.fixture";
export const scenario = { title: fixture.title, covers: ["TracesScreen"] };
export default function Scenario() {
 const {workspace,overlays} = useWorkspaceFixture({startView:fixture.view, initialMode:fixture.mode, theme: document.documentElement.dataset.theme === "dark" ? "dark" : "light", onThemeChange: theme => {document.documentElement.dataset.theme = theme;}});
 return <><TracesScreen {...workspace.traces!} />{overlays}</>;
}
