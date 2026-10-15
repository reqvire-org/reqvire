# Project branding

Keep original logos, app icons, and artwork variants here. This starter contains
no artwork. Record each asset's purpose, suitable backgrounds, usage, source
credits, and applicable licenses when adding it.

UI branding uses the fixed DS token names in `tokens/`: change values such as
`--accent`, `--font-sans`, and `--radius-md`. Components own `--ds-*` variables;
patterns own `--ux-*`; the showcase owns `--showcase-*`. Do not introduce a
brand-prefixed token vocabulary or override component state variables.

Put licensed font files in `assets/fonts/` and their declarations in
`tokens/fonts.css`. Keep that optional import first in `styles.css`.

To generate web icons, set `assets.iconSource` in the project's `ds.config.json`
to the SVG path relative to the DS folder, for example:

```json
{ "assets": { "iconSource": "assets/brand/app-icon.svg" } }
```

After `ds init`, the normal npm or Bun build generates `favicon.svg`,
`favicon.ico` (16/32/48), `apple-touch-icon.png` (180), `icon-192.png`, and
`icon-512.png` in ignored `.ds/assets/`. Run `ds assets` to regenerate manually.
Outputs retain the artwork's colors and transparent background; provide an
appropriate source background if a platform needs one. Mount this directory
through the application's existing bundler and keep app manifest metadata in
the application. Edit the original SVG, never generated variants.
