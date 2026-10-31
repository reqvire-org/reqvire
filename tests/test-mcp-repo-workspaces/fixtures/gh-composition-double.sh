#!/bin/bash
# Repository-scoped offline GitHub state for coordinated PR creation.
set -euo pipefail
state=$REQVIRE_GH_TEST_STATE
operation=$1
case "$1" in auth|repo|pr) operation="$1 ${2:-}" ;; esac
option() {
    local wanted=$1
    shift
    while [[ $# -gt 1 ]]; do
        if [[ "$1" == "$wanted" ]]; then printf '%s\n' "$2"; return; fi
        shift
    done
    return 1
}
repository=$(option --repo "$@" || true)
repository=${repository#github.com/}
body=$(mktemp "$state/body-XXXXXX")
trap 'rm -f "$body"' EXIT
cat > "$body"
jq -cn --arg operation "$operation" --arg repository "$repository" --rawfile body "$body" --args \
    '{operation:$operation,repository:$repository,body:$body,args:$ARGS.positional}' -- "$@" >> "$state/calls.jsonl"
key=${repository//\//_}
case "$operation" in
    --version) printf 'gh version 2.80.0 (fixture)\n' ;;
    'auth status') exit 0 ;;
    'repo view')
        repository=${3#github.com/}
        jq -cn --arg repository "$repository" '{nameWithOwner:$repository,viewerPermission:"WRITE"}' ;;
    api) printf '{"ahead_by":1}\n' ;;
    'pr list')
        if [[ -f "$state/$key.json" ]]; then cat "$state/$key.json"; else printf '[]\n'; fi ;;
    'pr create')
        if [[ -f "$state/deny-root" && "$repository" == test/project ]]; then
            printf 'HTTP 403: injected root PR denial\n' >&2; exit 1
        fi
        draft=false
        for argument in "$@"; do [[ "$argument" != --draft ]] || draft=true; done
        jq -n --arg repository "$repository" --arg head "$(option --head "$@")" \
            --arg base "$(option --base "$@")" --argjson draft "$draft" \
            '[{number:7,url:("https://github.com/"+$repository+"/pull/7"),headRefName:$head,
              baseRefName:$base,isDraft:$draft,isCrossRepository:false}]' > "$state/$key.json"
        printf 'https://github.com/%s/pull/7\n' "$repository" ;;
    'pr view') cat "$state/$key.json" | jq '.[0]' ;;
    'pr comment') printf 'https://github.com/%s/pull/%s#issuecomment-11\n' "$repository" "$3" ;;
    *) printf 'Unexpected coordinated gh operation: %s\n' "$*" >&2; exit 1 ;;
esac
