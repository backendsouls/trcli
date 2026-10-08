<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Simulations

**Feature Branch**: `009-simulations`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add simulation concept as part of experiment but different spec"

## Overview

Some experiments are not carried out on the world but on a **model** of it: a queue, an
epidemic, a market, a material, a network. The researcher builds a model, feeds it
**scenarios**, lets a simulator compute what happens, repeats each scenario many times with
different random numbers, and draws conclusions from the spread of outcomes. This is still
an experiment — it tests a hypothesis and produces results — but it has concepts an
ordinary experiment does not: the model and its assumptions, the scenario, the random seed,
the replication, simulated time, and the question every reader asks first, "why should I
believe this model?".

This specification adds those concepts **on top of** experiments. A simulation is an
experiment of the kind "simulation" (`specs/004-experiments`); everything specified there —
pipelines, runs, parameters, metrics, results, conclusions, replication — applies
unchanged. What is added here attaches to it:

| Experiment concept (004) | What simulation adds (here) |
|--------------------------|-----------------------------|
| Experiment | A **model** it simulates, at a stated version |
| Condition | A **scenario**: a named, complete setting of the model's inputs, with a time horizon |
| Run | A **replication**: one run of one scenario with one recorded random seed |
| Replicate number | The seed policy that makes replications independent and repeatable |
| Metric | **Output series** over simulated time, and summaries across replications with their uncertainty |
| Software | The **simulator** that computes the model |
| — | **Credibility**: checks that the model is computed correctly and resembles what it models |
| Sweep | **Sensitivity analysis** and **sampled ensembles** over uncertain inputs |

### The concepts, in one picture

```text
 Hypothesis ◀── tests ── Experiment (kind: simulation) ── simulates ──▶ Model ── version
                              │                                           │  inputs, outputs,
                              │ has                                       │  assumptions
                              ▼                                           ▼
                           Scenario ── sets values of ──────────────▶ Model inputs
                           (baseline, variants; horizon, warm-up)
                              │ run N times
                              ▼
                           Replication = Run + seed ── produces ──▶ Output series, summaries
                              │                                        │
                              └── aggregated per scenario ──▶ mean, spread, interval ──▶ Result
 Credibility: repeatability check · verification cases · validation against observed Dataset
 Exploration: sensitivity analysis · sampled ensemble
```

TRCLI does not simulate anything itself. The researcher's own simulator does the computing;
TRCLI tells it which scenario and seed to use, records what was run, collects what came out,
and keeps the account of why the model can be trusted.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Describe the model (Priority: P1)

A researcher records the model they simulate: what it represents and what it is for, what
kind of model it is, the assumptions it makes, the inputs it takes with their units and
plausible ranges, the outputs it produces, and the unit in which its time is counted. When
the model changes, they record a new version, so that every later run knows which model it
used.

**Why this priority**: A simulation result means nothing without the model behind it. The
model record is the smallest thing that is useful — a written, versioned account of what is
being simulated — and everything else in this specification refers to it.

**Independent Test**: Record a model with three inputs, two outputs, and four assumptions,
attach it to a simulation experiment, change an assumption, record a second version, and
view the differences between the versions.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records a model with a name, a purpose,
   and a kind (discrete-event, agent-based, system dynamics, equation-based, Monte Carlo,
   or other), **Then** it is stored at version 1 with the credibility status "unverified".
2. **Given** a model, **When** the researcher adds an input with a name, a description, a
   kind of value, a unit, a default, and the range or list of values it may take, **Then**
   it is stored as part of the model.
3. **Given** a model, **When** the researcher adds an output with a name, a description, a
   unit, and whether it is a single value or a series over simulated time, **Then** it is
   stored as part of the model.
4. **Given** a model, **When** the researcher records its assumptions, each as a statement
   with an optional justification and the references supporting it, **Then** they are
   listed with the model.
5. **Given** a model, **When** the researcher records the unit of simulated time and
   whether the model uses random numbers, **Then** these are shown with the model.
6. **Given** a model and the simulator software that computes it, **When** the researcher
   links them, **Then** the model shows its simulator and every run records the simulator's
   version.
