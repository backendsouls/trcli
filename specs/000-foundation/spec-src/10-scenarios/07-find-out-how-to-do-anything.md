### User Story 7 - Find out how to do anything (Priority: P7)

A researcher who does not know how to do something asks the tool. Every command explains
itself: what it does, what it takes, with an example. Every feature has a written guide
with worked examples that are known to be correct because they are tried against the tool
itself. When something goes wrong, the message points to the way forward.

**Why this priority**: A tool this broad is used a little of each part at a time; nobody
remembers it all. Help that is always there and always right is what makes the breadth
usable.

**Independent Test**: Ask for help on the tool, on a group of commands, and on one command,
and confirm each explains usage with an example; make a typing mistake in a command's name
and confirm a suggestion; pick an example from a usage guide, run it, and confirm it
behaves as written.

**Acceptance Scenarios**:

1. **Given** the tool, **When** the researcher asks for help with no further word, **Then**
   the groups of commands are listed with one line each, and how to learn more.
2. **Given** any command, **When** the researcher asks for help on it, **Then** its purpose,
   its arguments and options with their allowed values and defaults, and at least one
   example are shown.
3. **Given** a mistyped command or option, **When** it is run, **Then** the tool suggests
   the nearest ones that exist.
4. **Given** any feature, **When** the researcher looks for its guide, **Then** there is a
   usage guide with its purpose, its commands, worked examples with what they print, the
   ways it can fail, and how each failure ends.
5. **Given** an example in a usage guide, **When** it is run as written, **Then** it behaves
   as the guide says.
6. **Given** the tool, **When** the researcher asks which version they have, **Then** the
   version is shown, with the workspace format it reads and writes.
7. **Given** the researcher's command shell, **When** they ask for completion support,
   **Then** they obtain it, and it completes command names and options.
8. **Given** a message about a failure, **When** there is an obvious next step, **Then** the
   message names it.
9. **Given** a new workspace with nothing in it, **When** the researcher asks what to do
   first, **Then** the tool suggests the first steps.
10. **Given** help text and guides, **When** they are read, **Then** they use the same words
    for the same things as the commands and the messages do.

---
