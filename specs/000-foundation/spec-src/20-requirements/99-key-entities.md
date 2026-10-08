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