7. **Given** a model, **When** the researcher changes its inputs, outputs, or assumptions
   and records a new version with a note of what changed, **Then** the version number
   advances and the earlier version remains viewable.
8. **Given** two versions of a model, **When** the researcher compares them, **Then** the
   inputs, outputs, and assumptions added, removed, and changed are listed.
9. **Given** a simulation experiment, **When** the researcher sets the model it simulates,
   **Then** the experiment shows the model and its version, and the model lists the
   experiment.
10. **Given** an experiment that is not of the kind "simulation", **When** the researcher
    sets a model on it, **Then** the tool offers to change its kind, and does nothing
    otherwise.
11. **Given** an input whose default is outside its own range, two inputs with the same
    name, or a model without a purpose, **When** the researcher saves, **Then** the tool
    rejects it and reports every problem together.
12. **Given** a model used by experiments that have runs, **When** the researcher deletes
    it, **Then** the tool refuses and lists those experiments.

---

### User Story 2 - Define scenarios (Priority: P2)

A researcher defines the situations to simulate. Each scenario is a named, complete setting
of the model's inputs — "baseline", "two servers", "arrival rate +20%" — together with how
long to simulate, how much of the beginning to discard as warm-up, and how many times to
repeat. A variant is defined by saying only how it differs from another scenario.

**Why this priority**: Scenarios are what a simulation study compares. They are needed
before anything can run, and they are where most of a study's design lives.

**Independent Test**: Define a baseline scenario, derive two variants each changing one
input, list the scenarios showing only their differences from the baseline, and confirm a
variant with a value outside the model's range is rejected.

**Acceptance Scenarios**:

1. **Given** a simulation experiment with a model, **When** the researcher defines a
   scenario with a name and values for the model's inputs, **Then** inputs not given take
   the model's defaults, and the scenario is stored with every input's value.
2. **Given** scenarios, **When** the researcher marks one as the baseline, **Then** it is
   the one others are compared against, and only one scenario is the baseline.
3. **Given** a scenario, **When** the researcher derives another from it, changing some
   inputs, **Then** the new scenario records what it was derived from and what differs.
4. **Given** a scenario, **When** the researcher sets its horizon (how much simulated time
   to cover), its warm-up period, its time step where the model needs one, and its number of
   replications, **Then** these are stored with the scenario.
5. **Given** several scenarios, **When** the researcher lists them, **Then** each is shown
   with how it differs from the baseline, its horizon, and its number of replications.
6. **Given** a scenario, **When** the researcher views it, **Then** every input is shown
   with its value, its unit, and whether the value is the model's default, inherited, or set
   here.
7. **Given** a scenario from which others are derived, **When** one of its inputs changes,
   **Then** derived scenarios that did not set that input follow the change, and the tool
   says which scenarios are affected before saving.
8. **Given** a value outside the model's range for that input, a value of the wrong kind,
   an input the model does not have, a warm-up longer than the horizon, or a number of
   replications below one, **When** the researcher saves, **Then** the tool rejects it and
   reports every problem together.
9. **Given** a new version of the model that removes or renames an input, **When** the
   experiment is moved to that version, **Then** the tool lists the scenarios that set the
   input and requires each to be resolved before any run.
10. **Given** a scenario that has replications, **When** the researcher changes it, **Then**
    the tool warns that existing replications were run with the earlier values, keeps what
    they recorded, and marks them as belonging to an earlier definition.
11. **Given** two scenarios with identical values for every input, **When** the second is
    saved, **Then** the tool warns that it duplicates the first.

---

### User Story 3 - Run replications with recorded seeds (Priority: P3)

A researcher runs a scenario. The tool carries out the experiment's pipeline once per
replication, handing the simulator the scenario's input values and a random seed, and
records for each replication the seed it used. The same scenario with the same seed can be
run again later and must give the same outcome; a different seed gives an independent one.
If the work is interrupted, finished replications are kept and only the rest are run.

