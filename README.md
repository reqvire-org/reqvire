# Reqvire

**Reqvire** is a **framework for system modeling and project specifications**, designed to seamlessly integrate with modern agile development workflows and AI tools. While it currently focuses on **requirements, specifications, and traceability**, its vision extends far beyond—evolving into a fully **AI-powered Model-Based Systems Engineering (MBSE) framework**, where **AI assistance and automation** play a central role in designing, analyzing, and implementing system architectures.

It’s built for **product managers, architects, developers, and AI assistants** to collaborate effectively, keeping system models and requirements in sync with real-world implementation.  

Unlike traditional MBSE tools, Reqvire is **lightweight, Git-native, and practical** - inspired by **MBSE and SysML**, but without the complexity that slows teams down. 
By adopting **semi-structured conventions**, Reqvire bridges the gap between **formal modeling and agile documentation**, enabling **automation and AI-driven workflows** without the overhead of traditional modeling tools.  

---
<sub>Figure 1: Example Diagram (click image to browse requirements)</sub>
[![Example Diagram](doc/diagram_1.png)](specifications/SpecificationsRequirements.md#specifications-requirements)

---

## Why Reqvire?

Reqvire is a Git-native, AI-enabled **procces, toolset, and language** for modern requirements and system modeling. 

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

<sub>Figure 2: Example Change Impact Report (click image to read specifications)</sub>

[![Example Change Impact Report](doc/change_impact_report_pr.png)](specifications/SpecificationsRequirements.md#requirements-change-propagation)

---

## Get Started

For detailed information about **how to use Reqvire** visit the [documentation](./doc/README.md).


### Installation

To set up Reqvire, follow these steps:

#### Run install script (Linux and macOS)

Run the following command in your terminal:
```
curl -fsSL https://raw.githubusercontent.com/Reqvire/reqvire/main/scripts/install.sh | bash
```

#### From Source (Linux and macOS Apple Silicon)

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

2. **Clone the Reqvire Repository**
   ```bash
   git clone https://github.com/reqvire/reqvire.git
   cd reqvire
   ```

3. **Build Reqvire**
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
     - macOS: `reqvire-macos-arm64.tar.gz` 

2. **Extract the Binary**
   ```bash
   tar -xzf reqvire-<platform>.tar.gz
   ```

3. **Move to a Directory in Your PATH**
   ```bash
   sudo mv reqvire /usr/local/bin/
   ```

4. **Verify the Installation**
   ```bash
   reqvire --version
   ```
    
### Configuration

Create `reqflow.yaml` in the root of the git repo with following minimal content:
```
  # Path to the specifications folder
  specifications_folder: "specifications"
  
  # Default output folder for exported html specifications
  output_folder: "html"
  
  # Additional external folders that contain system requirements and other files
  # These can be absolute paths or paths relative to the input folder
  # All markdown files in these folders are considered requirements (except those matching exclusion patterns)
  external_folders:
    - tests
    - core
    - cli
      
  # Glob patterns to exclude from requirements processing
  # These are patterns that shouldn't be considered requirements even if they're in specifications or external folders
  excluded_filename_patterns:
    - "Usecases.md"
    - "**/Logical*.md"
    - "**/Physical*.md"
    - "**/TODO.md"  
```

Create `output` directory.


    
## Contributing

We welcome contributions to Reqvire! Whether it's improving the methodology, enhancing the tools, or refining the language, your input is valuable.

To maintain **consistency**, **traceability**, and **quality** in Reqvire, we follow a strict **contribution discipline** that ensures the system model, tests, and verifications evolve alongside the code.

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
     - Requirements should be added in the appropriate **User Requirements** or **System Requirements** folders, following the Reqvire methodology.
     - Requirments must be approved before code can be implemented or existing functionality changed
   - **Add or update end-to-end (E2E) tests**:
     - If your changes introduce **new functionality** or modify existing behavior that is not covered by tests, ensure to **add E2E tests** in the `tests/` directory.
     - Tests must validate the **expected behavior** of your feature or change.
   - **Update verifications**:
     - For every **new requirement** or **E2E test**, create or update the corresponding **verification cases** in the `specifications/verificationCases/` directory.
     - Verifications ensure that **requirements** are linked to **tests**, and all features are properly validated.

5. **Test Your Changes**:
   - Run **Reqvire validation** locally to ensure your requirements and verifications are consistent:
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

**Reqvire** is an open-source project created and maintained by [Ilija Ljubicic](https://github.com/ilijaljubicic). 

### Special Thanks:

- [Juanjo Andres](https://github.com/juanjoandres)  
  For valuable contributions to testing and in in shaping Reqvire's direction, especially in the early phases of Reqvire's development.

- [GrapheneDB](https://www.graphenedb.com/)  
  For **partial sponsorship** and for being the **first user** of Reqvire. Their support helped shape the tool's early direction.

## License

Licensed under the [Apache2 License](LICENSE).

