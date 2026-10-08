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
