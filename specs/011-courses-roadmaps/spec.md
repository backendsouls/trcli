<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Courses, Curricula, and Roadmaps

**Feature Branch**: `011-courses-roadmaps`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add a new spec for courses, curricular matrix, roadmaps, graduate roadmaps etc"

## Overview

A researcher in a degree programme is on two roads at once. One is the research itself,
which the other specifications cover. The other is the **programme**: the courses to take,
the credits to earn, the exams to pass, the things to deliver, each by a certain time. Lose
track of the second and the first cannot be finished, however good it is. Beyond any
programme there is a third road, the researcher's own learning: the things they decided to
study to be able to do the work.

This specification gathers that side of academic life into one place.

It **takes over** the course content of the first specification, which now points here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 11 | Courses taken and taught, linked to research records |

It **adds** the programme and its curriculum, progress against it, planning of terms, the
graduate roadmap of requirements beyond courses, and personal learning roadmaps.

### The words in the request

| Term | Meaning in TRCLI |
|------|------------------|
| **Course** | One course the researcher takes or teaches in one term, with its credits, grade, and status. |
| **Curriculum** *(curricular matrix)* | The programme's official list of what must and may be studied: each component with its credits, whether it is mandatory or elective, the term it is suggested for, and what must be completed before it. |
| **Graduate roadmap** | Everything a graduate programme requires besides passing courses — a minimum of credits, a language exam, a qualifying exam, a proposal defense, a teaching internship, a publication, the thesis — each with a time limit counted from enrolment. |
| **Roadmap** *(learning roadmap)* | The researcher's own plan toward a goal they chose — "be able to do Bayesian analysis" — as ordered stages of things to study and do. |

### The concepts, in one picture

```text
 Programme ── has ──▶ Curriculum ── lists ──▶ Component ── requires ──▶ Component
     │                (matrix)                  │ (credits, category, suggested term)
     │                                          ▲ fulfils
     │ enrolled since                           │
     │                                   Course (taken, in a Term, with grade)
     │                                          │
     ├── has ──▶ Graduate Roadmap ── lists ──▶ Requirement ◀── evidence ── Course / Manuscript /
     │           (time limits from enrolment)   (status, due by)            Milestone / Document
     │
     └── belongs to a ──▶ Project (doctoral, master's, …)                     (003)

 Learning Roadmap ── has ──▶ Stage ── has ──▶ Item (course, reference, skill, task) ── depends on ──▶ Item
```

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Record the courses I take and teach (Priority: P1)

A researcher records each course they take: its name and code, the institution, the term,
its credits and hours, who teaches it, and — as it goes — its status and final grade. They
record the courses they teach in the same way. They can see everything for a term, and
their whole history as a transcript.

**Why this priority**: Courses are the unit everything else here is built from, and a plain
record of what was taken, when, and with what grade is useful by itself — it is what a
researcher is asked for in every report and application.

**Independent Test**: Record three courses in two terms with credits, complete two with
grades, record one course taught, and view the transcript with totals per term.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records a course they take with a title
   and a term, **Then** it is stored with the status "planned".
2. **Given** a course, **When** the researcher records its code, institution, credits,
   hours, instructor, schedule, and notes, **Then** they are saved and shown.
3. **Given** a course, **When** the researcher changes its status (planned, enrolled, in
   progress, completed, failed, dropped, exempted), **Then** the status and date are
   recorded.
4. **Given** a completed course, **When** the researcher records its final grade, **Then**
   the grade is stored in the grading scheme the researcher uses, and is rejected if it is
   outside that scheme.
5. **Given** a course, **When** the researcher records assessments within it — an exam, an
   assignment — each with a date, a weight, and a result, **Then** they are listed, and
   those with a date in the future appear in the view of what is due.
6. **Given** courses in several terms, **When** the researcher lists them filtered by term,
   status, institution, or whether taken or taught, **Then** only matching courses are
   shown.
7. **Given** courses with grades, **When** the researcher asks for their transcript,
   **Then** courses are listed by term with credits and grades, with credits attempted and
   earned and the average grade per term and overall.
8. **Given** a course the researcher teaches, **When** they record it with its term, hours,
   number of students, and their role (lecturer, teaching assistant, intern), **Then** it is
   stored and appears in a list of teaching, separate from the transcript.
9. **Given** a course, **When** the researcher links references, methodologies, manuscripts,
   or research questions to it, **Then** the course lists them and each lists the course.
10. **Given** a course whose end is before its start, credits that are negative, or a term
    in an invalid form, **When** the researcher saves, **Then** the tool rejects it and
    reports every problem together.
11. **Given** a course failed and taken again, **When** both are recorded, **Then** both
    appear in the transcript, and only the passed one counts toward credits earned.
