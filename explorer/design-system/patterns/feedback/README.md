# Feedback patterns

`StoreNotice` is the blocking recovery screen shown when Explorer cannot load a
Project Store. Its default `layout="page"` fills its containing application pane.
Use `layout="embedded"` when presenting the same recovery panel inside a catalogue
card or another existing layout; it removes page-level padding and height.

The panel width and recovery-card minimum width use the project geometry tokens
`--space-store-notice-panel-max-w` and `--space-store-notice-action-min-w`.
Recovery cards reflow using their available container width, including a narrow
container inside a wide browser window. All three share a row when they fit;
wrapped cards fill their row rather than leaving an empty grid slot.
Code references wrap through their
supported non-state hooks so command labels and long source paths remain readable.
Consumers do not override the pattern or CodeRef internals.

The Patterns page uses the embedded variant. Production's `MissingStoreNotice`
adapter uses the page default. Verify both at wide and narrow widths in the
browser, then run lint/typecheck and application/showcase builds. This persistent
blocking state remains visible until its data-loading problem is resolved.

`HelpContent` and `HelpDialog` share the model legend and supported navigation.
Transient feedback uses the local Toast API documented in its component folder.
