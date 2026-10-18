import { CoverageReportScreen } from "@ds";
import { useWorkspaceFixture } from "../../composition/Workspace/Workspace.ads.helpers";
import { fixture } from "./CoverageReportScreen.ads.fixture";
export const scenario = { title: fixture.title, covers: ["CoverageReportScreen"] };
export default function Scenario() {
 const {workspace,overlays} = useWorkspaceFixture({startView:fixture.view, initialMode:fixture.mode, theme: document.documentElement.dataset.theme === "dark" ? "dark" : "light", onThemeChange: theme => {document.documentElement.dataset.theme = theme;}});
 return <><CoverageReportScreen {...workspace.coverage!} />{overlays}</>;
}