12. **Given** a transcript, **When** the researcher exports it, **Then** a document is
    produced.

---

### User Story 2 - Describe my programme and its curriculum (Priority: P2)

A researcher records the programme they are enrolled in — institution, degree, the date
they started, the time allowed — and its curriculum: every component with its code,
credits, category (mandatory, elective, optional), the term it is suggested for, the area
it belongs to, and what must be completed before it. They can type it in, or bring it in
from a table. They see it as a grid, term by term: the curricular matrix.

**Why this priority**: The curriculum is what progress is measured against. Without it the
courses of the first story are a list; with it they are a position on a map.

**Independent Test**: Record a programme, bring in a curriculum of twelve components from a
table, add a prerequisite between two of them, view the grid by suggested term, and confirm
a prerequisite loop is rejected.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records a programme with a name, an
   institution, a degree level, and the date of enrolment, **Then** it is stored, and can be
   linked to the project it belongs to.
2. **Given** a programme, **When** the researcher records the time normally expected and
   the maximum time allowed, **Then** the expected and latest dates of completion are shown.
3. **Given** a programme, **When** the researcher creates its curriculum with a name and the
   year it took effect, **Then** it is stored as the curriculum the researcher follows.
4. **Given** a curriculum, **When** the researcher adds a component with a code, a title,
   credits, hours, a category, a suggested term, and an area, **Then** it is stored.
5. **Given** components, **When** the researcher states that one requires another to be
   completed first, or to be taken at the same time, **Then** the requirement is stored and
   shown on both.
6. **Given** a curriculum, **When** the researcher records its rules — total credits
   required, and the minimum credits in each category or area — **Then** they are stored
   with the curriculum.
7. **Given** a curriculum, **When** the researcher views the matrix, **Then** components are
   shown in columns by suggested term, each with its code, credits, and category, and its
   prerequisites marked.
8. **Given** a table of components in a file, **When** the researcher brings it in,
   **Then** each valid row becomes a component, each invalid row is reported with its
   reason, and nothing is stored when the researcher only asked for a preview.
9. **Given** components whose prerequisites form a loop, or a prerequisite on a code that
   does not exist, **When** the researcher saves, **Then** the tool rejects it and names the
   components.
10. **Given** a programme that changes its curriculum, **When** the researcher records a new
    curriculum and moves to it, **Then** the earlier one is kept, and the tool shows which
    completed components carry over and which no longer count.
11. **Given** a group of electives from which a number of credits must be chosen, **When**
    the researcher defines the group, **Then** the matrix shows it as one requirement with
    its options.
12. **Given** a component code used twice in a curriculum, negative credits, or a suggested
    term below one, **When** the researcher saves, **Then** the tool rejects it.
13. **Given** a curriculum, **When** the researcher exports it, **Then** a table is produced
    that can be brought into another workspace.

---

### User Story 3 - See where I stand in the curriculum (Priority: P3)

A researcher sees their progress through the curriculum: which components are completed, in
progress, and still to do; credits earned against required, in total and per category; and
which components they could take next because their prerequisites are met. A course taken
elsewhere, or under another name, is counted by declaring it equivalent to a component.

**Why this priority**: "How much is left?" and "what can I take next term?" are the two
questions a student asks every term. This answers both from records that already exist.

**Independent Test**: With a curriculum and several completed courses, view progress and
confirm the credits per category; declare an outside course equivalent to a component and
confirm it now counts; list what can be taken next and confirm a component with an unmet
prerequisite is not on it.

**Acceptance Scenarios**:

1. **Given** a course taken with the same code as a component of the curriculum, **When**
   it is recorded, **Then** it is counted for that component without further action.
2. **Given** a course with a different code or from another institution, **When** the
   researcher declares it equivalent to a component, **Then** it counts for that component,
   and the equivalence is recorded with a note of who granted it.
3. **Given** a curriculum and courses, **When** the researcher asks for their progress,
   **Then** each component is shown as completed, in progress, planned, or to do, with the
   course and grade behind it.
4. **Given** progress, **When** it is summarized, **Then** credits earned, in progress, and
   remaining are shown in total and for each category and area with a minimum, and each
   rule of the curriculum is shown as met or not.
5. **Given** a curriculum, **When** the researcher asks what they can take next, **Then**
   the components not yet completed whose prerequisites are all completed are listed,
   mandatory ones first.
6. **Given** a component whose prerequisite is not completed, **When** the researcher asks
   why it is not available, **Then** the tool names the prerequisites still missing.
7. **Given** a course that fulfils no component, **When** progress is shown, **Then** it is
   listed separately as counting toward free or extra credits, or not at all, as the
   researcher states.
