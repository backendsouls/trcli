<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Research Projects, Milestones, and Tasks

**Feature Branch**: `003-research-projects`

**Created**: 2026-10-08

**Status**: Draft

**Amended**: 2026-10-08 — added the to-do list (User Story 7, FR-053 to FR-072, SC-013 to
SC-018), a view of projects, milestones, and tasks as one list.

**Input**: User description: "add concepts of projects, tasks, millistones, projects have a types Free Research (any better name), PhD, Master, Undergraduation, TCC(in portuguese trabalho de conclusao de curso, use english), etc"

## Overview

A researcher rarely does one piece of research at a time, and each piece has a shape set by
what it is for: a doctorate has a qualifying exam and a defense, a capstone project has a
proposal and a final presentation, independent research has neither. This specification
adds the **project** as the unit that organizes a researcher's work inside a workspace, gives
every project a **type** that reflects its purpose, and gives each project **milestones**
and **tasks** to plan and follow it.

A workspace (see `specs/001-research-workspace`) holds everything a researcher has. A
project is one research undertaking within it. Papers, drafts, experiments, datasets, and
every other record can belong to one or more projects, so a paper read for a master's
dissertation can be reused in the doctorate that follows.

### Project types

| Type | What it is |
|------|------------|
| Independent Research | Research not tied to a degree or programme, done on the researcher's own initiative. (Named "Free Research" in the request.) |
| Undergraduate Research | Research done during an undergraduate degree under a supervisor, outside the final-year requirement. |
| Capstone Project | The final project required to complete an undergraduate degree; also called an undergraduate thesis or final-year project. (TCC, *Trabalho de Conclusão de Curso*, in Portuguese.) |
| Master's | Research leading to a master's degree and its dissertation. |
| Doctoral (PhD) | Research leading to a doctorate and its thesis. |
| Postdoctoral | Research done in a postdoctoral position. |
| Funded Project | Research defined by a grant or an institution rather than by a degree. |
| Custom | Any other type the researcher defines. |

## User Scenarios & Testing *(mandatory)*

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

### User Story 3 - Plan and track milestones (Priority: P3)

A researcher lays out the major dated checkpoints of a project. When a project is created,
its type proposes a set of typical milestones — for a doctorate: coursework completed,
qualifying exam, proposal defense, thesis submitted, thesis defense, final version
deposited — which the researcher accepts, edits, or declines. As the project advances they
mark milestones reached, and the tool shows which are next and which have slipped.

**Why this priority**: Milestones are what make the type of a project matter. They are the
dates supervisors, programmes, and funders ask about, and they give the project a timeline
before any day-to-day task exists.

**Independent Test**: Create a master's project, accept the proposed milestones, change the
date of one, add one of their own, mark one reached, and view the project's timeline showing
reached, upcoming, and overdue milestones.

**Acceptance Scenarios**:

1. **Given** a new project of a type that has typical milestones, **When** it is created,
   **Then** the tool shows the proposed milestones and the researcher can accept all, choose
   some, or decline, and nothing is added without that choice.
2. **Given** accepted milestones and a project with start and expected end dates, **When**
   they are added, **Then** each receives a suggested date spread across the project's
   period, clearly marked as a suggestion to be confirmed.
3. **Given** a project, **When** the researcher adds a milestone with a title and a target
   date, **Then** it is stored with the status "upcoming".
4. **Given** a milestone, **When** the researcher changes its target date, **Then** the new
   date is saved and the previous date is kept in the milestone's history with a reason.
5. **Given** a milestone, **When** the researcher marks it reached, **Then** the date it was
   reached is recorded and, if that is after the target date, the delay is shown.
6. **Given** a milestone whose target date has passed and that is not reached, **When** the
   researcher views the project, **Then** the milestone is shown as overdue with the number
   of days.
7. **Given** a project, **When** the researcher asks for its timeline, **Then** milestones
   are shown in date order with their status, and the next one is highlighted.
8. **Given** a milestone, **When** the researcher marks it as required by the programme and
   records what must be delivered for it, **Then** this is shown with the milestone.
9. **Given** a milestone dated outside the project's start and expected end dates, **When**
   the researcher saves it, **Then** the tool warns and asks for confirmation.
