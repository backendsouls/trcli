<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Ideas, Topics, Questions, and Hypotheses

**Feature Branch**: `013-ideas-questions`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add a new spec for questions, hypothesis, ideas, topics so that is not forgotten"

## Overview

Research begins as a thought: "what if the effect disappears with more data?", "someone
should compare these two methods", "I keep running into this topic". Most such thoughts are
lost — they arrive in the middle of something else, are written on whatever is at hand, and
are never seen again. The ones that survive are not necessarily the best ones, only the
ones that happened to be remembered.

This specification is about **not forgetting**. It gives every thought a place to land in
seconds, a moment at which it is sorted, a path by which the good ones grow into research
questions and testable hypotheses, and a habit by which nothing sits unseen for long.

It **takes over** the content on questions and hypotheses of the first specification, which
now points here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 2 | Research questions, sub-questions, hypotheses, and linking records to them |

It **adds** ideas with an inbox, the sorting of that inbox, topics, and the review that
brings things back before they are forgotten.

### Four kinds of thought

| Kind | What it is | Example |
|------|------------|---------|
| **Idea** | Anything worth not losing; unformed is fine | "Try the method from the vision paper on our audio data" |
| **Topic** | An area of interest that things gather around | "Robustness to label noise" |
| **Research question** | Something the research commits to answering | "Does pre-training reduce the labelled data needed?" |
| **Hypothesis** | A testable claim that would answer a question | "Pre-training halves the labelled data needed for 90% accuracy" |

### How a thought travels

```text
 capture ──▶ Inbox ── sort ──┬──▶ Idea (kept, developing) ──┬──▶ Research Question ──▶ Hypothesis ──▶ Experiment (004)
 (seconds)                   │         │                    ├──▶ Task / Project (003)        │
                             │         └── grouped by ──▶ Topic ◀── also groups ── References, Questions, …
                             ├──▶ merged into an existing one
                             ├──▶ parked until a date ──▶ comes back by itself
                             └──▶ discarded, with a reason (kept, not erased)

 Review: anything not looked at for too long, or due to come back, is brought before the researcher again.
```

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Capture a thought before it is gone (Priority: P1)

In the middle of other work a researcher has a thought. They write it down with one short
command and nothing else — no kind to choose, no fields to fill, no decision to make — and
go back to what they were doing. The thought is in the inbox, dated, waiting.

**Why this priority**: A thought that takes more than a few seconds to record will not be
recorded. Capture is the whole point of "so that it is not forgotten"; everything else in
this specification works on what capture has saved.

**Independent Test**: From inside a workspace, capture three thoughts with a single short
command each, one of them while naming a paper it came from; list the inbox and confirm all
three are there, dated, in order.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher captures a thought with only its text,
   **Then** it is stored in the inbox with the date and time, and the tool confirms in one
   line.
2. **Given** a capture, **When** nothing but the text is given, **Then** nothing else is
   asked: no kind, no topic, no confirmation.
3. **Given** a thought that came from something — a paper being read, a run just finished,
   a meeting — **When** the researcher names that record while capturing, **Then** the
   thought is linked to it.
4. **Given** a current project, **When** a thought is captured, **Then** it belongs to that
   project; with no current project it belongs to none and is still captured.
5. **Given** a longer thought, **When** the researcher captures it from the standard input
   or from a file, **Then** the whole text is stored.
6. **Given** several thoughts to record at once, **When** the researcher gives them one per
   line, **Then** each becomes a separate entry.
7. **Given** the inbox, **When** the researcher lists it, **Then** unsorted thoughts are
   shown oldest first with their age, and the number waiting is shown.
8. **Given** a thought captured with a hint — that it is a question, or belongs to a topic —
   **When** the hint is given, **Then** it is recorded as a suggestion for sorting and the
   thought still lands in the inbox.
9. **Given** an empty text, **When** the researcher captures, **Then** the tool rejects it;
   every other text is accepted as written, in any language.
10. **Given** a thought captured while the tool is used from a script or another program,
    **When** it is captured, **Then** it succeeds without any prompt.
11. **Given** unsorted thoughts in the inbox, **When** the researcher looks at the status of
    the workspace, **Then** the number waiting and the age of the oldest are shown.
