import { FileNavigation } from "@ds";
import { useWorkspaceFixture } from "../../composition/Workspace/Workspace.ads.helpers";
import { fixture } from "./FileNavigation.ads.fixture";
export const scenario = { title: fixture.title, covers: ["FileNavigation"] };
export default function Scenario() {
 const {workspace,overlays} = useWorkspaceFixture({startView:fixture.view, initialMode:fixture.mode, theme: document.documentElement.dataset.theme === "dark" ? "dark" : "light", onThemeChange: theme => {document.documentElement.dataset.theme = theme;}});
 return <><FileNavigation {...workspace.fileNavigation!} />{overlays}</>;
}