10. **Given** a milestone with unfinished tasks, **When** the researcher marks it reached,
    **Then** the tool lists the unfinished tasks and asks for confirmation.

---

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

### User Story 5 - See progress and what is due (Priority: P5)

A researcher asks, for one project or for all of them, "where am I and what is next?". The
tool shows how far the project has come against its milestones and time, what is due in a
chosen period, and what is late. It also shows whether the project is on track to end by
its expected date.

**Why this priority**: This is the reason to enter milestones and tasks at all, but it only
has something to show once they exist.

**Independent Test**: In a project with milestones and tasks in different states, request
the progress view and the list of what is due in the next 14 days, and confirm both match
the underlying records; then request the same across all projects.

**Acceptance Scenarios**:

1. **Given** a project with milestones and tasks, **When** the researcher asks for its
   progress, **Then** they see milestones reached out of the total, tasks done out of the
   total, the share of the project's time elapsed, and the next milestone with the days
   remaining.
2. **Given** a project, **When** the researcher asks what is due in a period, **Then**
   every task and milestone due in that period is listed in date order, and overdue items
   are listed first.
3. **Given** several projects, **When** the researcher asks what is due across the
   workspace, **Then** items from all active projects are listed together, each naming its
   project.
4. **Given** a project in which more time has elapsed than milestones have been reached
   would suggest, **When** the researcher views its progress, **Then** the project is shown
   as behind, and the overdue milestones are named.
5. **Given** a project on hold, completed, or abandoned, **When** the researcher asks what
   is due across the workspace, **Then** its items are left out unless asked for.
6. **Given** a project with no milestones and no tasks, **When** the researcher asks for
   its progress, **Then** the tool says there is nothing to measure yet and suggests adding
   milestones.
7. **Given** a period in which nothing is due, **When** the researcher asks what is due,
   **Then** the tool says so and shows the next item after the period.
8. **Given** a project, **When** the researcher exports a progress report for a period,
   **Then** a document is produced listing what was reached, done, moved, and is pending,
   suitable to send to a supervisor.

---

### User Story 6 - Define custom project types and milestone templates (Priority: P6)

A researcher whose programme does not match the built-in types — a specialization course, a
research internship, a programme with its own required checkpoints — defines a project type
of their own, with the details it asks for and the milestones it proposes, or adjusts the
milestones a built-in type proposes to match their institution.

**Why this priority**: Degree structures differ between countries and institutions. The
built-in types cover the common cases; this story covers the rest and can come last.

**Independent Test**: Define a custom type with two extra details and four proposed
milestones, create a project of that type, and confirm the details are asked for and the
milestones proposed; then change the milestones a built-in type proposes and confirm new
projects use the change while existing ones are untouched.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher defines a project type with a name, a
   description, and whether it is a degree type, **Then** it can be chosen when creating
   projects.
2. **Given** a custom type, **When** the researcher defines the milestones it proposes, each
   with a title, an order, and a typical position in the project's duration, **Then** new
   projects of that type are offered those milestones.
3. **Given** a built-in type, **When** the researcher changes the milestones it proposes,
   **Then** the change applies to projects created afterwards and existing projects are not
   altered.
4. **Given** a built-in type whose proposed milestones were changed, **When** the researcher
   asks to restore the original, **Then** the original proposal returns.
5. **Given** a custom type used by existing projects, **When** the researcher deletes it,
   **Then** the tool refuses and lists the projects that use it.
6. **Given** a custom type with the same name as an existing type, **When** the researcher
   saves it, **Then** the tool rejects it.
7. **Given** an existing project, **When** the researcher changes its type, **Then** the
   tool shows which details would be added or no longer apply, keeps all existing
   milestones, offers the new type's milestones, and requires confirmation.

---

### User Story 7 - Work from a to-do list (Priority: P7)

A researcher opens their to-do list with one short command and sees, at a glance and
pleasant to look at, everything there is to do: each project as a board, each milestone as
a section within it with its date and its count of tasks done, each task as a line with a
checkbox — empty, in progress, or ticked — its sub-tasks indented beneath it, and small
marks for what is starred, urgent, blocked, or overdue. At the bottom, one line says how
much of it all is done. They add a task by typing its text, tick tasks off by their
numbers, and star the ones for today. The list is not a separate thing to maintain: it is
the projects, milestones, and tasks of this specification, shown as a list.

