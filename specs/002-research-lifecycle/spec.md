<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Research Lifecycle Extensions

**Feature Branch**: `002-research-lifecycle`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "Capabilities missing from the TRCLI research workspace: reading notes and annotations; tasks and deadlines; backup, export and migration; submissions and peer review; funding and grants; ethics and compliance; lab notebook; code and software; parameters and metrics for runs; pre-registration; collaboration; publishing outputs; venues, conferences and talks; instruments, samples and materials; glossary and concepts; integrations; dashboard. (Research questions and hypotheses, results/figures/tables, and online metadata lookup were added to specs/001-research-workspace instead.)"

## Overview

The research workspace specified in `specs/001-research-workspace` covers the path from
literature to results. This specification adds what surrounds that path in a real project:
the day-to-day working records (reading annotations, tasks, a lab notebook), the rigor around
experiments (parameters and metrics, code versions, pre-registration), the obligations of a
project (peer review, ethics, funding), the safety of the workspace itself (backup, restore,
upgrade), and the ways the work reaches other people (venues and talks, publishing outputs,
integrations, collaboration).

Every record type added here behaves like those of the base workspace: it can be created,
listed, viewed, updated, deleted, tagged, linked to other records, and is audited. The
stories are ordered in three tiers — needed early, needed for real projects, and later —
and each is usable on its own once the base workspace it builds on exists.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Annotate papers while reading (Priority: P1)

> **Moved (2026-10-08)**: reading annotations and structured summaries are now specified in
> `specs/005-literature` (its User Story 4). This story is kept as a pointer so that the
> numbering of the other stories does not change.

---

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

### User Story 3 - Back up, restore, move, and upgrade a workspace (Priority: P3)

A researcher makes a complete backup of a workspace as a single file, restores it on the
same or another machine, exports the whole workspace in an open, documented form that can be
read without the tool, and, when a new version of the tool changes how workspaces are kept,
upgrades the workspace safely.

**Why this priority**: A workspace holds years of work. Researchers will not trust it with
that work until they can get everything out, put it back, and survive an upgrade.

**Independent Test**: Back up a populated workspace, restore it to an empty location, and
confirm that every record, link, and audit entry is identical; then upgrade a workspace made
by an older version and confirm nothing is lost.

**Acceptance Scenarios**:

1. **Given** a populated workspace, **When** the researcher requests a backup, **Then** a
   single file is produced containing every record, link, and audit entry, and its
   completeness is verified before success is reported.
2. **Given** a backup file, **When** the researcher restores it to an empty location,
   **Then** the restored workspace is identical to the original at the time of backup.
3. **Given** a backup file and a location that already holds a workspace, **When** the
   researcher restores, **Then** the tool refuses unless the researcher explicitly chooses
   to replace it.
4. **Given** a damaged or incomplete backup file, **When** the researcher restores it,
   **Then** the tool reports the damage and changes nothing.
5. **Given** a workspace, **When** the researcher requests a full export, **Then** the
   records are written in an open, documented form that can be read without the tool.
6. **Given** a workspace created by an older version, **When** a newer version opens it,
   **Then** the tool says an upgrade is needed, changes nothing until asked, and takes a
   backup before upgrading.
7. **Given** an upgrade that fails part-way, **When** the failure occurs, **Then** the
   workspace is returned to its state before the upgrade.
8. **Given** a workspace created by a newer version than the tool in use, **When** the tool
   opens it, **Then** it refuses to modify it and explains why.

---

### User Story 4 - Keep a lab notebook (Priority: P4)

A researcher keeps a dated journal of what they did, observed, and decided. Entries are
added, never rewritten: a mistake is fixed by adding a correction that points to the earlier
entry. Entries can link to any record, and the notebook can be read by day, by record, or
exported for a period.

**Why this priority**: The notebook is the researcher's own narrative of the work and is
often a formal requirement. It differs from the audit trail, which records what the tool
did, not what the researcher thought or saw.

**Independent Test**: Write three entries on different days, link one to an experiment
run, correct one, and export the notebook for the period showing the original and the
correction.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher adds a notebook entry, **Then** it is
   stored with the date, time, and author.
2. **Given** an entry, **When** the researcher tries to change or delete it, **Then** the
   tool refuses and offers to add a correction instead.
3. **Given** an entry, **When** the researcher adds a correction, **Then** both are kept and
   the original is shown as corrected.
