#### Methodologies

- **FR-011**: Users MUST be able to create, list, view, update, and delete methodologies,
  each with a name unique in the workspace, a purpose, a description, the kind of work it
  is for, when to use it and when not, what it needs beforehand, what it produces, its
  pitfalls, its limitations, and the references that define or describe it.
- **FR-012**: Users MUST be able to write a methodology's procedure as ordered steps, each
  with what to do, an optional check that it was done right, and optional notes.
- **FR-013**: Users MUST be able to record new versions of a methodology with a note of what
  changed; earlier versions MUST remain viewable, and versions MUST be comparable step by
  step.
- **FR-014**: Users MUST be able to adapt a methodology into a new one that records what it
  was adapted from and each difference with its reason; the system MUST show a
  methodology's lineage back to its original and MUST reject a methodology adapted from
  itself, directly or indirectly.
- **FR-015**: Experiments, literature reviews, and models MUST be linkable to the
  methodology they follow, and the version in effect MUST be recorded with the work and
  with each run.
- **FR-016**: The system MUST show where a methodology is used, by version, and MUST refuse
  to delete one that is in use.
- **FR-017**: Users MUST be able to create the steps of an experiment's pipeline from a
  methodology's steps.