**Why this priority**: The earlier stories give tasks and milestones their rules; this one
gives them a face that is opened many times a day. It adds no new kind of record, so it
comes after them — but it is likely to be the most used command of all, and can be
delivered as soon as tasks exist.

**Independent Test**: With two projects, three milestones, and a dozen tasks in different
states, open the to-do list and confirm that projects, milestones, tasks, and sub-tasks
appear nested as they are, with correct counts and marks; add a task with one command,
tick two tasks by number, star one, and confirm the list and the summary line change
accordingly.

**Acceptance Scenarios**:

1. **Given** projects with milestones and tasks, **When** the researcher opens the to-do
   list, **Then** each active project is shown as a board with its title and its count of
   tasks done out of total; within it each milestone as a section with its title, target
   date, time remaining or overdue, and its own count; and within each milestone its tasks,
   with tasks that belong to no milestone under their own section.
2. **Given** tasks in different states, **When** they are listed, **Then** each shows a
   checkbox that tells its state at a glance — to do, in progress, done, cancelled — its
   title, and its sub-tasks indented beneath it with their own checkboxes.
3. **Given** tasks with a due date, a priority, a star, a dependency that blocks them, or a
   person responsible, **When** they are listed, **Then** each of these is shown by a small,
   consistent mark beside the task, and overdue tasks are unmistakable.
4. **Given** the to-do list, **When** it is shown, **Then** it ends with a summary: the
   share of all listed tasks that are done, and the numbers done, in progress, pending, and
   blocked.
5. **Given** a current project, **When** the researcher opens the to-do list without saying
   more, **Then** only that project's board is shown; all projects are shown on request,
   and with no current project.
6. **Given** the to-do list, **When** the researcher types a new task as plain text with one
   short command, **Then** it is added to the current project with the status "to do", and
   the tool confirms in one line with the task's number.
7. **Given** a new task typed with a mark for its milestone, its priority, its due date, or
   a star, **When** it is added, **Then** those are set from the marks, and the rest of the
   text is the title.
8. **Given** the numbers shown beside tasks, **When** the researcher ticks, unticks, starts,
   stars, or cancels tasks by giving one or several numbers, **Then** each task changes
   accordingly and the tool confirms what changed.
9. **Given** a task with sub-tasks, **When** all its sub-tasks are ticked, **Then** the tool
   offers to tick the task itself; ticking a task with unticked sub-tasks asks first.
10. **Given** completed tasks, **When** the to-do list is shown, **Then** tasks completed
    recently are shown ticked and visually quieter than open ones, and older completed
    tasks are left out unless asked for.
11. **Given** the to-do list, **When** the researcher asks for the timeline view, **Then**
    the same tasks and milestones are shown grouped by day — overdue, today, tomorrow, this
    week, later, and without a date — each still naming its project.
12. **Given** the to-do list, **When** the researcher asks for only starred tasks, only
    those of a person, only those of a milestone, only what is pending, or those matching a
    word, **Then** only matching tasks are shown, with the boards and sections they belong
    to.
13. **Given** a milestone reached or a project with nothing left to do, **When** the list is
    shown, **Then** it is shown as complete, in a way that reads as an achievement.
14. **Given** an empty workspace, or a project with no tasks, **When** the to-do list is
    opened, **Then** it says so in a friendly line and shows how to add the first task.
15. **Given** a terminal that cannot show color or special symbols, or output that is sent
    to a file or another program, **When** the to-do list is produced, **Then** it is
    readable in plain characters, every state is still distinguishable without color, and
    nothing decorative is left in output meant for programs.
16. **Given** a narrow terminal or very long titles, **When** the list is shown, **Then**
    lines are shortened with a visible mark and the layout stays aligned.
17. **Given** a number that matches no task, or a number given for a task already in the
    state asked for, **When** the researcher acts on it, **Then** the tool says so for that
    number and still acts on the other numbers given.
18. **Given** changes made through the to-do list, **When** the same tasks are viewed with
    the task commands, **Then** they show the same state, since both are the same records.

---

### Edge Cases

