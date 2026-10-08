<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Venues — Places to Publish

**Feature Branch**: `015-venues`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add a new spec for places to publish (events, conferences, journals)"

## Overview

A finished piece of research has to go somewhere, and where it goes matters: for who reads
it, for how it counts, for how long it takes, and for what it costs. Researchers carry this
knowledge in their heads and in bookmarks — which journals fit their topic, which
conference has a deadline in six weeks, which venue took nine months last time, which one
their programme does not count. Every deadline missed and every paper sent to the wrong
place is this knowledge failing.

This specification gives **venues** a place in the workspace: the journals, conferences,
and other places where the researcher publishes or might publish, what is known about each,
their calls and deadlines, the choice of venue for a manuscript, what experience has
taught, and the events attended and talks given.

It **takes over** the venue content of an earlier specification, which now points here:

| Came from | What |
|-----------|------|
| `specs/002-research-lifecycle`, User Story 12 | Venues, calls with deadlines, talks and posters |

### The words

| Term | Meaning in TRCLI |
|------|------------------|
| **Venue** | A place that publishes or presents research, considered as something lasting: a journal, a conference series, a workshop series, a book series, a preprint server. |
| **Event** | One occurrence of a venue that meets: this year's edition of a conference, with its dates and place. A journal has no events. |
| **Call** | An invitation to submit, with its deadlines: a conference's call for papers, a journal's special issue. A journal that always accepts submissions has an open, permanent call. |
| **Talk** | Something the researcher presented at an event: a talk, a poster, a tutorial, a demonstration. |

### The concepts, in one picture

```text
 Venue (journal · conference · workshop · …) ── ranked in ──▶ Ranking schemes
   │  profile: scope, review model, costs, policies, my interest
   │
   ├── has ──▶ Event (edition: year, dates, place) ── has ──▶ Call ── deadlines ──▶ What is due (003)
   │                    │                                       │
   │                    └── I attend / present ──▶ Talk         └── a Manuscript is aimed at it
   │
   ├── candidate for ──▶ Manuscript (008) ── shortlist, comparison, target
   └── history ◀── Submissions and decisions (002) ── what experience taught
```

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Keep a register of venues (Priority: P1)

A researcher records the places where they publish or might: each venue's name, what kind
it is, what it covers, who publishes it, how it reviews, what it costs, what it allows —
and their own view of it: a target, one to watch, or one to avoid. They find venues by
topic, kind, or standing, and share their list with a colleague.

**Why this priority**: The register is what everything else refers to: deadlines belong to
venues, manuscripts are aimed at them, experience accumulates on them. With only this story
a researcher has their own, searchable map of where their field publishes.

**Independent Test**: Record a journal and a conference with their profiles and a ranking
each, mark one as a target and one to avoid with a reason, find venues by a topic word, and
export the list for a colleague.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records a venue with a name and a kind —
   journal, conference, workshop, symposium, book series, preprint server, or other —
   **Then** it is stored.
2. **Given** a venue, **When** the researcher records its short name or acronym, publisher
   or organizer, web address, identifiers, the language it publishes in, and the topics it
   covers, **Then** these are saved and shown.
3. **Given** a venue, **When** the researcher records how it reviews — whether reviewers
   know the authors, whether authors know the reviewers, whether reviews are published —
   **Then** this is shown with it.
4. **Given** a venue, **When** the researcher records its costs — a charge to publish, a
   registration fee, with currency and the date the figure was noted — and its policies on
   open access and on preprints, **Then** these are shown.
5. **Given** a venue, **When** the researcher records its typical limits and requirements —
   length, format, anonymity, the template to use — **Then** these are shown, and are
   available when a manuscript is aimed at it.
6. **Given** a venue, **When** the researcher records its standing in one or more ranking
   schemes, each with the scheme's name, the value, and the year, **Then** venues can be
   listed and ordered by a scheme.
