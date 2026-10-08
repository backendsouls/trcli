### User Story 6 - Check a document against its template (Priority: P6)

Before sending a paper, a researcher asks whether it still has the shape its template
requires: every required section present and in order, no section left with only its
guidance, no placeholder unresolved, and the limits the template states — words in the
abstract, words in the whole text, number of references — respected.

**Why this priority**: Venues and programmes reject documents for form before reading them.
The check is a safety net at the end of writing and needs everything before it.

**Independent Test**: Take a document created from a template, remove a required section,
leave one section with only guidance, exceed the abstract's word limit, and confirm the
check reports exactly those three problems.

**Acceptance Scenarios**:

1. **Given** a document created from a template, **When** the researcher checks it,
   **Then** the tool reports whether every required section is present and whether the
   sections are in the template's order.
2. **Given** a section that still contains only the template's guidance, **When** the
   document is checked, **Then** the section is reported as not yet written.
3. **Given** a template that states limits (words in a section, words overall, number of
   references, number of figures), **When** the document is checked, **Then** each limit is
   shown with the document's count, and those exceeded are reported.
4. **Given** unresolved placeholders or remaining guidance, **When** the document is
   checked, **Then** each is listed with where it is.
5. **Given** a document that passes, **When** it is checked, **Then** the tool says so and
   the outcome can be used by another program to allow a next step.
6. **Given** a template requiring that the document not reveal its authors, **When** the
   document is checked, **Then** places where an author's name or affiliation from the
   workspace appears are reported.
7. **Given** a document with sections the template does not have, **When** it is checked,
   **Then** they are listed as additional, and are not treated as errors.
8. **Given** a document and a different template of the same kind, **When** the researcher
   checks the document against that template, **Then** the differences in structure are
   reported, to help move a paper from one venue to another.
9. **Given** a file the tool cannot read as text, **When** it is checked, **Then** the tool
   says so and reports nothing else.

---