- A project is created with the same title as an existing project: it is accepted with a
  warning, since identifiers distinguish them.
- The current project is deleted, completed, or abandoned: there is no current project
  afterwards, and the tool says so.
- A command is given for the current project and there is none: the tool asks which project
  is meant, or uses the only project if there is exactly one.
- A record is created in a project that is completed or abandoned: the tool refuses and
  offers to reopen the project.
- A supervisor recorded on a project is removed from the staff register: the project keeps
  the name, as other records do.
- A project's expected end date is moved earlier than existing milestones: the tool lists
  the milestones now outside the period and asks for confirmation.
- A milestone is deleted while tasks are attached: the tasks are kept and shown as attached
  to no milestone.
- A milestone is marked reached with a date in the future: the tool rejects it.
- A milestone marked reached is reopened: the reached date is cleared and the reopening is
  kept in its history.
- A task is moved to another project: its milestone is cleared, since milestones belong to
  one project, and its dependencies on tasks of the old project are kept and shown as
  cross-project.
- A task is linked to a record that is later deleted: the task is kept and names what it
  used to concern.
- A task's responsible person is not in the staff register: the tool rejects it and offers
  to add the person.
- A repeating task is cancelled: no further occurrence is created.
- A project has no start or expected end date: suggested milestone dates are not proposed,
  and time elapsed is not shown in progress.
- Proposed milestones are accepted twice for the same project: the tool reports those
  already present and adds none of them again.
- Two projects of the same degree type are active at once: both are allowed.
- A due date or target date is in an invalid form, or a title is empty or far too long: the
  tool rejects it, names the field, and shows a valid example.
- A project holds thousands of tasks: lists remain usable, shown in pages or limited to a
  stated number with a count of the rest.
- The to-do list is opened with hundreds of open tasks in one project: milestones with many
  tasks show the first ones and how many more there are, and can be opened in full.
- A task belongs to a milestone that is cancelled or already reached: it is shown under that
  milestone, marked, so that it is not lost.
- A sub-task is ticked whose parent is cancelled: accepted; the parent stays cancelled.
- Text typed for a new task begins with something that looks like a mark but is not one
  (an e-mail address, a price): it is kept as text.
- Two numbers given in one command are the same, or a number is given both to tick and to
  cancel: the tool acts once and says so, or refuses the contradictory pair and acts on the
  rest.
- A task is ticked by mistake: unticking restores the state it had before, including "in
  progress".
- A repeating task is ticked from the list: its next occurrence appears with a new number.
- The list is shown while another terminal changes tasks: the next opening shows the new
  state; the tool never shows a mix.

## Requirements *(mandatory)*

### Functional Requirements

#### Projects

- **FR-001**: Users MUST be able to create, list, view, update, and delete projects within a
  workspace, each with a title, a type, a goal or description, and a short identifier that
  is unique in the workspace and stable for the life of the project.
- **FR-002**: The system MUST provide these project types: Independent Research,
  Undergraduate Research, Capstone Project, Master's, Doctoral (PhD), Postdoctoral, and
  Funded Project.
- **FR-003**: For degree types (Undergraduate Research, Capstone Project, Master's,
  Doctoral), users MUST be able to record the institution, programme, degree sought,
  supervisor, co-supervisors, examining committee, start date, and expected end date.
- **FR-004**: For non-degree types, the system MUST NOT require academic details; for Funded
  Project, users MUST be able to record the funder and the grant reference.
- **FR-005**: Supervisors, co-supervisors, and committee members MUST be chosen from the
  workspace's staff register, and the project MUST keep their names if they are later
  removed from it.
- **FR-006**: The system MUST track each project's status (planned, active, on hold,
  completed, abandoned), record the date of each change, and keep the history.
- **FR-007**: The system MUST require a closing note to mark a project completed or
  abandoned, MUST make such a project read-only, and MUST allow it to be reopened.
- **FR-008**: Users MUST be able to set one project as current; commands that concern a
  project MUST apply to the current project unless another is named, and the system MUST
  make it clear which project is current.
- **FR-009**: When no project is current and more than one exists, the system MUST ask which
  project is meant and MUST NOT guess.
- **FR-010**: Before deleting a project, the system MUST list what it contains, MUST let the
  user choose between keeping its records in the workspace and deleting those that belong
  to no other project, and MUST require explicit confirmation.
