<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Foundation and Architecture

**Feature Branch**: `000-foundation`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add a new spec 000-foundation, the first one for the project foundation and architecture"

## Overview

TRCLI (The Research CLI) is specified as many features — literature, experiments,
manuscripts, projects, people, and more — each in its own specification. All of them stand
on the same ground: a workspace to keep things in, one way of naming and finding records,
one way of checking input, one way of answering, one record of everything that happened.
If each feature built that ground for itself, the tool would feel like fifteen tools.

This specification is that ground. It is numbered 000 because it is built first and
everything else depends on it. It has two readers:

- the **researcher**, for whom it defines how TRCLI behaves whatever they are doing in it;
- the **contributor**, for whom it defines what every feature gets for free and what every
  feature must respect.

It **takes over** the foundational content that was written into the first feature
specification before the product was divided, which now points here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 1 | Creating and using a workspace; behaviour shared by all records |
| `specs/001-research-workspace`, User Story 10 | The audit trail and local telemetry |
| `specs/001-research-workspace`, FR-001 to FR-015, FR-051 to FR-055 | Workspace, common behaviour, input validation, audit, telemetry |
| `specs/001-research-workspace/contracts` | CLI conventions, output and exit codes, configuration |

### What "architecture" means in this specification

A specification says what a system must do, not how it is built. The architecture appears
here as the **qualities every part of TRCLI must have** and the **rules every feature must
follow** — stated so that they can be tested — and not as a choice of technology. The
technology, the code structure, and the reasons for them belong to this specification's
implementation plan.

### The ground everything stands on

```text
 Researcher ──▶ one command grammar ──▶ any feature (references, runs, tasks, …)
                                             │
            ┌────────────────────────────────┼────────────────────────────────┐
            ▼                                ▼                                ▼
   Input is checked first          The record is changed            The answer comes back
   every value, all problems       all of it or none of it          for people or for programs
   together, nothing changed       with its audit entry             with a meaningful exit code
            │                                │                                │
            └──────────────── in one Workspace, found from where you stand ───┘
                 records with handles · tags · notes · links · guarded deletion
                 settings in layers · help everywhere · local, offline, yours
```

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Create and find a workspace (Priority: P1)

A researcher creates a workspace in a directory of their choice and gives it a name. From
then on, wherever they stand inside that directory or beneath it, TRCLI finds the workspace
by itself. Outside any workspace, it says so and explains what to do. They can keep
several workspaces, entirely separate from one another.

**Why this priority**: Nothing can be stored before there is somewhere to store it. This is
the first thing every user does and the first thing that must work.

**Independent Test**: Create a workspace, run a command from a subdirectory three levels
down and confirm the workspace is found; run a command from outside and confirm the
explanation; try to create a second workspace in the same place and confirm it is refused.

**Acceptance Scenarios**:

1. **Given** a directory with no workspace, **When** the researcher creates one there with a
   name, **Then** a workspace is created and reported as ready, with where it is.
2. **Given** a workspace, **When** the researcher runs a command from any directory beneath
   it, **Then** the workspace is found without being named.
3. **Given** a directory outside any workspace, **When** the researcher runs a command that
   needs one, **Then** the tool says there is no workspace here, explains how to create or
   point to one, creates nothing, and changes nothing.
4. **Given** a directory that already holds a workspace, **When** the researcher tries to
   create one there, **Then** the tool refuses and leaves the existing workspace untouched.
5. **Given** a workspace, **When** the researcher views it, **Then** its name, description,
   location, format version, and the number of records of each kind are shown.
6. **Given** a workspace, **When** the researcher changes its name, description, or the name
   under which their own actions are recorded, **Then** the change is saved.
7. **Given** two workspaces in different directories, **When** the researcher works in one,
   **Then** nothing of the other is visible or affected.
8. **Given** a workspace elsewhere, **When** the researcher names it explicitly for one
   command, or for a session, **Then** that workspace is used whatever the current
   directory.
9. **Given** a researcher who wants one workspace reachable from anywhere, **When** they set
   it as their default, **Then** it is used whenever no workspace is found from where they
   stand, and the tool says which workspace it used.
10. **Given** a workspace nested inside another workspace's directory, **When** a command is
    run inside the inner one, **Then** the nearest workspace is used.
11. **Given** a workspace created by a newer version of the tool, **When** an older version
    opens it, **Then** it refuses to change anything and explains why.
12. **Given** a workspace created by an older version, **When** a newer version opens it,
    **Then** the tool says an upgrade is needed, changes nothing until asked, and keeps a
    copy of the workspace as it was before upgrading.
