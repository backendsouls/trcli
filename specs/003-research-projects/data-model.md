# Phase 1 Data Model: TRCLI Research Projects, Milestones, and Tasks

**Date**: 2026-10-08 | **Plan**: [plan.md](./plan.md) | **Spec**: [spec.md](./spec.md)

The domain model of the `projects` bounded context: aggregates, value objects, rules, state
transitions, and the tables that hold them. Table and column names follow these names in
`snake_case`.

## Conventions

- **(AR)** marks an aggregate root. One command changes one aggregate in one transaction,
  together with its audit entry.
- Every aggregate root has `id` (UUID v7), `handle` (`<prefix>-<code>`), `created_at`, and
  `updated_at`, as defined by the foundation; they are not repeated below.
- "Text(n..m)" is trimmed text of n to m characters; control characters other than newline
  and tab are rejected.
- Dates are calendar dates (`YYYY-MM-DD`) with no time of day. Timestamps are UTC.
- Rules cite the requirement they implement.
- People are referred to by id through the `PersonDirectory` port; the name as it was at
  the time is copied wherever history must survive the person's removal (FR-005).

## Value objects

| Value object | Rule |
|--------------|------|
| `ProjectTitle`, `MilestoneTitle`, `TaskTitle` | Text(1..500) |
| `Goal`, `Description`, `Deliverable`, `ClosingNote`, `Reason` | Text(0..20000); `ClosingNote` and `Reason` are 1.. where required |
| `TypeKey` | 1–40 characters of `a-z 0-9 -`; unique among project types |
| `Period` | optional `starts_on`, optional `ends_on`; when both are set, `ends_on ≥ starts_on` |
| `Priority` | `low`, `normal` (default), `high`, `urgent` |
| `Effort` | Text(0..50), recorded and not computed with |
| `Recurrence` | `daily`, `weekly`, `monthly`, or a positive count with a unit: days, weeks, months |
| `TaskNumber` | positive integer, local to one copy of the workspace |
| `TemplatePosition` | integer 0–100: where in a project's duration a milestone typically falls |
| `RelativeDate` | input only — `today`, `tomorrow`, a weekday name, `+Nd`, `+Nw`, or an ISO date — resolved to a date when entered |

## Project (AR) — prefix `prj`

| Field | Type | Rule |
|-------|------|------|
| `title` | ProjectTitle | required (FR-001) |
| `type_key` | TypeKey | must name an existing project type (FR-002) |
| `goal` | Goal | optional |
| `period` | Period | optional |
| `status` | ProjectStatus | default `planned` (FR-006) |
| `closing_note` | ClosingNote | required when `status` is `completed` or `abandoned` (FR-007) |
| `academic` | AcademicDetails | only for degree types (FR-003); not asked for otherwise (FR-004) |
| `funder`, `grant_reference` | Text(0..200) | only for the type `funded` (FR-004) |
| `extra_details` | map of name → Text(0..500) | the additional details a custom type asks for (FR-045) |

**AcademicDetails**: `institution`, `programme`, `degree_sought` (each Text(0..300)),
`supervisor` (person), `co_supervisors` (ordered people), `committee` (ordered people). Each
person is stored as id plus the name at the time.

**ProjectStatus transitions** (FR-006, FR-007):

```text
 planned ──▶ active ◀──▶ on_hold
    │           │            │
    └───────────┴────────────┴──▶ completed   (closing note required)
    └───────────┴────────────┴──▶ abandoned   (closing note required)
 completed / abandoned ── reopen ──▶ active
```

Any other move between `planned`, `active`, and `on_hold` is allowed. Every change appends
a `ProjectStatusChange` (status, date, note).

**Rules**:
- A `completed` or `abandoned` project is read-only: no record may be created or changed in
  it until it is reopened (FR-018). Enforced through `ProjectScope::ensure_writable`, which
  every context calls before a write.
- Deleting a project lists what it contains and asks whether to keep its records or delete
  those in no other project (FR-010). Its milestones and tasks are deleted with it.
- A title equal to an existing project's is accepted with a notice.

## ProjectType (AR) — named by its key, no handle

| Field | Type | Rule |
|-------|------|------|
| `key` | TypeKey | unique |
| `name` | Text(1..100) | unique ignoring case |
| `description` | Description | optional |
| `is_degree` | boolean | degree types ask for academic details |
| `extra_detail_names` | ordered list of Text(1..50) | custom types only (FR-045) |
| `template` | ordered list of MilestoneTemplateItem | the milestones it proposes |
| `built_in` | boolean | built-in types cannot be deleted or renamed |

**MilestoneTemplateItem**: `title` (MilestoneTitle), `position` (TemplatePosition),
`required` (boolean).

**Built-in types** are constants in the domain, not rows (FR-002, FR-020):

