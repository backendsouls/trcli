#### Parameters, metrics, and sweeps

- **FR-030**: Users MUST be able to declare an experiment's parameters with a name, a kind,
  allowed values or a range, and a default.
- **FR-031**: The system MUST reject a run given an undeclared parameter or a value outside
  what is allowed, and MUST record the default for every parameter not given.
- **FR-032**: Users MUST be able to start a sweep over sets of parameter values; the system
  MUST show the number of runs before starting, require confirmation, and group the runs as
  one sweep.
- **FR-033**: Users and automated steps MUST be able to record named metrics for a run;
  values MUST be finite numbers, and repeated values under one name MUST be kept as an
  ordered series.
- **FR-034**: Users MUST be able to compare runs side by side by parameters, metrics,
  status, condition, and software versions, sort by any of them, and see which parameters
  differ.
- **FR-035**: Users MUST be able to find the best run of an experiment or sweep by a chosen
  metric and direction, mark a run as best with a note, and be told when a later run beats
  the marked one.
- **FR-036**: Users MUST be able to retry the failed runs of a sweep without repeating the
  successful ones.
- **FR-037**: The system MUST summarize runs by condition, showing per metric the number of
  replicates, the mean, and the spread.
