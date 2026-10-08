<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Activity Reports

**Feature Branch**: `006-reports`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add report function spec for daily, weekly, monthly, bimonthly, half-year(better name here), custom dates"

## Overview

A researcher is regularly asked "what did you do?" — by themselves at the end of a day, by
a supervisor every week, by a programme every semester, by a funder every year. The answer
is already in the workspace: every reference added, page annotated, run finished, result
recorded, task completed, and milestone reached is on record. This specification adds the
**report**: an account of what happened in the workspace during a period of time, produced
on demand, for the period and the audience the researcher chooses.

### Report periods

| Period | Covers | Note |
|--------|--------|------|
| Daily | One calendar day | |
| Weekly | Seven days, starting on the configured first day of the week | Monday by default |
| Monthly | One calendar month | |
| Bimonthly | Two consecutive calendar months | "Bimonthly" here always means *every two months*, never twice a month |
| Quarterly | Three consecutive calendar months | Added; not in the request |
| **Semiannual** | Six consecutive calendar months | The name chosen for "half-year"; "semester" is accepted as another word for it |
| Annual | Twelve consecutive calendar months | Added; not in the request |
| Custom | Any start date to any end date | Also "the last N days" |

Periods longer than a month are counted from the first month of the researcher's year,
which is January unless they set another (for example, an academic year starting in
August), so that "the second semiannual period" means the same thing to them and to their
institution.

## User Scenarios & Testing *(mandatory)*

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

### User Story 2 - Report on longer and custom periods (Priority: P2)

A researcher asks for a bimonthly, quarterly, semiannual, or annual report, or for any
range of dates they choose — "from the 3rd of March to the 17th of April", "the last 10
days", "since my last meeting". Long reports are summarized more heavily so they stay
readable.

**Why this priority**: These are the periods institutions and funders ask for. They reuse
everything in the first story and add the rules for where periods begin and how much detail
a long report shows.

**Independent Test**: Request a semiannual report and confirm its dates; set the first
month of the year to August and confirm the dates move; request a custom range and a "last
10 days" report and confirm their dates and content.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher requests a bimonthly, quarterly,
   semiannual, or annual report, **Then** it covers the current period of that length,
   counted from the first month of the researcher's year.
2. **Given** the first month of the year is set to August, **When** the researcher requests
   the semiannual report in October, **Then** it covers August to January.
