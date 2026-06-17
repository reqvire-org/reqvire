import type { ViewId } from "../../router/routes";

export interface ExplorerViewProps {
  activeView: ViewId;
  onNavigate?: (view: ViewId) => void;
}
