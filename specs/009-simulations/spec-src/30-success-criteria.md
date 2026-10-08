## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can record a model with five inputs, three outputs, and its assumptions
  in under 10 minutes, and derive a variant scenario from the baseline in under 30 seconds.
- **SC-002**: A user can start ten replications of a scenario with a single command, and
  100% of replications have a recorded seed that differs from every other seed of that
  scenario.
- **SC-003**: Rerunning any replication with its recorded seed, on the same model and
  simulator versions, gives identical outputs in 100% of cases for a simulator that honours
  its seed; when it does not, the repeatability check reports it in 100% of cases.
- **SC-004**: After an interruption, 100% of finished replications are kept and none is run
  again on resume.
- **SC-005**: For any reported summary, a user can list the exact replications, seeds, model
  version, and simulator version behind it in under 1 minute.
- **SC-006**: A user can see, for a scenario of 30 replications, the mean and interval of
  every output in under 5 seconds, and whether it differs from the baseline.
- **SC-007**: The mean, spread, and interval shown for a scenario agree with an independent
  calculation from the same values to the precision displayed, in 100% of cases.
- **SC-008**: A user can tell, for any model version, whether it is unverified, verified, or
  validated and what is missing for the next status, in under 30 seconds.
- **SC-009**: 100% of results computed from a model show the model's credibility status at
  the time of their runs.
- **SC-010**: A user can launch a sensitivity analysis over five inputs in under 3 minutes
  of their own time and obtain the ranking for any output in under 10 seconds once it
  finishes.
- **SC-011**: Repeating an ensemble with the same sampling seed draws the same combinations
  in 100% of cases.
- **SC-012**: Everything a user could do with an experiment before this specification still
  works identically for experiments that are not simulations.
- **SC-013**: 100% of invalid inputs are rejected before anything is stored or run, each
  with the invalid value named.
- **SC-014**: 90% of first-time users complete the primary task of each story on their
  first attempt using only that story's usage guide.
