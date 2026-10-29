# Reverse declarations and each declaration's relation lines independently.
def trim: sub("^\\s+"; "") | sub("\\s+$"; "");
def reverse_relations:
  if contains("#### Relations\n") then
    index("#### Relations\n") as $at |
    .[:$at] + "#### Relations\n" +
    (.[($at + ("#### Relations\n"|length)):] | split("\n") | reverse | join("\n"))
  else . end;
index("\n") as $first |
.[:$first] + "\n\n" +
(.[($first+1):] | split("\n---") | map(trim) | map(select(length>0)) |
 reverse | map(reverse_relations) | join("\n---\n\n")) + "\n---\n"
