<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Experiments

**Feature Branch**: `004-experiments`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add concepts of experiment as a separated spec on its own"

## Overview

An experiment is how a researcher turns a hypothesis into evidence. This specification
gathers everything TRCLI knows about experiments into one place: what an experiment is and
how it is designed, the pipeline of steps that carries it out, each run of that pipeline,
the parameters and measurements of a run, the results, figures, and tables a run produces,
the code that ran, and how an experiment is concluded and replicated.

It **takes over** the experiment content previously spread across two specifications, which
now point here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 5 | Experiments, pipelines, manual steps, runs |
| `specs/001-research-workspace`, User Story 6 | Results, figures, and tables |
| `specs/002-research-lifecycle`, User Story 5 | Run parameters, metrics, sweeps, comparison |
| `specs/002-research-lifecycle`, User Story 6 | Code and software versions behind a run |

It **adds** two things that were not specified before: the design of an experiment
(its kind, variables, and conditions), and concluding and replicating an experiment.

### The concepts, in one picture

```text
Research Question ── has ──▶ Hypothesis ◀── tests ── Experiment ── has ──▶ Design
                                 ▲                       │                 (kind, variables,
                                 │ evidence              │ has              conditions)
                                 │ (supports /           ▼
                                 │  contradicts)      Pipeline ── ordered ──▶ Step
                                 │                       │                    (automated | manual)
                              Result ◀── produces ──    Run ── one per ──▶ Step Result
                              Figure                     │
                              Table                      ├── Parameters, Metrics
                                                         ├── Condition, Replicate
                                                         └── Methodology, Dataset versions,
                                                             Software versions, Environment
```

Methodologies, datasets, environment snapshots, and reproducibility checks remain specified
in `specs/001-research-workspace`; research questions and hypotheses likewise. This
specification refers to them and does not redefine them.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Define and design an experiment (Priority: P1)

A researcher records an experiment: its name, its objective, the hypotheses it tests, and
its kind (computational, laboratory, field, survey, or simulation). They then describe its
design: the variables they will change (independent), the variables they will measure
(dependent), the variables they will hold fixed (controlled), and the conditions to compare,
one of which may be the control.

**Why this priority**: An experiment must exist before it can have a pipeline or a run.
With only this story, a researcher already has a register of what they plan to test, how,
and against which hypothesis.

**Independent Test**: Create an experiment linked to a hypothesis, add one independent and
one dependent variable and two conditions including a control, and view the experiment
showing its full design.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates an experiment with a name and an
   objective, **Then** it is stored with the status "planned".
2. **Given** an experiment and existing hypotheses, **When** the researcher links the
   hypotheses it tests, **Then** the experiment lists them and each hypothesis lists the
   experiment.
3. **Given** an experiment, **When** the researcher sets its kind, **Then** the kind is shown
   and experiments can be listed by kind.
4. **Given** an experiment, **When** the researcher adds a variable with a name, a role
   (independent, dependent, or controlled), a unit, and a description, **Then** it is stored
   as part of the design.
5. **Given** an experiment, **When** the researcher adds conditions, each with a name and
   the values of the independent variables it uses, and marks one as the control, **Then**
   the design shows the conditions and which is the control.
6. **Given** an experiment, **When** the researcher records the planned sample size or the
   planned number of replicates per condition, **Then** it is shown with the design.
7. **Given** an experiment, **When** the researcher links a methodology and dataset
   versions, **Then** they are shown and will be recorded with every run.
8. **Given** experiments, **When** the researcher lists them filtered by status, kind,
   hypothesis, or tag, **Then** only matching experiments are shown.
9. **Given** a condition that gives a value to a variable that is not an independent
   variable of the design, or two variables with the same name, **When** the researcher
   saves, **Then** the tool rejects it and names the problem.
10. **Given** an experiment with runs, **When** the researcher deletes it, **Then** the tool
    refuses and says how many runs exist.
11. **Given** an experiment with runs, **When** the researcher changes its design, **Then**
    the tool warns that past runs were made under the earlier design, keeps what each run
    recorded, and requires confirmation.

---

### User Story 2 - Build a pipeline of automated and manual steps (Priority: P2)