| Key | Name | Degree | Proposed milestones (position) |
|-----|------|--------|-------------------------------|
| `independent` | Independent Research | no | — |
| `undergraduate-research` | Undergraduate Research | yes | Work plan approved (10) · Interim report (50) · Final report (95) · Results presented (100) |
| `capstone` | Capstone Project | yes | Topic and supervisor defined (5) · Proposal approved (25) · Draft delivered to supervisor (75) · Final text submitted (90) · Presentation and defense (97) · Final version deposited (100) |
| `masters` | Master's | yes | Coursework completed (45) · Proposal defended (55) · Dissertation submitted (92) · Dissertation defended (97) · Final version deposited (100) |
| `doctoral` | Doctoral (PhD) | yes | Coursework completed (30) · Qualifying exam (40) · Proposal defended (50) · Thesis submitted (93) · Thesis defended (97) · Final version deposited (100) |
| `postdoctoral` | Postdoctoral | no | Work plan approved (5) · Interim report (50) · Final report (100) |
| `funded` | Funded Project | no | Kickoff (0) · Interim report (50) · Final report (100) |

**Stored**: only custom types, and — for a built-in type whose proposal the user changed —
a replacement template (FR-046). Restoring the original deletes the stored replacement.

**Rules**:
- A type used by any project cannot be deleted (FR-047).
- Changing a type's template affects projects created afterwards only (FR-046).
- Changing a project's type keeps its milestones, offers the new type's proposal, and shows
  which details are added or no longer apply (FR-048).

## Milestone (AR) — prefix `mil`

| Field | Type | Rule |
|-------|------|------|
| `project_id` | id | required; a milestone belongs to exactly one project and cannot move |
| `title` | MilestoneTitle | required (FR-019) |
| `description` | Description | optional |
| `target_on` | date | required |
| `date_confirmed` | boolean | `false` for a date suggested from a template until the user confirms or changes it (FR-021) |
| `status` | MilestoneStatus | default `upcoming` |
| `reached_on` | date | set when reached; not in the future (FR-024) |
| `required_by_programme` | boolean | FR-022 |
| `deliverable` | Deliverable | optional (FR-022) |

**MilestoneDateChange** (history, FR-023): `from`, `to`, `reason` (Reason 1..), `changed_at`.

**MilestoneStatus**: `upcoming`, `reached`, `missed`, `cancelled`.

```text
 upcoming ── reach ──▶ reached ── reopen ──▶ upcoming      (reached_on cleared; kept in audit)
 upcoming ── cancel ─▶ cancelled
 upcoming ── mark missed ─▶ missed ── reach (late) ──▶ reached
```

**Derived, never stored**:
- *Overdue* (FR-025): `status` is `upcoming` and `target_on` is before today; shown with the
  number of days.
- *Delay* (FR-024): `reached_on − target_on` when positive.
- *Progress*: tasks attached and done out of attached.

**Suggested dates** (FR-021): for a project with both ends of its period,
`target_on = starts_on + round(position / 100 × days in period)`, with
`date_confirmed = false`. Without a full period, no dates are suggested and the user is
asked for each.

**Rules**:
- A target date outside the project's period warns and needs confirmation (FR-027).
- Reaching a milestone with unfinished tasks warns and needs confirmation (FR-027).
- Accepting a proposal twice adds nothing that is already present, matched by title
  ignoring case.
- Deleting a milestone keeps its tasks, with no milestone.

## Task (AR) — prefix `tsk`

| Field | Type | Rule |
|-------|------|------|
| `project_id` | id | required; a task can be moved to another project (FR-039) |
| `title` | TaskTitle | required (FR-028) |
| `description` | Description | optional |
| `status` | TaskStatus | default `to_do` |
| `previous_status` | TaskStatus | the status before the last change to `done` or `cancelled`, so that "undo" restores it |
| `priority` | Priority | default `normal` |
| `due_on` | date | optional |
| `effort` | Effort | optional |
| `responsible` | person | optional; must exist in the person register (FR-029) |
| `milestone_id` | id | optional; a milestone of the same project (FR-030) |
| `parent_id` | id | optional; a task of the same project (FR-032) |
| `position` | integer | order among siblings |
| `starred` | boolean | default `false` (FR-063) |
| `recurrence` | Recurrence | optional (FR-038) |
| `repeats_from` | id | the occurrence this one was created from |
| `completed_on` | date | set when `done` (FR-035) |
| `concerns` | set of RecordRef | records the task is about; visible from both ends (FR-031) |

**TaskDependency**: `task_id` depends on `depends_on_id`. May cross projects after a move
(FR-039).

**TaskStatus**: `to_do`, `in_progress`, `done`, `cancelled`. Any change is allowed; each
records its date (FR-035).

```text
 to_do ◀──▶ in_progress
   │             │
   ├─────────────┴──▶ done ── undo ──▶ (previous status)
   └─────────────┴──▶ cancelled ── undo ──▶ (previous status)
```