7. **Given** a venue, **When** the researcher sets their interest in it — target, watching,
   neutral, or avoid, with a reason for avoid — **Then** venues can be listed by interest,
   and a venue to avoid warns when a manuscript is aimed at it.
8. **Given** venues, **When** the researcher lists them filtered by kind, topic, ranking,
   interest, language, or open-access policy, or searches their names and topics, **Then**
   only matching venues are shown.
9. **Given** a venue, **When** the researcher views it, **Then** its profile, its next
   deadline, the manuscripts aimed at it, and a summary of past experience are shown.
10. **Given** a venue with the same name or identifier as an existing one, **When** it is
    added, **Then** the tool reports the likely duplicate and asks whether to add or cancel.
11. **Given** a venue that changes its name, or is absorbed by another, **When** the
    researcher records the new name or the successor, **Then** the earlier name is kept and
    still finds it.
12. **Given** venues, **When** the researcher exports some or all of them, **Then** a file is
    produced with their profiles and without the researcher's private notes and experience;
    brought into another workspace, they are added and duplicates reported.
13. **Given** a missing name, an unknown kind, a web address or identifier in an invalid
    form, or a negative cost, **When** the researcher saves, **Then** the tool rejects it
    and reports every problem together.
14. **Given** a venue that manuscripts, events, or submissions refer to, **When** the
    researcher deletes it, **Then** the tool lists them and refuses; it can be marked as no
    longer active instead.

---

### User Story 2 - Never miss a deadline (Priority: P2)

A researcher records the events and calls of the venues they care about: this year's
edition of a conference with its dates and place, and its call with every date that matters
— abstract registration, full submission, notification, final version. They see all
upcoming deadlines in one list, are told when one is close, and when a deadline is
extended, record the new date. Next year's edition starts as a copy of this year's.

**Why this priority**: Deadlines are the most time-critical knowledge a researcher has
about venues and the easiest to lose. This turns the register into something that is
looked at every week.

**Independent Test**: Add an event with a call of four dates, see the dates in the view of
what is due, extend the submission deadline, and create next year's edition from this one.

**Acceptance Scenarios**:

1. **Given** a conference, workshop, or symposium, **When** the researcher adds an event with
   its year or name, its dates, its place, and whether it is in person, online, or both,
   **Then** the event is stored under the venue.
2. **Given** a venue or an event, **When** the researcher adds a call with a title and its
   dates — each with a kind (abstract, submission, notification, final version, other) and
   a date — **Then** the call is stored and its dates appear in the workspace's view of
   what is due.
3. **Given** a date of a call, **When** the researcher states its time and time zone, or that
   it is "anywhere on earth", **Then** the moment is shown in the researcher's local time as
   well; with no time zone it is treated as end of day anywhere on earth, and the tool says
   so.
4. **Given** a journal, **When** the researcher records that it accepts submissions at any
   time, **Then** it has an open call with no deadline; a special issue is added as a call
   with its own dates.
5. **Given** upcoming calls, **When** the researcher lists deadlines within a period,
   **Then** every date is shown in order with its venue, its kind, the time remaining, and
   the manuscripts aimed at that call.
6. **Given** a call whose date is extended or changed, **When** the researcher records the
   new date, **Then** the earlier date is kept in the call's history and everything that
   depends on it follows.
7. **Given** an event, **When** the researcher creates the next edition from it, **Then** a
   new event is created with the same call structure, dates moved by a year and marked as
   expected, not confirmed, until the researcher confirms them.
8. **Given** a call with dates marked expected, **When** deadlines are listed, **Then** they
   are shown as expected, distinguishable from confirmed ones.
9. **Given** a call, **When** the researcher records its particular requirements — length,
   format, anonymity, topics of this edition — **Then** they are shown with the call and
   take the place of the venue's general ones.
10. **Given** a call whose submission date has passed, **When** deadlines are listed,
    **Then** it is no longer shown as upcoming, and its later dates — notification, final
    version — still are, for the manuscripts sent to it.