A researcher describes how the experiment is carried out as a pipeline: ordered steps, each
either automated (something the machine does) or manual (something a person does, such as
"collect samples in the lab" or "label 50 examples by hand"). Steps can depend on other
steps, and the tool shows the order in which they will be carried out.

**Why this priority**: The pipeline is the experiment's procedure made explicit. Manual
steps are first-class because real research is rarely fully automated.

**Independent Test**: Add three steps to an experiment, one of them manual, with
dependencies between them, and view the pipeline in execution order.

**Acceptance Scenarios**:

1. **Given** an experiment, **When** the researcher adds an automated step with a name and
   what the machine must carry out, **Then** the step is stored in the pipeline.
2. **Given** an experiment, **When** the researcher adds a manual step with a name and
   instructions for the person, **Then** the step is stored and marked manual.
3. **Given** steps, **When** the researcher states that a step depends on others, **Then**
   the pipeline shows the steps in an order that respects every dependency.
4. **Given** a manual step, **When** the researcher assigns the person who performs it,
   **Then** the step shows that person.
5. **Given** a step, **When** the researcher declares the files it is expected to produce,
   **Then** the pipeline lists them as the step's outputs.
6. **Given** a pipeline, **When** the researcher changes, reorders, or removes a step,
   **Then** the change is saved and past runs are unaffected.
7. **Given** steps whose dependencies form a loop, or a dependency on a step that does not
   exist, **When** the researcher saves, **Then** the tool rejects it and names the steps.
8. **Given** an automated step with nothing to carry out, or a manual step without
   instructions, **When** the researcher saves, **Then** the tool rejects it.
9. **Given** a step that other steps depend on, **When** the researcher removes it,
   **Then** the tool lists the dependent steps and refuses until they are changed.
10. **Given** an experiment, **When** the researcher copies its pipeline from another
    experiment, **Then** the steps are copied and can be changed independently.

---

### User Story 3 - Run an experiment (Priority: P3)

A researcher starts a run of the pipeline. The tool carries out automated steps in order,
showing their output as it happens. When it reaches a manual step it saves the run, shows
the instructions, and gives the terminal back; the researcher confirms the step later, with
notes, and the run continues. A failed or interrupted run can be resumed from where it
stopped. Every run is kept, with exactly what was done.

**Why this priority**: Running is what produces evidence. It needs an experiment and a
pipeline, and everything after it — parameters, results, comparison — needs runs.

**Independent Test**: Run a three-step pipeline with one manual step, confirm the manual
step, and review the run showing the status, timing, and outcome of every step; then
interrupt another run and resume it.

**Acceptance Scenarios**:

1. **Given** an experiment with a pipeline, **When** the researcher starts a run, **Then** a
   run is created with the next run number for that experiment and the steps begin in
   order.
2. **Given** a run in progress, **When** an automated step is carried out, **Then** its
   output is shown as it happens and kept in the run's record.
3. **Given** a run that reaches a manual step, **When** the step is reached, **Then** the
   run is saved as paused, the instructions and how to continue are shown, and the
   researcher regains the terminal immediately.
4. **Given** a paused run, **When** the researcher confirms the manual step with notes,
   **Then** who confirmed, when, and the notes are recorded and the run continues.
5. **Given** a paused run, **When** the researcher marks the manual step failed with a
   reason, **Then** the run stops as failed and the reason is recorded.
6. **Given** a run in which an automated step fails, **When** the failure occurs, **Then**
   the run stops, the failure is recorded, and remaining steps are not carried out.
7. **Given** a failed run, **When** the researcher resumes it, **Then** it continues from
   the failed step and completed steps are not repeated.
8. **Given** a run in progress, **When** the researcher interrupts it or the machine stops,
   **Then** the run is recorded as interrupted — never as succeeded or still running — and
   can be resumed.
9. **Given** a paused, failed, or interrupted run, **When** the researcher cancels it,
   **Then** it is recorded as cancelled and cannot be resumed.
10. **Given** a run, **When** it starts, **Then** the pipeline as it was at that moment, the
    methodology, the dataset versions, and the environment are recorded with the run, and
    later changes to any of them do not alter the run's record.
11. **Given** an experiment with conditions, **When** the researcher starts a run for a
    condition, **Then** the run records the condition and its replicate number.
