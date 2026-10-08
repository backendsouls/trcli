### User Story 5 - Be shown what I am about to forget (Priority: P5)

Every so often the researcher asks the tool what needs another look. It brings back, in one
short list: the parked thoughts whose date has come, the ideas nobody has looked at for
months, the open questions with nothing happening, the hypotheses still untested long after
they were written. For each the researcher says "still relevant", acts on it, parks it
again, or lets it go — and it does not come back until it is due again.

**Why this priority**: Capturing and sorting keep things from being lost on the day they
occur; review keeps them from being lost in the months after. It needs the earlier stories
to have something to bring back.

**Independent Test**: Park one thought until today, leave one idea untouched beyond the
review period, and leave one open question without activity; ask for the review and confirm
exactly those three are presented; mark one as still relevant and confirm it is not
presented again the next day.

**Acceptance Scenarios**:

1. **Given** the workspace, **When** the researcher asks what is due for review, **Then**
   the tool lists parked thoughts whose date has come, ideas not looked at for longer than
   the review period, open questions without activity for longer than their period, and
   hypotheses untested for longer than theirs.
2. **Given** items due for review, **When** the researcher starts a review, **Then** they
   are presented one at a time with what they are, when they were last looked at, and what
   has happened around them since.
3. **Given** an item under review, **When** the researcher marks it still relevant,
   **Then** the date is recorded and it is not due again until the period has passed.
4. **Given** an item under review, **When** the researcher parks it until a date, **Then**
   it is not presented before that date.
5. **Given** an item under review, **When** the researcher acts on it — turns an idea into a
   question, links a question to an experiment, closes a question — **Then** the action
   counts as having looked at it.
6. **Given** an item under review, **When** the researcher lets it go, **Then** an idea is
   dropped or a question abandoned, with a reason, and it is kept among those let go.
7. **Given** the review periods, **When** the researcher sets them for ideas, questions, and
   hypotheses, **Then** later reviews use them.
8. **Given** an item, **When** the researcher asks to be shown it again on a particular
   date, **Then** it is due on that date whatever the periods are, and appears in the
   workspace's view of what is due.
9. **Given** nothing due, **When** the researcher asks for a review, **Then** the tool says
   so and tells when the next item will be due.
10. **Given** many items due after a long absence, **When** a review is started, **Then**
    the most important and the oldest are presented first, and the researcher can stop at
    any time with the rest still due.
11. **Given** items due for review, **When** the researcher looks at the status of the
    workspace or produces an activity report, **Then** the number due is shown.
12. **Given** an item viewed, edited, or linked outside a review, **When** that happens,
    **Then** it counts as looked at.

---
