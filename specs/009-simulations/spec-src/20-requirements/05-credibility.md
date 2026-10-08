#### Credibility

- **FR-038**: Users MUST be able to run a repeatability check on a scenario: the same
  inputs and seed are run twice and every output compared; the system MUST report the
  outputs that differ and any difference in simulator version between the two runs.
- **FR-039**: Users MUST be able to define verification cases for a model: a setting of the
  inputs, an output, an expected value, a tolerance, and the source of the expected value;
  and to run them, each reported as passed or failed with the obtained value, the expected
  value, and the difference.
- **FR-040**: Users MUST be able to record validations of a model: the observed dataset and
  its version, the scenario that corresponds to it, the output compared, the measure of
  agreement, its value, the threshold of acceptance, and the researcher's judgement
  (acceptable, not acceptable) with a note.
- **FR-041**: Each model version MUST carry a credibility status: "unverified"; "verified"
  when its repeatability check and all its verification cases pass; "validated" when it is
  verified and has at least one validation judged acceptable. A new version MUST start
  "unverified"; a failing check MUST lower the status and name the cause.
- **FR-042**: The system MUST show, for a model version, its assumptions, every check with
  its outcome and date, and what is missing for the next status.
- **FR-043**: Every result computed from a model MUST show the model's credibility status at
  the time of its runs, wherever the result is shown or reported.
- **FR-044**: Checks MUST be kept with the model version they were made on; a validation
  MUST keep the dataset version it used and be marked when that dataset has since changed.
