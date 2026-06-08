import { useState } from "react";
import { Icon, Tabs } from "@ds";
import { CorePage } from "./pages/CorePage";
import { ControlsPage } from "./pages/ControlsPage";
import { DataPage } from "./pages/DataPage";
import { MocksPage } from "./pages/MocksPage";
import { NavigationPage } from "./pages/NavigationPage";
import { TokensPage } from "./pages/TokensPage";

type Tab = "tokens" | "core" | "data" | "controls" | "navigation" | "mocks";

const TABS = [
  { value: "tokens" as Tab, label: "Tokens", icon: <Icon name="layers" size={14} /> },
  { value: "core" as Tab, label: "Core", icon: <Icon name="box" size={14} /> },
  { value: "data" as Tab, label: "Data", icon: <Icon name="database" size={14} /> },
  { value: "controls" as Tab, label: "Controls", icon: <Icon name="sliders" size={14} /> },
  { value: "navigation" as Tab, label: "Navigation", icon: <Icon name="git-branch" size={14} /> },
  { value: "mocks" as Tab, label: "Mocks", icon: <Icon name="grid" size={14} /> },
];

export function ShowcaseApp() {
  const [tab, setTab] = useState<Tab>("tokens");
  const isMocks = tab === "mocks";

  return (
    <div className="sc-app">
      <div className="sc-topbar">
        <div className="sc-topbar__brand">
          <span className="sc-topbar__brand-mark">RQ</span>
          Design System
        </div>
        <Tabs items={TABS} value={tab} onChange={setTab} />
      </div>
      <div className={isMocks ? "sc-body sc-body--full" : "sc-body"}>
        {tab === "tokens" && <TokensPage />}
        {tab === "core" && <CorePage />}
        {tab === "data" && <DataPage />}
        {tab === "controls" && <ControlsPage />}
        {tab === "navigation" && <NavigationPage />}
        {tab === "mocks" && <MocksPage />}
      </div>
    </div>
  );
}