4. **Given** an entry, **When** the researcher links it to records, **Then** each of those
   records lists the entry.
5. **Given** a notebook, **When** the researcher reads it for a date range, a record, or a
   word, **Then** matching entries are shown in time order.
6. **Given** a notebook, **When** the researcher exports a period, **Then** a document is
   produced with every entry and correction in order.
7. **Given** a notebook, **When** an entry is altered outside the tool, **Then** an integrity
   check reports it.

---

### User Story 5 - Record run parameters and metrics, sweep, and compare (Priority: P5)

> **Moved (2026-10-08)**: parameters, metrics, sweeps, and run comparison are now specified
> in `specs/004-experiments` (its User Story 4). This story is kept as a pointer so that
> the numbering of the other stories does not change.

---

### User Story 6 - Record the code and software behind a run (Priority: P6)

> **Moved (2026-10-08)**: code and software versions are now specified in
> `specs/004-experiments` (its User Story 6). This story is kept as a pointer so that the
> numbering of the other stories does not change.

---

### User Story 7 - Pre-register a hypothesis and analysis plan (Priority: P7)

Before running an experiment, a researcher freezes the hypothesis, the planned analysis, the
planned sample size, and the criteria for stopping, with a timestamp. Afterwards, anyone can
see what was planned, what was actually done, and every deviation between the two.

**Why this priority**: Pre-registration is how researchers show that the analysis was not
chosen after seeing the data. It depends on hypotheses, experiments, and runs existing.

**Independent Test**: Pre-register a plan for an experiment, try to change it, run the
experiment with a different sample size, and produce a report showing the deviation.

**Acceptance Scenarios**:

1. **Given** a hypothesis and an experiment, **When** the researcher writes a
   pre-registration with the analysis plan, **Then** it is stored as a draft that can still
   be edited.
2. **Given** a draft pre-registration, **When** the researcher freezes it, **Then** its
   content and the time are sealed and it can no longer be changed.
3. **Given** a frozen pre-registration, **When** the researcher needs to change the plan,
   **Then** they add a dated amendment with a reason; the original remains visible.
4. **Given** a frozen pre-registration, **When** a run started before the freeze time is
   linked to it, **Then** the tool flags that the run predates the registration.
5. **Given** a pre-registration and finished runs, **When** the researcher requests a
   comparison, **Then** the tool lists every recorded difference between plan and practice.
6. **Given** a frozen pre-registration, **When** it is altered outside the tool, **Then** an
   integrity check reports it.
7. **Given** a frozen pre-registration, **When** the researcher exports it, **Then** a
   document is produced that shows the content, the freeze time, and any amendments.

---

### User Story 8 - Track submissions and peer review (Priority: P8)

A researcher records each time a draft is submitted to a venue: which version was sent,
when, and the decision. For each round they record the reviewers' comments one by one,
write a response to each, track which have been addressed, and produce the response letter.
A rejection followed by submission elsewhere stays part of the same draft's history.

**Why this priority**: A draft's stage says where it is; it does not hold the venues tried,
what reviewers asked, or what was promised in reply — the content of months of work.

**Independent Test**: Submit a draft version to a venue, record a "major revision" decision
with four reviewer comments, respond to each, and generate the response letter.

**Acceptance Scenarios**:

1. **Given** a draft with a version, **When** the researcher records a submission to a venue
   with a date, **Then** it is stored and the draft's stage becomes "submitted".
2. **Given** a submission, **When** the researcher records the decision (accepted, minor
   revision, major revision, rejected, withdrawn) with its date, **Then** it is stored and
   the draft's stage is updated to match.
3. **Given** a decision, **When** the researcher records reviewer comments, each attributed
   to a reviewer label, **Then** each is stored with the status "open".
4. **Given** a comment, **When** the researcher writes a response and links the draft
   version that addresses it, **Then** the comment is shown as addressed.
5. **Given** a review round, **When** the researcher requests the response letter, **Then**
   a document lists every comment with its response, and unanswered comments are flagged.
6. **Given** a rejected submission, **When** the researcher submits the draft to another
   venue, **Then** both submissions appear in the draft's history in order.
7. **Given** a decision dated before its submission, **When** the researcher saves it,
   **Then** the tool rejects it.

---

### User Story 9 - Manage ethics and compliance (Priority: P9)