11. **Given** a venue the researcher watches, **When** it has no upcoming call recorded and
    its last edition was about a year ago, **Then** the tool lists it among venues whose
    next call is probably out.
12. **Given** dates out of order — notification before submission — an event ending before
    it begins, or a date in an invalid form, **When** the researcher saves, **Then** the
    tool rejects it.

---

### User Story 3 - Choose where to send a manuscript (Priority: P3)

For a manuscript, a researcher lists the venues it could go to, in order of preference,
with a note on why each fits. They compare the candidates side by side — next deadline,
standing, cost, time to decision, requirements — and choose one. The manuscript then takes
that venue's deadline and requirements. If it is rejected, the next candidate is ready.

**Why this priority**: This is the decision the register exists to support. It needs venues
and their calls, and a manuscript to decide for.

**Independent Test**: Give a manuscript three candidate venues with fit notes, compare
them, aim it at the first, confirm the manuscript's target and deadline, then record that
it was not accepted and move to the second candidate.

**Acceptance Scenarios**:

1. **Given** a manuscript, **When** the researcher adds venues as candidates in order of
   preference, each with a note on its fit, **Then** the manuscript shows its shortlist.
2. **Given** a manuscript with candidates, **When** the researcher compares them, **Then**
   one table shows for each its kind, next deadline and time remaining, standing in the
   researcher's preferred ranking scheme, costs, review model, open-access policy, the
   researcher's own time to decision there, and the researcher's interest.
3. **Given** a candidate, **When** the researcher asks whether the manuscript meets its
   requirements, **Then** the tool compares the manuscript's length, kind, language, and
   anonymity with the venue's or call's requirements and reports each as met, not met, or
   not checkable.
4. **Given** a manuscript, **When** the researcher aims it at a venue, or at a particular
   call, **Then** the manuscript's target venue and deadline are set from it, and the
   manuscript appears under that call.
5. **Given** a manuscript aimed at a call, **When** the call's date changes, **Then** the
   manuscript's deadline follows.
6. **Given** a manuscript aimed at a venue marked to avoid, **When** it is aimed, **Then**
   the tool warns with the recorded reason and asks for confirmation.
7. **Given** a manuscript aimed at a venue with a template recorded, **When** the researcher
   starts its document, **Then** that template is offered.
8. **Given** a manuscript that was not accepted at its target, **When** the researcher moves
   on, **Then** the next candidate becomes the target, the earlier one is kept in the
   manuscript's history, and the shortlist shows which have been tried.
9. **Given** a manuscript's topics and kind, **When** the researcher asks for suggestions,
   **Then** venues from the register whose topics overlap, that take that kind of
   manuscript, and that are not marked to avoid are listed, those with an upcoming deadline
   first.
10. **Given** a degree programme or a funder that counts only venues of a certain standing,
    **When** the researcher records that rule, **Then** candidates that do not meet it are
    marked.
11. **Given** a venue that does not exist in the register, **When** the researcher adds it
    as a candidate by name, **Then** the tool offers to create it.
12. **Given** the same venue added twice to a shortlist, or a call whose submission date has
    passed, **When** the researcher saves or aims, **Then** the tool rejects the first and
    warns on the second.

---

### User Story 4 - Learn from experience with each venue (Priority: P4)

Each time a manuscript goes to a venue, something is learned: how long the decision took,
what the reviews were like, whether the process was fair, whether the editors answered. A
researcher sees, for each venue, their own history there — what was sent, what happened,
how long it took — and writes down their impressions, so that the next choice is made from
experience rather than memory.

**Why this priority**: A researcher's own record is the most reliable information they have
about a venue, and it accumulates only if kept. It needs submissions to have happened, so
it follows the choice of venue.

**Independent Test**: With two past submissions to a venue, one accepted and one rejected,
view the venue's history with the time to decision of each, add an impression with a
rating, and list venues ordered by the researcher's own acceptance record.

**Acceptance Scenarios**:

