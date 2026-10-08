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