A researcher records ethics approvals (the body, the reference, the dates of validity, the
conditions), the consent basis under which data was collected, and the project's
data-management plan. Datasets are flagged when they hold personal or sensitive data, and
the tool warns when work touches such data without a valid approval.

**Why this priority**: These are obligations with consequences. They attach to datasets and
experiments that must already exist, and matter most once a project involves people.

**Independent Test**: Record an approval with an expiry date, flag a dataset as personal
data, link them, and see a warning when an experiment uses the dataset after the approval
has expired.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records an ethics approval with its body,
   reference, validity dates, and conditions, **Then** it is stored with the status derived
   from its dates (pending, valid, expiring soon, expired).
2. **Given** a dataset, **When** the researcher flags it as holding personal or sensitive
   data with a sensitivity level, **Then** the flag is shown wherever the dataset appears.
3. **Given** a flagged dataset, **When** the researcher records its consent basis and links
   an approval, **Then** both are shown with the dataset.
4. **Given** a flagged dataset with no valid approval, **When** an experiment using it is
   run, **Then** the tool warns and requires explicit acknowledgement, which is recorded.
5. **Given** approvals nearing expiry, **When** the researcher asks what is due, **Then**
   the expiring approvals are listed.
6. **Given** a workspace, **When** the researcher records a data-management plan with its
   sections and links datasets to it, **Then** the plan lists the datasets it covers and
   the datasets not covered by any plan can be listed.
7. **Given** a flagged dataset, **When** a reproducibility package or export that includes
   it is produced, **Then** the tool warns that sensitive data is referenced.

---

### User Story 10 - Manage funding and grants (Priority: P10)

A researcher records funders and grants through their life (idea, in preparation,
submitted, awarded, rejected, active, closed), with amounts, periods, budget lines, and
reporting deadlines, and links each grant to the outputs it paid for, so that a funder
report and an acknowledgement text can be produced.

**Why this priority**: Funding sustains the work and carries reporting duties, but nothing
else in the workspace depends on it.

**Independent Test**: Record a grant with two budget lines and a reporting deadline, link a
draft and a dataset to it, and produce the list of outputs for a reporting period.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records a grant with a title, a funder, an
   amount with its currency, and a period, **Then** it is stored at the stage "idea" unless
   another is given.
2. **Given** a grant, **When** the researcher adds budget lines and records spending against
   them, **Then** the grant shows planned, spent, and remaining amounts per line.
3. **Given** a grant, **When** the researcher adds reporting deadlines, **Then** they appear
   in the "what's due" view.
4. **Given** a grant, **When** the researcher links drafts, datasets, experiments, and staff
   to it, **Then** each shows the grant that supports it.
5. **Given** a grant and a period, **When** the researcher requests an output report,
   **Then** every linked output with activity in that period is listed.
6. **Given** a draft supported by grants, **When** the researcher requests its
   acknowledgement, **Then** a text naming each funder and grant reference is produced.
7. **Given** a negative amount, an unknown currency, or an end date before the start date,
   **When** the researcher saves the grant, **Then** the tool rejects it.

---

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

### User Story 12 - Track venues, conferences, and talks (Priority: P12)

> **Moved (2026-10-08)**: venues, events, calls with their deadlines, and talks are now
> specified in `specs/015-venues`. This story is kept as a pointer so that the numbering of
> the other stories does not change.

---

### User Story 13 - Build a glossary of concepts (Priority: P13)

A researcher records the terms and concepts of their field with definitions, synonyms, and
the papers that define or use them, and links concepts to one another as broader, narrower,
or related.

**Why this priority**: A shared vocabulary helps writing and onboarding, and nothing else
depends on it.

**Independent Test**: Add two concepts, relate one as narrower than the other, link a
defining paper, and look a concept up by a synonym.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher adds a concept with a term and a
   definition, **Then** it is stored.
2. **Given** a concept, **When** the researcher adds synonyms and looks one up, **Then** the
   concept is found.
3. **Given** two concepts, **When** the researcher relates them as broader, narrower, or
   related, **Then** the relation is visible from both.
4. **Given** a concept, **When** the researcher links papers to it, **Then** the concept
   lists them and each paper lists the concept.
5. **Given** a term that already exists, **When** the researcher adds it again, **Then** the
   tool reports the existing concept.

---

### User Story 14 - Track instruments, samples, and materials (Priority: P14)