**Derived, never stored**:
- *Blocked* (FR-033): at least one task it depends on is neither `done` nor `cancelled`.
- *Overdue*: `due_on` is before today and the task is `to_do` or `in_progress`.
- *Sub-task progress* (FR-032): direct sub-tasks done out of direct sub-tasks.

**Rules, enforced by the `task_graph` domain service** over the tasks of one project:
- A task cannot be its own ancestor (FR-034).
- Dependencies cannot form a loop; the error names the tasks in the loop (FR-034).
- A task's milestone must belong to the task's project; moving a task to another project
  clears its milestone and keeps its dependencies (FR-039).

**Other rules**:
- Marking `done` with unfinished sub-tasks warns and needs confirmation (FR-036).
- A due date after the target date of the task's milestone is accepted with a warning
  (FR-036).
- Completing a task with a recurrence creates the next occurrence: same title, project,
  milestone, priority, responsible person, and recurrence; due date advanced by the
  interval from the previous due date, or from the completion date when there was none;
  `to_do`; not starred. Cancelling it creates nothing (FR-038).
- When a record a task concerns is deleted, the task keeps the record's kind and name as
  text.
- When the last open sub-task of a task is done, or the last open task of a milestone, the
  tool offers to complete the parent or reach the milestone; nothing happens without
  acceptance (FR-062).

## Membership

**ProjectMembership**: `project_id`, `record: RecordRef`, `added_at`. Unique pair.

**Rules** (FR-012 to FR-017):
- A record may belong to several projects and exists once.
- A record created while a project is current belongs to that project (FR-013).
- Removing a membership never deletes the record; removing the last one warns and leaves
  the record unassigned on confirmation (FR-014).
- Milestones and tasks are **not** in this table: they carry `project_id` themselves.
- A workspace that has records and no project gets one `independent` project named after
  the workspace, with every record a member (FR-011). This runs once, when the feature's
  first migration is applied.

## Local-only state

These two tables are never part of what a workspace exchanges with another copy of itself.

| Table | Content | Why local |
|-------|---------|-----------|
| `task_number` | `task_id` → `number`; plus the next number to give | A short number is a typing convenience tied to what one screen showed (FR-058); see [research.md](./research.md) §7 |
| `local_state` | key → value; here the key `current_project` | Which project one is working in is a fact about a terminal session on one machine (FR-008) |

**TaskNumber rules**: assigned the first time a task is listed or created in this copy;
never changed; never given to another task, even after the task is deleted.

## Calculations

### Progress of a project (FR-040, FR-041)

| Figure | Definition |
|--------|------------|
| Milestones reached | `reached` out of all not `cancelled` |
| Tasks done | `done` out of all not `cancelled` |
| Time elapsed | `(today − starts_on) / (ends_on − starts_on)`, limited to 0–100%; not shown without a full period |
| Next milestone | the `upcoming` milestone with the earliest `target_on` |
| **Behind** | time elapsed > milestones reached (as shares) **and** at least one milestone is overdue |

### What is due (FR-042, FR-043)

A list of `DueItem { when, kind, title, record, project, overdue }` gathered from every
registered `DueSource`. This feature registers two: open tasks with a due date, and
`upcoming` milestones. Items are ordered overdue first, then by date. Only `active`
projects contribute unless others are asked for.

### To-do board (FR-053 to FR-057, FR-065)

```text
Board(project) ── sections ──▶ Section(milestone | "No milestone") ── lines ──▶ Line(task) ── children ──▶ Line …
```

- Sections are ordered by `target_on`; "No milestone" is last.
- Lines are ordered by `position`; open tasks before recently done ones.
- A done task is shown while `completed_on` is within `todo.keep_done`; older done tasks
  and cancelled tasks are counted as hidden.
- Counts on a board and a section count top-level tasks and sub-tasks alike, excluding
  cancelled ones.
- The summary counts every task in the boards shown.

## Tables

| Table | Holds | Shared between copies |
|-------|-------|-----------------------|
| `project` | Project, including academic details | yes |
| `project_status_change` | status history | yes |
| `project_type` | custom types, and changed templates of built-in types | yes |
| `milestone_template_item` | template rows of stored types | yes |
| `milestone` | Milestone | yes |
| `milestone_date_change` | target date history | yes |
| `task` | Task | yes |
| `task_dependency` | which task waits for which | yes |
| `task_concern` | records a task is about | yes |
| `project_membership` | records in projects | yes |
| `task_number` | short numbers | **no** |
| `local_state` | current project | **no** |

## What other specifications add later

- Further `DueSource` implementations: grant deadlines, calls, graduate requirements, items
  due for review. No change here.
- `specs/012-people` replaces the minimal person register behind `PersonDirectory`.
- `specs/006-reports` reads `project_status_change`, `milestone_date_change`, and task
  status dates through the audit trail for its planning section.
