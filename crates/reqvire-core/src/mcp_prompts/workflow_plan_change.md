# Reqvire Change Planning

Use this prompt when the user wants to add, modify, rename, move, or refactor Reqvire model content.

Workflow:
- Start with read-only evidence: `reqvire.workspace_status`, `reqvire.search`, `reqvire.read_element`, and structure tools relevant to the scope.
- Identify the capability, requirement, ontology, concept-scheme/concept, semantic-contract, and verification impacts before proposing edits.
- For ontology, concept, or semantic-contract changes, inspect semantic vocabulary and prefix information before writing terms.
- For standalone Thesaurus changes, inspect `reqvire.concept_schemes.list`, `reqvire.concepts.list`, or `reqvire.concepts.get` before planning edits.
- Use native `concept-scheme` and `concept` Markdown elements for Reqvire concept authoring. Concept schemes own `concept_base` and `concept_prefix` directly. Do not plan new concept authoring as Turtle-authored `skos:Concept` resources inside ontology elements.
- If mutation tools are enabled, preview changes first with each tool's dry-run or preview mode when available.
- After edits, run or request validation evidence through `reqvire.validate`, `reqvire.lint`, coverage, traces, or the matching MCP report tools.

Answer discipline:
- Keep canonical model semantics current; do not rely on legacy section names.
- Prefer requirement-owned contracts for implementation details and ontology elements for reusable vocabulary.
- Prefer native concepts for thesaurus terms used by `#### Concept References`; use `reqvire:mapsToConcept` only as a structural ontology-to-concept bridge.
- Update verification expectations in the same change when requirements or capabilities move.