A researcher doing laboratory or field work records instruments with their calibration
history, samples with their origin and where they are stored, and materials with their lot
and expiry, and records which of them each run used.

**Why this priority**: Essential for bench and field research but irrelevant to much
computational work; it extends runs that must already exist.

**Independent Test**: Register an instrument with a calibration due date, a sample, and a
material lot, record their use in a run, and view the run listing all three.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher registers an instrument with its
   identifier and calibration dates, **Then** it is stored and its next calibration appears
   in the "what's due" view.
2. **Given** a workspace, **When** the researcher registers a sample with its origin,
   collection date, and storage place, **Then** it is stored, and a sample derived from
   another shows its parent.
3. **Given** a workspace, **When** the researcher registers a material with its supplier,
   lot, quantity, and expiry, **Then** it is stored.
4. **Given** a run, **When** the researcher records the instruments, samples, and materials
   used, **Then** the run lists them and each lists the runs it was used in.
5. **Given** an instrument past its calibration date or a material past its expiry, **When**
   it is recorded as used in a run, **Then** the tool warns and records the acknowledgement.

---

### User Story 15 - Publish outputs and record their identifiers (Priority: P15)

A researcher prepares a dataset, a piece of software, or a reproducibility package for
deposit in a public archive, checks it is ready, records the persistent identifier it
receives, and obtains the citation others should use for it.

**Why this priority**: Sharing outputs is increasingly required, but it is the last step in
the life of records that must first exist and be complete.

**Independent Test**: Run a readiness check on a dataset, fix the reported gaps, produce the
deposit bundle, record the identifier received, and export the dataset's citation.

**Acceptance Scenarios**:

1. **Given** an output, **When** the researcher requests a readiness check, **Then** the tool
   lists what is missing for deposit (for example a license, a description, or creators).
2. **Given** an output that passes the check, **When** the researcher requests a deposit
   bundle, **Then** a bundle with the output's description in a widely accepted form is
   produced for upload to an archive.
3. **Given** a deposited output, **When** the researcher records its persistent identifier,
   the archive, and the date, **Then** the output is shown as published.
4. **Given** a published output, **When** the researcher requests its citation, **Then** a
   citation in a chosen style is produced and can be linked to drafts.
5. **Given** an output flagged as sensitive, **When** a deposit bundle is requested, **Then**
   the tool refuses unless the researcher explicitly confirms.

---

### User Story 16 - Connect with other tools and extend the workspace (Priority: P16)

A researcher exchanges records with the tools they already use — reference managers,
writing tools, and computational notebooks — and adds record types of their own with their
own fields, which then behave like built-in ones.

**Why this priority**: No tool is used alone, but connections are only worth building once
the records they carry are stable.

**Independent Test**: Keep a bibliography file used by a writing tool in step with a draft's
citations, and define a custom record type with two fields and create a record of it.

**Acceptance Scenarios**:

1. **Given** a draft, **When** the researcher asks the tool to keep a bibliography file in
   step with the draft's citations, **Then** the file is updated whenever those citations
   change.
2. **Given** a reference manager's library export, **When** the researcher synchronizes it
   repeatedly, **Then** new and changed entries are brought in without creating duplicates.
3. **Given** a computational notebook, **When** it is registered as a pipeline step or a
   source of results, **Then** runs record which version of the notebook was used.
4. **Given** a workspace, **When** the researcher defines a custom record type with named
   fields and their kinds, **Then** records of that type can be created, listed, viewed,
   updated, deleted, linked, and are validated and audited like built-in ones.
5. **Given** a custom record type with existing records, **When** the researcher changes its
   fields, **Then** the tool shows the effect on existing records and requires confirmation.
6. **Given** an extension from someone else, **When** the researcher adds it, **Then** the
   tool shows what it adds and what it can access before it is turned on.

---

### User Story 17 - Collaborate on a shared workspace (Priority: P17)

Several researchers work on the same workspace, each on their own machine. They exchange
changes, see who changed what, resolve conflicting edits explicitly, and the workspace owner
decides who may read, edit, or administer it.

**Why this priority**: Collaboration changes a founding assumption of the base workspace —
one researcher, one machine — and touches every record type. It is the most valuable later
addition and the most costly, so it comes last and builds on everything else.

**Independent Test**: Two members change different records and exchange changes, ending
with identical workspaces; then both change the same field, and the conflict is shown and
resolved by choice, with nothing lost silently.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** its owner invites a member with a role (reader, editor,
   administrator), **Then** the member can obtain a copy of the workspace.
