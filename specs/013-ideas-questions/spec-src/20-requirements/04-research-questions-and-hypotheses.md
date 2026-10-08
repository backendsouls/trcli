#### Research questions and hypotheses

- **FR-026**: Users MUST be able to create, list, view, update, and delete research
  questions, each with a statement, a motivation, an importance, a feasibility, and a
  status (open, answered, abandoned), and arrange them as questions and sub-questions.
- **FR-027**: Users MUST be able to record hypotheses under a research question, each with a
  statement, what would support or refute it, and a status (untested, supported, refuted,
  inconclusive).
- **FR-028**: Users MUST be able to link references, literature reviews, experiments,
  results, manuscripts, methodologies, datasets, and any other record to research questions
  and hypotheses, with every link visible from both ends.
- **FR-029**: The system MUST show, for any research question, its sub-questions, its
  hypotheses with their status, the ideas it grew from, and every linked record grouped by
  kind.
- **FR-030**: The system MUST require at least one linked result before a hypothesis can be
  marked supported, refuted, or inconclusive, and a closing note before a question can be
  marked answered or abandoned; it MUST keep the history of status changes and of changes
  to a statement's wording.
- **FR-031**: Users MUST be able to list the records that are linked to no research
  question, and the open questions that have had no activity for a stated period.
- **FR-032**: The system MUST reject a question hierarchy that contains a loop, and MUST
  refuse to delete a question that has sub-questions.
- **FR-033**: When a result a hypothesis relied on is replaced or removed, the system MUST
  flag the hypothesis for another look and MUST NOT change its status by itself.
- **FR-034**: When an experiment is concluded with an outcome for a hypothesis, the system
  MUST offer to update the hypothesis and MUST do so only on acceptance.
