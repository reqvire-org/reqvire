import { FileBrowserScreen } from "@ds";
import { useWorkspaceFixture } from "../../composition/Workspace/Workspace.ads.helpers";
import { fixture } from "./FileBrowserScreen.ads.fixture";
export const scenario = { title: fixture.title, covers: ["FileBrowserScreen"] };
export default function Scenario() {
 const {workspace,overlays} = useWorkspaceFixture({startView:fixture.view, initialMode:fixture.mode, theme: document.documentElement.dataset.theme === "dark" ? "dark" : "light", onThemeChange: theme => {document.documentElement.dataset.theme = theme;}});
 return <><FileBrowserScreen {...workspace.files!} />{overlays}</>;
}