2. **Given** two members with changes to different records, **When** they exchange changes,
   **Then** both workspaces end up identical and each change is attributed to its author.
3. **Given** two members who changed the same field of the same record, **When** they
   exchange changes, **Then** the conflict is shown with both values and their authors, and
   nothing is overwritten until one is chosen.
4. **Given** a member with the reader role, **When** they try to change a record, **Then**
   the change is refused.
5. **Given** a member whose access is removed, **When** they try to exchange changes,
   **Then** the exchange is refused.
6. **Given** a shared workspace, **When** any member views the audit trail, **Then** entries
   from all members appear in one ordered history.
7. **Given** a member working without a connection, **When** they reconnect and exchange
   changes, **Then** their offline work is merged under the same rules.
8. **Given** append-only records (notebook entries, audit entries, frozen
   pre-registrations), **When** members exchange changes, **Then** entries from all members
   are kept and none is altered.

---

### Edge Cases

- A task is attached to a record that is later deleted: the task is kept, shown as detached,
  and names what it used to belong to.
- Tasks depend on each other in a loop: the tool rejects the dependency.
- A backup is requested while a run is in progress: the backup records the run as in
  progress and says so.
- A backup or export would exceed the available space: the tool stops before writing
  anything and reports how much is needed.
- A restore is attempted from a backup made by a newer version of the tool: the tool
  refuses and explains.
- A notebook entry is written with a past date: it is accepted, and both the stated date and
  the time it was actually written are kept.
- A pre-registration is frozen with required sections empty: the tool refuses.
- An approval's dates change after a run acknowledged its absence: the acknowledgement
  remains in the record.
- A grant's spending exceeds a budget line: it is accepted and flagged as overspent.
- Amounts in different currencies are totalled: totals are shown per currency and never
  added across currencies.
- An extension or custom record type uses a name that a built-in one already uses: the tool
  rejects it.
- Two members delete and edit the same record: this is a conflict, shown like any other.
- Members' clocks disagree: the order of changes does not depend on the clocks alone, and
  the tool reports when a clock is clearly wrong.

## Requirements *(mandatory)*

### Functional Requirements

#### Common behavior

- **FR-001**: Every record type introduced here MUST support create, list, view, update,
  delete, tags, notes, links to other records visible from both ends, and audit entries,
  exactly as the base workspace's record types do, except where a requirement below makes a
  record append-only or frozen.
- **FR-002**: Every value a user supplies MUST be validated before anything is stored or
  acted upon; invalid input MUST change nothing and MUST be reported per value with what is
  expected.
- **FR-003**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-004**: Every command MUST have built-in help, and every feature MUST have a usage
  guide with examples.

#### Reading annotations

*FR-005 to FR-008 moved to `specs/005-literature` (FR-032 to FR-035 there).*

#### Tasks, milestones, and deadlines

*FR-009 to FR-011 are superseded by `specs/003-research-projects`; FR-012 remains.*

- **FR-009**: Users MUST be able to record tasks with a title, description, status (to do,
  in progress, done, cancelled), due date, priority, assignee, and the record they belong
  to.
- **FR-010**: Users MUST be able to record milestones with a date and attach tasks to them,
  and see each milestone's progress.
- **FR-011**: Users MUST be able to make a task depend on other tasks; the system MUST show
  blocked tasks and MUST reject dependency loops.
- **FR-012**: The system MUST provide one view of everything due in a chosen period —
  tasks, milestones, and every dated obligation held by any other record type — in date
  order, naming each item's source and marking overdue items.

#### Backup, restore, export, and upgrade

- **FR-013**: Users MUST be able to produce a complete backup of a workspace as a single
  file, and the system MUST verify the backup before reporting success.
- **FR-014**: Users MUST be able to restore a backup to an empty location and obtain a
  workspace identical to the original; restoring over an existing workspace MUST require
  explicit choice.
- **FR-015**: The system MUST detect a damaged or incomplete backup and MUST change nothing
  when restoring from one.
- **FR-016**: Users MUST be able to export an entire workspace in an open, documented form
  that can be read without the tool.
- **FR-017**: The system MUST record the format version of every workspace, MUST detect
  when an upgrade is needed, MUST NOT upgrade without being asked, and MUST take a backup
  before upgrading.
