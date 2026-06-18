# Security Policy

## Supported Versions

Reqvire security updates are provided for the current stable major release.
Reqvire 1.0.0 is the first stable release; pre-1.0 versions are no longer
supported with security fixes.

| Version | Supported |
| ------- | --------- |
| 1.x     | Yes       |
| < 1.0   | No        |

Security fixes are released on the latest supported patch version. Users should
upgrade to the newest available Reqvire release before reporting a vulnerability
against an older patch version.

## Reporting A Vulnerability

Please do not report security vulnerabilities in public GitHub issues,
Discussions, pull requests, or social channels.

Use GitHub private vulnerability reporting:

https://github.com/reqvire-org/reqvire/security/advisories

When reporting, include as much of the following as possible:

- affected Reqvire version and installation method
- affected command, server mode, Explorer route, MCP tool, or generated output
- operating system and relevant runtime versions
- minimal reproduction steps
- proof of impact, such as unexpected file access, code execution, data
  exposure, denial of service, model corruption, or unsafe generated output
- whether the issue requires a malicious Reqvire model, malicious repository
  contents, a network attacker, local user access, or another precondition
- suggested mitigation, if known

Security reports are welcome even while code, requirements, specification,
ontology, verification, and other model contributions are closed. Please focus
on reproduction, impact, and analysis rather than opening a pull request.

## Response Expectations

The maintainers aim to:

- acknowledge a new private report within 7 days
- provide an initial triage result within 14 days
- give status updates at least every 14 days while an accepted report remains
  open
- coordinate disclosure timing through the GitHub advisory thread

If a report is accepted, maintainers will work on a fix, release an updated
version, and publish an advisory when appropriate. If a report is declined, the
advisory thread will explain why, for example because the behavior is not a
security boundary, is already fixed in a supported version, requires unsupported
versions, or is outside the project scope.

## Scope

Security reports are in scope when they affect supported Reqvire releases,
including:

- the `reqvire` CLI
- the core model parser, validator, graph registry, migration, and export logic
- `reqvire serve`, including Explorer assets served by Reqvire
- MCP/server integrations shipped by this repository
- release artifacts and package distribution maintained by this repository
- dependency vulnerabilities with a plausible impact on shipped artifacts,
  development workflows, or generated output

Out of scope:

- unsupported pre-1.0 versions
- reports that require disabling normal operating-system, browser, or GitHub
  security controls
- issues only affecting local development experiments that are not shipped or
  documented
- spam, social engineering, or physical attacks
- denial-of-service reports based only on intentionally huge local input files
  without a practical security impact

## Public Disclosure

Please keep vulnerability details private until a fix or mitigation is available
and maintainers have coordinated disclosure through the advisory thread.

After disclosure, public writeups are welcome if they do not expose users to
unpatched risk.