12. **Given** a thought captured by someone else and told to the researcher, **When** the
    researcher records whose idea it was, **Then** the originator is stored with it.

---

### User Story 2 - Sort the inbox (Priority: P2)

Once in a while the researcher goes through the inbox. The tool shows the thoughts one at a
time, and for each they decide: keep it as an idea, under a topic; turn it into a research
question, a hypothesis, or a task; merge it into something that already says the same; park
it until a later date; or discard it with a word about why. When they are done the inbox is
empty, and nothing has been erased.

**Why this priority**: An inbox that is never emptied is another place where thoughts are
forgotten. Sorting is what turns captured text into something that can be found, grouped,
and acted on.

**Independent Test**: With six thoughts in the inbox, sort them — keep two as ideas under a
topic, turn one into a research question and one into a task, merge one into an existing
idea, and discard one with a reason — and confirm the inbox is empty and each result exists
with a link back to the original thought.

**Acceptance Scenarios**:

1. **Given** thoughts in the inbox, **When** the researcher starts sorting, **Then** they are
   presented one at a time, oldest first, each with its text, date, origin, and any hint.
2. **Given** a thought, **When** the researcher keeps it as an idea, optionally with a title
   and topics, **Then** it leaves the inbox and exists as an idea.
3. **Given** a thought, **When** the researcher turns it into a research question, a
   hypothesis under a question, a task, or a topic, **Then** that record is created with the
   thought's text, and remembers the thought it came from.
4. **Given** a thought that repeats something already recorded, **When** the researcher
   merges it into that idea or question, **Then** its text is added there as a dated note
   and it leaves the inbox.
5. **Given** a thought similar in wording to existing ideas or questions, **When** it is
   presented, **Then** the similar ones are shown, so that merging is easy.
6. **Given** a thought not for now, **When** the researcher parks it until a date or for a
   period, **Then** it leaves the inbox and returns to it on that date.
7. **Given** a thought not worth keeping, **When** the researcher discards it with a reason,
   **Then** it leaves the inbox and remains findable among discarded thoughts.
8. **Given** a sorting session, **When** the researcher skips a thought or stops, **Then**
   skipped thoughts stay in the inbox and what was decided is kept.
9. **Given** one thought that contains two, **When** the researcher splits it, **Then** two
   entries replace it, each to be sorted.
10. **Given** a thought, **When** the researcher sorts it directly with one command, without
    a session, **Then** the same outcomes are available.
11. **Given** a decision made by mistake, **When** the researcher undoes the sorting of a
    thought, **Then** it returns to the inbox and what was created from it is removed or,
    if already changed, kept and unlinked on the researcher's choice.
12. **Given** a hint recorded at capture, **When** the thought is presented, **Then** the
    hinted outcome is offered first.
13. **Given** a decision that needs something that does not exist — a question to place a
    hypothesis under, a topic by a name not yet used — **When** the researcher gives it,
    **Then** the tool offers to create it.

---

### User Story 3 - Frame research questions and hypotheses (Priority: P3)

A researcher records the questions the research is trying to answer and the hypotheses that
follow from them, and links every other kind of record to them. At any time they can ask,
for one question, "what have I read, reviewed, tried, found, and written about this?", and
for the whole workspace, "which questions are still open, and which have nothing happening?".

**Why this priority**: The question is what gives the rest of the workspace its meaning.
It comes after capture and sorting only because those are what feed it; it is the part of
this specification every other specification links to.

**Independent Test**: Create a research question with two hypotheses, link references to
the question, link a result to a hypothesis as evidence, mark the hypothesis supported, and
view the question showing its hypotheses and everything linked to it.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a research question with a
   statement, **Then** it is stored with the status "open".
2. **Given** a research question, **When** the researcher adds a hypothesis with a statement
   and what would count as support or refutation, **Then** the hypothesis is stored under
   that question with the status "untested".
3. **Given** a research question, **When** the researcher adds a narrower sub-question,
   **Then** the sub-question is shown beneath its parent.
4. **Given** questions and hypotheses, **When** the researcher links references, reviews,
   experiments, results, manuscripts, methodologies, or datasets to them, **Then** the link
   is visible from both ends.
5. **Given** a research question, **When** the researcher asks for its overview, **Then**
   they see its sub-questions, its hypotheses with their status, the ideas it grew from, and
   every linked record grouped by kind.
