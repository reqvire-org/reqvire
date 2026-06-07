import { useExplorerUiState } from "../components/ExplorerUiState";
import { ContainmentView } from "./ContainmentView";
import { FilesView } from "./FilesView";

export function ModelView({ onOpenElement }: { onOpenElement: (id: string) => void }) {
  const { modelMode } = useExplorerUiState();

  if (modelMode === "sunburst" || modelMode === "icicle") {
    return (
      <ContainmentView
        frameTestId="model"
        modeOverride={modelMode}
        onOpenElement={onOpenElement}
      />
    );
  }

  return (
    <FilesView
      path={null}
      forcedLayout={modelMode}
      onOpenElement={onOpenElement}
    />
  );
}
