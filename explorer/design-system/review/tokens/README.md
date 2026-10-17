# Token samples

Project-owned token review helper. Receives literal `{ token, property, probe }`
samples and text from colocated scenario fixtures, and reads values from the real foundation CSS.
Theme changes refresh computed values. The measured sample boundary writes only
`--ux-token-sample-value`; presentation remains in this pattern's Linaria styles.
Each sample declares `data-ads-token` and `data-ads-property` on the element that
consumes the token. Property roles come from fixtures, including geometry, grids,
filters, type and motion. The App verifies that the fixture probe changes the
sample's computed property. Theme observation depends on the stable fixture
samples, so updating displayed values does not create a render loop.

Adapted from the DS working example's TokenSamples pattern by Ilija Ljubicic,
https://github.com/reqvire-org/ads. The MIT notice is retained in
[LICENSE.source.md](LICENSE.source.md). Uses native Preact state in this project.
