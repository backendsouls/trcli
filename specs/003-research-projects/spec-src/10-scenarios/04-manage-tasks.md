### User Story 4 - Manage tasks (Priority: P4)

A researcher breaks the work of a project into tasks: something to do, with a status, a
priority, a due date, the person responsible, and optionally the milestone it serves and
the record it concerns (read this paper, rerun that experiment, revise that draft). Tasks
can have sub-tasks and can depend on other tasks.

**Why this priority**: Tasks are the daily level of planning. They are more useful once
projects and milestones exist to group them, but each task is useful on its own.

**Independent Test**: In a project, create five tasks, attach three to a milestone, give
one a sub-task and one a dependency, complete two, and list the tasks filtered by status and
by milestone.

**Acceptance Scenarios**:

1. **Given** a project, **When** the researcher creates a task with a title, **Then** it is
   stored in that project with the status "to do".
2. **Given** a task, **When** the researcher sets its description, priority (low, normal,
   high, urgent), due date, estimated effort, or responsible person, **Then** the change is
   saved.
3. **Given** a task and a milestone of the same project, **When** the researcher attaches
   the task to the milestone, **Then** the milestone lists the task and counts it in its
   progress.
4. **Given** a task, **When** the researcher links it to a record such as a paper, a draft,
   or an experiment, **Then** the task shows the record and the record lists the task.
5. **Given** a task, **When** the researcher adds sub-tasks, **Then** the task shows how
   many of its sub-tasks are done.
6. **Given** a task that depends on an unfinished task, **When** the researcher views or
   lists it, **Then** it is shown as blocked and names what blocks it.
7. **Given** a task, **When** the researcher changes its status (to do, in progress, done,
   cancelled), **Then** the status and date are recorded; marking it done records the
   completion date.
8. **Given** a task with unfinished sub-tasks, **When** the researcher marks it done,
   **Then** the tool lists the unfinished sub-tasks and asks for confirmation.
9. **Given** tasks in a project, **When** the researcher lists them filtered by status,
   priority, milestone, responsible person, or due date, **Then** only matching tasks are
   shown, sorted as requested.
10. **Given** a task due after the target date of the milestone it serves, **When** the
    researcher saves it, **Then** the tool warns.
11. **Given** tasks that depend on each other in a loop, or a task made a sub-task of
    itself, **When** the researcher saves, **Then** the tool rejects it and names the tasks.
12. **Given** a task that repeats (for example, a weekly meeting with the supervisor),
    **When** the researcher marks it done, **Then** the next occurrence is created with the
    next due date.

---
