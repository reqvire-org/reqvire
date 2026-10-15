# Token samples

Project-owned token review pattern. Receives literal token names and sample text
from colocated scenario fixtures, and reads values from the real foundation CSS.
Theme changes refresh computed values. The measured sample boundary writes only
`--ux-token-sample-value`; presentation remains in this pattern's Linaria styles.

Adapted from the DS working example's TokenSamples pattern by Ilija Ljubicic,
https://github.com/reqvire-org/ads. The MIT notice is retained in
[LICENSE.source.md](LICENSE.source.md). Uses native Preact state in this project.
