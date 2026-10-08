#### Experiments

- **FR-001**: Users MUST be able to create, list, view, update, and delete experiments, each
  with a name, objective, description, kind (computational, laboratory, field, survey,
  simulation, other), and status (planned, active, completed, abandoned).
- **FR-002**: Users MUST be able to link an experiment to the hypotheses it tests, to a
  methodology, to dataset versions, to software, and to the staff responsible for it, with
  every link visible from both ends.
- **FR-003**: An experiment's status MUST become "active" when its first run starts.
- **FR-004**: Users MUST be able to filter experiments by status, kind, hypothesis, tag, and
  responsible person, and search them by words in the name and objective.
- **FR-005**: The system MUST refuse to delete an experiment that has runs.
- **FR-006**: Experiments MUST support tags, notes, and free links to any other record, as
  every record of the workspace does.