**Why this priority**: Running is what produces outcomes, and the seed is what makes a
stochastic outcome repeatable. This is the core of the specification and needs the model
and scenarios before it.

**Independent Test**: Run a scenario with five replications, confirm five runs with five
different recorded seeds, interrupt a second batch part-way and resume it, and rerun one
replication with its seed.

**Acceptance Scenarios**:

1. **Given** a scenario with a number of replications, **When** the researcher runs it,
   **Then** one run per replication is created, each recording its scenario, its
   replication number, its seed, the model version, and the simulator version.
2. **Given** a batch of replications, **When** seeds are assigned, **Then** each replication
   has a different seed, derived from a base seed recorded with the batch so that the whole
   batch can be repeated.
3. **Given** a scenario, **When** the researcher gives the base seed or the exact seeds to
   use, **Then** those are used and recorded.
4. **Given** a replication, **When** it starts, **Then** the simulator receives the
   scenario's input values, the horizon, the warm-up, the time step, and the seed in the
   agreed way.
5. **Given** a replication in progress, **When** the simulator reports how far simulated
   time has advanced, **Then** progress is shown as simulated time covered out of the
   horizon.
6. **Given** a finished replication, **When** it is viewed, **Then** it shows the simulated
   time covered, how long it took, and its outputs, in addition to everything a run shows.
7. **Given** a batch that is interrupted, **When** the researcher resumes it, **Then**
   replications that finished are not repeated and the rest run with the seeds already
   assigned to them.
8. **Given** a scenario that already has replications, **When** the researcher asks for
   more, **Then** new replications continue the numbering with new seeds, never reusing a
   seed already used for that scenario.
9. **Given** several scenarios, **When** the researcher runs them together, **Then** every
   scenario gets its replications, and scenarios to be compared use the same seeds for the
   same replication number unless the researcher chooses otherwise.
10. **Given** a past replication, **When** the researcher reruns it, **Then** a new run is
    made with the same scenario values, model version, and seed, and linked to the
    original.
11. **Given** a replication that fails, **When** the batch ends, **Then** the failed
    replications are reported with their seeds and can be retried with the same seeds.
12. **Given** a model that uses no random numbers, **When** a scenario is run, **Then** one
    replication is made, no seed is assigned, and asking for more replications is refused
    with an explanation.
13. **Given** an experiment whose model version has scenarios still to be resolved, or
    whose pipeline is empty, **When** the researcher runs a scenario, **Then** the tool
    refuses and says why.

---

### User Story 4 - Collect outputs and compare scenarios (Priority: P4)

A researcher looks at what the simulations produced. For one replication: each output as a
single value or as a series over simulated time, with the warm-up left out. For one
scenario: each output summarized across its replications — mean, spread, and an interval
that says how precisely the mean is known. Across scenarios: each compared with the
baseline, saying whether the difference is larger than the noise. A summary can be recorded
as a result of the experiment.

**Why this priority**: One run of a stochastic model is an anecdote; the summary across
replications is the finding. This turns runs into something that can be reported.

**Independent Test**: With two scenarios of ten replications each, view one replication's
output series, the per-scenario summary of an output, and the comparison with the baseline;
then record the summary as a result and trace it back to its replications.

**Acceptance Scenarios**:

1. **Given** a finished replication, **When** its outputs are collected, **Then** each
   output the model declares is recorded for that replication, as a value or as a reference
   to its series, and declared outputs that are missing are reported.
2. **Given** an output that is a series, **When** the researcher views it for a
   replication, **Then** its summary is shown — number of points, first and last time,
   minimum, maximum, mean, and final value — computed after the warm-up.
3. **Given** a scenario with several replications, **When** the researcher asks for its
   summary, **Then** each output shows the number of replications, the mean, the spread,
   and an interval for the mean at a chosen level of confidence.
4. **Given** a scenario summary, **When** the interval is wider than a precision the
   researcher has stated, **Then** the tool says so and estimates how many more
   replications would be needed.
5. **Given** several scenarios, **When** the researcher compares them, **Then** each output
   is shown per scenario beside the baseline, with the difference, its interval, and
   whether the interval excludes zero.
