# User Stories

Each user story represents a high-level user need and is linked to specific measures of effectiveness (MOEs). User requirements refine user stories into actionable, detailed system requirements.

User stories are a key part of the user needs definition process and follow the definition of use cases. They serve as an entry point for defining user requirements, which refine user stories and group them into structures that facilitate the development of logical architecture diagrams.

A **user story** must have at least one `tracedFrom` relation linking it to a specific MOE. This ensures alignment with the project's objectives and stakeholder expectations.

User stories are expected to be documented in the `specifications/UserStories.md` file.

---

## User Stories Document Format and User Story Format

The user story document organizes stories into logical groups under grouping titles. Each user story includes a descriptive title, a structured text description, and relations to MOEs.

### Expected Document Format:

```markdown
# Document Title

## Personas, actions and values

Here one may explain which personas, actions and values are going to be used in the user stories

### <User Story Title>

As [persona], I want to [action], so that I can [value].

Relations:
 * tracedFrom: MOEs.md/MOE_CR

