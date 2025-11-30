<div align="center">

# Reqvire

<img src="doc/logo.png" alt="Reqvire Logo" width="200">

**Requirements-as-context framework for modern engineering teams.**

[![Latest Release](https://img.shields.io/github/v/release/Reqvire/reqvire?style=flat-square&logo=github&color=blue)](https://github.com/reqvire-org/reqvire/releases)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat-square)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[📖 **Documentation**](https://www.reqvire.org) • [🔍 **Browse Model**](https://reqvire-org.github.io/reqvire/) • [🚀 **Quick Start**](#get-started) • [👥 **Contributing**](./doc/README.md)

</div>

---

## What is Reqvire?

**Reqvire** is a lightweight, Git-native **Requirements-as-Context** framework that redefines how modern engineering teams design, build, and maintain software.

It unifies system modeling, requirements management, and AI-assisted development into a single workflow—providing complete traceability, intelligent automation, and faster delivery while staying fully aligned with your codebase.

## 🤖 Now Available for Claude Code

Reqvire is now available as a plugin for Claude Code! Get AI-assisted requirements engineering with specialized skills and commands directly in your Claude Code workflow.

**[Install and learn more →](https://www.reqvire.org/claude_plugin)**

---

## Key Features

### **Requirements-as-Context**
Create a structured, canonical source of truth for your project.  
Reqvire provides a consistent context layer that AI tools can reliably query—so they always understand what needs to be built, where changes belong, and how system components fit together.

### **Requirements-as-Code**
Evolve requirements from static documents into version-controlled, executable artifacts.  
Reqvire keeps your system model alive in Git—no more lost specs, outdated docs, or knowledge locked in transient tickets.

### **Intelligent Engineering**
Bring Model-Based Systems Engineering (MBSE) directly into your Git workflow:

- **Specification-Driven Development**  
  Develop from requirements. Enforce clear specifications and generate code that stays tied to its originating requirements.

- **Automated Traceability**  
  Maintain instant, bidirectional links between requirements, code, tests, and artifacts.

- **Verification & Validation**  
  Track verifications, ensure coverage, and validate that implementations meet intended behavior.

- **Smart Change Propagation**  
  Identify impacted parts of the system and help propagate requirement changes consistently across the model and codebase.

- **Seamless Integration**  
  Works naturally with branches, pull requests, reviews, and CI/CD. No workflow disruption—just added intelligence.


## Get Started

### Installation

```bash
curl -fsSL https://raw.githubusercontent.com/reqvire-org/reqvire/main/scripts/install.sh | bash
```

For detailed installation options (from source, pre-built binaries, etc.), see the [Installation Guide](https://www.reqvire.org/user_guide#installation).

### Next Steps

- **[Documentation](https://www.reqvire.org)** - Learn how to use Reqvire
- **[Browse Model](https://reqvire-org.github.io/reqvire/)** - Explore Reqvire's own specifications

---

## Contributing

We welcome contributions to **Reqvire**! Whether you're fixing bugs, adding features, improving documentation, or refining specifications, your input is highly valued.

### How to Contribute

1. **Read the [Contributing Guide](./doc/README.md)** - Understand our MBSE-based development workflow
2. **Fork and create a branch** - Work on your changes in a dedicated feature branch
3. **Follow the discipline** - Requirements first, then verifications, tests, and code
4. **Submit a PR** - Include clear description and link related issues

**Important**: Reqvire follows an MBSE workflow. All code changes must be accompanied by corresponding requirements and verifications. See our [Contributor Documentation](./doc/README.md) for details.

### Quick Links

- **[Report a Bug](https://github.com/reqvire-org/reqvire/issues/new?template=bug_report.yml)**
- **[Request a Feature](https://github.com/reqvire-org/reqvire/issues/new?template=feature_request.yml)**
- **[Ask a Question](https://github.com/reqvire-org/reqvire/discussions)**
- **[Contributor Guide](./doc/README.md)**

### Contributor License Agreement

All contributors must accept our [Contributor License Agreement](./doc/CLA.md). The CLA process is automated through GitHub PR comments.

---

## Credits

**reqvire** is an open-source project created and maintained by [Ilija Ljubicic](https://github.com/ilijaljubicic).

### Special Thanks:

- [Juanjo Andres](https://github.com/juanjoandres)
  For valuable contributions to testing and in shaping reqvire's direction, especially in the early phases of reqvire's development.

- [GrapheneDB](https://www.graphenedb.com/)
  For **partial sponsorship** and for being the **first user** of reqvire. Their support helped shape the tool's early direction.

## What's Next?

### Join Our Growing Community
- ⭐ **Star us** on GitHub to stay updated
- 💬 **Join discussions** to share your use cases
- 📝 **Contribute** to make Reqvire even better
- 🐦 **Follow updates** on our blog and social media

---

## License

Licensed under the [Apache 2.0 License](LICENSE).

---
