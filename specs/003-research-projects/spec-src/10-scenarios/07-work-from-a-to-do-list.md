### User Story 7 - Work from a to-do list (Priority: P7)

A researcher opens their to-do list with one short command and sees, at a glance and
pleasant to look at, everything there is to do: each project as a board, each milestone as
a section within it with its date and its count of tasks done, each task as a line with a
checkbox — empty, in progress, or ticked — its sub-tasks indented beneath it, and small
marks for what is starred, urgent, blocked, or overdue. At the bottom, one line says how
much of it all is done. They add a task by typing its text, tick tasks off by their
numbers, and star the ones for today. The list is not a separate thing to maintain: it is
the projects, milestones, and tasks of this specification, shown as a list.

**Why this priority**: The earlier stories give tasks and milestones their rules; this one
gives them a face that is opened many times a day. It adds no new kind of record, so it
comes after them — but it is likely to be the most used command of all, and can be
delivered as soon as tasks exist.

**Independent Test**: With two projects, three milestones, and a dozen tasks in different
states, open the to-do list and confirm that projects, milestones, tasks, and sub-tasks
appear nested as they are, with correct counts and marks; add a task with one command,
tick two tasks by number, star one, and confirm the list and the summary line change
accordingly.

**Acceptance Scenarios**:

1. **Given** projects with milestones and tasks, **When** the researcher opens the to-do
   list, **Then** each active project is shown as a board with its title and its count of
   tasks done out of total; within it each milestone as a section with its title, target
   date, time remaining or overdue, and its own count; and within each milestone its tasks,
   with tasks that belong to no milestone under their own section.
2. **Given** tasks in different states, **When** they are listed, **Then** each shows a
   checkbox that tells its state at a glance — to do, in progress, done, cancelled — its
   title, and its sub-tasks indented beneath it with their own checkboxes.
3. **Given** tasks with a due date, a priority, a star, a dependency that blocks them, or a
   person responsible, **When** they are listed, **Then** each of these is shown by a small,
   consistent mark beside the task, and overdue tasks are unmistakable.
4. **Given** the to-do list, **When** it is shown, **Then** it ends with a summary: the
   share of all listed tasks that are done, and the numbers done, in progress, pending, and
   blocked.
5. **Given** a current project, **When** the researcher opens the to-do list without saying
   more, **Then** only that project's board is shown; all projects are shown on request,
   and with no current project.
6. **Given** the to-do list, **When** the researcher types a new task as plain text with one
   short command, **Then** it is added to the current project with the status "to do", and
   the tool confirms in one line with the task's number.
7. **Given** a new task typed with a mark for its milestone, its priority, its due date, or
   a star, **When** it is added, **Then** those are set from the marks, and the rest of the
   text is the title.
8. **Given** the numbers shown beside tasks, **When** the researcher ticks, unticks, starts,
   stars, or cancels tasks by giving one or several numbers, **Then** each task changes
   accordingly and the tool confirms what changed.
9. **Given** a task with sub-tasks, **When** all its sub-tasks are ticked, **Then** the tool
   offers to tick the task itself; ticking a task with unticked sub-tasks asks first.
10. **Given** completed tasks, **When** the to-do list is shown, **Then** tasks completed
    recently are shown ticked and visually quieter than open ones, and older completed
    tasks are left out unless asked for.
11. **Given** the to-do list, **When** the researcher asks for the timeline view, **Then**
    the same tasks and milestones are shown grouped by day — overdue, today, tomorrow, this
    week, later, and without a date — each still naming its project.
12. **Given** the to-do list, **When** the researcher asks for only starred tasks, only
    those of a person, only those of a milestone, only what is pending, or those matching a
    word, **Then** only matching tasks are shown, with the boards and sections they belong
    to.
13. **Given** a milestone reached or a project with nothing left to do, **When** the list is
    shown, **Then** it is shown as complete, in a way that reads as an achievement.
14. **Given** an empty workspace, or a project with no tasks, **When** the to-do list is
    opened, **Then** it says so in a friendly line and shows how to add the first task.
15. **Given** a terminal that cannot show color or special symbols, or output that is sent
    to a file or another program, **When** the to-do list is produced, **Then** it is
    readable in plain characters, every state is still distinguishable without color, and
    nothing decorative is left in output meant for programs.
16. **Given** a narrow terminal or very long titles, **When** the list is shown, **Then**
    lines are shortened with a visible mark and the layout stays aligned.
17. **Given** a number that matches no task, or a number given for a task already in the
    state asked for, **When** the researcher acts on it, **Then** the tool says so for that
    number and still acts on the other numbers given.
18. **Given** changes made through the to-do list, **When** the same tasks are viewed with
    the task commands, **Then** they show the same state, since both are the same records.

---