1. **Given** a venue with past submissions, **When** the researcher views its history,
   **Then** each submission is listed with the manuscript, the dates, the decision, the
   number of review rounds, and the time from submission to first decision.
2. **Given** a venue's history, **When** it is summarized, **Then** the number of
   submissions, acceptances, and rejections, and the shortest, typical, and longest time to
   decision are shown, computed from the researcher's own submissions only.
3. **Given** a venue, **When** the researcher adds an impression — a dated note with an
   optional rating of the quality of reviews, the speed, and the handling — **Then** it is
   stored with the venue, private by default.
4. **Given** venues with history, **When** the researcher lists them ordered by their own
   time to decision or acceptance record, **Then** the order reflects their experience.
5. **Given** a submission still awaiting a decision, **When** the researcher looks at it,
   **Then** the time waited is shown beside the researcher's typical time at that venue,
   and it is marked when it has waited longer than their longest.
6. **Given** a venue with no submissions by the researcher, **When** its history is asked
   for, **Then** the tool says there is none and shows what the researcher recorded about
   its stated times.
7. **Given** a venue, **When** the researcher records what the venue itself states — its
   acceptance rate, its typical time to decision — with the date noted, **Then** these are
   shown separately from the researcher's own figures and never mixed with them.
8. **Given** impressions, **When** venues are exported for a colleague, **Then** impressions
   are left out unless the researcher explicitly includes them.
9. **Given** a rating outside its scale, **When** the researcher saves, **Then** the tool
   rejects it.

---

### User Story 5 - Attend events and give talks (Priority: P5)

A researcher records the events they plan to attend or have attended, and what they
presented there: a talk, a poster, a tutorial — its title, its date, its material, and the
manuscript or results it was based on. They keep the practical dates of attending — early
registration, travel — and afterwards have a list of everything they have presented, in the
form a CV or a funder's report asks for.

**Why this priority**: Presenting is part of publishing, and the list of talks is asked for
as often as the list of papers. It is the least urgent story because nothing else depends
on it.

**Independent Test**: Mark an event as one to attend with a registration deadline, record a
talk and a poster given there linked to a manuscript, and produce the list of presentations
for the year.

**Acceptance Scenarios**:

1. **Given** an event, **When** the researcher marks that they plan to attend, attended, or
   decided not to, **Then** events can be listed by that state.
2. **Given** an event to attend, **When** the researcher records practical dates — the end of
   early registration, a visa or travel-grant deadline — and costs with their currency,
   **Then** the dates appear in the workspace's view of what is due and the costs are shown
   with the event.
3. **Given** an event, **When** the researcher records something they presented — a talk, a
   poster, a tutorial, a demonstration, a panel — with its title, date, co-presenters, and
   the location of its material, **Then** it is stored under the event.
4. **Given** a presentation, **When** the researcher links the manuscript, results, or
   project it was based on, and the grant that paid for it, **Then** each shows the other.
5. **Given** a presentation not tied to a recorded event — an invited seminar at another
   university — **When** the researcher records it with the host and the place, **Then** it
   is stored without an event.
6. **Given** presentations, **When** the researcher asks for their list, **Then** they are
   listed newest first, grouped by year or by kind, in a chosen citation style, and can be
   filtered by period, kind, project, and grant.
7. **Given** a list of presentations, **When** the researcher exports it, **Then** a
   document is produced.
8. **Given** a presentation that was invited, or received an award, **When** the researcher
   records it, **Then** it is marked in the list.
9. **Given** an event, **When** the researcher adds notes — people met, talks worth
   following up — **Then** they are kept with the event, and a follow-up can be captured as
   a thought in the inbox.
10. **Given** a presentation dated outside its event's dates, a cost that is negative, or a
    missing title, **When** the researcher saves, **Then** the tool rejects it, warning
    rather than rejecting for the date.

---

### Edge Cases

- A conference publishes its proceedings in a journal or a book series: the venue records
  where its papers appear, and a manuscript published there shows both.
