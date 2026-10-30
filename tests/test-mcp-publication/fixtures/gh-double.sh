#!/bin/bash
# Offline GitHub CLI double; all effects stay in the fixture state directory.
set -euo pipefail
state=$REQVIRE_GH_TEST_STATE
body_file=$(mktemp "$state/body-XXXXXX")
trap 'rm -f "$body_file"' EXIT
cat > "$body_file"
jq -cn --args --rawfile body "$body_file" --arg prompt "${GH_PROMPT_DISABLED:-}" \
    '{args:$ARGS.positional,body:$body,prompt_disabled:(if $prompt == "" then null else $prompt end)}' \
    -- "$@" >> "$state/calls.jsonl"
operation=$1
case "$1" in auth|repo|pr) operation="$1 ${2:-}" ;; esac
if jq -e --arg op "$operation" '.fail == $op' "$state/scenario.json" >/dev/null; then
    printf 'HTTP 403 denied (secret-token-must-not-escape)\n' >&2; exit 1
fi
if jq -e --arg op "$operation" '.timeout == $op' "$state/scenario.json" >/dev/null; then sleep 30; fi
if jq -e --arg op "$operation" '.malformed == $op' "$state/scenario.json" >/dev/null; then
    printf 'invalid structured data\n'; exit 0
fi
option() {
    local wanted=$1
    shift
    while [[ $# -gt 1 ]]; do
        if [[ "$1" == "$wanted" ]]; then printf '%s\n' "$2"; return; fi
        shift
    done
    return 1
}
case "$operation" in
    --version) printf 'gh version 2.80.0 (fixture)\n' ;;
    'auth status')
        [[ " $* " == *' --active '* && " $* " == *' --hostname '* ]] ;;
    'repo view')
        requested=${3#*/}
        jq --arg requested "$requested" '{nameWithOwner:(.repository // $requested),viewerPermission:(.permission // "READ")}' "$state/scenario.json" ;;
    api) jq '{ahead_by:(.ahead_by // 1)}' "$state/scenario.json" ;;
    'pr list') if [[ -f "$state/prs.json" ]]; then cat "$state/prs.json"; else printf '[]\n'; fi ;;
    'pr create')
        draft=false
        for arg in "$@"; do [[ "$arg" != --draft ]] || draft=true; done
        jq -n --arg head "$(option --head "$@")" --arg base "$(option --base "$@")" --argjson draft "$draft" \
            '[{number:7,url:"https://github.com/test/project/pull/7",headRefName:$head,baseRefName:$base,isDraft:$draft,isCrossRepository:false}]' > "$state/prs.json"
        if jq -e .lost_create "$state/scenario.json" >/dev/null; then
            printf 'connection reset after creation\n' >&2; exit 1
        fi
        printf 'https://github.com/test/project/pull/7\n' ;;
    'pr view') jq -n --argjson number "$3" '{number:$number,url:("https://github.com/test/project/pull/" + ($number|tostring))}' ;;
    'pr comment')
        jq -cn --argjson number "$3" --rawfile body "$body_file" '{number:$number,body:$body}' >> "$state/comments.jsonl"
        if jq -e .lost_comment "$state/scenario.json" >/dev/null; then
            printf 'connection reset after posting\n' >&2; exit 1
        fi
        printf 'https://github.com/test/project/pull/%s#issuecomment-123\n' "$3" ;;
    *) printf 'Unexpected gh command: %s\n' "$*" >&2; exit 1 ;;
esac
