#!/usr/bin/env bash
# Regenerate store and expectation fixtures from the real coverage E2E model.
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
fixture=$root/tests/test-scoped-coverage
destination=$root/explorer/src/store/fixtures
cp "$fixture/expected/whole-model.json" "$destination/wholeCoverage.fixture.json"
cp "$fixture/expected/scopes.json" "$destination/coverageScopes.fixture.json"
workspace=$(mktemp -d /tmp/reqvire-coverage-showcase-XXXXXX)
trap 'rm -rf "$workspace"' EXIT
cp -a "$fixture/specifications" "$fixture/evidence" "$workspace/"
cp "$fixture/.reqvireignore" "$workspace/"
# Repeated basenames and long paths exercise the evidence panel.
links=$workspace/showcase-links.txt
: > "$links"
for artifact in README.md skills/SKILL.md skills/AnalyzeCoverage.md skills/AnalyzeModel.md skills/ChangeImpact.md skills/Lint.md implementation/SKILL.md; do
    mkdir -p "$workspace/evidence/$(dirname "$artifact")"
    printf 'Synthetic showcase evidence: %s\n' "$artifact" > "$workspace/evidence/$artifact"
    printf '  * satisfiedBy: [%s](../evidence/%s)\n' "${artifact##*/}" "$artifact" >> "$links"
done
jq -Rs -j --rawfile links "$links" '
  "  * satisfiedBy: [Alpha implementation](../evidence/alpha.txt)" as $evidence |
  "  * definedBy: [Shared Service Format](#shared-service-format)" as $contract |
  split($evidence)|join($evidence+"\n"+($links|rtrimstr("\n")))|
  split($contract)|join("  * satisfiedBy: [Parent implementation](../evidence/parent.txt)\n"+$contract)
' "$workspace/specifications/Alpha.md" > "$workspace/alpha.txt"
cp "$workspace/alpha.txt" "$workspace/specifications/Alpha.md"
printf 'Direct parent evidence does not cover Alpha Gap.\n' > "$workspace/evidence/parent.txt"
printf 'evidence/\n' >> "$workspace/.reqvireignore"
git init -q --initial-branch=showcase "$workspace"
"$root/target/debug/reqvire" --workspace "$workspace" export --output "$workspace/output/site" >/dev/null
jq -Rs '
  sub("^window\\.reqvireProjectStore\\s*=\\s*";"")|sub(";\\s*$";"")|fromjson|
  .project={name:"Scoped coverage example",root_label:"coverage-example",workspace_root:"/examples/coverage",
    repository:null,branch:"showcase",eligible_git_worktrees:[{root:"/examples/coverage",workspace_relative_root:"",head:null,dirty:true}]}
' "$workspace/output/site/assets/project-store.js" > "$destination/scopedCoverage.json"
printf 'explorer/src/store/fixtures/scopedCoverage.json\n'
