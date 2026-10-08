### User Story 2 - Plan tasks, milestones, and see what is due (Priority: P2)

> **Superseded in part (2026-10-08)**: tasks and milestones are now specified in
> `specs/003-research-projects`, where they belong to projects. What remains in scope here
> is extending the "what's due" view to the deadlines held by other record types (grants,
> approvals, venue calls, calibrations); see FR-012.

A researcher records to-dos and milestones, optionally attached to any record (a draft, an
experiment, a review, a grant), gives them due dates and priorities, and asks one question
across the whole workspace: "what is due, and what is late?" — which also includes the
deadlines already held by other records.

**Why this priority**: Research is governed by deadlines that live in many places. One view
of everything due is useful from the first week and needs nothing but the base workspace.

**Independent Test**: Create tasks with different due dates, attach one to a draft that has
its own deadline, and request the "what's due" view for the next 14 days.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a task with a title, **Then** it is
   stored with the status "to do".
2. **Given** a task, **When** the researcher sets a due date, a priority, an assignee, or the
   record it belongs to, **Then** the change is saved.
3. **Given** a workspace, **When** the researcher creates a milestone with a date and attaches
   tasks to it, **Then** the milestone shows how many of its tasks are done.
4. **Given** tasks, milestones, and other records with deadlines, **When** the researcher
   asks what is due within a period, **Then** every one is listed in date order with its
   source, and overdue items are marked.
5. **Given** a task, **When** the researcher marks it done, **Then** the completion date is
   recorded and it leaves the "what's due" view.
6. **Given** a task that depends on an unfinished task, **When** the researcher views it,
   **Then** it is shown as blocked and names what blocks it.
7. **Given** a due date in an invalid form, **When** the researcher saves the task, **Then**
   the tool rejects it and shows a valid example.

---
