import { ShowcaseRegion } from "@ds";
import { WorkspaceMock } from "../mocks/WorkspaceMock";

export function MocksPage({ theme, onThemeChange }: { theme: "light" | "dark"; onThemeChange: (theme: "light" | "dark") => void }) {
  const example = new URLSearchParams(window.location.search).get("example");
  return <ShowcaseRegion regions={["mocks"]}>
    <ShowcaseRegion regions={["mocks__workspace"]}>
      <WorkspaceMock theme={theme} onThemeChange={onThemeChange} example={example === "coverage" || example === "coverage-drilldown" ? "coverage" : "model"} />
    </ShowcaseRegion>
  </ShowcaseRegion>;
}
