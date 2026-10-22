# Contributor Documentation

Welcome to the **Reqvire** contributor documentation! This folder contains information for developers who want to contribute to Reqvire.

## For Users

Looking for how to **use** Reqvire? Visit **[www.reqvire.org](https://www.reqvire.org)** for:
- Installation instructions
- User guides and tutorials
- Command reference
- Configuration options
- Best practices

## For Contributors

This documentation is for those who want to contribute to Reqvire's development:

- **[Contributing Guide](./CONTRIBUTING.md)** - How to contribute code, tests, and documentation
- **[Code of Conduct](./code_of_conduct.md)** - Our community standards
- **[Contributor License Agreement](./CLA.md)** - Legal terms for contributions
- **[Codex Skills](./CODEX_SKILLS.md)** - Reqvire Codex skill package and global installation
- **[Release Process](./RELEASE.md)** - How releases are created
- **[Development Guide](../CLAUDE.md)** - Technical architecture and development workflow

## Development Quick Start

### Prerequisites

Git, Bash, Make and a native C/C++ linker/toolchain are operating-system
prerequisites. Browser checks also need Chromium and its system libraries.

The optional [Hermit](https://cashapp.github.io/hermit/usage/get-started/)
environment pins Rust/Cargo 1.92.0, Node.js 24.22.1, Bun 1.3.13, Python 3.12.3,
ripgrep 15.2.0 and Google Repo 2.70. The Repo source is pinned to an exact Git
commit and exposed as `REQVIRE_TEST_REPO_SOURCE`, so the multi-repository tests
create real Repo clients entirely from local objects.

```bash
git clone https://github.com/reqvire-org/reqvire.git
cd reqvire
# No global Hermit installation is required by the committed bootstrap wrapper.
source bin/activate-hermit
hermit install
```

Activation affects the current shell. The first installation downloads tools;
subsequent runs reuse the Hermit cache. It does not install OS libraries, model
repositories, Cargo dependencies, frontend packages or browser binaries. To use
an existing toolchain instead, install the listed tools yourself and set
`REQVIRE_TEST_REPO_SOURCE` to a local Git checkout of Google's `git-repo`.
Hermit is a development convenience, not a prerequisite for installed Reqvire.

### Building and testing

Build the Explorer before Rust so the binary embeds the current UI:

```bash
make build
cargo test --locked
REQVIRE_BIN="$PWD/target/debug/reqvire" ./tests/run_tests.sh test-mcp-repo-workspaces
```

The first Repo test setup uses the Hermit-provided local Repo source; the suite
itself performs no network fetch. Full browser coverage needs a browser:

```bash
cd explorer
npm exec -- playwright install --with-deps chromium --only-shell
cd ..
REQVIRE_TEST_BROWSER=/absolute/path/to/chrome-headless-shell ./tests/run_all.sh
```

For the public website:

```bash
cd website
bun install --no-save
bun run build
bun run dev
```

To install the built CLI locally:

```bash
cargo install --locked --path crates/reqvire-cli
```

### Project Structure

```
reqvire/
├── crates/
│   ├── reqvire-core/  # Core library package `reqvire` (parsing, model, analysis)
│   ├── reqvire-cli/   # Command-line package; builds the `reqvire` binary
│   └── o-kernel/      # Reusable RDF/OWL/SHACL kernel
├── system-model/      # Reqvire's own requirements and model
├── tests/             # End-to-end tests
└── doc/               # Contributor documentation (this folder)
```

## Key Resources

- **[Project Specifications](../system-model/README.md)** - Reqvire's own requirements model
- **[Architecture](../system-model/Architecture.md)** - System architecture documentation
- **[E2E Tests Guide](../tests/CLAUDE.md)** - How to write and run tests
- **[GitHub Issues](https://github.com/reqvire-org/reqvire/issues)** - Report reproducible bugs and concrete behavior problems
- **[GitHub Discussions](https://github.com/reqvire-org/reqvire/discussions)** - Ask questions and suggest model, ontology, requirement, verification, or architecture improvements
- **[Collaborator Interest](https://github.com/reqvire-org/reqvire/discussions)** - Express interest in future collaborator roles

## Contribution Workflow

1. **Read** the [Contributing Guide](./CONTRIBUTING.md)
2. **Open or join** an issue discussion for reproducible bugs and concrete behavior problems
3. **Use GitHub Discussions** for model-improvement suggestions and design tradeoffs
4. **Use GitHub Discussions** to express collaborator interest
5. **Do not open pull requests** for code, requirements, specifications, ontologies, verifications, or other model changes
6. **For maintainer-authored changes, follow the MBSE workflow** (see [CLAUDE.md](../CLAUDE.md)):
   - Add requirements first
   - Define verifications
   - Implement tests
   - Write code
   - Link implementation to requirements
7. **Test** your changes
8. **Use issues** to share reproduction details and analysis

## Need Help?

- Review the [user documentation](https://www.reqvire.org) to understand how Reqvire works
- Check the [CLAUDE.md](../CLAUDE.md) guide for development patterns
- Open a [GitHub Discussion](https://github.com/reqvire-org/reqvire/discussions) to ask questions or suggest model improvements
- Open a GitHub Discussion to express interest in future collaborator roles
- Open a reproducible bug report when behavior does not match the documentation

---

Thank you for helping improve Reqvire through reproducible reports and thoughtful model discussion.
