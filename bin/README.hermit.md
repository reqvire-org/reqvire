# Hermit environment

This is a [Hermit](https://github.com/cashapp/hermit) bin directory.

The symlinks in this directory are managed by Hermit and will automatically
download and install Hermit itself as well as packages. These packages are
local to this environment.

Run `make devenv` at the repository root to refresh package manifests and install
the pinned tools, including Bun 1.4.2 and Node 24.22.1. This preserves the linked
package versions. Then run `source bin/activate-hermit` to put those tools on this
shell's PATH; a Make recipe cannot change its parent shell's environment.