8. **Given** an elective group needing a number of credits, **When** progress is shown,
   **Then** the credits earned within the group are shown against the number needed.
9. **Given** a component for which the researcher was exempted, **When** the exemption is
   recorded with its justification, **Then** the component counts as completed without a
   grade.
10. **Given** one course declared equivalent to two components, or two courses to the same
    component, **When** the researcher saves, **Then** the tool warns and asks for
    confirmation, since institutions differ on whether this is allowed.
11. **Given** progress, **When** the researcher compares it with the suggested terms,
    **Then** the tool shows whether they are ahead of, on, or behind the suggested pace.
12. **Given** a programme with no curriculum recorded, **When** the researcher asks for
    progress, **Then** the tool shows the transcript totals and says no curriculum is
    recorded.

---

### User Story 4 - Plan the terms ahead (Priority: P4)

A researcher lays out which components to take in each coming term. The tool checks the
plan: prerequisites come first, the credit load of each term is within the limits the
researcher sets, every mandatory component is somewhere, and the last term falls within the
time the programme allows. They can keep more than one plan and compare them.

**Why this priority**: Planning is the forward-looking use of the curriculum. It is worth
doing once position is known, and it is what prevents discovering in the final year that a
prerequisite chain is one term too long.

**Independent Test**: Plan three terms, deliberately place a component before its
prerequisite and overload one term, and confirm the check reports both; fix them and
confirm the projected completion date.

**Acceptance Scenarios**:

1. **Given** a curriculum, **When** the researcher places components in future terms,
   **Then** the plan is stored and shown term by term with the credits of each term.
2. **Given** a plan, **When** it is checked, **Then** the tool reports components placed
   before or with a prerequisite that must come first, terms above or below the credit load
   the researcher set, mandatory components placed nowhere, and rules of the curriculum the
   plan would not meet.
3. **Given** a plan, **When** it is shown, **Then** the term in which the curriculum would
   be completed is shown, and whether that is within the expected and the maximum time.
4. **Given** a plan, **When** the researcher asks the tool to propose one, **Then**
   remaining components are spread over terms respecting prerequisites, suggested terms,
   and the credit load, as a proposal to edit; nothing is saved without acceptance.
5. **Given** a component offered only in certain terms, **When** the researcher records
   that, **Then** the check reports a plan that places it in another term.
6. **Given** a term that begins, **When** the researcher enrols in the planned components,
   **Then** courses are created from the plan for that term in one step.
7. **Given** a course failed or dropped, **When** the plan is checked, **Then** the
   component is shown as unplaced again, with what else is now delayed by it.
8. **Given** two plans, **When** the researcher compares them, **Then** their differences
   term by term and their completion terms are shown.
9. **Given** a plan, **When** the researcher marks it as the one they follow, **Then**
   progress and the view of what is due use it.
10. **Given** a term in the past, or a component already completed, **When** it is placed
    in a plan, **Then** the tool rejects it.
11. **Given** terms defined by the researcher's institution (two or three per year, with
    their dates), **When** the researcher records them, **Then** plans and courses use
    those terms.

---

### User Story 5 - Follow my graduate roadmap (Priority: P5)

A graduate researcher records what the programme requires besides courses — a minimum of
credits, a language proficiency exam, the qualifying exam, the proposal defense, a teaching
internship, a published or submitted paper, seminar attendance, the thesis and its defense
— and by when each must be done, counted from enrolment. For each they record its status
and the evidence that satisfies it. They see at any moment what is done, what is next, and
how much time is left before each limit.

**Why this priority**: These requirements, not the courses, are what graduate students lose
track of, and missing a time limit can end a degree. It builds on the programme and gives
the project's milestones their official counterpart.

**Independent Test**: Start from the proposed roadmap for a master's programme, set the time
limit of two requirements in months from enrolment, satisfy one with a completed course and
one with a submitted manuscript, and view the roadmap showing what is done, what is due
next, and days remaining.

**Acceptance Scenarios**:

1. **Given** a programme, **When** the researcher creates its graduate roadmap, **Then** the
   tool proposes typical requirements for the programme's degree level, and adds only those
   the researcher accepts.
2. **Given** a roadmap, **When** the researcher adds a requirement with a title, a
   description, a kind, and whether it is mandatory, **Then** it is stored with the status
   "pending".
3. **Given** a requirement, **When** the researcher sets its time limit as a number of
   months from enrolment or as a date, **Then** the due date is shown, and follows the
   enrolment date if that is corrected.
4. **Given** a requirement, **When** the researcher attaches evidence — a completed course,
   a manuscript at a given stage, a reached milestone, a document, or a note — **Then** the
   evidence is listed with the requirement.
