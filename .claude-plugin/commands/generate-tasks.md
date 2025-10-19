---
name: generate-tasks
description: Generate implementation task plan from requirement changes using change-impact analysis
---

# Generate Tasks

Generate implementation task plan from requirement changes on a feature branch.

## Steps

1. **Detect base branch:**
   ```bash
   CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

   if git show-ref --verify --quiet refs/heads/main; then
       BASE_BRANCH="main"
   elif git show-ref --verify --quiet refs/heads/master; then
       BASE_BRANCH="master"
   else
       echo "Specify base commit manually"
       exit 1
   fi

   BASE_COMMIT=$(git merge-base $BASE_BRANCH HEAD)
   ```

2. **Run change impact:**
   ```bash
   reqvire change-impact --git-commit=$BASE_COMMIT --json > /tmp/impact.json
   ```

3. **For each changed requirement:**

   Get details:
   ```bash
   reqvire summary --filter-id="<requirement-id>" --json
   ```

   Extract:
   - Requirement content
   - verifiedBy relations (tests to run)
   - satisfiedBy relations (code to update)
   - derivedFrom relations (context)

4. **For each verification:**

   Get test paths:
   ```bash
   reqvire summary --filter-id="<verification-id>" --json
   ```

   Extract satisfiedBy relations (test files).

5. **Generate TodoWrite task plan:**

   **For new requirements:**
   ```
   ☐ Implement "{Requirement Name}" ({REQ-ID})
     ☐ Review requirement: [link to blob]
     ☐ Implement functionality
     ☐ Run tests: {test paths}
     ☐ Add satisfiedBy relation
     ☐ Validate: reqvire validate
   ```

   **For modified requirements:**
   ```
   ☐ Update "{Requirement Name}" ({REQ-ID})
     ☐ Review changes: [link to blob]
     ☐ Review code: {satisfiedBy paths}
     ☐ Update implementation
     ☐ Run tests: {test paths}
     ☐ Validate: reqvire validate
   ```

6. **Generate git blob links:**
   ```bash
   REPO_URL=$(git remote get-url origin | sed 's/\.git$//' | sed 's/git@github.com:/https:\/\/github.com\//')
   BLOB_URL="${REPO_URL}/blob/${BASE_COMMIT}/${file_path}#${element-anchor}"
   ```

7. **Present task plan:**
   - Phase 1: New requirements to implement
   - Phase 2: Modified requirements to update
   - Phase 3: Affected verifications to review

## Task Plan Structure

```markdown
# Implementation Task Plan

**Base**: {base_branch}@{base_commit}
**Feature**: {current_branch}

## Summary
- New requirements: X
- Modified requirements: Y
- Tests to run: Z

## Tasks
{TodoWrite formatted tasks}
```

## Notes

- Task plan uses TodoWrite format for tracking
- Links to exact requirement versions via git blob URLs
- Repository-agnostic: no technology assumptions
- Always read full requirements, not just summaries
