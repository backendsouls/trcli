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
