import { type ComponentChildren } from "preact";
import { Icon } from "../../components/core/Icon";
import { SegmentedControl } from "../../components/controls/SegmentedControl";
import { RouteFrame, RouteLayout } from "./RouteFrame";
import { WorkspaceShell } from "../content/WorkspaceShell";
import { ElementFlow, type ElementFlowProps } from "../reports/TraceFlow";

export type ModelScreenMode = "list" | "grid" | "flow" | "graph";
export interface ModelModeSelectorProps { value: ModelScreenMode; onChange: (mode: ModelScreenMode) => void; }
export function ModelModeSelector({ value, onChange }: ModelModeSelectorProps) {
  return <SegmentedControl<ModelScreenMode> ariaLabel="Model layout" value={value} onChange={onChange} items={[
    { value: "list", label: "List", icon: <Icon name="list" /> },
    { value: "grid", label: "Grid", icon: <Icon name="layout-grid" /> },
    { value: "graph", label: "Graph", icon: <Icon name="git-branch" /> },
    { value: "flow", label: "Flow", icon: <Icon name="network" /> },
  ]} />;
}
export interface ModelFlowScreenProps extends ElementFlowProps {
  mode: ModelScreenMode;
  onModeChange: (mode: ModelScreenMode) => void;
  onRootClick: () => void;
}
export function ModelFlowScreen({ mode, onModeChange, onRootClick, ...flow }: ModelFlowScreenProps) {
  return <RouteFrame viewId="model"><RouteLayout><WorkspaceShell rootLabel="Model" currentLabel="Flow"
    breadcrumbLabel="Model flow breadcrumbs" onRootClick={onRootClick}
    controls={<ModelModeSelector value={mode} onChange={onModeChange} />}>
    <ElementFlow {...flow} />
  </WorkspaceShell></RouteLayout></RouteFrame>;
}
export interface ModelGraphScreenProps {
  graph: ComponentChildren;
  mode: ModelScreenMode;
  onModeChange: (mode: ModelScreenMode) => void;
  onRootClick: () => void;
}
export function ModelGraphScreen({ graph, mode, onModeChange, onRootClick }: ModelGraphScreenProps) {
  return <RouteFrame viewId="model"><RouteLayout><WorkspaceShell rootLabel="Model" currentLabel="Graph"
    breadcrumbLabel="Model graph breadcrumbs" onRootClick={onRootClick}
    controls={<ModelModeSelector value={mode} onChange={onModeChange} />}>
    {graph}
  </WorkspaceShell></RouteLayout></RouteFrame>;
}
