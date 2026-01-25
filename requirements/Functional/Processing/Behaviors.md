# Elements

### Change Propagation Behavior

How changes propagate through model based on relation types.

#### Details
**Propagation by Relation:**
| Relation | Propagates |
|----------|------------|
| derive/derivedFrom | Yes - parent to child |
| satisfiedBy/satisfy | Yes - requirement to implementation |
| verifiedBy/verify | Yes - requirement to verification |
| trace | No - documentation only |
| Attachments | Yes - if content hash changes |

**Attachment Impact:**
- Content changes → propagate impact
- Path renames → track but no impact

#### Metadata
  * type: behavior

#### Relations
  * satisfy: [CLI Change Impact Report Command](../../Interfaces/CLI/Commands.md#cli-change-impact-report-command)
---
