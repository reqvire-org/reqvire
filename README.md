# Reqvire

<div align="center">

![Reqvire Logo](doc/diagram_1.png)

**The AI-Native Requirements Platform for Modern Engineering Teams**

[![Build Status](https://img.shields.io/github/actions/workflow/status/Reqvire/reqvire/pr.yml?branch=main&style=flat-square&logo=github)](https://github.com/Reqvire/reqvire/actions)
[![Latest Release](https://img.shields.io/github/v/release/Reqvire/reqvire?style=flat-square&logo=github&color=blue)](https://github.com/Reqvire/reqvire/releases)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat-square)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)

[![GitHub Stars](https://img.shields.io/github/stars/Reqvire/reqvire?style=flat-square&logo=github&color=yellow)](https://github.com/Reqvire/reqvire/stargazers)
[![GitHub Forks](https://img.shields.io/github/forks/Reqvire/reqvire?style=flat-square&logo=github&color=blue)](https://github.com/Reqvire/reqvire/network)
[![Contributors](https://img.shields.io/github/contributors/Reqvire/reqvire?style=flat-square&logo=github&color=green)](https://github.com/Reqvire/reqvire/graphs/contributors)

[📖 **Documentation**](./doc/README.md) • [🚀 **Quick Start**](#get-started) • [💬 **Community**](https://github.com/Reqvire/reqvire/discussions) • [🐛 **Report Bug**](https://github.com/Reqvire/reqvire/issues/new?template=bug_report.yml)

</div>

---

## What is Reqvire?

**Reqvire** transforms how engineering teams manage requirements by bringing them into the **Git workflow** with **AI-native** markdown documents. No more requirements living in separate tools—everything lives alongside your code with full version control, automated traceability, and seamless CI/CD integration.

```bash
# Requirements as Code - it's that simple
reqvire validate
reqvire generate-diagrams
reqvire change-impact --git-commit=HEAD~1
```

### **Requirements as Code**
Write requirements in **structured Markdown** that's both human-readable and AI-friendly. Version control everything with Git.

### **AI-First Engineering**  
Built from day one for **Large Language Models**. Your requirements become the perfect context for AI-assisted development.

### **Automated Traceability**
Every requirement automatically traces to code, tests, and architecture. Change impact analysis with every commit.

### **Zero Context Switch**
Requirements, code, and documentation in one repository. Review requirement changes like code changes.


---
<sub>Figure 1: Example Diagram (click image to browse requirements)</sub>
[![Example Diagram](doc/diagram_1.png)](specifications/SpecificationsRequirements.md#specifications-requirements)

---

## Why reqvire?

- **Git-Native Workflow** – Requirements, specifications, and traceability artifacts live alongside code with full version control and collaboration via Git.
- **Agile & Human-Friendly** – Uses Markdown-based, semi-structured documents that are both readable and AI friendly.
- **Automation-Ready** – Supports traceability, impact analysis, validation, and documentation generation to reduce manual effort and enforce consistency.
- **Flexible Integration** – Seamlessly connects with GitHub, GitLab, CI/CD pipelines, and AI-driven tools to support continuous delivery and refinement.
- **MBSE-Inspired, Agile-Optimized** – Combines the rigor of Model-Based Systems Engineering with the speed and simplicity demanded by modern software teams.
- **Traceability & Impact Awareness** – Every change, decision, and dependency is trackable—enabling confident change management and system evolution.
- **Built for Developer Workflows** – Links directly to issues, pull requests, and test cases, keeping specifications aligned with real development activity.
- **Visual Modeling & Reporting** – Automatically generates diagrams, traceability matrices, and structured reports using tools like MermaidJS.
- **AI-Augmented Engineering** – Assists with requirement authoring, architecture analysis, validation, and code generation to boost both quality and velocity.

---

<sub>Figure 2: Example Change Impact Report (click image to read specifications)</sub>

[![Example Change Impact Report](doc/change_impact_report_pr.png)](specifications/SpecificationsRequirements.md#requirements-change-propagation)

---

## Why Choose Reqvire?

<table>
<tr>
<th>Challenge</th>
<th>Traditional Tools</th>
<th>Reqvire Solution</th>
</tr>
<tr>
<td><strong>Requirements live in separate tools</strong></td>
<td>Jira, Confluence, Word docs</td>
<td> Requirements live with your code</td>
</tr>
<tr>
<td><strong>Manual traceability maintenance</strong></td>
<td>Spreadsheets, manual linking</td>
<td> Automated traceability & change impact</td>
</tr>
<tr>
<td><strong>AI tools can't understand requirements</strong></td>
<td>Proprietary formats, PDFs</td>
<td> AI-native structured markdown</td>
</tr>
<tr>
<td><strong>Requirements changes break things</strong></td>
<td>Hope and manual checking</td>
<td> Automated impact analysis</td>
</tr>
<tr>
<td><strong>Complex enterprise tools</strong></td>
<td>Months of setup, expensive licenses</td>
<td> 5-minute setup, open source</td>
</tr>
<tr>
<td><strong>No version control for requirements</strong></td>
<td>Document versions, email chains</td>
<td>📝 Git-native with full history</td>
</tr>
</table>

### Perfect for These Teams

- **DevOps & System Engineers** building compliant, traceable systems
- **System Architects** managing complex requirements hierarchies  
- **AI Engineers** needing requirements as LLM context
- **Compliance Teams** requiring automated traceability matrices
- **Technical Writers** wanting markdown-based documentation workflows

---

## Get Started

> **"Reqvire transformed how we handle requirements. Finally, everything lives with our code!"**  
> — Engineering Team at GrapheneDB (First production user)

### Quick Start (5 minutes)

```bash
# 1. Install Reqvire
curl -fsSL https://raw.githubusercontent.com/Reqvire/reqvire/main/scripts/install.sh | bash

# 2. Create your first project
mkdir my-project && cd my-project
git init

# 3. Initialize Reqvire
echo "specifications_folder: specifications" > reqvire.yaml
mkdir specifications

# 4. Write your first requirement
echo "### User Authentication
The system shall authenticate users via OAuth2." > specifications/requirements.md

# 5. Validate and generate diagrams
reqvire validate
reqvire generate-diagrams
```

**[Full Documentation](./doc/README.md)** • 📋 **[Use Cases](./specifications/Usecases.md)** • 🎥 **[Demo Video](#)** (coming soon)

### Installation

To set up reqvire, follow these steps:

#### Run install script (Linux and macOS)

Run the following command in your terminal:
```
curl -fsSL https://raw.githubusercontent.com/Reqvire/reqvire/main/scripts/install.sh | bash
```

#### From Source (Linux and macOS)

1. **Install Rust and Cargo**
   - Install Rust using rustup:
     ```bash
     curl -sSf https://sh.rustup.rs | sh
     ```
   - Follow the on-screen prompts to complete the installation
   - Verify the installation:
     ```bash
     rustc --version
     cargo --version
     ```

2. **Clone the reqvire Repository**
   ```bash
   git clone https://github.com/Reqvire/reqvire.git
   cd reqvire
   ```

3. **Build reqvire**
   ```bash
   cargo build --release
   ```

4. **Install the Binary** (optional)
   ```bash
   cargo install --path .
   ```
   This will install the binary to `~/.cargo/bin/reqvire`

#### Using Pre-built Binaries

1. **Download the Latest Release**
   - Visit the [Releases page](https://github.com/Reqvire/reqvire/releases) on GitHub
   - Download the appropriate binary for your platform:
     - Linux: `reqvire-linux-x86_64.tar.gz`
     - macOS (Apple Silicon): `reqvire-darwin-arm64.tar.gz`
     - macOS: `reqvire-darwin-x86_64.tar.gz`       

2. **Extract the Binary**
   ```bash
   tar -xzf reqvire-<platform>.tar.gz
   ```

3. **Move to a Directory in Your PATH**
   For most Linux and Intel-based macOS systems:
   ```bash
   sudo mv reqvire /usr/local/bin/
   ````
   
    For Apple Silicon (M1/M2) macOS:
    ```bash
    sudo mv reqvire /opt/homebrew/bin/   
    ```
    
4. **Verify the Installation**
   ```bash
   reqvire --version
   ```
    
### Configuration

Create `reqvire.yaml` in the root of the git repo with following minimal content:
```
paths:

  # Path to the user requirements root folder
  user_requirements_root_folder: "specifications"
  
  # Glob patterns to exclude from requirements processing
  excluded_filename_patterns:
    - "Usecases.md"
    - "**/Logical*.md"
    - "**/Physical*.md"
    - "**/TODO.md"  
    
style:
  # Theme for HTML output (default, dark, light)
  theme: "default"
  
  # Maximum width for HTML output
  max_width: 1200
  
  # Optional path to custom CSS file
  # custom_css: "path/to/custom.css"
  
  # Diagram direction (TD for top-down, LR for left-to-right)
  diagram_direction: "LR"

  # If diagrams click links should be blobs to work from GitHub console
  diagrams_with_blobs: false      
```

Create `output` directory.


    
## Contributing

We welcome contributions to **Reqvire**! Whether you're improving the tooling, refining specifications, enhancing test cases, or shaping modeling conventions, your input is highly valued.

### Quick Contribution Links
- **[Report a Bug](https://github.com/Reqvire/reqvire/issues/new?template=bug_report.yml)**
- **[Request a Feature](https://github.com/Reqvire/reqvire/issues/new?template=feature_request.yml)**
- **[Ask a Question](https://github.com/Reqvire/reqvire/discussions)**
- **[Improve Documentation](./doc/CONTRIBUTING.md)**

### Contribution Discipline

To maintain **consistency**, **traceability**, and **quality** in **Reqvire**, we follow a strict **contribution discipline** that ensures the system model, tests, and verifications evolve alongside the code.

### Contribution Workflow

1. **Read the Contribution Guidelines**:
   - Begin by reviewing our [Contribution Guidelines](./doc/CONTRIBUTING.md) for details on processes, coding standards, and expectations.

2. **Fork the Repository**:
   - Create your own copy of the repository by forking it.

3. **Create a Feature Branch**:
   - Work on your changes in a dedicated branch:
     ```bash
     git checkout -b feature/your-feature-name
     ```

4. **Maintain Requirements, Tests, and Verifications**:
   - **Update or create requirements**:
     - For any **new code**, **feature**, or **change** in the codebase, always **create new requirements** or **update existing ones** to reflect those changes.
     - Requirements should be added in the appropriate **User Requirements** or **System Requirements** folders.
     - Requirments must be approved before code can be implemented or existing functionality changed
   - **Update verifications**:
     - For every **new requirement** or **E2E test**, create or update the corresponding **verification cases** in the `specifications/verifications/` directory.
     - Verifications ensure that **requirements** are linked to **tests**, and all features are properly validated.     
   - **Add or update end-to-end (E2E) tests**:
     - If your changes introduce **new functionality** or modify existing behavior that is not covered by tests, ensure to **add E2E tests** in the `tests/` directory.
     - Tests must validate the **expected behavior** of your feature or change.
5. **Test Your Changes**:
   - Run **reqvire validation** locally to ensure your requirements and verifications are consistent:
     ```bash
     reqvire --validate
     ```
   - Ensure that your code passes **all tests** and that **no existing functionality breaks**.

6. **Submit a Pull Request (PR)**:
   - Once your changes are ready, submit a pull request with a **clear and detailed description** of what you've implemented or fixed.
   - Include a summary of:
     - **Change impact report if requirements were updated**
     - **New or updated tests**
     - **New or updated verifications**

7. **Collaborate and Iterate**:
   - Engage with maintainers, respond to feedback, and collaborate to refine your PR until it's ready to be merged.

For more details, refer to the [Contributing Guide](./doc/CONTRIBUTING.md).


## Credits

**reqvire** is an open-source project created and maintained by [Ilija Ljubicic](https://github.com/ilijaljubicic). 

### Special Thanks:

- [Juanjo Andres](https://github.com/juanjoandres)  
  For valuable contributions to testing and in in shaping reqvire's direction, especially in the early phases of reqvire's development.

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

<div align="center">

**Built with ❤️ by the Reqvire Community**

[🏠 Website](#) • [📖 Docs](./doc/README.md) • [💬 Community](https://github.com/Reqvire/reqvire/discussions) • [🐛 Issues](https://github.com/Reqvire/reqvire/issues)

⭐ **Star us on GitHub** — it helps more than you know!

</div>

