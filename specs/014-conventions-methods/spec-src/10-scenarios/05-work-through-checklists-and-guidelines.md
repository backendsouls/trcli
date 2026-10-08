### User Story 5 - Work through checklists and guidelines (Priority: P5)

A researcher keeps the checklists their work must satisfy — a reporting guideline their
field expects for a kind of study, a venue's submission requirements, the lab's own list of
things to verify before sending anything out. They apply a checklist to a manuscript or an
experiment, go through it item by item, saying for each whether it is satisfied and where,
and see what is still missing.

**Why this priority**: Checklists are how a community's expectations are made explicit.
They are the most structured of the implicit things and the last to be needed — at
reporting time.

**Independent Test**: Create a checklist of eight items, two of them optional, apply it to
a manuscript, mark five satisfied with where each is addressed and one not applicable with
a reason, and view the report of what remains.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a checklist with a name, an
   origin, and what kind of work it is for, **Then** it is stored.
2. **Given** a checklist, **When** the researcher adds items in order, each with a
   statement, whether it is required, and optional guidance, grouped under headings,
   **Then** the checklist shows them.
3. **Given** a checklist in a table in a file, **When** the researcher brings it in,
   **Then** each row becomes an item, and invalid rows are reported.
4. **Given** a checklist and a manuscript or an experiment, **When** the researcher applies
   the checklist to it, **Then** every item is listed for that work as unanswered.
5. **Given** an applied checklist, **When** the researcher marks an item satisfied and says
   where it is addressed — a part of the manuscript, a record, a page, or a note — **Then**
   the answer is stored.
6. **Given** an item that does not apply to this work, **When** the researcher marks it not
   applicable with a reason, **Then** it is counted as answered.
7. **Given** an applied checklist, **When** the researcher asks for its state, **Then** the
   number of items satisfied, not applicable, and unanswered is shown, required ones
   distinguished, with the unanswered items listed.
8. **Given** an applied checklist with every required item answered, **When** the researcher
   asks whether it is complete, **Then** the answer is yes in a way another program can act
   on.
9. **Given** an applied checklist, **When** the researcher exports it, **Then** a document
   is produced listing each item with its answer and where it is addressed, in the form
   venues ask for.
10. **Given** a checklist that changes after it was applied, **When** the researcher looks at
    the application, **Then** it keeps the items it was applied with and says a newer
    version exists, with what differs.
11. **Given** a manuscript, **When** its readiness is checked, **Then** required checklist
    items still unanswered are among the findings.
12. **Given** an item marked satisfied without saying where, an item without a statement, or
    a checklist applied twice to the same work, **When** the researcher saves, **Then** the
    tool rejects it, or for the last asks whether a second application is intended.

---
