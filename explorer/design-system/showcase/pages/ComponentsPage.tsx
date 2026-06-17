import { ControlsPage } from "./ControlsPage";
import { CorePage } from "./CorePage";
import { DataPage } from "./DataPage";
import { NavigationPage } from "./NavigationPage";

export function ComponentsPage() {
  return (
    <div className="showcase-components-page">
      <CorePage />
      <DataPage />
      <ControlsPage />
      <NavigationPage />
    </div>
  );
}
