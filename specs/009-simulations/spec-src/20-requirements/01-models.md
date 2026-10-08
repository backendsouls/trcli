#### Models

- **FR-001**: Users MUST be able to create, list, view, update, and delete models, each with
  a name, a purpose, a description, a kind (discrete-event, agent-based, system dynamics,
  equation-based, Monte Carlo, other), the unit of simulated time, and whether it uses
  random numbers.
- **FR-002**: Users MUST be able to declare a model's inputs, each with a name unique in the
  model, a description, a kind of value (number, whole number, yes/no, one of a list, text),
  a unit, a default, and the range or list of allowed values; a default MUST lie within
  what is allowed.
- **FR-003**: Users MUST be able to declare a model's outputs, each with a name unique in
  the model, a description, a unit, and whether it is a single value or a series over
  simulated time.
- **FR-004**: Users MUST be able to record a model's assumptions, each a statement with an
  optional justification and links to references that support it.
- **FR-005**: Users MUST be able to link a model to the simulator software that computes it,
  to methodologies, and to references that describe it.
- **FR-006**: Users MUST be able to record a new version of a model with a note of what
  changed; versions MUST be numbered in sequence, earlier versions MUST remain viewable,
  and users MUST be able to compare two versions by inputs, outputs, and assumptions.
- **FR-007**: Users MUST be able to set the model, at a stated version, that an experiment
  of the kind "simulation" simulates; the system MUST NOT attach a model to an experiment
  of another kind without changing its kind on the user's confirmation.
- **FR-008**: The system MUST refuse to delete a model, or a version of it, that runs refer
  to, and MUST refuse to change an experiment's kind away from "simulation" while it has a
  model.