13. **Given** a workspace whose stored data has been damaged or edited by hand into an
    invalid state, **When** it is opened, **Then** the tool reports what is wrong and where,
    and does not overwrite it.
14. **Given** a missing or empty name, or a directory that cannot be written to, **When**
    the researcher creates a workspace, **Then** the tool rejects it and explains why.

---

### User Story 2 - Work with any record the same way (Priority: P2)

Whatever a researcher is working with — a reference, an experiment, a task, a person — the
same handful of actions works the same way: add one, list them, look at one, change it,
remove it; give it tags and notes; link it to any other record. Each record has a short
name they can type, and typing just the beginning is enough. Once they have learned one
kind of record, they know them all.

**Why this priority**: Consistency is what makes a tool with dozens of record kinds
learnable. It is also what keeps the workspace whole: links that are visible from both
ends, and deletions that never leave something pointing at nothing.

**Independent Test**: With two different kinds of record, add one of each, refer to each by
the start of its short name, tag one, add a note to the other, link them, list each kind
with a filter and a search, and delete one — confirming the link is listed first and gone
afterwards.

**Acceptance Scenarios**:

1. **Given** any kind of record, **When** the researcher adds one, **Then** it receives a
   short name that says what kind it is, is unique in the workspace, and never changes.
2. **Given** a record's short name, **When** the researcher types only its beginning,
   **Then** the record is found if the beginning matches exactly one record.
3. **Given** a beginning that matches several records, **When** it is used, **Then** the
   tool lists the matches and changes nothing.
4. **Given** a short name that matches nothing, **When** it is used, **Then** the tool says
   it was not found and suggests close matches where there are any.
5. **Given** any kind of record, **When** the researcher lists them, **Then** they can
   filter, sort, search by words in the main text, and limit how many are shown, in the
   same way for every kind.
6. **Given** any record, **When** the researcher changes it, **Then** only what they named
   is changed.
7. **Given** any record, **When** the researcher adds tags or a dated note, **Then** these
   are shown with the record, and records of any kind can be found by tag.
8. **Given** any two records, of the same or different kinds, **When** the researcher links
   them, optionally saying how they relate, **Then** the link is visible from both.
9. **Given** a record that other records refer to, **When** the researcher deletes it,
   **Then** the tool lists what refers to it and requires explicit confirmation.
10. **Given** a confirmed deletion, **When** it completes, **Then** no link, tag, or note
    still points to the deleted record.
11. **Given** a deletion that a feature forbids — because history would be lost — **When**
    it is attempted, **Then** the tool refuses, says what stands in the way, and offers the
    alternative that feature provides.
12. **Given** a command that asks for confirmation, **When** it is run where nobody can
    answer, **Then** it fails at once without changing anything, unless the researcher
    stated beforehand that the answer is yes.
13. **Given** the same action on two kinds of record, **When** the researcher compares how
    it is invoked, **Then** the words, the order, and the options are the same.
14. **Given** a list with nothing in it, **When** it is shown, **Then** the tool says so in
    one line and does not treat it as a failure.

---

### User Story 3 - Have every input checked before anything changes (Priority: P3)

Whatever a researcher gives the tool — typed on the command line, answered to a question,
read from a file, brought in from elsewhere — is checked before anything is stored or
done. If something is wrong, nothing changes, and they are told everything that is wrong
at once: which value, why, and what would be right.

**Why this priority**: A research record that contains a wrong date or a broken link is
worse than no record. Checking at the door, completely and helpfully, is what lets a
researcher trust what is inside.

**Independent Test**: Give a command three invalid values at once and confirm all three are
reported together, each named with what is expected, and that nothing was stored; bring in
a file with some invalid entries and confirm each is reported and the valid ones handled as
that feature specifies.

**Acceptance Scenarios**:

1. **Given** a command with an invalid value, **When** it is run, **Then** nothing is stored
   or done, and the tool names the value, says what is wrong with it, and says what is
   expected.
2. **Given** a command with several invalid values, **When** it is run, **Then** all of them
   are reported together, not only the first.
3. **Given** a value that must be of a certain form — a date, a number, an address, an
   identifier — **When** it is not, **Then** the message shows an example of a valid one.
4. **Given** a value that must be one of a set, **When** it is not, **Then** the valid
   choices are listed.
5. **Given** a value that names another record or a file, **When** that record or file does
   not exist, **Then** this is reported like any other invalid value.
