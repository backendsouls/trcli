#### Scope and audience

- **FR-018**: Users MUST be able to scope a report to the whole workspace or to one or more
  projects; with a current project and no scope given, the report MUST cover the current
  project and say so.
- **FR-019**: A report over the whole workspace MUST group activity by project, with records
  belonging to no project grouped apart; activity MUST be attributed to the project a
  record belonged to when the event happened.
- **FR-020**: Users MUST be able to include or exclude sections by name, and to filter a
  report by tag and by responsible person.
- **FR-021**: The system MUST provide three audiences that set the sections and tone of a
  report: personal (everything), supervisor (progress against milestones, completed work,
  work in progress, blockers, plans), and funder (outputs, milestones, deviations from
  plan).
- **FR-022**: Users MUST be able to mark any note or record as private. Reports for any
  audience other than personal MUST leave out everything marked private and every record
  marked sensitive, and MUST state how many items were withheld.
- **FR-023**: The system MUST reject an unknown section, audience, project, tag, or person,
  and list the valid choices.
