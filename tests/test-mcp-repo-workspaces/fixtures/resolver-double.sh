#!/usr/bin/env bash
# Count source-owner discovery, then invoke the real Git implementation.
set -euo pipefail
if [[ " $* " == *" rev-parse --show-toplevel "* ]]; then
    printf 'resolve\n' >> "$REQVIRE_REPO_TEST_RESOLVER_LOG"
fi
exec "$REQVIRE_REPO_TEST_GIT" "$@"
