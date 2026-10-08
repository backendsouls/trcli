#### Common behavior

- **FR-049**: Every value a user supplies — template names, kinds, levels, locations,
  answers to a template's questions, and the content of templates brought in — MUST be
  validated before anything is created or stored; invalid input MUST change nothing and
  MUST be reported per value, all together, with what is expected.
- **FR-050**: Creating, changing, deleting, bringing in, and exporting templates, applying a
  template, and refreshing a document MUST be recorded in the workspace's audit trail;
  previewing and checking MUST NOT.
- **FR-051**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-052**: Templates MUST work without a network connection.
- **FR-053**: Every command MUST have built-in help; templates MUST have a usage guide with
  examples for applying, refreshing, creating, sharing, and checking, and a reference of
  every placeholder.
