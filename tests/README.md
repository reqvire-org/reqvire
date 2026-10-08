# End-to-end tests

Run the shell suites from the repository root using the locally built CLI:

```bash
cargo build --bin reqvire
./tests/run_all.sh
./tests/run_all.sh test-serve-command
```

Install ripgrep (`rg`) before running the suites; ontology hygiene checks use it.
The pull-request workflow installs it explicitly.

The runner copies each suite into a temporary Git workspace. Keep fixtures and
expected output in the suite directory, and link its `test.sh` from the owning
system-model verification using `satisfiedBy`.

Run `./tests/run_all.sh test-model-revision-hashing` for the canonical model
revision contract. It uses Python 3's standard library to call the real HTTP MCP
server, compares fixed canonical byte/digest fixtures, and checks field changes,
ordering, excluded inputs, and agreement across fingerprint-bearing tools.
Responses and server logs are retained in the test workspace's `output` directory.
Explorer wire hashes and live refresh remain covered by the existing serve suite.

The serve suite runs the compiled Explorer in headless Chrome or Chromium. It
requires Node.js 24 and a browser executable. Set `REQVIRE_TEST_BROWSER` to that
executable, or install `chromium`, `chromium-browser`, or `google-chrome` on PATH.
Build the Explorer with `cd explorer && npm ci && npm run build` before building
the CLI so it embeds the current bundle.

The pull-request workflow installs headless Chromium with Playwright and passes
its executable to the same shell runner. Browser checks are required; an absent
browser fails the serve suite. Failure logs are retained in
`/tmp/reqvire-test-logs` and uploaded by the workflow.
The workflow also runs `cd explorer && npm test` for store validation, manifest
transactions, and refresh lifecycle tests.

Live-refresh checks use mutation-enabled embedded MCP to update already-open
Explorer tabs and verify conditional revisions, preserved navigation and modal
context, source and search updates, visibility changes, static-export behavior,
and continued MCP access. A plain server verifies that live refresh is disabled.
The shell has no manual Refresh action. The suite compares named browser checks
with its expected output files.

Manifest checks assert that the compiled browser requests only missing chunks,
verifies their content hashes, and publishes a complete store matching the server.
The refreshed immutable store must also render report and graph views correctly.
They exercise a real mutation between manifest and chunk requests, missing and
corrupt chunk responses, recovery without advancing a failed refresh's revision,
and catching up across several mutations and a deletion while the tab is hidden.
