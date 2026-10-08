<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI People and Lab Management

**Feature Branch**: `012-people`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add a new spec for staff an people management, just the minimum for a head of a lab handle the lab for example"

## Overview

Research is done by people, and someone — a lab head, a group leader, a supervisor with a
handful of students — has to keep them in mind: who is in the group, since when and until
when, what each is working on, who supervises whom, when they last talked, and what happens
to the work when somebody leaves.

This specification covers that, and deliberately **only the minimum**. It is a lab head's
notebook about their people, not a personnel system.

It **takes over** the staff content of the first specification, which now points here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 9 | The staff register and the assignment of people to work |

It **adds** four small things a lab head needs: each person's position and period in the
group, an overview of who is doing what, supervision with a record of meetings, and an
orderly handover when someone leaves.

### What this is not

| Not included | Why |
|--------------|-----|
| Salaries, contracts, leave, evaluations | That is the institution's personnel system |
| Accounts, logins, permissions | People here are records, not users of the workspace |
| Time sheets, hours worked | Out of scope across TRCLI |
| Recruitment, applications | Outside the life of the lab's research |

### The concepts, in one picture

```text
 Person ── has a ──▶ Position in the group (role, from, until, funding, status)
   │
   ├── supervised by ──▶ Person
   ├── assigned to ───▶ Project · Manuscript · Experiment · Step · Task · Dataset
   ├── met in ────────▶ Meeting ── agreed ──▶ Action (a task)
   └── on leaving ────▶ Handover: everything they hold, reassigned or closed
```

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Keep a register of the people in the group (Priority: P1)

A lab head records the people of the group and those who work with it: each person's name,
how to reach them, their position (doctoral student, postdoctoral researcher, technician,
visitor, outside collaborator…), when they joined, when they are expected to leave, and
what funds their stay. They can see at a glance who is in the group now, who is about to
leave, and who has left.

**Why this priority**: Every other part of TRCLI that names a person — an author, a
supervisor, someone responsible for an experiment — picks them from this register. With
only this story, a lab head has an accurate list of their group and its key dates.

**Independent Test**: Add four people in different positions with start and expected end
dates, list current members, list those whose stay ends within six months, mark one as
having left, and confirm they appear among former members.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the lab head adds a person with a name and a position,
   **Then** the person is stored as a current member of the group.
2. **Given** a person, **When** the lab head records their affiliation, e-mail address,
   researcher identifier, and web page, **Then** these are saved and shown.
3. **Given** a person, **When** the lab head records the date they joined, the date they are
   expected to leave, and what funds their stay with the date that funding ends, **Then**
   these are saved and shown.
4. **Given** people in the register, **When** the lab head lists them, **Then** current
   members are shown by default, with position, joining date, and expected end, and the list
   can be filtered by position, status, and funding.
5. **Given** people with expected end dates or funding end dates, **When** the lab head
   asks who is leaving or losing funding within a period, **Then** those people are listed
   in date order, and those dates also appear in the workspace's view of what is due.
6. **Given** a person who is not a member of the group, **When** the lab head adds them as
   an outside collaborator, **Then** they can be named as authors and contacts, and are not
   counted among the group's members.
7. **Given** a person whose position changes (a master's student becomes a doctoral
   student), **When** the lab head records the new position with its date, **Then** the
   earlier one is kept in the person's history.
8. **Given** a person, **When** the lab head views them, **Then** their details, position
   history, supervisor, and a summary of what they are assigned to are shown.
9. **Given** an e-mail address or a researcher identifier in an invalid form, an expected
   end before the joining date, or a missing name, **When** the lab head saves, **Then** the
   tool rejects it and reports every problem together.
10. **Given** a person with the same name or the same researcher identifier as one already
    in the register, **When** they are added, **Then** the tool reports the likely duplicate
    and asks whether to add or cancel.
11. **Given** a person, **When** the lab head adds a private note about them, **Then** the
    note is visible only in the workspace and is never included in anything produced for
    others.

---

### User Story 2 - See who is doing what (Priority: P2)

A lab head assigns people to the group's work — as members of a project, authors of a
manuscript, responsible for an experiment, performers of a manual step, owners of a task or
a dataset — and looks at it from the other side: for one person, everything they are on;
for the whole group, a table of who is on what, who has little, and which pieces of work
have nobody.

**Why this priority**: "Who is working on this?" and "what is this person working on?" are
the questions a lab head answers most often. The assignments already exist across the other
specifications; this gathers them into one view.

