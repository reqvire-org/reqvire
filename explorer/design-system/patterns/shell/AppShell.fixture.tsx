import type { WorktreeSelectorProps } from "@ds";
import { ShowcaseRegion } from "@ds";
import { type ShellActionItem, type ShellNavigationItem } from "@ds";

export const fixture = {"title":"Explorer Shell","description":"Shared shell with pane-aligned branding and view tabs, and a branch control at the far right after header actions. The real Explorer mock supports pane resizing and collapse with this same header."};
const SHOWCASE_WORKTREES: WorktreeSelectorProps["choices"] = [
    { id: "showcase-main", branch: "main", root: "/workspace/reqvire", available: true },
    { id: "showcase-coverage", branch: "coverage-review", root: "/workspace/reqvire-coverage", available: true },
    { id: "showcase-unavailable", branch: "archived", root: "/workspace/reqvire-archived", available: false },
];
const SHELL_PATTERN_NAVIGATION = [
    {
        value: "workspace",
        icon: "circle",
        label: (<>
        <ShowcaseRegion regions={["shell-visually-hidden"]} as="span">Workspace slot</ShowcaseRegion>
        <ShowcaseRegion regions={["shell-nav-placeholder", "shell-nav-placeholder--wide"]} as="span" aria-hidden="true"/>
      </>),
    },
    {
        value: "secondary",
        icon: "circle",
        label: (<>
        <ShowcaseRegion regions={["shell-visually-hidden"]} as="span">Secondary slot</ShowcaseRegion>
        <ShowcaseRegion regions={["shell-nav-placeholder"]} as="span" aria-hidden="true"/>
      </>),
    },
    {
        value: "tertiary",
        icon: "circle",
        label: (<>
        <ShowcaseRegion regions={["shell-visually-hidden"]} as="span">Tertiary slot</ShowcaseRegion>
        <ShowcaseRegion regions={["shell-nav-placeholder", "shell-nav-placeholder--medium"]} as="span" aria-hidden="true"/>
      </>),
        badge: <ShowcaseRegion regions={["shell-badge-placeholder"]} as="span" aria-hidden="true"/>,
    },
] as const satisfies readonly ShellNavigationItem[];

const SHELL_PATTERN_ACTIONS = [
    { id: "shell-action-a", label: "Shell action A", icon: "circle" },
    { id: "shell-action-b", label: "Shell action B", icon: "circle" },
    { id: "shell-action-c", label: "Shell action C", icon: "circle" },
] as const satisfies readonly Omit<ShellActionItem, "onClick">[];
export { SHOWCASE_WORKTREES, SHELL_PATTERN_NAVIGATION, SHELL_PATTERN_ACTIONS };
