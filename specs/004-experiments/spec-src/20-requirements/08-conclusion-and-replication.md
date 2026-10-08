#### Conclusion and replication

- **FR-048**: Users MUST be able to record a conclusion for an experiment with an outcome
  per tested hypothesis (supported, refuted, inconclusive), the results that support it,
  the limitations, and next steps; recording it MUST set the experiment to "completed".
- **FR-049**: The system MUST refuse to conclude an experiment that has no succeeded run or
  that has a run paused or in progress.
- **FR-050**: When a conclusion names an outcome for a hypothesis, the system MUST offer to
  update that hypothesis's status and MUST do so only when the user accepts.
- **FR-051**: Users MUST be able to abandon an experiment with a required reason, and to
  reopen a completed or abandoned experiment; earlier conclusions MUST be kept in the
  experiment's history.
- **FR-052**: The system MUST refuse to start runs or to change the design or pipeline of a
  completed or abandoned experiment until it is reopened.
- **FR-053**: Users MUST be able to create a replication of an experiment, which copies its
  design, pipeline, parameters, and links, and records the relationship in both directions.
- **FR-054**: Users MUST be able to compare an experiment with its replications, showing
  results of the same name side by side, whether they agree, and any design differences.
- **FR-055**: Users MUST be able to export a report of an experiment containing its
  objective, design, pipeline, runs, results, and conclusion.
