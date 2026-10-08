### User Story 7 - Conclude and replicate an experiment (Priority: P7)

When the runs are done, a researcher records what the experiment showed: an outcome for
each hypothesis it tested, a written conclusion, the limitations, and what to do next. The
experiment is then closed. Later, they or a colleague can replicate it: a new experiment is
created from the original's design and pipeline, linked back to it, and the two can be
compared.

**Why this priority**: A conclusion is what turns a set of runs into knowledge, and
replication is how that knowledge is tested. Both come after everything else exists.

**Independent Test**: Conclude an experiment with an outcome and a conclusion, confirm it
is closed, create a replication of it, and view the original showing its replication.

**Acceptance Scenarios**:

1. **Given** an experiment with at least one succeeded run, **When** the researcher records
   a conclusion with an outcome for each tested hypothesis (supported, refuted,
   inconclusive), the supporting results, the limitations, and next steps, **Then** it is
   stored and the experiment's status becomes "completed".
2. **Given** a conclusion that names an outcome for a hypothesis, **When** it is recorded,
   **Then** the tool offers to update that hypothesis's status accordingly and does so only
   when the researcher accepts.
3. **Given** a completed experiment, **When** the researcher tries to start a run or change
   its design or pipeline, **Then** the tool refuses and offers to reopen it.
4. **Given** an experiment with no succeeded run, **When** the researcher tries to conclude
   it, **Then** the tool refuses, unless the experiment is being abandoned.
5. **Given** an experiment, **When** the researcher abandons it with a reason, **Then** its
   status becomes "abandoned" and the reason is recorded.
6. **Given** a completed or abandoned experiment, **When** the researcher reopens it,
   **Then** it becomes active again and the earlier conclusion is kept in its history.
7. **Given** an experiment, **When** the researcher creates a replication of it, **Then** a
   new experiment is created with a copy of the design, pipeline, parameters, and links to
   hypotheses, methodology, and software, and each experiment shows the other.
8. **Given** an experiment and its replications, **When** the researcher compares them,
   **Then** results with the same name are shown side by side with whether they agree.
9. **Given** an experiment, **When** the researcher exports its report, **Then** a document
   is produced with the objective, design, pipeline, runs, results, and conclusion.

---
