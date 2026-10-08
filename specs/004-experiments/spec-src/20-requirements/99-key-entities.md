### Key Entities *(include if feature involves data)*

- **Experiment**: An investigation that tests one or more hypotheses. Has an objective, a
  kind, a status, a design, a pipeline, parameters, runs, an optional conclusion, and links
  to a methodology, dataset versions, software, and responsible staff.
- **Design**: How the experiment is set up: its variables, its conditions, the planned
  sample size, and the planned replicates.
- **Variable**: Something the experiment changes (independent), measures (dependent), or
  holds fixed (controlled), with a unit.
- **Condition**: One setting of the independent variables to be compared with others; one
  may be the control.
- **Pipeline**: The dependency-ordered set of steps that carries out the experiment.
- **Step**: One unit of work in a pipeline: automated or manual, with what it depends on
  and the files it should produce.
- **Run**: One execution of the pipeline, with everything in effect when it started
  (pipeline, parameters, methodology, dataset versions, software versions, environment),
  its condition and replicate number, and its outcome.
- **Step Result**: The outcome of one step in one run: status, timing, output, produced
  files, notes, and who performed it if manual.
- **Parameter**: A declared input of an experiment, with its allowed values and default.
- **Sweep**: A group of runs over combinations of parameter values.
- **Metric**: A named measurement, or ordered series of measurements, of one run.
- **Result**: A named finding from one run: a value with unit and uncertainty. Linked to
  hypotheses as evidence and to drafts that report it.
- **Figure / Table**: A visual or tabular finding from one run, known by reference to its
  file and its fingerprint. Linked to drafts.
- **Evidence**: The link between a result and a hypothesis, with a stance.
- **Software**: A piece of code or a tool an experiment depends on; each run records its
  version.
- **Conclusion**: What an experiment showed: an outcome per hypothesis, supporting results,
  limitations, and next steps.
- **Replication**: The relationship between an experiment and a later experiment that
  repeats it.