12. **Given** a finished run, **When** the researcher views it, **Then** every step's
    status, start and end time, outputs, and notes are shown.
13. **Given** several runs, **When** the researcher lists them, **Then** they can be
    filtered by experiment, status, condition, and date.
14. **Given** an experiment with no steps, **When** the researcher starts a run, **Then**
    the tool refuses and says the pipeline is empty.
15. **Given** a paused run, **When** another run of the same experiment is started,
    **Then** both are tracked independently.

---

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

### User Story 6 - Record the code and software behind a run (Priority: P6)

A researcher registers the software an experiment depends on — their own code and
third-party tools — and each run records exactly which version of each was used, including
whether the researcher's own code had unsaved changes at the time.

**Why this priority**: The method, the data, and the environment are recorded with every
run, but the code that actually ran is not. It is the most frequent reason a result cannot
be reproduced.

**Independent Test**: Register a code project, run an experiment, and view the run showing
the exact code version; change the code, run again, and see the difference reported when
the two runs are compared.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher registers a piece of software with a name,
   a location, a license, and whether it is their own or third-party, **Then** it is stored
   and can be linked to experiments.
2. **Given** an experiment linked to software, **When** a run starts, **Then** the exact
   version of each piece of software is recorded with the run.
3. **Given** the researcher's code has changes not yet recorded in its history, **When** a
   run starts, **Then** the run is marked as made from unrecorded changes and the
   researcher is warned.
4. **Given** code that is not kept under any change history, **When** a run starts,
   **Then** the run records the code's fingerprint instead and says no history was
   available.
5. **Given** two runs, **When** the researcher compares them, **Then** differences in
   software versions are listed.
6. **Given** a past run, **When** a reproducibility check is made, **Then** differences in
   software versions are part of the report.
7. **Given** software whose location does not exist, **When** the researcher registers it,
   **Then** the tool rejects it.

---

### User Story 7 - Conclude and replicate an experiment (Priority: P7)

When the runs are done, a researcher records what the experiment showed: an outcome for
each hypothesis it tested, a written conclusion, the limitations, and what to do next. The
experiment is then closed. Later, they or a colleague can replicate it: a new experiment is
created from the original's design and pipeline, linked back to it, and the two can be
compared.

**Why this priority**: A conclusion is what turns a set of runs into knowledge, and
replication is how that knowledge is tested. Both come after everything else exists.

**Independent Test**: Conclude an experiment with an outcome and a conclusion, confirm it
is closed, create a replication of it, and view the original showing its replication.

**Acceptance Scenarios**:

1. **Given** an experiment with at least one succeeded run, **When** the researcher records
   a conclusion with an outcome for each tested hypothesis (supported, refuted,
   inconclusive), the supporting results, the limitations, and next steps, **Then** it is
   stored and the experiment's status becomes "completed".
2. **Given** a conclusion that names an outcome for a hypothesis, **When** it is recorded,
   **Then** the tool offers to update that hypothesis's status accordingly and does so only
   when the researcher accepts.
3. **Given** a completed experiment, **When** the researcher tries to start a run or change
   its design or pipeline, **Then** the tool refuses and offers to reopen it.
4. **Given** an experiment with no succeeded run, **When** the researcher tries to conclude
   it, **Then** the tool refuses, unless the experiment is being abandoned.
5. **Given** an experiment, **When** the researcher abandons it with a reason, **Then** its
   status becomes "abandoned" and the reason is recorded.
6. **Given** a completed or abandoned experiment, **When** the researcher reopens it,
   **Then** it becomes active again and the earlier conclusion is kept in its history.
7. **Given** an experiment, **When** the researcher creates a replication of it, **Then** a
   new experiment is created with a copy of the design, pipeline, parameters, and links to
   hypotheses, methodology, and software, and each experiment shows the other.
8. **Given** an experiment and its replications, **When** the researcher compares them,
   **Then** results with the same name are shown side by side with whether they agree.
9. **Given** an experiment, **When** the researcher exports its report, **Then** a document
   is produced with the objective, design, pipeline, runs, results, and conclusion.

---

### Edge Cases

- A run is interrupted because the terminal closes or the machine shuts down: on the next
  use the run is shown as interrupted, not as succeeded or running, and can be resumed.
