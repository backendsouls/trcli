#### Common behavior

- **FR-001**: Every record type introduced here MUST support create, list, view, update,
  delete, tags, notes, links to other records visible from both ends, and audit entries,
  exactly as the base workspace's record types do, except where a requirement below makes a
  record append-only or frozen.
- **FR-002**: Every value a user supplies MUST be validated before anything is stored or
  acted upon; invalid input MUST change nothing and MUST be reported per value with what is
  expected.
- **FR-003**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-004**: Every command MUST have built-in help, and every feature MUST have a usage
  guide with examples.
