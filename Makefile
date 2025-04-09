# Set default version increment (major, minor, or patch)
VERSION_INCREMENT ?= patch

CARGO_TOML = Cargo.toml

VERSION_FILE = version.tmp

# Bump the version based on the version increment (patch, minor, major)
bump_version:
    @echo "Bumping version in Cargo.toml"
    # Bump version using cargo-release or manually (patch, minor, or major)
    cargo release $(VERSION_INCREMENT) --no-publish --no-verify
    
    # Get the new version from the Cargo.toml
    NEW_VERSION=$(shell grep -oP 'version\s*=\s*"\K[0-9\.]+' $(CARGO_TOML))

    # Display new version
    @echo "New version: $(NEW_VERSION)"

# Create a Git tag based on the bumped version
create_tag: bump_version
    @echo "Creating Git tag for version $(NEW_VERSION)"
    git tag -a v$(NEW_VERSION) -m "Release version v$(NEW_VERSION)"
    git push origin v$(NEW_VERSION)

# Default target to bump the version and create a tag for patch
release_patch: bump_patch create_tag
    @echo "Release created with version $(NEW_VERSION) and tag v$(NEW_VERSION)"

# Default target to bump the version and create a tag for minor
release_minor: bump_minor create_tag
    @echo "Release created with version $(NEW_VERSION) and tag v$(NEW_VERSION)"

# Default target to bump the version and create a tag for major
release_major: bump_major create_tag
    @echo "Release created with version $(NEW_VERSION) and tag v$(NEW_VERSION)"


