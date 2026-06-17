<div align="center">

<img src="doc/social-card.png" alt="Reqvire - Build verifiable and traceable software" width="100%">

[![Latest Release](https://img.shields.io/github/v/release/Reqvire/reqvire?style=flat-square&logo=github&color=blue)](https://github.com/reqvire-org/reqvire/releases)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat-square)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

Explore Reqvire > [📖 **Documentation**](https://www.reqvire.org) • [🔍 **Browse Model**](https://reqvire-org.github.io/reqvire/) • [🚀 **Quick Start**](#quick-start) • [👥 **Contributing**](./doc/README.md)

</div>

---

# What is Reqvire?

Build verifiable and traceable software.

Model ontologies, capabilities, requirements, and verifications in Git, with traceability and engineering context built in for humans, AI agents, and modern engineering workflows.

It unifies:
- system modeling
- requirements engineering
- verification & validation
- semantic traceability
- context engineering
- AI-assisted development

into a single workflow that stays aligned with your codebase.

Unlike traditional requirements systems, Reqvire treats engineering knowledge as a connected, versioned model rather than disconnected documents, tickets, or spreadsheets.

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

The Explorer UI is embedded in release/npm binaries. Source builds are for
development and must build the Explorer bundle before compiling Rust.

---

## Use with AI Coding Assistants

Reqvire ships with:
- Claude Code integrations
- Codex skills
- MCP server support

Start the MCP server with the Reqvire CLI:

```bash
reqvire --workspace /absolute/path/to/workspace mcp
```

Convenience no-install form using the npm package:

```bash
npx -y @reqvire-org/reqvire@latest --workspace /absolute/path/to/workspace mcp
```

This exposes Reqvire engineering context to AI assistants through the Model Context Protocol (MCP).

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

Enable mutation tools:

```bash
reqvire --workspace /absolute/path/to/workspace mcp --enable-mutations
```

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
Refinements
    ↓
Verification
```

---

## Ontologies

Ontologies define:
- concepts
- relationships
- domain vocabulary
- semantic meaning

Reqvire supports ontology modeling directly inside the engineering workflow.
Ontology boundary metadata and Turtle are explicit: a top ontology element declares `ontology_base` and `ontology_prefix`, and its Turtle must agree with the generated ontology IRI and term namespace.

This enables:
- semantic consistency
- AI-readable engineering context
- shared system understanding
- traceable domain semantics

Ontology itself defines the shared meaning; reusable constraints are authored separately as `semantic-contract` elements.

`semantic-contract` elements are first-class, reusable SHACL profiles that:
- declare constraints in a `#### Shapes` block
- bind to ontology terms through `use` / `usedBy`
- constrain requirements through `constrain` / `constrainedBy`

This keeps constraints explicit and reusable at the ontological level, while requirements remain implementation-facing obligations and remain machine-verifiable through ontology-derived checks.

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

In Reqvire, requirements are not treated as isolated specification statements or static documentation.

Requirements are first-class semantic graph elements that:
- derive from capabilities
- connect to ontology concepts
- refine into structured engineering contracts
- can be constrained by ontologically anchored `semantic-contract`s
- stay verifiable as ontology-derived constraints are reapplied through SHACL profiles
- trace to implementation artifacts
- link directly to verification evidence
- propagate change impact throughout the engineering graph

Reqvire requirements act as durable engineering context for:
- humans
- AI systems
- coding assistants
- architecture workflows
- verification pipelines

This enables:
- intelligent traceability
- automated impact analysis
- architecture-aware development
- specification-driven engineering
- semantic consistency across the lifecycle

Requirements can evolve from lightweight statements into progressively refined, machine-verifiable engineering semantics through:
- behavioral refinements
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

Unlike traditional requirements systems, Reqvire requirements remain:
- version-controlled
- AI-readable
- semantically connected
- continuously traceable
- tightly integrated with implementation and verification workflows

This transforms requirements from static documentation into living engineering intelligence embedded directly inside Git workflows.

---

## Refinements

Requirements can be refined into structured engineering contracts such as:
- behavioral contracts
- state models
- input/output contracts
- constraints
- detailed specifications

This allows requirements to evolve into precise, machine-readable engineering semantics while remaining Git-native and human-readable.

Semantic contracts are separate reusable SHACL profiles. They are linked to ontology terms using `use`/`usedBy`, and they constrain requirements via `constrain`/`constrainedBy`; requirements may still exist without ontology or semantic-contract links.

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

3) Connect each concrete verification element with `verify` relations to target capabilities or requirements.

4) Add evidence evidence in implementation-facing layers using `satisfiedBy` on concrete verifications once execution is complete.

5) Track quality using Reqvire coverage and traceability outputs. Coverage reflects leaf requirements; requirements roll up through existing requirement relations.

This enables:
- verifiable plans instead of ad-hoc test lists
- impact-aware updates when requirements or context changes
- traceability from obligations to evidence
- repeatable, AI-readable validation workflows

---

# Why Reqvire?

Traditional engineering knowledge is fragmented across:
- documents
- tickets
- architecture diagrams
- code
- spreadsheets
- wikis
- test systems

This creates:
- stale specifications
- broken traceability
- inconsistent semantics
- disconnected verification
- poor AI context
- difficult change propagation

