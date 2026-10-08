### User Story 7 - Pre-register a hypothesis and analysis plan (Priority: P7)

Before running an experiment, a researcher freezes the hypothesis, the planned analysis, the
planned sample size, and the criteria for stopping, with a timestamp. Afterwards, anyone can
see what was planned, what was actually done, and every deviation between the two.

**Why this priority**: Pre-registration is how researchers show that the analysis was not
chosen after seeing the data. It depends on hypotheses, experiments, and runs existing.

**Independent Test**: Pre-register a plan for an experiment, try to change it, run the
experiment with a different sample size, and produce a report showing the deviation.

**Acceptance Scenarios**:

1. **Given** a hypothesis and an experiment, **When** the researcher writes a
   pre-registration with the analysis plan, **Then** it is stored as a draft that can still
   be edited.
2. **Given** a draft pre-registration, **When** the researcher freezes it, **Then** its
   content and the time are sealed and it can no longer be changed.
3. **Given** a frozen pre-registration, **When** the researcher needs to change the plan,
   **Then** they add a dated amendment with a reason; the original remains visible.
4. **Given** a frozen pre-registration, **When** a run started before the freeze time is
   linked to it, **Then** the tool flags that the run predates the registration.
5. **Given** a pre-registration and finished runs, **When** the researcher requests a
   comparison, **Then** the tool lists every recorded difference between plan and practice.
6. **Given** a frozen pre-registration, **When** it is altered outside the tool, **Then** an
   integrity check reports it.
7. **Given** a frozen pre-registration, **When** the researcher exports it, **Then** a
   document is produced that shows the content, the freeze time, and any amendments.

---
