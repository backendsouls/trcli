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
