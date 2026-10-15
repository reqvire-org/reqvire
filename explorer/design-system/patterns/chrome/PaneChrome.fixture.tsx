import type { IconName } from "@ds";

export const fixture = {"title":"Pane Chrome And Trees","description":"Standalone chrome pieces and tree rows shown outside the app shell."};
interface ShowcaseTreeRow {
    id: string;
    label: string;
    icon: IconName;
    count?: number;
    depth: number;
    kind: "folder" | "file" | "element";
    expandable?: boolean;
    open?: boolean;
    selected?: boolean;
}

const MODEL_TREE_ROWS: ShowcaseTreeRow[] = [
    {
        id: "requirements-folder",
        label: "system-model/",
        icon: "folder-open",
        count: 3,
        depth: 0,
        kind: "folder",
        expandable: true,
        open: true,
    },
    {
        id: "system-requirements-file",
        label: "SystemRequirements.md",
        icon: "file-text",
        count: 12,
        depth: 1,
        kind: "file",
        expandable: true,
        open: true,
    },
    {
        id: "REQ-DET-042",
        label: "Traceability Coverage Requirement",
        icon: "box",
        depth: 2,
        kind: "element",
        selected: true,
    },
    {
        id: "VER-DET-010",
        label: "Coverage Export Verification",
        icon: "check",
        depth: 2,
        kind: "element",
    },
    {
        id: "ontology-folder",
        label: "ontologies/",
        icon: "folder",
        count: 2,
        depth: 0,
        kind: "folder",
        expandable: true,
    },
];

export { MODEL_TREE_ROWS };
