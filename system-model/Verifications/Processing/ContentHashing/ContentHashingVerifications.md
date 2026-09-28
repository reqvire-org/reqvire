# Elements

### Stable Content Hashing Verification Objective

This objective groups verification of the shared SHA-256 primitive and versioned model revision encoding before their implementation is accepted.

#### Details
Concrete test verifications define required evidence and link executable tests as they become available. An evidence link identifies a test artifact; acceptance requires its assertions to pass. Do not link planned modules or unimplemented assertions as evidence. Existing Explorer runtime and browser refresh verifications remain in their interface objective and own wire-format compatibility and client/server integration checks.

#### Metadata
  * type: verification-objective
---

### Model Revision Hash Stability Verification

Verify the complete versioned canonical encoding and model revision migration contract.

#### Details

##### Acceptance Criteria
- Fixed fixtures pin canonical bytes and independently established SHA-256 digests for an empty model, multiple elements, Unicode, and empty collections.
- Repeated runs and identical canonical inputs in different absolute workspace directories produce identical revisions. Fixtures pin big-endian lengths, counts, string framing, and both version markers.
- Representative changes to every included field change canonical bytes and the expected digest, including each governance field, ontology/concept namespace metadata, target kind, relation, and contract binding.
- Addition, removal, rename, and movement of elements change the revision as specified.
- Permuting elements, metadata, relations, or bindings preserves the encoding. Duplicate effective relations/bindings contribute once; authored/generated edge provenance does not change the record. Changing an effective edge updates the affected endpoint records.
- Length and count framing distinguish ambiguous concatenations, empty values, and different tuple boundaries.
- Changes to meaningful whitespace, including inside literals, are retained. Excluded runtime fields, parser bookkeeping, page frontmatter, and referenced external-file bytes do not change the revision by themselves.
- Resolved internal paths use workspace-relative `/` separators; non-UTF-8 paths are rejected without lossy hashing. External URL text is preserved.
- Existing affected tool responses retain their field names and shapes, carry 64 lowercase hexadecimal characters, and agree for the same snapshot. Clients cannot expect old 16-character values to map to new revisions.
- The model encoder and existing Explorer wire hashing use the same core primitive while retaining their distinct input bytes. Existing impact-analysis, model-cache file, and identifier hash behavior is unchanged.

##### Required Evidence
The linked `test-model-revision-hashing` E2E suite exercises the real HTTP MCP server with fixed canonical bytes and independently calculated digests for empty, Unicode, and multi-element models. It checks governance and namespace metadata edits, content and literal whitespace, element addition/removal/rename/movement, element/metadata/relation/binding order, equivalent forward/inverse authoring, relation labels and removal, binding target changes, and excluded page annotations, artifact bytes, size estimates, and absolute workspace location. Repeated calls and a new server process must agree. The existing fingerprint fields in workspace and semantic tool responses must agree with the revision for the same snapshot.

The linked Rust encoder tests pin canonical bytes and independent digests, including a record with all target kinds, nonempty relations and bindings, Unicode, and an empty metadata value. They check individual field changes, target-kind distinctions, ambiguous concatenations, empty values and tuple boundaries, deduplication of effective tuples, parser bookkeeping and runtime-field exclusions, generated inverse edges, exact URL text, and rejection of absolute, unresolved, and non-UTF-8 paths. Duplicate authored relations are rejected before the E2E hashing path, so encoder deduplication is exercised directly in Rust. The ontology-base E2E mutation also selects an already-declared matching prefix, keeping content unchanged; Rust tests isolate each metadata field. The E2E suite additionally checks the advertised fingerprint schemas and rejection of non-UTF-8 source paths before parsing loses their original bytes. The shared primitive's known-answer verification and existing Explorer runtime verification own primitive correctness and wire-hash compatibility respectively.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Stable Content Hashing Verification Objective](#stable-content-hashing-verification-objective)
  * satisfiedBy: [tests.rs](../../../../crates/reqvire-core/src/model_revision/tests.rs)
  * satisfiedBy: [test.sh](../../../../tests/test-model-revision-hashing/test.sh)
  * verify: [Stable Model Revision Fingerprinting](../../../Processing/ContentHashing/Requirements.md#stable-model-revision-fingerprinting)
---

### SHA-256 Known-Answer Verification

Verify the exact-byte digest contract of the shared core hashing primitive.

#### Details

##### Acceptance Criteria
- Empty input produces `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
- UTF-8 `abc` produces `ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad`.
- Binary input, embedded zero bytes, Unicode, whitespace, and line-ending differences are hashed as exact bytes. The input buffer remains unchanged.
- Every output is exactly 64 lowercase hexadecimal characters with no prefix.

##### Required Evidence
The linked core hashing module contains fixed known-answer vectors for empty input, `abc`, binary input with zero bytes, Unicode, spaces, and LF/CRLF line endings. Assertions check exact digests, lowercase hexadecimal width, and unchanged input buffers. The existing Explorer `abc` test checks caller compatibility and complements the shared primitive tests.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Stable Content Hashing Verification Objective](#stable-content-hashing-verification-objective)
  * satisfiedBy: [hashing.rs](../../../../crates/reqvire-core/src/hashing.rs)
  * verify: [SHA-256 Content Fingerprinting](../../../Processing/ContentHashing/Requirements.md#sha-256-content-fingerprinting)
---
