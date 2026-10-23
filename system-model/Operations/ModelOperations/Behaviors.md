# Elements

### Create Element Override Behavior

Atomic full-definition replacement for the override branch of the shared element authoring operation.

#### Details

##### Selection and Replacement Scope
- `override_existing: true` selects an existing element by the globally unique name in the supplied full Markdown definition. If the name is absent, use ordinary creation and its existing validation rules.
- For an existing name, retain its stable name-derived identity. A same-file replacement retains the complete identifier; a cross-file override retains the stable identity and rewrites location-based references to the new identifier.
- The supplied definition replaces the target's authored prose, metadata, and structured declarations. Omitted target-authored declarations are not implicitly merged back. Generated inverse relations and declarations authored on other elements are not part of that replacement payload.
- This operation does not introduce a text-only update or append API. Renaming remains the responsibility of the rename operation.

##### Incoming Relationships and Complete-Candidate Validation
- Retain all externally authored incoming relations, including child `derivedFrom`, capability specification, contract ownership, ontology hierarchy, verification hierarchy, and verification links, whenever compatible with the complete replacement candidate.
- Retain incoming Contract References, Contract Bindings, and Concept References to the replaced element. Preserve each declaration on its original authoring element, with current target names and source-relative destinations; maintain one logical relationship regardless of authored inverse direction.
- Replace the definition within one private candidate. Recompute generated inverses, relation target context, hierarchy ownership, governance inheritance, and graph adjacency from that candidate; do not treat the temporary absence of the old definition as a deletion or run deletion cleanup against its incoming links.
- A valid same-identity override succeeds for parents with children. Actual deletion retains orphan protection. A replacement that changes type, ownership, outgoing hierarchy, or semantic declarations must still satisfy every retained incoming dependency and the complete model's hierarchy, cycle, contract, concept, prose-link, and semantic validation rules.
- Preserve existing ontology rebasing: changes to `ontology_base` or `ontology_prefix` rewrite dependent boundaries, inherited prefixes, imports, and reachable SHACL references in the same validated candidate.

##### Preview, Persistence, and Failure
- Validate the complete candidate before generating a successful preview or accepting any write. A preview returns the same candidate diff without changing source files, accepted graph, model revision, or pending accepted changes.
- Persist the replacement and any required incoming-reference or ontology rewrites as one canonical mutation transaction, then publish the new accepted snapshot. A same-identifier prose-only update leaves incoming declarations and unrelated source documents unchanged.
- Reject an invalid replacement or failed transaction while retaining the prior source files, accepted model, and pending accepted changes under the existing recovery guarantees. Do not ask the caller to unlink children merely to update their parent's definition.
- Report an existing-target override as `Update`, with its identifier, name, changed files, diff, and preview status through the shared result contract.

#### Concept References
  * [Atomic Model Mutation](../../Thesaurus/Thesaurus.md#atomic-model-mutation)
  * [Authored Element Definition](../../Thesaurus/Thesaurus.md#authored-element-definition)
  * [Element Identity](../../Thesaurus/Thesaurus.md#element-identity)
  * [Element Override](../../Thesaurus/Thesaurus.md#element-override)
  * [Incoming Relation](../../Thesaurus/Thesaurus.md#incoming-relation)

#### Metadata
  * type: behavior
---

### Merge Content Transformation Behavior

Content transformation rules for the merge elements operation.

#### Details
**Content Merging:**
1. For each source element in order:
   - If source has main content (before any #### subsection): append to target's `#### Details`
   - If source has `#### Details`: create `#### Merged Details (Source Name)` with that content
2. Preserve target's original content structure
3. Merged content appears after target's existing Details section (or creates one)
4. For ontology elements:
   - Rewrite each source authored Turtle block to the target's resolved ontology base, prefix, and term namespace before consolidation
   - Fold all rewritten source ontology content into the target's single `#### Ontology` block
   - Preserve the target ontology metadata and keep the merged result normalized to one ontology block per element
   - Recompute or deduplicate inherited prefix bindings, document declarations, `owl:imports`, and SHACL-reachable ontology references after merge

**Relation Deduplication:**
- Relations are deduplicated by (relation_type, target) pair
- First occurrence is kept (target's relations take precedence)
- Different relation types to same target are NOT duplicates

**Contract Bindings Deduplication:**
- Contract Bindings are deduplicated by target identifier/path
- First occurrence is kept (target's contract_bindings take precedence)

**Cross-Section Duplicate Check:**
- Before merge completes, validate no target appears in both Relations AND Contract Bindings
- If cross-section duplicate detected, abort merge with error listing duplicates
- User must resolve by removing one of the duplicates before retrying

**Relation Redirection:**
- Find all elements with relations pointing to source elements
- Update those relations to point to target element's identifier
- This includes both forward and backward relations

#### Metadata
  * type: behavior
---

### Focused Element Editing Behavior

Behavior for changing one selected content region without supplying a replacement element definition.

#### Details
- Resolve one existing element and one target in the accepted snapshot. Element names, identifiers, file locations, metadata, authored relations, contract dependencies, concept references and other protected declarations outside that target remain unchanged. Preserve incoming children, owners, consumers and generated relationships without unlinking them.
- Element Text denotes opening prose and exists even when empty. A named subsection target selects only its contents; append inserts at its end before the following structural boundary. Replace retains its heading. Create inserts a new permitted subsection with its heading through canonical serialization. Delete removes its heading and complete contents.
- Match subsection names exactly, without trimming, fuzzy matching, implicit creation or guessing a repeated occurrence. Element Text is reserved as a selector; it cannot create/delete a subsection with that name. An existing custom subsection with that name remains outside this tool's subsection address space; full-definition override remains available.
- Resolve structural boundaries using the supported document grammar and fence rules, including single-element documents, CRLF input, Unicode, fenced heading examples and permitted nested headings. A replacement fragment must not introduce a new outer subsection or element boundary, escape through an open fence or HTML block, or reinterpret unchanged neighbouring content. Create supplies its heading itself; caller content supplies contents only.
- Check the expected accepted-model revision under the same exclusive mutation gate as candidate construction. Reject stale or missing application preconditions before edits; a retry with a previously used revision cannot append twice after a successful application. A no-op edit returns no changed files and retains the revision and pending set.
- Construct a private complete candidate and use shared validation, canonical persistence, ownership, transaction recovery and selected-context publication. Reject invalid prose links, semantic declarations, types and grammar. Preview changes no files, revision, HEAD, index, pending set or accepted reads. Apply must produce the same diff for the same base and request and expose the new content on subsequent reads and a fresh validated reload.
- Preserve unrelated accepted pending edits and unrelated files. A failure retains prior accepted state under existing transaction recovery rules. Do not report an effective edit as executed merely because its request parsed; changed-file and diff evidence must agree with persisted source and accepted reads.

#### Concept References
  * [Edit Target](../../Thesaurus/Thesaurus.md#edit-target)
  * [Focused Element Edit](../../Thesaurus/Thesaurus.md#focused-element-edit)

#### Metadata
  * type: behavior
---
