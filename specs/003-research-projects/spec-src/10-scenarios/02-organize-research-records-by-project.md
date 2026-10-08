### User Story 2 - Organize research records by project (Priority: P2)

A researcher places papers, reviews, drafts, experiments, datasets, research questions, and
every other record into projects. Working inside a project, they see only that project's
records. A record that serves more than one project belongs to each of them without being
copied.

**Why this priority**: A project that contains nothing is a label. Scoping records to
projects is what lets a researcher with a doctorate and a side project keep them apart and
still share a library between them.

**Independent Test**: With two projects, add a paper while working in the first, add the
same paper to the second, list papers in each project and across the workspace, and confirm
the paper appears once in each list and exists only once.

**Acceptance Scenarios**:

1. **Given** a current project, **When** the researcher creates any record, **Then** the
   record belongs to that project.
2. **Given** a record in one project, **When** the researcher adds it to another project,
   **Then** it belongs to both, exists only once, and a change made in either is seen in
   both.
3. **Given** a current project, **When** the researcher lists or searches records, **Then**
   only that project's records are shown unless the whole workspace is asked for.
4. **Given** a record in two projects, **When** the researcher removes it from one, **Then**
   it remains in the other and is not deleted.
5. **Given** a record in only one project, **When** the researcher removes it from that
   project, **Then** the tool says it would belong to no project and asks whether to keep it
   unassigned or cancel.
6. **Given** a record, **When** the researcher views it, **Then** the projects it belongs to
   are shown.
7. **Given** records that belong to no project, **When** the researcher asks for unassigned
   records, **Then** they are listed by type.
8. **Given** no current project and more than one project in the workspace, **When** the
   researcher creates a record without naming a project, **Then** the tool asks which
   project it belongs to instead of guessing.
9. **Given** a project, **When** the researcher asks for its summary, **Then** the number of
   records of each type in it is shown.

---
