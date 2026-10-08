#### Experiment design

- **FR-007**: Users MUST be able to record an experiment's variables, each with a name
  unique within the experiment, a role (independent, dependent, controlled), a unit, and a
  description.
- **FR-008**: Users MUST be able to record conditions, each with a name unique within the
  experiment and a value for one or more independent variables, and mark at most one
  condition as the control.
- **FR-009**: The system MUST reject a condition that gives a value to a variable that is
  not an independent variable of the same experiment.
- **FR-010**: Users MUST be able to record the planned sample size and the planned number of
  replicates per condition.
- **FR-011**: When the design of an experiment that has runs is changed, the system MUST
  warn, require confirmation, and leave what each past run recorded unchanged.
