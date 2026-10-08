#### Common behavior

- **FR-039**: Every value a user supplies for a report — dates, spans, names of periods,
  sections, audiences, scopes, and definitions — MUST be validated before anything is
  produced or stored; invalid input MUST be reported per value, all together, with what is
  expected and a valid example.
- **FR-040**: Producing a report MUST NOT change any record of the workspace; saving a
  report, changing its narrative, and deleting it MUST be recorded in the audit trail.
- **FR-041**: Producing a report MUST NOT appear as activity in later reports.
- **FR-042**: Reports MUST be produced without a network connection and MUST NOT send any
  content anywhere; sharing is done by the researcher with the exported document.
- **FR-043**: The settings for the first day of the week, the first month of the year, the
  default level of detail, and the default audience MUST be configurable per workspace.
- **FR-044**: Every report command MUST have built-in help, and reports MUST have a usage
  guide with an example for every period.
