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
