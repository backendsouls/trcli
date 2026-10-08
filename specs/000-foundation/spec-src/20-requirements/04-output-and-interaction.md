#### Output and interaction

- **FR-028**: Every command MUST present its result both in a form meant for people and, on
  request, in a structured form meant for other programs, with the same content.
- **FR-029**: The form for people MUST be laid out for reading and sized to the terminal;
  colour and special symbols MUST only ever add to information that is also given without
  them.
- **FR-030**: When output is not going to a terminal, the system MUST omit colour and
  decoration without being asked; users MUST be able to force colour on or off, and to
  choose plain symbols; the common conventions for disabling colour MUST be honoured.
- **FR-031**: Results MUST go to one output stream; messages, warnings, progress, and
  questions MUST go to another.
- **FR-032**: The way a command ends MUST distinguish: success; invalid input or usage;
  something not found or ambiguous; a problem with the workspace; a confirmation required,
  refused, or blocked; a check that did not pass; an operation that could not be completed;
  an interruption; and an unexpected failure. These meanings MUST be the same for every
  command and MUST NOT change between versions.
- **FR-033**: A failure MUST be reported in the structured form when that form was asked for,
  with a stable name for the kind of problem and the details needed to act on it.
- **FR-034**: A message about a problem MUST say what happened, whether anything was
  changed, and — where there is an obvious one — the next step.
- **FR-035**: A command that needs a person's answer MUST NOT wait when nobody can answer:
  it MUST fail at once, unchanged, unless the user stated the answer beforehand. The answer
  to an unanswered confirmation MUST be no.
- **FR-036**: Work that takes more than about a quarter of a second MUST show that it is progressing when run
  at a terminal, MUST be interruptible, and on interruption MUST stop within a second, leave the
  workspace valid, and say what was and was not done.
- **FR-037**: Listings MUST be limited to a stated number by default, MUST say how many more
  there are, and MUST let the user ask for more; an empty listing MUST say so and MUST NOT
  be a failure.
- **FR-038**: Dates MUST be shown in one unambiguous form everywhere; moments in time MUST
  be shown in the user's local time, with the zone where it matters.
