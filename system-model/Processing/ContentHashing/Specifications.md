# Elements

### Model Revision Hash Specification

Canonical encoding and compatibility contract for the parsed-element model revision.

#### Details

##### Encoding version and framing
- Hash explicit canonical bytes using the shared SHA-256 primitive. Standard-library `Hash` encodings and direct serialization of the runtime `Element` structure are not the revision format.
- Encode each string as UTF-8 preceded by its byte length as an unsigned 64-bit big-endian integer. Encode a byte buffer with the same length framing.
- Encode each collection as its unsigned 64-bit big-endian item count followed by its encoded items. Tuple fields use the order specified below.
- Sort strings by UTF-8 bytes and tuples lexicographically by their component strings, without locale-dependent ordering or Unicode normalization.
- The model stream begins with the framed string `reqvire.model-revision.v1`, followed by a counted collection of elements sorted by canonical identifier. Each element is a length-framed buffer containing its canonical record.
- Each element record begins with the framed string `reqvire.element.v1`, followed by the fields below in this order. Keep the record encoder reusable without exposing a public per-element fingerprint API.

| Field | Canonical value |
| --- | --- |
| Identifier | Resolved workspace-root-relative identifier, including fragment |
| Name | Parsed element name |
| Type | Canonical element type token |
| Content | Parsed `element.content` encoded as a framed UTF-8 string |
| File path | Workspace-root-relative source file path |
| Metadata | Counted `(key, value)` string tuples, sorted by key, covering all authored metadata |
| Relations | Counted, sorted, deduplicated `(relation_type, target_kind, target)` string tuples from the resolved element graph |
| Contract bindings | Counted, sorted, deduplicated `(target_kind, target)` string tuples |

##### Field semantics
- Metadata includes governance (`status`, `priority`, `risk`, `owner`), ontology and concept namespace metadata, `type`, and other authored keys. Exclude the parser-generated `_single_element_format` marker. Future internal metadata must remain outside the authored projection.
- Relation target kinds are `identifier`, `internal_path`, and `external_url`. Contract-binding target kinds are `identifier` and `internal_path`.
- Resolved internal paths and identifiers are workspace-root-relative with `/` separators. External URLs are preserved verbatim. Paths that cannot be represented losslessly as UTF-8 are rejected, rather than encoded as empty or lossy replacements. Resolved path fields must not include the absolute workspace directory.
- Relations include both authored edges and generated inverse edges. Exclude `user_created`, display labels, and cached target IDs. Identical effective tuples contribute once. An edge change can therefore change both endpoint records.
- Contract bindings encode targets and exclude cached referenced-file content hashes.
- Preserve the parser's stored content, including ontology/shapes blocks and concept-reference content. Apply no further whitespace removal, trimming, Markdown rewriting, or RDF canonicalization.
- Do not reuse the whitespace-stripped `hash_impact_content`: `"a b"` and `"ab"` remain distinct content. Formatting retained by the parser affects the revision; formatting discarded during parsing does not.

##### Projection boundary
- The revision covers the complete selected parsed-element collection. It excludes page frontmatter, line numbers, within-file element order, Git state, absolute workspace location, size estimates, change-impact flags and hashes, inherited governance projections, external referenced-file contents, and generated output bytes.
- Excluded-value changes alone need not change the revision. Source cache freshness and complete response invalidation continue to account for additional inputs under their owning contracts.
- Added or removed elements change the encoded collection. Moves and renames change identifiers or paths. Relocating the unchanged workspace preserves the revision when canonical relative paths and other inputs are identical.
- A parent's authored governance change changes the model revision even though the child's own canonical record may stay unchanged. This model revision does not define a dependency-aware per-element cache key.

