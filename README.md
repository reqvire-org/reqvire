# ReqFlow

ReqFlow is a requirements management tool designed to integrate directly into Git workflows. It uses a file-based approach where requirements are stored in markdown files within the project repository. This allows teams to manage requirements alongside their code, using familiar Git processes.

ReqFlow is inspired by SysML and systems engineering practices but focuses on a subset of these methodologies. The goal is to simplify requirements management and make it easier for programmers, product managers, and tech companies—especially those working on software, cloud, and tech products—to adopt. By narrowing the scope, ReqFlow ensures a balance between powerful features and ease of use.

ReqFlow is opinionated, requiring semi-structured markdown files that follow specific conventions. These rules enable GitHub Actions to automate tasks such as generating diagrams, creating traceability matrices, and linking requirements to tasks, issues, pull requests, and test cases.

#### How It Works:
1. **Markdown-Based Requirements**:  
   Requirements are written in markdown and organized in a predefined folder structure. These files follow a semi-structured format with specific headers and syntax to allow automated processing.

2. **Git Flow Integration**:  
   - Changes to requirements are made through **pull requests**, allowing teams to review and discuss updates.  
   - **Diffs** clearly show what has changed, making it easier to understand the impact of updates.

3. **GitHub Actions for Automation**:  
   - Generate **Mermaid diagrams** for visualizing requirements and their relationships.  
   - Build and update **traceability matrices** linking requirements to tasks, issues, PRs, and tests.  
   - Highlight downstream elements such as test cases that might need attention based on requirement changes.

4. **Traceability**:  
   - Links requirements to related issues, tasks, PRs, and test cases.  
   - Enables teams to track the impact of requirement changes on code and tests.  
   - Makes it easier for developers and operators to identify what needs updates, but does not fully automate or enforce these changes.

5. **Opinionated Structure**:  
   - ReqFlow relies on specific conventions for markdown files to enable automation.  
   - Teams must adhere to rules for headers, tags, and relationships to take full advantage of automated features.  
   - While flexible in content, the structure ensures consistency and traceability.

#### Inspired by Systems Engineering, Built for Tech Teams:
ReqFlow borrows from SysML and systems engineering practices to provide structure and traceability. However, it deliberately focuses on a simplified subset of these methodologies to make them accessible and practical for programmers, product managers, and tech companies. 

#### Key Features:
- **Markdown-Based**: Simple and human-readable format for managing requirements.  
- **Git-Integrated**: Manage changes using pull requests and track updates with Git diffs.  
- **Automation with GitHub Actions**: Generate diagrams, traceability matrices, and automate links between requirements, code, and tests.  
- **Traceability**: Easily link and track requirements across the development lifecycle.  
- **Collaborative Workflow**: Use Git’s standard tools for team collaboration on requirements.

#### Use Cases:
- Teams managing requirements alongside code in Git repositories.  
- Projects needing traceability between requirements, tasks, and tests.  
- Development workflows that benefit from markdown-based documentation and automation.  

