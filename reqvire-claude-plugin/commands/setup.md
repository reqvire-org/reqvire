---
allowed-tools: Read, Bash, Write, Edit, AskUserQuestion
argument-hint: [install]
description: Setup reqvire environment
model: default
---

## Setup Steps

1. Install reqvire binary
2. Ask user permission to update CLAUDE.md with reqvire instructions

### Step 1: Installing reqvire

Detect the operating system and install reqvire accordingly:

1. First check the platform (look at the `<env>` context for "Platform:" info)
2. Run the appropriate installation commands for that platform

#### Linux x86_64
```bash
mkdir -p ~/.local/bin
curl -fsSL -o /tmp/reqvire.tar.gz https://github.com/Reqvire/reqvire/releases/download/v0.9.0/reqvire-linux-x86_64.tar.gz
tar -xzf /tmp/reqvire.tar.gz -C ~/.local/bin
mv ~/.local/bin/reqvire-linux-x86_64 ~/.local/bin/reqvire
chmod +x ~/.local/bin/reqvire
```

#### Mac Silicon (ARM64)
```bash
mkdir -p ~/.local/bin
curl -fsSL -o /tmp/reqvire.tar.gz https://github.com/Reqvire/reqvire/releases/download/v0.9.0/reqvire-darwin-aarch64.tar.gz
tar -xzf /tmp/reqvire.tar.gz -C ~/.local/bin
mv ~/.local/bin/reqvire-darwin-aarch64 ~/.local/bin/reqvire
chmod +x ~/.local/bin/reqvire
```

#### Mac Intel (x86_64)
```bash
mkdir -p ~/.local/bin
curl -fsSL -o /tmp/reqvire.tar.gz https://github.com/Reqvire/reqvire/releases/download/v0.9.0/reqvire-darwin-x86_64.tar.gz
tar -xzf /tmp/reqvire.tar.gz -C ~/.local/bin
mv ~/.local/bin/reqvire-darwin-x86_64 ~/.local/bin/reqvire
chmod +x ~/.local/bin/reqvire
```

#### Windows (PowerShell)
```powershell
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.local\bin"
Invoke-WebRequest -Uri "https://github.com/Reqvire/reqvire/releases/download/v0.9.0/reqvire-windows-x86_64.zip" -OutFile "$env:TEMP\reqvire.zip"
Expand-Archive -Path "$env:TEMP\reqvire.zip" -DestinationPath "$env:USERPROFILE\.local\bin" -Force
Rename-Item "$env:USERPROFILE\.local\bin\reqvire-windows-x86_64.exe" "reqvire.exe"
```

### Step 2: Update CLAUDE.md (Ask Permission First)

**IMPORTANT**: Before modifying CLAUDE.md, you MUST ask the user for permission using AskUserQuestion.

Ask: "Would you like me to add Reqvire instructions to your repository's CLAUDE.md file? This will guide Claude to use the syseng skill and /reqvire:* commands for all requirements and model work."

If the user approves, add the following content to the repository's CLAUDE.md file:

#### Content to add at the TOP of CLAUDE.md (after any existing header):

```markdown
## CRITICAL: Requirements & Specifications & System Model Work

**FOR ANY CONVERSATION, WORK, REQUEST, OR TASK RELATED TO:**
- Requirements (user requirements, system requirements)
- Specifications
- System model (MBSE model)
- Verifications
- Architecture decisions
- Model refactoring or analysis

**YOU MUST:**
1. **Use the `syseng` skill** for all requirements and model work
2. **Use `/reqvire:*` commands** from the reqvire plugin for model operations
3. **NEVER manually edit** requirements files without using reqvire commands unless reqvire tool is not able to cover the need.
4. **ALWAYS validate** the model after changes with `reqvire validate`
```

#### Content to add at the BOTTOM of CLAUDE.md:

```markdown
---

**Remember**: For ANY work involving requirements, specifications, or the system model, **ALWAYS use the `syseng` skill and `/reqvire:*` commands**. Manual editing of requirements files will break model integrity!
```

#### Implementation Notes:
- If CLAUDE.md doesn't exist, create it with these sections
- If CLAUDE.md exists, read it first, then add the top section after the first heading (or at the top if no heading), and append the bottom section at the end
- Do not duplicate content if it already exists

