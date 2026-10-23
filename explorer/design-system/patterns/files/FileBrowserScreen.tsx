import { type ComponentChildren } from "preact";
import { RouteFrame } from "../shell/RouteFrame";
import { FileBrowserFrame, FileBrowserToolbar, FileBrowserMissingFile,
  FileBrowserList, FileBrowserGrid, FileBrowserElementsPanel, FileBrowserEmptyState,
  FileBrowserModeledElements, FileBrowserModeledElement,
  type FileBrowserToolbarProps, type FileBrowserListProps } from "./FileBrowser";

export interface FileBrowserScreenProps extends Omit<FileBrowserToolbarProps, "resultCount">,
  Pick<FileBrowserListProps, "items" | "sortKey" | "sortDirection" | "onSort" | "onOpenFile"> {
  /** Grid navigation when the consumer has not supplied in-place selection. Table rows retain native links. */
  onNavigateFile: (path: string) => void;
  missingPath?: string;
  elements?: readonly { id: string; name: string; type?: string; family?: string }[];
  onOpenElement: (id: string) => void;
  preview?: ComponentChildren;
}

/** Shared production file/grid screen; file selection and routing are adapters. */
export function FileBrowserScreen({ missingPath, items, sortKey, sortDirection, onSort,
  onOpenFile, onNavigateFile, onOpenElement, elements, preview, ...toolbar }: FileBrowserScreenProps) {
  return <RouteFrame viewId="files"><FileBrowserFrame>
    <FileBrowserToolbar {...toolbar} resultCount={items.length} />
    {missingPath ? <FileBrowserMissingFile path={missingPath} /> : <>
      {toolbar.layout === "list" ? <FileBrowserList items={items} sortKey={sortKey}
        sortDirection={sortDirection} onSort={onSort} onOpenFolder={toolbar.onOpenFolder} onOpenFile={onOpenFile} />
        : <FileBrowserGrid items={items} onOpenFolder={toolbar.onOpenFolder} onOpenFile={onOpenFile ?? onNavigateFile} />}
      <FileBrowserElementsPanel>
        {elements === undefined ? <FileBrowserEmptyState>Select a file row to inspect its modeled elements.</FileBrowserEmptyState>
          : elements.length === 0 ? preview
          : <FileBrowserModeledElements layout={toolbar.layout}>
            {elements.map(element => <FileBrowserModeledElement key={element.id} layout={toolbar.layout}
              name={element.name} type={element.type} family={element.family} onOpen={() => onOpenElement(element.id)} />)}
          </FileBrowserModeledElements>}
      </FileBrowserElementsPanel>
    </>}
  </FileBrowserFrame></RouteFrame>;
}
