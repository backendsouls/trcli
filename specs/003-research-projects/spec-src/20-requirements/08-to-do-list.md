#### To-do list

- **FR-053**: The system MUST provide a to-do list that presents the projects, milestones,
  tasks, and sub-tasks of this specification as they are structured: each project as a
  board, each of its milestones as a section, tasks under the milestone they serve, tasks
  with no milestone under a section of their own, and sub-tasks indented beneath their
  task, to any depth.
- **FR-054**: The to-do list MUST NOT be a separate store of items: every line MUST be a
  task or milestone record of this specification, and every change made through the list
  MUST be the same change as made through the task and milestone commands, validated and
  audited in the same way.
- **FR-055**: Each task line MUST show a checkbox whose form tells its state — to do, in
  progress, done, cancelled — the task's short number, and its title; and, where they
  apply, marks for a star, the priority, the due date as time remaining or overdue, being
  blocked, the responsible person, and having notes.
- **FR-056**: Each board MUST show the project's title and its tasks done out of total; each
  milestone section MUST show its title, target date, time remaining or overdue, whether it
  is required, and its tasks done out of total.
- **FR-057**: The list MUST end with a summary: the percentage of listed tasks that are
  done, and the counts done, in progress, pending, and blocked.
- **FR-058**: Every task MUST have a short number, shown in the list, by which it can be
  named in commands; a task's number MUST NOT change while the task is open and MUST NOT be
  given to another task.
- **FR-059**: Users MUST be able to add a task by giving only its text; it MUST be added to
  the current project with the status "to do" and confirmed in one line with its number.
  With no current project and several projects, the system MUST ask which, or accept a
  project named in the command.
- **FR-060**: When adding from the to-do list, users MUST be able to set a task's project,
  milestone, parent task, priority, due date, star, and responsible person by short marks
  typed with the text; the marks MUST be removed from the title, and an unrecognized mark
  MUST be kept as text.
- **FR-061**: Users MUST be able to tick, untick, start, cancel, star, unstar, set the
  priority of, move, and delete tasks by one or several numbers in one command; the system
  MUST report what changed for each, MUST report each number that matched nothing or needed
  no change, and MUST still act on the others.
- **FR-062**: Ticking a task with unticked sub-tasks MUST ask first; when the last sub-task
  of a task is ticked, the system MUST offer to tick the task. Ticking the last open task
  of a milestone MUST offer to mark the milestone reached and MUST do so only on acceptance.
- **FR-063**: Users MUST be able to star tasks to single them out, independently of
  priority, and to list only starred tasks.
- **FR-064**: By default the list MUST show the current project when there is one and all
  active projects otherwise; users MUST be able to ask for all projects, one project, or
  completed and on-hold projects.
- **FR-065**: Tasks completed within a recent period MUST be shown ticked and visually
  quieter than open tasks; tasks completed earlier and cancelled tasks MUST be left out
  unless asked for, and the summary MUST say how many are hidden.
- **FR-066**: The system MUST provide a timeline view of the same tasks and milestones
  grouped by date — overdue, today, tomorrow, the rest of this week, later, and without a
  date — each naming its project.
- **FR-067**: Users MUST be able to narrow the list to starred tasks, pending tasks, a
  milestone, a responsible person, a priority, a tag, or tasks whose text contains given
  words; boards and sections with no matching task MUST be left out.
- **FR-068**: A milestone that is reached, and a project with every task done, MUST be shown
  as complete in a way that distinguishes achievement from mere absence of work; an empty
  list MUST say so and show how to add a task.
- **FR-069**: The to-do list MUST be pleasant and quick to read: aligned columns, consistent
  symbols, restrained use of color in which each color has one meaning, open work visually
  ahead of finished work, and no more than one line per task unless details are asked for.
- **FR-070**: Every state and mark MUST be distinguishable without color and without special
  symbols: where a terminal cannot show them, or the user turns them off, plain characters
  MUST be used; lines MUST be shortened with a visible mark to fit the width available.
- **FR-071**: The list MUST be available in a structured form meant for other programs,
  nested as projects, milestones, tasks, and sub-tasks, with no decoration.
- **FR-072**: The symbols, the period for which completed tasks remain visible, and whether
  the default view is the board or the timeline MUST be configurable.
