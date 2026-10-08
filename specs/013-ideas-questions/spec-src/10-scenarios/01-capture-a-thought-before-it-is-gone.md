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