- A workshop is held at a larger conference: the workshop's event records the event it is
  part of, and shares its place and overall dates.
- A venue has several tracks with different deadlines: each track is a call of the same
  event.
- A venue has two rounds of submission a year: each round is a call.
- A deadline is given only as a day, with no time zone: it is treated as end of day anywhere
  on earth, and the tool says so wherever it is shown.
- A deadline falls on a different calendar day in the researcher's local time: both the
  stated moment and the local moment are shown.
- A deadline is extended after it has passed: the new date is recorded, and the call is
  upcoming again.
- An event is cancelled or moved online: its state is recorded; its calls remain with a
  note; manuscripts aimed at it are listed for the researcher to look at.
- A venue stops existing: it is marked as no longer active, kept with everything that refers
  to it, and left out of suggestions.
- Two venues have the same acronym: both are kept; wherever an acronym is typed and matches
  several, the tool lists them.
- A ranking scheme revises a venue's standing: the new value is recorded with its year and
  the earlier one kept; a manuscript published earlier shows the standing of its year.
- A venue is ranked in several schemes that disagree: all are shown; ordering uses the
  scheme the researcher asks for.
- Costs are in different currencies: they are shown in the currency recorded and never
  converted or added together.
- A cost was noted long ago: it is shown with the date it was noted, and marked when older
  than a year.
- A manuscript is aimed at a call and the manuscript is then abandoned: it leaves the call's
  list; the shortlist is kept.
- A manuscript is under review at one venue and the researcher aims it at another: the tool
  warns that it is still under consideration elsewhere.
- The researcher's own time to decision rests on a single submission: it is shown, marked as
  based on one.
- A venue's list is brought in from a colleague and contains venues already present with
  different details: the differences are shown and the researcher chooses.
- A presentation is given twice at different events: it is recorded for each, linked as the
  same material.
- Names in any language and script are stored and shown as written; searching ignores
  letter case and accents.

## Requirements *(mandatory)*

### Functional Requirements

#### Venues

- **FR-001**: Users MUST be able to create, list, view, update, and delete venues, each with
  a name, a kind (journal, conference, workshop, symposium, book series, preprint server,
  other), a short name or acronym, a publisher or organizer, a web address, identifiers,
  the languages it publishes in, and the topics it covers.
- **FR-002**: Users MUST be able to record a venue's review model (whether reviewers know the
  authors, whether authors know the reviewers, whether reviews are published), its policies
  on open access and on preprints, its costs with currency and the date each was noted, and
  what the venue itself states about its acceptance rate and time to decision, with the
  date noted.
- **FR-003**: Users MUST be able to record a venue's general requirements for submissions:
  the kinds of manuscript it takes, length limits, language, whether authors must be
  hidden, and the template to use.
- **FR-004**: Users MUST be able to record a venue's standing in any number of ranking
  schemes, each entry with the scheme's name, the value, and the year; earlier entries MUST
  be kept, and venues MUST be listable and orderable by a chosen scheme.
- **FR-005**: Users MUST be able to set their interest in a venue — target, watching,
  neutral, or avoid — with a reason required for avoid, and to mark a venue as no longer
  active.
- **FR-006**: Users MUST be able to filter venues by kind, topic, standing in a scheme,
  interest, language, open-access policy, and whether active, and search their names,
  acronyms, earlier names, and topics, ignoring letter case and accents.
- **FR-007**: Viewing a venue MUST show its profile, its next deadline, the manuscripts aimed
  at it, its events, and a summary of the user's experience there.
- **FR-008**: The system MUST report a likely duplicate when a venue is added with the same
  name, acronym, or identifier as an existing one, and MUST list the matches when an
  acronym is ambiguous.
- **FR-009**: Users MUST be able to record that a venue changed its name or was succeeded by
  another, and that a venue's papers appear in another venue; earlier names MUST still find
  it.
- **FR-010**: Users MUST be able to export venues to a file and bring such a file in;
  private notes, impressions, and the user's own history MUST be left out unless explicitly
  included; bringing in MUST report duplicates and show differences for the user to choose.