6. **Given** text that is empty where it is required, longer than its limit, or contains
   characters that cannot be stored or shown, **When** it is given, **Then** it is rejected
   with the field and the limit.
7. **Given** values that are each valid and together are not — an end before a start —
   **When** they are given, **Then** the pair is reported with the rule it breaks.
8. **Given** input read from a file or brought in from another tool, **When** it is
   processed, **Then** every entry is checked by the same rules as typed input.
9. **Given** an invalid command — an unknown action, a missing required value, an option
   that does not exist — **When** it is run, **Then** the tool says what is wrong and shows
   how the command is used.
10. **Given** something that is unusual but allowed, **When** it is given, **Then** the tool
    warns and proceeds, or asks first when the feature says so — and never silently changes
    the value.
11. **Given** free text in any language and script, **When** it is given where free text is
    expected, **Then** it is accepted and kept exactly as written.
12. **Given** invalid input, **When** the command ends, **Then** it reports failure in a way
    that a calling program can tell apart from success and from other kinds of failure.

---

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

### User Story 5 - Set the tool up my way (Priority: P5)

A researcher adjusts how TRCLI behaves: their preferences in one place, valid for
everything they do; choices that belong to a particular workspace kept with that workspace;
and, for one session or one command, an override. At any time they can see every setting,
its value, and where that value comes from.

**Why this priority**: Every feature has things worth adjusting, and without one place and
one rule for them each would invent its own. It comes after the behaviours that need no
adjusting to be useful.

**Independent Test**: Set a preference for yourself, a different value for one workspace,
and an override for one command; list settings and confirm each value and its source; set
an invalid value and confirm it is refused.

**Acceptance Scenarios**:

1. **Given** a setting, **When** the researcher sets it for themselves, **Then** it applies
   in every workspace they use.
2. **Given** a setting, **When** the researcher sets it for one workspace, **Then** it
   applies there only, and takes the place of their personal value.
3. **Given** a setting, **When** the researcher overrides it for a session or for one
   command, **Then** the override wins for that session or command and nothing stored is
   changed.
4. **Given** settings from several places, **When** the researcher lists them, **Then**
   every setting is shown with its value in effect and where that value comes from.
5. **Given** a setting, **When** the researcher asks about it, **Then** its meaning, its
   allowed values, and its default are shown.
6. **Given** an invalid value or a setting that does not exist, **When** the researcher sets
   it, **Then** the tool refuses and says what is allowed.
7. **Given** a settings file containing an invalid value or an unknown setting, **When** any
   command is run, **Then** the tool names the file, the setting, and what is expected, and
   does not run with a partly valid configuration.
8. **Given** a setting that only makes sense for a workspace, or only for a person, **When**
   it is found in the other place, **Then** it is ignored with a warning.
9. **Given** a setting, **When** the researcher removes their value, **Then** the next value
   in order applies again.
10. **Given** no settings at all, **When** the tool is used, **Then** every setting has a
    sensible default and nothing needs configuring first.
11. **Given** settings, **When** they are stored, **Then** nothing secret is ever among them.

---

### User Story 6 - Know everything that happened (Priority: P6)

Everything that changes in a workspace is written down as it happens: what was done, to
what, by whom, when, and what exactly changed. A researcher — or a supervisor, a reviewer,
an auditor — can look through this record, check that nobody has tampered with it, and hand
over a report of it. Separately, the tool keeps simple telemetry about its own use for the
researcher's benefit, which stay on their machine and can be turned off.

**Why this priority**: The record of what happened is what makes research in the tool
accountable, and several features are built on it (reports, sync, reproducibility). It is
written from the first change onward, so it belongs to the foundation; looking through it
becomes valuable once there is something to look at.

**Independent Test**: Make a known series of changes, look up the record for one item and
for a range of dates and confirm every change is there with what changed; alter one entry
outside the tool and confirm the check reports it; turn telemetry off and confirm none
are recorded while the record of changes continues.

**Acceptance Scenarios**:

1. **Given** any record is created, changed, or deleted, **When** the action completes,
   **Then** an entry records the action, the record, who acted, when, and what changed.
2. **Given** a change that fails or is refused, **When** it ends, **Then** neither the change
   nor an entry for it exists; a change and its entry are made together or not at all.
3. **Given** the record of what happened, **When** the researcher filters it by item, kind
   of item, who acted, kind of action, or range of dates, **Then** only matching entries
   are shown, newest first.
4. **Given** an entry about a record that has since been deleted, **When** it is shown,
   **Then** it still says what the record was called.
5. **Given** the record of what happened, **When** an entry is altered or removed outside
   the tool, **Then** a check reports that it has been tampered with and where the first
   problem is.