- A manual step is never confirmed: the run stays paused indefinitely, is listed as
  waiting, and can be cancelled.
- A manual step is confirmed by someone other than the assigned performer: it is accepted,
  and both names are recorded.
- A manual step is confirmed twice, or a run that is not paused is confirmed: the tool
  refuses and reports the run's actual state.
- An automated step never ends: it can be interrupted, and a time limit can be set after
  which it is stopped and recorded as failed.
- An automated step produces a very large amount of output: all of it is kept in the run's
  record, and only the most recent part is shown on screen.
- A step declares an output file that does not exist when the step ends: the step is
  recorded as failed, naming the missing file.
- The pipeline is changed while a run of it is paused: the run continues with the pipeline
  as it was when the run started.
- A dataset version, methodology, or piece of software linked to the experiment is changed
  or removed after a run: the run keeps what it recorded; removal is refused while runs
  refer to it.
- A run that results, figures, or tables were recorded from is deleted: the tool refuses
  while any of them depends on it.
- A sweep would create an unreasonably large number of runs: the tool shows the count and
  requires explicit confirmation above a stated limit.
- A parameter's allowed values are narrowed after runs used a value that is no longer
  allowed: past runs keep their values and are marked as outside the current definition.
- A metric value is not a finite number: the tool rejects it.
- Two results with the same name are recorded for the same run: the tool rejects the
  second and offers to replace the first.
- A condition is removed from the design after runs were made for it: the runs keep the
  condition's name and are shown as belonging to a removed condition.
- A replication's design is changed: it is still a replication, and the comparison reports
  the design differences alongside the results.
- A hypothesis tested by the experiment is deleted: the experiment keeps running; the link
  is removed after confirmation.
- An experiment is concluded while a run is still paused or in progress: the tool refuses
  and names the run.
- Values are given in an invalid form — an empty name, a negative replicate count, a number
  where text is expected: the tool rejects them all together, each with what is expected.

## Requirements *(mandatory)*

### Functional Requirements

#### Experiments

- **FR-001**: Users MUST be able to create, list, view, update, and delete experiments, each
  with a name, objective, description, kind (computational, laboratory, field, survey,
  simulation, other), and status (planned, active, completed, abandoned).
- **FR-002**: Users MUST be able to link an experiment to the hypotheses it tests, to a
  methodology, to dataset versions, to software, and to the staff responsible for it, with
  every link visible from both ends.
- **FR-003**: An experiment's status MUST become "active" when its first run starts.
- **FR-004**: Users MUST be able to filter experiments by status, kind, hypothesis, tag, and
  responsible person, and search them by words in the name and objective.
- **FR-005**: The system MUST refuse to delete an experiment that has runs.
- **FR-006**: Experiments MUST support tags, notes, and free links to any other record, as
  every record of the workspace does.

#### Experiment design

- **FR-007**: Users MUST be able to record an experiment's variables, each with a name
  unique within the experiment, a role (independent, dependent, controlled), a unit, and a
  description.
- **FR-008**: Users MUST be able to record conditions, each with a name unique within the
  experiment and a value for one or more independent variables, and mark at most one
  condition as the control.
- **FR-009**: The system MUST reject a condition that gives a value to a variable that is
  not an independent variable of the same experiment.
- **FR-010**: Users MUST be able to record the planned sample size and the planned number of
  replicates per condition.
- **FR-011**: When the design of an experiment that has runs is changed, the system MUST
  warn, require confirmation, and leave what each past run recorded unchanged.

#### Pipelines and steps

- **FR-012**: Users MUST be able to define a pipeline for an experiment as steps, each with
  a key unique within the pipeline, a name, the steps it depends on, and a kind: automated
  or manual.
- **FR-013**: An automated step MUST state what the machine carries out; a manual step MUST
  carry instructions for the person and MAY name the person who performs it.
- **FR-014**: Users MUST be able to declare the files a step is expected to produce.
- **FR-015**: The system MUST reject a pipeline whose dependencies form a loop or refer to a
  step that does not exist, naming the steps involved, and MUST refuse to remove a step
  that other steps depend on.
- **FR-016**: The system MUST show a pipeline's steps in the order they will be carried out,
  and users MUST be able to copy a pipeline from another experiment.

