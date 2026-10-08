#### Replications and seeds

- **FR-018**: Users MUST be able to run a scenario; the system MUST carry out the
  experiment's pipeline once per replication and record with each run its scenario, the
  scenario's input values, replication number, seed, model version, and simulator version.
- **FR-019**: The system MUST assign each replication of a scenario a different seed,
  derived from a base seed recorded with the batch, so that the same base seed yields the
  same seeds; users MUST be able to give the base seed or the exact seeds.
- **FR-020**: The system MUST never use the same seed twice for the same scenario
  definition, except when a replication is deliberately rerun.
- **FR-021**: The system MUST hand the simulator the scenario's input values, horizon,
  warm-up, time step, and seed in a documented way that works for any simulator.
- **FR-022**: Users MUST be able to run several scenarios together; by default the same
  replication number MUST receive the same seed in every scenario, and users MUST be able
  to ask for independent seeds instead.
- **FR-023**: Users MUST be able to add replications to a scenario; numbering MUST continue
  and new seeds MUST be used.
- **FR-024**: Users MUST be able to rerun a past replication with the same scenario values,
  model version, and seed; the new run MUST be linked to the original.
- **FR-025**: An interrupted batch MUST be resumable; finished replications MUST NOT be
  repeated, and unfinished ones MUST keep the seeds assigned to them. Failed replications
  MUST be reported with their seeds and be retryable with the same seeds.
- **FR-026**: Each replication MUST record the simulated time covered and the time it took;
  when the simulator reports its advance, progress MUST be shown as simulated time covered
  out of the horizon.
- **FR-027**: For a model that uses no random numbers, the system MUST make one replication
  per scenario, assign no seed, and refuse further replications with an explanation.
- **FR-028**: The system MUST refuse to run a scenario whose experiment has no model, an
  empty pipeline, or scenarios awaiting resolution under FR-015.
