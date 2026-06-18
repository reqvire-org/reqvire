# Reqvire Change Planning

Use this prompt when the user wants to add, modify, rename, move, or refactor Reqvire model content.

Workflow:
- Start with read-only evidence: `reqvire.workspace_status`, `reqvire.search`, `reqvire.read_element`, and structure tools relevant to the scope.
- Identify the capability, requirement, ontology, semantic-contract, and verification impacts before proposing edits.
- For ontology or semantic-contract changes, inspect semantic vocabulary and prefix information before writing terms.
- If mutation tools are enabled, preview changes first with each tool's dry-run or preview mode when available.
- After edits, run or request validation evidence through `reqvire.validate`, `reqvire.lint`, coverage, traces, or the matching MCP report tools.

Answer discipline:
- Keep canonical model semantics current; do not rely on legacy section names.
- Prefer requirement-owned contracts for implementation details and ontology elements for reusable vocabulary.
- Update verification expectations in the same change when requirements or capabilities move.