6. **Given** a hypothesis, **When** the researcher sets its status to supported, refuted, or
   inconclusive, **Then** the tool requires at least one linked result as evidence and
   records the date of the change.
7. **Given** a research question, **When** the researcher marks it answered or abandoned,
   **Then** the status, date, and a required closing note are recorded.
8. **Given** a research question, **When** the researcher records its importance and how
   feasible it seems, **Then** questions can be ordered by either.
9. **Given** records that are linked to no research question, **When** the researcher asks
   for unlinked records, **Then** they are listed by kind.
10. **Given** open questions, **When** the researcher asks which have had no activity for a
    period — no linked record added, no hypothesis changed — **Then** those are listed with
    the date of their last activity.
11. **Given** a hypothesis that an experiment tests, **When** the experiment is concluded
    with an outcome for it, **Then** the tool offers to update the hypothesis and does so
    only on acceptance.
12. **Given** a question whose statement the researcher refines, **When** it is changed,
    **Then** the earlier wording is kept in the question's history.
13. **Given** a question with sub-questions, **When** the researcher deletes it, **Then** the
    tool refuses until the sub-questions are moved or removed; deleting a question or
    hypothesis with linked records lists them and requires confirmation, and the linked
    records are kept.
14. **Given** a sub-question made the parent of its own ancestor, or a missing statement,
    **When** the researcher saves, **Then** the tool rejects it.

---

### User Story 4 - Gather things under topics (Priority: P4)

A researcher keeps a list of the topics they care about — the areas their ideas, questions,
and reading keep returning to. They place ideas, questions, references, and any other
record under one or more topics, arrange topics under broader ones, and open a topic to see
everything they have on it in one place.

**Why this priority**: Ideas are remembered by what they are about. Topics are how a
researcher finds the idea from last spring when the subject comes up again, and how they
see that a topic has many ideas and no question yet.

**Independent Test**: Create a topic with a narrower topic beneath it, place two ideas, a
question, and three references under them, open the broader topic, and confirm everything
under both is shown, grouped by kind.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a topic with a name and a
   description, **Then** it is stored.
2. **Given** a topic, **When** the researcher places it under a broader topic, **Then** it
   is shown beneath it, and the broader topic's page includes what the narrower one holds.
3. **Given** a topic, **When** the researcher places ideas, questions, references, or any
   other record under it, **Then** the topic lists them and each record lists its topics.
4. **Given** a record, **When** it is placed under several topics, **Then** it appears under
   each and exists once.
5. **Given** a topic, **When** the researcher opens it, **Then** everything under it and
   under its narrower topics is shown, grouped by kind, with counts, and with the most
   recent activity first.
6. **Given** topics, **When** the researcher lists them, **Then** each is shown with the
   number of ideas, open questions, and references under it, and when something was last
   added.
7. **Given** a topic, **When** the researcher marks it as one they are actively pursuing,
   watching, or have set aside, **Then** topics can be listed by that state.
8. **Given** topics, **When** the researcher asks for those with ideas but no research
   question, **Then** they are listed — candidates for a question worth asking.
9. **Given** two topics that turn out to be the same, **When** the researcher merges them,
   **Then** one remains holding everything of both.
10. **Given** a topic with things under it, **When** the researcher deletes it, **Then** the
    tool lists them and requires confirmation; the records themselves are kept.
11. **Given** a topic with the name of an existing one, a topic placed beneath itself, or a
    missing name, **When** the researcher saves, **Then** the tool rejects it.
12. **Given** a tag already used on many records, **When** the researcher turns it into a
    topic, **Then** a topic is created holding those records.

---

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

### User Story 6 - Develop an idea until it is worth pursuing (Priority: P6)

Some ideas need to grow before they are a question or a project. A researcher gives an idea
a title, writes what it would take and why it matters, notes arguments for and against,
links what inspired it and what it resembles, rates how promising it looks, and moves it
from raw to developing to ready. A ready idea becomes a research question, a project, an
experiment, or a manuscript, and keeps the record of where it came from.

**Why this priority**: Not every idea needs this; most are either acted on or dropped at
sorting. It serves the few that are worth nurturing, and so comes last.