#### Runs

- **FR-017**: Users MUST be able to start a run of an experiment's pipeline; the system MUST
  number runs in sequence per experiment and MUST refuse to start a run of an empty
  pipeline or of a completed or abandoned experiment.
- **FR-018**: The system MUST carry out automated steps in dependency order, show their
  output as it happens, and record each step's status, start and end time, output, and
  produced files.
- **FR-019**: When a run reaches a manual step, the system MUST save the run as paused, show
  the step's instructions and how to continue, and return control to the user without
  waiting.
- **FR-020**: Users MUST be able to confirm a waiting manual step as done, with notes, or
  mark it failed, with a required reason; the system MUST record who did so and when.
- **FR-021**: When a step fails, the system MUST stop the run, record the failure, and not
  carry out the remaining steps.
- **FR-022**: Users MUST be able to resume a failed or interrupted run; resuming MUST
  continue from the first step that did not succeed and MUST NOT repeat succeeded steps.
- **FR-023**: A run that is interrupted, by the user or by the machine stopping, MUST be
  recorded as interrupted and MUST never be shown as succeeded or as still in progress.
- **FR-024**: Users MUST be able to cancel a paused, failed, or interrupted run; a cancelled
  run MUST NOT be resumable.
- **FR-025**: Each run MUST record, at the moment it starts, the pipeline definition, the
  parameter values, the methodology, the dataset versions, the software versions, and the
  environment in effect; later changes to any of these MUST NOT alter the run's record.
- **FR-026**: A run MAY belong to one condition of the experiment's design; the system MUST
  then record the condition and number the run as a replicate of that condition.
- **FR-027**: The system MUST keep every run, and users MUST be able to list runs filtered
  by experiment, status, condition, sweep, and date, and view any run in full.
- **FR-028**: A declared output file that is missing when its step ends MUST make the step
  fail, naming the file.
- **FR-029**: Users MUST be able to set a time limit for automated steps, after which a step
  is stopped and recorded as failed.

#### Parameters, metrics, and sweeps

- **FR-030**: Users MUST be able to declare an experiment's parameters with a name, a kind,
  allowed values or a range, and a default.
- **FR-031**: The system MUST reject a run given an undeclared parameter or a value outside
  what is allowed, and MUST record the default for every parameter not given.
- **FR-032**: Users MUST be able to start a sweep over sets of parameter values; the system
  MUST show the number of runs before starting, require confirmation, and group the runs as
  one sweep.
- **FR-033**: Users and automated steps MUST be able to record named metrics for a run;
  values MUST be finite numbers, and repeated values under one name MUST be kept as an
  ordered series.
- **FR-034**: Users MUST be able to compare runs side by side by parameters, metrics,
  status, condition, and software versions, sort by any of them, and see which parameters
  differ.
- **FR-035**: Users MUST be able to find the best run of an experiment or sweep by a chosen
  metric and direction, mark a run as best with a note, and be told when a later run beats
  the marked one.
- **FR-036**: Users MUST be able to retry the failed runs of a sweep without repeating the
  successful ones.
- **FR-037**: The system MUST summarize runs by condition, showing per metric the number of
  replicates, the mean, and the spread.

#### Results, figures, and tables

- **FR-038**: Users MUST be able to record a result from a run with a name, a value, a unit,
  an optional uncertainty, and a description, and to create a result from a recorded
  metric; a result's name MUST be unique within its run.
- **FR-039**: Users MUST be able to record a figure or a table from a run with a title, a
  caption, the location of its file, and an integrity fingerprint, without the tool taking
  a copy of the file.
- **FR-040**: Every result, figure, and table MUST be permanently tied to the run that
  produced it, and the system MUST show for each one the run, its parameters and condition,
  the pipeline as run, the dataset versions, the methodology, the software versions, and
  the environment.
- **FR-041**: Users MUST be able to link results to hypotheses with a stance (supports,
  contradicts, neutral), and link results, figures, and tables to drafts.
- **FR-042**: The system MUST flag a draft when a result it reports has been followed by a
  newer result of the same name from the same experiment, and let the user keep the
  reported one or switch.
- **FR-043**: Users MUST be able to verify a figure or table file against its recorded
  fingerprint.
