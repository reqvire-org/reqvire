import { MockShell } from "../MockShell";

export function MocksPage() {
  const example = new URLSearchParams(window.location.search).get("example");
  return <div className="showcase-mocks">
    <div className="showcase-mocks__workspace">
      <MockShell example={example === "coverage" || example === "coverage-drilldown" ? "coverage" : "model"} />
    </div>
  </div>;
}