- **FR-011**: The system MUST refuse to delete a venue that manuscripts, events, calls,
  submissions, or presentations refer to.

#### Events and calls

- **FR-012**: Users MUST be able to add, update, and remove events of a venue, each with a
  name or year, start and end dates, a place, whether it is in person, online, or both, a
  state (announced, confirmed, cancelled, held), and the event it is part of, if any.
- **FR-013**: Users MUST be able to add, update, and remove calls of a venue or an event,
  each with a title and any number of dates, each date with a kind (abstract, submission,
  notification, final version, other), a moment, and whether it is confirmed or expected.
- **FR-014**: A date MUST be recordable with a time and a time zone or as "anywhere on
  earth"; with neither it MUST be treated as end of day anywhere on earth. Wherever a date
  is shown, the stated moment and the user's local moment MUST both be shown when they fall
  on different days.
- **FR-015**: A venue MUST be able to have an open call with no deadline, and several calls
  at once.
- **FR-016**: Users MUST be able to record a call's particular requirements, which take the
  place of the venue's general ones for that call.
- **FR-017**: Every date of every call MUST appear in the workspace's view of what is due,
  with its venue and kind; users MUST be able to list deadlines within a period with the
  time remaining and the manuscripts aimed at each call.
- **FR-018**: Changing a date MUST keep the earlier date in the call's history, and
  everything that depends on the date MUST follow.
- **FR-019**: Users MUST be able to create the next edition of an event from an existing
  one, with the same call structure and dates moved by a year and marked expected until
  confirmed.
- **FR-020**: After a call's submission date has passed it MUST no longer be listed as
  upcoming; its later dates MUST remain listed for manuscripts sent to it.
- **FR-021**: The system MUST list watched and target venues that have no upcoming call
  recorded and whose last edition was about a year ago or more.
- **FR-022**: The system MUST reject dates of a call that are out of order and an event that
  ends before it begins.

#### Choosing a venue

- **FR-023**: Users MUST be able to give a manuscript an ordered shortlist of candidate
  venues, each with a note on its fit and a state (considering, tried, chosen, dropped); a
  venue MUST NOT appear twice in one shortlist.
- **FR-024**: The system MUST compare a manuscript's candidates side by side: kind, next
  deadline and time remaining, standing in a chosen ranking scheme, costs, review model,
  open-access policy, the user's own time to decision and record there, and the user's
  interest.
- **FR-025**: The system MUST compare a manuscript with a venue's or call's requirements —
  kind, length, language, anonymity — and report each as met, not met, or not checkable.
- **FR-026**: Users MUST be able to aim a manuscript at a venue or at a particular call; the
  manuscript's target venue and deadline MUST be set from it, MUST follow later changes to
  the call's date, and the manuscript MUST be listed under the call.
- **FR-027**: Aiming a manuscript at a venue marked to avoid MUST warn with the recorded
  reason and require confirmation; aiming at a call whose submission date has passed, or
  while the manuscript is under review elsewhere, MUST warn.
- **FR-028**: When a manuscript is not accepted at its target, users MUST be able to move to
  the next candidate; the earlier target MUST be kept in the manuscript's history and
  marked as tried.
- **FR-029**: The system MUST suggest candidates from the register for a manuscript: venues
  whose topics overlap with the manuscript's, that take its kind, that are active and not
  marked to avoid, those with an upcoming deadline first.
- **FR-030**: Users MUST be able to record rules about which venues count — a ranking scheme
  and a minimum standing — for a programme, a funder, or themselves; candidates that do not
  meet a rule MUST be marked.
- **FR-031**: When a manuscript is aimed at a venue with a template recorded, that template
  MUST be offered when the manuscript's document is created.

#### Experience

- **FR-032**: The system MUST show, for a venue, the user's own history there: each
  submission with its manuscript, dates, decision, number of review rounds, and time from
  submission to first decision.
