# Explorer workspace

`ExplorerWorkspace` owns Explorer navigation, header actions, the compact
worktree selector and matching screen/pane composition. Production adapters
and `composition/Workspace/Scenario.tsx` render this public pattern. Applications
supply live DTOs, routing and services; scenarios supply fixtures and local state.

The typed screen props select the shared Thesaurus, Model, Traces, Coverage,
Ontologies and Search patterns. Model accepts file/grid, flow and graph modes.
Slots remain available for application adapters and exceptional routes. Do not
replace a screen with independently composed placeholder content in a mock.

`useExplorerPane` shares the existing persisted width, viewport limits,
pointer/keyboard resizing and collapse behavior between production and mocks.
Sandboxed previews and browsers that deny storage retain session-local width.
Unmounting cancels an active resize and releases its document listeners.
The shell forwards its ref for measured geometry; approved writes are declared
in `ds.config.json`. Header actions are supplied through callbacks, so the
workspace has no dependency on routing, stores or theme persistence.

`AppShell` keeps the leading brand segment and tab divider aligned with the
expanded pane during pointer and keyboard resizing. Header actions and
`headerContext` stay at the right edge, with the worktree context last. The
picker uses `menuAlign="end"`; collapse preserves a usable brand segment and
narrow headers reflow navigation into a scrollable row.

`WorktreeLoadDialog` presents loading, initial failures and refresh failures.
The refresh variant retains the displayed branch and retry policy in one
dismissible modal. Recovery guidance precedes a wrapping, selectable,
keyboard-scrollable diagnostic region; the Close footer remains visible.
The application suppresses repeated dialogs for the same unresolved failure
while retaining the accepted model and automatic refresh. Patterns previews
exercise the exported dialog with loading, connection and long-error fixtures.

Screen patterns accept layout/renderer services without changing worker or
cache policy. Heavy graph implementations remain lazy. Existing full-app
integration and route tests stay under `src/test/`; the workspace scenario test
verifies fixture callbacks. DS tooling supplies the neutral MCP client host.
Run lint, typecheck, tests and both builds, then compare the matching screen,
theme and state in the browser.
