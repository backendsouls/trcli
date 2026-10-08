### User Story 2 - Describe my programme and its curriculum (Priority: P2)

A researcher records the programme they are enrolled in — institution, degree, the date
they started, the time allowed — and its curriculum: every component with its code,
credits, category (mandatory, elective, optional), the term it is suggested for, the area
it belongs to, and what must be completed before it. They can type it in, or bring it in
from a table. They see it as a grid, term by term: the curricular matrix.

**Why this priority**: The curriculum is what progress is measured against. Without it the
courses of the first story are a list; with it they are a position on a map.

**Independent Test**: Record a programme, bring in a curriculum of twelve components from a
table, add a prerequisite between two of them, view the grid by suggested term, and confirm
a prerequisite loop is rejected.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records a programme with a name, an
   institution, a degree level, and the date of enrolment, **Then** it is stored, and can be
   linked to the project it belongs to.
2. **Given** a programme, **When** the researcher records the time normally expected and
   the maximum time allowed, **Then** the expected and latest dates of completion are shown.
3. **Given** a programme, **When** the researcher creates its curriculum with a name and the
   year it took effect, **Then** it is stored as the curriculum the researcher follows.
4. **Given** a curriculum, **When** the researcher adds a component with a code, a title,
   credits, hours, a category, a suggested term, and an area, **Then** it is stored.
5. **Given** components, **When** the researcher states that one requires another to be
   completed first, or to be taken at the same time, **Then** the requirement is stored and
   shown on both.
6. **Given** a curriculum, **When** the researcher records its rules — total credits
   required, and the minimum credits in each category or area — **Then** they are stored
   with the curriculum.
7. **Given** a curriculum, **When** the researcher views the matrix, **Then** components are
   shown in columns by suggested term, each with its code, credits, and category, and its
   prerequisites marked.
8. **Given** a table of components in a file, **When** the researcher brings it in,
   **Then** each valid row becomes a component, each invalid row is reported with its
   reason, and nothing is stored when the researcher only asked for a preview.
9. **Given** components whose prerequisites form a loop, or a prerequisite on a code that
   does not exist, **When** the researcher saves, **Then** the tool rejects it and names the
   components.
10. **Given** a programme that changes its curriculum, **When** the researcher records a new
    curriculum and moves to it, **Then** the earlier one is kept, and the tool shows which
    completed components carry over and which no longer count.
11. **Given** a group of electives from which a number of credits must be chosen, **When**
    the researcher defines the group, **Then** the matrix shows it as one requirement with
    its options.
12. **Given** a component code used twice in a curriculum, negative credits, or a suggested
    term below one, **When** the researcher saves, **Then** the tool rejects it.
13. **Given** a curriculum, **When** the researcher exports it, **Then** a table is produced
    that can be brought into another workspace.

---
