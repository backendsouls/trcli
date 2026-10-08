#### Projects

- **FR-001**: Users MUST be able to create, list, view, update, and delete projects within a
  workspace, each with a title, a type, a goal or description, and a short identifier that
  is unique in the workspace and stable for the life of the project.
- **FR-002**: The system MUST provide these project types: Independent Research,
  Undergraduate Research, Capstone Project, Master's, Doctoral (PhD), Postdoctoral, and
  Funded Project.
- **FR-003**: For degree types (Undergraduate Research, Capstone Project, Master's,
  Doctoral), users MUST be able to record the institution, programme, degree sought,
  supervisor, co-supervisors, examining committee, start date, and expected end date.
- **FR-004**: For non-degree types, the system MUST NOT require academic details; for Funded
  Project, users MUST be able to record the funder and the grant reference.
- **FR-005**: Supervisors, co-supervisors, and committee members MUST be chosen from the
  workspace's staff register, and the project MUST keep their names if they are later
  removed from it.
- **FR-006**: The system MUST track each project's status (planned, active, on hold,
  completed, abandoned), record the date of each change, and keep the history.
- **FR-007**: The system MUST require a closing note to mark a project completed or
  abandoned, MUST make such a project read-only, and MUST allow it to be reopened.
- **FR-008**: Users MUST be able to set one project as current; commands that concern a
  project MUST apply to the current project unless another is named, and the system MUST
  make it clear which project is current.
- **FR-009**: When no project is current and more than one exists, the system MUST ask which
  project is meant and MUST NOT guess.
- **FR-010**: Before deleting a project, the system MUST list what it contains, MUST let the
  user choose between keeping its records in the workspace and deleting those that belong
  to no other project, and MUST require explicit confirmation.
- **FR-011**: When a workspace that has no projects is opened, the system MUST place its
  existing records in one Independent Research project named after the workspace, without
  losing or altering any record.
