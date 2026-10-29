#!/usr/bin/env bash
# Verify the canonical skills and pinned installer with shell, jq, and file diffs.
set -euo pipefail
repo=$1 work=$2
suite=$repo/tests/test-ai-skill-installers
source=$repo/skills
export DISABLE_TELEMETRY=1
mkdir -p "$work"
trap 'printf "FAIL skill installer check at line %s\n" "$LINENO" >&2' ERR
manifest() { (cd "$1"; find . -type f -printf '%P\n' | LC_ALL=C sort); }
fixture() { diff -u "$suite/expected/$1" "$work/$1"; }
skills() {
    local label=$1 cwd=$2
    shift 2
    if ! (cd "$cwd"; npx --yes skills@1.7.2 add "$source" "$@") > "$work/$label.stdout" 2> "$work/$label.stderr"; then
        cat "$work/$label.stdout" "$work/$label.stderr" >&2
        return 1
    fi
}
without_fences() {
    # Buffer an open fence so an unterminated example remains visible to checks.
    awk '
      !inside && /^[[:space:]]*(```+|~~~+)/ {
        marker=$0;sub(/^[[:space:]]*/,"",marker);sub(/[^`~].*$/,"",marker)
        inside=1;buffer=$0 "\n";next
      }
      inside {
        line=$0;sub(/^[[:space:]]*/,"",line);sub(/[[:space:]]*$/,"",line)
        if(line==marker) {inside=0;buffer=""} else buffer=buffer $0 "\n"
        next
      }
      {print}
      END {if(inside) printf "%s",buffer}
    ' "$1"
}
decode_path() {
    local rest=$1 char hex
    decoded=''
    while [[ -n "$rest" ]]; do
        if [[ "$rest" =~ ^%([0-9A-Fa-f]{2}) ]]; then
            hex=${BASH_REMATCH[1]}
            [[ "$hex" != 00 ]] # A NUL cannot name a file.
            printf -v char '%b' "\\x$hex"
            decoded+=$char
            rest=${rest:3}
        else
            decoded+=${rest:0:1}
            rest=${rest:1}
        fi
    done
}
manifest "$source" > "$work/manifest.txt"
fixture manifest.txt
while IFS= read -r -d '' package; do
    name=${package##*/}
    rg -q -F -x "name: $name" "$package/SKILL.md"
    rg -q '^description: \S' "$package/SKILL.md"
    while IFS= read -r -d '' file; do
        without_fences "$file" > "$work/markdown-prose.txt"
        while IFS= read -r reference; do
            [[ -f "$(dirname "$file")/$reference" ]]
        done < <(jq -Rrs -r 'scan("`(references/[^`]+\\.md)`")|.[0]' "$work/markdown-prose.txt")
        jq -Rrs -r 'gsub("`+[^`]*`+";"")|scan("\\[[^\\]]+\\]\\(([^)]+)\\)")|.[0]' "$work/markdown-prose.txt" > "$work/markdown-links.txt"
        while IFS= read -r target; do
            path=${target%%#*}
            if [[ -z "$path" || "$path" == /* || "$path" == file.md || "$path" == path.md || "$path" =~ ^[A-Za-z][A-Za-z0-9+.-]*: ]]; then continue; fi
            decode_path "$path"
            if [[ ! -f "$(dirname "$file")/$decoded" ]]; then printf 'Broken skill link: %s -> %s\n' "$file" "$target" >&2; exit 1; fi
        done < "$work/markdown-links.txt"
        if rg -q -U -P '(?:/reqvire:|`reqvire:(?:syseng|audit)|add --override|lint --fix(?:\s|\)|:|`))|(?:reqvire\s+|`)(?:model|containment|resources|traces)\s+[^`\n]*(?:--json|--mmd)' "$file"; then
            printf 'Obsolete command guidance in %s\n' "$file" >&2; exit 1
        fi
    done < <(find "$package" -type f -name '*.md' -print0)
done < <(find "$source" -mindepth 1 -maxdepth 1 -type d -print0 | sort -z)
skills discovery "$work" --list
sed -E $'s/\033\\[[0-?]*[ -/]*[@-~]//g' "$work/discovery.stdout" > "$work/discovery-plain.txt"
sed -nE 's/^│[[:space:]]+(reqvire-[a-z-]+)[[:space:]]*$/\1/p' "$work/discovery-plain.txt" | LC_ALL=C sort > "$work/names.txt"
fixture names.txt
printf 'PASS canonical package manifests, links, and CLI discovery\n' > "$work/checks.txt"

main_source=https://github.com/reqvire-org/reqvire/tree/main/skills
for file in "$repo/README.md" "$repo/doc/SKILLS.md" "$repo/website/src/pages/CodingAssistants.tsx"; do
    for text in "npx skills add $main_source" '--agent codex claude-code' --global '--skill reqvire-syseng' deprecated /plugin; do rg -q -F -- "$text" "$file"; done
    if rg -q '/plugin install|curl.*install-(codex|claude)-skill' "$file"; then printf 'Legacy installation guidance: %s\n' "$file" >&2; exit 1; fi
done
for obsolete in codex-skills claude-plugins .claude-plugin scripts/install-codex-skill.sh scripts/install-claude-skill.sh; do [[ ! -e "$repo/$obsolete" ]]; done
printf 'PASS main-branch installation and marketplace migration guidance\n' >> "$work/checks.txt"

installation=$work/installed
mkdir "$installation"
skills installation "$installation" --agent codex claude-code --skill '*' --copy --yes --json
jq -r 'sort_by(.name)[].name' "$work/installation.stdout" > "$work/names.txt"
fixture names.txt
jq -e 'all(.[];.status=="installed" and .scope=="project" and .mode=="copy" and (.agents|sort)==["Claude Code","Codex"])' "$work/installation.stdout" >/dev/null
for agent in .agents .claude; do
    target=$installation/$agent/skills
    manifest "$target" > "$work/manifest.txt"
    fixture manifest.txt
    while IFS= read -r -d '' file; do diff -u "$file" "$target/${file#"$source/"}"; done < <(find "$source" -type f -print0)
done
skill=$installation/.agents/skills/reqvire-syseng/SKILL.md
reference=$(sed -n '/^traces /{p;q;}' "$skill")
[[ "$reference" == *--output* && "$reference" != *--json* ]]
awk '/^\*\*Common flags:\*\*/ {inside=1} inside && /^$/ {exit} inside {print}' "$skill" > "$work/common-flags.txt"
rg -q -F JSON-only "$work/common-flags.txt"
for command in model containment resources traces; do rg -q -F "\`$command\`" "$work/common-flags.txt"; done
(cd "$work"; "$REQVIRE_BIN" traces > traces-stdout.json; "$REQVIRE_BIN" traces --output traces-file.json > traces-file.stdout)
jq -e 'any(.files[].verifications[]?;.name=="Skill Reference Test")' "$work/traces-stdout.json" >/dev/null
jq -S . "$work/traces-stdout.json" > "$work/traces.json"
jq -S . "$work/traces-file.json" > "$work/traces-file-normalized.json"
diff -u "$work/traces.json" "$work/traces-file-normalized.json"
printf 'PASS complete Codex and Claude Code installs and executable report guidance\n' >> "$work/checks.txt"

selection=$work/selected
mkdir "$selection"
skills selection "$selection" --agent codex --skill reqvire-syseng --copy --yes --json
jq -e 'map(.name)==["reqvire-syseng"]' "$work/selection.stdout" >/dev/null
find "$selection/.agents/skills" -mindepth 1 -maxdepth 1 -printf '%f\n' | LC_ALL=C sort > "$work/selected-names.txt"
printf 'reqvire-syseng\n' > "$work/selected-names-expected.txt"
diff -u "$work/selected-names-expected.txt" "$work/selected-names.txt"
manifest "$source/reqvire-syseng" > "$work/selected-manifest-expected.txt"
manifest "$selection/.agents/skills/reqvire-syseng" > "$work/selected-manifest.txt"
diff -u "$work/selected-manifest-expected.txt" "$work/selected-manifest.txt"
[[ ! -e "$selection/.claude" ]]
printf 'PASS selection installs only the requested skill and agent\n' >> "$work/checks.txt"
fixture checks.txt
cat "$work/checks.txt"
