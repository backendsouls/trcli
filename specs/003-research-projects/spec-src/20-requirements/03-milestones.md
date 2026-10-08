#### Milestones

- **FR-019**: Users MUST be able to create, list, view, update, and delete milestones in a
  project, each with a title, description, target date, and status (upcoming, reached,
  missed, cancelled).
- **FR-020**: Each built-in project type MUST propose a set of typical milestones; when a
  project is created, the system MUST show the proposal and MUST add only the milestones
  the user accepts.
- **FR-021**: When the project has start and expected end dates, accepted milestones MUST
  receive suggested target dates across that period, marked as suggestions until the user
  confirms or changes them.
- **FR-022**: Users MUST be able to mark a milestone as required by the programme and record
  what must be delivered for it.
- **FR-023**: The system MUST keep the history of changes to a milestone's target date, with
  the reason given for each change.
- **FR-024**: Marking a milestone reached MUST record the date; the system MUST reject a
  reached date in the future and MUST show the delay when it is after the target date.
- **FR-025**: The system MUST show a milestone as overdue, with the number of days, when its
  target date has passed and it is neither reached nor cancelled.
- **FR-026**: The system MUST present a project's milestones as a timeline in date order
  with their status and the next one identified.
- **FR-027**: The system MUST warn, and require confirmation, when a milestone's target date
  falls outside the project's period, and when a milestone with unfinished tasks is marked
  reached.