5. **Given** a requirement that is a count or an amount — credits, seminars attended,
   papers submitted — **When** evidence is attached, **Then** the amount reached is shown
   against the amount required, and the requirement is satisfied when it is reached.
6. **Given** a requirement, **When** the researcher marks it satisfied with the date, or
   waived with a justification, **Then** the status and date are recorded.
7. **Given** a roadmap, **When** the researcher views it, **Then** requirements are shown in
   order of due date with status, time remaining or overdue, and evidence, and the next one
   is highlighted.
8. **Given** a roadmap, **When** it is summarized, **Then** the number of requirements
   satisfied out of the total, the next deadline, and the latest date to complete the
   programme are shown, with whether the researcher is within the time allowed.
9. **Given** requirements with due dates, **When** the researcher asks what is due,
   **Then** they appear with tasks and milestones in the workspace's view of what is due.
10. **Given** a requirement that must follow another (the defense after the qualifying
    exam), **When** the order is recorded, **Then** the later one cannot be marked satisfied
    before the earlier one, without an explicit override.
11. **Given** a requirement linked to a milestone of the researcher's project, **When** the
    milestone is reached, **Then** the tool offers to mark the requirement satisfied, and
    does so only on acceptance.
12. **Given** an approved extension of time or a leave of absence, **When** the researcher
    records it with its length and justification, **Then** the due dates that depend on
    enrolment move accordingly, and the extension is shown.
13. **Given** a roadmap, **When** the researcher exports it, **Then** a document is produced
    showing every requirement, its status, its evidence, and the dates, suitable to hand to
    a supervisor or a programme office.
14. **Given** a time limit of zero or negative months, a due date before enrolment, or a
    required amount that is not positive, **When** the researcher saves, **Then** the tool
    rejects it.

---

### User Story 6 - Build my own learning roadmaps (Priority: P6)

A researcher sets themselves a learning goal — "be able to run and interpret Bayesian
models" — and lays out how to get there: stages in order, each with items to complete: a
course (in their programme or elsewhere), a book or paper to read, a skill to practise, a
small task. Items can depend on others. They mark items done, see how far along each stage
is, and can share a roadmap with a colleague or start from one somebody gave them.

**Why this priority**: Much of what a researcher must learn is in no curriculum. A personal
roadmap gives that learning the same visibility. It is independent of the programme stories
and the least urgent.

**Independent Test**: Create a roadmap with two stages and six items of four kinds, add a
dependency, mark three items done, view progress per stage, export the roadmap, and bring
it into another workspace as a fresh, not-started copy.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a roadmap with a title and a goal,
   **Then** it is stored with the status "active".
2. **Given** a roadmap, **When** the researcher adds stages in order, each with a title and
   an optional target date, **Then** the roadmap shows them.
3. **Given** a stage, **When** the researcher adds an item of a kind — a course, a reference
   from the library, a skill, a task, or a free item with a web address — **Then** it is
   stored with the status "to do".
4. **Given** an item that is a course, a reference, or a task, **When** it is linked to the
   existing record, **Then** its status follows that record: the course completed, the
   reference read, the task done.
5. **Given** items, **When** the researcher states that one depends on another, **Then** the
   dependent item is shown as blocked until the other is done.
6. **Given** a roadmap, **When** the researcher views it, **Then** each stage shows its
   items with status and how many are done, and the roadmap shows its overall progress and
   what to do next.
7. **Given** an item, **When** the researcher marks it done, skipped, or in progress, and
   records the time it took or a note of what was learned, **Then** this is saved.
8. **Given** a skill item, **When** the researcher records their level before and after
   (none, basic, working, proficient), **Then** the roadmap shows the change.
9. **Given** a roadmap, **When** the researcher exports it, **Then** a file is produced with
   its stages, items, and dependencies and without their personal progress.
10. **Given** such a file, **When** the researcher brings it in, **Then** a new roadmap is
    created, not started; references it names that are not in the library are listed and
    can be added.
11. **Given** a roadmap, **When** every item is done or skipped, **Then** the tool offers to
    mark the roadmap completed with a closing note.
12. **Given** items that depend on each other in a loop, a stage without a title, or a
    target date in an invalid form, **When** the researcher saves, **Then** the tool rejects
    it.
13. **Given** several roadmaps, **When** the researcher lists them, **Then** each is shown
    with its goal, progress, and next item.

---

### Edge Cases

- A course spans two terms, or has no term (a short course, an online course at one's own
  pace): it is recorded with its dates and counted in the term it ends, or in none.
- Institutions grade differently — numbers from 0 to 10 or 0 to 100, letters, concepts,
  pass or fail: the researcher chooses the scheme per institution; averages are computed
  only within one scheme and never across schemes.
