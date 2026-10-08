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