3. **Given** any of these periods, **When** the researcher asks for the previous one, or
   names a date, or names the period by its number and year ("second semiannual period of
   2026"), **Then** the report covers that period.
4. **Given** a start date and an end date, **When** the researcher requests a custom
   report, **Then** it covers both dates and every day between them.
5. **Given** a number of days, weeks, or months, **When** the researcher requests a report
   for "the last N", **Then** it covers that span ending today.
6. **Given** only a start date, **When** the researcher requests a custom report, **Then**
   it covers from that date to now.
7. **Given** a report longer than one month, **When** it is shown, **Then** activity is
   summarized by month within the period, and individual items are listed only for
   significant events (milestones reached, drafts changing stage, experiments concluded,
   reviews completed) unless the researcher asks for full detail.
8. **Given** an end date before the start date, a start date in the future, or a span of
   zero or a negative number, **When** the researcher requests the report, **Then** the
   tool rejects it and explains.
9. **Given** a period that begins before the workspace existed, **When** the report is
   produced, **Then** it covers from the workspace's first day and says so.
10. **Given** a period that has not ended, **When** the report is produced, **Then** it is
    marked as partial, with the date it was produced.
11. **Given** the word "semester" or "half-year" is used for the period, **When** the
    researcher requests the report, **Then** it is understood as semiannual.

---

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

### User Story 4 - Add my own words, save, and share (Priority: P4)

A researcher adds what the records cannot say — the highlights of the period, what blocked
them, what they plan next — then saves the report as it stands, so there is a permanent
copy of what was reported and when. They export it as a document to send, and can find
past reports later.

**Why this priority**: A report that is only numbers is not yet a report to a person, and a
report that changes every time it is opened is not a record of what was said. This story
turns the output into something that can be sent and kept.

**Independent Test**: Produce a weekly report, add highlights and next steps, save it,
change something in the workspace, and confirm the saved report is unchanged; export it as
a document and list saved reports.

**Acceptance Scenarios**:

1. **Given** a report, **When** the researcher adds highlights, blockers, next steps, or a
   free comment, **Then** each appears in its own section of the report.
2. **Given** a report, **When** the researcher saves it, **Then** its entire content is kept
   exactly as it was at that moment, with the date it was saved and who saved it.
3. **Given** a saved report, **When** records it mentions are later changed or deleted,
   **Then** the saved report is unchanged.
4. **Given** saved reports, **When** the researcher lists them, **Then** they are shown with
   their period, scope, audience, and date saved, and can be filtered by any of these.
5. **Given** a saved report, **When** the researcher views it, **Then** it is shown exactly
   as saved and marked as a saved copy.
6. **Given** a report, saved or not, **When** the researcher exports it, **Then** a document
   is produced that can be sent or printed, and the same content can be produced in a form
   meant for other programs.
7. **Given** a saved report, **When** the researcher changes its narrative, **Then** a new
   revision is saved and the earlier one is kept.
8. **Given** a saved report for a period, **When** the researcher produces a fresh report
   for the same period and scope, **Then** the tool mentions that a saved one exists and
   can show what differs.
9. **Given** a period that has not ended, **When** the researcher saves its report, **Then**
   the saved copy is marked as partial.
10. **Given** a saved report, **When** the researcher deletes it, **Then** the tool asks for
    confirmation.
11. **Given** an export destination that already exists or cannot be written, **When** the
    researcher exports, **Then** the tool refuses to overwrite without confirmation, or
    explains why it cannot write.

---

### User Story 5 - Compare periods and see trends (Priority: P5)

A researcher sees how a period compares with the one before it — more references read,
fewer tasks completed — and looks at a series of periods side by side, for example the last
twelve weeks, to see how their work is trending.

**Why this priority**: Comparison turns a report into feedback. It needs nothing new except
the reports themselves, and is valuable only once they are in regular use.

**Independent Test**: With activity in two consecutive weeks, produce the weekly report
with comparison and confirm the differences are correct; produce a series of the last four
weeks and confirm each column matches that week's own report.

**Acceptance Scenarios**:

1. **Given** a report, **When** the researcher asks for comparison, **Then** each count in
   the summary is shown beside the count for the preceding period of the same length, with
   the difference.
2. **Given** a current period that has not ended, **When** it is compared, **Then** the
   comparison is with the same portion of the preceding period, and says so.
3. **Given** two periods named by the researcher, **When** they ask to compare them,
   **Then** their summaries are shown side by side.
4. **Given** a period length and a number, **When** the researcher asks for a series,
   **Then** one column per period is shown, for the chosen counts, oldest first.
5. **Given** a series, **When** it is shown, **Then** each column matches exactly what that
   period's own report shows.
6. **Given** a preceding period with no activity, **When** it is compared, **Then** the
   difference is shown as a count, not as a percentage of zero.
7. **Given** a series longer than the workspace's life, **When** it is shown, **Then**
   periods before the workspace existed are marked as not applicable rather than zero.

---

### User Story 6 - Reuse report definitions (Priority: P6)

A researcher who sends the same report every week saves its definition once — weekly,
this project, supervisor audience, these sections, this export form — under a name, and
from then on produces it with that name alone. They can also set which definition is used
when a report is requested with no options at all.

**Why this priority**: It removes repetition for people who already report regularly. It
depends on every earlier story and adds only convenience.

**Independent Test**: Define a report named "supervisor-weekly", produce it by name on two
different weeks, and confirm each covers its own week with the defined scope, audience, and
sections.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher defines a named report with a period
   length, a scope, an audience, sections, a level of detail, and an export form, **Then**
   the definition is stored.
2. **Given** a named report, **When** the researcher produces it, **Then** it covers the
   current period of its length, with everything else as defined.
3. **Given** a named report, **When** the researcher produces it with an option given
   explicitly, **Then** that option overrides the definition for this one time.
4. **Given** named reports, **When** the researcher lists them, **Then** each is shown with
   its definition.
5. **Given** a named report, **When** the researcher sets it as the default, **Then** a
   report requested with no options uses it.
6. **Given** a named report whose project no longer exists, **When** it is produced,
   **Then** the tool says so and produces nothing.
7. **Given** a name already in use, or a definition with an invalid value, **When** the
   researcher saves it, **Then** the tool rejects it and explains.
8. **Given** a named report with narrative prompts defined (for example "Blockers" and
   "Help needed"), **When** it is produced, **Then** those prompts appear as empty sections
   for the researcher to fill.

---

### Edge Cases

- A day has 23 or 25 hours because clocks changed: the daily report covers the whole
  calendar day, whatever its length.
- Activity happens just before midnight: it belongs to the day on which it happened in the
  researcher's local time, and to that day's week and month.
- The researcher travels and their local time changes: each event keeps the moment it
  happened; reports use the local time in effect when the report is produced, and say which.
- A week spans two months or two years: it appears whole in the weekly report, and each of
  its days is counted in its own month and year.
- The first day of the week or the first month of the year is changed: later reports use the
  new setting; saved reports keep the dates they were saved with.
- A period name is ambiguous ("bimonthly"): the tool always means every two months, and the
  report header states the exact dates.
- A record is changed many times in a period: at summary and standard detail it is counted
  once as "updated"; full detail lists every change.
- A record is created and deleted within the same period: both events appear; it is not
  counted among records existing at the end.
- A record moves from one project to another during the period: its activity is reported
  under the project it belonged to when each event happened.
- A task is completed, reopened, and completed again: the report shows it completed once,
  with the latest date, and full detail shows the history.
- A run started in one period and ended in the next: it is reported as started in the first
  and as finished in the second.
- A very long or very active period would produce thousands of items: the report summarizes,
  states how many items were not listed, and full detail remains available on request.
- A report is requested for a period entirely in the future: the tool refuses, and offers
  the view of what is due instead.
- A report is requested for a period entirely before the workspace existed: the tool says
  there is nothing to report and gives the workspace's first day.
- The record of activity has been found altered (the audit check fails): the report is still
  produced and carries a visible warning that its source could not be verified.
- An area of work is not in use at all (no experiments have ever been recorded): its section
  never appears and it is not listed as "quiet".
- Two saved reports cover overlapping periods: both are kept; neither replaces the other.
- A report is exported to a place that is not writable, or the disk is full: nothing partial
  is left behind, and the tool explains.

## Requirements *(mandatory)*

### Functional Requirements

#### Periods

- **FR-001**: Users MUST be able to request a report for these periods: daily, weekly,
  monthly, bimonthly (two months), quarterly, semiannual (six months), annual, and custom.
- **FR-002**: For every period other than custom, users MUST be able to choose the current
  period (to now), the previous complete period, the period containing a given date, or a
  period named by its number and year.
- **FR-003**: A custom period MUST accept a start date and an end date (both included), a
  start date alone (to now), or a span of the last N days, weeks, or months.
- **FR-004**: Weeks MUST begin on a configurable first day (Monday by default). Bimonthly,
  quarterly, semiannual, and annual periods MUST be counted from a configurable first month
  of the year (January by default).
- **FR-005**: The system MUST accept "semester" and "half-year" as other names for
  semiannual, and MUST treat "bimonthly" as every two months in all cases.
- **FR-006**: Every report MUST state the exact first and last date it covers, the local
  time zone used, the moment it was produced, and whether the period is complete or partial.
- **FR-007**: The system MUST reject a period whose end is before its start, that lies
  entirely in the future, or whose span is not a positive number, and MUST limit a period
  that begins before the workspace existed to the workspace's first day, saying so.
- **FR-008**: Period boundaries MUST follow calendar days in the researcher's local time,
  including on days when clocks change.

#### Content

- **FR-009**: A report MUST be built from the workspace's record of activity, so that every
  creation, change, deletion, status change, run, and confirmation in the period is
  accounted for, including those concerning records that no longer exist.
- **FR-010**: A report MUST begin with a summary of counts for the period and MUST then
  present activity in sections by area of work, each shown only when it has activity:
  - **Literature**: references added, read, and annotated; reviews advanced; candidates
    screened.
  - **Writing**: drafts created, stage changes, versions recorded, submissions and
    decisions.
  - **Inquiry**: research questions and hypotheses added and changes of their status.
  - **Experiments**: experiments created and concluded; runs started, succeeded, failed,
    and waiting; results, figures, and tables recorded.
  - **Data and methods**: datasets registered and new versions; methodologies added;
    environment snapshots captured.
  - **Planning**: milestones reached, missed, and moved; tasks created, completed, and
    overdue.
  - **Other**: courses, staff, and any other record type with activity.
- **FR-011**: A report MUST end with what needs attention at the moment it is produced
  (overdue tasks and milestones, runs waiting for a person, drafts reporting replaced
  results) and what is due in the next period of the same length.
- **FR-012**: The summary MUST name the areas of work that are in use but had no activity in
  the period; areas never used MUST NOT be mentioned.
- **FR-013**: Every item listed MUST state what happened, the record concerned by name and
  identifier, and the date; a record changed several times MUST be counted once per kind of
  event below full detail.
- **FR-014**: Reports MUST offer three levels of detail — summary (counts only), standard
  (counts and significant items), and full (every item) — with standard as the default up
  to one month and summary by month as the default for longer periods.
- **FR-015**: When items are left out for length, the report MUST say how many.
- **FR-016**: A report for a period with no activity MUST say so and MUST still include what
  needs attention and what is due next.
- **FR-017**: When the record of activity fails its integrity check, the report MUST still
  be produced and MUST carry a visible warning.

#### Scope and audience

- **FR-018**: Users MUST be able to scope a report to the whole workspace or to one or more
  projects; with a current project and no scope given, the report MUST cover the current
  project and say so.
- **FR-019**: A report over the whole workspace MUST group activity by project, with records
  belonging to no project grouped apart; activity MUST be attributed to the project a
  record belonged to when the event happened.
- **FR-020**: Users MUST be able to include or exclude sections by name, and to filter a
  report by tag and by responsible person.
- **FR-021**: The system MUST provide three audiences that set the sections and tone of a
  report: personal (everything), supervisor (progress against milestones, completed work,
  work in progress, blockers, plans), and funder (outputs, milestones, deviations from
  plan).
- **FR-022**: Users MUST be able to mark any note or record as private. Reports for any
  audience other than personal MUST leave out everything marked private and every record
  marked sensitive, and MUST state how many items were withheld.
- **FR-023**: The system MUST reject an unknown section, audience, project, tag, or person,
  and list the valid choices.

#### Narrative, saving, and export

- **FR-024**: Users MUST be able to add narrative to a report: highlights, blockers, next
  steps, and free comments, each shown in its own section.
- **FR-025**: Users MUST be able to save a report; a saved report MUST keep its entire
  content exactly as it was when saved, with the date, the author, and whether the period
  was partial, and MUST NOT change when the workspace changes afterwards.
- **FR-026**: Users MUST be able to list saved reports filtered by period, scope, audience,
  and date, view one exactly as saved, and delete one after confirmation.
- **FR-027**: Changing the narrative of a saved report MUST save a new revision and keep the
  earlier ones.
- **FR-028**: When a saved report exists for the same period and scope, a freshly produced
  report MUST say so, and users MUST be able to see what differs between the two.
- **FR-029**: Users MUST be able to export any report, saved or not, as a document suitable
  for sending and printing, as plain text, and in a structured form meant for other
  programs; all three MUST carry the same content.
- **FR-030**: An export MUST NOT overwrite an existing file without confirmation and MUST
  NOT leave a partial file behind when it fails.

#### Comparison and trends

- **FR-031**: Users MUST be able to show, beside each summary count, the count for the
  preceding period of the same length and the difference; a partial period MUST be compared
  with the same portion of the preceding one, and the report MUST say so.
- **FR-032**: Users MUST be able to compare any two periods they name, side by side.
- **FR-033**: Users MUST be able to produce a series of consecutive periods of one length
  for chosen counts, oldest first; each value MUST equal what that period's own report
  shows.
- **FR-034**: Differences MUST be shown as counts and, only when the earlier count is not
  zero, as percentages; periods before the workspace existed MUST be shown as not
  applicable.

#### Report definitions

- **FR-035**: Users MUST be able to create, list, view, update, and delete named report
  definitions, each holding a period length, scope, audience, sections, level of detail,
  filters, narrative prompts, and export form.
- **FR-036**: Users MUST be able to produce a report by the name of its definition, and to
  override any part of the definition for one production.
- **FR-037**: Users MUST be able to set one definition as the default used when a report is
  requested with no options; with no default set, a report with no options MUST be the
  weekly report of the current scope at standard detail.
- **FR-038**: The system MUST reject a definition whose name is already in use or whose
  values are invalid, and MUST refuse to produce a definition that refers to something that
  no longer exists, naming it.

#### Common behavior

- **FR-039**: Every value a user supplies for a report — dates, spans, names of periods,
  sections, audiences, scopes, and definitions — MUST be validated before anything is
  produced or stored; invalid input MUST be reported per value, all together, with what is
  expected and a valid example.
- **FR-040**: Producing a report MUST NOT change any record of the workspace; saving a
  report, changing its narrative, and deleting it MUST be recorded in the audit trail.
- **FR-041**: Producing a report MUST NOT appear as activity in later reports.
- **FR-042**: Reports MUST be produced without a network connection and MUST NOT send any
  content anywhere; sharing is done by the researcher with the exported document.
- **FR-043**: The settings for the first day of the week, the first month of the year, the
  default level of detail, and the default audience MUST be configurable per workspace.
- **FR-044**: Every report command MUST have built-in help, and reports MUST have a usage
  guide with an example for every period.

### Key Entities *(include if feature involves data)*

- **Report**: An account of the activity in a scope during a period, for an audience, at a
  level of detail. Produced on demand from the record of activity; it is not stored unless
  saved.
- **Period**: The span of time a report covers: a kind (daily, weekly, monthly, bimonthly,
  quarterly, semiannual, annual, custom), an exact first and last date, and whether it is
  complete or partial.
- **Scope**: What a report covers: the whole workspace or named projects, optionally
  narrowed by tags and responsible people.
- **Audience**: Who a report is for — personal, supervisor, or funder — which decides its
  sections and what is withheld.
- **Section**: One part of a report covering an area of work, the summary, attention, what
  is due next, or a narrative heading.
- **Activity Item**: One thing that happened to one record at one moment, as shown in a
  report.
- **Narrative**: The researcher's own words attached to a report: highlights, blockers,
  next steps, comments.
- **Saved Report**: A permanent copy of a report as it was at a moment, with its revisions.
- **Report Definition**: A named, reusable description of a report: period length, scope,
  audience, sections, detail, filters, narrative prompts, and export form.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can produce the report for today, this week, or this month with a
  single short command and no options, and see it in under 3 seconds.
- **SC-002**: 100% of creations, changes, deletions, status changes, and runs that happened
  in a period appear in that period's full-detail report, and none from outside it does.
- **SC-003**: For every period kind, the first and last dates shown in a report match the
  calendar definition of that period in 100% of cases, including across changes of clock,
  month ends, leap days, and year ends.
- **SC-004**: An annual report for a workspace with 100,000 recorded events is produced in
  under 10 seconds.
- **SC-005**: A user who reports weekly to a supervisor can go from nothing to an exported
  document, including writing three lines of narrative, in under 5 minutes; with a saved
  definition, producing the document takes under 30 seconds of their own time.
- **SC-006**: A saved report is identical, character for character, whenever it is viewed,
  whatever has changed in the workspace since.
- **SC-007**: A report scoped to one project contains 0 items from any other project.
- **SC-008**: A report for the supervisor or funder audience contains 0 items marked private
  or sensitive.
- **SC-009**: In a series, every value equals the corresponding value in that period's own
  report in 100% of cases.
- **SC-010**: At least 80% of users who send a report to a supervisor send it without
  editing the exported document outside the tool.
- **SC-011**: 100% of invalid period requests are rejected with a message naming the invalid
  value and a valid example.
- **SC-012**: A first-time user can produce a custom-date report on their first attempt
  using only the built-in help in at least 90% of cases.

## Assumptions

- **"Semiannual" is the name chosen for "half-year".** "Semester" and "half-year" are
  accepted as other words for it. The name can be changed without changing behavior.
- **"Bimonthly" means every two months.** The word also means twice a month in everyday
  use; this specification never uses it that way, and every report states its exact dates.
- **Quarterly and annual were added** because institutions and funders commonly ask for
  them and they follow the same rules; they can be dropped without affecting the others.
- **Reports are read from the audit trail** specified in `specs/001-research-workspace`,
  which records every change with its time. A report therefore covers exactly what the
  workspace recorded; work done outside the tool appears only if the researcher writes it
  into the narrative or the lab notebook.
- **Sections follow the other specifications**: literature (`specs/005-literature`),
  experiments (`specs/004-experiments`), projects, milestones, and tasks
  (`specs/003-research-projects`), and the rest (`specs/001-research-workspace`,
  `specs/002-research-lifecycle`). A section exists only once the records it reports on
  exist; reports work with whichever of those are available.
- **This specification supersedes the per-project progress report** of
  `specs/003-research-projects` (its FR-044): that report is the supervisor-audience report
  scoped to one project. The "what is due" view of that specification is reused here, not
  redefined.
- **No time tracking.** Reports say what happened and when, not how many hours were spent.
- **No scheduling or delivery.** The tool produces a report when asked; it does not run by
  itself at the end of a period, send messages, or remind anyone. The researcher sends the
  exported document.
- **Privacy markings**: "private" is introduced by this specification and can be set on any
  note or record; "sensitive" is the marking on datasets defined in
  `specs/002-research-lifecycle`. Unmarked items are never withheld.
- **Significant events** for longer reports are: milestones reached or missed, drafts
  changing stage, submissions and decisions, experiments concluded, hypotheses changing
  status, reviews completed, and datasets or software published.
- **Dates use the researcher's local time zone** at the moment a report is produced; a
  report states the zone used.
- **Single researcher.** A report describes the workspace's activity; attributing activity
  to several collaborators arrives with collaboration in `specs/002-research-lifecycle`.
- **The exported document is plain and portable**, readable anywhere and convertible by the
  researcher's own tools; producing typeset or branded documents is out of scope.