6. **Given** the tool, **When** the researcher looks for a way to change or remove an entry,
   **Then** there is none.
7. **Given** the record of what happened, **When** the researcher exports a range of it,
   **Then** a report is produced that can be handed to someone else.
8. **Given** use of the tool, **When** telemetry is on, **Then** which commands were
   used, how long they took, and whether they succeeded are recorded locally, and the
   researcher can view them.
9. **Given** telemetry, **When** the researcher turns it off, **Then** no more is
   recorded, and the record of what happened continues unchanged.
10. **Given** telemetry, **When** anything is recorded, **Then** it stays in the workspace
    and is never sent anywhere.
11. **Given** something secret — a credential, a passphrase — **When** anything is recorded,
    **Then** it is never included.
12. **Given** who acted, **When** it is recorded, **Then** it is the name the researcher set
    for themselves, or otherwise the name they are known by on their machine.

---

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

### User Story 8 - Add a feature without breaking the whole (Priority: P8)

A contributor adds a new kind of record — or a whole new feature — to TRCLI. They declare
what is particular to it, and it arrives with everything records have in common: a short
name, the shared actions, tags, notes, links, guarded deletion, input checking, both output
forms, an entry in the record of what happened, a place among projects. Before their work
is accepted, the same checks that every part passed are applied to it, on every supported
system.

**Why this priority**: TRCLI is specified as many features delivered over time by more than
one person. The foundation must make the consistent thing the easy thing, and must catch
what is inconsistent before it reaches a researcher. It is last because the contributor is
served once the researcher-facing ground exists.

**Independent Test**: Add a trivial new kind of record following the contributor guide,
and confirm that — without writing code for them — it can be listed, tagged, noted, linked,
deleted with confirmation, shown in both output forms, and that its changes appear in the
record of what happened; then confirm the acceptance checks reject a change that lacks
tests, documentation, or a usage guide.

**Acceptance Scenarios**:

1. **Given** a new kind of record, **When** a contributor defines what is particular to it,
   **Then** it has the behaviour shared by all records without that behaviour being written
   again.
2. **Given** a new command, **When** it is added, **Then** it follows the one command
   grammar, the shared options, the output forms, and the meanings of how a command ends.
3. **Given** a feature a researcher will use, **When** it is proposed, **Then** it is
   accepted only if every acceptance scenario of its specification is an automated check
   run against the tool as a researcher would run it.
4. **Given** any input a new command accepts, **When** the feature is proposed, **Then** it
   is accepted only if there are checks that invalid forms of that input are rejected.
5. **Given** new code, **When** it is proposed, **Then** it is accepted only if every part
   of it is documented for a human reader, and the feature has its usage guide.
6. **Given** a change, **When** it is checked, **Then** the full set of checks runs on every
   supported system, and the change is accepted only if all pass on all of them.
7. **Given** the rules of the research itself — what a valid record is, what may follow
   what — **When** code is organized, **Then** those rules depend on nothing about how
   records are stored, shown, or invoked, and this separation is checked automatically.
8. **Given** a part of the tool that reaches outside it — storage, files, the clock, another
   program, a service — **When** it is used by a feature, **Then** it can be replaced by a
   stand-in for testing without changing the feature.
9. **Given** a new version that changes how workspaces are stored, **When** it is prepared,
   **Then** it includes the step that brings an existing workspace forward, and a check
   that a workspace from each earlier version is brought forward without loss.
10. **Given** two features developed separately, **When** both are present, **Then** neither
    needs to know how the other is built: they refer to each other's records only by kind
    and name.

Two further expectations of contributors are rules rather than scenarios, because no
automated check can observe them: that tests are written and seen to fail before the code
(FR-073), and that a newcomer can build, check, and add a simple command within an hour
(FR-076, SC-015).

---

### Edge Cases

- A command is run outside any workspace: the tool says so and explains how to create or
  locate one, without creating anything.
- A workspace is created where one already exists: the tool refuses and leaves the existing
  workspace untouched.
- The workspace's directory is moved or renamed: the workspace keeps working from its new
  place; nothing inside depends on where it is.
- The workspace is on a disk that is full or read-only: commands that only read keep
  working; commands that change say why they cannot, and nothing is half-written.
- Two commands are run at the same moment in the same workspace: both complete, one after
  the other, or the second says the workspace is busy and can be tried again; the workspace
  is never left inconsistent.
- The machine loses power in the middle of a change: on the next use the workspace is as it
  was before the change or after it, never in between.
