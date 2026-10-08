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
