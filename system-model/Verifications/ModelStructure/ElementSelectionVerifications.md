# Elements

### Element Selection Verification Objective

Verify that existing-element references select the same model subjects through CLI, MCP and core operations while preserving context, argument domains, type restrictions and mutation safety.

#### Details
The verification below has executable black-box assertions in the existing owning E2E suites and direct core contract checks. Exact names remain controls for identifier parity. Test-source links identify the required assertions; implementation evidence belongs to the requirements.

#### Metadata
  * type: verification-objective

#### Relations
  * derivedFrom: [Model Parsing and Structure Verification Objective](ParsingVerifications.md#model-parsing-and-structure-verification-objective)
  * derive: [Existing Element Selection Verification](#existing-element-selection-verification)
---

### Existing Element Selection Verification

Verify interchangeable existing-element selection across the shared resolver and its operation consumers.

#### Details

##### Executable Acceptance Criteria
- Direct core checks exercise both lookup interpretations selecting one subject, exact case and whitespace handling, deterministic ambiguity, contradictory selectors, literal-name helpers, explicit identifier-only fields and separate registry snapshots. Every affected MCP selector description and CLI help for all managed-query actions advertises the shared domain.
- The existing model, submodels, scoped-coverage and collect suites compare exact-name controls with canonical identifier selections. Cover root/nested capabilities, requirement scopes where permitted, and upstream/downstream collection for capability, requirement, ontology, semantic-query, concept-scheme and concept roots. Compare complete structured results, including selected subjects and evidence; reads preserve fixture and Git state.
- Managed-query tests compare `--name` with canonical source identifiers for CLI list, validate, export and artifact check, and MCP discovery, validation and content export. Explicit semantic IRIs remain a separate domain; artifact checks use authored paths and retain matching hashes.
- Standalone and embedded MCP compare name/identifier selection through `read_element` and native concept/scheme `get`. Consistent explicit selectors select one subject; contradictory element and native-concept selectors must return structured errors instead of choosing a field silently.
- CLI/MCP mutation previews compare name controls with identifiers in every subject, each merge source member/target, link/unlink endpoints, relation relink source/old/new targets, and Contract Binding/Reference link/unlink and Reference relink endpoints. Each identifier is exercised independently and all selectors together. A nested source file exercises canonical endpoints without rebasing them as source-relative paths. Binding target identifiers remain a passing control; authored contract target names must also select that contract.
- A valid fixture contains one element whose exact name equals another element's canonical identifier. Model, submodels, coverage, collect and element reads must reject this ambiguity. CLI rename and MCP preview/applied rename must reject ambiguous subjects before persistence; diagnostics identify both canonical candidates.
- Compare mixed name/identifier merge duplicate and self-operation rejection with the owning operation's exact-name behavior. An unknown later merge source must not partially persist earlier subjects. Known wrong-type coverage/merge identifiers must be distinguished from missing elements.
- For real standalone/embedded mutation-enabled MCP under both automatic-commit policies, preview/rejection checks compare physical file bytes and modes, raw index bytes and logical entries, HEAD, accepted search/revision/workspace status and the embedded Explorer runtime manifest. Unknown selection after an earlier accepted write preserves pending changes. A subsequent name-based mutation proves the session remains usable.
- An accepted identifier rename must publish the renamed subject under the configured commit policy. Its old identifier must then fail without changing accepted state. CLI additionally checks that a `new_name` which resembles another element's identifier remains a literal authored name.
- With two registered worktree contexts containing the same name and relative identifier but different accepted content, select each context explicitly and compare its own subject. Reject a child-only identifier in the origin context. Writable sessions retain accepted content, evidence, model revision and context identity despite a physical edit while reporting live Git dirtiness; read-only sessions resolve from the refreshed content.
- Report/query CLI help and representative MCP selector discovery must advertise exact names and canonical identifiers while retaining existing keys. Schema-valid MCP selection failures must return `isError` with structured tool identity/cause and the rejected value where asserted. Search regex filters and explicit semantic/native concept IRIs retain their separate domains.
- All expected check manifests contain PASS. The helper accumulates failures and exits nonzero; there is no expected-failure exemption. Owning suites run their existing assertions before the new selector assertions. Explorer visual comparisons are outside this verification. Coverage URL navigation requires separate browser E2E when implemented.

##### Remaining Verification Work
- Broaden accepted mutation and stale-identifier checks from representative rename to move, remove, merge and every endpoint operation. Add held reads during publication and recovery-required identifier selection to the existing concurrency/recovery suites.
- Complete the excluded-input matrix for authored add/override headings, every path/URL branch, CURIEs, namespace filters, query text, Git refs and worktree IDs. The owning existing tests still cover those domains; the selector helper currently adds explicit literal-name, regex and IRI controls.

These additional verification cases do not exempt the existing executable assertions from passing.

#### Metadata
  * type: test-verification

#### Relations
  * derivedFrom: [Element Selection Verification Objective](#element-selection-verification-objective)
  * satisfiedBy: [element_selection.rs](../../../crates/reqvire-core/src/element_selection.rs)
  * satisfiedBy: [definitions.rs](../../../crates/reqvire-core/src/tool_interface/definitions.rs)
  * satisfiedBy: [element_selection_checks.py](../../../tests/element_selection_checks.py)
  * satisfiedBy: [test.sh](../../../tests/test-model-command/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-submodels-command/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-scoped-coverage/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-collect-command/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-semantic-queries/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-crud-manipulation/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-merge-elements/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-link-unlink/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-relink-command/test.sh)
  * satisfiedBy: [test.sh](../../../tests/test-mcp-ownership/test.sh)
  * verify: [Existing Element Reference Resolution](../../ModelStructure/ElementSelection.md#existing-element-reference-resolution)
  * verify: [MCP Managed Query Element Selection](../../Interfaces/MCP/ManagedQueries.md#mcp-managed-query-element-selection)
  * verify: [CLI Managed Query Element Selection](../../Interfaces/CLI/ManagedQueries.md#cli-managed-query-element-selection)
  * verify: [Managed Query Element Selection](../../Semantics/SemanticModelRequirements.md#managed-query-element-selection)
  * verify: [MCP Model Evidence Element Selection](../../Interfaces/MCP/Tools.md#mcp-model-evidence-element-selection)
  * verify: [CLI Collect Element Selection](../../Interfaces/CLI/Commands.md#cli-collect-element-selection)
  * verify: [Coverage Element Scope Selection](../../Reports/ModelReports/ReportingRequirements.md#coverage-element-scope-selection)
  * verify: [Managed Query Drift Check Element Selection](../../Semantics/SemanticModelRequirements.md#managed-query-drift-check-element-selection)
---