6. **Given** scenarios run with the same seeds per replication number, **When** they are
   compared, **Then** the comparison pairs the replications and says that it did.
7. **Given** a scenario summary, **When** the researcher records it as a result, **Then** a
   result is created with the mean as its value and the half-width of the interval as its
   uncertainty, tied to all the replications it was computed from.
8. **Given** such a result, **When** the researcher asks where it came from, **Then** the
   scenario, the replications with their seeds, the model version, and the simulator
   version are shown.
9. **Given** a scenario whose replications include some run under an earlier definition of
   the scenario or an earlier model version, **When** it is summarized, **Then** those are
   left out by default and the tool says how many.
10. **Given** a scenario with one replication, **When** it is summarized, **Then** the value
    is shown, and the spread and interval are shown as not available.
11. **Given** failed or interrupted replications, **When** a scenario is summarized,
    **Then** they are left out and counted.
12. **Given** an output series, **When** the researcher exports it for a scenario, **Then**
    a table is produced with one column per replication, or with the mean and interval at
    each time.

---

### User Story 5 - Show why the model can be trusted (Priority: P5)

A researcher builds the case for the model. They check that it is **repeatable** (the same
seed gives the same outcome), that it is **computed correctly** (cases with a known answer
give that answer), and that it **resembles what it models** (its outputs are close to
observed data). Each check is recorded with its outcome, and the model carries a status —
unverified, verified, validated — that every result shows.

**Why this priority**: Reviewers ask this of every simulation study, and the answer is
usually scattered or missing. It needs runs and outputs, so it follows them.

**Independent Test**: Run the repeatability check on a scenario, define a verification case
with a known answer and a tolerance and run it, record a validation against an observed
dataset, and see the model's status change after each.

**Acceptance Scenarios**:

1. **Given** a scenario of a model that uses random numbers, **When** the researcher runs
   the repeatability check, **Then** the same seed is run twice and the tool reports
   whether every output was identical, naming those that were not.
2. **Given** a model, **When** the researcher defines a verification case — a setting of
   the inputs, an output, the expected value, a tolerance, and where the expected value
   comes from — **Then** it is stored with the model.
3. **Given** verification cases, **When** the researcher runs them, **Then** each is
   reported as passed or failed with the value obtained, the expected value, and the
   difference.
4. **Given** a model whose repeatability check and all verification cases pass for its
   current version, **When** they complete, **Then** the model's status for that version
   becomes "verified".
5. **Given** a model and an observed dataset, **When** the researcher records a validation —
   the scenario that corresponds to the observation, the output compared, the measure of
   agreement used, its value, the threshold of acceptance, and their judgement — **Then**
   it is stored with the model.
6. **Given** a verified model with at least one validation judged acceptable, **When** it is
   recorded, **Then** the model's status for that version becomes "validated".
7. **Given** a new version of the model, **When** it is recorded, **Then** its status starts
   again as "unverified", and the checks of the earlier version remain with that version.
8. **Given** a model, **When** the researcher asks for its credibility, **Then** its
   assumptions, the outcome and date of every check, and what is still missing for the next
   status are shown.
9. **Given** a result computed from a model, **When** it is shown or reported in a
   manuscript, **Then** the model's status at the time of the runs is shown with it.
10. **Given** a verification case that fails, **When** the model was "verified", **Then**
    the status returns to "unverified" and the tool says which case failed.
11. **Given** a model that uses no random numbers, **When** the repeatability check is
    run, **Then** the model is run twice with identical inputs and the outputs compared.
12. **Given** a tolerance below zero, an expected value of the wrong kind, or a validation
    without a judgement, **When** the researcher saves, **Then** the tool rejects it.

---

### User Story 6 - Explore uncertain inputs (Priority: P6)

A researcher asks which inputs matter. In a **sensitivity analysis** they vary one input at
a time across its range, holding the others at the baseline, and see how much each moves an
output, ranked from most to least influential. In a **sampled ensemble** they state how
uncertain each input is, let the tool draw many combinations, run them all, and see the
resulting spread of an output.

