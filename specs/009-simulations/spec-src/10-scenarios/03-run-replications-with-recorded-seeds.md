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
