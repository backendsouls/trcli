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