**Why this priority**: These are what a simulation study does after its main comparison:
they say how robust the conclusion is. They reuse scenarios, replications, and summaries,
and can come last.

**Independent Test**: Run a sensitivity analysis over three inputs at five levels each and
obtain the ranking for an output; then define an ensemble of fifty samples over two
uncertain inputs and obtain the distribution of the output.

**Acceptance Scenarios**:

1. **Given** a baseline scenario, **When** the researcher starts a sensitivity analysis
   naming the inputs to vary, the number of levels, and the replications per level,
   **Then** the tool shows how many runs this will take, asks for confirmation, and creates
   the scenarios and replications, grouped as one analysis.
2. **Given** a sensitivity analysis, **When** levels are chosen, **Then** each input is
   varied across the range the model states, or a range the researcher gives within it.
3. **Given** a finished sensitivity analysis, **When** the researcher asks for its outcome
   for an output, **Then** each input is shown with the output at its lowest and highest
   level, the swing between them, and its rank by swing.
4. **Given** a sensitivity analysis, **When** an input's swing is smaller than the interval
   of the baseline, **Then** it is marked as indistinguishable from noise.
5. **Given** a model, **When** the researcher states the uncertainty of some inputs — a
   range with every value equally likely, a most likely value with a spread, or a list of
   values with weights — **Then** these are stored for an ensemble.
6. **Given** stated uncertainties, **When** the researcher starts an ensemble with a number
   of samples, **Then** the tool draws that many combinations using a recorded seed, shows
   the number of runs, asks for confirmation, and runs them, grouped as one ensemble.
7. **Given** a finished ensemble, **When** the researcher asks for its outcome for an
   output, **Then** the mean, spread, chosen percentiles, minimum, and maximum of the output
   across samples are shown.
8. **Given** an ensemble or a sensitivity analysis that is interrupted, **When** it is
   resumed, **Then** finished runs are kept and only the rest are run.
9. **Given** an ensemble, **When** it is repeated with the same seed for drawing samples,
   **Then** the same combinations are drawn.
10. **Given** an outcome of either kind, **When** the researcher records it as a result or
    exports it, **Then** it is tied to the analysis and all its runs.
11. **Given** a number of levels below two, a number of samples below one, an uncertainty
    whose range falls outside the model's range, or weights that are negative, **When** the
    researcher saves, **Then** the tool rejects it.
12. **Given** an analysis that would take more runs than a stated limit, **When** it is
    started, **Then** explicit confirmation is always required.

---

### Edge Cases

- The simulator does not accept a seed, or ignores it: the repeatability check fails, and
  the tool says that replications of this model cannot be claimed to be repeatable.
- Two replications of a scenario end up with the same seed because the researcher gave it
  twice: the tool rejects the second.
- A replication's outputs differ on rerun with the same seed because the simulator version
  changed: the tool reports the version difference first, since that explains it.
- The model declares an output that the simulator did not produce: the replication is
  recorded as failed for that output and is left out of that output's summary only.
- An output series is empty, or ends before the warm-up does: its summary is shown as not
  available, and the replication is reported.
- An output series is very large: the tool records where it is and its summary, and never
  holds the series itself.
- A replication ends before the horizon (the model stopped early): it is recorded as
  finished early, with the simulated time reached, and is left out of summaries unless the
  researcher includes it.
- Every replication of a scenario gives exactly the same value: the spread is zero, the
  interval has no width, and the tool mentions that the model may not be using the seed.
- Outputs contain values that are not finite: they are reported and left out of summaries.
- The baseline scenario is deleted or another is marked baseline: comparisons already
  recorded as results keep the baseline they used; new comparisons use the new one.
- A scenario is derived from a scenario that is then deleted: it keeps its values and is no
  longer derived from anything.
- Scenarios are derived from each other in a loop: the tool rejects it.
- The model moves to a new version while a batch is paused: the batch finishes with the
  version it started with.
- Verification cases exist for an input the new model version removed: they are marked as
  not applicable to that version.
- A validation uses a dataset that later changes: the validation keeps the dataset version
  it used and is marked as based on an earlier version.