**Independent Test**: Take a kept idea, give it a title and a rationale, add two arguments
for and one against, link a reference that inspired it, rate it, move it to ready, turn it
into a project, and confirm the project shows the idea it came from.

**Acceptance Scenarios**:

1. **Given** an idea, **When** the researcher gives it a title, a description, why it
   matters, and what it would take, **Then** these are saved with it.
2. **Given** an idea, **When** the researcher adds arguments for and against it, each with
   an optional reference, **Then** they are listed with the idea.
3. **Given** an idea, **When** the researcher links references that inspired it and other
   ideas it resembles or builds on, **Then** the links are shown from both ends.
4. **Given** an idea, **When** the researcher rates how promising it is and how much effort
   it needs, **Then** ideas can be listed in order of either, or of the two together.
5. **Given** an idea, **When** the researcher moves it between raw, developing, ready,
   realized, and dropped, **Then** the state and date are recorded; dropping requires a
   reason.
6. **Given** a ready idea, **When** the researcher turns it into a research question, a
   project, an experiment, or a manuscript, **Then** that record is created from the idea's
   title and description, the idea becomes "realized", and each shows the other.
7. **Given** an idea, **When** the researcher adds to it over time, **Then** each addition
   is dated, and the idea's history shows how it developed.
8. **Given** an idea, **When** the researcher records who originated it and who contributed
   to it, **Then** these people are shown with the idea and when the records it led to are
   viewed.
9. **Given** ideas, **When** the researcher lists them filtered by state, topic, rating,
   originator, or project, or searches their text, **Then** only matching ideas are shown.
10. **Given** a dropped idea, **When** the researcher revives it, **Then** it returns to the
    state it had before, and the reason it was dropped stays in its history.
11. **Given** a rating outside its scale or a state that does not exist, **When** the
    researcher saves, **Then** the tool rejects it.

---

### Edge Cases

- A thought is captured outside any workspace: the tool says there is no workspace here and
  names the default one, if set, capturing into it; with none, it says how to create one
  and loses nothing that was typed, by showing the text back.
- The same thought is captured twice: both are kept; sorting shows the earlier one as
  similar so they can be merged.
- A captured thought is very long, or contains line breaks, quotation marks, or symbols: it
  is stored exactly as written.
- A thought is captured about a record that is later deleted: the thought is kept and says
  what it used to be about.
- The inbox holds hundreds of thoughts: sorting presents them in batches, shows how many
  remain, and can sort several at once by one decision.
- A thought is parked again and again: each parking is kept in its history, and review
  shows how many times it has been put off.
- A parked thought's date passes while the researcher is away for months: it is in the
  inbox when they return, marked with how long it has been due.
- A thought is turned into a question, and the question is later deleted: the thought
  returns to the inbox, marked as having lost what it became.
- A discarded thought turns out to matter: it can be restored to the inbox.
- A hypothesis is created without a question: the tool asks for the question it belongs to,
  and offers to create one from the same text.
- A hypothesis is marked supported and the result it relied on is later replaced by a newer
  run: the hypothesis is flagged for another look and its status is not changed by itself.
- A question is marked answered while one of its hypotheses is still untested: accepted
  with a warning naming the hypothesis.
- A sub-question is moved under a different question: its hypotheses and links go with it.
- A topic is placed under two broader topics: allowed; it appears under both.
- A topic and a tag have the same name: they are different things and may coexist; the tool
  offers to turn the tag into the topic.
- An idea is linked to itself, or two ideas are merged that are already linked: the link to
  itself is refused, and merging removes the link between them.
- Review periods are set to zero: that kind of item is never brought back by time alone.
- A record is both parked and given a date to be shown again: the earlier date wins.
- Everything is due at once after the review periods are shortened: the review still
  presents the most important and the oldest first and can be stopped.

## Requirements *(mandatory)*

### Functional Requirements

#### Capture and inbox

- **FR-001**: Users MUST be able to capture a thought by giving only its text, with a single
  short command, without being asked anything; the thought MUST be stored in an inbox with
  the date and time.
- **FR-002**: Capture MUST accept text as an argument, from the standard input, and from a
  file, and MUST be able to take several thoughts at once, one per line; it MUST store the
  text exactly as written and reject only empty text.
