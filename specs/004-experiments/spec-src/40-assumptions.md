## Assumptions

- **This specification owns experiments.** User Stories 5 and 6 of
  `specs/001-research-workspace` and User Stories 5 and 6 of `specs/002-research-lifecycle`
  are replaced by this one; those specifications keep a short pointer in their place.
- **It depends on the base workspace** (`specs/001-research-workspace`) for the workspace
  itself, research questions and hypotheses, drafts, methodologies, datasets, environment
  snapshots, reproducibility checks, staff, the audit trail, and telemetry. Where this
  specification mentions them, their rules are the ones stated there.
- **Experiment design is descriptive.** Variables, conditions, sample size, and replicates
  document the design and label runs. The tool does not assign subjects to conditions,
  randomize, or compute required sample sizes.
- **Statistics are minimal.** The summary by condition shows count, mean, and spread.
  Significance tests, effect sizes, and plots are out of scope; researchers do them with
  their own tools and record the outcome as results.
- **Agreement between a result and its replication** means equal within the recorded
  uncertainties when both have one, and is otherwise left to the researcher's judgment;
  the tool shows the values side by side.
- **Kinds of experiment are labels.** They help filtering and reporting and do not change
  behavior, with one exception: the kind "simulation" gains models, scenarios, seeded
  replications, and credibility checks in `specs/009-simulations`, which adds to this
  specification without changing it. Laboratory specifics such as instruments, samples, and materials are specified
  in `specs/002-research-lifecycle`.
- **Automated steps are carried out on the researcher's own machine, one at a time.**
  Running steps in parallel, in the background, or on remote machines is out of scope here.
- **Results are named by the researcher.** The tool does not extract findings from outputs
  by itself, except that an automated step may report metrics in an agreed form, and a
  metric may be promoted to a result on request.
- **Files are referenced, not stored.** Figures, tables, code, and step outputs are known by
  location and fingerprint.
- **Pre-registration, ethics approvals, and lab notebooks** attach to experiments and runs
  but remain specified in `specs/002-research-lifecycle`.
- **Projects** (`specs/003-research-projects`) scope experiments like any other record; a
  replication may live in a different project from its original.
- **Single researcher**, as in the base workspace: people named as responsible or as
  performers are entries in the staff register, not accounts.
