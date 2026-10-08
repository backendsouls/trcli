### User Story 4 - Add my own words, save, and share (Priority: P4)

A researcher adds what the records cannot say — the highlights of the period, what blocked
them, what they plan next — then saves the report as it stands, so there is a permanent
copy of what was reported and when. They export it as a document to send, and can find
past reports later.

**Why this priority**: A report that is only numbers is not yet a report to a person, and a
report that changes every time it is opened is not a record of what was said. This story
turns the output into something that can be sent and kept.

**Independent Test**: Produce a weekly report, add highlights and next steps, save it,
change something in the workspace, and confirm the saved report is unchanged; export it as
a document and list saved reports.

**Acceptance Scenarios**:

1. **Given** a report, **When** the researcher adds highlights, blockers, next steps, or a
   free comment, **Then** each appears in its own section of the report.
2. **Given** a report, **When** the researcher saves it, **Then** its entire content is kept
   exactly as it was at that moment, with the date it was saved and who saved it.
3. **Given** a saved report, **When** records it mentions are later changed or deleted,
   **Then** the saved report is unchanged.
4. **Given** saved reports, **When** the researcher lists them, **Then** they are shown with
   their period, scope, audience, and date saved, and can be filtered by any of these.
5. **Given** a saved report, **When** the researcher views it, **Then** it is shown exactly
   as saved and marked as a saved copy.
6. **Given** a report, saved or not, **When** the researcher exports it, **Then** a document
   is produced that can be sent or printed, and the same content can be produced in a form
   meant for other programs.
7. **Given** a saved report, **When** the researcher changes its narrative, **Then** a new
   revision is saved and the earlier one is kept.
8. **Given** a saved report for a period, **When** the researcher produces a fresh report
   for the same period and scope, **Then** the tool mentions that a saved one exists and
   can show what differs.
9. **Given** a period that has not ended, **When** the researcher saves its report, **Then**
   the saved copy is marked as partial.
10. **Given** a saved report, **When** the researcher deletes it, **Then** the tool asks for
    confirmation.
11. **Given** an export destination that already exists or cannot be written, **When** the
    researcher exports, **Then** the tool refuses to overwrite without confirmation, or
    explains why it cannot write.

---
