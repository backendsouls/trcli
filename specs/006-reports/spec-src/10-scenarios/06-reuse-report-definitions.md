### User Story 6 - Reuse report definitions (Priority: P6)

A researcher who sends the same report every week saves its definition once — weekly,
this project, supervisor audience, these sections, this export form — under a name, and
from then on produces it with that name alone. They can also set which definition is used
when a report is requested with no options at all.

**Why this priority**: It removes repetition for people who already report regularly. It
depends on every earlier story and adds only convenience.

**Independent Test**: Define a report named "supervisor-weekly", produce it by name on two
different weeks, and confirm each covers its own week with the defined scope, audience, and
sections.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher defines a named report with a period
   length, a scope, an audience, sections, a level of detail, and an export form, **Then**
   the definition is stored.
2. **Given** a named report, **When** the researcher produces it, **Then** it covers the
   current period of its length, with everything else as defined.
3. **Given** a named report, **When** the researcher produces it with an option given
   explicitly, **Then** that option overrides the definition for this one time.
4. **Given** named reports, **When** the researcher lists them, **Then** each is shown with
   its definition.
5. **Given** a named report, **When** the researcher sets it as the default, **Then** a
   report requested with no options uses it.
6. **Given** a named report whose project no longer exists, **When** it is produced,
   **Then** the tool says so and produces nothing.
7. **Given** a name already in use, or a definition with an invalid value, **When** the
   researcher saves it, **Then** the tool rejects it and explains.
8. **Given** a named report with narrative prompts defined (for example "Blockers" and
   "Help needed"), **When** it is produced, **Then** those prompts appear as empty sections
   for the researcher to fill.

---