- A pass-or-fail course is completed: it counts for credits and is left out of the average.
- Credits are counted differently in two institutions (credits against hours): equivalences
  record how many credits of the curriculum the outside course is worth.
- A course is taken more often than the programme allows, or failed more often than
  allowed: the tool records it; it does not know the programme's limits unless the
  researcher writes them as a rule.
- A component is removed from the curriculum after the researcher completed it: it stays
  completed and is shown as no longer part of the curriculum.
- The researcher changes programme or transfers: a new programme is recorded; courses stay
  in the transcript and can be declared equivalent in the new curriculum.
- The researcher is in two programmes at once: each has its own curriculum, progress, and
  roadmap; a course may count in both.
- The enrolment date is corrected after requirements were given limits in months: every
  such due date is recalculated, and the tool lists what moved.
- A leave of absence covers a period in which a course was nonetheless recorded: accepted
  with a warning.
- A requirement's evidence is deleted (the manuscript is removed): the requirement keeps
  its status and shows the evidence as missing.
- A manuscript given as evidence moves backwards (accepted, then withdrawn): the requirement
  is flagged for the researcher to review; its status is not changed by itself.
- A term plan refers to a component that a new curriculum no longer has: the check reports
  it.
- Nothing can be taken next because every remaining component is blocked: the tool shows the
  chain of prerequisites that leads out.
- A proposed plan cannot fit in the maximum time whatever the load: the tool says so and
  shows the shortest possible plan.
- A roadmap item points to a course or reference that is later deleted: the item remains as
  a free item with the title it had.
- A roadmap brought in from someone else has the same title as an existing one: both are
  kept; the new one says where it came from.
- A table of components brought in has columns in another order or under other headings:
  the researcher says which column is which, and the tool remembers it for that source.
- Titles and names in any language and script are stored and shown as written.
- Values are given in an invalid form — credits that are not a number, a grade outside the
  scheme, a term that does not exist: all are reported together, each with what is expected.

## Requirements *(mandatory)*

### Functional Requirements

#### Courses

- **FR-001**: Users MUST be able to create, list, view, update, and delete courses, each with
  a title, whether it is taken or taught, a code, an institution, a term or dates, credits,
  hours, an instructor, a schedule, and notes.
- **FR-002**: The system MUST track a taken course's status — planned, enrolled, in progress,
  completed, failed, dropped, exempted — with the date of each change.
- **FR-003**: Users MUST be able to record a course's final grade in a grading scheme chosen
  per institution: a numeric range, a set of letters or concepts in order with which ones
  pass, or pass/fail. A grade outside the scheme MUST be rejected.
- **FR-004**: Users MUST be able to record assessments within a course, each with a title, a
  date, a weight, and a result; assessments with a future date MUST appear in the
  workspace's view of what is due.
- **FR-005**: For a course taught, users MUST be able to record their role, the hours, and
  the number of students; courses taught MUST be listed separately from courses taken.
- **FR-006**: Users MUST be able to filter courses by term, status, institution, programme,
  and whether taken or taught, and search by words in title and code.
- **FR-007**: The system MUST produce a transcript: courses taken by term with credits and
  grades; credits attempted and earned and the average grade per term and overall; and an
  export of it as a document.
- **FR-008**: Averages MUST be weighted by credits, MUST leave out pass/fail, exempted,
  dropped, and in-progress courses, and MUST NOT combine grades of different schemes.
- **FR-009**: Users MUST be able to link courses to references, methodologies, manuscripts,
  research questions, and any other record, with every link visible from both ends.
- **FR-010**: Users MUST be able to define the terms of an institution — how many per year,
  their names, and their dates — and courses and plans MUST use them.

#### Programmes and curricula

- **FR-011**: Users MUST be able to create, list, view, update, and delete programmes, each
  with a name, an institution, a degree level, the date of enrolment, the time normally
  expected, the maximum time allowed, a status (enrolled, on leave, completed, withdrawn),
  and the project it belongs to.
- **FR-012**: Users MUST be able to record more than one programme, at the same time or one
  after another.
- **FR-013**: Users MUST be able to create a curriculum for a programme with a name and the
  year it took effect, keep several curricula for one programme, and state which one they
  follow.
- **FR-014**: Users MUST be able to add, update, and remove a curriculum's components, each
  with a code unique in the curriculum, a title, credits, hours, a category (mandatory,
  elective, optional, complementary activity), a suggested term, an area, and the terms in
  which it is offered.
- **FR-015**: Users MUST be able to state that a component requires others to be completed
  first (prerequisites) or to be taken in the same term (corequisites); the system MUST
  reject a requirement on a component that does not exist and requirements that form a
  loop, naming the components.