- A sensitivity analysis is asked for an input that has no range: the tool rejects it and
  asks for a range.
- An ensemble's uncertainties cover an input that scenarios also set: within the ensemble
  the drawn value is used, and the tool says so.
- Simulated time has no natural unit (steps or ticks): "steps" is used as the unit.
- A model has no random numbers and no time (a single calculation): scenarios, runs,
  verification, and sensitivity analysis still apply; replications, warm-up, and horizon do
  not, and the tool does not ask for them.
- An experiment's kind is changed away from "simulation" after scenarios exist: the tool
  refuses until the model is removed from it.

## Requirements *(mandatory)*

### Functional Requirements

#### Models

- **FR-001**: Users MUST be able to create, list, view, update, and delete models, each with
  a name, a purpose, a description, a kind (discrete-event, agent-based, system dynamics,
  equation-based, Monte Carlo, other), the unit of simulated time, and whether it uses
  random numbers.
- **FR-002**: Users MUST be able to declare a model's inputs, each with a name unique in the
  model, a description, a kind of value (number, whole number, yes/no, one of a list, text),
  a unit, a default, and the range or list of allowed values; a default MUST lie within
  what is allowed.
- **FR-003**: Users MUST be able to declare a model's outputs, each with a name unique in
  the model, a description, a unit, and whether it is a single value or a series over
  simulated time.
- **FR-004**: Users MUST be able to record a model's assumptions, each a statement with an
  optional justification and links to references that support it.
- **FR-005**: Users MUST be able to link a model to the simulator software that computes it,
  to methodologies, and to references that describe it.
- **FR-006**: Users MUST be able to record a new version of a model with a note of what
  changed; versions MUST be numbered in sequence, earlier versions MUST remain viewable,
  and users MUST be able to compare two versions by inputs, outputs, and assumptions.
- **FR-007**: Users MUST be able to set the model, at a stated version, that an experiment
  of the kind "simulation" simulates; the system MUST NOT attach a model to an experiment
  of another kind without changing its kind on the user's confirmation.
- **FR-008**: The system MUST refuse to delete a model, or a version of it, that runs refer
  to, and MUST refuse to change an experiment's kind away from "simulation" while it has a
  model.

#### Scenarios

- **FR-009**: Users MUST be able to create, list, view, update, and delete scenarios of a
  simulation experiment, each with a name unique in the experiment, a description, and a
  value for every input of the model; inputs not given MUST take the model's default.
- **FR-010**: Users MUST be able to mark exactly one scenario of an experiment as the
  baseline.
- **FR-011**: Users MUST be able to derive a scenario from another by stating only the
  inputs that differ; a derived scenario MUST follow later changes to inputs it did not set,
  the system MUST name the scenarios affected before such a change is saved, and MUST
  reject derivation that forms a loop.
- **FR-012**: Each scenario MUST carry a horizon, a warm-up period, a time step where the
  model needs one, and a number of replications; the warm-up MUST be shorter than the
  horizon and the number of replications at least one.
- **FR-013**: The system MUST show, for each scenario, how it differs from the baseline, and
  for each input whether its value is the model's default, inherited, or set in the
  scenario.
- **FR-014**: The system MUST reject a scenario value that is outside the model's allowed
  values, of the wrong kind, or for an input the model does not have, and MUST warn when a
  scenario has the same values as another.
- **FR-015**: When an experiment moves to a model version in which an input a scenario sets
  no longer exists or no longer allows its value, the system MUST list the scenarios
  concerned and MUST refuse to run them until each is resolved.
- **FR-016**: When a scenario with replications is changed, the system MUST warn, keep what
  existing replications recorded, and mark them as belonging to an earlier definition.
- **FR-017**: A scenario MUST be usable wherever `specs/004-experiments` uses a condition:
  to label runs, to group summaries, and in comparisons.

#### Replications and seeds

- **FR-018**: Users MUST be able to run a scenario; the system MUST carry out the
  experiment's pipeline once per replication and record with each run its scenario, the
  scenario's input values, replication number, seed, model version, and simulator version.
