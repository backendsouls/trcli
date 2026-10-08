#### Environment and reproducibility

- **FR-043**: Users MUST be able to capture a named snapshot of the working environment
  (machine, operating system, tools and versions, and settings) and compare two snapshots.
- **FR-044**: The system MUST record the environment in effect with every run
  automatically.
- **FR-045**: The system MUST NOT store secret values when capturing an environment.
- **FR-046**: Users MUST be able to check whether a past run is reproducible; the check MUST
  compare the current environment, dataset versions, methodology, and pipeline definition
  with those recorded and list every difference.
- **FR-047**: Users MUST be able to produce a single shareable reproducibility package for a
  run, containing what a colleague needs to repeat it.
