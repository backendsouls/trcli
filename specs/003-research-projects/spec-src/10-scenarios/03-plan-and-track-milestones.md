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
