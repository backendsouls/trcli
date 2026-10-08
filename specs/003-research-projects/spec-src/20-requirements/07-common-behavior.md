#### Common behavior

- **FR-049**: Every value a user supplies for projects, milestones, tasks, and types MUST be
  validated before anything is stored: required values present, text within stated lengths,
  dates valid and in a consistent order, and referenced projects, milestones, tasks, staff,
  and records existing. Invalid input MUST change nothing and MUST be reported per value
  with what is expected.
- **FR-050**: Projects, milestones, and tasks MUST support tags, notes, and links, and every
  creation, change, deletion, and status change MUST be recorded in the workspace's audit
  trail.
- **FR-051**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-052**: Every command MUST have built-in help, and projects, milestones, tasks, and
  project types MUST each have a usage guide with examples.
