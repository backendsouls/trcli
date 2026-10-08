### User Story 8 - Capture environment and verify reproducibility (Priority: P8)

A researcher captures the environment in which an experiment ran (the machine, the
operating system, the tools and their versions, and the settings used) and later asks
whether a past run can be reproduced: the tool compares the current environment, data, and
method against what was recorded and reports every difference. The researcher can also
produce a reproducibility package that a colleague can use to repeat the work.

**Why this priority**: Reproducibility is the quality bar of modern research, but it can
only be checked once experiments, methods, and datasets are recorded.

**Independent Test**: Run an experiment, capture its environment, change one thing in the
environment or data, and run a reproducibility check that reports exactly that difference.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher captures the current environment,
   **Then** a named environment snapshot is stored.
2. **Given** an experiment run, **When** it starts, **Then** the environment snapshot in
   effect is recorded with the run automatically.
3. **Given** a past run, **When** the researcher requests a reproducibility check, **Then**
   the tool reports "reproducible" or lists each difference in environment, dataset version,
   methodology, and pipeline definition.
4. **Given** a past run, **When** the researcher requests a reproducibility package,
   **Then** a single shareable bundle is produced describing the pipeline, method, dataset
   references, environment, and results needed to repeat the run.
5. **Given** two environment snapshots, **When** the researcher compares them, **Then** the
   differences are listed item by item.
6. **Given** an environment containing secret values, **When** a snapshot is captured,
   **Then** secret values are not stored.

---