- **FR-011**: When a workspace that has no projects is opened, the system MUST place its
  existing records in one Independent Research project named after the workspace, without
  losing or altering any record.

#### Records in projects

- **FR-012**: Every record type of the workspace MUST be able to belong to one or more
  projects; a record that belongs to several MUST exist only once.
- **FR-013**: A record created while a project is current MUST belong to that project.
- **FR-014**: Users MUST be able to add a record to further projects and remove it from a
  project; removing it MUST NOT delete it.
- **FR-015**: Lists and searches MUST show only the current project's records by default and
  MUST show the whole workspace on request.
- **FR-016**: The system MUST show, for any record, the projects it belongs to, and MUST be
  able to list records that belong to no project.
- **FR-017**: The system MUST show, for any project, the number of records of each type it
  contains.
- **FR-018**: The system MUST refuse to create or change records in a completed or abandoned
  project until it is reopened.

#### Milestones

- **FR-019**: Users MUST be able to create, list, view, update, and delete milestones in a
  project, each with a title, description, target date, and status (upcoming, reached,
  missed, cancelled).
- **FR-020**: Each built-in project type MUST propose a set of typical milestones; when a
  project is created, the system MUST show the proposal and MUST add only the milestones
  the user accepts.
- **FR-021**: When the project has start and expected end dates, accepted milestones MUST
  receive suggested target dates across that period, marked as suggestions until the user
  confirms or changes them.
- **FR-022**: Users MUST be able to mark a milestone as required by the programme and record
  what must be delivered for it.
- **FR-023**: The system MUST keep the history of changes to a milestone's target date, with
  the reason given for each change.
- **FR-024**: Marking a milestone reached MUST record the date; the system MUST reject a
  reached date in the future and MUST show the delay when it is after the target date.
- **FR-025**: The system MUST show a milestone as overdue, with the number of days, when its
  target date has passed and it is neither reached nor cancelled.
- **FR-026**: The system MUST present a project's milestones as a timeline in date order
  with their status and the next one identified.
- **FR-027**: The system MUST warn, and require confirmation, when a milestone's target date
  falls outside the project's period, and when a milestone with unfinished tasks is marked
  reached.

#### Tasks

- **FR-028**: Users MUST be able to create, list, view, update, and delete tasks in a
  project, each with a title, description, status (to do, in progress, done, cancelled),
  priority (low, normal, high, urgent), due date, estimated effort, and responsible person.
- **FR-029**: The responsible person MUST be chosen from the workspace's staff register.
- **FR-030**: Users MUST be able to attach a task to one milestone of the same project, and
  the milestone MUST show its tasks and how many are done.
- **FR-031**: Users MUST be able to link a task to any records it concerns, with the link
  visible from both ends.
- **FR-032**: Users MUST be able to give a task sub-tasks, and the task MUST show how many
  are done.
- **FR-033**: Users MUST be able to make a task depend on other tasks; the system MUST show
  a task as blocked while a task it depends on is unfinished, naming it.
- **FR-034**: The system MUST reject dependency loops and a task made a sub-task of itself
  or of its own sub-task.
- **FR-035**: The system MUST record the date of every status change and the completion date
  of a task marked done.
- **FR-036**: The system MUST warn when a task is marked done with unfinished sub-tasks, and
  when a task is due after the target date of its milestone.
- **FR-037**: Users MUST be able to filter tasks by status, priority, milestone, responsible
  person, due date, and linked record, and sort by due date or priority.
- **FR-038**: Users MUST be able to make a task repeat at a stated interval; completing an
  occurrence MUST create the next, and cancelling it MUST stop the repetition.
- **FR-039**: Users MUST be able to move a task to another project; the system MUST clear
  its milestone and show remaining dependencies as cross-project.

#### Progress and what is due

- **FR-040**: The system MUST show, for a project, milestones reached out of the total,
  tasks done out of the total, the share of the project's period elapsed, and the next
  milestone with the days remaining.
- **FR-041**: The system MUST show a project as behind when the share of time elapsed
  exceeds the share of milestones reached and at least one milestone is overdue.
