# Shared flow canvas styling

`flowCanvasClasses()` is the public Linaria treatment for the installed React Flow renderer root. TraceFlow and ThesaurusExplorer apply it through `className`; it adds no wrapper DOM or renderer dependency.

The shared base cascade layer owns viewport, pan, node, edge, handle, selection and attribution structure. Graph-specific custom-node and edge skins remain with their existing patterns and take precedence. All visual roles use project foundation tokens. Built-in minimap, resize controls and built-in node skins are outside this custom-canvas API.

## References and credits

Adapted from `@xyflow/react` 12.12.0 base/style CSS, copyright 2019–2025 webkid GmbH, under MIT. See [the retained notice](LICENSE.xyflow.md), [source](https://github.com/xyflow/xyflow) and [theming documentation](https://reactflow.dev/learn/customization/theming). React Flow JavaScript remains an installed, declared dependency.

Registered TraceFlow layout/expansion checks and Thesaurus pan/zoom/drag checks exercise the real renderer. Validate both light and dark themes in ADS Explorer.