- **FR-016**: Users MUST be able to define groups of components from which a stated number
  of credits or of components must be completed.
- **FR-017**: Users MUST be able to record a curriculum's rules: total credits required, and
  minimum credits per category, per area, and per group.
- **FR-018**: The system MUST present a curriculum as a matrix: components by suggested term
  with code, title, credits, and category, prerequisites marked, groups shown as one
  requirement with their options, and totals per term.
- **FR-019**: Users MUST be able to bring in components from a table in a file and export a
  curriculum to one; bringing in MUST report each invalid row with its reason, MUST offer a
  preview that stores nothing, and MUST let the user say which column holds which detail.
- **FR-020**: When the user moves to another curriculum of the programme, the system MUST
  show which completed components carry over and which no longer count, and MUST keep the
  earlier curriculum.

#### Progress

- **FR-021**: A course taken MUST count for the curriculum component with the same code at
  the same institution without further action.
- **FR-022**: Users MUST be able to declare a course equivalent to a component, with the
  credits it is worth and a note of who granted it, and to record an exemption from a
  component with its justification; an exempted component MUST count as completed without a
  grade.
- **FR-023**: The system MUST warn, and require confirmation, when one course is counted for
  two components or two courses for one.
- **FR-024**: The system MUST show, for each component, whether it is completed, in progress,
  planned, or to do, with the course, term, and grade behind it.
- **FR-025**: The system MUST show credits earned, in progress, and remaining, in total and
  for every category, area, and group that has a minimum, and whether each rule of the
  curriculum is met.
- **FR-026**: A failed or dropped course MUST NOT count; when a component was attempted
  several times, the passed attempt MUST count and all attempts MUST remain in the
  transcript.
- **FR-027**: Users MUST be able to list the components they can take next — not completed,
  with every prerequisite completed — and, for any component, the prerequisites still
  missing.
- **FR-028**: Courses that fulfil no component MUST be listed separately, counted as free
  credits or not counted, as the user states for each.
- **FR-029**: The system MUST show whether the user is ahead of, on, or behind the pace of
  the suggested terms, counted from enrolment.

#### Term plans

- **FR-030**: Users MUST be able to create term plans for a curriculum, placing components
  in future terms, keep several plans, compare two, and mark one as followed.
- **FR-031**: Users MUST be able to set the minimum and maximum credit load of a term.
- **FR-032**: The system MUST check a plan and report components placed before a
  prerequisite, corequisites placed in different terms, components placed in a term in
  which they are not offered, terms outside the credit load, mandatory components placed
  nowhere, and rules of the curriculum the plan would leave unmet.
- **FR-033**: The system MUST show, for a plan, the term in which the curriculum would be
  completed and whether it is within the expected and the maximum time.
- **FR-034**: Users MUST be able to ask for a proposed plan that respects prerequisites,
  offering, suggested terms, and credit load; a proposal MUST be editable and MUST NOT be
  saved without acceptance. When no plan fits the maximum time, the system MUST say so and
  show the shortest.
- **FR-035**: Users MUST be able to create the courses of a term from the followed plan in
  one step.
- **FR-036**: When a planned or enrolled course is failed or dropped, the system MUST show
  its component as unplaced and name the components delayed by it.
- **FR-037**: The system MUST reject placing a completed component, or placing any
  component in a past term.

#### Graduate roadmaps

- **FR-038**: Users MUST be able to create a graduate roadmap for a programme: an ordered
  set of requirements, each with a title, a description, a kind, whether it is mandatory,
  and a status (pending, in progress, satisfied, waived, missed).
- **FR-039**: The system MUST propose typical requirements for a master's and for a doctoral
  programme when a roadmap is created, and MUST add only those the user accepts.
- **FR-040**: The system MUST support these kinds of requirement: credits, course, exam,
  language proficiency, qualifying, proposal, teaching, publication, attendance, residency,
  thesis, defense, deposit, and other.
- **FR-041**: Users MUST be able to give a requirement a time limit as a number of months
  from enrolment or as a date; limits in months MUST follow a corrected enrolment date, and
  the system MUST list what moved.
- **FR-042**: Users MUST be able to attach evidence to a requirement — courses, manuscripts
  at a stated stage, milestones, documents, results of exams, and notes — and, for a
  requirement that is an amount, the amount each piece of evidence contributes; the
  requirement MUST show the amount reached against the amount required.
- **FR-043**: Users MUST be able to mark a requirement satisfied with its date or waived
  with a justification. The system MUST offer to mark a requirement satisfied when its
  required amount is reached or a linked milestone is reached, and MUST do so only on
  acceptance.