- **FR-042**: Users MUST be able to list what is due in a chosen period for one project or
  across the workspace, in date order, with overdue items first and each item naming its
  project.
- **FR-043**: The view across the workspace MUST include active projects only, unless the
  user asks for others.
- **FR-044**: *(Superseded by `specs/006-reports`, which defines reports for any period,
  scope, and audience; kept here for numbering.)* Users MUST be able to export a progress report for a project and a period,
  listing milestones reached, tasks done, dates moved, and what is pending.

#### Custom types and templates

- **FR-045**: Users MUST be able to define custom project types with a name, description,
  whether it is a degree type, additional details to record, and proposed milestones.
- **FR-046**: Users MUST be able to change the milestones a built-in type proposes and
  restore the original proposal; such changes MUST apply only to projects created
  afterwards.
- **FR-047**: The system MUST refuse to delete a type used by any project, and MUST reject a
  type whose name duplicates an existing one.
- **FR-048**: Users MUST be able to change a project's type; the system MUST show the
  effect, keep all existing milestones, offer the new type's milestones, and require
  confirmation.

#### Common behavior

- **FR-049**: Every value a user supplies for projects, milestones, tasks, and types MUST be
  validated before anything is stored: required values present, text within stated lengths,
  dates valid and in a consistent order, and referenced projects, milestones, tasks, staff,
  and records existing. Invalid input MUST change nothing and MUST be reported per value
  with what is expected.
- **FR-050**: Projects, milestones, and tasks MUST support tags, notes, and links, and every
  creation, change, deletion, and status change MUST be recorded in the workspace's audit
  trail.
- **FR-051**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-052**: Every command MUST have built-in help, and projects, milestones, tasks, and
  project types MUST each have a usage guide with examples.

#### To-do list

- **FR-053**: The system MUST provide a to-do list that presents the projects, milestones,
  tasks, and sub-tasks of this specification as they are structured: each project as a
  board, each of its milestones as a section, tasks under the milestone they serve, tasks
  with no milestone under a section of their own, and sub-tasks indented beneath their
  task, to any depth.
- **FR-054**: The to-do list MUST NOT be a separate store of items: every line MUST be a
  task or milestone record of this specification, and every change made through the list
  MUST be the same change as made through the task and milestone commands, validated and
  audited in the same way.
- **FR-055**: Each task line MUST show a checkbox whose form tells its state — to do, in
  progress, done, cancelled — the task's short number, and its title; and, where they
  apply, marks for a star, the priority, the due date as time remaining or overdue, being
  blocked, the responsible person, and having notes.
- **FR-056**: Each board MUST show the project's title and its tasks done out of total; each
  milestone section MUST show its title, target date, time remaining or overdue, whether it
  is required, and its tasks done out of total.
- **FR-057**: The list MUST end with a summary: the percentage of listed tasks that are
  done, and the counts done, in progress, pending, and blocked.
- **FR-058**: Every task MUST have a short number, shown in the list, by which it can be
  named in commands; a task's number MUST NOT change while the task is open and MUST NOT be
  given to another task.
- **FR-059**: Users MUST be able to add a task by giving only its text; it MUST be added to
  the current project with the status "to do" and confirmed in one line with its number.
  With no current project and several projects, the system MUST ask which, or accept a
  project named in the command.
- **FR-060**: When adding from the to-do list, users MUST be able to set a task's project,
  milestone, parent task, priority, due date, star, and responsible person by short marks
  typed with the text; the marks MUST be removed from the title, and an unrecognized mark
  MUST be kept as text.
- **FR-061**: Users MUST be able to tick, untick, start, cancel, star, unstar, set the
  priority of, move, and delete tasks by one or several numbers in one command; the system
  MUST report what changed for each, MUST report each number that matched nothing or needed
  no change, and MUST still act on the others.
- **FR-062**: Ticking a task with unticked sub-tasks MUST ask first; when the last sub-task
  of a task is ticked, the system MUST offer to tick the task. Ticking the last open task
  of a milestone MUST offer to mark the milestone reached and MUST do so only on acceptance.
- **FR-063**: Users MUST be able to star tasks to single them out, independently of
  priority, and to list only starred tasks.
- **FR-064**: By default the list MUST show the current project when there is one and all
  active projects otherwise; users MUST be able to ask for all projects, one project, or
  completed and on-hold projects.
