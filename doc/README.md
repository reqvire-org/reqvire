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
├── specifications/    # Reqvire's own requirements and model
├── tests/             # End-to-end tests
└── doc/              # Contributor documentation (this folder)
```

## Key Resources

- **[Project Specifications](../specifications/README.md)** - Reqvire's own requirements model
- **[Architecture](../specifications/Architecture.md)** - System architecture documentation
- **[E2E Tests Guide](../tests/CLAUDE.md)** - How to write and run tests
- **[GitHub Issues](https://github.com/reqvire-org/reqvire/issues)** - Report bugs and request features
- **[GitHub Discussions](https://github.com/reqvire-org/reqvire/discussions)** - Ask questions and share ideas

## Contribution Workflow

1. **Read** the [Contributing Guide](./CONTRIBUTING.md)
2. **Fork** the repository
3. **Create** a feature branch
4. **Follow MBSE workflow** (see [CLAUDE.md](../CLAUDE.md)):
   - Add requirements first
   - Define verifications
   - Implement tests
   - Write code
   - Link implementation to requirements
5. **Test** your changes
6. **Submit** a pull request

## Need Help?

- Review the [user documentation](https://www.reqvire.org) to understand how Reqvire works
- Check the [CLAUDE.md](../CLAUDE.md) guide for development patterns
- Open a [GitHub Discussion](https://github.com/reqvire-org/reqvire/discussions) to ask questions
- Join our community to connect with other contributors

---

Thank you for contributing to Reqvire!