- **FR-003**: Users MUST be able to name, while capturing, the records the thought came
  from, the person who originated it, and a hint of what it is (an idea, a question, a
  hypothesis, a task) or which topic it belongs to; hints MUST be suggestions for sorting
  and MUST NOT sort the thought.
- **FR-004**: A captured thought MUST belong to the current project when there is one and to
  none otherwise, and capture MUST never fail for lack of a project.
- **FR-005**: Users MUST be able to list the inbox, oldest first, with each thought's age,
  origin, and hint, and the number waiting.
- **FR-006**: The number of unsorted thoughts and the age of the oldest MUST be shown in the
  workspace's status view.
- **FR-007**: Capture MUST work when the tool is run by another program, without any
  prompt.

#### Sorting

- **FR-008**: Users MUST be able to sort the inbox in a session that presents thoughts one at
  a time, and to sort a single thought directly; both MUST offer the same outcomes.
- **FR-009**: For each thought the outcomes MUST be: keep as an idea; turn into a research
  question, a hypothesis under a question, a task, or a topic; merge into an existing idea
  or question; split into two thoughts; park until a date or for a period; discard with a
  reason; skip.
- **FR-010**: A record created from a thought MUST carry the thought's text, date, origin,
  and originator, and MUST remember the thought it came from.
- **FR-011**: Merging MUST add the thought's text to the target as a dated note and remove
  it from the inbox.
- **FR-012**: When a thought is presented, the system MUST show existing ideas and questions
  whose wording is similar, and MUST offer first the outcome hinted at capture.
- **FR-013**: A parked thought MUST return to the inbox on its date; a discarded thought
  MUST be kept, findable, and restorable. No outcome may erase a thought.
- **FR-014**: A session MUST be stoppable at any time, keeping what was decided; skipped
  thoughts MUST stay in the inbox.
- **FR-015**: Users MUST be able to undo the sorting of a thought, returning it to the
  inbox; a record created from it MUST be removed if unchanged, and otherwise kept or
  removed on the user's choice.
- **FR-016**: When an outcome needs something that does not exist — a question for a
  hypothesis, a topic by a new name — the system MUST offer to create it.
- **FR-017**: Users MUST be able to sort several thoughts at once by one decision.

#### Ideas

- **FR-018**: Users MUST be able to create, list, view, update, and delete ideas, each with
  its text, an optional title, a description, why it matters, what it would take, and a
  state: raw, developing, ready, realized, or dropped.
- **FR-019**: Users MUST be able to add dated additions to an idea, arguments for and
  against it with optional references, links to the references that inspired it, and links
  to ideas it resembles or builds on; an idea MUST NOT be linked to itself.
- **FR-020**: Users MUST be able to rate an idea's promise and the effort it needs on a
  five-step scale, and list ideas ordered by either or by both.
- **FR-021**: Users MUST be able to record who originated an idea and who contributed to it;
  these people MUST be shown with the idea and with the records it led to.
- **FR-022**: Dropping an idea MUST require a reason; a dropped idea MUST be kept and
  revivable to the state it had before.
- **FR-023**: Users MUST be able to turn an idea into a research question, a project, an
  experiment, a manuscript, or a task; the idea MUST become "realized" and each record MUST
  show the other.
- **FR-024**: Users MUST be able to filter ideas by state, topic, rating, originator, and
  project, and search their text and additions.
- **FR-025**: The system MUST keep the history of an idea: its capture, sorting, additions,
  changes of state, and what it became.

#### Research questions and hypotheses

- **FR-026**: Users MUST be able to create, list, view, update, and delete research
  questions, each with a statement, a motivation, an importance, a feasibility, and a
  status (open, answered, abandoned), and arrange them as questions and sub-questions.
- **FR-027**: Users MUST be able to record hypotheses under a research question, each with a
  statement, what would support or refute it, and a status (untested, supported, refuted,
  inconclusive).
- **FR-028**: Users MUST be able to link references, literature reviews, experiments,
  results, manuscripts, methodologies, datasets, and any other record to research questions
  and hypotheses, with every link visible from both ends.
- **FR-029**: The system MUST show, for any research question, its sub-questions, its
  hypotheses with their status, the ideas it grew from, and every linked record grouped by
  kind.
- **FR-030**: The system MUST require at least one linked result before a hypothesis can be
  marked supported, refuted, or inconclusive, and a closing note before a question can be
  marked answered or abandoned; it MUST keep the history of status changes and of changes
  to a statement's wording.