- **FR-065**: Tasks completed within a recent period MUST be shown ticked and visually
  quieter than open tasks; tasks completed earlier and cancelled tasks MUST be left out
  unless asked for, and the summary MUST say how many are hidden.
- **FR-066**: The system MUST provide a timeline view of the same tasks and milestones
  grouped by date — overdue, today, tomorrow, the rest of this week, later, and without a
  date — each naming its project.
- **FR-067**: Users MUST be able to narrow the list to starred tasks, pending tasks, a
  milestone, a responsible person, a priority, a tag, or tasks whose text contains given
  words; boards and sections with no matching task MUST be left out.
- **FR-068**: A milestone that is reached, and a project with every task done, MUST be shown
  as complete in a way that distinguishes achievement from mere absence of work; an empty
  list MUST say so and show how to add a task.
- **FR-069**: The to-do list MUST be pleasant and quick to read: aligned columns, consistent
  symbols, restrained use of color in which each color has one meaning, open work visually
  ahead of finished work, and no more than one line per task unless details are asked for.
- **FR-070**: Every state and mark MUST be distinguishable without color and without special
  symbols: where a terminal cannot show them, or the user turns them off, plain characters
  MUST be used; lines MUST be shortened with a visible mark to fit the width available.
- **FR-071**: The list MUST be available in a structured form meant for other programs,
  nested as projects, milestones, tasks, and sub-tasks, with no decoration.
- **FR-072**: The symbols, the period for which completed tasks remain visible, and whether
  the default view is the board or the timeline MUST be configurable.

### Key Entities *(include if feature involves data)*

- **Project**: One research undertaking within a workspace. Has a title, type, goal, status
  with history, optional academic details, and optional period. Contains milestones and
  tasks, and has records of every other kind belonging to it.
- **Project Type**: The purpose of a project, built-in or custom. Determines which details
  are asked for and which milestones are proposed.
- **Academic Details**: For degree-type projects: institution, programme, degree sought,
  supervisor, co-supervisors, and examining committee.
- **Milestone Template**: The ordered set of milestones a project type proposes, each with a
  typical position in the project's duration.
- **Milestone**: A dated checkpoint of one project. Has a target date with its history, a
  status, whether the programme requires it, what must be delivered, and the tasks that
  serve it.
- **Task**: A piece of work in one project. Has a status, priority, due date, estimated
  effort, responsible person, an optional milestone, optional parent task, dependencies,
  optional repetition, and links to the records it concerns. Also carries a
  star and a short number by which the to-do list names it.
- **Project Membership**: The fact that a record belongs to a project; a record may have
  several.
- **Workspace** *(from the base specification, refined here)*: Everything a researcher
  keeps in one place. Holds one or more projects, the staff register, and the records they
  share.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A new user can create a project of any built-in type, accept its proposed
  milestones, and add a first task in under 3 minutes.
- **SC-002**: A user with five projects can tell, in under 10 seconds and from one view,
  what is due this week across all of them and which project each item belongs to.
- **SC-003**: A user can see how far any project has come and what its next milestone is in
  under 5 seconds.
- **SC-004**: 100% of overdue milestones and tasks are shown as overdue wherever they
  appear.
- **SC-005**: In a workspace with 20 projects, 5,000 tasks, and 10,000 papers, listing a
  project's tasks or records takes under 2 seconds.
- **SC-006**: A record added to a second project appears in both and is stored once; 100% of
  changes made through one project are visible through the other.
- **SC-007**: 100% of invalid inputs for projects, milestones, and tasks are rejected before
  any data changes, each with the invalid value named.
- **SC-008**: Opening a workspace created before projects existed loses 0 records and
  requires no action from the user.
- **SC-009**: At least 80% of users creating a degree-type project keep at least half of
  the proposed milestones, showing that the proposals fit real programmes.
- **SC-010**: A user can define a custom project type with four proposed milestones in under
  5 minutes.
- **SC-011**: A progress report for a three-month period can be produced in under 1 minute
  and be sent to a supervisor without editing.
- **SC-012**: 90% of first-time users complete the primary task of each story on their first
  attempt using only that story's usage guide.