- A short name is typed in a different letter case: it is found.
- The beginning of a short name matches records of different kinds where only one kind makes
  sense for the command: only that kind is considered.
- A record is linked to itself, or the same link is made twice: the tool refuses.
- A tag is given in mixed case or with spaces: it is normalized by a stated rule or
  rejected with the rule; it is never silently altered beyond that rule.
- A text field is empty, far too long, or contains characters that cannot be stored or
  displayed: the tool rejects it with the field name and the limit.
- A date is impossible or in the wrong form; a number is negative where only positive values
  make sense: the tool rejects the value and shows an example of a valid one.
- A deletion would leave other records pointing at nothing: the tool lists the dependents
  and requires confirmation; nothing is left dangling afterwards.
- A confirmation is asked and the researcher answers nothing, or anything but yes: the
  answer is no.
- Output is piped into a program that stops reading early: the tool ends quietly.
- The terminal is very narrow, or its width cannot be known: output is still readable, with
  long values shortened.
- The terminal cannot show colour or special symbols: plain characters are used and nothing
  becomes ambiguous.
- A settings file is missing, empty, or unreadable: defaults apply for a missing or empty
  file; an unreadable one is an error that names the file.
- The same setting is given in every place at once: the order of precedence decides, and the
  listing shows which place won.
- The clock of the machine is wrong or changes: the order of the record of what happened
  does not depend on the clock alone.
- The name of who acted cannot be determined: a stated placeholder is recorded, and the tool
  suggests setting a name.
- The record of what happened has grown very large: looking through it stays quick, and the
  check for tampering reports its progress.
- A command is interrupted while it is asking a question: nothing is changed.
- The tool is run with no arguments at all: it shows the short help, and does not fail.
- A feature is not yet present in the version in use: its commands are unknown commands,
  with the usual suggestion; the workspace is unaffected.

## Requirements *(mandatory)*

### Functional Requirements

#### Workspace

- **FR-001**: Users MUST be able to create a workspace in a directory with a name and an
  optional description, and to view and change those details and the name under which
  their own actions are recorded.
- **FR-002**: Creating a workspace where one exists MUST be refused and MUST leave the
  existing workspace untouched.
- **FR-003**: The system MUST find the workspace to use, in this order: the one named for
  the command; the one named for the session; the nearest one at or above the current
  directory; the user's default workspace. It MUST say which workspace it used when that
  was the default.
- **FR-004**: When no workspace is found, every command that needs one MUST fail without
  creating or changing anything and MUST explain how to create or point to one.
- **FR-005**: Each workspace MUST keep its records separate from every other workspace, and
  MUST keep working when its directory is moved or renamed.
- **FR-006**: Viewing a workspace MUST show its name, description, location, format version,
  and the number of records of each kind.
- **FR-007**: The system MUST record the format version of every workspace. It MUST refuse
  to change a workspace whose format is newer than it understands. For an older format it
  MUST say an upgrade is needed, MUST NOT upgrade without being asked, MUST keep a copy of
  the workspace as it was before upgrading, and MUST leave the workspace unchanged if the
  upgrade fails.
- **FR-008**: When a workspace's stored data is damaged or invalid, the system MUST report
  what is wrong and where, and MUST NOT overwrite it.
- **FR-009**: Every change to a workspace MUST be made completely or not at all, including
  when the command is interrupted or the machine stops; two commands run at the same moment
  MUST NOT leave the workspace inconsistent.

#### Records

- **FR-010**: Every record of every kind MUST have a short name that identifies its kind, is
  unique within the workspace, never changes, and is never given to another record.
- **FR-011**: Wherever a record is named, users MUST be able to give any beginning of its
  short name that matches exactly one record of a kind the command accepts, in any letter
  case; an ambiguous beginning MUST list the matches and change nothing; no match MUST say
  so and suggest close ones.
- **FR-012**: For every kind of record, users MUST be able to create, list, view, update,
  and delete records, unless the specification that owns the kind says a record cannot be
  changed or removed — in which case the system MUST say what to do instead.
- **FR-013**: These actions MUST be invoked in the same way for every kind: the same words,
  the same order, the same options for filtering, sorting, searching, and limiting.
- **FR-014**: Updating a record MUST change only what the user named.
- **FR-015**: Users MUST be able to attach tags and dated notes to any record, find records
  of any kind by tag, and see every tag in use with how many records carry it.
- **FR-016**: Users MUST be able to link any two records, optionally stating how they
  relate; a link MUST be visible from both records; a record MUST NOT be linked to itself
  and the same link MUST NOT exist twice.
