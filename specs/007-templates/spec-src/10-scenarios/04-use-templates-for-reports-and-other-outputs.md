### User Story 4 - Use templates for reports and other outputs (Priority: P4)

The documents the tool itself produces — activity reports, experiment reports, exported
literature reviews, response letters, reading notes, notebook exports — each come out in a
layout. The researcher chooses which template lays each of them out, sets a preferred
template per kind of output, and replaces a provided layout with their own.

**Why this priority**: These documents exist without templates; templates make them fit the
reader. It depends on the earlier stories and on the specifications that produce the
documents.

**Independent Test**: Produce the weekly activity report with the provided supervisor
template and with a custom one, confirm the same content appears in two layouts, then set
the custom one as preferred and confirm it is used without being named.

**Acceptance Scenarios**:

1. **Given** an output the tool produces as a document, **When** the researcher names a
   template for it, **Then** the document is laid out by that template with the same
   content it would otherwise have.
2. **Given** a kind of output, **When** the researcher sets a preferred template for it,
   **Then** that template is used whenever none is named.
3. **Given** no preferred template, **When** an output is produced, **Then** the provided
   template for that kind is used, and the result is the same as before this specification.
4. **Given** a template of the wrong kind for an output (a paper template for an activity
   report), **When** the researcher names it, **Then** the tool refuses and lists the
   templates that fit.
5. **Given** an activity report template, **When** it leaves out a section the report has
   content for, **Then** that content is left out of the document, and the tool says which
   sections were not placed.
6. **Given** a template with headings for the researcher's own words (highlights, blockers,
   help needed), **When** a report is produced with it, **Then** those headings appear for
   the researcher to fill, or filled with the narrative already given.
7. **Given** a named report definition, **When** the researcher sets its template, **Then**
   every report produced from that definition uses it.
8. **Given** a saved report, **When** it is viewed later, **Then** it keeps the layout it
   was saved with, whatever has happened to the template since.
9. **Given** an output in a structured form meant for other programs, **When** a template is
   named, **Then** the tool says templates apply only to documents and ignores it.

---
