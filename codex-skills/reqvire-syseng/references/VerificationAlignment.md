# Verification Criteria Alignment

Use this reference when synchronizing verification element criteria with actual test assertions.

**For common commands**, see [SKILL.md Command Cheatsheet](../SKILL.md#command-cheatsheet).

## Do It When

- Verification criteria and e2e assertions diverge
- A new command/rule was implemented and criteria mention behavior not explicitly tested
- User asks to align verification claims to actual tests

## Goal

Keep verification elements and test scripts synchronized so each critical claim has a concrete assertion.

## Workflow

1. **Locate owning verification element(s)** for the affected command/feature
2. **Inspect existing e2e assertions** and expected fixtures
3. **Rewrite verification criteria** to match explicit assertions
4. **Extend existing command test suite** with missing critical negative/positive cases
5. **Use expected output files** and diff checks for deterministic failures
6. **Run `./tests/run_tests.sh`** and only finalize after full pass

## How Not To Do It

- Do not keep unverifiable claims in verification criteria
- Do not add vague criteria without direct assertions
- Do not create a separate test if the existing feature suite should own the new criterion