**Independent Test**: Assign three people across two projects, a manuscript, and several
tasks; view one person's assignments; view the group overview; and list the active projects
and experiments that have nobody responsible.

**Acceptance Scenarios**:

1. **Given** a project, **When** the lab head adds people to it with their role in it,
   **Then** the project lists its members and each person lists the project.
2. **Given** a person, **When** the lab head asks what they are assigned to, **Then** their
   projects, manuscripts (with their position among the authors), experiments, manual
   steps, tasks, and datasets are listed, with the status and next date of each.
3. **Given** the group, **When** the lab head asks for the overview, **Then** each current
   member is shown with the number of active items of each kind, their next deadline, and
   their overdue items.
4. **Given** the overview, **When** a member has no active assignment, or more overdue
   items than a number the lab head has set, **Then** they are marked.
5. **Given** active projects, experiments, and manuscripts, **When** the lab head asks what
   has nobody responsible, **Then** those are listed.
6. **Given** a person, **When** the lab head lists what is due for them within a period,
   **Then** their tasks, milestones, and deadlines are listed in date order.
7. **Given** a person assigned to something, **When** the lab head removes the assignment,
   **Then** it is removed from both sides and the earlier assignment is kept in history.
8. **Given** a person who has left, **When** the lab head tries to assign them something
   new, **Then** the tool warns and asks for confirmation.
9. **Given** a person or a piece of work that does not exist, **When** the lab head assigns,
   **Then** the tool rejects it.
10. **Given** the overview, **When** the lab head exports it, **Then** a document is
    produced without private notes.

---

### User Story 3 - Follow the people I supervise (Priority: P3)

A lab head records who supervises whom, and after each meeting with a student or a team
member writes down the date, what was discussed, and what was agreed — each agreed action
becoming a task with an owner and a date. Before the next meeting they see, for that
person, what was agreed last time and whether it was done, where their project and
programme stand, and how long it has been since they last met.

**Why this priority**: Supervision is the part of running a lab that is easiest to let
slide. A record of meetings and of what was agreed is small to keep and prevents most of
what goes wrong.

**Independent Test**: Record a supervision, record two meetings with agreed actions, mark
one action done, view the preparation for the next meeting, and list the supervised people
not met for more than four weeks.

**Acceptance Scenarios**:

1. **Given** two people, **When** the lab head records that one supervises the other, as
   main supervisor or co-supervisor, from a date, **Then** each shows the relationship.
2. **Given** a person, **When** the lab head records a meeting with them — the date, who
   attended, and notes of what was discussed — **Then** the meeting is stored and listed
   with the person.
3. **Given** a meeting, **When** the lab head records agreed actions, each with an owner
   and a date, **Then** each becomes a task linked to the meeting.
4. **Given** a supervised person, **When** the lab head asks to prepare their next meeting,
   **Then** the tool shows the actions agreed last time and their status, what the person
   has completed and what is overdue since, their next milestone and programme requirement,
   and the date of the last meeting.
5. **Given** supervised people, **When** the lab head asks who has not been met for longer
   than a period, **Then** those people are listed with the date of their last meeting.
6. **Given** a meeting with several attendees, **When** it is recorded, **Then** it appears
   for each attendee, and its actions may belong to different people.
7. **Given** a person, **When** the lab head lists their meetings, **Then** they are shown
   in order of date with their agreed actions and whether each was done.
8. **Given** a meeting, **When** the lab head marks its notes as private, **Then** they
   appear in nothing produced for others.
9. **Given** a supervision that ends (the student graduates, the supervisor changes),
   **When** the lab head records the end date, **Then** it is kept in history and no longer
   listed among current supervisions.
10. **Given** a person recorded as their own supervisor, a meeting dated in the future, or
    an action without an owner, **When** the lab head saves, **Then** the tool rejects it.
11. **Given** a supervisor, **When** the lab head asks whom they supervise, **Then** current
    supervisees are listed with their position, expected end, and next milestone.

---

### User Story 4 - Hand over when someone leaves (Priority: P4)

When a member leaves the group, the lab head sees everything that person holds — the
projects they are on, the experiments and datasets they are responsible for, their open
tasks, the manual steps waiting for them, the manuscripts in progress — and goes through
it: give each to someone else, close it, or leave it with the person as an outside
collaborator. The person then becomes a former member; their name stays on everything they
did.

**Why this priority**: Knowledge and responsibility leave with people. A short, complete
handover is the minimum that protects the lab's work, and it needs the earlier stories to
know what a person holds.

