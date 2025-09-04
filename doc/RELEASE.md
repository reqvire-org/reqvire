# Release Process

This document outlines the automated release process for Reqvire, including version management, automated tagging, and binary distribution.

## Automated Release Workflow

The release process follows a structured Git workflow with automated tagging and CI/CD integration:

```
Feature Branch → Main Branch (via PR) → Release Branch → Auto-Tag → Release
```

## Step-by-Step Release Process

### 1. Version Update (on Feature Branch)

Create a feature branch for the version update:

```bash
# Create release branch
git checkout -b release/v0.3.1

# Update version (patch/minor/major)
make update-patch    # 0.3.0 → 0.3.1
make update-minor    # 0.3.0 → 0.4.0  
make update-major    # 0.3.0 → 1.0.0

# Commit version changes with Cargo.lock update
make version-commit
```

**What `make version-commit` does:**
- Updates `Cargo.lock` (resolves CI `--locked` conflicts)
- Builds and tests the project for verification
- Commits version changes to current branch
- Pushes branch to origin
- Provides next step instructions

### 2. Create Pull Request

Create a PR to merge the version update into main:

```bash
# Using GitHub CLI
gh pr create --base main --head release/v0.3.1 \
  --title "Update version to v0.3.1" \
  --body "Version bump for release v0.3.1"
```

**Review and merge the PR** to bring the new version into the main branch.

### 3. Trigger Automated Release

After the PR is merged, trigger the automated release:

```bash
# Switch to main and get latest
git checkout main
git pull origin main

# Trigger automated release
make release
```

**What `make release` does:**
- Switches to `release` branch
- Pulls latest `release` branch  
- Merges `main` into `release` with no-fast-forward
- Pushes to `release` branch
- **Triggers GitHub Action for auto-tagging**

### 4. Automated Steps (GitHub Actions)

When code is pushed to the `release` branch, GitHub Actions automatically:

1. **Auto-Tag Workflow** (`.github/workflows/auto_tag_release.yml`):
   - Extracts version from `Cargo.toml`
   - Creates git tag `v<version>` 
   - Pushes tag to trigger release workflow

2. **Release Workflow** (`.github/workflows/release.yml`):
   - Builds binaries for multiple platforms (Linux, macOS)
   - Runs comprehensive tests
   - Creates GitHub Release with binaries attached
   - Publishes release artifacts

## Available Makefile Commands

| Command | Description |
|---------|-------------|
| `make update-patch` | Increment patch version (0.3.0 → 0.3.1) |
| `make update-minor` | Increment minor version (0.3.0 → 0.4.0) |
| `make update-major` | Increment major version (0.3.0 → 1.0.0) |
| `make version-commit` | Commit version changes on feature branch |
| `make release` | Merge main → release (triggers auto-tag) |
| `make prepare-release` | Update Cargo.lock, build, test (standalone) |
| `make create_tag` | Manual tag creation (backup method) |

## Cargo.lock Conflict Resolution

The automated process resolves common CI/CD issues with Cargo.lock:

**Problem**: CI fails with `"the lock file needs to be updated but --locked was passed"`

**Solution**: 
- `make version-commit` runs `cargo update` without `--locked`
- Both `Cargo.toml` and `Cargo.lock` are committed together
- CI builds against the committed lock file state

## Branch Protection and Security

The workflow respects branch protection rules:

- **Direct pushes to main**: Not allowed (uses PR workflow)
- **Release branch**: Can be pushed to directly for releases
- **Tag creation**: Automated via GitHub Actions (no manual intervention)
- **Permissions**: GitHub Actions has `contents: write` for tagging

## Manual Override (Emergency)

If automation fails, manual release is possible:

```bash
# Manual tag creation
git checkout release
make create_tag

# Or completely manual
git tag -a v0.3.1 -m "Release version v0.3.1"
git push origin v0.3.1
```

## Verification

After release, verify:

1. **Tag created**: Check GitHub tags page
2. **Release published**: Check GitHub releases page  
3. **Binaries available**: Download and test binaries
4. **Version consistency**: Verify tag matches Cargo.toml version

## Example Complete Release

```bash
# 1. Create release branch and update version
git checkout -b release/v0.3.1
make update-patch
make version-commit

# 2. Create and merge PR
gh pr create --base main --head release/v0.3.1 --title "Update version to v0.3.1"
# Review and merge PR in GitHub

# 3. Trigger automated release  
git checkout main
git pull origin main
make release

# 4. Wait for GitHub Actions to complete
# Check GitHub releases page for published release
```

This process ensures consistent, reliable, and automated releases while maintaining proper Git workflow and branch protection.