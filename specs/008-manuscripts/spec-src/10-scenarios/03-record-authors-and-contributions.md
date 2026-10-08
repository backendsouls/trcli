### User Story 3 - Record authors and contributions (Priority: P3)

A researcher records who the authors of a manuscript are, in order, who the corresponding
author is, and what each contributed — conceived the study, ran the experiments, wrote the
first draft. They add people to thank who are not authors. From this the tool can write the
author list, the affiliations, and the contribution statement that venues ask for.

**Why this priority**: Authorship is credit, and its order and contributions are asked for
by almost every venue. It builds on the staff register and on the manuscript record.

**Independent Test**: Give a manuscript three authors in order, mark one as corresponding,
record two contribution roles each, add an acknowledged person, and produce the
contribution statement.

**Acceptance Scenarios**:

1. **Given** a manuscript and people in the staff register, **When** the researcher sets
   its authors in order, **Then** the manuscript shows them in that order with their
   affiliations.
2. **Given** an author who is not in the staff register, **When** the researcher adds them
   by name and affiliation, **Then** they are accepted, and the tool offers to add them to
   the register.
3. **Given** authors, **When** the researcher moves one to another position, **Then** the
   order changes and the earlier order is kept in the history.
4. **Given** authors, **When** the researcher marks one or more as corresponding, or marks
   authors as having contributed equally, **Then** this is shown with the author list.
5. **Given** an author, **When** the researcher records their contributions from the
   standard roles (conceptualization, methodology, software, investigation, data curation,
   analysis, writing the original draft, review and editing, visualization, supervision,
   funding acquisition, project administration), **Then** they are stored.
6. **Given** recorded contributions, **When** the researcher asks for the contribution
   statement, **Then** a text listing each role and who performed it is produced.
7. **Given** a manuscript, **When** the researcher adds people or organizations to
   acknowledge, with what for, **Then** they are stored separately from the authors.
8. **Given** a manuscript supported by grants, **When** the researcher asks for the
   acknowledgements, **Then** the acknowledged people and the funders are both included.
9. **Given** the same person added twice as an author, **When** the researcher saves,
   **Then** the tool rejects it.
10. **Given** a manuscript with no corresponding author, **When** it is moved to
    "submitted", **Then** the tool warns.
11. **Given** an author removed from the staff register later, **When** the manuscript is
    viewed, **Then** the author's name and affiliation as recorded remain.
12. **Given** a person, **When** the researcher asks for what they authored, **Then** every
    manuscript they are an author of is listed with their position.

---
