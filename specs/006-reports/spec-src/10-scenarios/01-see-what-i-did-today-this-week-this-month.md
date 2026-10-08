### User Story 1 - See what I did today, this week, this month (Priority: P1)

A researcher asks for a daily, weekly, or monthly report and immediately sees, on screen,
what happened in the workspace during that period: a short summary in numbers, then the
activity grouped by area of work — literature, writing, experiments, planning — and, at
the end, what needs attention and what is due next.

**Why this priority**: The short periods are the ones used most often, and a report on
screen with no options is the smallest thing that is useful. Everything else in this
specification refines it.

**Independent Test**: After a day with known activity (two references added, one read, one
run finished, one task completed), request the daily report and confirm that each of those
appears, in the right section, with correct counts; then request the weekly and monthly
reports and confirm they include that day.

**Acceptance Scenarios**:

1. **Given** a workspace with activity today, **When** the researcher requests the daily
   report, **Then** the report covers today from its start to now and lists that activity.
2. **Given** a workspace, **When** the researcher requests the weekly report, **Then** it
   covers the current week from its first day to now.
3. **Given** a workspace, **When** the researcher requests the monthly report, **Then** it
   covers the current calendar month to now.
4. **Given** any of these, **When** the researcher asks for the previous period instead,
   **Then** the report covers the last complete day, week, or month.
5. **Given** any of these, **When** the researcher names a date, **Then** the report covers
   the day, week, or month that contains it.
6. **Given** a report, **When** it is shown, **Then** it begins with the period's exact
   start and end dates, the scope, and a summary of counts, followed by one section per
   area of work that had activity.
7. **Given** a report, **When** an area of work had no activity in the period, **Then** its
   section is left out, and the summary says which areas were quiet.
8. **Given** a period with no activity at all, **When** the report is requested, **Then**
   the tool says so plainly and still shows what needs attention and what is due next.
9. **Given** a report, **When** it lists an item, **Then** the item shows what happened, to
   which record, and when, and names the record so the researcher can look it up.
10. **Given** a record that was created and later deleted within the period, **When** the
    report is produced, **Then** both events appear, with the record's name as it was.
11. **Given** a report, **When** it ends, **Then** it shows a section of what needs
    attention now (overdue items, runs waiting for a person) and what is due in the next
    period of the same length.
12. **Given** a date that does not exist or is in an invalid form, **When** the researcher
    requests a report for it, **Then** the tool rejects it and shows a valid example.

---
