# Release Process

This document outlines the simplified release process for Reqvire, including version management and automated binary distribution.

## Simplified Release Workflow

The release process is streamlined into two simple commands:

```
Update Version → Release (commit + tag + push) → GitHub Actions Build
```

## Step-by-Step Release Process

### 1. Update Version

Update the version number in `Cargo.toml`:

```bash
# Update version (patch/minor/major)
make update-patch    # 0.3.0 → 0.3.1
make update-minor    # 0.3.0 → 0.4.0  
make update-major    # 0.3.0 → 1.0.0
```

### 2. Release

Execute the complete release in one command:

```bash
make release
```

**What `make release` does:**
- Updates `Cargo.lock` (resolves CI `--locked` conflicts)
- Builds and tests the project for verification
- Commits version changes
- Creates git tag `v<version>`
- Pushes commit and tag to origin
- **Triggers GitHub Actions for automated build and release**

### 3. Automated Steps (GitHub Actions)

When a tag is pushed, GitHub Actions automatically:

1. **Release Workflow** (`.github/workflows/release.yml`):
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
| `make release` | Complete release: commit, tag, and push |
| `make prepare-release` | Update Cargo.lock, build, test (standalone) |
| `make create_tag` | Manual tag creation (backup method) |

## Cargo.lock Conflict Resolution

The automated process resolves common CI/CD issues with Cargo.lock:

**Problem**: CI fails with `"the lock file needs to be updated but --locked was passed"`

**Solution**: 
- `make release` runs `cargo update` without `--locked`
- Both `Cargo.toml` and `Cargo.lock` are committed together
- CI builds against the committed lock file state

## Manual Override (Emergency)

If automation fails, manual release is possible:

```bash
# Manual tag creation
make create_tag

# Or completely manual
git tag -a v0.3.1 -m "Release version v0.3.1"
git push origin main --tags
```

## Verification

After release, verify:

1. **Tag created**: Check GitHub tags page
2. **Release published**: Check GitHub releases page  
3. **Binaries available**: Download and test binaries
4. **Version consistency**: Verify tag matches Cargo.toml version

## Example Complete Release

```bash
# 1. Update version
make update-patch

# 2. Release everything
make release

# 3. Wait for GitHub Actions to complete
# Check GitHub releases page for published release
```

This simplified process ensures consistent, reliable, and automated releases with minimal manual steps.