Reqvire turns engineering knowledge into a living semantic graph directly inside Git.

---

# AI-Native Engineering

Reqvire models are designed for both humans and AI systems.

The semi-structured modeling language enables:
- LLM reasoning
- intelligent traceability
- semantic search
- automated impact analysis
- context-aware code generation
- AI-assisted verification
- intelligent change propagation
- architecture-aware engineering workflows

Reqvire acts as a structured, canonical context layer that AI systems can reliably reason over.

---

# Key Features

## Semantic Engineering in Git

Reqvire keeps engineering knowledge:
- versioned
- traceable
- reviewable
- AI-readable
- semantically connected

directly alongside implementation artifacts.

No more:
- lost specifications
- outdated documents
- disconnected architecture
- isolated requirements
- untraceable changes

---

## Capability-Driven Engineering

Develop systems around coherent operational capabilities instead of disconnected feature lists or isolated requirements.

Capabilities:
- decompose hierarchically
- generate requirements
- support verification
- connect naturally to ontology concepts
- remain stable across implementation changes

---

## Ontology-Driven Modeling

Bring semantic engineering directly into your Git workflow.

Reqvire supports:
- ontology modeling
- semantic relationships
- engineering vocabularies
- domain concepts
- AI-readable system meaning

This enables:
- consistent terminology
- shared system understanding
- semantic traceability
- better AI reasoning

---

## Intelligent Engineering

Bring modern Model-Based Systems Engineering (MBSE) directly into Git-native workflows.

### Specification-Driven Development

Develop from capabilities and requirements.

Keep:
- implementation
- architecture
- tests
- verification artifacts

aligned with engineering intent.

---

### Automated Traceability

Maintain bidirectional links between:
- ontologies
- capabilities
- requirements
- refinements
- code
- tests
- verification artifacts

---

### Verification & Validation

Track:
- verification coverage
- behavioral correctness
- implementation alignment
- validation evidence

throughout the engineering lifecycle.

---

### Smart Change Propagation

Identify impacted:
- requirements
- capabilities
- verifications
- implementation artifacts
- semantic dependencies

and propagate changes consistently across the engineering graph.

---

### AI-Assisted Engineering

Provide coding assistants and AI systems with structured engineering context that stays aligned with:
- architecture
- requirements
- capabilities
- verification
- semantic meaning

---

### Seamless Integration

Works naturally with:
- Git
- branches
- pull requests
- reviews
- CI/CD
- coding assistants

No workflow disruption — just structured engineering intelligence.

---

# Assistant Integrations

## Claude Code Plugin

Before installing the plugin, ensure you have:
1. **Claude Code** installed (available at [claude.com/claude-code](https://claude.com/claude-code))

The Reqvire plugin is available through the reqvire-org marketplace for Claude Code.

### Add the marketplace

```text
/plugin marketplace add https://github.com/reqvire-org/reqvire
```

### Install the plugin

```text
/plugin install reqvire@reqvire-org
```

Restart Claude Code after installation.

Read more in:
[https://www.reqvire.org/coding_assistants.html](https://www.reqvire.org/coding_assistants.html)

---

## Codex Skill (Reqvire SysEng)

This repository includes an installable Codex skill package at:

```text
codex-skills/reqvire-syseng
```

The skill uses the npm package directly:

```bash
npx -y @reqvire-org/reqvire@latest --workspace /absolute/path/to/workspace validate
```

To pin workflows to a specific Reqvire version:

```bash
export REQVIRE_NPX_PACKAGE=@reqvire-org/reqvire@0.13.2
```

Install globally:

```bash
git clone https://github.com/reqvire-org/reqvire.git
cd reqvire
./scripts/install-codex-skill.sh
```

The installer uses `CODEX_HOME` if set, otherwise defaults to:

```text
~/.codex
```

Installed location:

```text
$CODEX_HOME/skills/reqvire-syseng
```

Restart Codex after installing or updating the skill.

See:
[doc/CODEX_SKILLS.md](./doc/CODEX_SKILLS.md)

---

# Next Steps

- **[Documentation](https://www.reqvire.org)** — Learn how to use Reqvire
- **[Browse Model](https://reqvire-org.github.io/reqvire/)** — Explore Reqvire's own specifications

---

# Contributing

External pull request contributions are by invitation only.

---

## How to Contribute

1. **Read the [Contributing Guide](./doc/CONTRIBUTING.md)**
2. **Open or upvote an issue**
3. **Contribute analysis and design discussion**
4. **Submit a PR only if invited**

---

## Engineering Workflow

Reqvire follows a semantic MBSE workflow.

Invited code changes should include:
- capability updates
- requirements
- refinements
- verifications
- tests

when applicable.

See:
[Contributor Documentation](./doc/README.md)

---

## Quick Links

- **[Report a Bug](https://github.com/reqvire-org/reqvire/issues/new?template=bug_report.yml)**
- **[Request a Feature](https://github.com/reqvire-org/reqvire/issues/new?template=feature_request.yml)**
- **[Ask a Question](https://github.com/reqvire-org/reqvire/discussions)**
- **[Contributor Guide](./doc/README.md)**

---

## Contributor License Agreement

All contributors must accept the [Contributor License Agreement](./doc/CLA.md).

The CLA process is automated through GitHub PR comments.

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
