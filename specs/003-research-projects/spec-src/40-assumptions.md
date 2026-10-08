## Assumptions

- **A workspace holds many projects.** The base specification described a workspace as "one
  research project or programme"; with this specification a workspace is the researcher's
  whole body of work and a project is one undertaking inside it. The staff register, the
  audit trail, and telemetry stay at the workspace level.
- **Records are shared, not copied.** A record may belong to several projects, and there is
  one copy of it. A researcher who wants two projects fully separated uses two workspaces.
- **Milestones and tasks belong to exactly one project.** A task can be moved; a milestone
  cannot.
- **"Independent Research" is the English name chosen for "Free Research"**, and "Capstone
  Project" for TCC (*Trabalho de Conclusão de Curso*); "Undergraduate Research" is read as
  supervised research during an undergraduate degree (as in *iniciação científica*), which
  is distinct from the final-year capstone. Type names can be reworded without changing
  behavior.
- **"etc." in the request** is covered by adding Postdoctoral and Funded Project as built-in
  types and by custom types for anything else, such as specialization courses or research
  internships.
- **Proposed milestones are generic starting points.** Degree structures differ by country
  and institution, so proposals are always shown for acceptance and are editable; the tool
  does not claim to know any programme's rules. The initial proposals are:
  - *Independent Research*: none.
  - *Undergraduate Research*: work plan approved, interim report, final report, results
    presented.
  - *Capstone Project*: topic and supervisor defined, proposal approved, draft delivered to
    supervisor, final text submitted, presentation and defense, final version deposited.
  - *Master's*: coursework completed, proposal defended (qualifying), dissertation
    submitted, dissertation defended, final version deposited.
  - *Doctoral (PhD)*: coursework completed, qualifying exam, proposal defended, thesis
    submitted, thesis defended, final version deposited.
  - *Postdoctoral*: work plan approved, interim report, final report.
  - *Funded Project*: kickoff, interim report, final report.
- **This specification supersedes the tasks and milestones of
  `specs/002-research-lifecycle`** (its User Story 2 and FR-009 to FR-011). What remains
  there is the wider "what's due" view that also gathers deadlines held by other record
  types (grants, approvals, venue calls); it extends the view defined here.
- **Planning, not scheduling.** There are no calendars, reminders or notifications sent
  outside the tool, time tracking, charts of task dependencies over time, or workload
  balancing between people. Estimated effort is recorded and not otherwise used.
- **Single researcher.** As in the base workspace, one person uses the tool; "responsible
  person" and "supervisor" name people from the staff register and do not give them access.
- **Dates are calendar dates** without times of day or time zones.
- **Interface language is English.**
- **The to-do list is a view, not a second system.** It was added to this specification on
  2026-10-08 as User Story 7. It shows and changes the same projects, milestones, and tasks
  the other stories define; it introduces a star on tasks and a short number for each, and
  nothing else that is stored.
- **"Beautiful" is specified by what the reader must be able to see**, not by a particular
  look: structure visible at a glance, states told by the checkbox, a few consistent marks,
  quiet finished work, one summary line. The exact symbols and colors are chosen at
  planning time, in the spirit of the board-and-checkbox to-do lists popular in terminals,
  and can be changed by the user.
- **Quick notes are not part of the to-do list.** A thought that is not yet a task is
  captured with the inbox of `specs/013-ideas-questions` and turned into a task there.
- **Other dated things stay in the view of what is due.** Deadlines of grants, calls,
  requirements, and reviews appear in `trcli due`; the to-do list shows tasks and
  milestones only.
