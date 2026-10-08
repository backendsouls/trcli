#### Sensitivity and ensembles

- **FR-045**: Users MUST be able to run a sensitivity analysis from a baseline scenario:
  named inputs are varied one at a time across a number of levels within the model's range
  or a narrower range given, with a number of replications per level; the scenarios and
  runs created MUST be grouped as one analysis.
- **FR-046**: For a sensitivity analysis and an output, the system MUST show for each input
  the output at its lowest and highest level, the swing between them, its rank by swing,
  and whether the swing is distinguishable from the baseline's interval.
- **FR-047**: Users MUST be able to state the uncertainty of model inputs — a range with
  equal likelihood, a most likely value with a spread, or listed values with weights — and
  run an ensemble of a given number of samples drawn from them with a recorded seed, so
  that the same seed draws the same combinations.
- **FR-048**: For an ensemble and an output, the system MUST show the mean, spread, chosen
  percentiles, minimum, and maximum across samples.
- **FR-049**: Before starting a sensitivity analysis or an ensemble the system MUST show the
  number of runs and require confirmation; both MUST be resumable without repeating
  finished runs, and their outcomes MUST be recordable as results and exportable, tied to
  the analysis and its runs.
