### User Story 3 - Cite and assemble bibliographies (Priority: P3)

A researcher gives each reference a citation key to use in their writing, sees how a
reference looks in a chosen citation style, and assembles named bibliographies — "Chapter
2", "Related work for the journal paper" — that they export as a file for their writing
tool or as a formatted reference list.

**Why this priority**: Citing is the purpose of collecting. Keys and bibliographies are what
connect the library to the papers the researcher writes.

**Independent Test**: Give three references citation keys, show one in two styles, create a
bibliography with the three in a chosen order, and export it both as a file for a writing
tool and as a formatted list.

**Acceptance Scenarios**:

1. **Given** a reference, **When** the researcher saves a citation for it, **Then** the
   citation receives a key that is unique in the workspace, proposed from the author, year,
   and title unless the researcher gives one.
2. **Given** two references that would receive the same key, **When** the second is saved,
   **Then** the tool makes the key unique and tells the researcher.
3. **Given** a citation key already used in the researcher's writing, **When** the
   reference's details change, **Then** the key does not change unless the researcher asks.
4. **Given** a reference, **When** the researcher asks to see it in a citation style,
   **Then** it is shown as that style formats a reference-list entry and an in-text
   citation.
5. **Given** a workspace, **When** the researcher creates a bibliography with a name and a
   purpose and adds references to it, **Then** the bibliography lists them and each
   reference lists the bibliographies it is in.
6. **Given** a bibliography, **When** the researcher sets its citation style and its
   ordering (by author, by year, by order of addition, or by hand), **Then** it is shown
   and exported that way.
7. **Given** a bibliography, **When** the researcher exports it, **Then** they can obtain a
   file for a writing tool or a formatted reference list in the bibliography's style.
8. **Given** a draft paper, **When** the researcher links a bibliography to it, **Then** the
   draft's citations are those of the bibliography.
9. **Given** a reference missing a detail its citation style requires, **When** it is
   formatted, **Then** the tool formats what it can and names the missing detail.
10. **Given** a bibliography, **When** the researcher asks for a check, **Then** the tool
    lists references with missing required details, without a citation key, or likely to
    be duplicates of one another.
11. **Given** a citation style the tool does not know, **When** the researcher supplies a
    style definition file, **Then** the style can be used like a built-in one.

---
