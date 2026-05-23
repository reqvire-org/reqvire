# Documents

## Metadata
  * type: specification

## Relations
  * refine: [Search Report Generator](../Reporting.md#search-report-generator)

## SearchFiltering

# Search Filtering Specification

## Summary

This specification defines the functional requirements for a filtering subsystem used within the `search` reporting feature. The system must allow clients to selectively include or exclude elements from the search output based on metadata, content, and traceability properties.

The filters shall be composable and applied conjunctively (i.e., all active filters must match for an element to be included). The filtering system must support both human-readable text output and structured machine-readable output (e.g., JSON), as well as abbreviated short mode output.

Full search summaries include governance metadata counters for matched governance-bearing elements (`feature` and `requirement`). The counters use effective metadata after explicit, inherited, and default resolution.

---

## Filtering Scope

Filtering shall operate on the level of individual `Element` objects in the model registry. Each `Element` has the following relevant properties:

- `file_path: String`
- `name: String`
- `element_type: ElementType`
- effective requirement governance metadata (status, priority, risk, owner) when applicable
- `content: String`
- `relations: Vec<Relation>`
- `page_content: String` (from parent file)

---

## Supported Filters

The filtering system **must support the following filters**, which may be active simultaneously.

### 1. File Path Filter (Glob)

**Purpose:** Restrict search to elements defined in files whose paths match a given glob pattern.

**Input:** A single string pattern using glob syntax (e.g., `"src/**/*Spec.md"`)

**Match Target:** `Element.file_path`

**Behavior:** Case-sensitive glob match. If the glob does not match any file, no elements are included.

---

### 2. Name Filter (Regex)

**Purpose:** Include only elements whose `name` matches a regular expression.

**Input:** A valid Rust-compatible regular expression (e.g., `"autonomous.*"`)

**Match Target:** `Element.name`

**Behavior:** Case-sensitive match by default. The filter is considered invalid if the regex fails to compile.

---

### 3. Type Filter (Exact Match)

**Purpose:** Include only elements of a specific type.

**Input:** One of the following valid string identifiers:

- `"feature"` - Product/capability feature
- `"requirement"` - System requirement (default type)
- `"verification"` / `"test-verification"` - Test verification
- `"analysis-verification"` - Analysis verification
- `"inspection-verification"` - Inspection verification
- `"demonstration-verification"` - Demonstration verification
- `"formal-proof-verification"` - Formal proof/model-checking/theorem verification
- `"source"` - Feature-owned source/context refinement
- `"semantic-contract"` - Requirement-owned SHACL semantic contract refinement
- `"constraint"` - Refinement documenting constraints
- `"behavior"` - Refinement documenting behavior details
- `"specification"` - Refinement documenting specifications
- `"state"` - Refinement documenting lifecycle state
- `"input-output"` - Refinement documenting payloads, messages, fixtures, or data contracts
- `"file"` - File element
- Any user-defined type (e.g., `"interface"`, `"design"`)

**Match Target:** `Element.element_type`

**Behavior:** Matching must be exact. Internally, the filter string shall be mapped to an `ElementType` via a deterministic lookup function.

---

### 4. Governance Metadata Filters

**Purpose:** Include only governance-bearing elements whose effective governance metadata matches the requested values.

**Inputs:**
- `filter-status`: comma-separated list of accepted status values (`draft`, `review`, `approved`)
- `filter-priority`: comma-separated list of accepted priority values (`low`, `medium`, `high`, `critical`)
- `filter-risk`: comma-separated list of accepted risk values (`low`, `medium`, `high`, `critical`)
- `filter-owner`: Rust-compatible regular expression applied to effective owner value

**Match Target:** Effective governance metadata after explicit, inherited, and default resolution.

**Behavior:** Status, priority, and risk filters apply exact OR matching within each comma-separated filter. Owner filtering applies regex matching. Invalid governance enum values and invalid owner regex patterns must cause an immediate user-facing error. Elements that are not governance-bearing elements are excluded when any governance metadata filter is active.

---

### 5. Content Filter (Regex)

**Purpose:** Include only elements whose body content matches a regular expression.

**Input:** A valid regex pattern applied to the element's `content`.

**Match Target:** `Element.content`

**Behavior:** Case-sensitive regex match. Invalid patterns must cause an immediate user-facing error.

---

### 6. Page Content Filter (Regex)

**Purpose:** Include only elements whose parent file's page content (frontmatter) matches a regular expression.

**Input:** A valid regex pattern applied to the file's page content.

**Match Target:** Page content (frontmatter) of the element's parent file

**Behavior:** Case-sensitive regex match. Invalid patterns must cause an immediate user-facing error.

---

### 7. Have Relations Filter (Comma-separated list)

**Purpose:** Include only elements that have ALL specified relation types.

**Input:** Comma-separated list of relation type names (e.g., `"verifiedBy,satisfiedBy"`)

**Match Target:** `Element.relations`

**Behavior:** When specified, element must have at least one relation of each specified type to be included. Invalid relation type names shall cause an error with a list of valid relation types.

---

### 8. Not Have Relations Filter (Comma-separated list)

**Purpose:** Include only elements that do NOT have ALL specified relation types.

**Input:** Comma-separated list of relation type names (e.g., `"verifiedBy"`)

**Match Target:** `Element.relations`

**Behavior:** When specified, element must NOT have all the specified relation types to be included. If element has all specified relation types, it is excluded. Invalid relation type names shall cause an error with a list of valid relation types.

---

## Filter Composition

All filters are applied **conjunctively**. That is, an element is included in the search results **only if all active filters return `true`** for that element.

---

## Error Handling

- Invalid regular expressions must produce a fatal error with a descriptive message.
- Invalid glob patterns should fail at startup with appropriate feedback.
- Unknown or malformed `type` filters should be rejected with a list of accepted values.
- Unknown or malformed governance metadata enum filters should be rejected with a list of accepted values for that property.
- Invalid relation type names in `--have-relations` or `--not-have-relations` shall produce an error listing valid relation types.

---

## Output Behavior

Filtered results must be consistent across all output modes (text, JSON, short text, short JSON). The final search results must include only elements passing the full filter set, and global counters should reflect the filtered subset.

---

## Performance Considerations

The filtering system must evaluate filters with minimal passes over element data. Repeated relation scans should be avoided in favor of single-pass accumulation.

---

## Test Cases (Examples)

| Filter Combination | Expected Result |
|--------------------|------------------|
| `type = verification` | Only verification elements |
| `type = constraint` | Only constraint refinement elements |
| `type = behavior` | Only behavior refinement elements |
| `type = specification` | Only specification refinement elements |
| `status = approved` | Feature and requirement elements whose effective status is approved |
| `priority = high,critical` | Feature and requirement elements whose effective priority is high or critical |
| `risk = critical` | Feature and requirement elements whose effective risk is critical |
| `owner = "Platform.*"` | Feature and requirement elements whose effective owner matches the regex |
| `filter-file = "System*"` + `name = ".*GPS.*"` | Elements in System files with GPS in name |
| `have-relations = verifiedBy,satisfiedBy` | Elements that have both verifiedBy AND satisfiedBy relations |
| `not-have-relations = verifiedBy` | Elements that do NOT have any verifiedBy relations |
| `filter-page-content = "architecture"` | Elements in files whose frontmatter contains "architecture" |