- **FR-031**: Users MUST be able to list the records that are linked to no research
  question, and the open questions that have had no activity for a stated period.
- **FR-032**: The system MUST reject a question hierarchy that contains a loop, and MUST
  refuse to delete a question that has sub-questions.
- **FR-033**: When a result a hypothesis relied on is replaced or removed, the system MUST
  flag the hypothesis for another look and MUST NOT change its status by itself.
- **FR-034**: When an experiment is concluded with an outcome for a hypothesis, the system
  MUST offer to update the hypothesis and MUST do so only on acceptance.

#### Topics

- **FR-035**: Users MUST be able to create, list, view, update, and delete topics, each with
  a name unique in the workspace ignoring case and accents, a description, and a state:
  pursuing, watching, or set aside.
- **FR-036**: Users MUST be able to place a topic under one or more broader topics; the
  system MUST reject a topic placed beneath itself directly or indirectly.
- **FR-037**: Users MUST be able to place any record under one or more topics; a record
  under several topics MUST exist once, and each record MUST show its topics.
- **FR-038**: Opening a topic MUST show everything under it and under its narrower topics,
  grouped by kind, with counts and the most recent activity first.
- **FR-039**: Listing topics MUST show, for each, the number of ideas, open questions, and
  references under it and when something was last added, and MUST be filterable by state.
- **FR-040**: Users MUST be able to list topics that hold ideas but no research question.
- **FR-041**: Users MUST be able to merge two topics into one that holds everything of both,
  and to turn a tag into a topic holding the records that carry the tag.
- **FR-042**: Deleting a topic MUST list what is under it, require confirmation, and keep
  the records themselves.

#### Review

- **FR-043**: The system MUST treat as due for review: parked thoughts whose date has come;
  ideas in the states raw, developing, or ready not looked at for longer than the review
  period for ideas; open questions without activity for longer than the period for
  questions; untested hypotheses older than the period for hypotheses; and any item the
  user asked to be shown again on a date that has come.
- **FR-044**: Users MUST be able to list what is due for review and to go through it in a
  session that presents items one at a time, most important and oldest first, with when
  each was last looked at and what has happened around it since.
- **FR-045**: For each item under review the outcomes MUST be: still relevant; park until a
  date; act on it; let it go with a reason; skip. "Still relevant" and any action MUST
  record that the item was looked at; letting go MUST drop an idea or abandon a question and
  keep it.
- **FR-046**: Viewing, changing, or linking an item at any time MUST count as looking at it.
- **FR-047**: Users MUST be able to set the review periods for ideas, questions, and
  hypotheses; a period of zero MUST mean that kind is never due by time alone.
- **FR-048**: Users MUST be able to ask for any item to be shown again on a chosen date; it
  MUST then appear in the workspace's view of what is due.
- **FR-049**: The number of items due for review MUST be shown in the workspace's status
  view and in activity reports; with nothing due, the system MUST say when the next item
  will be.
- **FR-050**: The system MUST record how many times an item has been parked or put off and
  show it at review.

#### Common behavior

- **FR-051**: Every value a user supplies for thoughts, ideas, questions, hypotheses, topics,
  ratings, dates, and periods MUST be validated before anything is stored; invalid input
  MUST change nothing and MUST be reported per value, all together, with what is expected.
  Free text MUST be accepted as written, in any language.
- **FR-052**: Ideas, questions, hypotheses, and topics MUST support tags, notes, and links,
  and belong to projects, as every record does; every creation, change, sorting decision,
  and deletion MUST be recorded in the workspace's audit trail.
- **FR-053**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-054**: Every command MUST have built-in help, and capture, sorting, ideas, research
  questions and hypotheses, topics, and review MUST each have a usage guide with examples.

### Key Entities *(include if feature involves data)*

- **Thought**: A piece of text captured before it is gone, with its date, origin,
  originator, and hints. Lives in the inbox until sorted; never erased.
- **Inbox**: The thoughts not yet sorted, including parked ones whose date has come.
- **Sorting Decision**: What was decided for a thought — kept, turned into something,
  merged, split, parked, discarded — and when.
