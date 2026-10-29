import { OntologyControlsPane } from "@ds";
import { useWorkspaceFixture } from "../../composition/Workspace/Workspace.ads.helpers";
import { fixture } from "./GraphControlsPane.ads.fixture";
export const scenario = { title: fixture.title, covers: ["OntologyControlsPane"] };
export default function Scenario() {
 const {workspace,overlays} = useWorkspaceFixture({startView:fixture.view, initialMode:fixture.mode, theme: document.documentElement.dataset.theme === "dark" ? "dark" : "light", onThemeChange: theme => {document.documentElement.dataset.theme = theme;}});
 return <><OntologyControlsPane {...workspace.ontologyControls!} />{overlays}</>;
}