##### Consumer migration and compatibility
- Replace the `DefaultHasher` accumulation used by `model_fingerprint` in `tool_interface/arg_helpers.rs` with this one shared model revision computation. All existing callers use the same projection and encoder.
- Retain existing `model_fingerprint` and `fingerprint` field names and response shapes. The digest changes from 16 to 64 lowercase hexadecimal characters; there is no old-to-new mapping.
- Update shared interface contracts, output schemas, examples, and fixtures for affected workspace and semantic tools. Release notes identify the new encoding version, metadata coverage, exclusions, and one-time cache invalidation.
- Format markers belong to the hashed bytes; returned digests remain unprefixed. Pin the canonical bytes and digests with fixed fixtures. A change to field selection, encoding, or canonicalization requires a new encoding version and documented cache invalidation.
- Parser changes that alter canonical inputs can change the revision. Cross-version stability applies when the encoding version and canonical inputs remain identical.
- Explorer's manifest revision retains its existing exact-wire-byte input and is not required to equal the model revision. Model-source equality and generated-artifact equality have separate input contracts and share only the hashing primitive.

##### Deferred consumers
Per-element fingerprinting is a designated future consumer of the same complete versioned element record and SHA-256 primitive. Public exposure, identity matching across moves or federated copies, snapshot-consistent manifests, dependency-aware invalidation, and hash fields in model/search/read responses are outside this implementation slice. Fingerprints are not persisted in authored Markdown. Incremental generation also needs generator, configuration, and dependency tracking beyond this revision contract.

#### Metadata
  * type: specification

#### Relations
  * define: [Stable Model Revision Fingerprinting](Requirements.md#stable-model-revision-fingerprinting)
---

### SHA-256 Hash Encoding Specification

Shared byte hashing and integration boundary for model revision and generated artifact consumers.

#### Details

##### Primitive contract
- Algorithm: SHA-256 as defined by FIPS 180-4.
- Input: an arbitrary byte slice; an empty slice is valid. Hash every supplied byte exactly once and in order.
- Output: 64 lowercase hexadecimal characters with no prefix.
- Identical bytes produce identical digests across platforms, locales, absolute workspace locations, and Rust compiler versions.
- The primitive does not mutate, trim, normalize, canonicalize, or interpret its input. Consumer-specific serialization occurs before the primitive is called.
- Public core API: `pub fn sha256_hex(bytes: &[u8]) -> String`, owned by `reqvire-core` in `crates/reqvire-core/src/hashing.rs` and exported through the core library.

##### Existing Explorer implementation consolidation
- Extract the existing SHA-256 implementation in `crates/reqvire-cli/src/live_store.rs` into the shared core primitive. All Rust SHA-256 content consumers, including existing Explorer chunks, ontology output, and manifest revision generation, call that primitive.
- Declare `sha2` once in workspace dependencies and consume it from `reqvire-core`. Remove the CLI's direct SHA-256 implementation and unused direct dependency when migrating its caller. Consumer adapters may select input bytes but must not implement a second digest or hexadecimal encoder.
- Explorer keeps responsibility for JSON serialization, manifest assembly, chunk storage, and published snapshot lifecycle. Those behaviors remain owned by its existing runtime and live-store contracts.
- For identical serialized JSON, ontology, and manifest bytes, Explorer emits exactly the same hashes, manifest protocol, and ETags as before primitive extraction. The model revision encoder does not replace Explorer wire-byte hashing.
- The existing browser SHA-256 verifier remains an independent implementation of the same byte contract in the browser runtime; it verifies received bytes before parsing and does not need to execute Rust code.
- Model revision and future artifact consumers reuse this same core function. Each consumer documents its input projection and encoding separately.

##### Scope
- Existing FxHasher-based impact-content, model-cache file, and identifier hashes retain their existing contracts in this implementation slice.
- Hashes remain derived values. No SHA-256 field is persisted in authored Markdown, and no artifact header stamping or signature behavior is added.
- This contract adds no command, flag, or per-element API field. Managed SPARQL construct export and additional artifact drift workflows are later consumers.

#### Metadata
  * type: specification

#### Relations
  * define: [SHA-256 Content Fingerprinting](Requirements.md#sha-256-content-fingerprinting)
---
