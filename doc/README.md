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

**Install Rust and Cargo**

Install Rust using rustup:
```bash
curl -sSf https://sh.rustup.rs | sh
```

Follow the on-screen prompts to complete the installation, then verify:
```bash
rustc --version
cargo --version
```

### Building from Source

**Clone the repository**
```bash
git clone https://github.com/reqvire-org/reqvire.git
cd reqvire
```

**Build the project**
```bash
cargo build
```

**Run tests**
```bash
cargo test
```

**Run Reqvire locally**
```bash
cargo run -- --help
```

**Install the binary (optional)**
```bash
cargo install --path .
```
This will install the binary to `~/.cargo/bin/reqvire`

### Project Structure

```
reqvire/
├── core/              # Core library (parsing, model, analysis)
├── cli/               # Command-line interface
├── system-model/    # Reqvire's own requirements and model
├── tests/             # End-to-end tests
└── doc/              # Contributor documentation (this folder)
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
