import { useState } from "react";
import { Icon, Tabs } from "@ds";
import { ComponentsPage } from "./pages/ComponentsPage";
import { MocksPage } from "./pages/MocksPage";
import { ProductPatternsPage } from "./pages/ProductPatternsPage";
import { TokensPage } from "./pages/TokensPage";

type Tab = "tokens" | "components" | "patterns" | "mocks";

const TABS = [
  { value: "tokens" as Tab, label: "Tokens", icon: <Icon name="layers" size={14} /> },
  { value: "components" as Tab, label: "Components", icon: <Icon name="box" size={14} /> },
  { value: "patterns" as Tab, label: "Patterns", icon: <Icon name="layout-grid" size={14} /> },
  { value: "mocks" as Tab, label: "Mocks", icon: <Icon name="grid" size={14} /> },
];

export function ShowcaseApp() {
  const [tab, setTab] = useState<Tab>("tokens");
  const isMocks = tab === "mocks";

  return (
    <div className="showcase-app">
      <div className="showcase-topbar">
        <div className="showcase-topbar__brand">
          <span className="showcase-topbar__brand-mark">RQ</span>
          Design System
        </div>
        <Tabs items={TABS} value={tab} onChange={setTab} />
      </div>
      <div className={isMocks ? "showcase-body showcase-body--full" : "showcase-body"}>
        {tab === "tokens" && <TokensPage />}
        {tab === "components" && <ComponentsPage />}
        {tab === "patterns" && <ProductPatternsPage />}
        {tab === "mocks" && <MocksPage />}
      </div>
    </div>
  );
}
