### Edge Cases

- The simulator does not accept a seed, or ignores it: the repeatability check fails, and
  the tool says that replications of this model cannot be claimed to be repeatable.
- Two replications of a scenario end up with the same seed because the researcher gave it
  twice: the tool rejects the second.
- A replication's outputs differ on rerun with the same seed because the simulator version
  changed: the tool reports the version difference first, since that explains it.
- The model declares an output that the simulator did not produce: the replication is
  recorded as failed for that output and is left out of that output's summary only.
- An output series is empty, or ends before the warm-up does: its summary is shown as not
  available, and the replication is reported.
- An output series is very large: the tool records where it is and its summary, and never
  holds the series itself.
- A replication ends before the horizon (the model stopped early): it is recorded as
  finished early, with the simulated time reached, and is left out of summaries unless the
  researcher includes it.
- Every replication of a scenario gives exactly the same value: the spread is zero, the
  interval has no width, and the tool mentions that the model may not be using the seed.
- Outputs contain values that are not finite: they are reported and left out of summaries.
- The baseline scenario is deleted or another is marked baseline: comparisons already
  recorded as results keep the baseline they used; new comparisons use the new one.
- A scenario is derived from a scenario that is then deleted: it keeps its values and is no
  longer derived from anything.
- Scenarios are derived from each other in a loop: the tool rejects it.
- The model moves to a new version while a batch is paused: the batch finishes with the
  version it started with.
- Verification cases exist for an input the new model version removed: they are marked as
  not applicable to that version.
- A validation uses a dataset that later changes: the validation keeps the dataset version
  it used and is marked as based on an earlier version.
- A sensitivity analysis is asked for an input that has no range: the tool rejects it and
  asks for a range.
- An ensemble's uncertainties cover an input that scenarios also set: within the ensemble
  the drawn value is used, and the tool says so.
- Simulated time has no natural unit (steps or ticks): "steps" is used as the unit.
- A model has no random numbers and no time (a single calculation): scenarios, runs,
  verification, and sensitivity analysis still apply; replications, warm-up, and horizon do
  not, and the tool does not ask for them.
- An experiment's kind is changed away from "simulation" after scenarios exist: the tool
  refuses until the model is removed from it.