- **FR-044**: Users MUST be able to state that a requirement must follow another; the later
  one MUST NOT be marked satisfied first without an explicit override that is recorded.
- **FR-045**: Users MUST be able to record extensions of time and leaves of absence with
  their length and justification; due dates counted from enrolment MUST move accordingly,
  as MUST the latest date to complete the programme.
- **FR-046**: The system MUST present a roadmap in order of due date with each requirement's
  status, time remaining or overdue, and evidence; and summarize it with requirements
  satisfied out of the total, the next deadline, the latest completion date, and whether
  the user is within the time allowed.
- **FR-047**: Requirements with due dates MUST appear in the workspace's view of what is
  due, and a requirement whose due date has passed without being satisfied or waived MUST
  be shown as missed.
- **FR-048**: When evidence is deleted or goes backwards, the system MUST flag the
  requirement for review and MUST NOT change its status by itself.
- **FR-049**: Users MUST be able to export a roadmap as a document with every requirement,
  its status, evidence, and dates.

#### Learning roadmaps

- **FR-050**: Users MUST be able to create, list, view, update, and delete learning
  roadmaps, each with a title, a goal, a status (active, paused, completed, abandoned), and
  ordered stages with a title and an optional target date.
- **FR-051**: Users MUST be able to add items to a stage, of the kinds course, reference,
  skill, task, and free item with an optional web address; each item has a status (to do,
  in progress, done, skipped), an optional estimate of effort, and notes.
- **FR-052**: An item linked to a course, a reference, or a task MUST take its status from
  that record; when the record is deleted the item MUST remain as a free item.
- **FR-053**: Users MUST be able to make items depend on other items; a dependent item MUST
  be shown as blocked until the others are done or skipped, and the system MUST reject
  dependencies that form a loop.
- **FR-054**: The system MUST show, for a roadmap, the items done per stage, overall
  progress, the items that can be worked on next, and stages past their target date.
- **FR-055**: Users MUST be able to record, for an item, the time it took and what was
  learned, and for a skill item their level before and after (none, basic, working,
  proficient).
- **FR-056**: Users MUST be able to export a roadmap's stages, items, and dependencies
  without personal progress, and bring such a file in as a new roadmap that is not started;
  the system MUST list references the file names that are not in the library and offer to
  add them.
- **FR-057**: When every item of a roadmap is done or skipped, the system MUST offer to mark
  it completed with a closing note.

#### Common behavior

- **FR-058**: Every value a user supplies for courses, grades, terms, programmes, curricula,
  components, equivalences, plans, requirements, evidence, and roadmaps — typed or brought
  in from a file — MUST be validated before anything is stored; invalid input MUST change
  nothing and MUST be reported per value, all together, with what is expected.
- **FR-059**: Every record type here MUST support tags, notes, and links, belong to projects
  as other records do, and have every creation, change, deletion, and status change
  recorded in the workspace's audit trail.
- **FR-060**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-061**: Every command MUST have built-in help, and courses and transcripts, programmes
  and curricula, progress, term plans, graduate roadmaps, and learning roadmaps MUST each
  have a usage guide with examples.

### Key Entities *(include if feature involves data)*

- **Course**: One course taken or taught in one term: title, code, institution, credits,
  hours, status, grade, assessments.
- **Assessment**: An exam or assignment within a course, with date, weight, and result.
- **Term**: A teaching period of an institution, with a name and dates.
- **Grading Scheme**: How an institution grades: a numeric range, ordered letters or
  concepts, or pass/fail, with what counts as passing.
- **Programme**: A degree programme the researcher is enrolled in: institution, level,
  enrolment date, expected and maximum time, status. Belongs to a project.
- **Curriculum** *(curricular matrix)*: A version of a programme's official list of
  components and the rules for completing it.
- **Component**: One entry of a curriculum: code, title, credits, category, suggested term,
  area, when it is offered, and what it requires first.
- **Component Group**: Components from which a number of credits or components must be
  chosen.
- **Curriculum Rule**: A total or minimum the curriculum demands, overall or per category,
  area, or group.
- **Equivalence / Exemption**: The decision that a course counts for a component, or that a
  component need not be taken.
- **Term Plan**: An arrangement of remaining components over future terms; one is the plan
  followed.
- **Graduate Roadmap**: The requirements of a graduate programme besides its curriculum,
  with their time limits.
- **Requirement**: One thing the programme demands: its kind, whether mandatory, its limit,
  its status, the amount required if any, what it must follow, and its evidence.
- **Evidence**: A record or note showing that a requirement is met, with the amount it
  contributes.
