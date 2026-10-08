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
