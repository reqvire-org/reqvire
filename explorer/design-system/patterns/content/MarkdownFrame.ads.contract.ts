import type { UnitContract } from "@reqvire-org/ads/contracts";

export default {
  schemaVersion: 1,
  // An interactive diagram's frame height is measured from the layout; the rest is CSS.
  styles: { writes: { "./MarkdownFrame.tsx": ["--ux-markdown-diagram-height"] } },
} satisfies UnitContract;
