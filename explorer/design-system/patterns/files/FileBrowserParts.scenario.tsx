import { ShowcaseRegion, FileBrowserFrame, FileBrowserGrid, FileBrowserElementsPanel, FileBrowserModeledElements, FileBrowserModeledElement, FileBrowserEmptyState, FileBrowserMissingFile } from "@ds";
import { useState } from "preact/hooks";

import { fixture } from "./FileBrowserParts.fixture";
export const scenario = {"title": "FileBrowserParts", "covers": ["FileBrowserFrame", "FileBrowserGrid", "FileBrowserElementsPanel", "FileBrowserModeledElements", "FileBrowserModeledElement", "FileBrowserEmptyState", "FileBrowserMissingFile"]};
export default function Scenario() {
const [selected,setSelected] = useState("none");
return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<FileBrowserFrame><FileBrowserGrid items={[...fixture.items]} onOpenFolder={setSelected} /><FileBrowserElementsPanel><FileBrowserModeledElements layout="grid"><FileBrowserModeledElement layout="grid" name="Release traceability" onOpen={() => setSelected("Release traceability")} /></FileBrowserModeledElements><FileBrowserEmptyState>No other elements in this file.</FileBrowserEmptyState></FileBrowserElementsPanel><FileBrowserMissingFile path="missing.md" /></FileBrowserFrame><p>Selection: {selected}</p>
</ShowcaseRegion>;
}