- **FR-018**: A failed upgrade MUST leave the workspace as it was before the upgrade.
- **FR-019**: The system MUST refuse to modify a workspace whose format is newer than the
  tool understands.

#### Lab notebook

- **FR-020**: Users MUST be able to add notebook entries, each stored with its author, the
  date it refers to, and the time it was written.
- **FR-021**: Notebook entries MUST NOT be changed or deleted; users MUST be able to add
  corrections that refer to an earlier entry, and both MUST remain visible.
- **FR-022**: Users MUST be able to link entries to any record, read the notebook by date
  range, record, or text, and export a period as a document.
- **FR-023**: The system MUST be able to detect that the notebook has been altered outside
  the tool.

#### Run parameters and metrics

*FR-024 to FR-029 moved to `specs/004-experiments` (FR-030 to FR-036 there).*

#### Code and software

*FR-030 to FR-032 moved to `specs/004-experiments` (FR-045 to FR-047 there).*

#### Pre-registration

- **FR-033**: Users MUST be able to write a pre-registration for a hypothesis and experiment
  covering the planned analysis, the planned sample size, the stopping criteria, and the
  planned outcomes to measure.
- **FR-034**: Users MUST be able to freeze a pre-registration; freezing MUST record the time
  and MUST make the content unchangeable, and MUST be refused when required sections are
  empty.
- **FR-035**: Changes to a frozen pre-registration MUST be possible only as dated amendments
  with a reason, with the original kept visible.
- **FR-036**: The system MUST flag linked runs that started before the freeze time, and MUST
  produce a comparison of what was planned against what was recorded as done.
- **FR-037**: The system MUST be able to detect that a frozen pre-registration has been
  altered outside the tool, and users MUST be able to export it with its freeze time and
  amendments.

#### Submissions and peer review

- **FR-038**: Users MUST be able to record submissions of a draft version to a venue with
  dates and a decision (accepted, minor revision, major revision, rejected, withdrawn), and
  the draft's stage MUST follow.
- **FR-039**: Users MUST be able to record reviewer comments per review round, each
  attributed to a reviewer label, with a response, a status (open, addressed, declined), and
  the draft version that addresses it.
- **FR-040**: The system MUST produce a response letter for a round listing every comment
  and response and flagging comments without one.
- **FR-041**: A draft MUST keep the full ordered history of its submissions across venues.

#### Ethics and compliance

- **FR-042**: Users MUST be able to record ethics approvals with body, reference, validity
  dates, and conditions; the system MUST derive each approval's status from its dates.
- **FR-043**: Users MUST be able to flag datasets as holding personal or sensitive data with
  a sensitivity level and a consent basis, and link them to approvals.
- **FR-044**: The system MUST warn, and require a recorded acknowledgement, when a run uses
  a flagged dataset without a valid linked approval, and when a package, export, or deposit
  bundle references a flagged dataset.
- **FR-045**: Users MUST be able to record a data-management plan, link datasets to it, and
  list datasets covered by no plan.
- **FR-046**: Approvals nearing expiry MUST appear in the view of what is due.

#### Funding and grants

- **FR-047**: Users MUST be able to record funders and grants with title, reference, amount
  and currency, period, and stage (idea, in preparation, submitted, awarded, rejected,
  active, closed).
- **FR-048**: Users MUST be able to record budget lines and spending against them, and see
  planned, spent, and remaining amounts; totals MUST NOT be added across currencies.
- **FR-049**: Users MUST be able to record reporting deadlines for a grant, which MUST
  appear in the view of what is due.
- **FR-050**: Users MUST be able to link grants to drafts, datasets, experiments, software,
  talks, and staff, produce the list of outputs for a period, and produce an acknowledgement
  text for a draft.

#### Status view

- **FR-051**: The system MUST provide a single status view of the workspace covering what is
  due and overdue, runs waiting or failed, drafts by stage, open research questions, reading
  backlog, expiring approvals, and recent activity, with access to the detail behind each
  section.

#### Venues and talks

*FR-052 and FR-053 moved to `specs/015-venues`.*

#### Glossary

- **FR-054**: Users MUST be able to record concepts with a term, definition, and synonyms,
  relate them as broader, narrower, or related, link them to papers, and find them by term
  or synonym; duplicate terms MUST be reported.

#### Instruments, samples, and materials