- **FR-033**: The system MUST summarize that history — submissions, acceptances, rejections,
  and shortest, typical, and longest time to decision — from the user's own submissions
  only, MUST say on how many submissions a figure rests, and MUST keep the venue's own
  stated figures separate.
- **FR-034**: Users MUST be able to add dated impressions of a venue, with optional ratings
  of the quality of reviews, the speed, and the handling on a five-step scale; impressions
  MUST be private by default.
- **FR-035**: Users MUST be able to order venues by their own time to decision and by their
  own acceptance record.
- **FR-036**: For a submission awaiting a decision, the system MUST show the time waited
  beside the user's typical time at that venue, and MUST mark it when longer than their
  longest.

#### Attendance and presentations

- **FR-037**: Users MUST be able to record, for an event, whether they plan to attend,
  attended, or will not, with practical dates (which MUST appear in the view of what is
  due), costs with currency, and notes.
- **FR-038**: Users MUST be able to create, list, view, update, and delete presentations,
  each with a title, a kind (talk, poster, tutorial, demonstration, panel, seminar, other),
  a date, the event or — without one — the host and place, co-presenters, the location of
  its material, and whether it was invited or received an award.
- **FR-039**: Users MUST be able to link a presentation to the manuscripts, results, and
  projects it was based on and to the grants that paid for it, and to record that two
  presentations used the same material.
- **FR-040**: The system MUST produce a list of the user's presentations, newest first,
  grouped by year or kind, in a chosen citation style, filterable by period, kind, project,
  and grant, and exportable as a document.
- **FR-041**: Users MUST be able to capture a follow-up from an event's notes as a thought
  in the inbox.

#### Common behavior

- **FR-042**: Every value a user supplies for venues, rankings, costs, events, calls, dates,
  shortlists, impressions, attendance, and presentations — typed or brought in from a file
  — MUST be validated before anything is stored; invalid input MUST change nothing and MUST
  be reported per value, all together, with what is expected.
- **FR-043**: Costs MUST be shown in the currency recorded with the date noted, MUST NOT be
  converted or added across currencies, and MUST be marked when noted more than a year ago.
- **FR-044**: Venues, events, calls, and presentations MUST support tags, notes, links, and
  topics as other records do, and every creation, change, and deletion MUST be recorded in
  the workspace's audit trail.
- **FR-045**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-046**: Every command MUST have built-in help, and venues, events and calls, choosing
  a venue, experience, and presentations MUST each have a usage guide with examples.

### Key Entities *(include if feature involves data)*

- **Venue**: A lasting place that publishes or presents research: a journal, a conference
  series, a workshop series. Has a profile (scope, review model, costs, policies,
  requirements), standings, earlier names, and the researcher's interest in it.
- **Standing**: A venue's value in a named ranking scheme in a given year.
- **Event**: One occurrence of a venue that meets: an edition with dates, a place, and a
  state; possibly part of a larger event.
- **Call**: An invitation to submit to a venue or an event, with its dates and particular
  requirements. Open and without deadline for a journal that always accepts.
- **Call Date**: One date of a call: its kind, its moment and time zone, whether confirmed
  or expected, and its earlier values.
- **Candidate**: A venue on a manuscript's shortlist, with its order, a note on fit, and
  whether it is being considered, was tried, was chosen, or was dropped.
- **Counting Rule**: Which venues count for a programme, a funder, or the researcher: a
  ranking scheme and a minimum standing.
- **Impression**: The researcher's dated note about a venue, with optional ratings; private.
- **Attendance**: The researcher's relation to an event: planning to attend, attended, or
  not; with practical dates and costs.
- **Presentation**: Something the researcher presented — a talk, a poster, a tutorial — at an
  event or elsewhere, with its material and what it was based on. Called a "talk" in the
  earlier specification.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can record a venue with its kind, topics, and one ranking in under 2
  minutes, and find any venue among 500 by a word of its name or topics in under 2 seconds.
