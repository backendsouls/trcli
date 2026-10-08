### User Story 1 - Define and design an experiment (Priority: P1)

A researcher records an experiment: its name, its objective, the hypotheses it tests, and
its kind (computational, laboratory, field, survey, or simulation). They then describe its
design: the variables they will change (independent), the variables they will measure
(dependent), the variables they will hold fixed (controlled), and the conditions to compare,
one of which may be the control.

**Why this priority**: An experiment must exist before it can have a pipeline or a run.
With only this story, a researcher already has a register of what they plan to test, how,
and against which hypothesis.

**Independent Test**: Create an experiment linked to a hypothesis, add one independent and
one dependent variable and two conditions including a control, and view the experiment
showing its full design.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates an experiment with a name and an
   objective, **Then** it is stored with the status "planned".
2. **Given** an experiment and existing hypotheses, **When** the researcher links the
   hypotheses it tests, **Then** the experiment lists them and each hypothesis lists the
   experiment.
3. **Given** an experiment, **When** the researcher sets its kind, **Then** the kind is shown
   and experiments can be listed by kind.
4. **Given** an experiment, **When** the researcher adds a variable with a name, a role
   (independent, dependent, or controlled), a unit, and a description, **Then** it is stored
   as part of the design.
5. **Given** an experiment, **When** the researcher adds conditions, each with a name and
   the values of the independent variables it uses, and marks one as the control, **Then**
   the design shows the conditions and which is the control.
6. **Given** an experiment, **When** the researcher records the planned sample size or the
   planned number of replicates per condition, **Then** it is shown with the design.
7. **Given** an experiment, **When** the researcher links a methodology and dataset
   versions, **Then** they are shown and will be recorded with every run.
8. **Given** experiments, **When** the researcher lists them filtered by status, kind,
   hypothesis, or tag, **Then** only matching experiments are shown.
9. **Given** a condition that gives a value to a variable that is not an independent
   variable of the design, or two variables with the same name, **When** the researcher
   saves, **Then** the tool rejects it and names the problem.
10. **Given** an experiment with runs, **When** the researcher deletes it, **Then** the tool
    refuses and says how many runs exist.
11. **Given** an experiment with runs, **When** the researcher changes its design, **Then**
    the tool warns that past runs were made under the earlier design, keeps what each run
    recorded, and requires confirmation.

---