- **FR-017**: Before deleting a record, the system MUST list the records that refer to it and
  MUST require explicit confirmation. After a deletion no link, tag, or note MUST point to
  the deleted record.
- **FR-018**: The system MUST record, for every record, when it was created and when it was
  last changed.
- **FR-019**: A record of one feature MUST be able to refer to a record of any other feature
  by its kind and identity alone.

#### Input validation

- **FR-020**: The system MUST validate every value a user supplies — typed, answered to a
  question, read from a file, or brought in from another source — before anything is stored
  or acted upon.
- **FR-021**: Validation MUST cover the presence of required values, form, length, allowed
  values, numeric and date ranges, rules between values, and whether records and files that
  are named exist.
- **FR-022**: When input is invalid, the system MUST change nothing, MUST name each invalid
  value, and MUST say what is wrong and what is expected, with an example of a valid value
  where a form is required and the list of choices where a set is.
- **FR-023**: When several values are invalid, the system MUST report all of them together.
- **FR-024**: An invalid command — an unknown action or option, a missing required value —
  MUST be reported with how the command is used and, where one exists, the nearest valid
  command or option.
- **FR-025**: The system MUST NOT silently change a value a user supplied, beyond stated
  normalization rules; something unusual but allowed MUST be met with a warning, or with a
  question where the owning specification says so.
- **FR-026**: Free text MUST be accepted in any language and script and kept exactly as
  written; control characters other than line breaks and tabs MUST be rejected.
- **FR-027**: Invalid input MUST end the command in a way a calling program can tell apart
  from success and from every other kind of failure.

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

#### Settings

- **FR-039**: Users MUST be able to set, view, and remove settings for themselves (applying
  in every workspace) and for a workspace (applying there), and to override any setting for
  a session or a single command.
- **FR-040**: The value in effect MUST be decided in this order, later winning: the default;
  the user's value; the workspace's value; the session's override; the command's override.
- **FR-041**: Users MUST be able to list every setting with its value in effect and where
  that value comes from, and to see for any setting its meaning, allowed values, and
  default.
- **FR-042**: Every setting MUST have a default with which the tool is usable; nothing MUST
  need configuring before first use.
- **FR-043**: Every setting MUST be validated when it is set and when it is read; an invalid
  value or an unknown setting, in a command or in a stored file, MUST be an error that names
  where it is and what is expected, and the tool MUST NOT run with a partly valid
  configuration.
- **FR-044**: A setting that applies only to a workspace, or only to a person, MUST be
  ignored with a warning when found in the other place.
- **FR-045**: Settings MUST NOT hold secrets.

#### Audit trail and local telemetry

- **FR-046**: The system MUST record an audit entry for every creation, change, and
  deletion of any record, and for every other action a specification names (imports,
  exports, runs, confirmations), stating the action, the record with its name at the time,
  the actor, the moment, and what changed.
- **FR-047**: A change and its audit entry MUST be made together or not at all.
- **FR-048**: Audit entries MUST NOT be editable or removable through the tool, and the
  system MUST be able to detect that the trail has been altered or shortened outside the
  tool, reporting where the first problem is.
- **FR-049**: Users MUST be able to query the audit trail by record, kind of record, actor,
  action, and range of dates, newest first, and to export the result as a report.
- **FR-050**: The actor MUST be the name the user set for themselves, or otherwise the name
  they are known by on their machine; when neither can be determined a stated placeholder
  MUST be recorded.
- **FR-051**: The order of the audit trail MUST NOT depend on the machine's clock alone.
- **FR-052**: The system MUST record local telemetry about its own use — which commands were
  used, how long they took, whether they succeeded — MUST let users view it, and MUST let
  users turn it off, after which none is recorded while the audit trail continues.
- **FR-053**: Telemetry MUST stay inside the workspace and MUST never be transmitted.
- **FR-054**: Secrets MUST never appear in the audit trail, in telemetry, in messages, or
  in any output.

#### Help and documentation

- **FR-055**: The tool run with no arguments, and asked for help with no further word, MUST
  list its groups of commands with one line each and how to learn more, and MUST NOT fail.
- **FR-056**: Every command MUST have built-in help stating its purpose, its arguments and
  options with allowed values and defaults, and at least one example.
- **FR-057**: Every feature MUST have a usage guide covering its purpose, its commands and
  options, worked examples with what they print, the ways it can fail, and how each failure
  ends.
- **FR-058**: The examples in usage guides MUST be checked against the tool automatically,
  so that a guide that no longer matches the tool is detected.
