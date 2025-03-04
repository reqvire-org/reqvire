# ReqFlow

**ReqFlow** is an **agile** **Model-Based Systems Engineering (MBSE)** framework designed to integrate seamlessly with modern Git workflows.

ReqFlow simplifies MBSE by leveraging Markdown-based artifacts, automation, and Agile principles all within the familiar environment of tools like GitHub, GitLab, and CI/CD pipelines.
This makes ReqFlow intuitive and accessible to engineers, developers, and product managers alike, enabling iterative development and enhanced collaboration.

---

## Why ReqFlow?

- **A Unified Framework**: Combines methodology, tools, and language into a cohesive MBSE solution.
- **Git-Centric**: Built around Git workflows, enabling integration with GitHub, GitLab, Bitbucket, and CI/CD pipelines.
- **Lightweight & Accessible**: Uses Markdown for requirements and models, ensuring readability and ease of use.
- **Methodology**: Follows ISO/IEC/IEEE 15288 principles, tailored to software product development and agile workflows.
- **Agile MBSE**: Facilitates iterative development, continuous integration, and cross-functional collaboration, aligning MBSE with agile principles.
- **Toolset**: Automates traceability, validation, and diagram generation, integrating seamlessly with version control.
- **Language**: Inspired by SysML but optimized for Markdown and Mermaid, ensuring compatibility with modern AI tools.

### ReqFlow as a tool

ReqFlow is a requirements management tool designed to integrate directly into Git workflows. It uses a file-based approach where requirements are stored in markdown files within the project repository. This allows teams to manage requirements alongside their code, using familiar Git processes.

### ReqFlow as a language

ReqFlow is inspired by SysML and systems engineering practices but focuses on a subset of these methodologies. The goal is to simplify requirements management and make it easier for programmers, product managers, and tech companies—especially those working on software, cloud, and tech products—to adopt. By narrowing the scope, ReqFlow ensures a balance between powerful features and ease of use.

ReqFlow is opinionated, requiring semi-structured markdown files that follow specific conventions. These rules enable automation such as generating diagrams, creating traceability matrices, and linking requirements to tasks, issues, pull requests, and test cases.


For detailed information about the methodology, tools, and language, visit the [documentation](./doc/README.md).


## Getting Started

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
