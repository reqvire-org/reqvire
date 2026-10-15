import { TraceNavigation } from "@ds";
import { useWorkspaceFixture } from "../../composition/Workspace/Workspace.scenario-adapter";
import { fixture } from "./TraceNavigation.fixture";
export const scenario = { title: fixture.title, covers: ["TraceNavigation"] };
export default function Scenario() {
 const {workspace,overlays} = useWorkspaceFixture({startView:fixture.view, initialMode:fixture.mode, theme: document.documentElement.dataset.theme === "dark" ? "dark" : "light", onThemeChange: theme => {document.documentElement.dataset.theme = theme;}});
 return <><TraceNavigation {...workspace.traceNavigation!} />{overlays}</>;
}