- **FR-059**: The tool MUST report its version and the workspace format it reads and writes,
  and MUST provide completion support for the commonly used command shells.
- **FR-060**: Help, guides, messages, and commands MUST use the same word for the same thing.
- **FR-061**: In a workspace with nothing in it, the tool MUST be able to suggest first
  steps.

#### Qualities every part must have

- **FR-062**: The tool MUST run on the three desktop operating systems in common use among
  researchers, behave the same on each, and need nothing else to be installed.
- **FR-063**: Every capability MUST work without a network connection, except those whose
  purpose is to reach a service, which MUST say so and MUST leave everything else working.
- **FR-064**: Nothing a user stores MUST leave their machine except through a command whose
  stated purpose is to send it, after the user has set that up.
- **FR-065**: A simple command MUST answer in under a tenth of a second, and listing or
  searching the records of a large workspace MUST remain quick; the figures are in the
  success criteria.
- **FR-066**: The tool MUST never leave a file it was writing half-written, and MUST never
  delete or alter a user's own files unless a command's stated purpose is to do so.
- **FR-067**: Names, titles, and text in any language and script MUST be stored, shown,
  searched, and sorted correctly; searching and sorting MUST ignore letter case and accents.

#### Rules every feature must follow

- **FR-068**: A new kind of record MUST obtain the behaviour common to all records — short
  name, shared actions, tags, notes, links, guarded deletion, both output forms, audit
  entries — by declaring what is particular to it, without that behaviour being written
  again.
- **FR-069**: Every command MUST follow one grammar, the shared options, the two output
  forms, and the shared meanings of how a command ends.
- **FR-070**: The rules of the research itself MUST NOT depend on how records are stored,
  shown, or invoked; this separation MUST be checked automatically.
- **FR-071**: Everything a feature uses that reaches outside the tool — storage, files, the
  clock, the generation of identifiers, other programs, services — MUST be replaceable by a
  stand-in for testing without changing the feature.
- **FR-072**: Features MUST refer to one another's records only by kind and identity; no
  feature may depend on how another is built.
- **FR-073**: A change MUST be accepted only when: tests for it exist and failed before it;
  every acceptance scenario of a user-facing feature is an automated check run against the
  tool as a user runs it; every accepted input has checks for its invalid forms; every part
  of the code is documented for a human reader; and the feature has its usage guide.
- **FR-074**: The full set of checks MUST run on every supported operating system for every
  proposed change, and a change MUST be accepted only when all pass on all of them.
- **FR-075**: A version that changes how workspaces are stored MUST include the step that
  brings existing workspaces forward and a check that a workspace of every earlier format
  is brought forward without loss.
- **FR-076**: There MUST be a contributor guide that lets a newcomer build the tool, run
  every check, and add a simple command.

### Key Entities *(include if feature involves data)*

- **Workspace**: Everything a researcher keeps in one place, found from the directory they
  stand in. Has a name, a description, a format version, and settings of its own.
- **Record**: Anything a feature stores: a reference, a task, an experiment. Has a kind, a
  short name, the moments it was created and last changed, and whatever its own
  specification gives it.
- **Short Name** *(handle)*: What a researcher types to mean one record: a mark of its kind
  and a short code. Unique, permanent, and usable by any unambiguous beginning.
- **Tag**: A word placed on records of any kind, to find them together.
- **Note**: A dated remark attached to a record.
- **Link**: A relation between two records of any kinds, optionally named, seen from both.
- **Setting**: An adjustable behaviour with a meaning, allowed values, a default, and a
  place where each value in effect comes from: the tool, the person, the workspace, the
  session, or the command.
- **Audit Entry**: A permanent record of one action: what, on which record, by whom, when,
  and what changed. Entries form a trail whose integrity can be checked.
- **Telemetry Record**: A local note of one use of the tool: which command, how long, whether it
  succeeded.
- **Outcome**: How a command ended, as one of a fixed set of meanings shared by all
  commands.
- **Problem**: What is reported when a command does not succeed: its kind, by a stable
  name, and the details needed to act on it.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A new user can install the tool, create a workspace, and see it reported as
  ready in under 2 minutes, consulting nothing but the built-in help.
- **SC-002**: From any directory beneath a workspace, 100% of commands find it without being
  told where it is.
- **SC-003**: A user who has learned to add, list, view, change, delete, tag, note, and link
  one kind of record can do the same with any other kind on their first attempt in at least
  90% of cases.
