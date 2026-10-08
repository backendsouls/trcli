## Assumptions

- **A simulation is an experiment.** It is an experiment of the kind "simulation" as defined
  in `specs/004-experiments`, which until now treated the kind as a label. This
  specification gives that kind its own concepts and changes nothing for other kinds.
- **A scenario is the simulation form of a condition, and a replication is a run.** Both
  reuse what `specs/004-experiments` specifies; the summary by condition there becomes the
  scenario summary here, with an interval added.
- **TRCLI does not simulate.** The model is computed by the researcher's own simulator,
  registered as software and invoked by the experiment's pipeline. TRCLI supplies inputs
  and seed, records, collects, and summarizes. It contains no simulation engine and no
  modelling language.
- **The simulator must accept its inputs and seed from outside and say where its outputs
  are.** How exactly — and the form in which a simulator reports outputs and progress — is
  an agreed convention fixed at planning time and documented in the usage guide; adapting a
  simulator to it is the researcher's part.
- **Repeatability depends on the simulator.** TRCLI records seeds and checks repeatability;
  it cannot make a simulator repeatable that reads the clock or runs its parts in an
  uncontrolled order.
- **Statistics are the standard ones for replications**: mean, spread, and an interval for
  the mean that assumes independent replications; comparisons use paired differences when
  seeds are shared. This goes one step beyond the "count, mean, and spread" of
  `specs/004-experiments` because replications are meaningless without it. Tests of
  hypotheses, fitting of distributions, and plots remain out of scope.
- **Sensitivity analysis is one input at a time.** Methods that vary inputs together to
  measure interactions are out of scope; an ensemble gives the overall spread instead.
- **Calibration and optimization are out of scope**: the tool does not search for input
  values that best fit data or that maximize an output. A researcher who does so with
  another tool records the outcome as a scenario.
- **Verification and validation are recorded, not certified.** The statuses summarize the
  checks the researcher has made; they do not prove a model correct. The measure of
  agreement and its threshold in a validation are the researcher's choice.
- **Series are referenced, not stored**, like all data in TRCLI; their summaries are kept.
- **Runs happen on the researcher's machine, one at a time**, as in
  `specs/004-experiments`. Running replications at the same time or on other machines is a
  natural later addition and is the reason every replication carries its own seed.
- **Observed data for validation is a dataset** as specified in
  `specs/001-research-workspace`; references and methodologies likewise come from their own
  specifications.
- **Single researcher**, as elsewhere.
