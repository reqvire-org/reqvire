# Content Extracted from README.md for Website Documentation

This file contains content extracted from README.md that should be moved to www.reqvire.org.

---

## AI Compatibility

Reqvire is built to be **AI-friendly from the ground up**. Its use of **structured Markdown** with clearly defined sections, headings, metadata, and conventions makes it ideal for use with modern **Large Language Models (LLMs)** and other AI systems.

Because **requirements** are written in plain text with predictable patterns, AI tools can easily:
- Parse and understand system models
- Analyze relationships between artifacts
- Generate meaningful suggestions or summaries
- Validate consistency and completeness across the model

### Example AI Use Cases

#### 1. Requirement Analysis
LLMs can analyze Markdown-based requirements to identify missing elements, suggest clearer phrasing, propose edge cases, and generate acceptance criteria or verification steps.

#### 2. Architecture Suggestion
By understanding relationships in architecture files, AI can detect inconsistent component relationships, propose modular design improvements, and suggest missing interfaces or dependencies.

#### 3. Traceability & Impact Prediction
AI tools can automatically trace which tests or requirements are impacted by changes, flag affected downstream areas, and summarize model diffs for engineering teams.

#### 4. Test Coverage Assistance
Based on requirements and use cases, AI can recommend missing test scenarios, generate test case outlines, and flag unverified requirements.

#### 5. Code Generation Assistance
AI can leverage structured requirements for:
- **Context-Aware Generation**: Understanding the "why" behind every component through Reqvire's traceability structure
- **Specification-Driven Coding**: Generating code directly tied to specific requirements, reducing guesswork or misinterpretation
- **Verification-Backed Validation**: Referring to defined verifications to ensure implementation meets intended behavior
- **Trace-Based Refactoring**: Assisting in propagating requirement changes through the codebase and related artifacts

#### 6. Report Generation
AI can automatically generate traceability reports, summarize system architecture, and prepare documentation for releases or reviews.

### Human in Control — Always

While Reqvire empowers AI tools to act as smart collaborators, the **human engineer remains the system's captain** — setting direction, making decisions, and approving outcomes.

Reqvire ensures that:
- Every AI-suggested change is traceable and reviewable
- System evolution remains understandable and documented
- AI tooling acts in service of the engineer's intent, not in place of it

---

## Diagram & Traceability Automation

Reqvire automatically parses structured Markdown to generate:

- **Architecture diagrams** (component hierarchies, interfaces, dependencies)
- **Traceability matrices** linking requirements to architecture, verifications, and tests
- **Impact reports** showing what was changed, and what it affects
- **Requirement flow diagrams** visualizing hierarchical relationships and derivations
- **Verification coverage maps** showing which requirements are verified and how
- **Change propagation analysis** identifying all downstream effects of modifications
- **Cross-reference reports** tracking bidirectional relationships between elements
- **Compliance matrices** mapping requirements to standards, regulations, or policies
- **Model summary reports** providing overview statistics and health metrics
- **Interactive HTML documentation** with clickable diagrams and searchable content
- **Mermaid diagram exports** for integration with documentation platforms
- **JSON/CSV exports** for integration with external tools and dashboards

---

## CI/CD Integration

Reqvire is designed for automation. In typical Git-based workflows, you can:

- **Validate Markdown structure** using static checks
- **Auto-generate diagrams** during CI builds or PRs
- **Generate traceability and change impact reports** for stakeholder review
- **Block PRs** if requirements are incomplete or broken
- **Package models** into versioned documentation releases
- **Integrate with GitHub Actions/GitLab CI** for automated validation
- **Generate reports** that become part of your release artifacts

---

## Installation

### Quick Installation

Run the following command in your terminal:
```bash
curl -fsSL https://raw.githubusercontent.com/reqvire-org/reqvire/main/scripts/install.sh | bash
```

### Quick Start Tutorial

```bash
# 1. Create your first project
mkdir my-project && cd my-project
git init

# 2. Create specifications directory
mkdir -p specifications/Verifications

# 3. Write your requirements
# Create your first requirement file
```

### Installation From Source (Linux and macOS)

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
   git clone https://github.com/reqvire-org/reqvire.git
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

### Using Pre-built Binaries

1. **Download the Latest Release**
   - Visit the [Releases page](https://github.com/reqvire-org/reqvire/releases) on GitHub
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
