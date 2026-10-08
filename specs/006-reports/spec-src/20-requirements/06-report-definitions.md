#### Report definitions

- **FR-035**: Users MUST be able to create, list, view, update, and delete named report
  definitions, each holding a period length, scope, audience, sections, level of detail,
  filters, narrative prompts, and export form.
- **FR-036**: Users MUST be able to produce a report by the name of its definition, and to
  override any part of the definition for one production.
- **FR-037**: Users MUST be able to set one definition as the default used when a report is
  requested with no options; with no default set, a report with no options MUST be the
  weekly report of the current scope at standard detail.
- **FR-038**: The system MUST reject a definition whose name is already in use or whose
  values are invalid, and MUST refuse to produce a definition that refers to something that
  no longer exists, naming it.
