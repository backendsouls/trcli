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
