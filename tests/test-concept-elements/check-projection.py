"""Extract relation facts without inferring missing reciprocal edges."""
import json
import sys

with open(sys.argv[1], encoding="utf-8") as source:
    graph = json.load(source)
if isinstance(graph, dict):
    graph = graph.get("@graph", [graph])
skos = "http://www.w3.org/2004/02/skos/core#"
facts = []
for node in graph:
    subject = node.get("@id", "")
    if not (subject.startswith("https://example.test/projection-") or subject in ("https://external.example/Exact", "https://external.example/Close")):
        continue
    for relation in ("broader", "narrower", "related", "exactMatch", "closeMatch"):
        for obj in node.get(skos + relation, []):
            facts.append(f'{subject} {relation} {obj["@id"]}')
# Do not deduplicate: duplicate serialization must fail the expected-file comparison.
print("\n".join(sorted(facts)))
