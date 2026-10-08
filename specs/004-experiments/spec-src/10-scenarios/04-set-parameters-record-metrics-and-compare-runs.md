### User Story 4 - Set parameters, record metrics, and compare runs (Priority: P4)

A researcher declares the parameters an experiment accepts, runs it with chosen values or
across a set of values (a sweep), records named metrics for each run, compares runs side by
side, and marks the run that is best by a chosen metric.

**Why this priority**: Most empirical work is a search over settings. Without parameters
and metrics as named things, runs cannot be compared and "which run was best" has no answer.

**Independent Test**: Declare two parameters, launch a sweep of six combinations, record a
metric for each run, compare them in one table, and mark the best.

**Acceptance Scenarios**:

1. **Given** an experiment, **When** the researcher declares a parameter with a name, a
   kind, allowed values or a range, and a default, **Then** it is stored with the
   experiment.
2. **Given** declared parameters, **When** the researcher starts a run with a value outside
   what is allowed, or for an undeclared parameter, **Then** the tool rejects the run.
3. **Given** declared parameters, **When** the researcher starts a run without giving a
   value, **Then** the default is used and recorded with the run.
4. **Given** declared parameters, **When** the researcher starts a sweep over sets of
   values, **Then** the tool shows how many runs will be created, asks for confirmation,
   and creates one run per combination, grouped as one sweep.
5. **Given** a run, **When** a metric with a name and a value is recorded, by the
   researcher or by an automated step, **Then** it is stored with that run.
6. **Given** a metric recorded several times under one name for one run, **When** it is
   viewed, **Then** the values are shown as an ordered series.
7. **Given** several runs, **When** the researcher compares them, **Then** a table shows
   their parameters and metrics side by side, sortable by any column, with the parameters
   that differ highlighted.
8. **Given** an experiment or a sweep, **When** the researcher asks for the best run by a
   metric, higher or lower, **Then** the tool names it, and the researcher can mark it as
   best with a note.
9. **Given** a run marked best, **When** a later run beats it on the same metric, **Then**
   the mark stays and the tool reports that a better run exists.
10. **Given** a sweep in which some runs fail, **When** the sweep finishes, **Then** the
    failed runs are reported and can be retried without repeating the successful ones.
11. **Given** runs of several conditions, **When** the researcher asks for a summary by
    condition, **Then** each condition shows its number of runs and, per metric, the mean
    and spread across its replicates.

---