- **SC-002**: A user can see every submission deadline of the next 90 days across all their
  venues in a single list in under 5 seconds, and 100% of recorded call dates appear in the
  workspace's view of what is due.
- **SC-003**: Every deadline is shown with the correct moment in the user's local time in
  100% of cases, including dates stated as "anywhere on earth" and dates that fall on
  another calendar day locally.
- **SC-004**: A user can create next year's edition of a conference with its call in under
  30 seconds.
- **SC-005**: A user can compare five candidate venues for a manuscript in a single view in
  under 5 seconds.
- **SC-006**: When a call's date changes, 100% of manuscripts aimed at it show the new
  deadline.
- **SC-007**: After a rejection, a user can aim a manuscript at its next candidate in under
  30 seconds, and the earlier target remains in the manuscript's history.
- **SC-008**: For any venue the user has submitted to, their own time to decision and record
  are shown, computed from their submissions only, and agree with a calculation by hand in
  100% of cases.
- **SC-009**: A user can produce the list of their presentations for a period in a chosen
  style in under 30 seconds.
- **SC-010**: Impressions and private notes appear in 0 exported venue lists unless
  explicitly included.
- **SC-011**: 100% of invalid inputs are rejected before any data changes, each with the
  invalid value named.
- **SC-012**: Users who keep their target venues' calls in the workspace miss no submission
  deadline for lack of knowing it.
- **SC-013**: 90% of first-time users complete the primary task of each story on their
  first attempt using only that story's usage guide.

## Assumptions

- **This specification owns venues.** User Story 12 of `specs/002-research-lifecycle`
  (FR-052 and FR-053: venues, calls, talks) is replaced by this one; that specification
  keeps a short pointer.
- **"Places to publish (events, conferences, journals)" is read as** venues that last
  (journals, conference series), their occurrences (events), and their invitations to
  submit (calls). Talks are included because they were specified with venues before and are
  what a researcher does at an event.
- **Submissions and peer review stay where they are**, in `specs/002-research-lifecycle`
  (its User Story 8). This specification reads them to show a venue's history; it does not
  redefine them. Manuscripts, their target venue and deadline, and their stages are those of
  `specs/008-manuscripts`.
- **The researcher records what they know; the tool looks nothing up.** It does not fetch
  calls for papers, deadlines, rankings, fees, or acceptance rates from anywhere, and ships
  no list of venues and no ranking scheme. Figures go out of date, which is why each is
  kept with the date it was noted.
- **Ranking schemes are whatever the researcher's context uses** — a national
  classification, a field's conference ranking, a journal quartile. The tool stores scheme
  name, value, and year; it does not know which values are better unless the researcher
  says, when recording a counting rule, what the minimum is.
- **Suggestions come only from the researcher's own register**, by overlap of topics and
  kind. The tool does not recommend venues it has not been told about and does not judge a
  venue's quality or legitimacy; "avoid" is the researcher's own mark.
- **Requirements are compared only where the workspace knows the answer**: kind, length,
  language, and anonymity of the manuscript. Everything else is shown for the researcher to
  check, and a venue's checklist can be kept with `specs/014-conventions-methods`.
- **The researcher's own figures are few.** Time to decision and acceptance record rest on
  their own submissions, often one or two; the tool says how many and never presents them
  as the venue's statistics.
- **Attending an event is recorded lightly**: state, practical dates, costs, notes. Booking
  travel, expense claims, and schedules of sessions are out of scope.
- **Other specifications are used, not redefined**: the view of what is due
  (`specs/003-research-projects`), templates (`specs/007-templates`), citation styles
  (`specs/005-literature`), grants (`specs/002-research-lifecycle`), the inbox and topics
  (`specs/013-ideas-questions`), the "private" marking (`specs/006-reports`), and links,
  tags, notes, and the audit trail (`specs/001-research-workspace`).
- **Sharing a list of venues is by file**, as elsewhere in TRCLI.
- **Single researcher**, as elsewhere.
