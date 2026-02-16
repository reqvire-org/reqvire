# Contributing to Reqvire

## External Contributions Are By Invitation Only

At this time, the Reqvire maintainers do not accept unsolicited pull requests.

If you would like to propose a feature or behavior change, please open an issue (or upvote an existing one). If you find a bug, please open a bug report and include detailed analysis, reproduction steps, and root-cause hypotheses when possible.

Pull requests that have not been explicitly invited by a maintainer may be closed without review.

## Why We Use This Model

Maintaining Reqvire requires architectural context, roadmap awareness, and prioritization across many requests. In practice, unsolicited PRs often increase review overhead and delay higher-priority work.

The most valuable external input is usually early problem analysis in issue discussions. We therefore focus community contributions on issue collaboration first, and invite implementation PRs selectively when the scope and approach are aligned.

## If You Are Invited To Contribute Code

### Development Workflow

1. Start with an issue and align on the approach before writing code.
2. Apply MBSE principles first: begin with requirements/specifications/verifications updates in the system model.
3. Create a focused topic branch from `main`.
4. Keep changes scoped to one problem per PR.
5. Add or update tests that fail before and pass after your change.
6. Update relevant docs when behavior changes.
7. Run project checks locally before opening the PR.

### Opening a Pull Request (By Invitation)

1. Link the approved issue in your PR.
2. Clearly describe what changed and why.
3. Ensure all local checks pass.
4. Mark the PR ready for review only when it is merge-ready.

### Review Process

1. A maintainer will review the invited PR.
2. If scope diverges from the agreed issue plan, the PR may be closed.
3. Revisions may be requested for correctness, consistency, or maintainability.
4. Approved PRs are merged by maintainers.

## Code of Conduct

By participating in this project, you agree to follow the [Code of Conduct](./code_of_conduct.md).

## Contributor Agreement (CLA)

All merged contributions require CLA acceptance. See [CLA.md](./CLA.md) for details.

## Getting Help

- Review the [user documentation](https://www.reqvire.org)
- Review the [contributor documentation](./README.md)
- Open an issue for bug reports or proposals
- Use [GitHub Discussions](https://github.com/reqvire-org/reqvire/discussions) for questions
