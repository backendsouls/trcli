#### Outputs and summaries

- **FR-029**: For each finished replication the system MUST record every output the model
  declares — a value, or for a series a reference to where it is and its fingerprint — and
  MUST report declared outputs that are missing.
- **FR-030**: For a series the system MUST show the number of points, the first and last
  time, and the minimum, maximum, mean, and final value, all computed after the warm-up;
  the system MUST NOT hold the series itself.
- **FR-031**: For a scenario the system MUST summarize each output across its replications
  with the number of replications, the mean, the spread, and an interval for the mean at a
  level of confidence the user chooses (95% by default).
- **FR-032**: Summaries MUST leave out, and count, replications that failed, were
  interrupted, ended before the horizon, contain values that are not finite, or were run
  under an earlier scenario definition or model version; users MUST be able to include the
  last two groups explicitly.
- **FR-033**: Users MUST be able to state a required precision for an output; when a
  summary's interval is wider, the system MUST say so and estimate the number of additional
  replications needed.
- **FR-034**: Users MUST be able to compare scenarios: for each output, each scenario beside
  the baseline, the difference, an interval for the difference, and whether it excludes
  zero; replications with the same seeds MUST be compared in pairs, and the comparison MUST
  state which method it used.
- **FR-035**: Users MUST be able to record a scenario summary or a comparison as a result of
  the experiment, with the mean as its value and the half-width of the interval as its
  uncertainty; the result MUST be tied to every replication it was computed from, and its
  provenance MUST show the scenario, seeds, model version, and simulator version.
- **FR-036**: Users MUST be able to export a series for a scenario as a table, per
  replication or as mean and interval at each time.
- **FR-037**: With a single replication, the system MUST show the value and show spread and
  interval as not available.
