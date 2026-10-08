### User Story 4 - Get answers fit for people and for programs (Priority: P4)

A researcher at a terminal gets answers that are easy to read: aligned, coloured where
colour helps, shortened to fit. The same researcher, writing a script, gets the same
answers in a structured form with nothing decorative in it, and can tell from how the
command ended whether it worked and, if not, what kind of thing went wrong. Long work shows
that it is progressing and can be interrupted; nothing ever waits forever for an answer
nobody is there to give.

**Why this priority**: A command-line tool is used by hand and by scripts in equal measure,
and the two need different things from the same command. Getting this right once, here,
means every feature gets it right.

**Independent Test**: Run a listing at a terminal and confirm it is aligned and coloured;
pipe it to a file and confirm there is no colour; ask for the structured form and confirm
it is valid and complete; provoke each kind of failure and confirm each ends differently.

**Acceptance Scenarios**:

1. **Given** a command run at a terminal, **When** it answers, **Then** the result is laid
   out for reading, sized to the terminal, with colour used to help and never as the only
   way to tell things apart.
2. **Given** a command whose output goes to a file or another program, **When** it answers,
   **Then** there is no colour and no decoration, without the researcher having to ask.
3. **Given** any command, **When** the researcher asks for the structured form, **Then** the
   whole result is given in one well-formed document, with the same content as the form for
   people.
4. **Given** any command, **When** it succeeds or fails, **Then** results go to one stream
   and messages, warnings, progress, and questions to another, so that a script can read
   results alone.
5. **Given** a command that fails, **When** it ends, **Then** the way it ended tells apart
   at least: invalid input, something not found, a workspace problem, a confirmation
   needed or refused, a check that did not pass, and an operation that could not be
   completed.
6. **Given** a command that fails and the structured form was asked for, **When** it ends,
   **Then** the failure is described in the structured form too, with a stable name for the
   kind of problem.
7. **Given** work that takes more than about a quarter of a second, **When** it runs at a terminal, **Then**
   the researcher sees that it is progressing, and the indication disappears when done.
8. **Given** work in progress, **When** the researcher interrupts it, **Then** it stops
   within a second, leaves the workspace in a valid state, and says what was and was not done.
9. **Given** the researcher's preferences for colour and for plain symbols, **When** they
   set them, **Then** every command follows them, and the common conventions for turning
   colour off are respected.
10. **Given** a message about a problem, **When** it is shown, **Then** it says what
    happened, that nothing was changed if so, and what the researcher can do next.
11. **Given** a long listing, **When** it is shown, **Then** it is limited to a stated
    number with a count of the rest, and the researcher can ask for more.
12. **Given** dates and times, **When** they are shown, **Then** they are in one unambiguous
    form everywhere, and times are shown in the researcher's local time with the zone when
    it matters.

---
