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
