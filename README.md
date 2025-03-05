# ReqFlow

**ReqFlow** is an **AI-driven framework for system modeling and project specifications**, built to seamlessly integrate with modern agile development workflows. While it currently focuses on **requirements, specifications, and traceability**, its vision is much broader—evolving into a fully **AI-powered Model-Based Systems Engineering (MBSE) framework**, where **AI assistance and automation** play a central role in designing, analyzing, and implementing entire system architectures.

It’s built for **product managers, architects, developers, and AI assistants** to collaborate effectively, keeping system models and requirements in sync with real-world implementation.  

Unlike traditional MBSE tools, ReqFlow is **lightweight, Git-native, and practical**—inspired by **MBSE and SysML**, but without the complexity that slows teams down. It integrates **AI-powered automation, LLM-driven analysis, and version control**, making it easy to track, refine, and evolve your system models as your project grows.  

Built around **Git workflows**, ReqFlow ensures that **system models are always traceable, versioned, and actionable**, evolving alongside project development. It provides a **structured yet flexible approach to system modeling, specifications, and requirements management**, helping teams move fast without losing control.


---

## Why ReqFlow?

- **AI-First**: ReqFlow isn’t just a requirements tool—it’s a **methodology and framework** designed to integrate AI into every phase of software and system development. It enables **AI-driven requirement implementation, code generation, architecture analysis, and automated validation**, bridging the gap between specification and execution.
- **Git-Native**: Built to work seamlessly within Git workflows, ensuring requirements evolve alongside code with full traceability and version control.
- **Agile & Lightweight**: Uses **Markdown-based artifacts** for a balance of **human readability and machine processability**, making collaboration easy.
- **Automation-Ready**: Supports **traceability, validation, and diagram generation**, reducing manual effort and improving accuracy.
- **Extensible & Integrable**: Works with **GitHub, GitLab, CI/CD pipelines, and AI-powered development tools**, enhancing automation and continuous improvement.
- **MBSE-Inspired, Not MBSE-Exclusive**: Draws from **Model-Based Systems Engineering (MBSE) and SysML**, but is optimized for **modern, software-driven teams** rather than traditional, heavyweight modeling approaches.
- **AI-Powered Collaboration**: Designed for **seamless interaction between humans and AI**, enabling AI tools to assist with **requirement generation, validation, implementation, and impact analysis**, accelerating development cycles.

---
## ReqFlow as a Tool  

ReqFlow is a toolset that provides a structured approach to **requirements and specifications management** within modern development workflows. Rather than being a modeling tool itself, ReqFlow enables teams to **define, structure, and trace system models through specifications**. It facilitates:  

- **Requirements and specifications management** using human-readable **Markdown**.  
- **Automated traceability, change impact analysis, and versioning** to maintain consistency.  
- **Linking requirements to issues, pull requests, and test cases** for real-time alignment with development.  
- **Generating diagrams, traceability matrices, and HTML outputs** for enhanced visibility.  
- **Validating document structure, relationships, and compliance rules** to ensure model integrity.  
- **AI-driven workflows** to **generate, refine, analyze, and implement requirements** with intelligent assistance.  
- **AI-assisted code generation, architecture analysis, and compliance validation**, accelerating implementation and reducing errors.  

By embedding these capabilities into **Git workflows**, ReqFlow ensures that **requirements, specifications, and their relationships remain structured, versioned, and actionable**, evolving naturally alongside project development.

---

## ReqFlow as a Language  

ReqFlow provides a **structured yet flexible language** for defining, linking, and managing requirements and specifications. Inspired by **SysML**, it simplifies structured documentation **using Markdown**, enabling:  

- **AI-assisted requirement authoring** with semantic analysis and contextual refinement.  
- **Automated traceability** through structured links, ensuring consistency across documents.  
- **Diagram generation** via **MermaidJS**, making relationships and dependencies easier to visualize.  
- **Seamless collaboration between humans and machines**, with specifications that are both **machine-readable and human-friendly**.  
- **Human-in-the-loop AI-driven code generation, architecture analysis, and compliance validation**, ensuring that requirements stay aligned with design and implementation.  

By adopting **semi-structured conventions**, ReqFlow bridges the gap between **formal modeling and agile documentation**, enabling **automation and AI-driven workflows** without the overhead of traditional modeling tools.  

---

## Get Started

For detailed information about **how to use ReqFlow**, including the **tooling, methodology, and AI-powered workflows**, visit the [documentation](./doc/README.md).


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

### Quick Start

Once you have ReqFlow installed, you can start using it with your project:

1. **Initialize ReqFlow in Your Project**
   ```bash
   reqflow init
   ```
   This creates a basic ReqFlow structure with example requirements

2. **Validate Your Requirements**
   ```bash
   reqflow validate specifications/
   ```

3. **Generate Documentation**
   ```bash
   reqflow specifications/ output/ --html
   ```

4. **Lint and Format Requirements**
   ```bash
   reqflow --lint specifications/
   ```

5. **Generate Index and Diagrams**
   ```bash
   reqflow --lint specifications/ --generate-diagrams
   ```

For detailed usage instructions, refer to the [User Guide](doc/user_guide.md).


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
