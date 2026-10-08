### Key Entities *(include if feature involves data)*

- **Model**: A representation of something, to be simulated. Has a purpose, a kind, a unit
  of simulated time, inputs, outputs, assumptions, versions, a simulator, and a credibility
  status per version.
- **Model Input**: Something the model can be given: its kind of value, unit, default, and
  allowed values.
- **Model Output**: Something the model produces: a single value or a series over simulated
  time, with a unit.
- **Assumption**: A statement the model takes as true, with its justification.
- **Model Version**: The model as it was at one moment, with what changed since the previous
  one and its own checks.
- **Scenario**: A named, complete setting of a model's inputs within a simulation
  experiment, with horizon, warm-up, time step, and number of replications; possibly derived
  from another; one is the baseline. The simulation form of an experiment's condition.
- **Replication**: One run of one scenario with one seed; a run of the experiment with the
  scenario values, seed, model version, and simulated time added.
- **Seed**: The number that fixes a replication's random numbers, derived from a batch's
  base seed.
- **Batch**: A group of replications started together, with its base seed.
- **Output Record**: One output of one replication: a value, or a reference to a series with
  its summary.
- **Scenario Summary**: An output across the replications of a scenario: count, mean,
  spread, interval.
- **Comparison**: The difference of an output between a scenario and the baseline, with its
  interval.
- **Verification Case**: Inputs with a known answer, and the tolerance within which the
  model must give it.
- **Validation**: A recorded comparison of the model with observed data, with a measure of
  agreement and a judgement.
- **Sensitivity Analysis**: A group of scenarios varying inputs one at a time, and the
  ranking that results.
- **Input Uncertainty / Ensemble**: How uncertain inputs are, and a group of runs over
  combinations drawn from that uncertainty.
