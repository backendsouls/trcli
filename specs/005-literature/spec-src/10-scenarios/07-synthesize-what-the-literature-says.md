### User Story 7 - Synthesize what the literature says (Priority: P7)

For the references a review includes, a researcher defines what to extract from each —
"sample size", "method", "main finding", "limitations" — fills in those items reference by
reference, and views the outcome as a matrix: references down the side, extracted items
across the top. They also group references by theme and see which themes are well covered
and which are gaps.

**Why this priority**: Synthesis is the result of a review. It is the last step and needs
included references to exist.

**Independent Test**: In a review with three included references, define three items to
extract, fill them in, view and export the matrix, tag the references with two themes, and
see the count per theme.

**Acceptance Scenarios**:

1. **Given** a review, **When** the researcher defines the items to extract, each with a
   name, a kind of value (text, number, yes/no, one of a list), and whether it is required,
   **Then** the items are stored with the review.
2. **Given** an included reference, **When** the researcher records a value for an item,
   optionally with the page it was found on, **Then** the value is stored for that
   reference and item.
3. **Given** a review, **When** the researcher asks what remains to extract, **Then**
   included references with missing required items are listed.
4. **Given** extracted values, **When** the researcher views the matrix, **Then** included
   references are rows, items are columns, and empty cells are visibly empty.
5. **Given** the matrix, **When** the researcher exports it, **Then** a table is produced
   that a spreadsheet or a writing tool can open, with each reference's citation.
6. **Given** a review, **When** the researcher defines themes and assigns included
   references to them, **Then** each theme lists its references and a reference may belong
   to several themes.
7. **Given** themes, **When** the researcher asks for coverage, **Then** each theme shows
   its number of references, and themes with none are shown as gaps.
8. **Given** a value of the wrong kind for its item, **When** the researcher saves it,
   **Then** the tool rejects it and states what is expected.
9. **Given** an item that already has values, **When** the researcher changes its kind or
   removes it, **Then** the tool shows how many values are affected and requires
   confirmation.
10. **Given** an annotation or a structured summary on a reference, **When** the researcher
    extracts an item for it, **Then** they can copy the value from the annotation or
    summary, and the link to its source is kept.

---