**Independent Test**: For a person with a project, two open tasks, a dataset, and a paused
experiment step, start the handover, reassign two items, close one, leave one with them,
complete it, and confirm they are a former member with nothing left unassigned.

**Acceptance Scenarios**:

1. **Given** a member, **When** the lab head starts their handover, **Then** everything they
   currently hold is listed by kind: projects, experiments, manual steps waiting for them,
   open tasks, datasets, manuscripts in progress, and people they supervise.
2. **Given** an item in the handover, **When** the lab head gives it to another person,
   **Then** the assignment moves, and the item records from whom it came and when.
3. **Given** an item, **When** the lab head closes it or leaves it with the departing
   person, **Then** the choice is recorded.
4. **Given** a handover, **When** the lab head adds notes for an item — where things are,
   what the next step was — **Then** the notes are kept with the item for whoever takes it.
5. **Given** a handover with items not yet decided, **When** the lab head tries to complete
   it, **Then** the tool lists them and asks for confirmation.
6. **Given** a completed handover, **When** it is confirmed, **Then** the person becomes a
   former member with their leaving date, or an outside collaborator if they keep working
   with the group.
7. **Given** a former member, **When** past records are viewed — runs they performed,
   manuscripts they authored, audit entries, meetings — **Then** their name remains.
8. **Given** a former member, **When** the lab head lists the group, **Then** they are not
   shown unless former members are asked for.
9. **Given** a handover, **When** the lab head exports it, **Then** a document listing what
   was handed to whom, with the notes, is produced.
10. **Given** a person who returns, **When** the lab head records a new period in the
    group, **Then** they are a current member again and their earlier period is kept.
11. **Given** a person, **When** the lab head asks to remove them from the register
    entirely, **Then** the tool explains that their name will remain where history needs
    it, removes their contact details and private notes, and requires confirmation.

---

### Edge Cases

- The lab head is themselves a person in the register: they are, and "my" views (whom I
  supervise, my meetings) use the person the workspace's researcher is set to.
