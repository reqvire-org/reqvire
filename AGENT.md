# Agent Guide for Reqvire

## Overview
This document defines the specialized agents that should be used when working with Reqvire and managing project requirements and specifications or assisting in code development following the same. 
It serves as a single reference for:
- Which agent to invoke for a given task.
- How each agent should behave.
- Key commands and workflow steps.
- References to detailed guides for deeper reading.

---

## 1. Core Agents

| Agent | Primary Purpose | When to Use | Relevant Commands / Files |
|-------|-----------------|-------------|---------------------------|
| **requirements‑engineer** | Manage specifications, add/update requirements and verifications, analyse model relations and coverage. | Any work that touches the Markdown specification files (e.g., `specifications/`, `SystemRequirements/`). | See the specifications guide for full syntax, element types, and relation rules.
| **code‑reviewer** *(optional)* | Review Rust source changes, ensure style, linting, and correctness before committing. | After implementing code, before running `cargo test`. | Uses `cargo fmt`, `cargo clippy`, and runs unit tests under `core/tests/`.
| **documentation‑agent** *(optional)* | Generate or update HTML documentation, diagrams, and traceability matrices. | When the model changes or a new release is prepared. | Commands: `./target/debug/reqvire export`, `./target/debug/reqvire serve`.

### 1.1 Requirements‑Engineer Agent
- **Scope**: Only works with specification Markdown files. It never edits Rust code.
- **Key responsibilities**:
  - Add new requirement elements (`### My New Requirement`).
  - Add verification elements (`### My Verification Test`).
  - Maintain proper `#### Metadata` and `#### Relations` sections.
  - Ensure every leaf requirement has a `verifiedBy` relation.
  - Link implementations via `satisfiedBy` after code is written.
- **Typical workflow**:
  1. Draft the element in a local branch.
  2. Run validation commands (`cargo run -- summary --filter-is-not-verified`) to spot gaps.
  3. Commit with a clear message referencing the requirement ID.

### 1.2 E2E‑Test‑Engineer Agent
- **Scope**: Works exclusively in the `tests/` directory.
- **Key responsibilities**:
  - Create a new `test-<feature>` directory.
  - Provide a `test.sh` script following the standard test script template.
  - Add any required `reqvire.yaml` config.
  - Populate `specifications/` sub‑folders with the requirement and verification markdown that the test will satisfy.
- **Typical workflow**:
  1. Identify an unverified requirement (summary --filter-is-not-verified`).
  2. Scaffold a test directory (`mkdir tests/test‑my‑feature`).
  3. Implement the script, run it locally (`./tests/run_tests.sh test‑my‑feature`).
  4. Verify success and commit.

---

## 2. Supporting Commands (Quick Reference)

| Command | Description |
|---------|-------------|
| `cargo build` | Compile the project (debug binary placed in `target/debug/`). |
| `cargo test` | Run unit tests for core Rust code. |
| `./target/debug/reqvire format` | Preview formatting changes for specifications. |
| `./target/debug/reqvire format --fix` | Apply automatic formatting fixes. |
| `./target/debug/reqvire validate --json > /tmp/validation.json` | Validate model consistency and output JSON report. |
| `./target/debug/reqvire summary --json > /tmp/model-summary.json` | Full model summary (useful for coverage queries). |
| `./target/debug/reqvire traces --json > /tmp/traces.json` | Generate verification traceability tree. |
| `./target/debug/reqvire lint [--fix]` | Lint specifications for issues; use `--fix` to automatically apply safe fixes. |
| `./target/debug/reqvire export --output out/` | Export HTML documentation with diagrams and matrices. |
| `./tests/run_tests.sh` | Execute all end‑to‑end tests. |
| `./tests/run_tests.sh <test‑name>` | Run a single test suite. |

---

## 3. How to Invoke an Agent (Practical Tips)

1. **Determine the domain** – Is the work about specifications or tests?
2. **Select the agent** – Use `requirements-engineer` for specs, `e2e-test-engineer` for tests.
3. **Call the agent** – In this environment you simply describe the action; the assistant will act as the chosen agent.
   - Example: *"Please act as the requirements‑engineer and add a new requirement for exporting CSV files with proper metadata."*
4. **Follow up** – The assistant will create/modify files, run validation commands, and report any issues.

---

## 4. Reference Summaries

### 4.1 Root Development Guide
- Provides overall project overview, MBSE workflow, and core architecture.
- Defines **specialized agents** (requirements‑engineer, e2e‑test‑engineer) and stresses that the requirements step must never be skipped.
- Lists essential cargo commands, Reqvire CLI commands, and key file locations.
- Emphasizes linking implementations to requirements via `satisfiedBy` relations.

### 4.2 Specification Writing Guide
- Details the markdown structure for specifications, including element format, metadata, and relations.
- Enumerates element types (`requirement`, `verification`, etc.) and relation types (`derivedFrom`, `verifiedBy`, `satisfiedBy`).
- Provides EARS requirement patterns (SHALL/MUST, etc.) and best‑practice guidelines for atomic requirements and traceability.
- Includes validation commands such as `cargo run -- model-summary --filter-is-not-verified`.

### 4.3 End‑to‑End Test Guide
- Describes test directory layout, required `test.sh` script template, and environment variables.
- Offers common test patterns (JSON validation, file modification checks, filter testing, error condition testing).
- Provides instructions for adding new tests and running them via `./tests/run_tests.sh`.
- Highlights best practices like deterministic results and silent success output.

These summaries capture the essential information from each original CLAUDE file, giving you quick reference while still allowing deep dives by opening the full files when needed.

---

## 5. Best Practices Summary
1. **Never skip the requirements step** – Add or update specs before any code changes.
2. **Keep requirements atomic** – One capability per element, using EARS patterns.
3. **Maintain traceability** – Every leaf requirement must have a verification; link implementations via `satisfiedBy`.
4. **Run validation after each change** – `cargo run -- summary --filter-is-not-verified` and `cargo fmt`.
5. **Keep tests deterministic** – Tests should be silent on success, verbose only on failure.
6. **Commit often with clear messages** – Reference requirement IDs and test names.

---

*This AGENT.md file is generated to give the opencode assistant a concise, actionable reference for all agent‑related workflows in the Reqvire project.*
