#!/usr/bin/env bash
# Count only the production Repo resolver, then invoke the actual interpreter.
set -euo pipefail
if [[ $# -gt 1 && "$1" == -c && "$2" == *'from manifest_xml import RepoClient'* ]]; then
    printf 'resolve\n' >> "$REQVIRE_REPO_TEST_RESOLVER_LOG"
fi
exec "$REQVIRE_REPO_TEST_PYTHON" "$@"
