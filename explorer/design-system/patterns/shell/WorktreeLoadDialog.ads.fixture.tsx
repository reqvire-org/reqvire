import type { WorktreeLoadDialogProps } from "@ds";

export const fixture = {"title":"Worktree Loading and Errors","description":"The shared Explorer dialog identifies the branch, explains recovery, and keeps full diagnostics readable. The loading preview completes after five seconds; errors support Close, Escape and outside dismissal."};
const SHOWCASE_WORKTREE_DIALOGS = {
    pending: { branch: "coverage-review" },
    connection: { branch: "coverage-review", error: "NetworkError when attempting to fetch resource." },
    validation: {
        branch: "bug-fixes-01",
        error: [
            "Validation failed with 2 errors:",
            "Circular dependency error: requirement fulfillment system-model/Interfaces/WebExplorer/Capabilities.md#serve-command-embedded-mcp-endpoint",
            "−[contract consumer]→ system-model/Interfaces/WebExplorer/Capabilities.md#serve-command",
            "−[child requirement]→ system-model/Interfaces/WebExplorer/Capabilities.md#served-explorer-runtime-freshness",
            "−[contract consumer]→ system-model/Interfaces/WebExplorer/Capabilities.md#serve-command-embedded-mcp-endpoint",
            "Circular dependency error: requirement fulfillment system-model/Interfaces/WebExplorer/Capabilities.md#serve-command-embedded-mcp-endpoint",
            "−[contract consumer]→ system-model/Interfaces/WebExplorer/Capabilities.md#serve-command",
            "−[contract consumer]→ system-model/Interfaces/WebExplorer/Capabilities.md#serve-command-embedded-mcp-endpoint",
        ].join(" "),
    },
    refresh: {
        branch: "main", operation: "refresh", retryAutomatically: true,
        error: "The server connection is unavailable.",
    },
} as const satisfies Record<string, Omit<WorktreeLoadDialogProps, "onDismiss">>;

export { SHOWCASE_WORKTREE_DIALOGS };
