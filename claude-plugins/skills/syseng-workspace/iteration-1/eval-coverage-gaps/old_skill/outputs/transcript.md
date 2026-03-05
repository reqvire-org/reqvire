# Eval: coverage-gaps (with old skill)

## Task
"Find all requirements that don't have verifications yet. For each one, suggest what type of verification would be appropriate..."

## Approach
- Read skill-snapshot/SKILL.md
- Read all requirement files systematically
- Cross-referenced against verifiedBy relations
- Did NOT use `reqvire search --not-have-relations="verifiedBy"` (used file reading instead)

## Findings
Identified **37 requirements without verifiedBy relations**:
- 29 recommended for test-verification (78%)
- 5 recommended for inspection-verification (14%)
- 1 recommended for demonstration-verification (3%)
- 1 recommended for analysis-verification (3%)

## Key Insights Provided
1. Code traceability subsystem (CodeAlignment.md) has 8 unverified requirements -- largest cluster
2. CLI command wrappers lack verifications even when underlying functional requirements are verified
3. Architecture requirements need inspection/analysis rather than testing
4. AI Skills requirements are documentation requirements best verified by inspection

## Notable Behavior
- Did NOT use `reqvire search --not-have-relations="verifiedBy"` filter
- Read files directly instead of using reqvire commands
- More detailed analysis than with_skill run (97k vs 84k tokens)
- Provided insightful clustering of gaps by subsystem