- A person has two positions at once (a technician who is also a master's student): both
  are recorded, each with its own dates.
- A person has no expected end date (a permanent member): they never appear among those
  leaving.
- A person's funding ends before their expected end: both dates are shown, and the gap is
  marked.
- An expected end date passes and the person is still a current member: they are shown as
  past their expected end until the lab head updates the date or starts a handover.
- Two people have the same name: both are accepted after the duplicate warning; their
  identifiers and affiliations tell them apart wherever a person is chosen.
- A person's name changes: the new name is used from then on; what was already published or
  recorded keeps the name it had.
- A person is supervised by someone outside the group: the supervisor is added as an outside
  collaborator.
- A supervision would form a loop (A supervises B, B supervises A): the tool rejects it.
- A meeting is recorded with nobody but the lab head: accepted, as a note to self.
- An agreed action is given to a former member: accepted with a warning.
- A person leaves while a run is paused waiting for them at a manual step: the handover
  lists the step; whoever takes it can confirm it.
- A person holds nothing when they leave: the handover says so and completes at once.
- A handover is started and the person stays after all: it is cancelled, and items already
  moved stay moved unless moved back.
- A former member is named as an author of a new manuscript: allowed, without warning, since
  authorship outlives membership.
- The group is one person: every view works and simply shows one row.
- The overview is asked for with fifty people: it remains one table, sortable, shown in
  pages.
- Personal details are requested to be forgotten: contact details and private notes are
  removed; the name remains only where the record of the research needs it.

## Requirements *(mandatory)*

### Functional Requirements

#### People and positions

- **FR-001**: Users MUST be able to create, list, view, update, and remove people, each with
  a name, an affiliation, an e-mail address, a researcher identifier, and a web page.
- **FR-002**: Each person MUST have a relation to the group — member, outside collaborator,
  or former member — and members MUST have one or more positions, each with a title, a
  start date, an expected end date, and an actual end date.
- **FR-003**: The system MUST offer common positions — group leader, faculty, postdoctoral
  researcher, doctoral student, master's student, undergraduate student, technician,
  visitor — and accept any other.
- **FR-004**: Users MUST be able to record what funds a person's stay — a grant, a
  scholarship, or a free description — with the date that funding ends.
- **FR-005**: The system MUST keep the history of a person's positions and relation to the
  group.
- **FR-006**: Listing people MUST show current members by default and MUST be filterable by
  relation, position, funding, and supervisor; outside collaborators and former members
  MUST be shown on request.
- **FR-007**: Users MUST be able to list members whose expected end or funding end falls
  within a period; those dates MUST appear in the workspace's view of what is due, and a
  member past their expected end MUST be marked.
- **FR-008**: The system MUST report a likely duplicate when a person is added with the same
  name or researcher identifier as an existing one, and let the user add or cancel.
- **FR-009**: Users MUST be able to add private notes about a person. Private notes and
  contact details MUST NOT appear in anything produced for others unless the user asks for
  contact details explicitly.
- **FR-010**: Removing a person MUST keep their name wherever past records of the research
  refer to them, MUST remove their contact details and private notes, and MUST require
  confirmation.
- **FR-011**: The workspace MUST be able to state which person is the researcher who uses
  it, so that "my" views refer to them.

#### Assignments and overview

- **FR-012**: Users MUST be able to add people to a project with their role in it, and
  remove them; the assignment MUST be visible from both the project and the person.
- **FR-013**: The system MUST show, for a person, everything they are assigned to across the
  workspace — projects, manuscripts with their position among the authors, experiments,
  manual steps, tasks, datasets, and people they supervise — with the status and next date
  of each.
- **FR-014**: The system MUST show a group overview: each current member with their number
  of active items by kind, their next deadline, and their overdue items; members with no
  active assignment, and members with more overdue items than a number the user sets, MUST
  be marked.
- **FR-015**: Users MUST be able to list active projects, experiments, and manuscripts that
  have nobody responsible.
- **FR-016**: Users MUST be able to list what is due for one person within a period.
- **FR-017**: The system MUST keep the history of assignments, warn when something new is
  assigned to a former member, and reject assignments to people or work that do not exist.
- **FR-018**: Users MUST be able to export the group overview and a person's assignments as
  documents, without private notes.

#### Supervision and meetings

- **FR-019**: Users MUST be able to record that one person supervises another, as main
  supervisor or co-supervisor, with a start date and an end date; the system MUST reject a
  person supervising themselves and supervision that forms a loop.
- **FR-020**: Users MUST be able to list whom a person supervises, with each supervisee's
  position, expected end, and next milestone, and to see a person's supervisors.
- **FR-021**: Users MUST be able to record meetings, each with a date, the people who
  attended, and notes; a meeting MUST be listed with every attendee, and its notes MUST be
  markable as private.
- **FR-022**: Users MUST be able to record actions agreed in a meeting, each with an owner
  and a due date; each MUST become a task linked to the meeting.
- **FR-023**: The system MUST show, to prepare a meeting with a person, the actions agreed at
  their last meeting and whether each was done, what they completed and what became overdue
  since, their next milestone and programme requirement, and the date of the last meeting.
- **FR-024**: Users MUST be able to list supervised people who have not been met for longer
  than a period they state.
- **FR-025**: The system MUST reject a meeting dated in the future and an action without an
  owner.

#### Handover

- **FR-026**: Users MUST be able to start a handover for a member; the system MUST list
  everything the person currently holds: projects, experiments, manual steps waiting for
  them, open tasks, datasets, manuscripts in progress, and people they supervise.
- **FR-027**: For each item of a handover, users MUST be able to give it to another person,
  close it, or leave it with the departing person, and to add notes for whoever takes it;
  an item given to someone MUST record from whom it came and when.
- **FR-028**: Completing a handover with undecided items MUST list them and require
  confirmation; a handover MUST be cancellable, leaving items already moved where they are.
- **FR-029**: Completing a handover MUST make the person a former member with their leaving
  date, or an outside collaborator when the user says they keep working with the group.
- **FR-030**: A former member's name MUST remain on every past record — runs, manuscripts,
  versions, meetings, audit entries — and a former member MUST remain selectable as an
  author.
- **FR-031**: Users MUST be able to record that a former member has returned, with a new
  position; earlier periods MUST be kept.
- **FR-032**: Users MUST be able to export a handover as a document listing what went to
  whom, with the notes.

#### Common behavior

- **FR-033**: Every value a user supplies for people, positions, funding, assignments,
  supervisions, meetings, actions, and handovers MUST be validated before anything is
  stored; invalid input MUST change nothing and MUST be reported per value, all together,
  with what is expected.
- **FR-034**: People MUST support tags, notes, and links to any other record; every
  creation, change, removal, assignment, and handover MUST be recorded in the workspace's
  audit trail.
- **FR-035**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-036**: Every command MUST have built-in help, and the register, assignments and
  overview, supervision and meetings, and handover MUST each have a usage guide with
  examples.

### Key Entities *(include if feature involves data)*

- **Person**: Someone in or around the group: name, affiliation, contact details, researcher
  identifier, and relation to the group (member, outside collaborator, former member).
  Called a "staff member" in the first specification.
- **Position**: What a member is in the group during a period: title, start, expected end,
  actual end.
- **Funding**: What pays for a person's stay — a grant, a scholarship, or a description —
  and when it ends.
- **Assignment**: The fact that a person is on a piece of work, in a role, since a date.
  Held by the work itself (a project's members, a manuscript's authors, a task's owner) and
  gathered here per person.
- **Supervision**: One person supervising another, as main or co-supervisor, during a
  period.
- **Meeting**: A dated conversation with one or more people, with notes and agreed actions.
- **Action**: Something agreed in a meeting, with an owner and a date; a task linked to the
  meeting.
- **Handover**: The going-through of everything a departing member holds, with a decision
  and notes for each item.
- **Private Note**: Something the lab head writes about a person for themselves only.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A lab head can add a person with their position and dates in under 1 minute,
  and see the current group with everyone's expected end in under 5 seconds.
- **SC-002**: A lab head can see, for any person, everything they are assigned to across the
  workspace in a single view in under 5 seconds, and 100% of their assignments recorded
  anywhere in the workspace appear in it.
- **SC-003**: A lab head can see who is working on what across a group of 20 people in a
  single view in under 5 seconds.
- **SC-004**: 100% of members whose stay or funding ends within the chosen period are
  listed, and none whose dates fall outside it.
- **SC-005**: A lab head can record a meeting with three agreed actions in under 2 minutes,
  and prepare the next meeting with that person in under 10 seconds.
- **SC-006**: 100% of supervised people not met within the stated period are listed.
- **SC-007**: When a handover is completed, 0 open items remain assigned to the departing
  person without a recorded decision.
- **SC-008**: After a person leaves or is removed, their name is still shown on 100% of the
  past records that referred to them.
- **SC-009**: Private notes and contact details appear in 0 documents produced for others,
  unless contact details were explicitly asked for.
- **SC-010**: 100% of invalid inputs are rejected before any data changes, each with the
  invalid value named.
- **SC-011**: 90% of first-time users complete the primary task of each story on their
  first attempt using only that story's usage guide.

## Assumptions

- **This specification owns people.** User Story 9 of `specs/001-research-workspace`
  (FR-048 to FR-050) is replaced by this one; that specification keeps a short pointer.
  "Staff member" there means a person here.
- **It is the minimum, on purpose.** The request asked for just what a lab head needs. Four
  things were added to the plain register — positions with dates, an overview, supervision
  with meetings, and handover — and nothing else. Anything a personnel system does
  (salaries, contracts, leave, evaluations, recruitment) is out of scope.
- **The workspace is the lab head's own.** As everywhere in TRCLI, one person uses a
  workspace. The people in it are records: they do not log in, see the workspace, or
  receive anything from it. A group sharing one workspace, with roles and permissions, is
  collaboration in `specs/002-research-lifecycle`.
- **A student uses the same features from the other side.** A single researcher's workspace
  has the same register — their supervisor, co-authors, and collaborators — and the same
  meeting records; the group views simply have little to show.
- **Assignments stay where they are defined.** Who authors a manuscript is specified in
  `specs/008-manuscripts`, who is responsible for an experiment or performs a step in
  `specs/004-experiments`, who owns a task or supervises a project in
  `specs/003-research-projects`. This specification adds membership of a project and
  gathers all of them per person; it does not redefine them.
- **Agreed actions are tasks** of `specs/003-research-projects`, and dates to watch appear
  in its view of what is due; a supervisee's "next requirement" comes from
  `specs/011-courses-roadmaps`; funding may refer to a grant of
  `specs/002-research-lifecycle`.
- **"Private" is the marking introduced in `specs/006-reports`**; this specification relies
  on it to keep notes about people out of reports and exports.
- **Personal data is kept to what running a lab needs**: name, affiliation, contact,
  identifier, dates. The tool stores no identity documents, addresses, or health or
  financial details, and the lab head remains responsible for handling what they record
  according to their institution's rules.
- **Workload is counted, not measured.** The overview counts active and overdue items; it
  does not estimate hours or judge performance.
- **Meetings are notes, not scheduling.** The tool records meetings that happened; it does
  not send invitations or reminders.
