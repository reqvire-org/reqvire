import type { WorktreeSelectorProps } from "@ds";

export const fixture = {"title":"Worktree Selector","description":"Choose an admitted branch and worktree. Unavailable contexts remain visible and disabled. In Mocks, this control switches the displayed fixture model through the Explorer shell."};
const SHOWCASE_WORKTREES: WorktreeSelectorProps["choices"] = [
    { id: "showcase-main", branch: "main", root: "/workspace/reqvire", available: true },
    { id: "showcase-coverage", branch: "coverage-review", root: "/workspace/reqvire-coverage", available: true },
    { id: "showcase-unavailable", branch: "archived", root: "/workspace/reqvire-archived", available: false },
];

export { SHOWCASE_WORKTREES };