- **SC-013**: A user can open the to-do list with one short command and see it in under 1
  second for a workspace with 20 projects and 2,000 open tasks.
- **SC-014**: A user can add a task in under 5 seconds and tick three tasks with a single
  command.
- **SC-015**: For any state of the workspace, the to-do list and the task and milestone
  commands show the same tasks in the same states in 100% of cases.
- **SC-016**: Shown without color and without special symbols, 100% of task states and marks
  remain distinguishable.
- **SC-017**: A user shown the to-do list for the first time can say which tasks are done,
  in progress, overdue, and starred, and which milestone each belongs to, without being
  told what the symbols mean, in at least 90% of cases.
- **SC-018**: At least 80% of users who try the to-do list say they would rather keep their
  research tasks in it than in the to-do tool they used before.

## Assumptions

- **A workspace holds many projects.** The base specification described a workspace as "one
  research project or programme"; with this specification a workspace is the researcher's
  whole body of work and a project is one undertaking inside it. The staff register, the
  audit trail, and telemetry stay at the workspace level.
- **Records are shared, not copied.** A record may belong to several projects, and there is
  one copy of it. A researcher who wants two projects fully separated uses two workspaces.
- **Milestones and tasks belong to exactly one project.** A task can be moved; a milestone
  cannot.
- **"Independent Research" is the English name chosen for "Free Research"**, and "Capstone
  Project" for TCC (*Trabalho de Conclusão de Curso*); "Undergraduate Research" is read as
  supervised research during an undergraduate degree (as in *iniciação científica*), which
  is distinct from the final-year capstone. Type names can be reworded without changing
  behavior.
- **"etc." in the request** is covered by adding Postdoctoral and Funded Project as built-in
  types and by custom types for anything else, such as specialization courses or research
  internships.
- **Proposed milestones are generic starting points.** Degree structures differ by country
  and institution, so proposals are always shown for acceptance and are editable; the tool
  does not claim to know any programme's rules. The initial proposals are:
  - *Independent Research*: none.
  - *Undergraduate Research*: work plan approved, interim report, final report, results
    presented.
  - *Capstone Project*: topic and supervisor defined, proposal approved, draft delivered to
    supervisor, final text submitted, presentation and defense, final version deposited.
  - *Master's*: coursework completed, proposal defended (qualifying), dissertation
    submitted, dissertation defended, final version deposited.
  - *Doctoral (PhD)*: coursework completed, qualifying exam, proposal defended, thesis
    submitted, thesis defended, final version deposited.
  - *Postdoctoral*: work plan approved, interim report, final report.
  - *Funded Project*: kickoff, interim report, final report.
- **This specification supersedes the tasks and milestones of
  `specs/002-research-lifecycle`** (its User Story 2 and FR-009 to FR-011). What remains
  there is the wider "what's due" view that also gathers deadlines held by other record
  types (grants, approvals, venue calls); it extends the view defined here.
- **Planning, not scheduling.** There are no calendars, reminders or notifications sent
  outside the tool, time tracking, charts of task dependencies over time, or workload
  balancing between people. Estimated effort is recorded and not otherwise used.
- **Single researcher.** As in the base workspace, one person uses the tool; "responsible
  person" and "supervisor" name people from the staff register and do not give them access.
- **Dates are calendar dates** without times of day or time zones.
- **Interface language is English.**
- **The to-do list is a view, not a second system.** It was added to this specification on
  2026-10-08 as User Story 7. It shows and changes the same projects, milestones, and tasks
  the other stories define; it introduces a star on tasks and a short number for each, and
  nothing else that is stored.
- **"Beautiful" is specified by what the reader must be able to see**, not by a particular
  look: structure visible at a glance, states told by the checkbox, a few consistent marks,
  quiet finished work, one summary line. The exact symbols and colors are chosen at
  planning time, in the spirit of the board-and-checkbox to-do lists popular in terminals,
  and can be changed by the user.
- **Quick notes are not part of the to-do list.** A thought that is not yet a task is
  captured with the inbox of `specs/013-ideas-questions` and turned into a task there.
- **Other dated things stay in the view of what is due.** Deadlines of grants, calls,
  requirements, and reviews appear in `trcli due`; the to-do list shows tasks and
  milestones only.
