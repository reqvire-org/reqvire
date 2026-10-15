import { ShowcaseRegion } from "@ds";
import { ControlsExamples } from "../examples/components/ControlsExamples";
import { CoreExamples } from "../examples/components/CoreExamples";
import { DataExamples } from "../examples/components/DataExamples";
import { NavigationExamples } from "../examples/components/NavigationExamples";

export function ComponentsPage() {
  return (
    <ShowcaseRegion regions={["components-page"]}>
      <CoreExamples />
      <DataExamples />
      <ControlsExamples />
      <NavigationExamples />
    </ShowcaseRegion>
  );
}
