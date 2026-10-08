### Edge Cases

- A run is interrupted because the terminal closes or the machine shuts down: on the next
  use the run is shown as interrupted, not as succeeded or running, and can be resumed.
- A manual step is never confirmed: the run stays paused indefinitely, is listed as
  waiting, and can be cancelled.
- A manual step is confirmed by someone other than the assigned performer: it is accepted,
  and both names are recorded.
- A manual step is confirmed twice, or a run that is not paused is confirmed: the tool
  refuses and reports the run's actual state.
- An automated step never ends: it can be interrupted, and a time limit can be set after
  which it is stopped and recorded as failed.
- An automated step produces a very large amount of output: all of it is kept in the run's
  record, and only the most recent part is shown on screen.
- A step declares an output file that does not exist when the step ends: the step is
  recorded as failed, naming the missing file.
- The pipeline is changed while a run of it is paused: the run continues with the pipeline
  as it was when the run started.
- A dataset version, methodology, or piece of software linked to the experiment is changed
  or removed after a run: the run keeps what it recorded; removal is refused while runs
  refer to it.
- A run that results, figures, or tables were recorded from is deleted: the tool refuses
  while any of them depends on it.
- A sweep would create an unreasonably large number of runs: the tool shows the count and
  requires explicit confirmation above a stated limit.
- A parameter's allowed values are narrowed after runs used a value that is no longer
  allowed: past runs keep their values and are marked as outside the current definition.
- A metric value is not a finite number: the tool rejects it.
- Two results with the same name are recorded for the same run: the tool rejects the
  second and offers to replace the first.
- A condition is removed from the design after runs were made for it: the runs keep the
  condition's name and are shown as belonging to a removed condition.
- A replication's design is changed: it is still a replication, and the comparison reports
  the design differences alongside the results.
- A hypothesis tested by the experiment is deleted: the experiment keeps running; the link
  is removed after confirmation.
- An experiment is concluded while a run is still paused or in progress: the tool refuses
  and names the run.
- Values are given in an invalid form — an empty name, a negative replicate count, a number
  where text is expected: the tool rejects them all together, each with what is expected.
