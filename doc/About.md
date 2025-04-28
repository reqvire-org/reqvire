# Reqvire MBSE Methodology 

## Overview

Reqvire is an optimized **Model-Based Systems Engineering (MBSE) methodology** designed to integrate directly into **Git workflows**. It provides seamless integration with tools and services like **Git**, **GitHub**, and **GitLab**, bringing systems engineering practices into modern software development processes.

## Key Principles

### Git-Native Requirements Management

Reqvire employs a file-based approach where requirements and system design are stored as **Markdown files** within the project repository. This allows teams to manage system models alongside their code, using familiar processes:
- **Branches** for parallel development of features and requirements
- **Pull requests (PRs)** for reviewing model changes
- **Issues** for tracking work and defects
- **Reviews** for ensuring quality and consistency

These processes are supported by platforms like **GitHub**, **GitLab**, and **Bitbucket**, making Reqvire accessible to teams already using these tools.

### Traceability & Version Control

This tight integration ensures version control for all design and model changes, enabling full traceability. Every change to a requirement, architecture element, or design decision is tracked with the same rigor as code changes.

### Agile MBSE Approach

Reqvire supports an **Agile MBSE approach**, making it particularly well-suited for **software product development**, where iterative design, collaboration, and continuous integration are critical. It is intuitive and familiar to developers, engineers, and product managers, aligning with workflows they are already accustomed to.

### Structured Markdown

Reqvire is **opinionated**, requiring semi-structured Markdown files that follow specific conventions. This structure enables:
- **GitHub Actions** and similar CI/CD tools to automate tasks
- Generating diagrams directly from requirements
- Creating **traceability matrices** automatically
- Linking requirements to tasks, issues, pull requests, and test cases

The structured approach keeps the model simple and easy to use in the targeted environment while providing powerful capabilities.

### Simplified Modeling Language

The **Reqvire language** is a subset of **SysML**, inspired by systems engineering practices, with adjustments in visuals and syntax to integrate seamlessly into **Markdown Mermaid diagrams**. This enables lightweight yet powerful modeling.

By narrowing the scope to a focused subset of methodologies, Reqvire simplifies requirements management while retaining robust capabilities, making it particularly accessible to programmers, product managers, and tech teams—especially those working on **software, cloud, and tech products**.

### AI-Friendly Documentation

Being Markdown-oriented makes Reqvire highly **AI-friendly**. Large Language Models (LLMs) can easily parse and interpret the structured Markdown, facilitating integration with modern AI tooling for tasks such as:
- **Automated consistency checks**
- **Traceability analysis**
- **Intelligent assistance in system design**
- **Requirement validation and verification**


## Reqvire model

In **Reqvire**, a **model** represents the structured, interconnected set of artifacts that define the **system of interest**, including its requirements, architecture, behaviors, constraints, and relationships. The model is designed to be lightweight, Markdown-based, and highly compatible with Git workflows, making it intuitive for developers, engineers, and product managers.

---

### **What is a 'Model' in Reqvire?**

A **model** in Reqvire is a collection of structured Markdown files that capture:
1. **Requirements**: Stakeholder needs (user stories, user and mission requirements), system requirements, and constraints.
2. **System Structure**: Breakdown of components, modules, and their relationships.
3. **Behaviors**: Functional flows, state transitions, and operational processes.
4. **Traceability Links**: Relationships between requirements, design elements, and test cases.
5. **Diagrams**: Representations of architecture, behaviors, and dependencies (generated automatically via Mermaid diagrams).
6. **Use Cases**: Descriptions of user interactions with the system, specifying scenarios, inputs, and expected outcomes. Use cases help clarify functional requirements and provide context for design and validation.
8. **Verifications Definitions**: Detailed definitions of verifications linked to specific user and mission requirements and system behaviors. These verifications ensure the system functions as intended and provide traceability from design and requirements to validation activities.


---

### **How is the Model Stored in Reqvire?**

#### **1. File-Based Storage (Markdown)**
- The entire model is stored as a set of **Markdown files** in a Git repository.
- These files follow a specific, semi-structured format to represent system elements, such as:
  - Requirements: Organized in folders and subfolders.
  - System Architecture: Defined hierarchically with textual descriptions and references.
  - Diagrams: Specified using **Mermaid syntax** within the Markdown.  
  - Traceability: Embedded as links or tags within Markdown content.

Important to understand about Reqvire is that diagrams, treacability matrices and change impact reports are automatically generated by Reqvire tooling, taking structure from Markdown files and creating diagrams and connections while making items in diagrams links to the actual content in the Markdown files. This way, diagrams become a way to navigate the model. Also, diagrams can be committed directly into relevant documents in the CI/CD pipelines and as part of PRs, which makes automated diagrams artifacts and documentation itself but automated.

#### **2. Git Repository Integration**
- The model is **natively version-controlled** as part of a Git repository, enabling:
  - **Branching and Merging**: Changes to the model can be isolated in feature branches and integrated via pull requests.
  - **Commit History**: Every change to the model is tracked, providing a detailed record of its evolution.
  - **Collaboration**: Teams can collaborate on the model using Git workflows, such as code reviews, pull requests, and issue tracking.

#### **3. Automation Support**
- **CI/CD Integration**:
  - GitHub Actions, GitLab CI, or similar tools automate tasks like:
    - Generating **Mermaid diagrams** from Markdown content.
    - Building traceability matrices.
    - Validating Markdown structure and consistency.
  - Changes are validated as part of the Git workflow before merging into the main branch.
  - Diagrams and documentation can also be viewed directly in GitHub or similar systems/tools/browsers.
  
Reqvire tools, if integrated into CI/CD pipelines of Git-like workflows and processes, can trace diffs (changes in design, requirements, etc.) and trace through to connections to produce invalidation reports for test cases and summaries. These summaries can be used for downstream tasks and for easier understanding of what has changed in the system, which tests need to be re-done, and what changes need to be implemented downstream (architecture, specifications, software, functionality, etc.).

---

### **Key Features of Model Storage in Reqvire**
1. **Version Control**:
   - Every aspect of the model is stored in text format, making it inherently version-controlled.
   - Branching and merging allow parallel development of system designs.

2. **Traceability**:
   - Relationships between requirements, architecture, and other artifacts are embedded directly in Markdown files and managed using Git-based workflows.

3. **Human-Readable and Machine-Readable**:
   - Markdown files are simple enough for humans to edit yet structured enough for automation tools to parse and process.

4. **AI-Friendly**:
   - The use of Markdown and it's structure makes the model highly compatible with **AI tools**, allowing Large Language Models (LLMs) to interpret and assist with system design tasks, such as consistency checks or requirements analysis.

---

In summary, a **Reqvire model** is a structured set of Markdown files stored and managed in a Git repository, with support for traceability, automation, and collaboration through modern Git workflows and tools.


## Mapping to ISO/IEC/IEEE 15288

The **Reqvire methodology**, a model-based systems engineering (MBSE) approach, aligns closely with the processes defined in **ISO/IEC/IEEE 15288** but organizes and emphasizes them according to its structured model-driven workflow. Drawing inspiration and best practices from methodologies such as **Arcadia**, **MBSE Grid**, **OOSEM**, **OPM**, and others, Reqvire integrates key principles to provide a comprehensive framework for managing system requirements and architecture.


---

