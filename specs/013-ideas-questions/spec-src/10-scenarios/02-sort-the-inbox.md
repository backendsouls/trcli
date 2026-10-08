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
