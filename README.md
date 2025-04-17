# ReqFlow

**ReqFlow** is a **framework for system modeling and project specifications**, designed to seamlessly integrate with modern agile development workflows and AI tools. While it currently focuses on **requirements, specifications, and traceability**, its vision extends far beyond—evolving into a fully **AI-powered Model-Based Systems Engineering (MBSE) framework**, where **AI assistance and automation** play a central role in designing, analyzing, and implementing system architectures.

It’s built for **product managers, architects, developers, and AI assistants** to collaborate effectively, keeping system models and requirements in sync with real-world implementation.  

Unlike traditional MBSE tools, ReqFlow is **lightweight, Git-native, and practical** - inspired by **MBSE and SysML**, but without the complexity that slows teams down. 
By adopting **semi-structured conventions**, ReqFlow bridges the gap between **formal modeling and agile documentation**, enabling **automation and AI-driven workflows** without the overhead of traditional modeling tools.  

---

## Why ReqFlow?

ReqFlow is a Git-native, AI-enabled **procces, toolset, and language** for modern requirements and system modeling. 

- **Git-Native** – Embeds directly into Git workflows, ensuring requirements evolve alongside code with full traceability and version control.  
- **Agile & Lightweight** – Uses Markdown-based artifacts for a balance of human readability and machine processability, making collaboration seamless.  
- **Automation-Ready** – Supports traceability, impact analysis, validation, and diagram generation, reducing manual effort while improving consistency.  
- **Extensible & Integrable** – Works with GitHub, GitLab, CI/CD pipelines, and AI-driven development tools, enabling automation and continuous refinement.  
- **MBSE-Inspired, Not MBSE-Exclusive** – Draws from Model-Based Systems Engineering (MBSE) and SysML but is tailored for modern, software-driven teams, avoiding unnecessary complexity.  
- **Traceability & Impact Awareness** – Links every requirement, specification, and decision to ensure full versioning and impact analysis.  
- **Seamless Development Integration** – Connects requirements to issues, pull requests, and test cases, ensuring real-world alignment.  
- **Automated Documentation & Visualization** – Generates diagrams, traceability matrices, and structured reports for enhanced visibility.  
- **Diagram Generation & Relationship Mapping** – Supports structured linking and MermaidJS to visualize dependencies and improve clarity.  
- **AI-enabled Development & Collaboration** – Enhancing teamwork through AI-enabled requirement authoring, architecture analysis, code generation, validation, and impact analysis.


---

## Get Started

For detailed information about **how to use ReqFlow** visit the [documentation](./doc/README.md).


### Installation

To set up ReqFlow, follow these steps:

#### From Source (Linux and macOS)

1. **Install Rust and Cargo**
   - Install Rust using rustup:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Follow the on-screen prompts to complete the installation
   - Verify the installation:
     ```bash
     rustc --version
     cargo --version
     ```

2. **Clone the ReqFlow Repository**
   ```bash
   git clone https://github.com/reqflow/reqflow.git
   cd reqflow
   ```

3. **Build ReqFlow**
   ```bash
   cargo build --release
   ```

4. **Install the Binary** (optional)
   ```bash
   cargo install --path .
   ```
   This will install the binary to `~/.cargo/bin/reqflow`

#### Using Pre-built Binaries

1. **Download the Latest Release**
   - Visit the [Releases page](https://github.com/reqflow/reqflow/releases) on GitHub
   - Download the appropriate binary for your platform:
     - Linux: `reqflow-linux-x86_64.tar.gz`
     - macOS: `reqflow-macos-x86_64.tar.gz` or `reqflow-macos-arm64.tar.gz` (for Apple Silicon)

2. **Extract the Binary**
   ```bash
   tar -xzf reqflow-<platform>.tar.gz
   ```

3. **Move to a Directory in Your PATH**
   ```bash
   sudo mv reqflow /usr/local/bin/
   ```

4. **Verify the Installation**
   ```bash
   reqflow --version
   ```

## Contributing

We welcome contributions to ReqFlow! Whether it's improving the methodology, enhancing the tools, or refining the language, your input is valuable. Here's how to contribute:

1. **Read the Contribution Guidelines**:
   - Start by reviewing our [Contribution Guidelines](./doc/CONTRIBUTING.md) to understand the process and expectations.

2. **Fork the Repository**:
   - Create your own copy of the repository by forking it.

3. **Create a Feature Branch**:
   - Work on your changes in a dedicated branch:
     ```bash
     git checkout -b feature/your-feature-name
     ```

4. **Test Your Changes**:
   - Ensure that your changes are fully tested and do not break existing functionality.

5. **Submit a Pull Request**:
   - Once your changes are ready, submit a pull request with a clear and detailed description of what you've implemented or fixed.

6. **Collaborate**:
   - Be responsive to feedback and collaborate with the maintainers to get your pull request merged.

For more details, refer to the [Contributing Guide](./doc/CONTRIBUTING.md).
