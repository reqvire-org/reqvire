These fixed vectors implement Model Revision Hash Specification v1 independently
of Reqvire. The test reads them; it never regenerates them from tool responses.

Each .hex file contains the complete canonical model stream, with a newline
every 32 bytes for readability. Its companion .sha256 contains the SHA-256
digest of the decoded bytes, independently calculated with Python hashlib.
The test verifies fixture integrity before comparing the real MCP revision.

Strings and record buffers have unsigned 64-bit big-endian byte-length prefixes.
Collections have unsigned 64-bit big-endian item-count prefixes. The model
marker is reqvire.model-revision.v1; each record marker is reqvire.element.v1.

empty: 41 bytes, model marker and zero elements.
minimal: 340 bytes, one record for Model.md#hash-subject, name Hash Subject,
type capability, content "Café 測定." (13 UTF-8 bytes), path Model.md. Metadata
tuples in order: owner/team-a, priority/medium, risk/low, status/draft,
type/capability. Relations and bindings are empty.
multiple: 382 bytes, records for Model.md#alpha and Model.md#omega in that order.
Names Alpha and Omega, type capability, content "First." and "Last.", path
Model.md. Each has only type/capability metadata and empty relations/bindings.
rich: 648 bytes, used by Rust encoder tests. Like minimal with type requirement
(both canonical type and metadata), an extra note/empty-string metadata tuple,
and these sorted tuples:
  Relations: satisfiedBy/external_url/https://example.test/A%2fb?q=a%20b#C,
             satisfiedBy/internal_path/artifacts/a.txt,
             specify/identifier/Model.md#root.
  Bindings: identifier/Other.md#spec, internal_path/contracts/spec.txt.

Expected checks are all PASS. Failures must remain failures until the production
implementation meets the specification; do not copy current output here.
