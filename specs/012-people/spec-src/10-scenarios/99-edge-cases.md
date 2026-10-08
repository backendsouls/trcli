### Edge Cases

- The lab head is themselves a person in the register: they are, and "my" views (whom I
  supervise, my meetings) use the person the workspace's researcher is set to.
- A person has two positions at once (a technician who is also a master's student): both
  are recorded, each with its own dates.
- A person has no expected end date (a permanent member): they never appear among those
  leaving.
- A person's funding ends before their expected end: both dates are shown, and the gap is
  marked.
- An expected end date passes and the person is still a current member: they are shown as
  past their expected end until the lab head updates the date or starts a handover.
- Two people have the same name: both are accepted after the duplicate warning; their
  identifiers and affiliations tell them apart wherever a person is chosen.
- A person's name changes: the new name is used from then on; what was already published or
  recorded keeps the name it had.
- A person is supervised by someone outside the group: the supervisor is added as an outside
  collaborator.
- A supervision would form a loop (A supervises B, B supervises A): the tool rejects it.
- A meeting is recorded with nobody but the lab head: accepted, as a note to self.
- An agreed action is given to a former member: accepted with a warning.
- A person leaves while a run is paused waiting for them at a manual step: the handover
  lists the step; whoever takes it can confirm it.
- A person holds nothing when they leave: the handover says so and completes at once.
- A handover is started and the person stays after all: it is cancelled, and items already
  moved stay moved unless moved back.
- A former member is named as an author of a new manuscript: allowed, without warning, since
  authorship outlives membership.
- The group is one person: every view works and simply shows one row.
- The overview is asked for with fifty people: it remains one table, sortable, shown in
  pages.
- Personal details are requested to be forgotten: contact details and private notes are
  removed; the name remains only where the record of the research needs it.
