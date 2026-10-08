#### Common behavior

- **FR-050**: Everything specified for experiments, pipelines, runs, parameters, metrics,
  results, software, conclusions, and replication in `specs/004-experiments` MUST apply to
  simulation experiments unchanged; this specification MUST only add to it.
- **FR-051**: Every value a user supplies for models, inputs, outputs, scenarios, seeds,
  verification cases, validations, uncertainties, and analyses MUST be validated before
  anything is stored or run; invalid input MUST change nothing and MUST be reported per
  value, all together, with what is expected.
- **FR-052**: Models and scenarios MUST support tags, notes, and links, and belong to
  projects, as every record does; every creation, change, deletion, run, and check MUST be
  recorded in the workspace's audit trail.
- **FR-053**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-054**: Every command MUST have built-in help, and models, scenarios, replications and
  seeds, outputs and summaries, credibility, and sensitivity and ensembles MUST each have a
  usage guide with examples, including how a simulator receives its inputs and seed.
