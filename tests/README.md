# End-to-end tests

Run the shell suites from the repository root using the locally built CLI:

```bash
cargo build --bin reqvire
./tests/run_all.sh
./tests/run_all.sh test-serve-command
```

The runner copies each suite into a temporary Git workspace. Keep fixtures and
expected output in the suite directory, and link its `test.sh` from the owning
system-model verification using `satisfiedBy`.

The serve suite runs the compiled Explorer in headless Chrome or Chromium. It
requires Node.js 24 and a browser executable. Set `REQVIRE_TEST_BROWSER` to that
executable, or install `chromium`, `chromium-browser`, or `google-chrome` on PATH.
Build the Explorer with `cd explorer && npm ci && npm run build` before building
the CLI so it embeds the current bundle.

The pull-request workflow installs headless Chromium with Playwright and passes
its executable to the same shell runner. Browser checks are required; an absent
browser fails the serve suite. Failure logs are retained in
`/tmp/reqvire-test-logs` and uploaded by the workflow.
