### User Story 3 - Aim a report at a project and an audience (Priority: P3)

A researcher narrows a report to one project, chooses which sections it contains and how
much detail it shows, and picks the audience it is written for: themselves (everything), a
supervisor (progress, blockers, next steps), or a funder or programme (outputs and
milestones, no day-to-day detail).

**Why this priority**: The same period produces very different reports for different
readers. Scope and audience are what make a report something that can be handed over rather
than something to copy from.

**Independent Test**: In a workspace with two projects, produce a weekly report for one
project and confirm it contains nothing from the other; produce the same report for the
supervisor audience and confirm the sections differ from the personal one.

**Acceptance Scenarios**:

1. **Given** a workspace with several projects, **When** the researcher requests a report
   for one project, **Then** only activity on that project's records, tasks, and milestones
   is included.
2. **Given** a current project, **When** the researcher requests a report without naming a
   scope, **Then** the report covers the current project, and says so; the whole workspace
   can be asked for instead.
3. **Given** a report across the whole workspace, **When** it is shown, **Then** activity is
   grouped by project, with records that belong to no project grouped apart.
4. **Given** a report, **When** the researcher names the sections to include or to leave
   out, **Then** only those sections appear.
5. **Given** a report, **When** the researcher chooses a level of detail (summary, standard,
   full), **Then** the report shows counts only, counts with significant items, or every
   item.
6. **Given** a report, **When** the researcher chooses the supervisor audience, **Then** it
   contains progress against milestones, work completed, work in progress, blockers, and
   plans for the next period, and leaves out private notes.
7. **Given** a report, **When** the researcher chooses the funder audience, **Then** it
   contains outputs (drafts and their stage, results, datasets, talks), milestones, and
   deviations from plan, and leaves out day-to-day activity.
8. **Given** a report, **When** the researcher filters by tag or by person responsible,
   **Then** only matching activity is included.
9. **Given** a section name, audience, or project that does not exist, **When** the
   researcher requests the report, **Then** the tool rejects it and lists the valid ones.
10. **Given** records marked sensitive or notes marked private, **When** a report for any
    audience other than the researcher is produced, **Then** they are left out and the
    report says how many items were withheld.

---
