# Setup Reqvire Environment

Setup steps for the Reqvire plugin and environment.

## Steps

### Step 1: Update Plugin

First, update the Reqvire plugin itself to ensure you have the latest skills and capabilities.

**Detect OS and plugin path:**
- **Linux/macOS**: `$HOME/.claude/plugins/marketplaces/reqvire-org-marketplace`
- **Windows**: `$USERPROFILE\.claude\plugins\marketplaces\reqvire-org-marketplace`

**Update plugin from GitHub:**

#### Linux/macOS
```bash
PLUGIN_DIR="$HOME/.claude/plugins/marketplaces/reqvire-org-marketplace"
cd "$PLUGIN_DIR" && git fetch origin && git reset --hard origin/main
```

#### Windows (PowerShell)
```powershell
$PLUGIN_DIR = "$env:USERPROFILE\.claude\plugins\marketplaces\reqvire-org-marketplace"
Set-Location $PLUGIN_DIR
git fetch origin
git reset --hard origin/main
```

After updating, show the user what version they now have:
```bash
cd "$PLUGIN_DIR" && git log --oneline -1
```

**Note**: User may need to restart Claude Code after plugin update for changes to take effect.

### Step 2: Verify Reqvire via npx

Reqvire skills in this plugin use the npm package by default, so users do not need a separate binary install. The default package is `@reqvire-org/reqvire@latest`.

Check that Node/npm are available, then verify the Reqvire runner:

```bash
node --version
npm --version
npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --version
```

If the user wants reproducible/pinned command behavior, have them set `REQVIRE_NPX_PACKAGE` before running Claude Code:

```bash
export REQVIRE_NPX_PACKAGE=@reqvire-org/reqvire@0.13.2
```

On Windows PowerShell:

```powershell
$env:REQVIRE_NPX_PACKAGE = "@reqvire-org/reqvire@0.13.2"
```

If Node/npm are unavailable, tell the user to install Node.js 22 or newer, then rerun setup.

### Step 3: Update CLAUDE.md (Ask Permission First)

**IMPORTANT**: Before modifying CLAUDE.md, you MUST ask the user for permission.

Ask: "Would you like me to add Reqvire instructions to your repository's CLAUDE.md file? This will guide Claude to use the syseng skill for all requirements and model work."

If the user approves, add the following content to the repository's CLAUDE.md file:

#### Content to add at the TOP of CLAUDE.md (after any existing header):

```markdown
## CRITICAL: Requirements & Specifications & System Model Work

**FOR ANY CONVERSATION, WORK, REQUEST, OR TASK RELATED TO:**
- Capabilities, requirements, semantic contracts, and system model elements
- Specifications
- System model (MBSE model)
- Verifications
- Architecture decisions
- Model refactoring or analysis

**YOU MUST:**
1. **Use the `reqvire:syseng` skill** for all requirements and model work
2. **Use the `reqvire:audit` skill** for model analysis, coverage, and lint checks
3. **NEVER manually edit** requirements files without using reqvire commands unless reqvire tool is not able to cover the need.
4. **ALWAYS validate** the model after changes with `npx -y "${REQVIRE_NPX_PACKAGE:-@reqvire-org/reqvire@latest}" --workspace "$PWD" validate`
5. **Pin Reqvire when needed** by setting `REQVIRE_NPX_PACKAGE`, for example `export REQVIRE_NPX_PACKAGE=@reqvire-org/reqvire@0.13.2`
```

#### Content to add at the BOTTOM of CLAUDE.md:

```markdown
---

**Remember**: For ANY work involving requirements, specifications, or the system model, **ALWAYS use the `reqvire:syseng` and `reqvire:audit` skills**. Manual editing of requirements files will break model integrity!
```

#### Implementation Notes:
- If CLAUDE.md doesn't exist, create it with these sections
- If CLAUDE.md exists, read it first, then add the top section after the first heading (or at the top if no heading), and append the bottom section at the end
- Do not duplicate content if it already exists
