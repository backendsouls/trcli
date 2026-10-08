### User Story 6 - Keep and share a handbook (Priority: P6)

Everything written down under the earlier stories is the lab's handbook. A lab head hands
it to a new member as one file; the member brings it into their own workspace, where each
entry arrives marked as coming from the lab and is read before it is relied on. When the
lab revises its handbook, members bring in the new one and see what changed. A researcher
also produces, for a manuscript, a statement of the methods, assumptions, and departures
behind it.

**Why this priority**: The value of making things explicit multiplies when they are passed
on. This comes last because there must be a handbook before it can be shared.

**Independent Test**: Export the lab-origin conventions and methodologies as a handbook,
bring it into another workspace, confirm they arrive with their origin and unread, change
one in the first workspace, export again, bring it in, and see the difference reported.

**Acceptance Scenarios**:

1. **Given** conventions, methodologies, and checklists, **When** the researcher exports a
   handbook — all of them, or those of an origin, a category, or a project — **Then** one
   file is produced that contains them completely.
2. **Given** a handbook file, **When** the researcher brings it in, **Then** the tool shows
   what it contains, and adds each entry with its origin, marked as received and unread.
3. **Given** a handbook brought in again in a newer version, **When** it is brought in,
   **Then** the tool shows what was added, changed, and retired, and applies the changes
   only on acceptance.
4. **Given** an entry received from a handbook, **When** the researcher changes it locally,
   **Then** it is marked as locally changed, and a later handbook does not overwrite it
   without the researcher's choice.
5. **Given** an entry received from a handbook, **When** the researcher disagrees with it for
   their own work, **Then** they record a departure, and the entry itself is left as
   received.
6. **Given** a handbook, **When** the researcher exports it as a document, **Then** a
   readable handbook is produced, organized by category, with each entry's origin,
   strength, rationale, and examples.
7. **Given** a manuscript, **When** the researcher asks for its methods statement, **Then** a
   text is produced listing the methodologies used with their versions and sources, how
   they were adapted, the assumptions made, and the departures from conventions and
   methods, ready to be edited into the manuscript.
8. **Given** a new member, **When** the lab head asks which entries a member-facing handbook
   should start with, **Then** the binding conventions and the methodologies in current use
   are listed first.
9. **Given** entries received from several handbooks — the lab's and a community's — **When**
   two say different things about the same subject, **Then** both are kept, marked as
   conflicting, with which takes precedence.
10. **Given** a file that is damaged or is not a handbook, **When** it is brought in,
    **Then** the tool refuses and adds nothing.
11. **Given** a handbook, **When** it is exported, **Then** private notes, people's contact
    details, and anything marked private are left out.

---
