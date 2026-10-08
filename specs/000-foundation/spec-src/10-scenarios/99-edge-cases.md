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
