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