- **FR-019**: The system MUST assign each replication of a scenario a different seed,
  derived from a base seed recorded with the batch, so that the same base seed yields the
  same seeds; users MUST be able to give the base seed or the exact seeds.
- **FR-020**: The system MUST never use the same seed twice for the same scenario
  definition, except when a replication is deliberately rerun.
- **FR-021**: The system MUST hand the simulator the scenario's input values, horizon,
  warm-up, time step, and seed in a documented way that works for any simulator.
- **FR-022**: Users MUST be able to run several scenarios together; by default the same
  replication number MUST receive the same seed in every scenario, and users MUST be able
  to ask for independent seeds instead.
- **FR-023**: Users MUST be able to add replications to a scenario; numbering MUST continue
  and new seeds MUST be used.
- **FR-024**: Users MUST be able to rerun a past replication with the same scenario values,
  model version, and seed; the new run MUST be linked to the original.
- **FR-025**: An interrupted batch MUST be resumable; finished replications MUST NOT be
  repeated, and unfinished ones MUST keep the seeds assigned to them. Failed replications
  MUST be reported with their seeds and be retryable with the same seeds.
- **FR-026**: Each replication MUST record the simulated time covered and the time it took;
  when the simulator reports its advance, progress MUST be shown as simulated time covered
  out of the horizon.
- **FR-027**: For a model that uses no random numbers, the system MUST make one replication
  per scenario, assign no seed, and refuse further replications with an explanation.
- **FR-028**: The system MUST refuse to run a scenario whose experiment has no model, an
  empty pipeline, or scenarios awaiting resolution under FR-015.

#### Outputs and summaries

- **FR-029**: For each finished replication the system MUST record every output the model
  declares — a value, or for a series a reference to where it is and its fingerprint — and
  MUST report declared outputs that are missing.
- **FR-030**: For a series the system MUST show the number of points, the first and last
  time, and the minimum, maximum, mean, and final value, all computed after the warm-up;
  the system MUST NOT hold the series itself.
- **FR-031**: For a scenario the system MUST summarize each output across its replications
  with the number of replications, the mean, the spread, and an interval for the mean at a
  level of confidence the user chooses (95% by default).
- **FR-032**: Summaries MUST leave out, and count, replications that failed, were
  interrupted, ended before the horizon, contain values that are not finite, or were run
  under an earlier scenario definition or model version; users MUST be able to include the
  last two groups explicitly.
- **FR-033**: Users MUST be able to state a required precision for an output; when a
  summary's interval is wider, the system MUST say so and estimate the number of additional
  replications needed.
- **FR-034**: Users MUST be able to compare scenarios: for each output, each scenario beside
  the baseline, the difference, an interval for the difference, and whether it excludes
  zero; replications with the same seeds MUST be compared in pairs, and the comparison MUST
  state which method it used.
- **FR-035**: Users MUST be able to record a scenario summary or a comparison as a result of
  the experiment, with the mean as its value and the half-width of the interval as its
  uncertainty; the result MUST be tied to every replication it was computed from, and its
  provenance MUST show the scenario, seeds, model version, and simulator version.
- **FR-036**: Users MUST be able to export a series for a scenario as a table, per
  replication or as mean and interval at each time.
- **FR-037**: With a single replication, the system MUST show the value and show spread and
  interval as not available.

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

#### Sensitivity and ensembles

- **FR-045**: Users MUST be able to run a sensitivity analysis from a baseline scenario:
  named inputs are varied one at a time across a number of levels within the model's range
  or a narrower range given, with a number of replications per level; the scenarios and
  runs created MUST be grouped as one analysis.
- **FR-046**: For a sensitivity analysis and an output, the system MUST show for each input
  the output at its lowest and highest level, the swing between them, its rank by swing,
  and whether the swing is distinguishable from the baseline's interval.
- **FR-047**: Users MUST be able to state the uncertainty of model inputs — a range with
  equal likelihood, a most likely value with a spread, or listed values with weights — and
  run an ensemble of a given number of samples drawn from them with a recorded seed, so
  that the same seed draws the same combinations.
