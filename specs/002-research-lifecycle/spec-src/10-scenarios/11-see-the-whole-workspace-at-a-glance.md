### User Story 11 - See the whole workspace at a glance (Priority: P11)

A researcher asks for a single status view of the workspace: what is due and overdue, runs
that are waiting or failed, drafts by stage, open research questions, reading backlog,
approvals about to expire, and recent activity.

**Why this priority**: The view adds no new information, only convenience, and is only as
good as the records beneath it; it belongs after them.

**Independent Test**: In a populated workspace, request the status view and confirm each
section matches what the individual lists report.

**Acceptance Scenarios**:

1. **Given** a populated workspace, **When** the researcher requests the status view,
   **Then** one screen shows a section for each area that has something to report.
2. **Given** an area with nothing to report, **When** the view is shown, **Then** that
   section is omitted or shown as clear.
3. **Given** the status view, **When** the researcher asks for one section in detail,
   **Then** the full list behind it is shown.
4. **Given** an empty workspace, **When** the researcher requests the view, **Then** it
   suggests the first things to do.

---
