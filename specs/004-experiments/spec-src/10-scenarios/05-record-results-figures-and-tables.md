### User Story 5 - Record results, figures, and tables (Priority: P5)

A researcher records the specific findings a run produced: a named value with its unit and
uncertainty, a figure, or a table. Each one remembers the run it came from. The researcher
links results to the hypotheses they bear on and to the drafts that report them, so any
number or figure in a paper can be traced to its evidence, and is warned when a draft
reports a result that a newer run has replaced.

**Why this priority**: A run's raw outputs and metrics are not yet a finding. Naming the
finding and tying it to its run is the link between experiments and writing; without it,
the trace from a claim to its evidence breaks exactly where reviewers ask.

**Independent Test**: Finish a run, record one numeric result and one figure from it, link
both to a hypothesis and a draft, then ask where each came from and receive the run,
pipeline, dataset versions, methodology, software versions, and environment.

**Acceptance Scenarios**:

1. **Given** a finished run, **When** the researcher records a result with a name, a value,
   and a unit, **Then** it is stored and permanently tied to that run.
2. **Given** a run with a recorded metric, **When** the researcher promotes the metric to a
   result, **Then** a result is created with that name and value, tied to the same run.
3. **Given** a finished run, **When** the researcher records a figure or a table with a
   title, a caption, and the location of its file, **Then** it is stored with an integrity
   fingerprint and tied to that run.
4. **Given** a result, figure, or table, **When** the researcher asks where it came from,
   **Then** the tool shows the run, its parameters and condition, the pipeline as run, the
   dataset versions, the methodology, the software versions, and the environment.
5. **Given** a result and a hypothesis, **When** the researcher links them stating whether
   the result supports, contradicts, or is neutral to the hypothesis, **Then** the
   hypothesis shows the result as evidence with that stance.
6. **Given** a draft, **When** the researcher links results, figures, and tables to it,
   **Then** the draft lists them, each with the run it came from.
7. **Given** a result reported in a draft, **When** a later run records a result with the
   same name for the same experiment, **Then** the draft is flagged as reporting a result
   that may be out of date, and the researcher can keep the old one or switch to the new.
8. **Given** a figure whose file has changed since it was recorded, **When** the researcher
   verifies it, **Then** the tool reports that it no longer matches the recorded run.
9. **Given** a result with a value that is not a number where a number is required, or a
   figure whose file does not exist, **When** the researcher saves it, **Then** the tool
   rejects it and explains why.
10. **Given** a run that has not finished or has failed, **When** the researcher records a
    result from it, **Then** the tool warns that the run did not succeed and requires
    confirmation.

---
