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
- Content comparison policies that strip whitespace do not define this projection: `"a b"` and `"ab"` remain distinct content. Formatting retained by the parser affects the revision; formatting discarded during parsing does not.

##### Projection boundary
- The revision covers the complete selected parsed-element collection. It excludes page frontmatter, line numbers, within-file element order, Git state, absolute workspace location, size estimates, change-impact flags and hashes, inherited governance projections, external referenced-file contents, and generated output bytes.
- Excluded-value changes alone need not change the revision. Source cache freshness and complete response invalidation continue to account for additional inputs under their owning contracts.
- Added or removed elements change the encoded collection. Moves and renames change identifiers or paths. Relocating the unchanged workspace preserves the revision when canonical relative paths and other inputs are identical.
- A parent's authored governance change changes the model revision even though the child's own canonical record may stay unchanged. This model revision does not define a dependency-aware per-element cache key.

##### Consumer responsibilities and compatibility
- Subsystems comparing or identifying parsed-element snapshots shall use this complete versioned projection and shared encoder. Their requirements bind this specification and own the field names, messages, storage, and invalidation behavior through which revisions are exposed.
- Returned revisions use the digest representation defined by the bound SHA-256 contract. Consumers treat them as opaque equality tokens for this projection; revisions from a different encoding are not interchangeable without an explicit compatibility contract.
- Format markers belong to the hashed bytes; returned digests remain unprefixed. Pin the canonical bytes and digests with fixed fixtures. A change to field selection, encoding, or canonicalization requires a new encoding version and documented cache invalidation.
- Parser changes that alter canonical inputs can change the revision. Cross-version stability applies when the encoding version and canonical inputs remain identical.
- Subsystems identifying generated artifacts or transmitted payloads define their own exact-byte input contracts. Equality of a parsed-element projection does not imply equality of those outputs, and their digests need not equal the model revision.

##### Deferred consumers
Subsystems requiring per-element comparison shall reuse the complete versioned element record and SHA-256 primitive. Public exposure, identity matching across moves or federated copies, snapshot-consistent collections, and dependency-aware invalidation require separate consumer contracts. This specification introduces no per-element interface field and does not persist fingerprints in authored Markdown. Incremental generation also needs generator, configuration, and dependency tracking beyond this revision contract.

#### Metadata
  * type: specification

#### Relations
  * define: [Stable Model Revision Fingerprinting](Requirements.md#stable-model-revision-fingerprinting)
---

### SHA-256 Hash Encoding Specification

Shared exact-byte hashing contract for content comparison, addressing, and integrity verification.

#### Details

##### Primitive contract
- Algorithm: SHA-256 as defined by FIPS 180-4.
- Input: an arbitrary byte slice; an empty slice is valid. Hash every supplied byte exactly once and in order.
- Output: 64 lowercase hexadecimal characters with no prefix.
- Identical bytes produce identical digests across platforms, locales, absolute workspace locations, and Rust compiler versions.
- The primitive does not mutate, trim, normalize, canonicalize, or interpret its input. Consumer-specific serialization occurs before the primitive is called.
- Shared native-library API: `pub fn sha256_hex(bytes: &[u8]) -> String`.

##### Consumer responsibilities
- Native subsystems producing SHA-256 content fingerprints shall reuse this shared primitive. This includes subsystems for state comparison, content-addressed storage or transfer, generated-artifact identification, and content-integrity verification.
- Consuming requirements bind this specification. Each consumer separately defines its input projection, serialization, and compatibility policy in contracts that it owns.
- The shared implementation owns the digest algorithm, hexadecimal encoding, and native hashing dependency. Consumer adapters select input bytes and do not implement a second native digest or hexadecimal encoder.
- A semantic-state consumer defines a versioned canonical encoding before hashing. An exact-content or transmitted-payload consumer hashes the authoritative serialized bytes; parsing and reserialization are not substitutes for those bytes.
- Verifiers in independent execution environments may use their platform's SHA-256 implementation while conforming to the same exact-byte and output-encoding contract.
- Adopting the shared primitive preserves existing digest values when the supplied bytes are unchanged. Changes to consumer serialization, identifiers, transport fields, storage, publication, or refresh policies require the consumer's own contract changes.

##### Scope
- This contract governs consumers that require SHA-256 content fingerprints; it does not redefine other hash algorithms or equality policies.
- Hashes are derived values. Persisting, exposing, or embedding a digest requires a separate consumer contract; the primitive does not modify authored content or generated artifacts.
- Command surfaces, interface fields, artifact annotations, signatures, and automatic regeneration are consumer responsibilities outside this primitive contract.

#### Metadata
  * type: specification

#### Relations
  * define: [SHA-256 Content Fingerprinting](Requirements.md#sha-256-content-fingerprinting)
---
