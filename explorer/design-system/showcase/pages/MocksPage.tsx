import { useState } from "react";
import { SegmentedControl } from "@ds";
import { MockShell } from "../MockShell";

type Example = "model" | "coverage";

export function MocksPage() {
  const [example, setExample] = useState<Example>(() => {
    const requested = new URLSearchParams(window.location.search).get("example");
    return requested === "coverage" || requested === "coverage-drilldown" ? "coverage" : "model";
  });
  return <div className="showcase-mocks">
    <div className="showcase-mocks__toolbar">
      <SegmentedControl ariaLabel="Example model" value={example} onChange={value => {
        const url = new URL(window.location.href);
        url.searchParams.set("example", value);
        url.hash = value === "model" ? "#/model" : "#/coverage";
        window.history.replaceState(null, "", url);
        setExample(value);
      }} items={[
        { value: "model", label: "Model and ontologies" },
        { value: "coverage", label: "Scoped coverage" },
      ]} />
    </div>
    <div className="showcase-mocks__workspace">
      <MockShell key={example} example={example} />
    </div>
  </div>;
}
