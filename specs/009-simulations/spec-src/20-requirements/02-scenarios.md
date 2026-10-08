#### Scenarios

- **FR-009**: Users MUST be able to create, list, view, update, and delete scenarios of a
  simulation experiment, each with a name unique in the experiment, a description, and a
  value for every input of the model; inputs not given MUST take the model's default.
- **FR-010**: Users MUST be able to mark exactly one scenario of an experiment as the
  baseline.
- **FR-011**: Users MUST be able to derive a scenario from another by stating only the
  inputs that differ; a derived scenario MUST follow later changes to inputs it did not set,
  the system MUST name the scenarios affected before such a change is saved, and MUST
  reject derivation that forms a loop.
- **FR-012**: Each scenario MUST carry a horizon, a warm-up period, a time step where the
  model needs one, and a number of replications; the warm-up MUST be shorter than the
  horizon and the number of replications at least one.
- **FR-013**: The system MUST show, for each scenario, how it differs from the baseline, and
  for each input whether its value is the model's default, inherited, or set in the
  scenario.
- **FR-014**: The system MUST reject a scenario value that is outside the model's allowed
  values, of the wrong kind, or for an input the model does not have, and MUST warn when a
  scenario has the same values as another.
- **FR-015**: When an experiment moves to a model version in which an input a scenario sets
  no longer exists or no longer allows its value, the system MUST list the scenarios
  concerned and MUST refuse to run them until each is resolved.
- **FR-016**: When a scenario with replications is changed, the system MUST warn, keep what
  existing replications recorded, and mark them as belonging to an earlier definition.
- **FR-017**: A scenario MUST be usable wherever `specs/004-experiments` uses a condition:
  to label runs, to group summaries, and in comparisons.
