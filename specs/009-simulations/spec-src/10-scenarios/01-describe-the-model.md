### User Story 1 - Describe the model (Priority: P1)

A researcher records the model they simulate: what it represents and what it is for, what
kind of model it is, the assumptions it makes, the inputs it takes with their units and
plausible ranges, the outputs it produces, and the unit in which its time is counted. When
the model changes, they record a new version, so that every later run knows which model it
used.

**Why this priority**: A simulation result means nothing without the model behind it. The
model record is the smallest thing that is useful — a written, versioned account of what is
being simulated — and everything else in this specification refers to it.

**Independent Test**: Record a model with three inputs, two outputs, and four assumptions,
attach it to a simulation experiment, change an assumption, record a second version, and
view the differences between the versions.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records a model with a name, a purpose,
   and a kind (discrete-event, agent-based, system dynamics, equation-based, Monte Carlo,
   or other), **Then** it is stored at version 1 with the credibility status "unverified".
2. **Given** a model, **When** the researcher adds an input with a name, a description, a
   kind of value, a unit, a default, and the range or list of values it may take, **Then**
   it is stored as part of the model.
3. **Given** a model, **When** the researcher adds an output with a name, a description, a
   unit, and whether it is a single value or a series over simulated time, **Then** it is
   stored as part of the model.
4. **Given** a model, **When** the researcher records its assumptions, each as a statement
   with an optional justification and the references supporting it, **Then** they are
   listed with the model.
5. **Given** a model, **When** the researcher records the unit of simulated time and
   whether the model uses random numbers, **Then** these are shown with the model.
6. **Given** a model and the simulator software that computes it, **When** the researcher
   links them, **Then** the model shows its simulator and every run records the simulator's
   version.
7. **Given** a model, **When** the researcher changes its inputs, outputs, or assumptions
   and records a new version with a note of what changed, **Then** the version number
   advances and the earlier version remains viewable.
8. **Given** two versions of a model, **When** the researcher compares them, **Then** the
   inputs, outputs, and assumptions added, removed, and changed are listed.
9. **Given** a simulation experiment, **When** the researcher sets the model it simulates,
   **Then** the experiment shows the model and its version, and the model lists the
   experiment.
10. **Given** an experiment that is not of the kind "simulation", **When** the researcher
    sets a model on it, **Then** the tool offers to change its kind, and does nothing
    otherwise.
11. **Given** an input whose default is outside its own range, two inputs with the same
    name, or a model without a purpose, **When** the researcher saves, **Then** the tool
    rejects it and reports every problem together.
12. **Given** a model used by experiments that have runs, **When** the researcher deletes
    it, **Then** the tool refuses and lists those experiments.

---
