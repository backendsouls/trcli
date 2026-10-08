### User Story 1 - Create and manage typed research projects (Priority: P1)

A researcher creates a project, chooses its type, and records what the type calls for: for a
degree project, the institution, the programme, the supervisor and co-supervisors, and the
start and expected end dates; for independent research, only a title and a goal. They follow
the project's status through its life and choose which project they are currently working
in, so that later commands apply to it without naming it each time.

**Why this priority**: The project is the container everything else in this specification
hangs from. With only this story a researcher can already list what they are working on,
of which kind, with whom, and until when.

**Independent Test**: Create a doctoral project and an independent research project, fill
in their details, set one as current, change the status of the other, and list projects
filtered by type and status.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a project with a title and a type,
   **Then** it is stored with a unique short identifier and the status "planned".
2. **Given** a degree-type project (undergraduate research, capstone, master's, doctoral),
   **When** the researcher records its institution, programme, supervisor, co-supervisors,
   start date, and expected end date, **Then** the details are saved and shown with the
   project.
3. **Given** an independent research project, **When** the researcher creates it with only a
   title and a goal, **Then** it is accepted and no academic details are asked for.
4. **Given** a project, **When** the researcher changes its status (planned, active, on
   hold, completed, abandoned), **Then** the status and the date of the change are recorded
   and the history of status changes is kept.
5. **Given** several projects, **When** the researcher lists them filtered by type or
   status, **Then** only matching projects are shown, with their type, status, and dates.
6. **Given** several projects, **When** the researcher sets one as the current project,
   **Then** later commands apply to that project unless another is named, and the tool shows
   which project is current.
7. **Given** a project, **When** the researcher marks it completed or abandoned, **Then** a
   closing note is required, and the project becomes read-only until reopened.
8. **Given** a project whose expected end date is before its start date, or with a type
   that does not exist, **When** the researcher saves it, **Then** the tool rejects it and
   explains what is expected.
9. **Given** a project that contains records, **When** the researcher deletes it, **Then**
   the tool lists what it contains and asks whether to keep those records in the workspace
   or delete those that belong to no other project, and requires confirmation.
10. **Given** a workspace created before projects existed, **When** it is opened, **Then**
    all its records are placed in one project of type Independent Research, named after the
    workspace, and nothing is lost.

---
