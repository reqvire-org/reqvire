# Showcase demonstration regions

`ShowcaseRegion` is a development review helper (`.ads.review.tsx`) exported
through `@ds/review` for scenarios only. It offers a finite set of named
arrangements. Generic page, section, heading, row, column and label framing maps
to ADS's preview framing (`previewFrameClasses`); Reqvire-specific demonstration
layouts (shell mocks, pattern cards, tree/sidebar demos) are styled in
`frame.ads.review.css`. It accepts no arbitrary `style` or `className` props.

ADS owns the showcase shell, navigation, theme switching and preview framing;
the preview mount (`showcase/mcp/main.ads.mount.tsx`) renders into
`createPreviewRoot` and applies the product document skin.

Originally adapted from the DS tooling repository's working example:
https://github.com/reqvire-org/ads. Its MIT notice is retained in
`LICENSE.source.md`.
