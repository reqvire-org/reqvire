# Contributing to Reqvire

## Current Collaboration Model

Reproducible bug reports are very welcome.

Code, requirements, specification, ontology, verification, and other model
contributions are closed at the moment.

If you find a bug, please open a bug report and include detailed reproduction
steps, the exact command or workflow, relevant files or model snippets, expected
and actual behavior, logs, and root-cause hypotheses when possible.

If you would like to propose a capability or behavior change, please open an
issue (or upvote an existing one). Design discussion is welcome, but do not open
pull requests for code or model changes.

If you want to suggest improvements to Reqvire's own model, ontology,
requirements, verification structure, or architecture, use
[GitHub Discussions](https://github.com/reqvire-org/reqvire/discussions).
Discussions are the right place to compare modeling options before maintainers
decide whether and how to update the model.

External pull requests for code or model changes may be closed without review.

## Collaborator Interest

The collaborator program may open selectively over time. Repository access is not
granted through unsolicited pull requests.

If you are interested in becoming a collaborator, start a
[GitHub Discussion](https://github.com/reqvire-org/reqvire/discussions) with:

- the areas where you want to help, such as reproducible testing, ontology
  modeling, verification strategy, documentation, release engineering, or AI
  workflow evaluation
- examples of relevant technical, modeling, or project work
- the amount of time and responsibility you can realistically take on
- how you would work within Reqvire's AI-driven engineering process, where
  humans guide architecture and model intent while implementation is generated
  and refined through agentic coding workflows
- any conflicts of interest, vendor interests, or production use cases that may
  shape your perspective

The common path is sustained useful participation first: reproducible bug
reports, high-signal model-improvement discussions, careful design analysis, and
constructive review of behavior and documentation. Maintainers may then invite a
person into a narrower collaborator role with explicit scope, expectations, and
access level.

## Why We Use This Model

Reqvire uses its own repository as an experiment in AI-driven engineering with a
human in the loop. Humans work primarily as architects on higher-level planes:
ontology, architecture, requirements, specifications, verification strategy, and
system behavior. Implementation is generated, tested, and refined through
agentic coding workflows.

The goal is to achieve high-quality code that is not directly human-authored,
while keeping architectural accountability human-led. Accepting general code or
model pull requests would weaken that experiment and add review paths that do
not match the current development process.

The most valuable external input is therefore precise problem analysis:
reproducible bugs, regression cases, confusing behavior, documentation gaps, and
well-scoped model-improvement discussion in GitHub Discussions.

## Maintainer Development Workflow

This workflow applies to maintainer-authored changes while external code and
model contributions are closed.

1. Start with an issue and align on the approach before writing code.
2. Apply MBSE principles first: begin with requirements/specifications/verifications updates in the system model.
3. Create a focused topic branch from `main`.
4. Keep changes scoped to one problem per PR.
5. Add or update tests that fail before and pass after your change.
6. Update relevant docs when behavior changes.
7. Run project checks locally before opening the PR.

### Opening a Pull Request

1. Link the issue or internal context for the change.
2. Clearly describe what changed and why.
3. Ensure all local checks pass.
4. Mark the PR ready for review only when it is merge-ready.

### Review Process

1. A maintainer will review the PR.
2. If scope diverges from the agreed issue plan, the PR may be closed.
3. Revisions may be requested for correctness, consistency, or maintainability.
4. Approved PRs are merged by maintainers.

## Code of Conduct

By participating in this project, you agree to follow the [Code of Conduct](./code_of_conduct.md).

## Contributor Agreement (CLA)

If external code or model contributions are reopened in the future, merged
contributions will require CLA acceptance. See [CLA.md](./CLA.md) for details.

## Getting Help

- Review the [user documentation](https://www.reqvire.org)
- Review the [contributor documentation](./README.md)
- Open an issue for reproducible bug reports
- Use [GitHub Discussions](https://github.com/reqvire-org/reqvire/discussions) for questions and model-improvement suggestions
- Use GitHub Discussions to express collaborator interest