- **SC-004**: 100% of invalid inputs are rejected before any data changes, and every
  rejection names the invalid value and what is expected; when several values are invalid,
  100% of them are named in the one response.
- **SC-005**: 100% of commands offer the structured form, and its content equals that of the
  form for people in 100% of cases.
- **SC-006**: A script can tell success from each kind of failure by how the command ended
  alone, for 100% of commands.
- **SC-007**: No command ever waits for an answer when run where nobody can give one.
- **SC-008**: After any interruption — of the command, the terminal, or the machine — the
  workspace is valid in 100% of cases, with each change either wholly present or wholly
  absent.
- **SC-009**: 100% of creations, changes, and deletions appear in the audit trail, and any
  alteration of the trail made outside the tool is detected.
- **SC-010**: Nothing leaves the user's machine unless a command whose purpose is to send it
  was run after the user set it up; with no such setup, 0 commands send anything.
- **SC-011**: With no network connection, 100% of capabilities that do not exist to reach a
  service work.
- **SC-012**: A simple command answers in under a tenth of a second, and listing or
  searching 10,000 records of one kind answers in under 2 seconds, on a mid-range laptop no
  more than five years old.
- **SC-013**: Every command has built-in help with an example, every feature has a usage
  guide, and 100% of the examples in usage guides behave as written.
- **SC-014**: The tool behaves identically on every supported operating system: 100% of the
  automated checks pass on each.
- **SC-015**: A contributor can add a new kind of record that has all shared behaviour by
  writing only what is particular to it, and a newcomer following the contributor guide can
  build, check, and add a simple command in under 1 hour.
- **SC-016**: 0 changes are accepted without tests that failed first, automated acceptance
  scenarios, checks of invalid input, documentation of every part, and a usage guide.
- **SC-017**: A workspace created by any earlier version is brought forward by any later
  version with 0 records lost.
- **SC-018**: 90% of first-time users complete the primary task of each story on their
  first attempt using only the built-in help.

## Assumptions

- **This specification comes first.** Every other specification of TRCLI depends on it and
  none is built before it. It owns the workspace, the behaviour shared by all records,
  input validation, output and interaction, settings, the audit trail, local telemetry,
  help, and the rules every feature follows.
- **It replaces foundational parts of `specs/001-research-workspace`**: its User Stories 1
  and 10 and its requirements FR-001 to FR-015 and FR-051 to FR-055, which keep pointers.
  The contracts that state these rules precisely — the CLI conventions, output and exit
  codes, and configuration — now live with this specification.
- **"Architecture" is specified here as testable qualities and rules**, in the sense given
  in the overview. The decisions already taken for the implementation — the programming
  language, the storage, the libraries, the layout of the code, and practices such as
  writing tests first, modelling the domain explicitly, and specifying behaviour as
  scenarios — are recorded in the project's constitution and in the implementation plans,
  and will be consolidated in this specification's own plan.
- **The implementation plan written for `specs/001-research-workspace` predates this
  specification.** Its architectural content (the layering, the storage, the handling of
  identifiers, the conventions for testing) is the starting point for this specification's
  plan and is to be moved there; the later rule to keep libraries to a minimum applies to
  it.
- **Single researcher, local workspace.** One person uses a workspace at a time, on their
  own machine, with no accounts, sign-in, or permissions. Two commands may run at the same
  moment (two terminals); two people editing at once is not in scope here.
- **The actor is not authenticated.** The audit trail records the name the researcher set,
  or the name they are known by on the machine; it is evidence of what the tool did, not
  proof of who was at the keyboard.
- **Tamper-evidence, not tamper-proofing.** The audit trail makes alterations detectable;
  someone who rewrites the entire trail consistently is not detected without an outside
  reference, which is out of scope.
- **Telemetry is for the researcher**, about their own use; it is never usage
  reporting to the authors of the tool.
- **Exceptions to "nothing leaves the machine" are owned by the features that make them**:
  looking up a reference by identifier (`specs/005-literature`) and connecting an outside
  service (`specs/010-integrations`). This specification states the rule they must respect.
- **Projects scope records** as specified in `specs/003-research-projects`; full backup,
  restore, and export of a workspace in `specs/002-research-lifecycle`. This specification
  requires only that an upgrade keeps a copy first.
- **The three supported systems are Linux, macOS, and Windows**, in their versions in
  current use.
- **Interface language is English**; content in any language is accepted. Dates are shown in
  an unambiguous international form.
- **Researchers are the users of every story except the last**, whose user is a contributor
  to TRCLI itself. That story is in this specification because the consistency the
  researcher experiences depends on it.
