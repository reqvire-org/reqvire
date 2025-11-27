# Elements

### Web Interface Style Specification

Styling conventions for HTML export web interface.

#### Details
**Page Layout:**
- Navigation bar at top with links to views
- Content area with responsive width
- Sidebar for element tree (optional)

**Typography:**
- System font stack for readability
- Monospace for code and identifiers
- Heading hierarchy matches markdown levels

**Color Palette:**
| Usage | Light Mode | Dark Mode |
|-------|------------|-----------|
| Background | #FFFFFF | #1a1a1a |
| Text | #333333 | #e0e0e0 |
| Links | #0066CC | #6699FF |
| Borders | #DDDDDD | #444444 |
| Code background | #F5F5F5 | #2d2d2d |

**Element Cards:**
| Element Type | Border Color | Background |
|--------------|--------------|------------|
| Requirement | #0066FF | #D0E0FF |
| User-requirement | #0066FF | #D0E0FF |
| Verification | #CC9900 | #FFF7B3 |
| Behavior | #9900CC | #E0D0FF |
| Specification | #009900 | #DFFFD0 |
| Constraint | #CC0000 | #FFD0D0 |

**Navigation:**
- Breadcrumb trail for element hierarchy
- Clickable relation links
- Collapsible sections for long content

#### Metadata
  * type: specification
---

### Web Interface Navigation Behavior

Navigation behavior in HTML export.

#### Details
**Inter-page Navigation:**
- Element links navigate to element page with anchor
- File links navigate to file page
- External links open in new tab

**Breadcrumbs:**
- Show path: Home > File > Section > Element
- Each segment clickable

**Search:**
- Client-side search in index page
- Filter by name, type, content

#### Metadata
  * type: behavior
---
