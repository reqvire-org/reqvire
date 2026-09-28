# Elements

### Stable Content Hashing Verification Objective

This objective groups verification of the shared SHA-256 primitive and versioned model revision encoding before their implementation is accepted.

#### Details
Concrete test verifications define required evidence and relate executable artifacts through satisfiedBy. An evidence relation identifies a test artifact; acceptance requires its assertions to pass. Consumer-specific wire-format compatibility and integration checks belong to the consuming subsystem's verification hierarchy.

#### Metadata
  * type: verification-objective
---

### Model Revision Hash Stability Verification

Verify the complete versioned canonical encoding and model revision stability contract.

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
- The returned model revision carries 64 lowercase hexadecimal characters with no prefix and agrees across consumers for identical canonical inputs.
- The model encoder uses the shared primitive over the specified canonical bytes. Generated-artifact equality, source-cache freshness, and other hash policies remain outside the parsed-element projection.

##### Required Evidence
Use independently established canonical byte and digest fixtures for empty, Unicode, and multi-element models, all target kinds, nonempty relations and bindings, and empty metadata values. Compare repeated requests and separate processes over identical snapshots. Isolate each included field and each exclusion so an unrelated change cannot supply the expected digest difference.

Exercise both direct record encoding and parsed-model construction. Direct encoding establishes tuple deduplication and invalid-path rejection independently of parser rules. Parsed-model cases establish resolved identifiers, generated inverse edges, metadata/content boundaries, workspace relocation, and rejection of non-UTF-8 source paths before information can be lost. Consumer interface compatibility and source-cache visibility are verified under their own requirements.

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
Use fixed known-answer vectors for empty input, `abc`, binary input with zero bytes, Unicode, spaces, and LF/CRLF line endings. Assert exact digests, lowercase hexadecimal width, and unchanged input buffers. Consumer integration checks remain owned by the consuming subsystem's verifications.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Stable Content Hashing Verification Objective](#stable-content-hashing-verification-objective)
  * satisfiedBy: [hashing.rs](../../../../crates/reqvire-core/src/hashing.rs)
  * verify: [SHA-256 Content Fingerprinting](../../../Processing/ContentHashing/Requirements.md#sha-256-content-fingerprinting)
---