- **FR-055**: Users MUST be able to record instruments with calibration history and next
  calibration date, samples with origin, collection date, storage place, and parent sample,
  and materials with supplier, lot, quantity, and expiry.
- **FR-056**: Users MUST be able to record which instruments, samples, and materials a run
  used; the system MUST warn, and record the acknowledgement, when an instrument is past
  calibration or a material is past expiry.

#### Publishing outputs

- **FR-057**: Users MUST be able to check whether a dataset, piece of software, or
  reproducibility package is ready for deposit and see what is missing.
- **FR-058**: Users MUST be able to produce a deposit bundle with the output's description
  in a widely accepted form, and record the persistent identifier, archive, and date once
  deposited.
- **FR-059**: Users MUST be able to obtain a citation for a published dataset or piece of
  software and link it to drafts.

#### Integrations and extension

- **FR-060**: Users MUST be able to keep a bibliography file in step with a draft's
  citations, and synchronize repeatedly with a reference manager's library without creating
  duplicates.
- **FR-061**: Users MUST be able to register a computational notebook as a pipeline step or
  as a source of results, with its version recorded per run.
- **FR-062**: Users MUST be able to define custom record types with named, typed fields;
  records of those types MUST be validated, linked, and audited like built-in ones.
- **FR-063**: Users MUST be able to add extensions made by others; the system MUST show what
  an extension adds and what it can access before it is turned on, and MUST allow it to be
  turned off.

#### Collaboration

- **FR-064**: A workspace owner MUST be able to add and remove members and give each a role:
  reader, editor, or administrator.
- **FR-065**: Members MUST be able to exchange changes so that their workspaces converge to
  the same content, including after working without a connection.
- **FR-066**: Every change MUST be attributed to the member who made it.
- **FR-067**: Conflicting changes MUST be shown with both values and their authors, and MUST
  NOT be resolved without a member's explicit choice; no change may be lost silently.
- **FR-068**: The system MUST enforce roles: readers MUST NOT change records, and removed
  members MUST NOT be able to exchange changes.
- **FR-069**: Append-only and frozen records MUST keep those properties across members, and
  the audit trail MUST present one ordered history for the whole workspace.

### Key Entities *(include if feature involves data)*

- **Annotation, Reference Summary**: specified in `specs/005-literature`.
- **Task**: A piece of work with status, due date, priority, assignee, dependencies, and an
  optional parent record.
- **Milestone**: A dated goal that groups tasks.
- **Backup**: A complete, verifiable copy of a workspace at a moment in time.
- **Notebook Entry**: An unchangeable dated journal entry, optionally correcting another.
- **Parameter, Sweep, Metric, Software**: specified in `specs/004-experiments`.
- **Pre-registration**: A frozen, timestamped plan for testing a hypothesis, with
  amendments.
- **Submission**: One sending of a draft version to a venue, with its decision.
- **Review Round / Reviewer Comment / Response**: The reviewers' remarks on a submission and
  the researcher's answers.
- **Ethics Approval**: Permission from an ethics body, valid for a period, under conditions.
- **Data-Management Plan**: The project's stated handling of its data, covering datasets.
- **Funder / Grant / Budget Line**: Who pays, for what, how much, and against which lines.
- **Venue, Event, Call, Talk (Presentation)**: specified in `specs/015-venues`.
- **Concept**: A term of the field with definition, synonyms, and relations.
- **Instrument / Sample / Material**: The physical means of an experiment, with calibration,
  provenance, and lot.
- **Deposit**: The publication of an output in an archive, with its persistent identifier.
- **Custom Record Type / Extension**: User-defined or third-party additions to the workspace.
- **Member / Role**: A person with access to a shared workspace and what they may do.
- **Conflict**: Two members' incompatible changes to the same thing, awaiting a choice.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: *Moved to `specs/005-literature` (SC-004 and SC-007).*
- **SC-002**: A user can see everything due in the next 30 days across the whole workspace
  in a single view in under 5 seconds, and 100% of dated obligations from every record type
  appear in it.
- **SC-003**: A workspace of 10,000 papers and 1,000 runs can be backed up in under 2
  minutes, and 100% of restores from a verified backup reproduce the workspace exactly.
- **SC-004**: 100% of upgrades either complete or leave the workspace exactly as it was; no
  upgrade ever results in lost records.
- **SC-005**: A workspace exported in full can be read and understood by a person with no
  access to the tool, using only the export and its documentation.
