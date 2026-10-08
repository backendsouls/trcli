### Edge Cases

- A project is created with the same title as an existing project: it is accepted with a
  warning, since identifiers distinguish them.
- The current project is deleted, completed, or abandoned: there is no current project
  afterwards, and the tool says so.
- A command is given for the current project and there is none: the tool asks which project
  is meant, or uses the only project if there is exactly one.
- A record is created in a project that is completed or abandoned: the tool refuses and
  offers to reopen the project.
- A supervisor recorded on a project is removed from the staff register: the project keeps
  the name, as other records do.
- A project's expected end date is moved earlier than existing milestones: the tool lists
  the milestones now outside the period and asks for confirmation.
- A milestone is deleted while tasks are attached: the tasks are kept and shown as attached
  to no milestone.
- A milestone is marked reached with a date in the future: the tool rejects it.
- A milestone marked reached is reopened: the reached date is cleared and the reopening is
  kept in its history.
- A task is moved to another project: its milestone is cleared, since milestones belong to
  one project, and its dependencies on tasks of the old project are kept and shown as
  cross-project.
- A task is linked to a record that is later deleted: the task is kept and names what it
  used to concern.
- A task's responsible person is not in the staff register: the tool rejects it and offers
  to add the person.
- A repeating task is cancelled: no further occurrence is created.
- A project has no start or expected end date: suggested milestone dates are not proposed,
  and time elapsed is not shown in progress.
- Proposed milestones are accepted twice for the same project: the tool reports those
  already present and adds none of them again.
- Two projects of the same degree type are active at once: both are allowed.
- A due date or target date is in an invalid form, or a title is empty or far too long: the
  tool rejects it, names the field, and shows a valid example.
- A project holds thousands of tasks: lists remain usable, shown in pages or limited to a
  stated number with a count of the rest.
- The to-do list is opened with hundreds of open tasks in one project: milestones with many
  tasks show the first ones and how many more there are, and can be opened in full.
- A task belongs to a milestone that is cancelled or already reached: it is shown under that
  milestone, marked, so that it is not lost.
- A sub-task is ticked whose parent is cancelled: accepted; the parent stays cancelled.
- Text typed for a new task begins with something that looks like a mark but is not one
  (an e-mail address, a price): it is kept as text.
- Two numbers given in one command are the same, or a number is given both to tick and to
  cancel: the tool acts once and says so, or refuses the contradictory pair and acts on the
  rest.
- A task is ticked by mistake: unticking restores the state it had before, including "in
  progress".
- A repeating task is ticked from the list: its next occurrence appears with a new number.
- The list is shown while another terminal changes tasks: the next opening shows the new
  state; the tool never shows a mix.
