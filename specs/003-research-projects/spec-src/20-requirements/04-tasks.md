#### Tasks

- **FR-028**: Users MUST be able to create, list, view, update, and delete tasks in a
  project, each with a title, description, status (to do, in progress, done, cancelled),
  priority (low, normal, high, urgent), due date, estimated effort, and responsible person.
- **FR-029**: The responsible person MUST be chosen from the workspace's staff register.
- **FR-030**: Users MUST be able to attach a task to one milestone of the same project, and
  the milestone MUST show its tasks and how many are done.
- **FR-031**: Users MUST be able to link a task to any records it concerns, with the link
  visible from both ends.
- **FR-032**: Users MUST be able to give a task sub-tasks, and the task MUST show how many
  are done.
- **FR-033**: Users MUST be able to make a task depend on other tasks; the system MUST show
  a task as blocked while a task it depends on is unfinished, naming it.
- **FR-034**: The system MUST reject dependency loops and a task made a sub-task of itself
  or of its own sub-task.
- **FR-035**: The system MUST record the date of every status change and the completion date
  of a task marked done.
- **FR-036**: The system MUST warn when a task is marked done with unfinished sub-tasks, and
  when a task is due after the target date of its milestone.
- **FR-037**: Users MUST be able to filter tasks by status, priority, milestone, responsible
  person, due date, and linked record, and sort by due date or priority.
- **FR-038**: Users MUST be able to make a task repeat at a stated interval; completing an
  occurrence MUST create the next, and cancelling it MUST stop the repetition.
- **FR-039**: Users MUST be able to move a task to another project; the system MUST clear
  its milestone and show remaining dependencies as cross-project.
