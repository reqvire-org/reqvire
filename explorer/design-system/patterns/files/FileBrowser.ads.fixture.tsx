import type { FileBrowserBreadcrumb, FileBrowserItem } from "@ds";

export const fixture = {"title":"File Browser","description":"Explorer file/model table product pattern. This is the real table surface used by the model browser."};
const FILE_BROWSER_BREADCRUMBS = [
    { path: "", label: "Project" },
    { path: "requirements", label: "requirements" },
] as const satisfies readonly FileBrowserBreadcrumb[];

const FILE_BROWSER_ITEMS = [
    {
        kind: "folder",
        id: "requirements-capabilities",
        name: "Capabilities",
        path: "system-model/Capabilities",
        displayPath: "system-model/Capabilities",
        elementCount: 8,
        childCount: 2,
    },
    {
        kind: "folder",
        id: "requirements-functional",
        name: "Functional",
        path: "system-model/Functional",
        displayPath: "system-model/Functional",
        elementCount: 5,
        childCount: 3,
    },
    {
        kind: "file",
        id: "requirements-capabilities-md",
        name: "Capabilities.md",
        path: "system-model/Capabilities.md",
        displayPath: "system-model/Capabilities.md",
        elementCount: 3,
        childCount: 0,
        selected: true,
        contentHref: "#/content/system-model/Capabilities.md",
    },
    {
        kind: "file",
        id: "requirements-contracts-md",
        name: "Contracts.md",
        path: "system-model/Contracts.md",
        displayPath: "system-model/Contracts.md",
        elementCount: 6,
        childCount: 0,
        contentHref: "#/content/system-model/Contracts.md",
    },
] as const satisfies readonly FileBrowserItem[];

export { FILE_BROWSER_BREADCRUMBS, FILE_BROWSER_ITEMS };