- **FR-048**: For an ensemble and an output, the system MUST show the mean, spread, chosen
  percentiles, minimum, and maximum across samples.
- **FR-049**: Before starting a sensitivity analysis or an ensemble the system MUST show the
  number of runs and require confirmation; both MUST be resumable without repeating
  finished runs, and their outcomes MUST be recordable as results and exportable, tied to
  the analysis and its runs.

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

### Key Entities *(include if feature involves data)*

- **Model**: A representation of something, to be simulated. Has a purpose, a kind, a unit
  of simulated time, inputs, outputs, assumptions, versions, a simulator, and a credibility
  status per version.
- **Model Input**: Something the model can be given: its kind of value, unit, default, and
  allowed values.
- **Model Output**: Something the model produces: a single value or a series over simulated
  time, with a unit.
- **Assumption**: A statement the model takes as true, with its justification.
- **Model Version**: The model as it was at one moment, with what changed since the previous
  one and its own checks.
- **Scenario**: A named, complete setting of a model's inputs within a simulation
  experiment, with horizon, warm-up, time step, and number of replications; possibly derived
  from another; one is the baseline. The simulation form of an experiment's condition.
- **Replication**: One run of one scenario with one seed; a run of the experiment with the
  scenario values, seed, model version, and simulated time added.
- **Seed**: The number that fixes a replication's random numbers, derived from a batch's
  base seed.
- **Batch**: A group of replications started together, with its base seed.
- **Output Record**: One output of one replication: a value, or a reference to a series with
  its summary.
- **Scenario Summary**: An output across the replications of a scenario: count, mean,
  spread, interval.
- **Comparison**: The difference of an output between a scenario and the baseline, with its
  interval.
- **Verification Case**: Inputs with a known answer, and the tolerance within which the
  model must give it.
- **Validation**: A recorded comparison of the model with observed data, with a measure of
  agreement and a judgement.
- **Sensitivity Analysis**: A group of scenarios varying inputs one at a time, and the
  ranking that results.
- **Input Uncertainty / Ensemble**: How uncertain inputs are, and a group of runs over
  combinations drawn from that uncertainty.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can record a model with five inputs, three outputs, and its assumptions
  in under 10 minutes, and derive a variant scenario from the baseline in under 30 seconds.
- **SC-002**: A user can start ten replications of a scenario with a single command, and
  100% of replications have a recorded seed that differs from every other seed of that
  scenario.
- **SC-003**: Rerunning any replication with its recorded seed, on the same model and
  simulator versions, gives identical outputs in 100% of cases for a simulator that honours
  its seed; when it does not, the repeatability check reports it in 100% of cases.
- **SC-004**: After an interruption, 100% of finished replications are kept and none is run
  again on resume.
- **SC-005**: For any reported summary, a user can list the exact replications, seeds, model
  version, and simulator version behind it in under 1 minute.
- **SC-006**: A user can see, for a scenario of 30 replications, the mean and interval of
  every output in under 5 seconds, and whether it differs from the baseline.
- **SC-007**: The mean, spread, and interval shown for a scenario agree with an independent
  calculation from the same values to the precision displayed, in 100% of cases.
- **SC-008**: A user can tell, for any model version, whether it is unverified, verified, or
  validated and what is missing for the next status, in under 30 seconds.
- **SC-009**: 100% of results computed from a model show the model's credibility status at
  the time of their runs.
- **SC-010**: A user can launch a sensitivity analysis over five inputs in under 3 minutes
  of their own time and obtain the ranking for any output in under 10 seconds once it
  finishes.
- **SC-011**: Repeating an ensemble with the same sampling seed draws the same combinations
  in 100% of cases.
- **SC-012**: Everything a user could do with an experiment before this specification still
  works identically for experiments that are not simulations.
- **SC-013**: 100% of invalid inputs are rejected before anything is stored or run, each
  with the invalid value named.
- **SC-014**: 90% of first-time users complete the primary task of each story on their
  first attempt using only that story's usage guide.

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
