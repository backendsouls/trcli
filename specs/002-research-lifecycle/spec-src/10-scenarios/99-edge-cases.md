### Edge Cases

- A task is attached to a record that is later deleted: the task is kept, shown as detached,
  and names what it used to belong to.
- Tasks depend on each other in a loop: the tool rejects the dependency.
- A backup is requested while a run is in progress: the backup records the run as in
  progress and says so.
- A backup or export would exceed the available space: the tool stops before writing
  anything and reports how much is needed.
- A restore is attempted from a backup made by a newer version of the tool: the tool
  refuses and explains.
- A notebook entry is written with a past date: it is accepted, and both the stated date and
  the time it was actually written are kept.
- A pre-registration is frozen with required sections empty: the tool refuses.
- An approval's dates change after a run acknowledged its absence: the acknowledgement
  remains in the record.
- A grant's spending exceeds a budget line: it is accepted and flagged as overspent.
- Amounts in different currencies are totalled: totals are shown per currency and never
  added across currencies.
- An extension or custom record type uses a name that a built-in one already uses: the tool
  rejects it.
- Two members delete and edit the same record: this is a conflict, shown like any other.
- Members' clocks disagree: the order of changes does not depend on the clocks alone, and
  the tool reports when a clock is clearly wrong.