- **Extension / Leave**: An approved change to the time allowed.
- **Learning Roadmap**: The researcher's own plan toward a goal: stages and items.
- **Stage**: An ordered step of a learning roadmap, with a target date.
- **Roadmap Item**: Something to complete in a stage: a course, a reference, a skill, a
  task, or a free item; possibly depending on others.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can record a course with its term and credits in under 30 seconds, and
  produce their transcript in under 5 seconds.
- **SC-002**: A user can bring in a curriculum of 60 components from a table in under 5
  minutes, including saying which column is which, and receives a reason for every row not
  brought in.
- **SC-003**: A user can see credits earned and remaining, in total and per category, in a
  single view in under 5 seconds.
- **SC-004**: Credits earned, credits remaining, and averages shown agree with a calculation
  by hand from the same courses in 100% of cases.
- **SC-005**: A user can find out which components they can take next term in under 10
  seconds, and 100% of those listed have every prerequisite completed.
- **SC-006**: The check of a term plan reports 100% of prerequisite-order problems, overloaded
  terms, and unplaced mandatory components, and reports none for a plan without them.
- **SC-007**: A user can see, for any graduate requirement, how many days remain before its
  limit in under 5 seconds, and every requirement with a limit appears in the view of what
  is due.
- **SC-008**: After the enrolment date is corrected or an extension recorded, 100% of
  due dates counted from enrolment are recalculated correctly.
- **SC-009**: 0 requirements change to "satisfied" without the user's acceptance.
- **SC-010**: A user starting a master's or doctoral programme can have a graduate roadmap
  with time limits in under 10 minutes, starting from the proposed requirements.
- **SC-011**: A learning roadmap exported from one workspace and brought into another has
  identical stages, items, and dependencies, and no progress.
- **SC-012**: 100% of invalid inputs, typed or brought in, are rejected before any data
  changes, each with the invalid value named.
- **SC-013**: 90% of first-time users complete the primary task of each story on their
  first attempt using only that story's usage guide.

## Assumptions

- **This specification owns courses.** User Story 11 of `specs/001-research-workspace`
  (FR-056 and FR-057) is replaced by this one; that specification keeps a short pointer.
- **"Curricular matrix" is rendered as "curriculum"**, with "matrix" kept for its view as a
  grid by term. It is the programme's official structure (*matriz curricular*), as the
  researcher records it.
- **"Graduate roadmap" is read as the path through a graduate programme**: its requirements
  besides courses and their time limits. **"Roadmap" alone is read as a personal learning
  roadmap.** They are two record types because one is imposed by a programme and the other
  chosen by the researcher.
- **The researcher enters their programme's rules; the tool does not know them.** It ships
  no curricula of real institutions and does not claim to know any programme's regulations.
  Proposed graduate requirements are generic starting points to edit:
  - *Master's*: minimum credits; language proficiency; qualifying exam or proposal defense;
    dissertation submitted; defense; final version deposited.
  - *Doctoral*: minimum credits; proficiency in one or two languages; qualifying exam;
    proposal defense; teaching internship; a paper submitted or published; thesis
    submitted; defense; final version deposited.
- **The tool is the student's own record, not the institution's system.** It is not an
  official transcript, does not enrol anyone, and does not exchange data with academic
  systems. Its totals help the student; the institution's are the ones that count.
- **Undergraduate programmes are covered by the same curriculum, progress, and plan
  stories.** The graduate roadmap is offered for master's and doctoral programmes; an
  undergraduate may still create a roadmap of requirements of their own (internship,
  complementary hours, capstone).
- **Project milestones and graduate requirements are related but distinct.** A milestone
  (`specs/003-research-projects`) is a dated checkpoint of the research project that the
  researcher plans; a requirement is an obligation of the programme. A requirement can take
  a milestone as evidence. The typical milestones proposed per project type there and the
  requirements proposed here overlap by design and are linked, not merged.
- **Tasks, the view of what is due, and projects** are those of
  `specs/003-research-projects`; **references** those of `specs/005-literature`;
  **manuscripts** and their stages those of `specs/008-manuscripts`; **staff**, links, and
  the audit trail those of `specs/001-research-workspace`.
- **Averages follow the commonest rule** — weighted by credits within one grading scheme.
  Institutions with another rule should treat the figure as indicative.
- **Proposed term plans are simple and explainable**: earliest term that respects
  prerequisites, offering, and load. The tool does not optimize for workload balance,
  timetable clashes, or instructor.
- **Courses taught are recorded for the researcher's own account** of their teaching; class
  lists, attendance, and grading of students are out of scope.
- **Skill levels are the researcher's self-assessment** on a four-step scale.
- **Single researcher.** Supervisors and programme offices receive exported documents; they
  do not use the workspace.