- **FR-044**: The system MUST refuse to delete a run while any result, figure, or table
  depends on it, and MUST require confirmation to record a finding from a run that did not
  succeed.

#### Code and software

- **FR-045**: Users MUST be able to register software with a name, location, license, and
  whether it is their own or third-party, and link it to experiments.
- **FR-046**: Every run MUST record the exact version of each linked piece of software; for
  the user's own code it MUST record and warn about unrecorded changes, and MUST record a
  fingerprint when no change history is available.
- **FR-047**: Run comparison and reproducibility checks MUST include differences in software
  versions.

#### Conclusion and replication

- **FR-048**: Users MUST be able to record a conclusion for an experiment with an outcome
  per tested hypothesis (supported, refuted, inconclusive), the results that support it,
  the limitations, and next steps; recording it MUST set the experiment to "completed".
- **FR-049**: The system MUST refuse to conclude an experiment that has no succeeded run or
  that has a run paused or in progress.
- **FR-050**: When a conclusion names an outcome for a hypothesis, the system MUST offer to
  update that hypothesis's status and MUST do so only when the user accepts.
- **FR-051**: Users MUST be able to abandon an experiment with a required reason, and to
  reopen a completed or abandoned experiment; earlier conclusions MUST be kept in the
  experiment's history.
- **FR-052**: The system MUST refuse to start runs or to change the design or pipeline of a
  completed or abandoned experiment until it is reopened.
- **FR-053**: Users MUST be able to create a replication of an experiment, which copies its
  design, pipeline, parameters, and links, and records the relationship in both directions.
- **FR-054**: Users MUST be able to compare an experiment with its replications, showing
  results of the same name side by side, whether they agree, and any design differences.
- **FR-055**: Users MUST be able to export a report of an experiment containing its
  objective, design, pipeline, runs, results, and conclusion.

#### Common behavior

- **FR-056**: Every value a user supplies for experiments, designs, pipelines, runs,
  parameters, metrics, results, figures, tables, software, and conclusions MUST be
  validated before anything is stored or carried out; invalid input MUST change nothing and
  MUST be reported per value, all together, with what is expected.
- **FR-057**: Every creation, change, deletion, run, confirmation, and status change
  specified here MUST be recorded in the workspace's audit trail.
- **FR-058**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-059**: Every command MUST have built-in help, and experiments, pipelines, runs,
  parameters and metrics, results, software, and conclusions MUST each have a usage guide
  with examples.

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

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can create an experiment, describe its design with two variables and
  two conditions, and link it to a hypothesis in under 5 minutes.
- **SC-002**: A user can define a five-step pipeline including a manual step and complete a
  run of it in under 10 minutes, excluding the time the steps themselves take.
- **SC-003**: When a run reaches a manual step, the user regains the terminal in under 2
  seconds, and can confirm the step days later from any terminal.
- **SC-004**: After an interruption, 100% of runs are shown in their true state and can be
  resumed without repeating steps that had already succeeded.
- **SC-005**: 100% of runs retain the pipeline, parameters, methodology, dataset versions,
  software versions, and environment in effect when they started, whatever is changed
  afterwards.
- **SC-006**: A user can launch a 20-run sweep in under 2 minutes of their own time, and
  identify the best run by any metric in under 30 seconds once it finishes.
- **SC-007**: A user can compare any 10 runs by parameters and metrics in a single view in
  under 5 seconds.
- **SC-008**: For any result, figure, or table, a user can identify the run, pipeline,
  methodology, dataset versions, software versions, and environment that produced it in
  under 1 minute.
- **SC-009**: 100% of drafts reporting a result that a newer run has replaced are flagged.
- **SC-010**: For any run, a user can state the exact version of every piece of software it
  used in under 1 minute.
- **SC-011**: A replication of an experiment can be created in under 1 minute, and its
  results compared with the original's in a single view.
- **SC-012**: 100% of invalid inputs are rejected before any data changes or any step is
  carried out, each with the invalid value named.
- **SC-013**: With 1,000 runs in a workspace, listing and filtering runs takes under 2
  seconds.
- **SC-014**: 90% of first-time users complete the primary task of each story on their
  first attempt using only that story's usage guide.

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