- **SC-006**: 100% of attempts to alter a notebook entry or a frozen pre-registration,
  through the tool or outside it, are either refused or detected.
- **SC-007**: *Moved to `specs/004-experiments` (SC-006).*
- **SC-008**: *Moved to `specs/004-experiments` (SC-010).*
- **SC-009**: A user can produce a complete response letter for a review round of 30
  comments in under 1 minute once the responses are written, with 100% of unanswered
  comments flagged.
- **SC-010**: 100% of runs that use sensitive data without a valid approval produce a
  warning and a recorded acknowledgement.
- **SC-011**: A user can produce a funder's list of outputs for a reporting period in under
  1 minute.
- **SC-012**: Each section of the status view agrees exactly with the detailed list it
  summarizes.
- **SC-013**: 100% of outputs that pass the readiness check are accepted by the archive they
  were prepared for without metadata corrections.
- **SC-014**: A custom record type with five fields can be defined and its first record
  created in under 5 minutes.
- **SC-015**: After any exchange of changes between members, their workspaces are identical,
  and 0 changes are lost without a member having chosen to discard them.
- **SC-016**: 90% of first-time users complete the primary task of each story on their first
  attempt using only that story's usage guide.

## Assumptions

- **Builds on `specs/001-research-workspace`**: The workspace, papers, citations, drafts,
  experiments, pipelines, runs, results, datasets, methodologies, environment snapshots,
  staff, research questions, hypotheses, audit trail, and telemetry are assumed to exist as
  specified there. Each story names, by its content, the base records it needs.
- **Venues are specified separately**: venues, events, calls, and talks (formerly User Story
  12 here) are in `specs/015-venues`. Submissions and peer review (User Story 8) stay here
  and refer to those venues.
- **Sync and outside services are specified separately**: connecting to a service, syncing
  a workspace between one researcher's machines, remote file storage, and remote backups
  are in `specs/010-integrations`. Collaboration (User Story 17 here) builds on that sync
  and adds members and roles.
- **Literature is specified separately**: reading annotations and structured summaries
  (formerly User Story 1 here) are in `specs/005-literature`.
- **Experiments are specified separately**: run parameters, metrics, sweeps, and code and
  software versions (formerly User Stories 5 and 6 here) are in `specs/004-experiments`.
- **Three tiers, delivered in order**: Stories 1–3 are expected soon after the base
  workspace; stories 4–10 serve real projects; stories 11–17 come later. Each story is
  planned and delivered separately.
- **Collaboration replaces the single-researcher assumption only when it arrives**: Until
  story 17 is delivered, the base workspace's assumption holds — one researcher, one
  machine, no accounts. Story 17 assumes a small team (up to about 20 members), each
  working on their own copy and exchanging changes deliberately; it is not live, simultaneous
  editing. How members are identified and how a shared copy is hosted are decided at
  planning time.
- **Files are referenced, not stored**: As in the base workspace, slides, figures, code,
  notebooks, and data are known by location, version, and fingerprint. Backups cover the
  workspace's records; whether referenced files are included is a choice the researcher
  makes per backup, off by default.
- **Publishing prepares, the researcher deposits**: The tool checks readiness, produces the
  bundle, and records the identifier. Uploading to an archive directly, and obtaining an
  identifier automatically, are out of scope.
- **Funding is tracking, not accounting**: Budget lines and spending are recorded for the
  researcher's overview. Invoices, payroll, and institutional finance systems are out of
  scope.
- **Ethics features support, they do not certify**: Warnings and acknowledgements help the
  researcher meet obligations; the tool does not judge legal compliance, and it does not
  encrypt or anonymize data.
- **Peer review is recorded by the author**: The tool holds the comments the researcher
  received and their responses. It is not a system through which reviewers submit reviews.
- **Pre-registration is sealed locally**: Freezing proves the content has not changed since
  the recorded time within the workspace. Depositing it with a public registry is covered
  by publishing outputs or done by the researcher.
- **Sweeps run on the researcher's own machine**, like other runs; distributing runs across
  remote machines is out of scope.
- **Integrations are with the file formats and conventions researchers already use**; which
  specific tools are supported first is decided at planning time.
- **Extensions are trusted by the researcher who adds them**; the tool informs and asks, but
  it does not vet third-party extensions.
- **Money is recorded in the currency entered**, with no conversion.
- **Interface language is English**, with dates in an unambiguous international form.
