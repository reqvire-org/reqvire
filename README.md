<div align="center">

<img src="doc/explorer-showcase.gif" alt="Reqvire Explorer semantic model showcase" width="100%">

[![Latest Release](https://img.shields.io/github/v/release/Reqvire/reqvire?style=flat-square&logo=github&color=blue)](https://github.com/reqvire-org/reqvire/releases)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat-square)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[📖 **Documentation**](https://www.reqvire.org) • [🔍 **Explore Reqvire**](https://reqvire-org.github.io/reqvire/) • [🚀 **Quick Start**](#quick-start) • [👥 **Contributing**](./doc/README.md)

</div>

# What is Reqvire?

A semantic engineering framework for building verifiable and traceable software.

Reqvire is a Git-native semantic engineering model for ontologies, capabilities,
requirements, contracts, verification plans, evidence, and implementation links.
It keeps engineering intent close to the codebase, computes typed change impact
across the model, and gives engineers and AI assistants scoped implementation
context constrained by requirements, contracts, and verification evidence.

Use the CLI, Explorer UI, and MCP tools to navigate from intent to code,
understand what changes affect, and keep implementation work aligned with the
verified system model.

Use Reqvire to:
- define shared ontology vocabulary and semantic contracts
- connect capabilities to implementable requirements
- attach requirement-owned contracts for behavior, state, I/O, sources, and constraints
- plan verification and link concrete evidence
- inspect traceability, coverage, and change impact
- expose structured engineering context to AI workflows without exporting a separate model

## What's new in 1.5.2

- **Repo migration and portable paths.** Migration reads the normal
  `.repo/manifest.xml` entry and its sources/includes under `.repo/manifests/`;
  no root XML copy is required. An explicit `--source FILE` selects a different
  XML source. Shared path and temporary-directory handling fixes macOS prefix
  mismatches, with path checks on Linux, macOS and Windows.
- **Branch-based MCP tools (breaking change).** Tool contract version 5 uses
  `reqvire.branch.list`, `reqvire.branch.create`, `reqvire.branch.open`,
  `reqvire.branch.close`, and `reqvire.branch.refresh` for both single repositories
  and multi-repository workspaces. Previous `reqvire.worktree.*` names are no
  longer supported; clients must inspect contract compatibility.
- **Native workspace management (breaking change).** Track
  `reqvire-workspace.toml` with a configured `repositories_dir`, repository URLs,
  tracking refs and exact commit pins. CLI `workspace status/add/remove/sync/freeze/migrate`
  and typed MCP workspace tools share one workspace crate. Native operation no
  longer requires Google Repo or Python. Convert existing XML with
  `reqvire workspace migrate --from repo`; model-syntax migration remains separate.
  Coordinated branches, component-first checkpoints, explicit publication and
  linked PR sets retain their existing workflow.
- **Change impact analysis across repositories.** Comparisons reconstruct the
  selected composition from recorded manifest revisions and component pins.
  Available PR context selects the target's merge-base; otherwise supply an
  explicit baseline. Select `HEAD` for pending changes. Analysis never fetches
  or synchronizes repositories. See [multi-repository models](#multi-repository-models).

For an existing Google Repo workspace, preview conversion and then apply it:

```bash
reqvire workspace migrate --from repo --dry-run
reqvire workspace migrate --from repo
```

This reads `.repo/manifest.xml` and its selected sources under `.repo/manifests/`.
Use `--source FILE` to import a different XML, such as a newer root manifest.
Review and commit the generated `reqvire-workspace.toml`. Existing XML and Git
backing storage are preserved.

The CLI retains validation, lint inspection, discovery, coverage, reports,
exports, managed-query inspection, and server startup. Migration remains
CLI-only so older models can be upgraded before MCP startup validation:

```bash
reqvire migrate
reqvire migrate --fix
```

`reqvire migrate` previews supported model migrations by default; `--fix` applies
them, including legacy headers, verification-objective holders, contract relations
and legacy contract section headings.

---

# Quick Start

## Install Reqvire CLI

```bash
curl -fsSL https://raw.githubusercontent.com/reqvire-org/reqvire/main/scripts/install.sh | bash
```

Validate a Reqvire workspace:

```bash
reqvire validate
```

Open the Explorer UI from an installed Reqvire binary:

```bash
reqvire serve
```

Then open the printed local URL, usually:

```text
http://localhost:8080/
```

No-install npm form:

```bash
npx -y @reqvire-org/reqvire@latest --workspace /absolute/path/to/workspace serve
```

The effective workspace root is the path used for Reqvire identifiers and
relative paths. It must contain at least one eligible Git worktree; files under
the workspace but outside Git worktrees are ignored by model processing.

### Multi-repository models

Track one native `reqvire-workspace.toml` in the repository and branch owning
the product composition. The owner may contain a shared model or only
configuration while pinned components supply the entire model. Set the
checkout parent explicitly; `repositories` is never a hardcoded directory:

```toml
version = 1
repositories_dir = "external"
```

```bash
reqvire workspace add https://github.com/YOUR_ORG/website.git --path website --ref refs/heads/main
reqvire workspace add https://github.com/YOUR_ORG/user-docs.git --path user-docs --ref refs/heads/main
reqvire workspace status
```

Add specific root ignore entries for `/external/website/` and
`/external/user-docs/`, then commit the manifest and ignore configuration
through the normal branch/review process. Another clone runs
`reqvire workspace sync` to reproduce those pins before model validation
or MCP startup. Native preparation requires Git and no Repo/Python runtime.

Each declaration retains `ref` as its configured update source and `commit`
as its exact selection. Normal sync reproduces commits; `sync --latest`
explicitly follows refs and advances pins. `freeze --dry-run` previews
committed component HEADs, and `freeze` records them without changing refs.
`workspace remove PATH` unregisters a declaration and preserves its checkout.
These CLI operations do not commit or publish. Without a manifest, Reqvire
uses its normal single-repository workflow.

The same MCP branch tools manage one repository or a declared composition.
Requested root bases supply their own component pins; worktrees stay internal.
Typed workspace add/remove/sync/freeze/migrate tools share the CLI's service
and validate changed compositions before admission. Checkpoints commit
changed components first, update pins, then commit pins with accepted root
changes in one root commit. Publication remains explicit: components before
root, with linked component/root PRs when requested.

For external work, commit components, preview/apply CLI freeze without a live
MCP owner, and commit the pins with related root work before full validated
`branch.open`. The owning MCP context can explicitly freeze committed external
state through candidate validation. Rejected candidates retain accepted reads.

CLI, MCP and Explorer use the same combined model and workspace-relative
identifiers. Select the actual PR baseline or use the sole explicit
`--git-commit` override; explicit `HEAD` reviews pending work.
See the [User Guide](https://www.reqvire.org/user-guide) and
[MCP Server guide](https://www.reqvire.org/mcp-server) for setup and recovery.

## MCP Server

Start the MCP server with the Reqvire CLI:

```bash
reqvire --workspace /absolute/path/to/workspace mcp
```

Convenience no-install form using the npm package:

```bash
npx -y @reqvire-org/reqvire@latest --workspace /absolute/path/to/workspace mcp
```

This exposes Reqvire engineering context to AI assistants through the Model Context Protocol (MCP).

Model authoring is available through typed MCP tools only. Start the server with
`--enable-mutations` for CRUD tools and their previews, formatting application,
and safe lint repair. Successful authored-model mutations automatically persist
canonical formatting. `reqvire.format` previews and `reqvire.lint` inspection
remain available in read-only mode.

The CLI retains validation, lint inspection, coverage, discovery, reports,
exports, managed-query inspection, and server startup. `migrate` and
`migrate --fix` remain CLI operations so incompatible source can be upgraded
before validated MCP startup. CLI CRUD, `format` (including preview), and
`lint --fix` have been removed; removed invocations fail before model loading.

Reqvire MCP uses Streamable HTTP. Start the server with the command above, then point MCP clients at:

```text
http://127.0.0.1:8081/mcp
```

Example MCP client configuration:

```json
{
  "mcpServers": {
    "reqvire-myrepository": {
      "type": "http",
      "url": "http://127.0.0.1:8081/mcp"
    }
  }
}
```

Use `mcp --transport stdio` for a client that launches Reqvire locally. HTTP
remains the default; `mcp --transport http` selects it explicitly. Stdio uses
one Reqvire process for the selected project without a listening port:

```json
{
  "mcpServers": {
    "reqvire": {
      "type": "stdio",
      "command": "npx",
      "args": [
        "-y", "@reqvire-org/reqvire@latest",
        "--workspace", "/absolute/path/to/project",
        "mcp", "--transport", "stdio"
      ]
    }
  }
}
```

Client configuration syntax varies; use the client's command/args form and an
absolute workspace path when its launch directory is uncertain. Append
`--enable-mutations` when authoring is required. Mutation
startup still requires a clean, valid, exclusively owned Git worktree; commits
remain a separate `--enable-commits` opt-in. HTTP listener and access options
(`--host`, `--port`, `--allow-origin`, `--allow-host`) are rejected with stdio.
Protocol messages use stdout and diagnostics use stderr. Closing stdin ends the
server session, releases workers and locks, and preserves pending model work.

The MCP server exposes tools, resources, and prompts. Prompt templates include
regular Reqvire workflows such as model exploration, change planning, and
verification coverage review, plus semantic workflows for vocabulary-driven
SPARQL query construction and verification search.

Enable mutation tools:

```bash
reqvire --workspace /absolute/path/to/workspace mcp --enable-mutations
```

After eligible workspace-root implementation or evidence commits with unchanged
declared composition, call
`reqvire.branch.refresh` with the selected `branch`. It checks and
guards the captured branch commit and reloads through normal validated loading
without creating a commit; `dry_run: true` checks only. Changes to currently eligible
model documents, their consumed semantic inputs, or a changed repository
composition require explicit full validated admission with `reqvire.branch.open`
instead. Prepare clean committed owners, exact pins and the root composition
association first; pending accepted edits and recovery block admission.
An external component commit also needs that composition admission, even when
it changes only ordinary implementation or evidence files.
Ignore-file changes are allowed; current rules determine eligibility and
the model loaded after refresh. Mutation-enabled
Explorer uses HEAD mode on branch selection and provides **Refresh HEAD** for
the selected context. Blocked refreshes retain the accepted view and identify
the inputs or Git state that need attention.

Tool contract version 5 uses exact local branch names for context-bound MCP
tools and prompts, and percent-encoded `?branch=...` for MCP resources. Open an
existing branch with `reqvire.branch.open`, or create one with
`reqvire.branch.create`; use `from_branch` to select the source for a relative
base. An optional branch may be omitted with one loaded context; multiple
contexts require it. Branch names are the public interface. Opaque worktree IDs are private runtime/cache identity; public responses, picker, browser URLs and HTTP routes use branch names.
Version 5 retains `reqvire.branch.list`, `reqvire.branch.create`,
`reqvire.branch.open`, `reqvire.branch.close`, and `reqvire.branch.refresh`.
Closing a managed context preserves its Git branch and commits. Previous
`reqvire.worktree.*` tool names and obsolete MCP ID selectors are unavailable;
inspect discovery and tool contract compatibility when upgrading a client or build.

---

## Assistant Skills

The shared packages in [`skills/`](./skills) work with Codex, Claude Code, and
other agents supported by the [skills CLI](https://github.com/vercel-labs/skills).
Install from the repository's **main branch**:

```bash
npx skills add https://github.com/reqvire-org/reqvire/tree/main/skills
```

The installer prompts for skills and target agents. Installation is scoped to
this project by default. To install all four skills globally for Codex and Claude Code:

```bash
npx skills add https://github.com/reqvire-org/reqvire/tree/main/skills --agent codex claude-code --skill '*' --global
```

To install just one skill for a project:

```bash
npx skills add https://github.com/reqvire-org/reqvire/tree/main/skills --agent codex --skill reqvire-syseng
```

- `reqvire-syseng`: capability, requirement, contract, verification, traceability, and MBSE workflows.
- `reqvire-audit`: validation, linting, coverage, change impact, and model review.
- `reqvire-ontology-authoring`: OWL/Turtle vocabulary, semantic contracts, namespaces, and ontology refactoring.
- `reqvire-concept-authoring`: native SKOS schemes, concepts, mappings, and terminology.

Rerun the install command to refresh skills from main, then restart your assistant.
The skills use the Reqvire npm runner and typed MCP tools; installing them does
not install or start an MCP server. See [the shared skill guide](./doc/SKILLS.md)
for local checkout installation and Reqvire version pinning.

The Claude marketplace plugin and custom curl installers are deprecated. Existing
plugin users should disable or uninstall the Reqvire plugin through Claude Code's
`/plugin` manager, then install the shared skills with `--agent claude-code`.
Shared skill names use the `reqvire-*` prefix.

---

# Reqvire Conceptual Model

Reqvire organizes engineering knowledge into a semantic graph:

```text
Ontology
    ↓
Capabilities
    ↓
Requirements
    ↓
Contracts
    ↓
Verification
```

---

## Ontologies

Ontologies define:
- structural classes and properties
- relationships
- domain vocabulary
- semantic meaning

Reqvire supports ontology modeling directly inside the engineering workflow.
Ontology boundary metadata and Turtle are explicit: a top ontology element declares `ontology_base` and `ontology_prefix`, and its Turtle must agree with the generated ontology IRI and term namespace.

Concept schemes and concepts are modeled as first-class Reqvire elements, not as
ontology Turtle. A concept scheme owns `concept_base` and `concept_prefix`
directly and forms a standalone Thesaurus root. Concepts own curated human
terms, labels, synonyms, definitions, and broader/narrower taxonomy.
Structural ontologies own classes, properties, individuals, axioms, and SHACL
targets. Use `reqvire:mapsToConcept` only where a structural term needs an
explicit link back to a curated native concept; do not collapse the concept
layer and the structural layer just because they share labels.

```text
Thesaurus
  └─ concept-scheme root
      └─ concept

Ontologies
  └─ structural ontology
      may map structural terms to curated native concepts
```

Ontology elements can reference local external Turtle/TTL, RDF/XML, or JSON-LD
vocabularies through `External Ontology` source sections when model validation needs terms from a
third-party or shared vocabulary. These sources support reference resolution and
optional export materialization with `reqvire semantic ontologies --include-external`, `reqvire semantic graph --include-external`, or
MCP `include_external: true`, but exposed external content is limited to the
used external subset. Full external ontology files remain local dependency
inputs for validation and term resolution; they are not part of public
`include_external` output. Even `semantic graph --full --include-external` emits authored
semantic content, authored model facts, generated facts, and the used external
subset only.
Imported terms are marked
as external and do not replace authored Reqvire ontology elements as the
canonical system model.

RDF, RDFS, OWL, XSD reserved vocabulary and core SHACL shape syntax are built into Reqvire, so standard vocabulary does not need vendored source files.

Ontology itself defines the shared meaning; reusable constraints are authored separately as `semantic-contract` elements.

`semantic-contract` elements are first-class, reusable SHACL profiles that:
- declare constraints in a `#### Shapes` block
- bind to ontology terms through `use` / `usedBy`
- constrain requirements through `constrain` / `constrainedBy`

Requirements may exist without ontology or semantic-contract links. When they do
use semantic contracts, the SHACL profile stays in the ontology plane while the
requirement remains an implementation-facing obligation.

---

## Capabilities

Capabilities represent coherent operational or system abilities that the system provides or supports.

Capabilities:
- act as stable semantic anchors
- bridge ontology and requirements
- remain implementation-independent whenever possible
- support hierarchical decomposition
- provide durable traceability nodes

Examples:
- Real-Time Collaboration
- Autonomous Navigation
- Workflow Orchestration
- Semantic Search
- Distributed Coordination

Capabilities can contain optional semantic context sections such as:
- stakeholder needs
- product features
- operational context
- mission objectives
- regulatory drivers

without turning those concepts into separate graph node types.

---

## Requirements

Requirements define:
- implementable obligations
- system constraints
- measurable guarantees
- operational expectations
- behavioral contracts

Requirements are graph elements that:
- derive from capabilities
- connect to ontology concepts
- are defined by structured engineering contracts
- reuse compatible cross-subgraph contract context through `#### Contract Bindings`
- can be constrained by ontologically anchored `semantic-contract`s
- trace to implementation artifacts
- link directly to verification evidence

Requirements can evolve from lightweight statements into progressively precise, machine-verifiable engineering semantics through:
- behavioral contracts
- state models
- I/O contracts
- constraints
- detailed specifications
- ontologically anchored semantic-contracts that constrain obligations via explicit ontology-shape references and remain machine-checkable

Examples:
- latency limits
- interoperability guarantees
- safety constraints
- synchronization behavior
- auditability obligations
- semantic consistency requirements

---

## Contracts

Requirements can be defined by structured engineering contracts such as:
- source contracts
- specifications
- behavioral contracts
- state models
- input/output contracts
- constraints

This allows requirements to evolve into precise, machine-readable engineering semantics while remaining Git-native and human-readable.

Semantic contracts are ontology-plane SHACL profiles covered in the Ontologies section; requirement-owned contracts are the implementation-facing detail layer.

Requirements can also reuse compatible requirement-owned contracts from another subgraph through the `#### Contract Bindings` section. This is a dependency/context mechanism, not ownership: the reused contract remains owned by its defining requirement, while the consuming requirement makes the cross-subgraph obligation explicit.

---

## Verification Plan

Reqvire treats verification as an executable planning model, not a static checklist.

Verification planning in Reqvire follows this flow:

1) Define plan structure with `verification-objective` elements.
   These organize scope, sequencing, and grouping across capabilities and requirements.

2) Derive concrete verification elements from a `verification-objective`, such as:
- `test-verification`
- `formal-proof-verification`
- `analysis-verification`
- `inspection-verification`
- `demonstration-verification`

3) Connect each concrete verification element with `verify` relations to target requirements.

4) Add execution evidence in implementation-facing layers using `satisfiedBy` on concrete verifications once execution is complete.

5) Track quality using Reqvire coverage and traceability outputs. Coverage reflects leaf requirements; requirements roll up through existing requirement relations.

This enables:
- verifiable plans instead of ad-hoc test lists
- impact-aware updates when requirements or context changes
- traceability from obligations to evidence
- repeatable, AI-readable validation workflows

---

# Next Steps

- **[Documentation](https://www.reqvire.org)** — Learn how to use Reqvire
- **[Explore Model](https://reqvire-org.github.io/reqvire/)** — Explore Reqvire's own specifications

---

# Contributing

Reproducible bug reports are very welcome.

Code, requirements, specification, ontology, verification, and other model
contributions are closed at the moment.

Reqvire currently uses its own repository as an experiment in AI-driven
engineering with a human in the loop: humans work primarily as architects on
higher-level planes such as ontology, architecture, requirements, and
specification intent, while implementation is generated and improved through
agentic coding workflows. The goal is to produce high-quality code that is not
directly human-authored, while keeping architectural accountability human-led.

For that reason, the most useful external contribution right now is a clear,
reproducible issue: what happened, what should have happened, the exact command
or workflow, the model/files involved, and any root-cause hypothesis.

Suggestions for improving Reqvire's own model, ontology, requirements,
verification structure, or architecture should be opened as
[GitHub Discussions](https://github.com/reqvire-org/reqvire/discussions), not
pull requests. Discussions are the right place to compare modeling options
before maintainers decide whether and how to update the model.

The collaborator program may open selectively over time. If you are interested,
start a GitHub Discussion with your background, the areas where you want to
help, examples of relevant work, and how you would support Reqvire's AI-driven
engineering process. The normal path is sustained useful participation through
reproducible bug reports, model discussions, and design analysis before any
repository access is considered.

---

## How to Contribute

1. **Read the [Contributing Guide](./doc/CONTRIBUTING.md)**
2. **Open or upvote a bug report**
3. **Use Discussions for model-improvement suggestions and design discussion**
4. **Use Discussions to express collaborator interest**
5. **Do not open pull requests for code or model changes**

---

## Quick Links

- **[Report a Bug](https://github.com/reqvire-org/reqvire/issues/new?template=bug_report.yml)**
- **[Request a Feature](https://github.com/reqvire-org/reqvire/issues/new?template=feature_request.yml)**
- **[Suggest a Model Improvement](https://github.com/reqvire-org/reqvire/discussions)**
- **[Express Collaborator Interest](https://github.com/reqvire-org/reqvire/discussions)**
- **[Ask a Question](https://github.com/reqvire-org/reqvire/discussions)**
- **[Contributor Guide](./doc/README.md)**

---

# Credits

**Reqvire** is an open-source project created and maintained by [Ilija Ljubicic](https://github.com/ilijaljubicic).

---

## Special Thanks

### [Juanjo Andres](https://github.com/juanjoandres)

For valuable contributions to testing and in shaping Reqvire's direction, especially during the early phases of development.

---

### [GrapheneDB](https://www.graphenedb.com/)

For partial sponsorship and for being the first user of Reqvire.

Their support helped shape the project's early direction.

---

# What's Next?

## Join the Community

- ⭐ Star the repository
- 💬 Join discussions
- 📝 Contribute ideas and analysis
- 🚀 Explore semantic engineering workflows
- 🤖 Build AI-native engineering systems

---

# License

Licensed under the [Apache 2.0 License](LICENSE).

---
