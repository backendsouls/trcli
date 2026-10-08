### User Story 3 - Follow the people I supervise (Priority: P3)

A lab head records who supervises whom, and after each meeting with a student or a team
member writes down the date, what was discussed, and what was agreed — each agreed action
becoming a task with an owner and a date. Before the next meeting they see, for that
person, what was agreed last time and whether it was done, where their project and
programme stand, and how long it has been since they last met.

**Why this priority**: Supervision is the part of running a lab that is easiest to let
slide. A record of meetings and of what was agreed is small to keep and prevents most of
what goes wrong.

**Independent Test**: Record a supervision, record two meetings with agreed actions, mark
one action done, view the preparation for the next meeting, and list the supervised people
not met for more than four weeks.

**Acceptance Scenarios**:

1. **Given** two people, **When** the lab head records that one supervises the other, as
   main supervisor or co-supervisor, from a date, **Then** each shows the relationship.
2. **Given** a person, **When** the lab head records a meeting with them — the date, who
   attended, and notes of what was discussed — **Then** the meeting is stored and listed
   with the person.
3. **Given** a meeting, **When** the lab head records agreed actions, each with an owner
   and a date, **Then** each becomes a task linked to the meeting.
4. **Given** a supervised person, **When** the lab head asks to prepare their next meeting,
   **Then** the tool shows the actions agreed last time and their status, what the person
   has completed and what is overdue since, their next milestone and programme requirement,
   and the date of the last meeting.
5. **Given** supervised people, **When** the lab head asks who has not been met for longer
   than a period, **Then** those people are listed with the date of their last meeting.
6. **Given** a meeting with several attendees, **When** it is recorded, **Then** it appears
   for each attendee, and its actions may belong to different people.
7. **Given** a person, **When** the lab head lists their meetings, **Then** they are shown
   in order of date with their agreed actions and whether each was done.
8. **Given** a meeting, **When** the lab head marks its notes as private, **Then** they
   appear in nothing produced for others.
9. **Given** a supervision that ends (the student graduates, the supervisor changes),
   **When** the lab head records the end date, **Then** it is kept in history and no longer
   listed among current supervisions.
10. **Given** a person recorded as their own supervisor, a meeting dated in the future, or
    an action without an owner, **When** the lab head saves, **Then** the tool rejects it.
11. **Given** a supervisor, **When** the lab head asks whom they supervise, **Then** current
    supervisees are listed with their position, expected end, and next milestone.

---
