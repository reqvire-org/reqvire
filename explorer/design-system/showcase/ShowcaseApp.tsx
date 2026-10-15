import { Icon, ShowcaseRegion, ShowcaseHeader, ShowcaseNavigation } from "@ds";
import { useEffect, useState } from "preact/hooks";
import { Toaster, type ToastPosition } from "@ds";
import { ToastDemoSettingsContext } from "./toastDemoSettings";
import { ComponentsPage } from "./pages/ComponentsPage";
import { MocksPage } from "./pages/MocksPage";
import { PatternsPage } from "./pages/PatternsPage";
import { TokensPage } from "./pages/TokensPage";

type Tab = "tokens" | "components" | "patterns" | "mocks";

const TABS = [
  { value: "tokens" as Tab, label: "Tokens", icon: <Icon name="layers" size={14} /> },
  { value: "components" as Tab, label: "Components", icon: <Icon name="box" size={14} /> },
  { value: "patterns" as Tab, label: "Patterns", icon: <Icon name="layout-grid" size={14} /> },
  { value: "mocks" as Tab, label: "Mocks", icon: <Icon name="grid" size={14} /> },
];

export function ShowcaseApp() {
  const [tab, setTab] = useState<Tab>(() => {
    const requested = new URLSearchParams(window.location.search).get("tab");
    return TABS.find(item => item.value === requested)?.value ?? "tokens";
  });
  const selectTab = (value: Tab) => {
    const url = new URL(window.location.href);
    url.searchParams.set("tab", value);
    window.history.replaceState(null, "", url);
    setTab(value);
  };
  const [toastPosition, setToastPosition] = useState<ToastPosition>("bottom-right");
  const [toastExpand, setToastExpand] = useState(false);
  const [theme, setTheme] = useState<"light" | "dark">("light");
  useEffect(() => { document.documentElement.dataset.theme = theme; }, [theme]);
  const isMocks = tab === "mocks";

  return (
    <ToastDemoSettingsContext.Provider value={{ position: toastPosition, setPosition: setToastPosition, expand: toastExpand, setExpand: setToastExpand }}>
    <ShowcaseRegion regions={["app"]}>
      <ShowcaseHeader title="Reqvire Design System" navigation={<ShowcaseNavigation items={TABS} value={tab} onChange={selectTab} />} theme={theme} onThemeChange={setTheme} />
      <ShowcaseRegion regions={isMocks ? ["body", "body--full"] : ["body"]}>
        {tab === "tokens" && <TokensPage />}
        {tab === "components" && <ComponentsPage />}
        {tab === "patterns" && <PatternsPage />}
        {tab === "mocks" && <MocksPage theme={theme} onThemeChange={setTheme} />}
      </ShowcaseRegion>
    </ShowcaseRegion>
    <Toaster position={toastPosition} expand={toastExpand} />
    </ToastDemoSettingsContext.Provider>
  );
}