- **Idea**: A thought kept for its own sake: text, title, description, rationale, arguments,
  ratings, state, originator and contributors, additions over time, and what it became.
- **Argument**: A reason for or against an idea, optionally backed by a reference.
- **Topic**: An area of interest, possibly under broader topics, with a state, under which
  any records gather.
- **Research Question**: Something the research is trying to answer: a statement, a
  motivation, importance and feasibility, a status, optional sub-questions, hypotheses, and
  links to everything that bears on it.
- **Hypothesis**: A testable statement under a research question, with what would support or
  refute it, a status, and results linked as evidence.
- **Review Period**: How long an idea, a question, or a hypothesis may go unlooked-at before
  it is due for review.
- **Review Record**: When an item was last looked at, how often it has been put off, and
  when it is next due.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can capture a thought in under 10 seconds, typing nothing but a short
  command and the thought itself, and is asked nothing.
- **SC-002**: 100% of captured thoughts are either in the inbox or accounted for by a
  sorting decision; none is ever erased.
- **SC-003**: A user can sort an inbox of 20 thoughts in under 10 minutes.
- **SC-004**: For any research question, a user can see every linked reference, review,
  experiment, result, and manuscript, and the ideas it grew from, in a single view in under
  5 seconds.
- **SC-005**: For any idea, question, or hypothesis, a user can see where it came from and
  what it led to in under 30 seconds.
- **SC-006**: 100% of parked thoughts return to the inbox on their date.
- **SC-007**: 100% of items not looked at for longer than their review period are listed as
  due, and none that was looked at within it.
- **SC-008**: A user can find any idea among 5,000 by a word of its text or by its topic in
  under 2 seconds.
- **SC-009**: A user can open a topic and see everything under it, across every kind of
  record, in under 5 seconds.
- **SC-010**: A user who captures at least one thought a week and reviews once a month has
  no unsorted thought older than 45 days and no idea unseen for longer than the review
  period.
- **SC-011**: 0 hypotheses change status without the user's decision.
- **SC-012**: 100% of invalid inputs are rejected before any data changes, each with the
  invalid value named.
- **SC-013**: 90% of first-time users capture a thought and sort it on their first attempt
  using only the built-in help.

## Assumptions

- **This specification owns research questions and hypotheses.** User Story 2 of
  `specs/001-research-workspace` (FR-058 to FR-064) is replaced by this one; that
  specification keeps a short pointer. Their rules are carried over unchanged and extended.
- **"So that it is not forgotten" is the purpose of the whole specification**, and is read
  as three things: capture that costs seconds, an inbox that gets sorted, and a review that
  brings things back. Reminders are shown when the researcher uses the tool; it sends no
  notifications by itself.
- **A thought and an idea are different.** A thought is raw text in the inbox; an idea is a
  thought the researcher decided to keep. The distinction is what lets capture ask nothing.
- **Topics are records, tags are labels.** A topic has a description, a place among other
  topics, and a page of its own; a tag is a word on a record. Both exist; a tag can be
  turned into a topic. Topics are also distinct from the glossary's concepts in
  `specs/002-research-lifecycle`, which define terms rather than gather work.
- **Similarity is by wording.** The tool points out thoughts, ideas, and questions that
  share words. It does not judge meaning and may miss a duplicate phrased differently.
- **Review periods have defaults** — three months for ideas, two for open questions, six for
  untested hypotheses — which the researcher changes to suit themselves.
- **Importance, feasibility, promise, and effort are the researcher's own judgement** on
  simple scales; the tool orders by them and computes nothing from them.
- **Originators are people from the register** of `specs/012-people`, or a name. Recording
  who had an idea is for credit and memory; it is not proof of priority.
- **Other specifications are used, not redefined**: tasks, projects, and the view of what
  is due (`specs/003-research-projects`); experiments, results, and conclusions
  (`specs/004-experiments`); references (`specs/005-literature`); manuscripts
  (`specs/008-manuscripts`); the status view (`specs/002-research-lifecycle`); activity
  reports (`specs/006-reports`); links, tags, notes, and the audit trail
  (`specs/001-research-workspace`).
- **Capture needs a workspace.** A researcher who wants to capture from anywhere sets a
  default workspace; capturing on a phone or by voice is out of scope.
- **Single researcher**, as elsewhere: the inbox is theirs alone.
